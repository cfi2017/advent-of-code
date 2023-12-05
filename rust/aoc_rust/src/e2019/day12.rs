use std::str::FromStr;
use euclid::default::Vector3D;
use tinyvec::ArrayVec;
use crate::aoc::Puzzle;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid input")]
    InvalidInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Moons (ArrayVec<[Moon; 4]>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Moon {
    position: Vector3D<i32>,
    velocity: Vector3D<i32>,
}


impl Moon {
    fn new(position: Vector3D<i32>) -> Self {
        Moon { position, velocity: Vector3D::zero() }
    }

    pub fn update_gravity(&mut self, other: &Moon) {
        self.velocity.x += gravity_check(self.position.x, other.position.x);
        self.velocity.y += gravity_check(self.position.y, other.position.y);
        self.velocity.z += gravity_check(self.position.z, other.position.z);
    }

    pub(crate) fn update_position(&mut self) {
        self.position += self.velocity;
    }

    pub fn energy(&self) -> u64 {
        let pot = self.position.abs();
        let pot = pot.x + pot.y + pot.z;
        let kin = self.velocity.abs();
        let kin = kin.x + kin.y + kin.z;
        pot as u64 * kin as u64
    }
}

fn gravity_check(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
    }
}

impl FromStr for Moon {
    type Err = Error;

    // <x=-8, y=-10, z=0>
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(", ");
        let x = parts.next().unwrap()[3..].parse().unwrap();
        let y = parts.next().unwrap()[2..].parse().unwrap();
        let z = &parts.next().unwrap()[2..];
        let z = z[..z.len() - 1].parse().unwrap();
        Ok(Moon::new(Vector3D::new(x, y, z)))
    }
}

impl Moons {

    fn step_gravity_simple(&mut self) {
        for a in 0..self.0.len() {
            for b in a+1..self.0.len() {
                let moon_a = self.0[a];
                let moon_b = self.0[b];
                self.0[a].update_gravity(&moon_b);
                self.0[b].update_gravity(&moon_a);
            }
        }
    }

    fn step(&mut self) {
        // apply gravity (of other moons) to each moon
        self.step_gravity_simple();

        // apply velocity to each moon
        for moon in self.0.iter_mut() {
            moon.update_position();
        }
    }

    fn simulate(&mut self, steps: u64) {
        for _ in 0..steps {
            self.step();
        }
    }

    fn total_energy(&self) -> u64 {
        self.0.iter().map(|moon| moon.energy()).sum()
    }

