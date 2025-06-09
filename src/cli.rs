use clap::{Parser, ValueEnum};
use crate::action::Action;

#[derive(Parser)]
#[command(name = "homelabctl")]
#[command(about = "Manages docker compose stacks", long_about = None)]
pub struct Cli {
    #[arg(value_enum)]
    pub action: Action,

    pub stack: Option<String>,
}
