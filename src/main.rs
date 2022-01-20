use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::process;

fn main() {
    let hidden_number: u8 = rand::thread_rng().gen_range(1..101);
    println!("请猜测一个数字");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("请输入正确的值");

        let input: u8 = input.trim().parse().expect("无法转换");

        match input.cmp(&hidden_number) {
            Ordering::Less => {
                println!("太小了");
                continue;
            }
            Ordering::Greater => {
                println!("太大了");
                continue;
            }
            Ordering::Equal => {
                println!("你赢了✌️");
                process::exit(1);
            }
        }
    }
}