    // lets do this less shitty
    // hypothesis: each axis cycles in some way
    // hypothesis: the axis never interact, so we can do each axis independently
    // hypothesis: find axis cycle lengths, then find the least common multiple of those
    fn find_cycle_lcm(&mut self) -> u64 {
        let x_positions = self.0.iter().map(|moon| moon.position.x).collect::<Vec<_>>();
        let y_positions = self.0.iter().map(|moon| moon.position.y).collect::<Vec<_>>();
        let z_positions = self.0.iter().map(|moon| moon.position.z).collect::<Vec<_>>();
        let x_cycle = find_cycle(x_positions);
        let y_cycle = find_cycle(y_positions);
        let z_cycle = find_cycle(z_positions);
        lcm(x_cycle, lcm(y_cycle, z_cycle))
    }

}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd_loop(a, b)
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn gcd_loop(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn gravity_step_simple(bodies: &mut [(i32, i32)]) {
    for a in 0..bodies.len() {
        for b in a+1..bodies.len() {
            let moon_a = bodies[a].0;
            let moon_b = bodies[b].0;
            bodies[a].1 += gravity_check(moon_a, moon_b);
            bodies[b].1 += gravity_check(moon_b, moon_a);
        }
    }
}

// todo: bugs if two bodies have the same position on an axis
#[deprecated]
pub fn gravity_step_sort(bodies: &mut [(i32, i32)]) {
    let len = bodies.len() as i32;
    // sort by position
    bodies.sort_by_key(|(p, _)| *p);
    bodies.iter_mut().enumerate().for_each(|(i, (_, v))| *v += (len - i as i32 - 1) - i as i32);
}

pub fn find_cycle(positions: Vec<i32>) -> u64 {
    // pair of positions and velocities
    let mut bodies: Vec<(i32, i32)> = positions.iter().map(|&p| (p, 0)).collect();
    // let mut velocities = vec![0; positions.len()];
    // let initial_state = velocities.clone();

    let mut steps = 0;
    loop {
        // apply gravity
        gravity_step_simple(&mut bodies);

        // apply velocity
        for (a, b) in &mut bodies {
            // for each body add velocity to position
            *a += *b;
        }

        steps += 1;
        // if all velocities are 0
        if bodies.iter().all(|&(_, v)| v == 0) {
            break
        }
    }
    // multiply by 2 because we only count half the cycle
    steps * 2
}

pub struct Day12;

impl Puzzle<Moons, u64, u64, 2019, 12> for Day12 {

    fn sanitize_input(&self, input: &str) -> Moons {
        let moons: Vec<Moon> = input.lines().map(|line| line.parse().unwrap()).collect();
        let arr: [Moon; 4] = moons[..4].try_into().unwrap();
        let vec = ArrayVec::from(arr);
        Moons(vec)
    }

    fn solve_a(&self, input: Moons) -> u64 {
        let mut moons = input;
        moons.simulate(1000);
        moons.total_energy()
    }

    fn solve_b(&self, input: Moons) -> u64 {
        let mut moons = input;
        moons.find_cycle_lcm()
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::Puzzle;

    #[test]
    pub fn test_parse() {
        let input = "<x=-1, y=0, z=2>";
        let moon: Result<super::Moon, super::Error> = input.parse();
        assert!(moon.is_ok());
        let moon = moon.unwrap();
        assert_eq!(moon.position.x, -1);
        assert_eq!(moon.position.y, 0);
        assert_eq!(moon.position.z, 2);
    }

    #[test]
    pub fn test_solve_a() {
        let day = super::Day12;
        let input = day.get_input();
        let input = day.sanitize_input(&input);
        let result = day.solve_a(input);
        println!("result = {}", result);
        assert_eq!(result, 7636);
    }

    #[test]
    pub fn test_solve_b() {
        let day = super::Day12;
        let input = day.get_input();
        let input = day.sanitize_input(&input);
        let result = day.solve_b(input);
        println!("result = {}", result);
        assert_eq!(result, 281691380235984);
    }

    #[test]
    pub fn test_gravity_step() {
        let mut bodies = vec![(1, 0), (2, 0), (3, 0), (4, 0)];
        super::gravity_step_simple(&mut bodies);
        assert_eq!(bodies, vec![(1, 3), (2, 1), (3, -1), (4, -3)]);
    }

    #[test]
    pub fn test_gravity_step_sort() {
        let mut bodies = vec![(4, 0), (2, 0), (3, 0), (1, 0)];
        super::gravity_step_sort(&mut bodies);
        assert_eq!(bodies, vec![(1, 3), (2, 1), (3, -1), (4, -3)]);
    }

    #[test]
    pub fn test_gravity_step_same_pos() {
        let mut bodies = vec![(4, 0), (2, 0), (2, 0), (1, 0)];
        super::gravity_step_simple(&mut bodies);
        assert_eq!(bodies, vec![(4, -3), (2, 0), (2, 0), (1, 3)]);

        // all the same position, nothing should change on this axis
        let mut bodies = vec![(2, 0), (2, 0), (2, 0), (2, 0)];
        super::gravity_step_simple(&mut bodies);
        assert_eq!(bodies, vec![(2, 0), (2, 0), (2, 0), (2, 0)]);

        // let mut bodies = vec![(4, 0), (2, 0), (2, 0), (1, 0)];
        // super::gravity_step_sort(&mut bodies);
        // assert_eq!(bodies, vec![(1, 3), (2, 0), (2, 0), (4, -3)]);
//
        // // all the same position, nothing should change on this axis
        // let mut bodies = vec![(2, 0), (2, 0), (2, 0), (2, 0)];
        // super::gravity_step_sort(&mut bodies);
        // assert_eq!(bodies, vec![(2, 0), (2, 0), (2, 0), (2, 0)]);
    }

}
