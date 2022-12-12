use std::fs;

pub struct TreeGrid {
    grid: Vec<Vec<u32>>,
    visibility_grid: Vec<Vec<TreeStatus>>,
}

enum TreeStatus {
    Visible,
    NotVisible,
    NotVisited,
}

impl TreeGrid {
    fn populate_trees(&mut self, line: &str) {
        let mut v = vec![];
        for char in line.chars() {
            if char.is_numeric() {
                v.push(char.to_digit(10).unwrap());
            }
        }
        self.grid.push(v);
    }

    // Generate visibility grid with edges pre set to true
    fn generate_v_grid(&mut self) {
        for (y_coords, vec) in self.grid.iter().enumerate() {
            let mut subgrid = vec![];
            for (x_coords, _) in vec.iter().enumerate() {
                // Edges
                if x_coords == 0
                    || y_coords == 0
                    || self.grid.len() - 1 == y_coords
                    || self.grid[0].len() - 1 == x_coords
                {
                    subgrid.push(TreeStatus::Visible);
                    continue;
                }
                subgrid.push(TreeStatus::NotVisited);
            }
            self.visibility_grid.push(subgrid);
        }
    }

    // Print visibility map
    fn print_v_grid(&self) {
        for y in self.visibility_grid.iter() {
            let mut line = String::new();
            for x in y {
                match x {
                    TreeStatus::Visible => {
                        line += "V";
                        continue;
                    }
                    TreeStatus::NotVisible => {
                        line += "N";
                        continue;
                    }
                    TreeStatus::NotVisited => {
                        line += "?";
                        continue;
                    }
                }
            }
            println!("{line}")
        }
    }

    // Set visibility for other trees
    fn set_visible(&mut self, visible: bool, x_coord: usize, y_coord: usize) {
        if visible {
            self.visibility_grid[y_coord][x_coord] = TreeStatus::Visible;
        } else {
            self.visibility_grid[y_coord][x_coord] = TreeStatus::NotVisible;
        }
    }

    // Check visibility of the tree across X and Y axis
    fn is_visible(&mut self, x: u32, y: u32, height: u32) -> bool {
        let x_coord: usize = x.try_into().unwrap();
        let y_coord: usize = y.try_into().unwrap();

        // check if at edge, cause if at edge it's visible
        if x_coord == 0
            || y_coord == 0
            || self.grid.len() - 1 == y_coord
            || self.grid[0].len() - 1 == x_coord
        {
            return true;
        }

        let is_visible = self.is_visible_x(x_coord, y_coord, height)
            || self.is_visible_y(x_coord, y_coord, height);

        self.set_visible(is_visible, x_coord, y_coord);

        is_visible
    }

    // Check the X-axis visibility
    fn is_visible_x(&self, x_coord: usize, y_coord: usize, tree_height: u32) -> bool {
        let left_of_tree = &self.grid[y_coord][..x_coord];
        let right_of_tree = &self.grid[y_coord][x_coord + 1..];
        // println!("({}, {})", x_coord+1, y_coord+1);
        // println!("{left_of_tree:#?}");
        // println!("{right_of_tree:#?}");

        let tallest_on_left = left_of_tree.iter().max().unwrap();
        let tallest_on_right = right_of_tree.iter().max().unwrap();
        //println!("{}", tree_height);
        // println!("{} > {} || {} > {}", tree_height, *tallest_on_left, tree_height, *tallest_on_right);
        // println!("-----");

        tree_height > *tallest_on_left || tree_height > *tallest_on_right
    }

    // Check the Y-axis visibility
    fn is_visible_y(&self, x_coord: usize, y_coord: usize, tree_height: u32) -> bool {
        let above_tree = &self.grid[..y_coord];
        let below_tree = &self.grid[y_coord..];

        //println!("{below_tree:#?}");

        let mut above_tree_vec = Vec::new();
        for t in above_tree {
            above_tree_vec.push(t[x_coord]);
        }
        // println!("For {} at ({},{}): {:#?}", tree_height, x_coord+1, y_coord+1, above_tree_vec);

        let mut below_tree_vec = Vec::new();
        for t in below_tree {
            below_tree_vec.push(t[x_coord]);
        }

        below_tree_vec.remove(0);

        let tallest_above = above_tree_vec.iter().max().unwrap();
        let tallest_below = below_tree_vec.iter().max().unwrap();

        // println!("{} > {} || {} > {}", tree_height, *tallest_above, tree_height, *tallest_below);
        // println!("-----");
        tree_height > *tallest_above || tree_height > *tallest_below
    }

