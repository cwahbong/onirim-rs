use super::{Card, Color, Kind};
use ::action::NightmareAction;
use ::content::Content;
use ::core::Core;
use ::put::{self, PutCard};
use ::result::{End, Result};

static NIGHTMARE_COLOR: Color = Color::Void;
static NIGHTMARE_KIND: Kind = Kind::Nightmare;

#[derive(Clone)]
struct Nightmare;

impl Nightmare {
    fn new() -> Self {
        Nightmare {}
    }

    fn by_key(&self, content: &mut Content, idx: Option<usize>) -> Result<()> {
        let idx = idx.ok_or(End::BadParameter)?;
        {
            let key = content.hand.get(idx).ok_or(End::BadParameter)?;
            if key.get_kind() != &Kind::Key {
                return Err(End::BadParameter);
            }
        }
        let key = content.hand.swap_remove(idx);
        content.put_discard(key);
        Ok(())
    }

    fn by_door(&self, content: &mut Content, idx: Option<usize>) -> Result<()> {
        let idx = idx.ok_or(End::BadParameter)?;
        if idx >= content.opened.len() {
            return Err(End::BadParameter);
        }
        let door = content.opened.swap_remove(idx);
        content.put_limbo(door);
        Ok(())
    }

    fn by_hand(&self, content: &mut Content, idx: Option<usize>) -> Result<()> {
        if idx.is_some() {
            return Err(End::BadParameter);
        }
        content.discard_hand();
        content.replenish_hand()?;
        Ok(())
    }

    fn by_deck(&self, content: &mut Content, idx: Option<usize>) -> Result<()> {
        if idx.is_some() {
            return Err(End::BadParameter);
        }
        let drawn = content.draw(5).ok_or(End::Lose)?;
        for card in drawn.into_iter() {
            if card.is_location() {
                content.put_discard(card);
            } else {
                content.put_limbo(card);
            }
        }
        Ok(())
    }
}

impl Card for Nightmare {
    fn get_color(&self) -> &Color {
        &NIGHTMARE_COLOR
    }

    fn get_kind(&self) -> &Kind {
        &NIGHTMARE_KIND
    }

    fn is_location(&self) -> bool {
        false
    }

    fn on_drawn(&self, core: &mut Core) -> Result<Box<PutCard>> {
        let (nightmare_action, idx) = core.actor.nightmare_action(&core.content);
        let by = match nightmare_action {
            NightmareAction::ByKey => Nightmare::by_key,
            NightmareAction::ByDoor => Nightmare::by_door,
            NightmareAction::ByHand => Nightmare::by_hand,
            NightmareAction::ByDeck => Nightmare::by_deck,
        };
        by(&self, &mut core.content, idx)?;
        Ok(Box::new(put::Discarded))
    }

    fn on_played(&self, _: &mut Core) -> Result<Box<PutCard>> {
        Err(End::ShouldNotReach)
    }

    fn on_discarded(&self, _: &mut Core) -> Result<Box<PutCard>> {
        Err(End::ShouldNotReach)
    }

    fn clone_into_box(&self) -> Box<Card> {
        Box::new(self.clone())
    }
}

pub fn nightmare() -> Box<Card> {
    Box::new(Nightmare::new())
}
