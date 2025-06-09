use clap::{Parser};
use crate::action::Action;

#[derive(Parser)]
#[command(name = "homelabctl")]
#[command(about = "Manages docker compose stacks", long_about = None)]
#[command(long_about = "A command-line tool to manage Docker Compose stacks in a homelab environment.")]
#[command(version = "0.01")]
#[command(author = "Alissa Araujo")]
pub struct Cli {
    // The action to perform on the stack
    #[arg(short, long, help = "Action to perform on the stack")]
    #[arg(default_value = "up", help = "Default action is 'up'")]
    #[arg(value_enum)]
    pub action: Action,

    // The stack to operate on
    #[arg(short, long, help = "Name of the stack to operate on")]
    #[arg(value_name = "STACK_NAME")]
    #[arg(help = "Specify the stack name to operate on. If not provided, all stacks will be affected.")]
    pub stack: Option<String>,
    
    // The path to the homelab directory
    #[arg(long, help = "Path to the homelab directory")]
    #[arg(value_name = "HOMELAB_PATH")]
    #[arg(help = "Specify the path to the homelab directory. 
    If not provided, it will use the path specified in $HOME/homelabctl
    or the $HOMELAB environment variable.")]
    #[arg(short, long)]
    pub path: Option<String>,

    // Verbose
    #[arg(short, long)]
    pub verbose: bool,

    // Keep stacks
    #[arg(long, help = "Keep specified stacks running")]
    #[arg(short = 'k', long = "keep-stacks", value_delimiter = ',')]
    pub keep_stacks: Option<Vec<String>>,
}
