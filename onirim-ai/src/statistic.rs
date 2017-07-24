use std::fmt;
use std::iter;
use std::ops;

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
    pub opened: u32,
}

impl Statistic {
    pub fn new() -> Self {
        Statistic {
            win_game: 0,
            lose_game: 0,
            success_game: 0,
            total_game: 0,
            opened: 0,
        }
    }
}

impl fmt::Display for Statistic {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        writeln!(formatter, "win: {}", self.win_game)?;
        writeln!(formatter, "total: {}", self.success_game)?;
        writeln!(formatter, "tried: {}", self.total_game)?;
        writeln!(formatter, "avg opened: {}", self.opened as f64 / self.success_game as f64)?;
        let report = CountStatisticReport::new(self.win_game as f64, self.success_game as f64);
        write!(formatter, "win ratio: {:.3}% mean, {:.3e} ({:.3}%) stdev, {:.3e} ({:.3}%) sem",
            report.mean, report.std_ev, report.std_ev_pct,
            report.std_err_mean, report.std_err_mean_pct)
    }
}

impl ops::Add for Statistic {
    type Output = Statistic;

    fn add(mut self, other: Statistic) -> Self::Output {
        self.win_game += other.win_game;
        self.lose_game += other.lose_game;
        self.success_game += other.success_game;
        self.total_game += other.total_game;
        self.opened += other.opened;
        self
    }
}

impl iter::Sum for Statistic {
    fn sum<I: Iterator<Item = Statistic>>(iter: I) -> Statistic {
        iter.fold(Statistic::new(), ops::Add::add)
    }
}
