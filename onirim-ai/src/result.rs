use std::result;

#[derive(Debug)]
pub enum Error {
    ResourceLeak,
}

pub type Result<T> = result::Result<T, Error>;
