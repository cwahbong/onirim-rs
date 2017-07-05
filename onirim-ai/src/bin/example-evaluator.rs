extern crate onirim;
extern crate onirim_ai;

use onirim::card::{nightmare, Card, Color, Kind};
use onirim::content::Content;
use onirim::rule::combo_count;
use onirim::util::Count;
use onirim_ai::evaluator::{Evaluator, NewEvaluator, NewBoxEvaluateActor};
use onirim_ai::{paralleled, run_experiment_basic, Result, Statistic};

use std::cmp::{max, min};

const NIGHTMARE_SCORE: i64 =    10000000000;
const DOOR_SCORE: i64      =    10000000000;
const WIN_SCORE: i64       =   100000000000;
const LOSE_BASE_SCORE: i64 = -1000000000000;

struct Counts {
    undrawn: Count,
    discarded: Count,
    limbo: Count,
    opened: Count,
    hand: Count,
}

impl Counts {
    pub fn count(content: &Content) -> Self {
        Counts {
            undrawn: content.count_undrawn(),
            discarded: content.count_discard(),
            limbo: content.count_limbo(),
            opened: content.count_opened(),
            hand: content.count_hand(),
        }
    }

    pub fn available_color_kind(&self, color_kind: &(Color, Kind)) -> usize {
        let mut count = 0;
        count += self.undrawn.color_kind(color_kind);
        count += self.limbo.color_kind(color_kind);
        count += self.hand.color_kind(color_kind);
        count
    }
}

fn evaluate_lose(content: &Content, counts: &Counts) -> i64 {
    if counts.discarded.kind(&Kind::Door) > 0 {
        return LOSE_BASE_SCORE
    }
    for color in Color::colors() {
        let opened = counts.opened.color(&color);
        if opened == 2 {
            continue;
        }
        let sun = counts.undrawn.color_kind(&(color, Kind::Sun)) +
            counts.limbo.color_kind(&(color, Kind::Sun)) +
            counts.hand.color_kind(&(color, Kind::Sun));
        let moon = counts.undrawn.color_kind(&(color, Kind::Moon)) +
            counts.limbo.color_kind(&(color, Kind::Moon)) +
            counts.hand.color_kind(&(color, Kind::Moon));
        let key = counts.undrawn.color_kind(&(color, Kind::Key)) +
            counts.limbo.color_kind(&(color, Kind::Key)) +
            counts.hand.color_kind(&(color, Kind::Key));
        let need_open = 2 - opened;
        let combo = match content.get_explore().last() {
            Some(card) => {
                if card.get_color() == &color {
                    combo_count(content)
                } else {
                    0
                }
            }
            None => 0
        };
        if (sun + moon + combo) / 3 + key < need_open {
            return LOSE_BASE_SCORE + 1
        }
    }
    0
}

fn evaluate_win_opened(content: &Content) -> i64 {
    let opened = content.get_opened().len() as i64;
    let mut score = opened * DOOR_SCORE;
    if opened == 8 {
        score += WIN_SCORE;
    }
    return score
}

fn evaluate_win_resolved_nightmare(counts: &Counts) -> i64 {
    counts.discarded.kind(&Kind::Nightmare) as i64 * NIGHTMARE_SCORE
}

fn evaluate_win_available_key(counts: &Counts) -> i64 {
    let mut score = 0;
    for color in Color::colors() {
        let color_opened = counts.opened.color(&color);
        let weight = 10 + (2 - color_opened) as i64;
        score += counts.available_color_kind(&(color, Kind::Key)) as i64 * 10000 * weight;
    }
    score
}

fn evaluate_win_available_sun_moon(counts: &Counts) -> i64 {
    let mut score = 0;
    for color in Color::colors() {
        let sun_count = counts.available_color_kind(&(color, Kind::Sun)) as i64;
        let moon_count = counts.available_color_kind(&(color, Kind::Moon)) as i64;
        let color_opened = counts.opened.color(&color);
        let weight = if color_opened == 2 { 1 } else { 6 + (2 - color_opened) } as i64;
        let count = min(sun_count, moon_count);
        score += (21 - count) * count / 2 * weight * 10;
    }
    score
}

