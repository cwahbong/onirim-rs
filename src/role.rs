use ::action::Phase1Action;
use ::card::Card;
use ::content::Content;

pub trait Actor {
    fn phase_1_action(&mut self, content: &Content) -> (Phase1Action, usize);
    fn key_discard_react(&mut self, content: &Content, cards: Vec<&Card>);
    fn open_door(&mut self);
    fn nightmare_action(&mut self);
}

pub trait Observer {
}

