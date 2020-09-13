use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {

  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101);
  // println!("The secret number is: {}", secret_number);

  loop {
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failted to read line");
    // 输入错误直接崩溃
    // let guess: u32 = guess.trim().parse().expect("Please type a number!");
    // 针对输入错误可进行逻辑处理
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        // println!("-------- Please input a number! -------");
        continue
      },
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),

      // rust的返回即函数的语法， 是不是有点像javascript？
      Ordering::Equal => {
        println!("You win!");
        break;
      }

    }
  }

}