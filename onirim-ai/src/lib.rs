extern crate onirim;

use onirim::content::Content;
use onirim::data::starting_content_basic;
use onirim::role::{Actor, Observer};
use onirim::runner::run;

use std::fmt;
use std::sync::{Arc, RwLock};

mod result;
pub mod util;

use result::{Error, Result};

pub struct CountStatisticReport {
    pub mean: f64,
    pub std_ev: f64,
    pub std_ev_pct: f64,
    pub std_err_mean: f64,
    pub std_err_mean_pct: f64,
}

impl CountStatisticReport {
    pub fn new(numerator: f64, denominator: f64) -> Self {
        // TODO check numerator < denominator
        let mean = numerator / denominator;
        let std_ev = mean * (1.0 - mean);
        let std_err_mean = std_ev / denominator.sqrt();
        let hundred = 100 as f64;
        CountStatisticReport {
            mean: mean,
            std_ev: std_ev,
            std_ev_pct: std_ev / mean * hundred,
            std_err_mean: std_err_mean,
            std_err_mean_pct: std_err_mean / mean * hundred,
        }
    }
}

pub struct Statistic {
    pub win_game: u32,
    pub lose_game: u32,
    pub success_game: u32,
    pub total_game: u32,
}

impl Statistic {
    pub fn new() -> Self {
        Statistic {
            win_game: 0,
            lose_game: 0,
            success_game: 0,
            total_game: 0,
        }
    }
}

impl fmt::Display for Statistic {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        writeln!(formatter, "win: {}", self.win_game)?;
        writeln!(formatter, "total: {}", self.success_game)?;
        writeln!(formatter, "tried: {}", self.total_game)?;
        let report = CountStatisticReport::new(self.win_game as f64, self.success_game as f64);
        write!(formatter, "win ratio: {:.3}% mean, {:.3e} ({:.3}%) stdev, {:.3e} ({:.3}%) sem",
            report.mean, report.std_ev, report.std_ev_pct,
            report.std_err_mean, report.std_err_mean_pct)
    }
}

struct ExperimentObserver {
    statistic: Arc<RwLock<Statistic>>,
}

impl ExperimentObserver {
    fn new(statistic: Arc<RwLock<Statistic>>) -> Self {
        ExperimentObserver { statistic: statistic }
    }
}

impl Observer for ExperimentObserver {
    fn on_end(&mut self, _: &Content, result: &onirim::result::Result<()>) {
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
    }
}

pub trait NewBoxActor {
    fn new_box_actor() -> Box<Actor>;
}

pub fn run_experiment<N: NewBoxActor>(content: Content) -> Result<Statistic> {
    let statistic = Arc::new(RwLock::new(Statistic::new()));
    for _ in 0..1000 {
        let actor = N::new_box_actor();
        let observer = Box::new(ExperimentObserver::new(statistic.clone()));
        run(actor, observer, content.clone());
    }
    Arc::try_unwrap(statistic).map_err(|_| Error::ResourceLeak)?
        .into_inner().map_err(|_| Error::ResourceLeak)
}

pub fn run_experiment_basic<N: NewBoxActor>() -> Result<Statistic> {
    run_experiment::<N>(starting_content_basic())
}
