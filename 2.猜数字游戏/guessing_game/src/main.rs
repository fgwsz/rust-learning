use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secter_number=rand::thread_rng().gen_range(1..101);

    let mut guess=String::new();//放到loop循环外,规避堆内存频繁分配

    loop{
        println!("Please input your guess.");

        guess.clear(); //清空

        io::stdin()
            .read_line(&mut guess) //注意:read_line是追加内容到字符串,而不是覆盖
            .expect("Failed to read line");

        let guess:u32=match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };

        println!("Your guessed: {}",guess);

        match guess.cmp(& secter_number){
            Ordering::Less=>println!("Too small!"),
            Ordering::Greater=>println!("Too big!"),
            Ordering::Equal=>{
                println!("You win!");
                break;
            }
        }
    }
}
