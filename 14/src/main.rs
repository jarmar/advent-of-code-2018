use std::char;
use std::collections::VecDeque;
use std::mem;

struct RecipeIterator {
    recipes: Vec<u32>,
    elf1_ix: usize,
    elf2_ix: usize,
    waiting: Option<u32>,
}

impl RecipeIterator {
    fn new(seed: &str) -> Option<Self> {
        let digits = seed
            .chars()
            .map(|c| c.to_digit(10))
            .collect::<Option<Vec<_>>>()?;
        Some(RecipeIterator {
            recipes: digits,
            elf1_ix: 0,
            elf2_ix: 1,
            waiting: None,
        })
    }
}

impl Iterator for RecipeIterator {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.waiting.is_some() {
            let mut taken_waiting = None;
            mem::swap(&mut self.waiting, &mut taken_waiting);
            return taken_waiting;
        }
        let elf1_recipe = self.recipes[self.elf1_ix] as usize;
        let elf2_recipe = self.recipes[self.elf2_ix] as usize;
        let sum = (elf1_recipe + elf2_recipe) as u32;

        let new_recipe = if sum > 9 { sum / 10 } else { sum };
        self.waiting = if sum > 9 { Some(sum % 10) } else { None };
        self.recipes.push(new_recipe);
        if let Some(waiting) = self.waiting {
            self.recipes.push(waiting);
        }
        self.elf1_ix = (self.elf1_ix + elf1_recipe + 1) % self.recipes.len();
        self.elf2_ix = (self.elf2_ix + elf2_recipe + 1) % self.recipes.len();
        Some(new_recipe)
    }
}

fn part1() -> String {
    let mut answer = String::new();
    let recipegenerator = RecipeIterator::new("37").unwrap();
    for x in recipegenerator.skip(330121 - 2).take(10) {
        answer.push(char::from_digit(x, 10).unwrap());
    }
    answer
}

fn part2() -> u32 {
    const NEEDLE: [u32; 6] = [3, 3, 0, 1, 2, 1];
    let mut recipegenerator = RecipeIterator::new("37").unwrap();
    let mut last_seen: VecDeque<u32> = (&mut recipegenerator).take(6).collect();
    for (i, x) in (3..).zip(recipegenerator) {
        last_seen.pop_front();
        last_seen.push_back(x);
        if last_seen == NEEDLE {
            return i;
        }
    }
    0
}

fn main() {
    let result_1 = part1();
    println!("answer 1: {}", result_1);
    let result_2 = part2();
    println!("answer 2: {}", result_2);
}
