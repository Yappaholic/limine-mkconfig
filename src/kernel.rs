use std::fs;
use std::path::PathBuf;
pub const KERNEL_PREFIX: [&str; 2] = ["kernel-", "vmlinuz-"];
pub const INITRAMFS_PREFIX: &str = "initramfs-";

// Kernel version can be "6.10.8-gentoo-dist", "6.15.9-cachyos", etc.
pub fn find_kernel_versions() -> Vec<String> {
    let modules_path = "/usr/lib/modules";
    let mut kernel_entries: Vec<String> = Vec::new();
    if let Ok(dir) = fs::read_dir(modules_path) {
        for dir_entry in dir {
            kernel_entries.push(dir_entry.unwrap().file_name().to_string_lossy().to_string());
        }
    }
    kernel_entries
}
#[derive(Default)]
pub struct Kernel {
    pub name: String,
    pub path: String,
    pub initramfs_path: String,
}

pub fn find_installed_kernels() -> Vec<Kernel> {
    let kernel_versions = find_kernel_versions();
    let mut kernels: Vec<Kernel> = Vec::new();

    for kernel_name in kernel_versions {
        let mut kernel: Kernel = Kernel {
            name: kernel_name.clone(),
            ..Default::default()
        };

        let mut kernel_name1 = PathBuf::from("/boot/");
        kernel_name1.push(format!("{}{}", KERNEL_PREFIX[0], kernel_name.as_str()));

        let mut kernel_name2 = PathBuf::from("/boot/");
        kernel_name2.push(format!("{}{}", KERNEL_PREFIX[1], kernel_name.as_str()));

        let mut initramfs_name = PathBuf::from("/boot/");
        initramfs_name.push(format!("{}{}.img", INITRAMFS_PREFIX, kernel_name.as_str()));
        //println!("{:#?}", initramfs_name.as_os_str());
        if let Ok(true) = fs::exists(&kernel_name1) {
            kernel.path = kernel_name1
                .clone()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();
        }
        if let Ok(true) = fs::exists(&kernel_name2) {
            if kernel.name.is_empty() {
                kernel.path = kernel_name2
                    .clone()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
            } else {
                eprintln!(
                    "Conflicting kernel binaries for {kernel_name:#?}:\n{kernel_name1:#?}\n{kernel_name2:#?}"
                );
                std::process::exit(1);
            }
        }
        if let Ok(true) = fs::exists(&initramfs_name) {
            kernel.initramfs_path = initramfs_name
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();
        } else {
            eprintln!("Could not find initramfs for kernel {kernel_name}, exiting..");
            std::process::exit(1);
        }
        if kernel.path.is_empty() {
            eprintln!("Could not find kernel images for {kernel_name}, exiting...");
            std::process::exit(1);
        }
        kernels.push(kernel);
    }
    kernels
}
