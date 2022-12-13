#![allow(dead_code)]

use std::{collections::VecDeque, fs::read_to_string, num::ParseIntError};

#[derive(Debug)]
enum Operation {
    Add(u128),
    Mul(u128),
    Square(),
}

#[derive(Debug)]
struct Test {
    divisor: u128,
    // T, F
    dest: (usize, usize),
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u128>,
    activity: u128,
    operation: Operation,
    test: Test,
}

impl Monkey {
    fn from(lines: Vec<&str>) -> Result<Self, &str> {
        if lines.len() != 5 {
            return Err("bad number of lines");
        };

        Ok(Monkey {
            activity: 0,
            items: lines[0][18..]
                .split(", ")
                .map(|n| n.parse::<u128>())
                .collect::<Result<VecDeque<u128>, ParseIntError>>()
                .map_err(|_| "Error parsing int")?,

            operation: if lines[1][25..].starts_with("old") {
                Operation::Square()
            } else {
                match &lines[1][23..24] {
                    "*" => Operation::Mul(lines[1][25..].parse().map_err(|_| "Error parsing int")?),
                    "+" => Operation::Add(lines[1][25..].parse().map_err(|_| "Error parsing int")?),
                    _ => return Err("Bad operator in line 2"),
                }
            },
            test: Test {
                divisor: lines[2][21..].parse().map_err(|_| "Error parsing int")?,
                dest: (
                    lines[3][29..].parse().map_err(|_| "Error parsing int")?,
                    lines[4][30..].parse().map_err(|_| "Error parsing int")?,
                ),
            },
        })
    }

    fn inspect(&self, worry_level: u128) -> u128 {
        match self.operation {
            Operation::Add(x) => x + worry_level,
            Operation::Mul(x) => x * worry_level,
            Operation::Square() => worry_level.pow(2),
        }
    }

    fn bored(worry_level: u128) -> u128 {
        worry_level / 3
    }

    fn test_result(&self, worry_level: u128) -> usize {
        if worry_level % self.test.divisor == 0 {
            self.test.dest.0
        } else {
            self.test.dest.1
        }
    }

    fn throw(&mut self) -> u128 {
        let item = self.items.pop_front().unwrap();
        let post_operation = self.inspect(item);
        self.activity +=1;
        Self::bored(post_operation)
    }
}

struct Throw {
    dest: usize,
    worry: u128,
}

fn main() {
    let mut monkeys: Vec<Monkey> = read_to_string("input.txt")
        .expect("Should have been able to read the file")
        .lines()
        // .take(15)
        .fold(vec![], |mut monkey_datas, line| {
            if line.starts_with("Monkey") {
                monkey_datas.push(vec![]);
            } else {
                if !line.is_empty() {
                    monkey_datas.last_mut().map(|last| last.push(line));
                }
            };

            monkey_datas
        })
        .into_iter()
        .map(|datas| Monkey::from(datas).unwrap())
        .collect();

    let length = monkeys.len();
    
    for _ in 0..20 {
        for i in 0..length {
            let length = monkeys[i].items.len();
            for _ in 0..length {
                let new_worry = monkeys[i].throw();
                let dst = monkeys.get(i).unwrap().test_result(new_worry);
                monkeys[dst].items.push_back(new_worry);
            }
        }
    }

    monkeys.iter().enumerate().for_each(|(i, monkey)| {
        print!("Monkey {i}: ");
        for worry in monkey.items.iter() {
            print!("{worry}, ");
        }

        print!("Monkey {i} inspected items {} times.", monkey.activity);

        println!("")
    });
}
