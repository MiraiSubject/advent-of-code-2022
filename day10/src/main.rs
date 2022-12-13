#![allow(dead_code)]

use std::fs;

#[derive(Debug)]
struct CPU {
    x: i32,
    cycle: u32,
    signal_strength: [i32;6]
}

#[derive(Debug)]
enum Program {
    Noop,
    Addx(i32),
}

impl CPU {
    fn exec(&mut self, program: Program) {
        match program {
            Program::Noop => {
                self.cycle();
            }
            Program::Addx(num) => {
                for _ in 0..2 {
                    self.cycle();
                }
                self.x += num;
            }
        }
    }

    fn cycle(&mut self) {
        self.cycle +=1;
        self.generate_display();
    }

    // fn cpu_debug_display(&mut self) {
    //     println!("Cycle: {0}", self.cycle);
    //     println!("Register X: {0}", self.x);
    //     // self.part_a_signal_strength();
    // }

    fn generate_display(&mut self) {
        // 1 = 0; 40 = 39; 
        // should do -1 mod 40
        // let row = self.cycle -1 / 40;
        let col = (self.cycle -1) % 40; // <-- current sprite position

        if self.x.abs_diff(col.try_into().unwrap()) <= 1 {
            print!("#");
        } else {
            print!(".");
        }
        if col == 39 {
            println!();
        }
    }

    fn part_a_signal_strength(&mut self) {
        match self.cycle {
            20 => self.signal_strength[0] = 20 * self.x,
            60 => self.signal_strength[1] = 60 * self.x,
            100 => self.signal_strength[2] = 100 * self.x,
            140 => self.signal_strength[3] = 140 * self.x,
            180 => self.signal_strength[4] = 180 * self.x,
            220 => self.signal_strength[5] = 220 * self.x,
            _ => {}
        };
    }

    fn display_total_strength(&self) {
        // println!("{:#?}", self.signal_strength);
        let mut output = 0;
        for strength in self.signal_strength {
            output += strength
        }
        println!("{}", output);
    }

    pub fn new() -> Self {
        CPU { x: 1, cycle: 0, signal_strength: [0;6] }
    }
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Here's a file");

    let mut cpu = CPU::new();

    for line in contents.split("\n") {
        let command_length = line.split(" ").into_iter().count();
        let mut command = line.split(" ");
        let mut arg = "";
        let program = command.next().unwrap();
        if command_length > 1 {
            arg = command.next().unwrap();
        }

        match program {
            "noop" => {
                cpu.exec(Program::Noop);
            }
            "addx" => {
                cpu.exec(Program::Addx(arg.parse().unwrap()));
            }
            _ => panic!("Not implemented"),
        };
    }

    cpu.display_total_strength();
}
