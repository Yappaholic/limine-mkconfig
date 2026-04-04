use crate::kernel;
use std::fs;
use std::path::PathBuf;

pub fn generate_config() -> String {
    if let Some(user_config) = parse_user_config() {
        let mut main_config = generate_config_entries(Some(user_config.cmdline));
        main_config = format!(
            "{}{main_config}\n{}",
            user_config.settings, user_config.entries
        );
        main_config
    } else {
        generate_config_entries(None)
    }
}

pub fn find_existing_config() -> Option<PathBuf> {
    let paths = [
        "/boot/EFI/BOOT/limine.conf",
        "/boot/limine.conf",
        "/boot/limine/limine.conf",
        "/etc/limine.conf",
        "/limine.conf",
    ]
    .iter()
    .map(|e: &&str| PathBuf::from(e))
    .collect::<Vec<PathBuf>>();
    let mut result = PathBuf::new();
    for path in paths {
        let conf = fs::exists(&path);
        if conf.is_ok() && conf.unwrap() {
            result = path;
            break;
        }
    }
    if result.to_str().unwrap().is_empty() {
        None
    } else {
        Some(result)
    }
}

pub fn generate_config_entries(cmdline: Option<String>) -> String {
    let kernels = kernel::find_installed_kernels();
    let mut result = String::new();
    for kernel in kernels {
        result = format!(
            "{}\n/ {}\n\tprotocol: linux\n\tpath: boot():/{}\n\tmodules_path: boot():/{}\n\tcomment: Boot {}!",
            result, &kernel.name, kernel.path, kernel.initramfs_path, kernel.name
        );
        if let Some(ref cmd) = cmdline {
            result = format!("{result}\n\tcmdline:{}", cmd);
        }
    }
    result
}

pub fn overwrite_config(contents: String) {
    if let Some(path) = find_existing_config() {
        std::fs::write(&path, contents).unwrap_or_else(|_| {
            eprintln!(
                "Failed to overwrite config at {}, exiting...",
                &path.display()
            );
            std::process::exit(1);
        })
    } else {
        eprintln!("Existing config not found, skipping...")
    }
}

#[derive(Default)]
pub struct UserConfig {
    pub entries: String,
    pub cmdline: String,
    pub settings: String,
}

pub fn parse_user_config() -> Option<UserConfig> {
    let mut user_config = UserConfig::default();
    if let Ok(true) = std::fs::exists("/etc/limine/extras.conf") {
        let user_config_file =
            std::fs::read_to_string("/etc/limine/extras.conf").unwrap_or_else(|e| {
                eprintln!("Failed to open user config file: {e}");
                std::process::exit(2);
            });
        let mut entry_started = false;
        for line in user_config_file.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            if entry_started {
                //println!("entry: {}", &user_config.entries);
                user_config.entries = format!("{}\n\t{}", user_config.entries, line);
                continue;
            } else if line.starts_with('/') {
                if !entry_started {
                    user_config.entries.push_str(line);
                    entry_started = true;
                } else {
                    user_config.entries = format!("{}\n\t{}", user_config.entries, line);
                }
                continue;
            }
            if !line.starts_with("cmdline") {
                user_config.settings = format!("{}\n{}", user_config.settings, line);
            } else {
                user_config.cmdline = line.trim_start_matches("cmdline:").to_owned();
            }
        }
    }
    Some(user_config)
}
