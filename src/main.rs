use bank_app::bank;

fn main() {
    let users = bank::greeting();
    dbg!(users);
}
