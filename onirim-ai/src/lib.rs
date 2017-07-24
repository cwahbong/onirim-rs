extern crate onirim;

use onirim::content::Content;
use onirim::data::starting_content_basic;
use onirim::role::{Actor, Observer};
use onirim::runner::run;

use std::sync::{Arc, RwLock};
use std::thread;

mod result;
mod statistic;
pub mod evaluator;

pub use result::{Error, Result};
pub use statistic::Statistic;

struct ExperimentObserver {
    statistic: Arc<RwLock<Statistic>>,
}

impl ExperimentObserver {
    fn new(statistic: Arc<RwLock<Statistic>>) -> Self {
        ExperimentObserver { statistic: statistic }
    }
}

impl Observer for ExperimentObserver {
    fn on_end(&mut self, content: &Content, result: &onirim::result::Result<()>) {
        let mut statistic = self.statistic.try_write().unwrap();
        match *result {
            Err(onirim::result::End::Win) => {
                statistic.win_game += 1;
                statistic.success_game += 1;
            }
            Err(onirim::result::End::Lose) => {
                statistic.lose_game += 1;
                statistic.success_game += 1;
            }
            _ => {}
        }
        statistic.total_game += 1;
        statistic.opened += content.get_opened().len() as u32;
    }
}

pub trait NewBoxActor {
    fn new_box_actor(&self) -> Box<Actor>;
}

pub fn run_experiment<N: NewBoxActor>(new_box_actor: N, content: Content, count: u32) -> Result<Statistic> {
    let statistic = Arc::new(RwLock::new(Statistic::new()));
    for _ in 0..count {
        let actor = new_box_actor.new_box_actor();
        let observer = Box::new(ExperimentObserver::new(statistic.clone()));
        run(actor, observer, content.clone());
    }
    Arc::try_unwrap(statistic).map_err(|_| Error::ResourceLeak)?
        .into_inner().map_err(|_| Error::ResourceLeak)
}

pub fn run_experiment_basic<N: NewBoxActor>(new_box_actor: N, count: u32) -> Result<Statistic> {
    run_experiment(new_box_actor, starting_content_basic(), count)
}

pub fn paralleled(run: fn(u32) -> Result<Statistic>, count: u32, worker: u32) -> Result<Statistic> {
    let handles: Vec<thread::JoinHandle<Result<Statistic>>> = (0..worker).map(|idx| {
        let job_count = count / worker + if count % worker < idx + 1 { 0 } else { 1 };
        thread::Builder::new()
            .name(idx.to_string())
            .spawn(move || { run(job_count) })
            .unwrap()
        }).collect();
    handles.into_iter().map(|handle| { handle.join().unwrap() }).sum()
}
