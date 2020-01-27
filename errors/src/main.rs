mod guess;
use guess::Guess;

fn main() {
    // panic!("crash and burn"); panic! 매크로를 통해 에러를 발생시킬 수 있다. 이를 통해 에러 발생된곳을 추적할 수 있다.

    let v = vec![1, 2, 3];

    // v[99]; 존재하지 않는 엘리먼트에 대한 접근이 일어날때 panic!이 발생된다.
    // 이를 추적하려면 RUST_BACKTRACE=1 cargo run 와 같이 RUST_BACKTRACE env를 설정해주면 된다.
    // RUST_BACKTRACE는 --release 플래그가 있을 경우에는 작동하지 않는다.

    use std::fs::File;

    let f = File::open("hello.txt"); // Result 타입이 반환된다.
                                     // let f = match f {
                                     //     // match를 통해 해당 파일이 존재하는지 확인할 수 있다.
                                     //     Ok(file) => file,
                                     //     Err(error) => panic!("Problem opening the file: {:?}", error),
                                     // };

    // 에러가 났을 때, 해당 파일이 존재하지 않는 경우 새로 만들어주는 동작
    use std::io::ErrorKind;
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
    // match 대신 closure를 사용한 방식
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // unwrap을 사용하면, 위에서 match로 해준것과 똑같은 동작을 해준다. Result가 Ok면 그대로 반환해주고, 아니면 panic!을 호출한다.
    let f = File::open("hello.txt").unwrap();
    // expect는 unwrap과 똑같이 동작하지만, 에러 메시지를 우리가 만들 수 있게 해준다.
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // 잘못된 값이 들어오는 것을 사전에 방지하기 위해서, 커스텀 타입을 만들어 줄 수 있다.
    // 이는 panic을 만들기 전에 타입 체킹에서 잘못된 값이 들어오는 것을 막아줄 수 있다.
    // guess 파일을 참고하자.
    let guess = Guess::new(10);
    let myVal = guess.value();
    println!("my Val: {}", myVal);
}

use std::fs::{read_to_string, File};
use std::io::{self, Error, Read};

// error propagating 하는 함수.
// 에러를 함수내에서 처리하는 것보다 반환해주는게 나은 경우가 있음.
// 함수를 호출하는쪽에서 에러 제어가 가능
fn read_username_from_file() -> Result<String, Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 이러한 패턴은 꽤 자주 요구되므로, ? 연산자를 통해 쉽게 만들 수 있다.
// ?와 위 match의 차이점은, ?로 부터 반환되는 Error 타입은 함수의 Return 타입으로 변환된다.
// 그렇기 때문에 ?를 사용하려면 함수의 반환 타입이 Result, Option 또는 std::ops::Try이 구현된 타입이어야 한다.
fn read_username_from_file_2() -> Result<String, Error> {
    let mut f = File::open("hello.txt")?; // ? 연산자를 사용한다.
                                          // 성공하면 값이 저장되고, 실패하면 Err를 함수에서 반환해준다.
    let mut s = String::new();
    f.read_to_string(&mut s)?; // 위와 마찬가지
    Ok(s)
}

// ?는 chaining도 가능하다.
fn read_username_from_file3() -> Result<String, Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 이렇게 파일에서 문자열을 읽어오는건 자주사용되는 작업이므로, 더 짧은 방법도 제공된다.
fn read_username_from_file4() -> Result<String, io::Error> {
    read_to_string("hello.txt")
}
