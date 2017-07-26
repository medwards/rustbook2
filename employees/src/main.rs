use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::process::exit;

struct Company {
    employees: HashMap<String, Vec<String>>
}

impl Company {
    fn add_employee(&mut self, name: &String, department: &String) {
        let department = department.to_lowercase();
        if department == "all" {
            println!("'all' is an invalid department name");
            return;
        }
        let department = self.employees.entry(department.to_lowercase()).or_insert(vec!());
        department.push(name.clone());
    }

    fn list_department(&self, department: &String) {
        let department = department.to_lowercase();
        if department == "all" {
            let mut departments = self.employees.keys().collect::<Vec<&String>>();
            departments.sort();
            for department in departments {
                self.list_department(department);
            }
            return;
        }

        // why do I have to clone here? isn't to_vec also good?
        // review:
        // https://stackoverflow.com/questions/21369876/what-is-the-idiomatic-rust-way-to-copy-clone-a-vector-in-a-parameterized-functio
        let mut employees = match self.employees.get(&department.to_lowercase()) {
            Some(employees) => employees.clone(),
            None => {
                println!("Department '{}' does not exist", department);
                return;
            }
        };
        employees.sort();
        println!("Employees in {}", department.to_uppercase());
        println!("{}", employees.join(", "));
    }

    fn _demo(&mut self) {
        let eng = String::from("Engineering");
        let sales = String::from("Sales");
        let fin = String::from("Finance");
        self.add_employee(&String::from("Sally"), &eng);
        self.add_employee(&String::from("Bob"), &eng);
        self.add_employee(&String::from("Zelda"), &eng);
        self.add_employee(&String::from("Alice"), &sales);
        self.add_employee(&String::from("Xerxe"), &sales);
        self.add_employee(&String::from("Melissa"), &sales);
        self.add_employee(&String::from("Adam"), &fin);
    }
}

fn main() {
    // TODO: practice file IO by loading/dumping the company
    let mut company = Company{ employees: HashMap::new() };
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut command_line = String::new();
        // TODO: read Ctrl+D as empty string
        io::stdin().read_line(&mut command_line)
            .expect("Failed to read line");
        command(&command_line, &mut company);
    }
}

fn command(command: &String, company: &mut Company) {
    let command_parts: Vec<_> = command.trim().split(" ").map(|w| String::from(w)).collect();
    match command_parts[0].to_lowercase().as_ref() {
        // TODO: holy crap...
        "add" => match command_parts.len() {
            4 => match command_parts.get(2) {
                Some(to) => match to.as_ref() {
                    "to" => company.add_employee(&command_parts[1], &command_parts[3]),
                    _ => failed_add()
                },
                None => failed_add()
            },
            _ => failed_add()
        },
        "list" => company.list_department(&command_parts.get(1).unwrap_or(&String::from("all"))),
        "_debug" => println!("{:#?}", company.employees),
        "_demo" => company._demo(),
        // TODO: can this be shortened?
        "quit" => exit(0),
        "exit" => exit(1),
        _ => print_help()
    }
}

fn print_help() {
    println!("Commands:");
    println!("add NAME to DEPARTMENT");
    println!("list [DEPARTMENT|all]");
    println!("quit");
}

fn failed_add() {
    println!("Bad add command: add NAME to DEPARTMENT");
}
