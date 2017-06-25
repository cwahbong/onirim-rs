use super::{Card, Color, Kind};
use ::core::Core;
use ::put::{self, PutCard};
use ::result::Result;

#[derive(Clone)]
struct Location {
    color: Color,
    kind: Kind,
}

impl Location {
    pub fn new(color: Color, kind: Kind) -> Self {
        Location {
            color: color,
            kind: kind,
        }
    }
}

impl Card for Location {
    fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_kind(&self) -> &Kind {
        &self.kind
    }

    fn is_location(&self) -> bool {
        true
    }

    fn on_drawn(&self, _: &Core) -> Box<PutCard> {
        Box::new(put::Hand)
    }

    fn on_played(&self, core: &Core) -> Result<Box<PutCard>> {
        if let Some(last_card) = core.content.explored.last() {
            if *last_card.get_kind() == self.kind {
                return Ok(Box::new(put::Hand));
            }
        }
        if false /* can_obtain_door */ {
        }
        Ok(Box::new(put::Explored))
    }

    fn on_discarded(&self, core: &Core) -> Result<Box<PutCard>> {
        Ok(Box::new(put::Discarded))
    }

    fn clone_into_box(&self) -> Box<Card> {
        Box::new(self.clone())
    }
}

pub fn sun(color: Color) -> Box<Card> {
    Box::new(Location::new(color, Kind::Sun))
}

pub fn moon(color: Color) -> Box<Card> {
    Box::new(Location::new(color, Kind::Moon))
}

pub fn key(color: Color) -> Box<Card> {
    Box::new(Location::new(color, Kind::Key))
}
