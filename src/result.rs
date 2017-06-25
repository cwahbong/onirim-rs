use std::result;

pub enum End {
    Win,
    Lose,
}

pub type Result<T> = result::Result<T, End>;
