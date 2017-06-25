use std::result;

pub enum End {
    Win,
    Lose,
    BadParameter,
    ShouldNotReach,
}

pub type Result<T> = result::Result<T, End>;
