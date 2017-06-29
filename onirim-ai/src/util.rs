use onirim::card::{Card, Color, Kind};

use std::collections::BTreeMap;
use std::iter::Iterator;

pub struct Count {
    color_map: BTreeMap<Color, usize>,
    kind_map: BTreeMap<Kind, usize>,
    color_kind_map: BTreeMap<(Color, Kind), usize>,
}

impl Count {
    pub fn count<'a, I: Iterator<Item=&'a Box<Card>>>(iter: I) -> Self {
        let mut color_map = BTreeMap::new();
        let mut kind_map = BTreeMap::new();
        let mut color_kind_map = BTreeMap::new();
        for card in iter {
            let color = *card.get_color();
            let kind = *card.get_kind();
            *color_map.entry(color).or_insert(0) += 1;
            *kind_map.entry(kind).or_insert(0) += 1;
            *color_kind_map.entry((color, kind)).or_insert(0) += 1;
        }
        Count {
            color_map: color_map,
            kind_map: kind_map,
            color_kind_map: color_kind_map,
        }
    }

    pub fn get_color_map(&self) -> &BTreeMap<Color, usize> {
        &self.color_map
    }

    pub fn get_kind_map(&self) -> &BTreeMap<Kind, usize> {
        &self.kind_map
    }

    pub fn get_color_kind_map(&self) -> &BTreeMap<(Color, Kind), usize> {
        &self.color_kind_map
    }

    pub fn color(&self, color: &Color) -> usize {
        self.color_map.get(color).cloned().unwrap_or(0)
    }

    pub fn kind(&self, kind: &Kind) -> usize {
        self.kind_map.get(kind).cloned().unwrap_or(0)
    }

    pub fn color_kind(&self, color_kind: &(Color, Kind)) -> usize {
        self.color_kind_map.get(color_kind).cloned().unwrap_or(0)
    }
}
