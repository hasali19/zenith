use std::io::SeekFrom;
use std::ops::Bound;
use std::path::Path;

use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::{StatusCode, request};
use axum::response::{IntoResponse, Response};
use axum_extra::TypedHeader;
use axum_extra::typed_header::{TypedHeaderRejection, TypedHeaderRejectionReason};
use headers::{
    AcceptRanges, ContentDisposition, ContentLength, ContentRange, ContentType, Header,
    HeaderMapExt, HeaderValue, Range,
};
use speq::{HeaderSpec, RouteHandlerInput};
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

impl RouteHandlerInput for FileRequest {
    fn describe(_cx: &mut speq::RouteHandlerInputContext, route: &mut speq::RouteSpec) {
        route.headers.push(HeaderSpec {
            name: "Range".into(),
            type_desc: None,
            is_optional: true,
        });
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

#[cfg(test)]
mod tests {
    use http_body_util::BodyExt;

    use super::*;

    const TEST_FILE: &[u8] = include_bytes!("../test/test.txt");

    #[tokio::test]
    async fn simple_request() {
        let FileResponse { res, .. } =
            FileResponse::from_request(FileRequest { range: None }, "test/test.txt")
                .await
                .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let accept_ranges = res.headers().get(AcceptRanges::name());
        assert_eq!(accept_ranges, Some(&HeaderValue::from_static("bytes")));

        let content_type = res.headers().get(ContentType::name());
        assert_eq!(content_type, Some(&HeaderValue::from_static("text/plain")));

        let content_length = res.headers().get(ContentLength::name());
        assert_eq!(
            content_length,
            Some(&HeaderValue::from_str(&format!("{}", TEST_FILE.len())).unwrap())
        );
    }

    #[tokio::test]
    async fn range_request() {
        let FileResponse { res, .. } = FileResponse::from_request(
            FileRequest {
                range: Some(Range::bytes(10..=19).unwrap()),
            },
            "test/test.txt",
        )
        .await
        .unwrap();

        assert_eq!(res.status(), StatusCode::PARTIAL_CONTENT);

        let accept_ranges = res.headers().get(AcceptRanges::name());
        assert_eq!(accept_ranges, Some(&HeaderValue::from_static("bytes")));

        let content_type = res.headers().get(ContentType::name());
        assert_eq!(content_type, Some(&HeaderValue::from_static("text/plain")));

        let content_length = res.headers().get(ContentLength::name());
        assert_eq!(content_length, Some(&HeaderValue::from_static("10")));

        let content_range = res.headers().get(ContentRange::name());
        assert_eq!(
            content_range,
            Some(&HeaderValue::from_str(&format!("bytes 10-19/3227")).unwrap())
        );

        let bytes = res
            .into_body()
            .into_data_stream()
            .collect()
            .await
            .unwrap()
            .to_bytes();

        assert_eq!(&bytes, &TEST_FILE[10..=19]);
    }
}
