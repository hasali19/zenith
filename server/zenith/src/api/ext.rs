use super::error::{self};
use super::ApiResult;

pub trait OptionExt<T> {
    fn or_bad_request(self, msg: impl ToString) -> ApiResult<T>;
    fn or_not_found(self, msg: impl ToString) -> ApiResult<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn or_bad_request(self, msg: impl ToString) -> ApiResult<T> {
        self.ok_or_else(|| error::bad_request(msg))
    }

    fn or_not_found(self, msg: impl ToString) -> ApiResult<T> {
        self.ok_or_else(|| error::not_found(msg))
    }
}
