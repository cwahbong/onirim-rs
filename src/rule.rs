use ::card::{Color, Kind};
use ::content::Content;

pub fn may_open_door(content: &Content, color: Color) -> bool {
    content.hand.iter()
                .any(|ref card| {
                    card.get_kind() == &Kind::Key && card.get_color() == &color
                })
}

pub fn can_obtain_door(content: &Content) -> bool {
    if let Some(ref last_explored) = content.explored.last() {
        let mut count = 0;
        for card in content.explored.iter().rev() {
            if last_explored.get_color() == card.get_color() {
                count += 1;
            } else {
                break;
            }
        }
        count % 3 == 0
    } else {
        false
    }
}
