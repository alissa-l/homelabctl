use clap::ValueEnum;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Action {
    #[clap(help = "Starts stacks using docker compose")]
    Up,
    #[clap(help = "Stops stacks using docker compose")]
    Down,
    #[clap(help = "Shows logs for stacks using docker compose")]
    Logs,
    #[clap(help = "Restarts stacks using docker compose")]
    Restart,
    #[clap(help = "Shows status for stacks using docker compose")]
    Status,
    #[clap(help = "Keeps specified stacks running")]
    Keep,
    #[clap(help = "Stops and removes all stacks")]
    Kill,
    #[clap(help = "Removes orphaned containers")]
    RemoveOrphaned,
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
            Action::Kill => "kill",
            Action::RemoveOrphaned => "remove-orphaned",
        };
        write!(f, "{}", s)
    }
}
