enum Status {
    Active,
    Inactive,
}
struct User {
    name: String,
    balance: Option<u64>,
    status: Status,
}
fn show_balance(user: &User) {
    match user.balance {
        Some(amount) => {
            println!("{}的余额是：{}元。", user.name, amount);
        }
        None => {
            println!("{}没有余额记录。", user.name);
        }
    }
}

fn main() {
    let u1 = User {
        name: String::from("Alice"),
        balance: Some(100),
        status: Status::Active,
    };

    let u2 = User {
        name: String::from("Bob"),
        balance: None,
        status: Status::Inactive,
    };
    show_balance(&u1);
    show_balance(&u2);
}
