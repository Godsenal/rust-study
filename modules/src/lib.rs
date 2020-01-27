// mod 키워드로 모듈을 정할 수 있다. 
mod front_of_house {
    // 모듈안에 모듈도 정할 수 있다.
    // pub을 붙여줘야 public 하게 사용할 수 있다.
    pub mod hosting {
        // 모듈의 컨텐츠도 public하게 사용하기 위해서는 pub키워드가 필요하다
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// use를 통해 현재 스코프에 name을 가져오는것은 기본적으로 private하다.
// pub use 를 통해 public 하게 만들어 re-exporting이 가능하다.
pub use crate::front_of_house::hosting;
// 위와 같이 pub use를 사용하면, 외부 코드도 hosting::add_to_waitlist(); 와 같이 사용할 수 있다.

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super를 통해 현재 path를 사용할 수 있다.
        // .. 와 같은 역할을 한다.
        super::serve_order();
    }

    fn cook_order() {}
    // struct와 property, 메서드도 pub 키워드를 사용할 수 있다.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // Breakfast의 seasonal_fruit가 pub이 아니기 때문에, 아래와 같은 메서드가 존재하지 않으면,
        // Breakfast 인스턴스를 만들어낼 수가 없다!
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    // enum은 특정 property가 private일 경우가 거의 없기 때문에
    // pub을 붙여주면 모든 property가 pub인게 default이다.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 아래 코드는 컴파일 되지 않는다.
    // public이 아니기 때문!
    // meal.seasonal_fruit = String::from("blueberries");

    // use 키워드로 해당 path를 현재 스코프에 넣어줄 수 있다.
    use front_of_house::hosting;
    // 이 아래로는 다음과 같이 사용 가능하다.
    hosting::add_to_waitlist();

    // 좀더 specific하게 사용할수도 있다.
    use front_of_house::hosting::add_to_waitlist;
    add_to_waitlist();

    /*
        use std::fmt::Result;
        use std::io::Result;
        이렇게 같은 이름일 경우는 당연히 사용할 수 없다.
        하지만, as 키워드를 사용하면 가능하다.
    */
    use std::fmt::Result;
    use std::io::Result as IoResult;

    /*
    이 코드와
    use std::cmp::Ordering;
    use std::io;
    이 코드는 같다.
    use std::{cmp::Ordering, io};
    */
    /*
    use std::io;
    use std::io::Write;
    // 위 코드는
    use std::io::{self, Write};
    // 이렇게 self 키워드를 사용할 수 있다.
    */
    // glob을 통해 모든 public 아이템을 가져올 수도 있다.
    use std::collections::*;
}

// 뒤에 세미콜론을 붙여주면, 모듈을 같은 이름의 다른 파일로 부터 가져온다는 의미다.
// 위 front_of_house의 hosting을 별도의 파일로 옮겨보자.
// 모듈을 export 해줄 별도의 파일 (밑에서 선언해준 이름과 같아야 한다.)을 만들고,
// 모듈 안에 모듈들은 같은 이름의 디렉토리 안에 만들어준다.
mod another_front_of_house;
pub use crate::another_front_of_house::hosting as another_hosting;

pub fn another_eat() {
    another_hosting::add_to_waitlist();
}