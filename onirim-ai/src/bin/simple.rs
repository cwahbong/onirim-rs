extern crate onirim;
extern crate onirim_ai;

use onirim::card::{Card, Color, Kind};
use onirim::action::{Phase1Action, NightmareAction};
use onirim::content::Content;
use onirim::role::Actor;
use onirim::rule::combo_count;
use onirim_ai::{run_experiment_basic, NewBoxActor};
use onirim_ai::util::Count;

fn first_idx_of_kind<'a, I: Iterator<Item=&'a Box<Card>>>(iter: I, kind: &Kind) -> Option<usize> {
    iter.enumerate()
        .filter(|&(_, card)| card.get_kind() == kind)
        .next()
        .map(|(idx, _)| idx)
}

fn first_idx_of_color<'a, I: Iterator<Item=&'a Box<Card>>>(iter: I, color: &Color) -> Option<usize> {
    iter.enumerate()
        .filter(|&(_, card)| card.get_color() == color)
        .next()
        .map(|(idx, _)| idx)
}

fn pick_first_explore(hand: &Vec<Box<Card>>, hand_count: &Count) -> Option<usize> {
    hand_count.get_color_map().iter()
        .max_by_key(|&(_, freq)| freq)
        .map(|(color, _)| color)
        .map(|color| first_idx_of_color(hand.iter(), color).unwrap())
}

struct Phase1Act<'a> {
    content: &'a Content,
    hand_count: Count,
    door_count: Count,
}

impl<'a> Phase1Act<'a> {
    fn new(content: &'a Content) -> Self {
        Phase1Act {
            content: content,
            hand_count: Count::count(content.get_hand().iter()),
            door_count: Count::count(content.get_opened().iter()),
        }
    }

    fn play_combo_first_idx(&self) -> Option<usize> {
        match self.content.get_explore().last() {
            Some(last_explore) => {
                self.content.get_hand().iter()
                    .enumerate()
                    .filter(|&(_, card)| card.get_kind() != last_explore.get_kind())
                    .filter(|&(_, card)| card.get_kind() != &Kind::Key)
                    .filter(|&(_, card)| self.door_count.color(card.get_color()) < 2)
                    .next()
                    .map(|(idx, _)| idx)
            }
            None => pick_first_explore(self.content.get_hand(), &self.hand_count),
        }
    }

    fn play_combo_continue_idx(&self) -> Option<usize> {
        self.content.get_explore().last().and_then(|last_explore| {
            self.content.get_hand().iter()
                .enumerate()
                .filter(|&(_, card)| card.get_kind() != last_explore.get_kind())
                .filter(|&(_, card)| card.get_color() == last_explore.get_color())
                .next()
                .map(|(idx, _)| idx)
        })
    }

    fn try_play_idx(&self) -> Option<usize> {
        if combo_count(self.content) == 0 {
            self.play_combo_first_idx()
        } else {
            self.play_combo_continue_idx()
        }
    }

    fn try_play_action(&self) -> Option<(Phase1Action, usize)> {
        self.try_play_idx().map(|idx| (Phase1Action::Play, idx))
    }

    fn discard_idx(&self) -> usize {
        for kind in &[Kind::Sun, Kind::Moon] {
            if let Some(idx) = first_idx_of_kind(self.content.get_hand().iter(), kind) {
                return idx
            }
        }
        let idx_by_opened_door = self.door_count.get_color_map().iter()
            .filter(|&(color, _)| self.hand_count.color(color) > 0)
            .max_by_key(|&(_, freq)| freq)
            .map(|(color, _)| color)
            .map(|color| first_idx_of_color(self.content.get_hand().iter(), color).unwrap());
        if let Some(idx) = idx_by_opened_door {
            return idx
        }
        0
    }

    fn discard_action(&self) -> (Phase1Action, usize) {
        (Phase1Action::Discard, self.discard_idx())
    }

    fn get_action(&self) -> (Phase1Action, usize) {
        if let Some(action) = self.try_play_action() {
            return action
        }
        self.discard_action()
    }

    pub fn action(content: &'a Content) -> (Phase1Action, usize) {
        Phase1Act::new(content).get_action()
    }
}

fn nightmare_action_by_key(content: &Content) -> Option<usize> {
    let opened_count = Count::count(content.get_opened().iter());
    let mut opened_color_count: Vec<(&Color, &usize)> = opened_count.get_color_map().iter().collect();
    opened_color_count.sort_by_key(|&(_, freq)| freq);
    opened_color_count.iter().rev().filter_map(|&(color, _)| {
        content.get_hand().iter().enumerate()
            .filter(|&(_, card)| card.get_kind() == &Kind::Key)
            .filter(|&(_, card)| card.get_color() == color)
            .next()
            .map(|(idx, _)| idx)
    }).next()
}

fn nightmare_action_by_hand(content: &Content) -> bool {
    // TODO if discarded hand cause lose then return false
    true
}

fn discard_by_key(idx: usize) -> (usize, Vec<usize>) {
    let reorder = (0..5).filter(|&i| i != idx).collect();
    return (idx, reorder)
}

struct SimpleActor;

impl Actor for SimpleActor {
    fn phase_1_action(&mut self, content: &Content) -> (Phase1Action, usize) {
        Phase1Act::action(content)
    }

    fn key_discard_react(&mut self, _: &Content, cards: &Vec<Box<Card>>) -> (usize, Vec<usize>) {
        for (idx, card) in cards.iter().enumerate() {
            if card.get_kind() == &Kind::Nightmare {
                return discard_by_key(idx)
            }
        }
        return discard_by_key(0)
    }

    fn open_door(&mut self, _: &Content) -> bool {
        true
    }

    fn nightmare_action(&mut self, content: &Content) -> (NightmareAction, Option<usize>) {
        if let Some(idx) = nightmare_action_by_key(content) {
            return (NightmareAction::ByKey, Some(idx)) // TODO discard with most opened door
        }
        if nightmare_action_by_hand(content) {
            return (NightmareAction::ByHand, None)
        }
        (NightmareAction::ByDeck, None)
    }
}

struct SimpleNewBoxActor;

impl NewBoxActor for SimpleNewBoxActor {
    fn new_box_actor() -> Box<Actor> {
        Box::new(SimpleActor)
    }
}

fn main() {
    let statistic = run_experiment_basic::<SimpleNewBoxActor>().unwrap();
    println!("{}", statistic);
}
