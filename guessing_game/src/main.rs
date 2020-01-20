use std::io;
// rust는 기본적으로 prelude에 있는 몇가지 타입들을 제공한다.
// 다만 이 타입에 속해있지 않은 것들을 사용하기 위해서는 위와 같이 불러와야 한다.
use rand::Rng;
// Rng는 random number generator 가 구현된 메서드다.
use std::cmp::Ordering;
// Ordering은 Less, Greater, Equal을 가지는 enum이다.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new(); // mutable
        //let guess = String::new(); // immutable - 기본
    
        // 위에서 use std:io를 입력하지 않았으면, std::io::stdin().~~ 라고 써야한다.
        // read_line은 유저가 standard input에 입력한 것을 받아서 문자열에 넣어준다.
        // &는 reference를 뜻한다.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        // read_line은 io::Result 라는 return 타입을 가지고 있다.
        // Result 타입은 enum 타입으로, Ok와 Err이라는 고정값을 가지고 있다.
        // 모든 타입들과 마찬가지로 메서드를 가지고 있는데,
        // io::Result의 경우 반환값이 Err일때 실행되는 expect라는 메서드를 가지고 있다.
        // 위 코드에서 read_line이 Err을 반환할 경우 
        // expect를 실행하며 위 메시지를 출력하고 프로그램을 종료한다.
    
        // match를 integer와 integer의 비교로 하기 위해서 parse한다.
        // shadowing이 가능하다.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);
        // {}는 placeholder로 println에 다음 인자로 들어오는 값들을 넣어준다.
    
        // match는 컨트롤 제어 연산자로, 어떤 값을 연속속적인 패턴과 비교한 후 맞는 패턴에 대해 코드를 실행할 수 있게 해주는 연산자다.
        // guess와 secret_number를 비교하여 아래 패턴 중 맞는 패턴에 해당하는 코드를 실행한다.
        // cmp 메서드를 실행하면, Ordering enum 중 하나의 값이 나온다.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
}
