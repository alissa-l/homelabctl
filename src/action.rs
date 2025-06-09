use clap::ValueEnum;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Action {
    Up,
    Down,
    Logs,
    Restart,
    Status,
    Keep,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Action::Up => "up",
            Action::Down => "down",
            Action::Logs => "logs",
            Action::Restart => "restart",
            Action::Status => "status",
            Action::Keep => "keep",
        };
        write!(f, "{}", s)
    }
}
