use std::str::FromStr;
use std::fs::File;
use std::io::Read;

pub fn get_input_path(year: i32, day: i32) -> String {
    format!("assets/input/{}/day{:02}.txt", year, day)
}

pub fn read_input(year: i32, day: i32) -> anyhow::Result<String> {
    let mut file = File::open(get_input_path(year, day))?;
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn parse_ints<T>(input: &str, separator: &str) -> Vec<T>
    where T: FromStr, <T as FromStr>::Err: std::fmt::Debug
{
    input.trim().split(separator).map(str::parse::<T>).map(Result::unwrap).collect()
}
