use std::io::SeekFrom;
use std::ops::Bound;
use std::path::Path;

use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::{request, StatusCode};
use axum::response::{IntoResponse, Response};
use axum_extra::typed_header::{TypedHeaderRejection, TypedHeaderRejectionReason};
use axum_extra::TypedHeader;
use headers::{
    AcceptRanges, ContentDisposition, ContentLength, ContentRange, ContentType, Header,
    HeaderMapExt, HeaderValue, Range,
};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio_util::codec::{BytesCodec, FramedRead};

pub struct FileRequest {
    range: Option<Range>,
}

impl<S: Send + Sync> FromRequestParts<S> for FileRequest {
    type Rejection = TypedHeaderRejection;

    async fn from_request_parts(
        req: &mut request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let header = TypedHeader::from_request_parts(req, state).await;
        let range = match header {
            Ok(TypedHeader(range)) => Some(range),
            Err(e) => match e.reason() {
                TypedHeaderRejectionReason::Missing => None,
                _ => return Err(e),
            },
        };

        Ok(FileRequest { range })
    }
}

pub struct FileResponse {
    res: Response<Body>,
    content_disposition: Option<HeaderValue>,
}

impl FileResponse {
    pub async fn from_request(
        req: FileRequest,
        path: impl AsRef<Path>,
    ) -> std::io::Result<FileResponse> {
        let mime = mime_guess::from_path(path.as_ref()).first_or_octet_stream();
        let mut file = tokio::fs::File::open(path).await?;

        let total_length = file.metadata().await?.len();
        let range = req
            .range
            .and_then(|range| range.satisfiable_ranges(total_length).next());

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
                *res.body_mut() = Body::from_stream(stream);
            }
            None => {
                res.headers_mut().typed_insert(ContentLength(total_length));

                *res.status_mut() = StatusCode::OK;
                *res.body_mut() = Body::from_stream(FramedRead::new(file, BytesCodec::new()));
            }
        }

        Ok(FileResponse {
            res,
            content_disposition: None,
        })
    }

    pub fn with_content_disposition(mut self, value: HeaderValue) -> Self {
        self.content_disposition = Some(value);
        self
    }
}

impl IntoResponse for FileResponse {
    fn into_response(mut self) -> Response {
        if let Some(content_disposition) = self.content_disposition {
            self.res
                .headers_mut()
                .insert(ContentDisposition::name(), content_disposition);
        }
        self.res.into_response()
    }
}
