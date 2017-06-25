use ::card::{Card, Color, Kind};

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
        if let Some(idx) = door_idx {
            Some(self.undrawn.swap_remove(idx))
        } else {
            None
        }
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

    pub fn replenish_hand(&mut self) { // TODO what if cannot draw?
        while self.hand.len() < 5 {
            let card = self.do_draw(1).unwrap().pop().unwrap();
            if card.is_location() {
                self.hand.push(card)
            } else {
                self.limbo.push(card)
            }
        }
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
