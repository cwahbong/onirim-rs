#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase1Action {
    Play,
    Discard,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NightmareAction {
    ByKey,
    ByDoor,
    ByHand,
    ByDeck,
}
