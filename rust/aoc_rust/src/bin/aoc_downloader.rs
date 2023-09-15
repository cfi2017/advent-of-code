use std::fs::{File, create_dir_all};
use std::io::prelude::*;
use std::path::Path;
use aocf::Aoc;

fn main() {
    let aoc_client = Aoc::new()
        .cookie_file("../../.aocf/cookie")
        .init()
        .unwrap()
        .year(Some(2018));

    let _ = create_dir_all("assets/input/2018");
    let _ = create_dir_all("assets/brief/2018");
    for i in 1..25 {
        println!("downloading day {}", i);
        get_day(aoc_client.clone(), i);
    }

}

fn get_day(aoc: Aoc, num: u32) {
    let aoc = aoc.day(Some(num));

    let brief = aoc.clone().get_brief(true).unwrap_or(String::from("Failed"));
    let _ = write_to_file(format!("assets/brief/{}/day{:02}.md", aoc.year.unwrap(), num), &brief);
    let input = aoc.clone().get_input(true).unwrap_or(String::from("Failed"));
    let _ = write_to_file(format!("assets/input/{}/day{:02}.txt", aoc.year.unwrap(), num), &input);
}

fn write_to_file<P: AsRef<Path>>(path: P, content: &str) -> std::io::Result<()> {
    File::create(path).map(|mut file| file.write_all(content.as_bytes())).unwrap()
}
