use super::{Card, Color, Kind};
use ::core::Core;
use ::put::{self, PutCard};
use ::result::{End, Result};
use ::rule::can_obtain_door;

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

    fn on_drawn(&self, _: &mut Core) -> Result<Box<PutCard>> {
        Ok(Box::new(put::Hand))
    }

    fn on_played(&self, core: &mut Core) -> Result<Box<PutCard>> {
        if let Some(last_card) = core.content.explored.last() {
            if *last_card.get_kind() == self.kind {
                // LOG ERROR
                return Ok(Box::new(put::Hand));
            }
        }
        if can_obtain_door(&core.content) {
            let color = *core.content.explored.last().unwrap().get_color();
            if let Some(door) = core.content.pull_door(color) {
                core.content.opened.push(door);
                if core.content.opened.len() == 8  {
                    return Err(End::Win);
                }
            }
        }
        Ok(Box::new(put::Explored))
    }

    fn on_discarded(&self, core: &mut Core) -> Result<Box<PutCard>> {
        if self.kind == Kind::Key {
            let drawn = core.content.draw(5).ok_or(End::Lose)?; // maybe draw as much
            let (discarded_idx, back_idxs) = core.actor.key_discard_react(&core.content, &drawn);
            let mut optioned_drawn: Vec<Option<Box<Card>>> = drawn.into_iter()
                .map(|card| Some(card))
                .collect();
            // TODO check indices
            if let Some(optioned_card) = optioned_drawn.get_mut(discarded_idx) {
                core.content.discarded.push(optioned_card.take().unwrap());
            }
            for idx in back_idxs.into_iter().rev() {
                if let Some(optioned_card) = optioned_drawn.get_mut(idx) {
                    core.content.put_undrawn(optioned_card.take().unwrap());
                }
            }
        }
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
