use clap::{Parser, ValueEnum};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(name = "homelabctl")]
#[command(about = "Manages docker compose stacks", long_about = None)]
struct Cli {
    #[arg(value_enum)]
    action: Action,

    stack: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Action {
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

// Stacks that should always be kept running, even when using `keep` action.
// This is useful for stacks that are critical to the homelab's operation, like Traefik.
// Can be configured via environment variable, cli argument or configuration file
const ALWAYS_STACKS: &[&str] = &["traefik"];

use std::fmt;

fn main() {
    let cli = Cli::parse();
    // Homelab path can be either set as an environment variable, passed as an argument or executed in the current directory.
    // If not set, it will panic with a message.

    let homelab_path = env::var("HOMELAB").expect("Environment variable $HOMELAB not set");

    let stacks: Vec<String> = fs::read_dir(&homelab_path)
        .expect("Failed to read HOMELAB directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir() && path.join("docker-compose.yml").exists() {
                Some(
                    path.file_name()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                )
            } else {
                None
            }
        })
        .collect();

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

fn execute_stack(action: &Action, homelab: &str, stacks: &[String], target_stack: Option<&String>) {
    match action {
        Action::Keep => {
            let exclude = match target_stack {
                Some(s) => {
                    let mut all = ALWAYS_STACKS.to_vec();
                    all.push(s.as_str());
                    all
                }
                None => ALWAYS_STACKS.to_vec(),
            };

            for s in stacks {
                if !exclude.contains(&s.as_str()) {
                    run_docker(homelab, s, "down");
                } else {
                    println!("Stack '{}' is always active and will not be stopped.", s);
                }
            }
        }
        _ => {
            let targets: Vec<&String> = match target_stack {
                Some(s) => vec![s],
                None => stacks.iter().collect(),
            };

            for stack in targets {
                let cmd = match action {
                    Action::Up => "up -d",
                    Action::Down => "down",
                    Action::Logs => "logs -f",
                    Action::Restart => "restart",
                    Action::Status => "ps",
                    _ => "",
                };
                run_docker(homelab, stack, cmd);
            }
        }
    }
}

fn run_docker(homelab: &str, stack: &str, args: &str) {
    println!("Executing '{}' on stack '{}'...", args, stack);
    let path = Path::new(homelab).join(stack);
    println!("Stack path: {:?}", path);

    let docker_args: Vec<&str> = args.split_whitespace().collect();

    let status = Command::new("docker")
        .arg("compose")
        .args(&docker_args)
        .current_dir(&path)
        .status();


    if !status.expect("Failed to execute command").success() {
        eprintln!("Failed to execute command on stack '{}'", stack);
    }
}
