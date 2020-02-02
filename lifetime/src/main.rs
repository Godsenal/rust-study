// struct가 레퍼런스를 가지게 하려면 lifetime을 지정해줘야한다.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let r; // r 의 lifetime 을 'a 라고 하자. 'a 가 여기서 시작된다.

    {
        let x = 5; // x의 lifetime 을 'b라고 하자. 'b 가 여기서 시작된다.
                   // r = &x; dangling reference!
                   // 'b 는 여기서 끝난다.
    }
    r = 5;

    println!("r: {}", r);
    // 'a는 여기서 끝난다.

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(&string1.as_str(), string2);

    /*
    아래 코드는 에러가 발생한다.
    string2의 lifetime이 아래 블록 스코프 안에 있으므로, result의 lifetime도 string2와 같아야하기 때문이다.
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    */
    // 'static lifetime은 프로그램 전체에서 살 수 있는 lifetime을 가진다.
    let s: &'static str = "I have a static lifetime.";
}

// x의 레퍼런스가 반환될지, y의 레퍼런스가 반환될지 알 수 없다.
// 즉, x, y의 lifetime 중 어떤 lifetime이 반환되는 lifetime과 같은지 알 수 없다.
/*
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

/*
    lifetime annotaion
    &i32; 레퍼런스
    &'a i32; lifetime이 명시된 레퍼런스
    &'a mut i32; lifetime이 명시된 mutable 레퍼런스
*/
// generic lifetime을 사용하여 위 함수를동작하게 만들었다.
// generic 'a는 x와 y의 lifetime이 적어도 'a 만큼은 지속된다는 것을 의미한다. (x, y중 더 작은 lifetime을 가진다)
// 그리고 반환타입도 'a 만큼은 지속된다는 것을 의미한다.
fn longest<'a>(x: &'a &str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
/*

    아래 함수는 에러가 나지 않는다.
    그 이유는 lifetime 생략 덕분이다.
    원래는 아래와 같이 되어야 하지만 세가지 규칙을 적용시켰을 때 추론이 가능하다면 생략이 가능하다.
    fn first_word<'a>(s: &'a str) -> &'a str {
    세가지 규칙은 다음과 같다.
    1. 각 인자는 각각의 lifetime을 가진다.
    2. 단 하나의 lifetime 인자가 존재할 경우 모든 결과값이 해당 lifetime과 같다.
    3. 여러 lifetime 인자가 존재하고, 그 중 하나가 &self 또는 &mut self라면 self의 lifetime이 모든 결과값의 lifetime에 해당된다.

    하나씩 적용시켜보자.
*/

// 1. 각 인자는 각각의 lifetime을 가진다.
fn first_word<'a>(s: &'a str) -> &str {}
// 2. 단 하나의 lifetime 인자가 존재할 경우 모든 결과값이 해당 lifetime과 같다.
fn first_word<'a>(s: &'a str) -> &'a str {}
// 3은 self가 없으므로 해당 X 다.

// longest에도 적용시켜보자.
// 1. 각 인자는 각각의 lifetime을 가진다.
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
// 2, 3은 적용되지 않는다.
// 모든 규칙을 적용시킨 후에도, 결과값의 lifetime을 알 수 없다.
// 따라서 명시적으로 lifetime을 지정해주어야 한다.

// 위에서 선언했던 lifetime을 가지는 struct에도 적용시켜보자.
impl<'a> ImportantExcerpt<'a> {
    // 3에 따라서 lifetime을 생략할 수 있다.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
