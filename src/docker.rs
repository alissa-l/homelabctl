use crate::action::Action;
use std::path::Path;
use std::process::Command;

pub fn execute_stack(
    action: &Action,
    homelab: &str,
    stacks: &[String],
    target_stack: Option<&String>,
    keep_stacks: &[String],
    up_ignore: &[String],
    up_only: &[String],
) {
    match action {
        Action::Kill => {
            for s in stacks {
                run_docker(homelab, s, "down");
            }
        }

        Action::Keep => {
            let mut exclude: Vec<&str> = keep_stacks.iter().map(|s| s.as_str()).collect();

            if let Some(s) = target_stack {
                exclude.push(s.as_str());
            }

            for s in stacks {
                if !exclude.contains(&s.as_str()) {
                    run_docker(homelab, s, "down");
                } else {
                    println!("Stack '{}' will be kept active.", s);
                }
            }
        }

        Action::Up => {
            let targets: Vec<&String> = if let Some(s) = target_stack {
                vec![s]
            } else if !up_only.is_empty() {
                stacks
                    .iter()
                    .filter(|s| up_only.contains(s))
                    .collect()
            } else {
                stacks
                    .iter()
                    .filter(|s| !up_ignore.contains(s))
                    .collect()
            };

            for stack in targets {
                run_docker(homelab, stack, "up -d");
            }
        }

        _ => {
            let targets: Vec<&String> = match target_stack {
                Some(s) => vec![s],
                None => stacks.iter().collect(),
            };

            for stack in targets {
                let cmd = match action {
                    Action::Down => "down",
                    Action::Logs => "logs -f",
                    Action::Restart => "restart",
                    Action::Status => "ps",
                    Action::RemoveOrphaned => "down --remove-orphans",
                    _ => "",
                };
                run_docker(homelab, stack, cmd);
            }
        }
    }
}

fn run_docker(homelab: &str, stack: &str, args: &str) {
    let path = Path::new(homelab).join(stack);

    let docker_args: Vec<&str> = args.split_whitespace().collect();

    let status = Command::new("docker")
        .arg("compose")
        .args(&docker_args)
        .current_dir(&path)
        .status();

    if !status.expect("Error executing command").success() {
        eprintln!("Error executing command on stack '{}'", stack);
    }
}
