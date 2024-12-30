pub mod anime;
pub mod manga;
mod util;

pub struct EnmaError;

impl EnmaError {
    pub fn new() -> EnmaError {
        EnmaError {}
    }
}
