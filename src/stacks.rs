use std::fs;
use std::path::Path;

pub fn get_stacks(homelab_path: &str) -> Vec<String> {
    fs::read_dir(homelab_path)
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
        .collect()
}
