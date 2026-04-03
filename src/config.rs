use std::fs;
use std::path::PathBuf;

pub fn find_existing_config() -> PathBuf {
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
    result
}
