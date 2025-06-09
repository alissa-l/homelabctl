use crate::action::Action;
use std::path::Path;
use std::process::Command;

pub fn execute_stack(
    action: &Action,
    homelab: &str,
    stacks: &[String],
    target_stack: Option<&String>,
    keep_stacks: &[String],
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
                    println!("Stack '{}' será mantido ativo.", s);
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
                    Action::RemoveOrphaned => "down --remove-orphans",
                    _ => "",
                };
                run_docker(homelab, stack, cmd);
            }
        }
    }
}

fn run_docker(homelab: &str, stack: &str, args: &str) {
    println!("Executando '{}' no stack '{}'...", args, stack);
    let path = Path::new(homelab).join(stack);
    println!("Caminho do stack: {:?}", path);

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
