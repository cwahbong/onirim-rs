use card::Card;
use content::Content;

pub trait PutCard {
    fn perform(&self, &mut Content, Box<Card>);
}

pub struct Noop;

impl PutCard for Noop {
    fn perform(&self, _: &mut Content, _: Box<Card>) {}
}

pub struct Hand;

impl PutCard for Hand {
    fn perform(&self, content: &mut Content, card: Box<Card>) {
        content.hand.push(card)
    }
}

pub struct Explored;

impl PutCard for Explored {
    fn perform(&self, content: &mut Content, card: Box<Card>) {
        content.explored.push(card)
    }
}

pub struct Discarded;

impl PutCard for Discarded {
    fn perform(&self, content: &mut Content, card: Box<Card>) {
        content.discarded.push(card)
    }
}
