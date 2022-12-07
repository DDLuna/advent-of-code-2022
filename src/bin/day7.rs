use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

struct Dir {
    dirs: HashMap<String, Rc<RefCell<Dir>>>,
    files: HashMap<String, u32>,
    parent: Option<Rc<RefCell<Dir>>>,
}

impl Dir {
    fn new(parent: Option<Rc<RefCell<Dir>>>) -> Dir {
        Dir {
            dirs: HashMap::new(),
            files: HashMap::new(),
            parent,
        }
    }
    fn dir_size(&self) -> u32 {
        self.files.values().sum::<u32>()
            + self
                .dirs
                .values()
                .map(|v| v.borrow().dir_size())
                .sum::<u32>()
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day7.txt").unwrap();
    let root = parse_input(input);

    let mut total = 0;
    let total_space = 70_000_000;
    let space_available = total_space - root.borrow().dir_size();
    let space_needed = 30_000_000 - space_available;
    let mut smallest_to_delete = total_space;

    let mut dirs = vec![root];
    while !dirs.is_empty() {
        let current = dirs.pop().unwrap();
        let size = current.borrow().dir_size();
        if size <= 100_000 {
            total += size;
        }
        if size >= space_needed {
            smallest_to_delete = u32::min(smallest_to_delete, size);
        }
        dirs.append(&mut current.borrow().dirs.values().map(Rc::clone).collect());
    }
    println!("Part 1: {}", total);
    println!("Part 2: {}", smallest_to_delete);
}

fn parse_input(input: String) -> Rc<RefCell<Dir>> {
    let root = Rc::new(RefCell::new(Dir::new(None)));
    let mut lines = input.split("\n").peekable();
    let mut current_dir = Rc::clone(&root);
    while let Some(line) = lines.next() {
        let mut command = line.split_whitespace();
        match command.nth(1).expect("at least 2 words") {
            "cd" => {
                let dir_name = command.next().expect("cd should specify where");
                current_dir = match dir_name {
                    "/" => Rc::clone(&root),
                    ".." => Rc::clone(current_dir.borrow().parent.as_ref().unwrap()),
                    _ => Rc::clone(current_dir.borrow().dirs.get(dir_name).unwrap()),
                }
            }
            "ls" => {
                while let Some(line) = lines.peek() {
                    if line.starts_with("$") {
                        break;
                    }

                    let mut parts = line.split_whitespace();
                    match parts.next().unwrap() {
                        "dir" => {
                            let name = parts.next().unwrap();
                            current_dir.borrow_mut().dirs.insert(
                                name.to_string(),
                                Rc::new(RefCell::new(Dir::new(Some(Rc::clone(&current_dir))))),
                            );
                        }
                        number => {
                            let number: u32 = number.parse().unwrap();
                            let name = parts.next().unwrap();
                            current_dir
                                .borrow_mut()
                                .files
                                .insert(name.to_string(), number);
                        }
                    }
                    lines.next();
                }
            }
            _ => panic!("unrecognized command"),
        }
    }
    Rc::clone(&root)
}
