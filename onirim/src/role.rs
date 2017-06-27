use ::action::{NightmareAction, Phase1Action};
use ::card::Card;
use ::content::Content;
use ::result::Result;

pub trait Actor {
    fn phase_1_action(&mut self, content: &Content) -> (Phase1Action, usize);
    fn key_discard_react(&mut self, content: &Content, cards: &Vec<Box<Card>>) -> (usize, Vec<usize>);
    fn open_door(&mut self, content: &Content) -> bool;
    fn nightmare_action(&mut self, content: &Content) -> (NightmareAction, Option<usize>);
}

pub trait Observer {
    fn on_end(&mut self, content: &Content, result: &Result<()>);
}

