#[derive(Debug)]
#[non_exhaustive]
pub enum AppError {
    ParseError(&'static str)
}