fn evaluate_win_available_location(counts: &Counts) -> i64 {
    let mut score = 0;
    score += evaluate_win_available_key(counts);
    score += evaluate_win_available_sun_moon(counts);
    score
}

fn next_permutation_by_key<T: Clone, B: Ord, F: FnMut(&T) -> B>(v: &mut [T], mut key: F) -> bool {
    if v.len() < 2 {
        return false
    }
    for idx in (0..v.len() - 1).rev() {
        if key(&v[idx]) < key(&v[idx + 1]) {
            let mut idx2 = v.len() - 1;
            while key(&v[idx2]) <= key(&v[idx]) {
                idx2 -= 1;
            }
            v.swap(idx, idx2);
            let mut lo = idx + 1;
            let mut hi = v.len() - 1;
            while lo < hi {
                v.swap(lo, hi);
                lo += 1;
                hi -= 1;
            }
            return true
        }
    }
    false
}

fn get_cont_combo(combo: i64, last_card: &Box<Card>, hand: &Vec<Box<Card>>) -> i64 {
    let mut color_hand: Vec<Box<Card>> = hand.iter()
        .filter(|card| card.get_color() == last_card.get_color())
        .filter(|card| card.get_kind() != &Kind::Key)
        .cloned()
        .collect();
    color_hand.sort_by_key(|card| {
        (*card.get_color(), *card.get_kind())
    });
    let mut cont_combo = 0;
    while let Some(hand_card) = color_hand.get(cont_combo as usize) {
        let prev_card = if let Some(card) = color_hand.get(cont_combo as usize - 1) { card } else { last_card };
        if prev_card.get_kind() != hand_card.get_kind() {
            cont_combo += 1;
        } else {
            break;
        }
    }
    while next_permutation_by_key(&mut color_hand, |card| { (*card.get_color(), *card.get_kind()) }) {
        let mut tmp_cont_combo = 0;
        while let Some(hand_card) = color_hand.get(tmp_cont_combo as usize) {
            let prev_card = if let Some(card) = color_hand.get(cont_combo as usize - 1) { card } else { last_card };
            if prev_card.get_kind() != hand_card.get_kind() {
                tmp_cont_combo += 1;
            } else {
                break;
            }
        }
        cont_combo = max(cont_combo, tmp_cont_combo);
    }
    min(cont_combo, 3 - combo)
}

fn evaluate_win_three_combo(content: &Content, counts: &Counts) -> i64 {
    let combo = combo_count(content) as i64;

    let last_card = content.get_explore().last().cloned().unwrap_or(nightmare());
    let last_color = last_card.get_color();
    let last_color_opened = counts.opened.color(last_color);
    let combo_weight = if last_color_opened == 2 { 1 } else { 4 + (2 - last_color_opened) } as i64;

    let cont_combo = get_cont_combo(combo, &last_card, content.get_hand()); // XXX should try all possible colors
    (combo * 6000 + cont_combo * 4000) * combo_weight
}

fn evaluate_win(content: &Content, counts: &Counts) -> i64 {
    let mut score = 0;
    score += evaluate_win_opened(content);
    score += evaluate_win_resolved_nightmare(counts);
    score += evaluate_win_available_location(counts);
    score += evaluate_win_three_combo(content, counts);
    score
}

struct SimpleEvaluator;

impl Evaluator for SimpleEvaluator {
    fn evaluate(&self, content: &Content) -> i64 {
        let counts = Counts::count(content);
        let mut score = 0;
        score += evaluate_lose(content, &counts);
        score += evaluate_win(content, &counts);
        score
    }
}

struct SimpleNewEvaluator;

impl NewEvaluator for SimpleNewEvaluator {
    type E = SimpleEvaluator;

    fn new_evaluator(&self) -> Self::E {
        SimpleEvaluator
    }
}

fn run(count: u32) -> Result<Statistic> {
        run_experiment_basic(NewBoxEvaluateActor::new(SimpleNewEvaluator), count)
}

fn main() {
    let statistic = paralleled(run, 10000, 4).unwrap();
    println!("{}", statistic);
}
