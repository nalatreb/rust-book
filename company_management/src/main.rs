use std::{collections::HashMap, io, process::exit};

fn main() {
    println!("Welcome to the company management software");
    let mut company = HashMap::new();
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let mut split = input.split_whitespace();

        let command = String::from(split.next().unwrap_or(""));
        let argument_1 = String::from(split.next().unwrap_or(""));
        let argument_2 = String::from(split.next().unwrap_or(""));

        match command.as_str() {
            "exit" => exit(0),
            "add" => {
                company
                    .entry(argument_2)
                    .or_insert(Vec::new())
                    .push(argument_1);
            }
            "list" => {
                if argument_1 == "all" {
                    list_company(&company);
                } else if company.contains_key(&argument_2) {
                    list_department(&company[&argument_1], &argument_1);
                } else {
                    println!("Wrong department");
                }
            }
            _ => (),
        }
    }
}

fn list_company(company: &HashMap<String, Vec<String>>) {
    for (department, employees) in company {
        list_department(employees, department);
        println!("-------------------");
    }
}

fn list_department(employees: &Vec<String>, department: &String) {
    println!("{}", department);
    for employee in employees {
        println!("- {}", employee);
    }
}
