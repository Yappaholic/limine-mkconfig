mod blkid;

fn main() {
    let boot_device = blkid::get_mounted_boot_device();
    println!("Mounted boot device is: {}", boot_device);
}
