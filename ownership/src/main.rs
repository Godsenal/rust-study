fn main() {
    let s = String::from("hello");

    takes_ownership(s); // s가 takes_ownership으로 move 한다.
    // println!("{}", s); 그렇기 때문에 이 밑으로는 s를 사용할 수가 없다.
    let x = 5;

    makes_copy(x); // x는 스택에 저장되는 값이므로 copy 된다.
    // 따라서 여기서도 x를 사용할 수 있다.
    println!("{} 사용가능 ㅋ", x);

    let s = gives_ownership(); // some_string에 대한 ownership을 받는다.

    println!("{} 사용가능 ㅋ", s);  // 사용이 가능하다.

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // ownership을 주고 다시 돌려받는다.

    println!("{} 사용가능 ㅋ", s3); // 사용이 가능하다.

    let s1 = String::from("hiiii");
    let len = calculate_length(&s1); // reference를 사용해 빌려준다.
    // &를 사용하면 s1을 가리키는 s를 만드는 것이다. 

    println!("{} 사용가능 ㅋ 길이는 {}", s, len);

    let mut s1 = String::from("Hello");
    change(&mut s1); // reference도 mutable 하게 할 수 있다.
    // 이렇게 하면 reference를 바꿀 수 있다.
    // 한가지 제한이 있는데, 특정 스코프 안에서 단 하나의 mutable한 reference만을 가질 수 있다.
    // 이는 data race를 막아준다.

    let s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); 에러가 발생한다. word는 s에 대한 레퍼런스를 가지고 있다.

    println!("the first word of {} is {}", s, word);

    let s = "hello world";
    println!("the first word of {} is {}", s, first_word_slice(s));

    let a = [1, 2, 3];
    let slice = &a[0..3]; // 배열에도 사용 가능. 타입은 &[i32]
    for item in slice.iter() {
        println!("{}", item);
    }
}

fn takes_ownership(some_string: String) { // s가 some_string 으로 move 한다.
    println!("{}", some_string);
} // some_string이 drop되고 메모리가 해제된다. 즉, s 메모리가 해제된다.

fn makes_copy(some_integer: i32) { // x가 some_integer로 copy 된다.
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("zz");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

// reference를 사용하여, s를 move하지 않고 빌린다. 즉 소유권이 없다.
// s를 다시 리턴 (소유권을 반환) 해줄 필요가 없다.
fn calculate_length(s: &String) -> usize {
    s.len()
}
// 소유권이 없기 때문에 drop 할것도 없다.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {

//     let s = String::from("hello");

//     &s // we return a reference to the String, s
// } // s 가 스코프에서 벗어나서 메모리 해제된다.
// 따라서 dangling reference가 발생하기 때문에 컴파일 에러가 발생한다.

// 첫번째 단어를 찾아주는 함수이다. &str 타입은 string slice 타입에 해당한다.
// b' ' 는 byte 리터럴로, ' '에 해당하는 바이트를 의미한다.
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// String 대신 string slice를 사용하면 좀더 일반적인 함수로 만들 수 있다.
// string 리터럴도 허용해주기 때문
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}