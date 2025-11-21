pub mod account_option;
pub mod user;

use account_option::AccountOption;
use std::{collections::HashMap, io, io::Write};
use user::User;

fn all_users() -> Vec<User> {
    let mut new_users: Vec<User> = Vec::new();
    let generated_users = user::generate_users(20);

    new_users.extend(generated_users);
    let current_user = collect_user_information();
    let new_user = user::User::new_user(
        &current_user.get("First Name").unwrap(),
        &current_user.get("Last Name").unwrap(),
        &current_user.get("Email").unwrap(),
        &current_user.get("Password").unwrap(),
        current_user.get("Gender").unwrap().chars().next().unwrap(),
        current_user
            .get("Year of Birth")
            .unwrap()
            .parse::<i16>()
            .unwrap(),
        current_user
            .get("Month of Birth")
            .unwrap()
            .parse::<i8>()
            .unwrap(),
        current_user
            .get("Day of Birth")
            .unwrap()
            .parse::<i8>()
            .unwrap(),
    );
    new_users.push(new_user);
    return new_users;
}

fn collect_user_information() -> HashMap<String, String> {
    let mut user: HashMap<String, String> = HashMap::new();
    let mut counter = 0;
    let input_field = [
        "Email",
        "Password",
        "First Name",
        "Last Name",
        "Gender",
        "Year of Birth",
        "Month of Birth",
        "Day of Birth",
    ];

    loop {
        print!("Enter your {}: ", input_field[counter]);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        user.insert(input_field[counter].to_string(), input.trim().to_string());

        counter += 1;
        if counter == input_field.len() {
            break user;
        };
    }
}

pub fn greeting() -> Vec<User> {
    println!("Welcome to Hephzisoft Bank");
    println!("Register your account");

    all_users()
    user::User::create_pin();
}

// fn check_user_already_exist (users: &Vec<User>)-> bool
// {
//  users.iter().any(|user| user.current_user == true)
// }
pub fn account_options() {}
