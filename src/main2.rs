mod cli;
mod action;
mod stacks;
mod docker;

use cli::Cli;
use clap::Parser;
use action::Action;
use stacks::get_stacks;
use docker::execute_stack;

fn main() {
    let cli = Cli::parse();

    let homelab_path = std::env::var("HOMELAB")
        .expect("Environment variable $HOMELAB not set");

    let stacks = get_stacks(&homelab_path);

    match &cli.stack {
        Some(stack) => {
            if !stacks.contains(stack) && cli.action != Action::Keep {
                eprintln!("Stack '{}' not found!", stack);
                std::process::exit(1);
            }
            execute_stack(&cli.action, &homelab_path, &stacks, Some(stack));
        }
        None => {
            execute_stack(&cli.action, &homelab_path, &stacks, None);
        }
    }

    println!("Action '{}' completed!", cli.action);
}
