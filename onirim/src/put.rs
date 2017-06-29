use card::Card;
use content::Content;
use result::Result;
use rule::put_opened_and_check;

pub trait PutCard {
    fn perform(&self, &mut Content, Box<Card>) -> Result<()>;
}

pub struct Discarded;

impl PutCard for Discarded {
    fn perform(&self, content: &mut Content, card: Box<Card>) -> Result<()> {
        content.put_discard(card);
        Ok(())
    }
}

pub struct Hand;

impl PutCard for Hand {
    fn perform(&self, content: &mut Content, card: Box<Card>) -> Result<()> {
        content.put_hand(card);
        Ok(())
    }
}

pub struct Explored;

impl PutCard for Explored {
    fn perform(&self, content: &mut Content, card: Box<Card>) -> Result<()> {
        content.put_explore(card);
        Ok(())
    }
}

pub struct Limbo;

impl PutCard for Limbo {
    fn perform(&self, content: &mut Content, card: Box<Card>) -> Result<()> {
        content.put_limbo(card);
        Ok(())
    }
}

pub struct Opened;

impl PutCard for Opened {
    fn perform(&self, content: &mut Content, card: Box<Card>) -> Result<()> {
        put_opened_and_check(content, card)
    }
}
