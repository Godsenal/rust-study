// Traits: 공유할 행위(동작)을 정의

trait Summary {
    // 특정 행위의 타입을 정의한다.
    // 이 trait를 implement 하는 모든 타입은 이 행위를 정의해야 한다.
    fn summarize(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 타입에 trait를 implement
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

/*

// 주의해야할 점은 trait나 타입 둘 중하나는 local이여야 한다.
// 즉, trait나 타입 둘다 library에서 가져온 경우 적용시킬 수 없다.

// 요런건 안된다.
use std::clone::Clone;
use std::vec::Vec;
impl Clone for Vec {

}
*/

trait SummaryDefault {
    // default behavior 를 정의해놓을 수 있다.
    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl SummaryDefault for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
    // default behavior 는 정의하지 않아도 된다.
}

// item 파라미터에 대한 단단한 타입을 제공해주는 대신 trait를 제공해줄 수 있다.
// 이 trait 가 implement 된 모든 타입을 받을 수 있다.
fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 이 함수와 위 함수는 같다.
// impl 키워드를 인자로 사용하는 것은 이 함수에 대한 syntax sugar이다.
// 해당 trait 타입의 인자가 많은 경우에는 아래 방식이 깔끔할 수 있다.
fn notifySyntax<T: Summary>(item: T) {}

use std::clone::Clone;
use std::fmt::{Debug, Display};
// 두 trait가 implement된 인자를 받을 경우 아래와 같이 사용할 수 있따.
fn notifyPlus(item: impl Summary + Display) {}

// 너무 많은 trait bound를 사용하는 것은 함수를 읽기 어렵게할 수 있다.
// 이때 where를 사용해볼 수 있다.
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
// 아래와 같다.

fn some_function<T, U>(t: T, u: U) -> String
where
    T: Display + Clone,
    U: Clone + Debug,
{
    String::from("Hi")
}

// 아래와 같이 trait 를 리턴 타입으로도 사용할 수 있다.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// 하지만 다음과 같이 같은 trait를 implement 하여도 다른 타입을 리턴하는 경우는 작동하지 않는다.
/*
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
*/

trait Greet {
    fn sayHi(&self) -> ();
}
trait GetGreet {
    fn get(&self) -> String;
}
struct Hello {
    greeting: String,
}
impl GetGreet for Hello {
    fn get(&self) -> String {
        format!("{}", self.greeting)
    }
}
// 다른 trait를 implement한 모든 타입에 대해 trait를 implement 할 수 있다.
// 아래와 SetGreet가 implement 된 타입에 대해 Greet trait를 implement를 할 수 있다.
impl<T: GetGreet> Greet for T {
    fn sayHi(&self) {
        println!("{}", self.get())
    }
}

use std::cmp::PartialOrd;

// Copy trait가 있고, PartialOrd trait가 있는 경우 가장 큰 값을 찾는 함수를 만들 수 있다.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
        hockey team in the NHL.",
        ),
    };
    println!("New article available by {}", article.summarize());

    // i32, char 모두 largest 함수 이용이 가능하다.
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let greet = Hello {
        greeting: String::from("hello!"),
    };
    greet.sayHi();
}
