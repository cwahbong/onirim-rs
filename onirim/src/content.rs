use ::card::{Card, Color, Kind};
use ::result::{End, Result};
use ::util::Count;

use rand::{Rng, thread_rng};

#[derive(Clone)]
pub struct Content {
    undrawn: Vec<Box<Card>>,
    discarded: Vec<Box<Card>>,
    limbo: Vec<Box<Card>>,
    explored: Vec<Box<Card>>,
    opened: Vec<Box<Card>>,
    hand: Vec<Box<Card>>,
}

impl Content {
    pub fn new(undrawn: Vec<Box<Card>>) -> Self {
        Content {
            undrawn: undrawn,
            discarded: vec![],
            limbo: vec![],
            explored: vec![],
            opened: vec![],
            hand: vec![],
        }
    }

    pub fn pull_door(&mut self, color: Color) -> Option<Box<Card>> {
        let mut door_idx = None;
        for (idx, card) in self.undrawn.iter().enumerate() {
            if *card.get_kind() == Kind::Door && *card.get_color() == color {
                door_idx = Some(idx);
                break;
            }
        }
        door_idx.map(|idx| { self.undrawn.swap_remove(idx) })
    }

    fn do_draw(&mut self, count: usize) -> Option<Vec<Box<Card>>> {
        if count > self.undrawn.len() {
            None
        } else {
            let undrawn_len = self.undrawn.len();
            Some(self.undrawn.drain(undrawn_len - count..).collect())
        }
    }

    pub fn draw(&mut self, count: usize) -> Option<Vec<Box<Card>>> {
        self.do_draw(count)
    }

    pub fn get_discard(&self) -> &Vec<Box<Card>> {
        &self.discarded
    }

    pub fn get_limbo(&self) -> &Vec<Box<Card>> {
        &self.limbo
    }

    pub fn get_explore(&self) -> &Vec<Box<Card>> {
        &self.explored
    }

    pub fn get_opened(&self) -> &Vec<Box<Card>> {
        &self.opened
    }

    pub fn get_hand(&self) -> &Vec<Box<Card>> {
        &self.hand
    }

    pub fn put_undrawn(&mut self, card: Box<Card>) {
        self.undrawn.push(card)
    }

    pub fn put_discard(&mut self, card: Box<Card>) {
        self.discarded.push(card)
    }

    pub fn put_limbo(&mut self, card: Box<Card>) {
        self.limbo.push(card)
    }

    pub fn put_explore(&mut self, card: Box<Card>) {
        self.explored.push(card)
    }

    pub fn put_opened(&mut self, card: Box<Card>) {
        self.opened.push(card)
    }

    pub fn put_hand(&mut self, card: Box<Card>) {
        self.hand.push(card)
    }

    pub fn take_opened(&mut self, idx: usize) -> Box<Card> {
        self.opened.swap_remove(idx)
    }

    pub fn take_hand(&mut self, idx: usize) -> Box<Card> {
        self.hand.swap_remove(idx)
    }

    pub fn discard_opened(&mut self, idx: usize) {
        self.discarded.push(self.opened.swap_remove(idx))
    }

    pub fn discard_hand(&mut self, idx: usize) {
        self.discarded.push(self.hand.swap_remove(idx))
    }

    pub fn discard_all_hand(&mut self) {
        self.discarded.append(&mut self.hand)
    }

    pub fn replenish_hand(&mut self) -> Result<()> {
        while self.hand.len() < 5 {
            let mut drawn = self.do_draw(1).ok_or(End::Lose)?;
            let card = drawn.pop().ok_or(End::ShouldNotReach)?;
            if card.is_location() {
                self.hand.push(card)
            } else {
                self.limbo.push(card)
            }
        }
        Ok(())
    }

    pub fn shuffle_undrawn(&mut self) {
        thread_rng().shuffle(&mut self.undrawn)
    }

    pub fn shuffle_limbo_to_undrawn(&mut self) {
        if !self.limbo.is_empty() {
            self.undrawn.append(&mut self.limbo);
            thread_rng().shuffle(&mut self.undrawn)
        }
    }

    pub fn count_undrawn(&self) -> Count {
        Count::count(self.undrawn.iter())
    }

    pub fn count_discard(&self) -> Count {
        Count::count(self.discarded.iter())
    }

    pub fn count_limbo(&self) -> Count {
        Count::count(self.limbo.iter())
    }

    pub fn count_explore(&self) -> Count {
        Count::count(self.explored.iter())
    }

    pub fn count_opened(&self) -> Count {
        Count::count(self.opened.iter())
    }

    pub fn count_hand(&self) -> Count {
        Count::count(self.hand.iter())
    }
}
