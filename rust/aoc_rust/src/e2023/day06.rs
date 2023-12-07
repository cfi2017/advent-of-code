use num::ToPrimitive;
use crate::aoc_boilerplate;

/// I hate math
/// t_travelled = t_total - t_windup
/// d_travelled = t_travelled * t_windup
/// d_travelled = (t_total - t_windup) * t_windup
/// d_travelled = t_total * t_windup - t_windup^2
/// t_windup^2 - t_total * t_windup + d_travelled = 0
/// t1 = (t_total + sqrt(t_total^2 - 4 * d_travelled)) / 2
/// t2 = (t_total - sqrt(t_total^2 - 4 * d_travelled)) / 2

fn solutions(time: i64, distance: i64) -> i64 {
    let time = time as f64;
    let distance = (distance + 1) as f64;
    let t1 = (time + (time * time - 4.0 * distance).sqrt()) / 2.0;
    let t2 = (time - (time * time - 4.0 * distance).sqrt()) / 2.0;
    let t1 = t1.floor().to_i64().unwrap();
    let t2 = t2.ceil().to_i64().unwrap();
    t1 - t2 + 1
}

aoc_boilerplate!(2023, 6, sanitize_a, sanitize_b, solve_a, solve_b);

pub fn sanitize_a(input: &str) -> Vec<(i64, i64)> {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split(':').nth(1).unwrap();
    let distances = lines.next().unwrap().split(':').nth(1).unwrap();
    times.split(' ').filter(|x| !x.is_empty()).zip(distances.split(' ').filter(|x| !x.is_empty())).map(|(t, d)| (t.parse::<i64>().unwrap(), d.parse::<i64>().unwrap())).collect()
}

pub fn solve_a(races: Vec<(i64, i64)>) -> i64 {
    races.iter().map(|(t, d)| solutions(*t, *d))
        .reduce(|a, b| a * b).unwrap()
}

pub fn sanitize_b(input: &str) -> (i64, i64) {
    let mut lines = input.lines();
    let time = lines.next().unwrap().split(':').nth(1).unwrap().replace(' ', "").parse::<i64>().unwrap();
    let distance = lines.next().unwrap().split(':').nth(1).unwrap().replace(' ', "").parse::<i64>().unwrap();
    (time, distance)
}

pub fn solve_b((time, distance): (i64, i64)) -> i64 {
    solutions(time, distance)
}

#[cfg(test)]
mod tests {
    use crate::add_test;
    use super::*;

    add_test!(test_solve_a_example, sanitize_a, solve_a, r#"Time:      7  15   30
Distance:  9  40  200"#, 288);
    add_test!(test_solve_b_example, sanitize_b, solve_b, r#"Time:      7  15   30
Distance:  9  40  200"#, 71503);

}

