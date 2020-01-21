fn main() {
    println!("Hello, world!");
    another_function(5, 10);
    println!("five fn {}", five());
}

// 함수 인자는 전부 타입 선언을 해줘야 한다.
fn another_function(x: i32, y: i32) {

    let z = {
        x + y // 세미콜론을 붙이지 않으면 expression이 된다.
    };
    println!("x is {} y is {} z is {}", x, y, z);
}

// 리턴 타입을 지정해준다.
// 리턴 타입이 없는 경우 타입은 () - empty tuple 이 된다.
fn five() -> i32 {
    5
}