use super::{Card, Color, Kind};
use ::core::Core;
use ::put::{self, PutCard};
use ::result::Result;

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

    fn on_drawn(&self, _: &Core) -> Box<PutCard> {
        Box::new(put::Noop)
    }

    fn on_played(&self, core: &Core) -> Result<Box<PutCard>> {
        Ok(Box::new(put::Noop))
    }

    fn on_discarded(&self, core: &Core) -> Result<Box<PutCard>> {
        Ok(Box::new(put::Discarded))
    }

    fn clone_into_box(&self) -> Box<Card> {
        Box::new(self.clone())
    }
}

pub fn door(color: Color) -> Box<Card> {
    Box::new(Door::new(color))
}
