use std::fs;

fn main() {
    subject();
}

fn subject() {
    let contents = fs::read_to_string("./test.txt").expect("Here's a file");
    let mut totalCals = vec![];

    for elves in contents.split("\n\n") {
        let mut v = vec![];
        for elf in elves.split("\n") {
            let calorie: u32 = elf.trim().parse().unwrap();
            v.push(calorie);
        }
        let mut total = 0;
        for number in v {
            total += number
        }
        totalCals.push(total);
    }
    totalCals.sort_by(|a, b| b.cmp(a));
    println!("{:#?}",totalCals[0] + totalCals[1] + totalCals[2]);
}

#[cfg(test)]
mod test {
    #[bench]
    fn subjectTest() {
        subject();
    }
}