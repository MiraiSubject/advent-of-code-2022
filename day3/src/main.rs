use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Here's a file");
    let mut sum: u64 = 0;
    let mut firstElf = HashSet::new();
    let mut secondElf = HashSet::new();
    let mut thirdElf = HashSet::new();

    for (index, rucksack) in contents.split("\n").enumerate() {
        match index % 3 {
            0 => firstElf = rucksack.chars().collect(),
            1 => secondElf = rucksack.chars().collect(),
            2 => {
                thirdElf = rucksack.chars().collect();
                let intersect: HashSet<char> = firstElf.intersection(&secondElf).cloned().collect(); //.collect::<HashSet<&char>>();
                let intersect = intersect.intersection(&thirdElf);

                for item in intersect {
                    let groupnr = (index / 3) + 1;
                    println!("Group {groupnr}: {:#}", *item);
                    let priority = priority(*item);
                    sum += priority as u64;
                }
            }
            _ => panic!()
        }
        // if index % 3 == 0 {
        //     firstElf = rucksack.chars().collect();
        //     continue;
        // }

        // if index % 3 == 1 {
        //     secondElf = rucksack.chars().collect();
        //     continue;
        // }

        // if index % 3 != 2 {
        //     continue;
        // }

        // thirdElf = rucksack.chars().collect();
        // let intersect: HashSet<char> = firstElf.intersection(&secondElf).cloned().collect(); //.collect::<HashSet<&char>>();
        // let intersect = intersect.intersection(&thirdElf);

        // for item in intersect {
        //     let groupnr = (index / 3) + 1;
        //     println!("Group {groupnr}: {:#}", *item);
        //     let priority = priority(*item);
        //     sum += priority as u64;
        // }

        // let halfString = rucksack.len() / 2;
        // let firstHalf = &rucksack[..halfString];
        // let secondHalf = &rucksack[halfString..];

        // let firstHalf: HashSet<char> = firstHalf.chars().collect();
        // let secondHalf: HashSet<char> = secondHalf.chars().collect();
        // let mut result = secondHalf.intersection(&firstHalf);
        // let result = &rucksack[..].chars().collect();

        // finalResult += priority(*result.next().unwrap()) as u32
    }
    println!("The sum of the priorities is: {sum}");
}

fn priority(input: char) -> u8 {
    let number = input as u8;
    if input.is_lowercase() {
        return number - 96;
    }
    number - 64 + 26
}
