use ::content::Content;
use ::role::{Actor, Observer};

pub struct Core {
    pub actor: Box<Actor>,
    pub observer: Box<Observer>,
    pub content: Content,
}
