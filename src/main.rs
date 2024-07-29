use curl::easy::{Easy::{self}};
use std::fs::OpenOptions;
use std::io::{self, Write};

fn main () {
    let url = "https://ftp.lysator.liu.se/pub/archlinux/core/os/x86_64/linux-6.10.2.arch1-1-x86_64.pkg.tar.zst";
    let out = "./temp/linux.zst";
    let chunk = 1024 * 1024;

    let mut easy = Easy::new();
    easy.url(url).unwrap();

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(out)
        .expect("err")
        ;

    easy.write_function(move |data| {
        file.write_all(data)
            .unwrap();
        Ok(data.len())
    }).unwrap();

    easy.perform().unwrap();
}