use std::{fs, vec};

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Here's a file");
    let mut stackVector: Vec<Vec<char>> = vec![];

    for (index, line) in contents.split("\n\n").enumerate() {
        //println!("{} {:?}", index, line);

        // if index is zero then we have the stack
        // if index is one then we have stack move instructions
        if index == 0 {
            let mut emptySpaceCounter = 0;

            let firstLine = line.lines().next().unwrap();

            for _ in 0..(firstLine.chars().count() + 1) / 4 {
                stackVector.push(vec![]);
            }

            for stackLine in line.lines() {
                let mut column = 0;

                for stackEntry in stackLine.split(" ") {

                    //Check whether current entry is empty 4 times
                    if stackEntry.is_empty() && emptySpaceCounter != 3 {
                        emptySpaceCounter += 1;
                        continue;
                    }

                    // Means previous 4 spaces are empty on the stack so ignore.
                    if stackEntry.is_empty() && emptySpaceCounter == 3 {
                        emptySpaceCounter = 0;
                        column += 1;
                        continue;
                    }

                    if stackEntry == "[" || stackEntry == "]" {
                        continue;
                    }

                    emptySpaceCounter = 0;
                    stackVector[column].push(stackEntry.chars().nth(1).unwrap());
                    column += 1;
                }
            }
        }

        if index == 1 {

            // Because the populating in index 0 was done in reverse.
            // cba to do it properly so I just reverse it here to correct for that.
            let iterVec = stackVector.clone();
            for (index, _) in iterVec.into_iter().enumerate() {
                stackVector[index].reverse();
            }

            for line in line.lines() {
                let mut instruction = line.split(" ");
                let select: usize = instruction.nth(1).unwrap().parse().unwrap();
                let src: usize = instruction.nth(1).unwrap().parse().unwrap();
                let dst: usize = instruction.last().unwrap().parse().unwrap();
                
                // Second Star
                let mut craneArm: Vec<char> = vec![];

                println!("Before picking up: {:#?}", stackVector);
                for _ in 0..select {
                    let topOfSource = stackVector[src - 1].pop().unwrap();

                    // Second Star
                    craneArm.push(topOfSource);

                    // First Star
                    //stackVector[dst - 1].push(topOfSource);
                    
                }
                println!("After picking up: {:#?}", stackVector);

                // Second Star
                craneArm.reverse();
                for boxs in craneArm {
                    stackVector[dst-1].push(boxs);
                }
            }
        }
    }

    let mut output = String::new();
    for entry in stackVector.iter() {
        output += &entry.last().unwrap().to_string();
    }
    println!("{:#?}", output);
}
