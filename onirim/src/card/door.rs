use super::{Card, Color, Kind};
use ::core::Core;
use ::put::{self, PutCard};
use ::result::{End, Result};
use ::rule::may_open_door;

static DOOR_KIND: Kind = Kind::Door;

#[derive(Clone)]
struct Door {
    color: Color,
}

impl Door {
    fn new(color: Color) -> Self {
        Door { color: color }
    }
}

impl Card for Door {
    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_kind(&self) -> &Kind {
        &DOOR_KIND
    }

    fn is_location(&self) -> bool {
        false
    }

    fn on_drawn(&self, core: &mut Core) -> Result<Box<PutCard>> {
        let do_open = if may_open_door(&core.content, self.color) {
            core.actor.open_door(&core.content)
        } else {
            false
        };
        if do_open {
            let mut key_idx = None;
            for (idx, card) in core.content.get_hand().iter().enumerate() {
                if card.get_kind() == &Kind::Key {
                    key_idx = Some(idx);
                    break;
                }
            }
            let idx = key_idx.ok_or(End::ShouldNotReach)?;
            core.content.discard_hand(idx);
            Ok(Box::new(put::Opened))
        } else {
            Ok(Box::new(put::Limbo))
        }
    }

    fn on_played(&self, _: &mut Core) -> Result<Box<PutCard>> {
        Err(End::ShouldNotReach)
    }

    fn on_discarded(&self, _: &mut Core) -> Result<Box<PutCard>> {
        Err(End::ShouldNotReach)
    }

    fn clone_into_box(&self) -> Box<Card> {
        Box::new(self.clone())
    }
}

pub fn door(color: Color) -> Box<Card> {
    Box::new(Door::new(color))
}
