use std::io::SeekFrom;
use std::ops::Bound;
use std::path::Path;

use axum::async_trait;
use axum::body::{Body, HttpBody};
use axum::extract::rejection::{TypedHeaderRejection, TypedHeaderRejectionReason};
use axum::extract::{FromRequest, RequestParts, TypedHeader};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use headers::{AcceptRanges, ContentLength, ContentRange, ContentType, HeaderMapExt, Range};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio_util::codec::{BytesCodec, FramedRead};

pub struct FileRequest {
    range: Option<Range>,
}

#[async_trait]
impl<B: Send> FromRequest<B> for FileRequest {
    type Rejection = TypedHeaderRejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let header = TypedHeader::from_request(req).await;
        let range = match header {
            Ok(TypedHeader(range)) => Some(range),
            Err(e) => match e.reason() {
                TypedHeaderRejectionReason::Missing => None,
                TypedHeaderRejectionReason::Error(_) => return Err(e),
            },
        };

        Ok(FileRequest { range })
    }
}

pub struct FileResponse {
    res: Response<Body>,
}

impl FileResponse {
    pub async fn from_request(
        req: FileRequest,
        path: impl AsRef<Path>,
    ) -> std::io::Result<FileResponse> {
        let mime = mime_guess::from_path(path.as_ref()).first_or_octet_stream();
        let mut file = tokio::fs::File::open(path).await?;

        let total_length = file.metadata().await?.len();
        let range = req.range.and_then(|range| range.iter().next());

        let mut res = Response::new(Body::empty());

        res.headers_mut().typed_insert(AcceptRanges::bytes());
        res.headers_mut().typed_insert(ContentType::from(mime));

        match range {
            Some((from, to)) => {
                let from = match from {
                    Bound::Included(n) => n,
                    Bound::Excluded(n) => n + 1,
                    Bound::Unbounded => 0,
                };

                let to = match to {
                    Bound::Included(n) => n,
                    Bound::Excluded(n) => n - 1,
                    Bound::Unbounded => total_length - 1,
                };

                file.seek(SeekFrom::Start(from)).await?;

                let read_length = u64::min(total_length - from, to - from + 1);
                let reader = file.take(read_length);
                let stream = FramedRead::new(reader, BytesCodec::new());
                let range = from..=from + read_length - 1;

                res.headers_mut().typed_insert(ContentLength(read_length));
                res.headers_mut()
                    .typed_insert(ContentRange::bytes(range, total_length).unwrap());

                *res.status_mut() = StatusCode::PARTIAL_CONTENT;
                *res.body_mut() = Body::wrap_stream(stream);
            }
            None => {
                res.headers_mut().typed_insert(ContentLength(total_length));

                *res.status_mut() = StatusCode::OK;
                *res.body_mut() = Body::wrap_stream(FramedRead::new(file, BytesCodec::new()));
            }
        }

        Ok(FileResponse { res })
    }
}

impl IntoResponse for FileResponse {
    type Body = Body;
    type BodyError = <Body as HttpBody>::Error;

    fn into_response(self) -> Response<Self::Body> {
        self.res
    }
}
