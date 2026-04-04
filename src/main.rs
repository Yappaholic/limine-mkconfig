mod blkid;
mod config;
mod kernel;

fn main() {
    let config = config::generate_config();
    println!("{config}");
}
