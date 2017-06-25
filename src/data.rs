use ::card::{door, key, moon, nightmare, sun, Card, Color};
use ::content::Content;

fn color_cards(new_card: fn(Color) -> Box<Card>, color: Color, count: usize) -> Vec<Box<Card>> {
    vec![new_card(color); count]
}

fn all_color_cards(new_card: fn(Color) -> Box<Card>, count: usize) -> Vec<Box<Card>> {
    let mut cards = Vec::new();
    for color in Color::colors().into_iter() {
        cards.append(&mut color_cards(new_card, color, count));
    }
    cards
}

fn no_color_cards(new_card: fn() -> Box<Card>, count: usize) -> Vec<Box<Card>> {
    vec![new_card(); count]
}

pub fn starting_cards_basic() -> Vec<Box<Card>> {
    let mut cards = Vec::new();
    cards.append(&mut color_cards(sun, Color::Red, 9));
    cards.append(&mut color_cards(sun, Color::Blue, 8));
    cards.append(&mut color_cards(sun, Color::Green, 7));
    cards.append(&mut color_cards(sun, Color::Yellow, 6));
    cards.append(&mut all_color_cards(moon, 4));
    cards.append(&mut all_color_cards(key, 3));
    cards.append(&mut all_color_cards(door, 2));
    cards.append(&mut no_color_cards(nightmare, 10));
    cards
}

pub fn starting_content_basic() -> Content {
    Content::new(starting_cards_basic())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starting_cards_basic_len() {
        assert_eq!(76, starting_cards_basic().len());
    }
}
