use thiserror::Error;
#[derive(Error, Debug)]
pub enum AppError {
    #[error("site blocked")]
    Blocked,
}
