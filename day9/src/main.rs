use std::fs;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Coords {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct RopeBridge {
    head_pos: Coords,
    tail_pos: Coords,
    tails: [Coords; 9],
    tail_visited_coords: Vec<Coords>,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl RopeBridge {
    fn move_head(&mut self, direction: Direction, count: i32) {
        for _ in 0..count {
            match direction {
                Direction::Up => {
                    self.head_pos.y += 1;
                }
                Direction::Down => {
                    self.head_pos.y -= 1;
                }
                Direction::Left => {
                    self.head_pos.x -= 1;
                }
                Direction::Right => {
                    self.head_pos.x += 1;
                }
            }

            let mut temp_head = self.head_pos.clone();
            for tail in self.tails.iter_mut() {
                RopeBridge::move_tails(temp_head, tail);
                temp_head = tail.clone();

                // self.tail_visited_coords.push(*tail);
            }
            if !self.tail_visited_coords.contains(&self.tails.last().unwrap()) {
                self.tail_visited_coords.push(*self.tails.last().unwrap());
                continue;
            }

            // Part 1
            //self.move_tail();
        }
    }

    // Part 1
    fn move_tail(&mut self) {
        if (self.head_pos.x - self.tail_pos.x).abs() <= 1
            && (self.head_pos.y - self.tail_pos.y).abs() <= 1
        {
            return;
        }

        let temp_tail_pos = self.tail_pos.clone();

        if (self.head_pos.x - temp_tail_pos.x) == 1 {
            self.tail_pos.x += 1;
        }

        if (self.head_pos.y - temp_tail_pos.y) == 1 {
            self.tail_pos.y += 1;
        }

        if (self.head_pos.x - temp_tail_pos.x) == -1 {
            self.tail_pos.x -= 1;
        }

        if (self.head_pos.y - temp_tail_pos.y) == -1 {
            self.tail_pos.y -= 1;
        }

        if (self.head_pos.x - temp_tail_pos.x) == 2 {
            self.tail_pos.x += 1;
        }
        if (self.head_pos.y - temp_tail_pos.y) == 2 {
            self.tail_pos.y += 1;
        }

        if (self.head_pos.x - temp_tail_pos.x) == -2 {
            self.tail_pos.x -= 1;
        }

        if (self.head_pos.y - temp_tail_pos.y) == -2 {
            self.tail_pos.y -= 1;
        }

        if self.tail_visited_coords.contains(&self.tail_pos) {
            return;
        }

        self.tail_visited_coords.push(self.tail_pos);
    }

    // Part 2
    fn move_tails(head: Coords, tail: &mut Coords) {
        if (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1 {
            return;
        }

        let temp_tail_pos = tail.clone();

        if (head.x - temp_tail_pos.x) == 1 {
            tail.x += 1;
        }

        if (head.y - temp_tail_pos.y) == 1 {
            tail.y += 1;
        }

        if (head.x - temp_tail_pos.x) == -1 {
            tail.x -= 1;
        }

        if (head.y - temp_tail_pos.y) == -1 {
            tail.y -= 1;
        }

        if (head.x - temp_tail_pos.x) == 2 {
            tail.x += 1;
        }
        if (head.y - temp_tail_pos.y) == 2 {
            tail.y += 1;
        }

        if (head.x - temp_tail_pos.x) == -2 {
            tail.x -= 1;
        }

        if (head.y - temp_tail_pos.y) == -2 {
            tail.y -= 1;
        }
    }

    fn print_current_coord(&self) {
        println!("{:#?}", self);
    }
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Here's a file");

    let mut rope_bridge = RopeBridge {
        head_pos: Coords { x: 0, y: 0 },
        tail_pos: Coords { x: 0, y: 0 },
        tails: [Coords { x: 0, y: 0 }; 9],
        tail_visited_coords: Vec::from([Coords { x: 0, y: 0 }]),
    };

    for line in contents.split("\n") {
        let mut instructions = line.split(" ");
        let direction = instructions.next().unwrap();
        let direction_count = instructions.next().unwrap();

        match direction {
            "U" => {
                rope_bridge.move_head(Direction::Up, direction_count.parse().unwrap());
            }
            "D" => {
                rope_bridge.move_head(Direction::Down, direction_count.parse().unwrap());
            }
            "L" => {
                rope_bridge.move_head(Direction::Left, direction_count.parse().unwrap());
            }
            "R" => {
                rope_bridge.move_head(Direction::Right, direction_count.parse().unwrap());
            }
            _ => panic!(),
        }
    }

    rope_bridge.print_current_coord();
    println!("{}", rope_bridge.tail_visited_coords.len());
}
