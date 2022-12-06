use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("L");

    let chars = contents.chars();
    let mut charVec: Vec<char> = vec![];
    for (i, chardriver) in chars.enumerate() {
        // println!("{:#?}", charVec);

        if charVec.len() == 100 {
            // println!("{:#?}", charVec);
            if !isUnique(&charVec) {
                charVec.remove(0);
            } else {
                println!("{i}");
                return;
            }
        }
        charVec.push(chardriver);
    }
}

fn isUnique(charVec: &Vec<char>) -> bool {
    let mut mySet = HashSet::new();

    for i in charVec {
        if mySet.contains(&i.to_owned()) {
            return false;
        }
        mySet.insert(i);
    }
    return true;
}