    // Check visibility for all trees
    fn check_all_visible(&mut self) -> u32 {
        let mut output = 0;
        let binding = self.grid.clone();
        let iter = binding.iter();

        for (y_coords, vec) in iter.enumerate() {
            for (x_coords, height) in vec.iter().enumerate() {
                if self.is_visible(
                    x_coords.try_into().unwrap(),
                    y_coords.try_into().unwrap(),
                    *height,
                ) {
                    //println!("Visible: {height}");
                    output += 1;
                } else {
                    //println!("Not visible: {height}")
                }
            }
        }
        output
    }

    fn calc_highest_scenic_score(&mut self) -> u32 {
        let binding = self.grid.clone();
        let iter = binding.iter();

        let mut scores = Vec::new();

        for (y_coords, vec) in iter.enumerate() {
            for (x_coords, _) in vec.iter().enumerate() {
                let num = self.calc_scenic_score_for(
                    x_coords.try_into().unwrap(),
                    y_coords.try_into().unwrap(),
                );
                scores.push(num);
            }
        }
        *scores.iter().max().unwrap()
    }

    fn calc_scenic_score_for(&self, x_coord: usize, y_coord: usize) -> u32 {
        let tree = self.grid[y_coord][x_coord];

        if x_coord == 0
            || y_coord == 0
            || self.grid.len() - 1 == y_coord
            || self.grid[0].len() - 1 == x_coord
        {
            return 0;
        }

        let mut left_visible = 0;
        let mut right_visible = 0;
        let mut above_visible = 0;
        let mut below_visible = 0;

        let left_of_tree = &self.grid[y_coord][..x_coord];
        let mutable_left = &mut left_of_tree.to_owned().clone();
        let right_of_tree = &self.grid[y_coord][x_coord + 1..];
        
        let mut printleft = mutable_left.clone();
        printleft.reverse();

        mutable_left.reverse();

        for other_tree in mutable_left {
            if tree > *other_tree {
                left_visible += 1;
            }
            if tree <= *other_tree {
                left_visible += 1;
                break;
            }
        }

        // println!("{}: {:#?}", tree, printleft);
        // println!("{tree}: {right_of_tree:#?}");

        for other_tree in right_of_tree {
            if tree > *other_tree {
                right_visible += 1;
            }
            if tree <= *other_tree {
                right_visible += 1;
                break;
            }
        }

        let mut above_tree_vec = Vec::new();
        for t in &self.grid[..y_coord] {
            above_tree_vec.push(t[x_coord]);
        }
        above_tree_vec.reverse();

        for other_tree in above_tree_vec {
            if tree > other_tree {
                above_visible += 1;
            }
            if tree <= other_tree {
                above_visible += 1;
                break;
            }
        }

        let mut below_tree_vec = Vec::new();
        for t in &self.grid[y_coord..] {
            below_tree_vec.push(t[x_coord]);
        }

        below_tree_vec.remove(0);

        println!("{:#?}", below_tree_vec);

        for other_tree in below_tree_vec {
            if tree > other_tree {
                below_visible += 1;
            }
            if tree <= other_tree {
                below_visible += 1;
                break;
            }
        }

        // println!(
        //     "({}, {}): {} . {} . {} . {}",
        //     x_coord,
        //     y_coord,
        //     above_visible,
        //     right_visible,
        //     below_visible,
        //     left_visible
        // );

        above_visible * below_visible * left_visible * right_visible
    }
}

fn main() {
    let mut grid = TreeGrid {
        grid: Vec::new(),
        visibility_grid: Vec::new(),
    };

    let contents = fs::read_to_string("./input.txt").expect("Here's a file");
    for line in contents.split("\n") {
        grid.populate_trees(line);
    }

    grid.generate_v_grid();
    let number = grid.check_all_visible();
    grid.print_v_grid();

    let highscore = grid.calc_highest_scenic_score();
    println!("Part 1: {number}");
    println!("Part 2: {highscore}");
    // println!("{:#?}", grid.grid);
}
