use ::action::Phase1Action;
use ::content::Content;
use ::core::Core;
use ::role::{Actor, Observer};
use ::result::{End, Result};

pub struct Runner {
    core: Core,
}

impl Runner {
    pub fn new(core: Core) -> Self {
        Runner { core: core }
    }

    fn setup(&mut self) -> Result<()> {
        self.core.content.shuffle_undrawn();
        self.core.content.replenish_hand()?;
        self.core.content.shuffle_limbo_to_undrawn();
        Ok(())
    }

    fn phase_1(&mut self) -> Result<()> {
        let (action, idx) = self.core.actor.phase_1_action(&self.core.content);
        let card = self.core.content.hand.swap_remove(idx);
        let put = match action {
            Phase1Action::Play => card.on_played(&mut self.core)?,
            Phase1Action::Discard => card.on_discarded(&mut self.core)?,
        };
        put.perform(&mut self.core.content, card);
        Ok(())
    }

    fn phase_2(&mut self) -> Result<()> {
        while self.core.content.hand.len() < 5 {
            let cards = self.core.content.draw(1);
            if cards.is_none() {
                return Err(End::Lose);
            } else {
                let card = cards.unwrap().pop().unwrap();
                let put = card.on_drawn(&mut self.core)?;
                put.perform(&mut self.core.content, card);
            }
        }
        Ok(())
    }

    fn phase_3(&mut self) -> Result<()> {
        self.core.content.shuffle_limbo_to_undrawn();
        Ok(())
    }

    fn whole(&mut self) -> Result<()> {
        self.setup()?;
        loop {
            self.phase_1()?;
            self.phase_2()?;
            self.phase_3()?;
        }
    }
}

pub fn run(actor: Box<Actor>, observer: Box<Observer>, content: Content) -> End {
    let mut runner = Runner::new(Core {
        actor: actor,
        observer: observer,
        content: content,
    });
    runner.whole().unwrap_err()
}
