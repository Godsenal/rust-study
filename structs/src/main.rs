
// 이렇게 struct를 만들 수 있다.
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 튜플형식의 struct
struct Color(i32, i32, i32);

struct Rect(i32, i32);

struct Rectangle {
    width: i32,
    height: i32,
}
// struct에 메서드를 지정해줄 수 있다.
impl Rectangle {
    // 첫번째 인자는 항상 self다.
    // ownership 을 얻지 않기 위해 &를 썼다.
    // 값을 바꾸고 싶으면 &mut self 를 쓰면된다.
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// impl 을 두개 만들면 알아서 합쳐진다.
// 같은 이름의 메서드는 가질 수 없는 것 같다.
impl Rectangle {
    // associate fucntion은 Rectangle::square() 와같이 호출한다.
    // self를 인자로 받지 않는다.
    fn square(size: i32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    // 이렇게 인스턴스를 만들어 낼 수 있다.
    let user = User { 
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    let user = build_user(user.email, user.username);
    println!("{}", user.email);

    let user = User {
        email: String::from("other@example.com"),
        username: String::from("otheruser"),
        ..user // 이것도 가능하다. .이 2개다
    };

    // 튜플 형식의 struct를 이렇게 만들 수 있다.
    let color = Color(15, 20, 30);

    let rect = Rect(10, 10);

    println!("area is {}", calculate(&rect));

    // println!에서 {} 에 넣을 수 있으려면 Display가 구현되어있어야 한다.
    // 원시타입은 구현되어 있지만, Struct는 아니다.
    // 이럴 때 debug를 사용할 수 있다.
    #[derive(Debug)]
    struct Some {
        width: i32,
        height: i32,
    }

    let some = Some {
        width: 15,
        height: 15
    };

    println!("debug {:?}", some);
    println!("debug {:#?}", some); // 인덴트까지 표현해준다.

    let rect1 = Rectangle {
        width: 15,
        height: 15,
    };
    // area는 &self 를 받고 있는데, 이는 rust가 자동으로 레퍼런싱을 해주기 떄문이다.
    // 즉 아래 함수 호출은 (&rect1).area()와 같다.
    // mutable할때도 자동으로 인자값에 따라 함수를 어떻게 호출할지 결정하기 때문에
    // 호출할떄는 요렇게만 쓰면된다.
    println!("rectangle {}", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 10,
    };

    println!("can hold? {}", rect1.can_hold(&rect2));

    let associateRect = Rectangle::square(15);
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // 같은이름이면 이렇게 사용 가능하다.
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn calculate(rect: &Rect) -> i32 {
    rect.0 * rect.1
}