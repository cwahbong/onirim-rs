use super::{Card, Color, Kind};
use ::core::Core;
use ::put::{self, PutCard};
use ::result::Result;

static NIGHTMARE_COLOR: Color = Color::Void;
static NIGHTMARE_KIND: Kind = Kind::Nightmare;

#[derive(Clone)]
struct Nightmare;

impl Nightmare {
    fn new() -> Self {
        Nightmare {}
    }
}

impl Card for Nightmare {
    fn get_color(&self) -> &Color {
        &NIGHTMARE_COLOR
    }

    fn get_kind(&self) -> &Kind {
        &NIGHTMARE_KIND
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

pub fn nightmare() -> Box<Card> {
    Box::new(Nightmare::new())
}
