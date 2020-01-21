fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if 는 표현이기 때문에 이렇게도 사용할 수 있다.
    // if 블록 안에서 나올 수 있는 값들은 전부 같은 타입이여야 한다.
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("number: {}", number);
}
