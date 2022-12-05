use std::{fs, ops::{RangeInclusive}};

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Here's a file");
    let mut totalPairsRecon: u64 = 0;

    for assignments in contents.split("\n") {
        let mut pair = assignments.split(",").into_iter();
        let firstElf = pair.next().unwrap();
        let secondElf = pair.next().unwrap();

        let mut firstElf = firstElf.split("-").into_iter();
        let mut secondElf = secondElf.split("-").into_iter();

        let mut firstElf: RangeInclusive<u32> =
            firstElf.next().unwrap().parse().unwrap() ..=firstElf.next().unwrap().parse().unwrap();
        let mut secondElf: RangeInclusive<u32> =
            secondElf.next().unwrap().parse().unwrap() ..=secondElf.next().unwrap().parse().unwrap();

        //let (mut firstElf, mut secondElf) = dottyOutput(firstElf, secondElf);

        // First Star
        // if firstElf.start() >= secondElf.start() && firstElf.end() <= secondElf.end() {
        //     totalPairsRecon+=1;
        // } else if secondElf.start() >= firstElf.start() && secondElf.end() <= firstElf.end() {
        //     totalPairsRecon+=1;
        // }

        // Second Star
        let firstElfOverlap = firstElf.any(|x| secondElf.contains(&x));
        let secondElfOverlap = secondElf.any(|x| firstElf.contains(&x));

        if firstElfOverlap || secondElfOverlap {
            totalPairsRecon += 1;
            continue;
        }
    }
    println!("---------------------------");
    println!("Total pairs that need to be changed: {totalPairsRecon}");

}

fn dottyOutput(
    firstElf: RangeInclusive<u32>,
    secondElf: RangeInclusive<u32>,
) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let mut stdo = String::new();

    // Dotty output
    for item in 0..100 {
        if firstElf.contains(&item) || secondElf.contains(&item) {
            if item < 10 {
                stdo += &("0".to_owned() + &item.to_string() + ".");
            } else {
                stdo += &(item.to_string() + ".");
            }
        } else {
            stdo += "...";
        }
    }
    println!("{stdo}");

    (firstElf, secondElf)
}
