#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Void,
}

impl Color {
    pub fn colors() -> Vec<Color> {
        vec![
            Color::Red,
            Color::Blue,
            Color::Green,
            Color::Yellow
        ]
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Kind {
    Sun,
    Moon,
    Key,
    Door,
    Nightmare,
}
