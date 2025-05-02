//用户状态
enum Status {
    Active,
    Inactive,
}

//用户结构体，包含姓名，余额（可能没有），状态
struct User {
    name: String,
    balance: Option<u64>,
    status: Status,
}
//查询余额（只允许Active 用户）
fn show_balance(user: &User) {
    match user.status {
        Status::Active => match user.balance {
            Some(amount) => println!("{}的账户是是{}元。", user.name, amount),
            None => println!("{}没有余额记录。", user.name),
        },

        Status::Inactive => {
            println!("{}的账户未激活，无法查看余额。", user.name);
        }
    }
}
//充值函数（只允许Active 用户充值）
fn deposit(user: &mut User, amount: u64) {
    match user.status {
        Status::Active => {
            //如果有余额实现累加；否则初始化
            user.balance = match user.balance {
                Some(current) => Some(current + amount),
                None => Some(amount),
            };
            println!("{}成功充值{}元。", user.name, amount);
        }
        Status::Inactive => {
            println!("{}是非激活账户，禁止充值。", user.name);
        }
    }
}

fn main() {
    let mut u1 = User {
        name: String::from("Alice"),
        balance: Some(100),
        status: Status::Active,
    };

    let mut u2 = User {
        name: String::from("Bob"),
        balance: None,
        status: Status::Inactive,
    };
    show_balance(&u1); //应该显示余额
    show_balance(&u2); //应提示未激活
    deposit(&mut u1, 50); //可充值
    deposit(&mut u2, 30); //不可充值
    show_balance(&u1);
}
