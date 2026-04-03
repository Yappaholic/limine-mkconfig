mod blkid;
mod config;
mod kernel;

fn main() {
    let kernels = kernel::find_installed_kernels();
    for kernel in kernels {
        println!("{} | {}", kernel.path, kernel.initramfs_path);
    }
}
