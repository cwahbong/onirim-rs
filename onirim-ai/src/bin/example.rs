extern crate onirim;
extern crate onirim_ai;

use onirim::card::Card;
use onirim::action::{Phase1Action, NightmareAction};
use onirim::content::Content;
use onirim::role::Actor;
use onirim_ai::{run_experiment_basic, NewBoxActor};

struct ExampleActor;

impl Actor for ExampleActor {
    fn phase_1_action(&mut self, _: &Content) -> (Phase1Action, usize) {
        (Phase1Action::Discard, 0)
    }

    fn key_discard_react(&mut self, _: &Content, _: &Vec<Box<Card>>) -> (usize, Vec<usize>) {
        (0, vec![1, 2, 3, 4])
    }

    fn open_door(&mut self, _: &Content) -> bool {
        true
    }

    fn nightmare_action(&mut self, _: &Content) -> (NightmareAction, Option<usize>) {
        (NightmareAction::ByDeck, None)
    }
}

struct ExampleNewBoxActor;

impl NewBoxActor for ExampleNewBoxActor {
    fn new_box_actor(&self) -> Box<Actor> {
        Box::new(ExampleActor)
    }
}

fn main() {
    let statistic = run_experiment_basic(ExampleNewBoxActor).unwrap();
    println!("{}", statistic);
}
