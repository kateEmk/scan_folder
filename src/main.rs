use scan_folder::lib::*;
use scan_folder::lib::parsing::scan_folder;


fn main() {
    println!("Hello, world!");

    println!("{:?}",scan_folder(".", true));
}
