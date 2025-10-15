pub mod factory;
pub mod instrument;
use crate::error::code::AppError;

type APPResult<T> = Result<T, AppError>;