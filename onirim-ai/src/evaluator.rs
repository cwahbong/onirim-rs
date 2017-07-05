use onirim::card::{Card, Kind};
use onirim::action::{NightmareAction, Phase1Action};
use onirim::content::Content;
use onirim::role::Actor;
use onirim::rule::can_obtain_door;

use std::collections::BTreeMap;
use std::i64;

use ::NewBoxActor;

fn key_of_max_value<'a, K: Clone + 'a, V: Ord + 'a, I: Iterator<Item=(&'a K, &'a V)>>(iter: I) -> Option<K>
{
    iter.max_by_key(|&(_, value)| value)
        .map(|(key, _)| key)
        .cloned()
}

pub trait Evaluator {
    fn evaluate(&self, content: &Content) -> i64;
}

pub trait NewEvaluator {
    type E: Evaluator;

    fn new_evaluator(&self) -> Self::E;
}

pub const RULE_VIOLATION_SCORE: i64 = i64::MIN;

struct EvaluateActor<E: Evaluator> {
    evaluator: E,
}

impl<E: Evaluator> EvaluateActor<E> {
    pub fn new(evaluator: E) -> Self {
        EvaluateActor { evaluator: evaluator }
    }

    fn evaluate_phase_1_play(&self, content: &Content, idx: usize) -> i64 {
        let mut econtent = content.clone();
        let card = econtent.take_hand(idx);
        if let Some(last_card) = econtent.get_explore().last() {
            if last_card.get_kind() == card.get_kind() {
                return RULE_VIOLATION_SCORE
            }
        }
        if can_obtain_door(&econtent, card.get_color(), card.get_kind()) {
            let color = *econtent.get_explore().last().unwrap().get_color();
            if let Some(door) = econtent.pull_door(color) {
                econtent.put_opened(door);
            }
        }
        econtent.put_explore(card);
        self.evaluator.evaluate(&econtent)
    }

    fn evaluate_phase_1_discard(&self, content: &Content, idx: usize) -> i64 {
        let mut econtent = content.clone();
        let card = econtent.take_hand(idx);
        econtent.put_discard(card);
        self.evaluator.evaluate(&econtent)
    }

    fn evaluate_key_discard_react(&self, content: &Content, discard_card: &Box<Card>, keep_cards: &Vec<&Box<Card>>) -> i64 {
        let mut econtent = content.clone();
        econtent.put_discard(discard_card.clone());
        for keep_card in keep_cards.iter().rev() {
            econtent.put_undrawn((*keep_card).clone());
        }
        self.evaluator.evaluate(&econtent)
    }

    fn evaluate_nightmare_by_key(&self, content: &Content, idx: usize) -> i64 {
        let mut econtent = content.clone();
        let key = econtent.take_hand(idx);
        econtent.put_discard(key);
        self.evaluator.evaluate(&econtent)
    }

    fn evaluate_nightmare_by_door(&self, content: &Content, idx: usize) -> i64 {
        let mut econtent = content.clone();
        let door = econtent.take_opened(idx);
        econtent.put_limbo(door);
        self.evaluator.evaluate(&econtent)
    }

    fn evaluate_nightmare_by_hand(&self, content: &Content) -> i64 {
        let mut econtent = content.clone();
        econtent.discard_all_hand();
        self.evaluator.evaluate(&econtent)
    }

    fn evaluate_nightmare_by_deck(&self, _: &Content) -> i64 {
        -1000000
    }
}

impl<E: Evaluator> Actor for EvaluateActor<E> {
    fn phase_1_action(&mut self, content: &Content) -> (Phase1Action, usize) {
        let mut action_scores = BTreeMap::new();
        for idx in 0..content.get_hand().len() {
            action_scores.insert((Phase1Action::Play, idx), self.evaluate_phase_1_play(&content, idx));
            action_scores.insert((Phase1Action::Discard, idx), self.evaluate_phase_1_discard(&content, idx));
        }
        key_of_max_value(action_scores.iter()).unwrap()
    }

    fn key_discard_react(&mut self, content: &Content, cards: &Vec<Box<Card>>) -> (usize, Vec<usize>) {
        let mut react_scores = BTreeMap::new();
        for discard_idx in 0..5 {
            let discard_card = cards.get(discard_idx).unwrap();
            let keep_idxs: Vec<usize> = (0..5).filter(|&keep_idx| keep_idx != discard_idx).collect();
            let keep_cards = keep_idxs.iter()
                .map(|&keep_idx| cards.get(keep_idx).unwrap())
                .collect();
            react_scores.insert((discard_idx, keep_idxs), self.evaluate_key_discard_react(&content, &discard_card, &keep_cards));
        }
        key_of_max_value(react_scores.iter()).unwrap()
    }

    fn open_door(&mut self, _: &Content) -> bool {
        // XXX need color information
        true
    }

    fn nightmare_action(&mut self, content: &Content) -> (NightmareAction, Option<usize>) {
        let mut action_scores = BTreeMap::new();
        let key_idx_iter = content.get_hand().iter().enumerate()
            .filter(|&(_, card)| card.get_kind() == &Kind::Key)
            .map(|(idx, _)| idx);
        for idx in key_idx_iter {
            action_scores.insert((NightmareAction::ByKey, Some(idx)), self.evaluate_nightmare_by_key(content, idx));
        }
        for idx in 0..content.get_opened().len() { // TODO do not calc duplicate door
            action_scores.insert((NightmareAction::ByDoor, Some(idx)), self.evaluate_nightmare_by_door(content, idx));
        }
        action_scores.insert((NightmareAction::ByHand, None), self.evaluate_nightmare_by_hand(content));
        action_scores.insert((NightmareAction::ByDeck, None), self.evaluate_nightmare_by_deck(content));
        key_of_max_value(action_scores.iter()).unwrap()
    }
}

pub struct NewBoxEvaluateActor<NE: NewEvaluator<E=E>, E: Evaluator + 'static> {
    new_evaluator: NE,
}

impl<NE: NewEvaluator<E=E>, E: Evaluator + 'static> NewBoxEvaluateActor<NE, E> {
    pub fn new(new_evaluator: NE) -> Self {
        NewBoxEvaluateActor { new_evaluator: new_evaluator }
    }
}

impl<NE: NewEvaluator<E=E>, E: Evaluator + 'static> NewBoxActor for NewBoxEvaluateActor<NE, E> {
    fn new_box_actor(&self) -> Box<Actor> {
        Box::new(EvaluateActor::new(self.new_evaluator.new_evaluator()))
    }
}

