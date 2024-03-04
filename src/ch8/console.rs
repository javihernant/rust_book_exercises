use std::collections::HashMap;
use std::io::{self, Write};

enum Command {
    Add { name: String, department: String },
    ListDepartments,
    ListEmployees { department: Option<String> },
    Exit,
    Help,
}

fn build_command(line: String) -> Command {
    let words = line.split_whitespace().collect::<Vec<_>>();
    if words.len() == 0 {
        return Command::Help;
    }
    match words[0] {
        "add" => {
            if words.len() == 4 && words[2] == "to" {
                Command::Add {
                    name: words[1].to_owned(),
                    department: words[3].to_owned(),
                }
            } else {
                Command::Help
            }
        }
        "employees" => {
            if words.len() == 3 && words[1] == "in" {
                Command::ListEmployees {
                    department: Some(words[2].to_owned()),
                }
            } else if words.len() == 1 {
                Command::ListEmployees { department: None }
            } else {
                Command::Help
            }
        }
        "list" => {
            if words.len() == 2 && words[1] == "departments" {
                Command::ListDepartments
            } else {
                Command::Help
            }
        }
        "exit" => Command::Exit,
        _ => Command::Help,
    }
}

pub fn console() {
    let mut storage: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        print!("$> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let cmd = build_command(input);
        match cmd {
            Command::Add { name, department } => {
                println!("Adding {name} to {department}");
                let dps = storage.entry(department).or_insert(Vec::new());
                dps.push(name)
            }
            Command::ListDepartments => {
                println!("Printing all departments...");
                for (dep, _) in storage.iter() {
                    println!("{dep}");
                }
            }
            Command::ListEmployees { department } => {
                if let Some(department) = department {
                    println!("Printing employees in {department}");
                    if let Some(employees) = storage.get(&department) {
                        let mut employees = employees.to_owned();
                        employees.sort();
                        for e in employees {
                            println!("{e}")
                        }
                    }
                } else {
                    println!("Printing all employees...");
                    let mut employees = storage.values().flatten().collect::<Vec<&String>>();
                    employees.sort();
                    for e in employees {
                        println!("{e}")
                    }
                }
            }
            Command::Exit => {
                println!("Exiting...");
                break;
            }
            Command::Help => {
                println!("Commands available:");
                println!("add <name> to <department>");
                println!("list departments");
                println!("employees in <department>");
                println!("employees");
                println!("exit");
            }
        }
    }
}
