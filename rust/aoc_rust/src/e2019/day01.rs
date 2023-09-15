pub fn solve(masses: Vec<i32>) -> (i32, i32) {

    // part one
    let initial_fuel = masses
        .iter()
        .map(fuel_for)
        .reduce(|sum, x| sum + x)
        .unwrap();

    // part two
    let total_fuel = masses
        .iter()
        .map(recursive_fuel_for)
        .reduce(|sum, x| sum + x)
        .unwrap();
    return (initial_fuel, total_fuel);
}

pub fn sanitise(i: String) -> Vec<i32> {
    return i.split("\n")
        .filter(|x| !x.is_empty())
        .map(str::parse::<i32>)
        .map(Result::unwrap).collect();
}

pub fn fuel_for(i: &i32) -> i32 {
    i / 3 - 2
}

pub fn recursive_fuel_for(i: &i32) -> i32 {
    let mut added = fuel_for(i);
    let mut sum = 0;
    while added > 0 {
        sum += added;
        added = fuel_for(&added);
    }
    return sum;
}
