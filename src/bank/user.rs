use bcrypt::{DEFAULT_COST, hash};
use fake::Fake;
use rand::{self, Rng};
use uuid::Uuid;
use std::io::{self, stdin, Write};

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password_hash: String,
    pub year_of_birth: i16,
    pub month_of_birth: i8,
    pub day_of_birth: i8,
    pub account_number: i64,
    pub gender: char,
    pub current_user: bool,
    pub pin : i16,
}

impl User {
    pub fn new_user(
        first_name: &str,
        last_name: &str,
        email: &str,
        password: &str,
        gender: char,
        year_of_birth: i16,
        month_of_birth: i8,
        day_of_birth: i8,
    ) -> Self {

       
        let password_hash = hash(password, DEFAULT_COST).unwrap();

        Self {
            id: Uuid::now_v7(),
            email: email.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            password_hash,
            account_number: rand::rng().random_range(1_000_000_000..9_999_999_999),
            gender,
            year_of_birth,
            month_of_birth,
            day_of_birth,
            current_user: true,
            pin: Self::create_pin()
        }
    }


    pub fn create_pin()-> i16{
        print!("Create your new pin");
        io::stdout().flush().unwrap();

        let mut input_pin = String::new();

        stdin().read_line(&mut input_pin).expect("Failed to read_line");
        

    }

}
pub fn generate_users(count: u32) -> Vec<User> {
    let mut users = Vec::new();
    let mut rng = rand::rng();
    let genders = ['M', 'F'];

    for _ in 0..count {
        let first_name: String = fake::faker::name::en::FirstName().fake();
        let last_name: String = fake::faker::name::en::LastName().fake();

        let user = User {
            id: Uuid::now_v7(),
            email: format!(
                "{}.{}@example.com",
                first_name.to_lowercase(),
                last_name.to_lowercase()
            ),
            first_name,
            last_name,
            // Generating a dummy hash-like string
            password_hash: format!("$2b$12${}", fake::faker::lorem::en::Word().fake::<String>()),
            account_number: rng.random_range(1_000_000_000..9_999_999_999),
            gender: genders[rng.random_range(0..genders.len())],
            current_user: false,
            year_of_birth: rng.random_range(1900..2023),
            month_of_birth: rng.random_range(1..=12),
            day_of_birth: rng.random_range(1..=31),
            pin: rng.random_range(1_000..9_999)
        };
        users.push(user);
    }
    users
}
