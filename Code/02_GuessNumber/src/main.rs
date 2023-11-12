use std::io;

// fn main() {
//     println!("Guess the number!");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin().read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {}", guess);
// }

use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("猜数字游戏");
    let secret_number = rand::thread_rng().gen_range(1,50);

    loop{

    println!("secret_number随机数为：{}",secret_number);
    println!("猜数字：");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("读取错误");

    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

        println!("你输入的数字为：{}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {println!("对了");
            break;
        }
    }
    }
}
