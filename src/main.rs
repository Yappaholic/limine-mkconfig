mod blkid;

fn main() {
    let version = blkid::get_library_version();
    println!("libblkid version is: {}", version);
}
