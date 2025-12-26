use std::collections::HashMap;
use std::io::{self, Write};

fn prompt(msg: &str) {
    print!("{msg}");
    io::stdout().flush().unwrap();
}

fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    input.trim().to_string()
}

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("1) add new employee");
        println!("2) list department");
        println!("3) quit");
        
        prompt("enter an option: ");
        let choice = input();

        match choice.as_str() {
            "1" => {
                prompt("enter employee name: ");
                let name = input();

                prompt("enter employee department");
                let dept = input();

                company.entry(dept).or_insert(Vec::new()).push(name);
            }
            "2" => {
                prompt("enter department to list");
                let dept_list = input();

                if let Some(list) = company.get(&dept_list) {
                    let list = list.clone().sort();
                    println!("{dept_list} department: {list:?}");
                } else {
                    println!("department does not exist");
                }
            }
            "3" => {
                println!("exit application");
                break;
            }
            _ => println!("invalid option... try again")
        }

    }
}
