use ::card::{Card, Color, Kind};
use ::result::{End, Result};

use rand::{Rng, thread_rng};

pub struct Content {
    pub undrawn: Vec<Box<Card>>,
    pub discarded: Vec<Box<Card>>,
    pub limbo: Vec<Box<Card>>,
    pub explored: Vec<Box<Card>>,
    pub opened: Vec<Box<Card>>,
    pub hand: Vec<Box<Card>>,
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

    pub fn put_undrawn(&mut self, card: Box<Card>) {
        self.undrawn.push(card);
    }

    pub fn put_discard(&mut self, card: Box<Card>) {
        self.discarded.push(card);
    }

    pub fn put_limbo(&mut self, card: Box<Card>) {
        self.limbo.push(card);
    }

    pub fn put_hand(&mut self, card: Box<Card>) {
        self.hand.push(card);
    }

    pub fn discard_hand(&mut self) {
        self.discarded.append(&mut self.hand);
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
        thread_rng().shuffle(&mut self.undrawn);
    }

    pub fn shuffle_limbo_to_undrawn(&mut self) {
        if !self.limbo.is_empty() {
            self.undrawn.append(&mut self.limbo);
            thread_rng().shuffle(&mut self.undrawn);
        }
    }
}
