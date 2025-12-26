use std::collections::HashMap;
use std::io::{self, Write};

fn prompt(msg: &str) {
    print!("{msg}");
    io::stdout().flush().unwrap();
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to Read Input");
    input.trim().to_string()
}

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("1) Add Employee, 2) List Department or 3) Quit");
        prompt("Choose an Option: ");

        let choice = get_user_input();

        match choice.as_str() {
           "1" => {
                prompt("Enter Employee Name: ");
                let name = get_user_input();

                prompt("Enter Employee Department: ");
                let department = get_user_input();

                employees.entry(department).or_insert(Vec::new()).push(name);
           } 
           "2" => {
               prompt("Enter Department to List Employees");
               let department = get_user_input();
               
               if let Some(list) = employees.get(&department) {
                   let mut list = list.clone();
                   list.sort();
                   println!("List of Employees for: {department}, {list:?}");
               } else {
                   println!("No Such Department");
               }
           }
           "3" => {
               println!("Exit");
               break;
           }
           _ => println!("Invalid Choice"),
        }
    }

}
