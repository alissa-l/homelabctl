mod cli;
mod config;
mod docker;
mod action;
mod stacks;

use cli::Cli;
use clap::Parser;
use config::load_config;
use docker::execute_stack;
use stacks::get_stacks;
use action::Action;

fn main() {
    let cli = Cli::parse();

let homelab_path = cli
    .path
    .clone()
    .or_else(|| std::env::var("HOMELAB").ok())
    .expect("The path to the homelab directory must be specified either via --path, homelabctl.toml or the HOMELAB environment variable.");


    if cli.verbose {
        println!("Using directory: {}", homelab_path);
    }

    let stacks = get_stacks(&homelab_path);

    // Load global configuration
    let config = load_config();

    // Resolve lista final de stacks a manter
    let keep_stacks: Vec<String> = match (&cli.keep_stacks, &config) {
        (Some(cli_ks), _) => cli_ks.clone(),
        (None, Some(cfg)) => cfg.keep_stacks.clone().unwrap_or_default(),
        _ => Vec::new(),
    };

    let up_ignore: Vec<String> = match (&cli.up_ignore, &config) {
        (Some(cli_ui), _) => cli_ui.clone(),
        (None, Some(cfg)) => cfg.up_ignore.clone().unwrap_or_default(),
        _ => Vec::new(),
    };

    let up_only: Vec<String> = match (&cli.up_only, &config) {
        (Some(cli_uo), _) => cli_uo.clone(),
        (None, Some(cfg)) => cfg.up_only.clone().unwrap_or_default(),
        _ => Vec::new(),
    };

    match &cli.stack {
        Some(stack) => {
            if !stacks.contains(stack) && cli.action != Action::Keep {
                eprintln!("Stack '{}' not found!", stack);
                std::process::exit(1);
            }
            execute_stack(&cli.action, &homelab_path, &stacks, Some(stack), &keep_stacks, &up_ignore, &up_only);
        }
        None => {
            execute_stack(&cli.action, &homelab_path, &stacks, None, &keep_stacks, &up_ignore, &up_only);
        }
    }

    println!("Action '{}' completed!", cli.action);
}
