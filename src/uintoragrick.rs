use std::io::{self, Write};
use bcrypt::{hash, verify, DEFAULT_COST};
use rpassword::read_password;
use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub fn get_hashed_passwords() -> (String, String, String) {
    dotenv().ok();
    let password1 = env::var("PASSWORD1").expect("PASSWORD1 not found in .env");
    let password2 = env::var("PASSWORD2").expect("PASSWORD2 not found in .env");
    let password3 = env::var("PASSWORD3").expect("PASSWORD3 not found in .env");

    (password1, password2, password3)
}

pub fn userinterface() {
    // Define valid credentials
    let user1 = "Abolaji";
    let user2 = "Kazeem";
    let user3 = "Biola";

    println!("");
    println!("Please Login as Admin");
    println!("");

    // Hash the password
    let (password1_plain, password2_plain, password3_plain) = get_hashed_passwords();

    let password1_h = hash(password1_plain, DEFAULT_COST).unwrap();
    let password2_h = hash(password2_plain, DEFAULT_COST).unwrap();
    let password3_h = hash(password3_plain, DEFAULT_COST).unwrap();

    let mut attempt = 0;
    const MAX_ATTEMPT: u8 = 3;

    while attempt < MAX_ATTEMPT {
        // Prompt for username
        let mut username = String::new();
        print!("Enter your Username: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut username).unwrap();
        let username = username.trim(); // Trim the input to remove newline characters

        // Prompt for password
        print!("Enter your Password: ");
        io::stdout().flush().unwrap();
        let passwordinp = read_password().unwrap();

        // Check credentials
        if (username == user1 && verify(&passwordinp, &password1_h).unwrap())
            || (username == user2 && verify(&passwordinp, &password2_h).unwrap())
            || (username == user3 && verify(&passwordinp, &password3_h).unwrap())
        {
            println!("");
            println!("Login Successful!");
            println!("");
            println!("Welcome {}!", username);
            println!("");

            // Beginning of add and list employees
            let mut organizer: HashMap<String, Vec<String>> = HashMap::new();

            println!("Text Command User Interface To Add and List Employees");
            println!("");
            println!("Please enter command in the below format");
            println!("E.g - Add employee or 1 (to add new employee) or");
            println!("List Employee or 2 (to retrieve employees by department)");
            println!("List All Employee or 3 (to retrieve all employees in company)");

            loop {
                println!("");
                println!("1. Add Employee");
                println!("2. List [department]");
                println!("3. List all");
                println!("4. Exit or control c");
                println!("");

                    
                    
                    
                

                // Taking input from user to add or list
                let mut input_user = String::new();
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input_user).unwrap();
                let input_user = input_user.trim().to_lowercase();

                if input_user == "add employee" || input_user == "1" {
                    
                    println!("");
                    print!("New Employee First Name : ");
                    io::stdout().flush().unwrap();

                    let mut first_name = String::new();
                    io::stdin().read_line(&mut first_name).unwrap();
                    let first_name = capitalize(&first_name.trim().to_lowercase());

                    println!("");
                    print!("New Employee Second Name : ");
                    io::stdout().flush().unwrap();

                    let mut second_name = String::new();
                    io::stdin().read_line(&mut second_name).unwrap();
                    let second_name = capitalize(&second_name.trim().to_lowercase());

                    println!("");
                    print!("New Employee Department : ");
                    io::stdout().flush().unwrap();

                    let mut department_name = String::new();
                    io::stdin().read_line(&mut department_name).unwrap();
                    let department_name = capitalize(&department_name.trim().to_lowercase());

                    println!("");
                    add_employees(&mut organizer, first_name, second_name, department_name);
                } else if input_user == "list" || input_user == "2" {
                   

                    let mut department = String::new();
                    print!("Enter Department Name: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut department).unwrap();
                    let department = capitalize(&department.trim().to_lowercase());

                    println!(" ");
                    println!("Employee List in {}", department);
                    println!("");

                    list_department_name(&organizer, &department);
                } else if input_user == "list all" || input_user == "3" {
                    list_all(&organizer);
                } else if input_user == "exit" || input_user == "4" {
                    println!("");
                    println!("{}, You have successfully logged out", username );
                    println!("");
                    println!("You can login again as admin");
                    println!("");
                    break;
                }



                
                thread::spawn(|| {
                    let np = 0;
                    loop { 
                        if np <= 2 {
                        print!("\r This is Oragrick Proprietary");
                        thread::sleep(Duration::from_millis(200));
                        println!("");
                        thread::sleep(Duration::from_millis(50));

                        }else {
                            break;
                        }
                        break;
                        
                    }
                });
                
                println!("");
            }

            // End of add and list of employees
        } else {
            println!("");
            println!("Incorrect Username or Password");
            println!("");

            attempt += 1;

            if attempt == 1 {
                println!("You have 2 more attempts to login");
                println!("");
            }

            if attempt == 2 {
                println!("Last attempt to login");
                println!("");
            }
            if attempt >= MAX_ATTEMPT {
                println!("You have exceeded the maximum login attempt.");
                println!("");
                println!("Your account is locked.");
                println!("");
                println!("Please contact admin!");
                return;
            }
        }
    }
}

fn capitalize(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn add_employees(
    organizer: &mut HashMap<String, Vec<String>>,
    first_name: String,
    second_name: String,
    department_name: String,
) {
    let employees = organizer.entry(department_name.clone()).or_insert_with(Vec::new);
    employees.push(format!("{} {}", first_name, second_name));
    employees.sort();

    println!("First Name         Second Name        Department");
    println!("{}              {}          {}", first_name, second_name, department_name);
}

fn list_department_name(organizer: &HashMap<String, Vec<String>>, department: &str) {
    match organizer.get(department) {
        Some(employees) => {
            println!("");
            println!("First Name         Second Name        Department");
            for employee in employees {
                let names: Vec<&str> = employee.split_whitespace().collect();
                if names.len() == 2 {
                    println!("{}              {}          {}", names[0], names[1], department);
                }
            }
        }
        None => println!("No such department: {}", department),
    }
}

fn list_all(organizer: &HashMap<String, Vec<String>>) {
    for (department, employees) in organizer {

        println!("");
        println!("Department: {}", department);
        println!("");
        println!("First Name         Second Name        Department");
        
        for employee in employees {
            let names: Vec<&str> = employee.split_whitespace().collect();
            if names.len() == 2 {
                println!("{}              {}          {}", names[0], names[1], department);
            }
        }
        println!("");
    }
}
