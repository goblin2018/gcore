use std::{fs, io::Write, thread, time};

fn main() {
    println!("Hello, world!");

    thread::sleep(time::Duration::from_secs(5));
    let info = "this is test line";
    println!("{}", info);
    let mut file = fs::File::create("test.txt").expect("create failed");
    file.write_all(info.as_bytes()).expect("write error");
}
