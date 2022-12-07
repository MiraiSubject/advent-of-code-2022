#![allow(unused_assignments)]

use std::fs;

#[derive(Debug)]
enum Node {
    File { size: u64, name: String },
    Directory { name: String, contents: Vec<Node> },
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Here's a file");

    let mut path = vec![];
    // [ "a", "e"]

    let mut root: Node = Node::Directory {
        name: "/".to_string(),
        contents: vec![],
    };

    let mut finalOutput = 0;

    for (index, line) in contents.split("\n").enumerate() {
        if line.contains("$") {
            let commandLine = line.split("$ ").last().unwrap();

            let command = commandLine.split(" ");
            let program = command.clone().nth(0).unwrap();
            let args = command.clone().last().unwrap();

            match program {
                "cd" => {
                    match args {
                        ".." => {
                            path.pop();
                        }
                        "/" => {
                            path.clear();
                        }
                        _ => {
                            path.push(args);
                        }
                    };
                }
                "ls" => {
                    let s = contents.split("\n").skip(index + 1);
                    //println!("Current directory: {}", fmtVec((*path).to_vec()));

                    let currentDir = subdir(path.clone(), &mut root);

                    for listing in s {
                        if listing.starts_with("$") {
                            break;
                        }

                        if listing.starts_with("dir") {
                            if let Node::Directory { name: _, contents } = currentDir {
                                contents.push(Node::Directory {
                                    name: listing.split("dir").last().unwrap().trim().to_string(),
                                    contents: vec![],
                                });
                            }

                            continue;
                        }

                        if let Node::Directory { contents, name: _ } = currentDir {
                            contents.push(Node::File {
                                size: listing.split(" ").nth(0).unwrap().parse::<u64>().unwrap(),
                                name: listing.split(" ").nth(1).unwrap().to_string(),
                            })
                        }
                        // println!("{}", lines);
                    }
                }
                _ => todo!(),
            };
        }
    }

    finalOutput = reccalc(&root);

    println!("{finalOutput}");
}

fn getCurrentDirectory(dirs: Vec<&str>) -> String {
    match dirs.last() {
        None => String::from("/"),
        Some(result) => String::from(result.clone()),
    }
}

fn fmtVec(dirs: Vec<&str>) -> String {
    let mut output = String::from("/").to_owned();

    for dir in dirs {
        output.push_str(&(dir.to_owned() + &"/".to_owned()));
    }
    output
}

// a
fn subdir<'a>(path: Vec<&'a str>, filesystem: &'a mut Node) -> &'a mut Node {
    let mut v = path.clone();

    if v.len() == 0 {
        return filesystem;
    }

    let dst = v.remove(0);

    if let Node::Directory { name: _, contents } = filesystem {
        let found = contents
            .iter_mut()
            .find(|x| {
                if let Node::Directory { name, contents: _ } = x {
                    name == dst
                } else {
                    false
                }
            })
            .unwrap();
        if v.len() > 0 {
            subdir(v, found)
        } else {
            found
        }
    } else {
        panic!()
    }
}

fn calcsize(dir: &Node) -> u64 {
    let mut output: u64 = 0;
    if let Node::Directory { name: _, contents } = dir {
        for list in contents {
            if let Node::File { size, name } = list {
                //println!("{name} is {size}");
                output += size;
            }
            if let Node::Directory { name, contents: _ } = list {
                output += calcsize(list);
                println!("{name}: {output}");
            }
        }
        output
    } else {
        output
    }
}

fn reccalc(dir: &Node) -> u64 {
    let mut output = 0;
    if let Node::Directory { name, contents } = dir {
        for listing in contents {
            if let Node::Directory { name, contents } = &listing {
                println!("{name}");
                output += reccalc(&listing);
            }
            let num = calcsize(&listing);
            if num <= 100000 {
                output += num;
            }
        }
    }
    output
}

#[test]
fn test() {
    let path = vec!["a", "b", "j"];

    let filesystem = Node::Directory {
        name: "/".to_owned(),
        contents: vec![Node::Directory {
            name: "a".to_string(),
            contents: vec![
                Node::File {
                    name: "file".to_string(),
                    size: 64,
                },
                Node::File {
                    name: "file 2".to_string(),
                    size: 64,
                },
                Node::Directory {
                    name: "b".to_string(),
                    contents: vec![Node::Directory {
                        name: "j".to_string(),
                        contents: vec![],
                    }],
                },
                Node::Directory {
                    name: "c".to_string(),
                    contents: vec![],
                },
                Node::Directory {
                    name: "d".to_string(),
                    contents: vec![],
                },
            ],
        }],
    };

    println!("{:#?}", subdir(path, &filesystem));
}
