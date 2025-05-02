use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("欢迎来到猜字谜游戏！");
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("字谜的答案是{secret_number}");
    loop {
        println!("请输入你猜的数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜的数字{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("你赢了");
                break;
            }
        }
    }
}
