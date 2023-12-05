use std::collections::HashMap;
use good_lp::{default_solver, Expression, Solution, SolverModel, Variable, variable, variables};

use crate::aoc::Puzzle;

pub struct Day14;

#[derive(Debug, Clone)]
pub struct Recipe {
    pub quantity: i32,
    pub ingredients: HashMap<String, i32>,
}

impl Puzzle<HashMap<String, Recipe>, f64, i32, 2019, 14> for Day14 {
    fn sanitize_input(&self, input: &str) -> HashMap<String, Recipe> {
        // each line in input has the format "12 ABC, 1 DEF => 2 GHI"
        input.lines()
            .map(|line| {
                let mut parts = line.split(" => ");
                let ingredients = parts.next().unwrap();
                let result = parts.next().unwrap();
                let ingredients = ingredients.split(", ")
                    .map(|ingredient| {
                        let mut parts = ingredient.split(' ');
                        let quantity = parts.next().unwrap().parse().unwrap();
                        let name = parts.next().unwrap().to_string();
                        (name, quantity)
                    })
                    .collect::<HashMap<_, _>>();
                let mut parts = result.split(' ');
                let quantity = parts.next().unwrap().parse().unwrap();
                let name = parts.next().unwrap().to_string();
                (name, Recipe { quantity, ingredients })
            })
            .collect()
    }

    fn solve_a(&self, input: HashMap<String, Recipe>) -> f64 {
        // instrument good_lp
        variables! {
            vars:
                1 <= ore;
                1 <= fuel;
        }
        let constraints: HashMap<String, (Variable, Recipe)> = input.iter().map(|(name, recipe)| {
            let var = if name == "FUEL" {
                fuel
            } else {
                vars.add(variable().name(name).min(0))
            };
            (name.clone(), (var, recipe.clone()))
        }).collect();

        // we want to minimise ore use
        let mut problem = vars.minimise(ore)
            .using(default_solver);

        for (name, (var, recipe)) in &constraints {
            let ingredients = recipe.ingredients.iter()
                .map(|(name, q)| {
                    // one of the ingredients is ore
                    if name == "ORE" {
                        return (ore, q);
                    }
                    return (constraints.get(name).unwrap().0, q)
                })
                .fold(Expression::from(0), |acc, (n, q)| {
                    // rest of the ingredients + (current ingredient * quantity)
                    acc + (n * *q)
                });

            println!("{}: {:?} * {:?} = {:?}", name, var, recipe.quantity, ingredients);
            // desired component * quantity = ingredients
            problem = problem.with((recipe.quantity * *var).eq(ingredients));
        }

        // we want 1 fuel
        problem = problem.with(Expression::from(1).eq(fuel));

        let solution = problem.solve().unwrap();
        solution.value(ore)
    }

    fn solve_b(&self, input: HashMap<String, Recipe>) -> i32 {
        input.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc::Puzzle;

    #[test]
    fn test_solve_a() {
        let input = r"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        let input = super::Day14.sanitize_input(input);
        assert_eq!(super::Day14.solve_a(input), 31.0);

    }

}
