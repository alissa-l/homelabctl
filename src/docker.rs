use crate::action::Action;
use std::path::Path;
use std::process::Command;

const ALWAYS_STACKS: &[&str] = &["traefik"];

pub fn execute_stack(action: &Action, homelab: &str, stacks: &[String], target_stack: Option<&String>) {
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
