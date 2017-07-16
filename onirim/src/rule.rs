use ::card::{Card, Color, Kind};
use ::content::Content;
use ::result::{End, Result};

pub fn may_open_door(content: &Content, color: Color) -> bool {
    content.get_hand().iter()
                      .any(|ref card| {
                          card.get_kind() == &Kind::Key && card.get_color() == &color
                      })
}

pub fn combo_count(content: &Content) -> usize {
    if let Some(ref last_explored) = content.get_explore().last() {
        let mut count = 0;
        for card in content.get_explore().iter().rev() {
            if last_explored.get_color() == card.get_color() {
                count += 1;
            } else {
                break;
            }
        }
        count % 3
    } else {
        0
    }
}

pub fn can_obtain_door(content: &Content, color: &Color, kind: &Kind) -> bool {
    combo_count(content) == 2
        && color == content.get_explore().last().unwrap().get_color()
        && kind != content.get_explore().last().unwrap().get_kind()
}

pub fn put_opened_and_check(content: &mut Content, card: Box<Card>) -> Result<()> {
    content.put_opened(card);
    if content.get_opened().len() == 8 {
        Err(End::Win)
    } else {
        Ok(())
    }
}
