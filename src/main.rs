use std::process;
use std::env;
fn main () {
    let mut args: Vec<_> = env::args().collect();
    args.remove(0);
    println!("{}", process::id());
    process::Command::new("wget")
        .args(&args)
        .output()
        .expect("failed");
}