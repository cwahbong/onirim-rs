use super::{Color, Kind};
use ::core::Core;
use ::put::PutCard;
use ::result::Result;

pub trait Card {
    fn get_color(&self) -> &Color;
    fn get_kind(&self) -> &Kind;
    fn is_location(&self) -> bool;
    fn on_drawn(&self, &Core) -> Box<PutCard>;
    fn on_played(&self, &Core) -> Result<Box<PutCard>>;
    fn on_discarded(&self, &Core) -> Result<Box<PutCard>>;
    fn clone_into_box(&self) -> Box<Card> ;
}

impl Clone for Box<Card> {
    fn clone(&self) -> Box<Card> {
        self.clone_into_box()
    }
}
