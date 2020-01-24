


fn main() {
    // 이렇게 enum을 만들어줄 수 있다.
    // 사용할때는 ::을 사용하여 프로퍼티를 사용할 수 있고, 프로퍼티들의 타입 전부 IpAddrKind 이다.
    // 즉, fn route(ip_kind: IpAddrKind) { } 이런식으로 가능하다
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    
    // 위 코드를 enum만을 이용하여 만들 수도 있다.
    // 값을 직접 넣어줄 수 있다.
    enum IpAddrKind2 {
        V4(String),
        V6(String)
    }

    let home = IpAddrKind2::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind2::V6(String::from("::1"));

    // 아무 타입이나 다 넣어줄 수 있다.
    struct Ipv4Addr {
        address: (i32, i32, i32, i32),
    }

    struct Ipv6Addr {
        address: String,
    }

    enum IpAddr2 {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
    // struct 처럼 impl을 통해 메서드를 넣어줄 수도 있다.
    impl IpAddr2 {
        fn call(&self) {
            //
        }
    }
    // rust에는 null 값대신 Option 이라는 enum이 존재한다. (null이 없는 대신 null에서 필요한 부분만 가져왔다)
    // Option은 prelude로 이미 포함되어 있기 때문에 바로 사용가능하다.
    // Some과 None 은 Option의 property이다. 
    // Option의 경우,
    // enum Option<T> { Some: T, None }
    // 이렇게 되어있는데, 제네릭을 이용하여 Some일 경우 타입 추측이 가능하지만, None일 경우 타입을 명시해주어야 한다.
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; 이는 동작하지 않는다. T 와 Option<T>는 다른 타입이다.
    // Option<T> 가 None이 아님을 확실히 해야 T로서 계산을 할 수 있다.
    // 이는 match를 통해 할 수 있다.

    let sum = match y {
        Some(exist) => Some(x + exist),
        None => None // enum의 모든 케이스에 대해 명시적으로 작성해줘야 한다.
    };

    match sum {
        Some(exist) => println!("sum {}", exist),
        None => ()
    }
    // enum과 match는 찰떡궁합

    // match에서 아무것도 매치되지 않을때에 대한 매칭은 _을 이용하여 할 수 있다.
    // 필요한 케이스들을 위에 작성하고, 뒤에 _를 작성하면, 위 케이스에서 매치되지 않는 경우를 컨트롤 할 수 있다.
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // 그런데 Some과 None 이렇게 두개만 있는 경우에는 match를 매번 쓰기 귀찮을 수도 있다.
    // 이럴 땐 if let을 사용해볼 수 있다.
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // 위 match와 if let은 같은 동작을 한다.
    // if let을 match의 syntax sugar 라고 생각하면 된다.
    // 각각 필요할 때 사용하자.
    // if let은 match와 달리 한 케이스에 대해서만 신경쓴다는 점이 큰 차이점이다.

    // 아래와 같이 if let 다음에 else도 사용 가능하다.
    // 이는 match에서 _를 쓰는것과 같다.
    let mut count = 0;
    // match 버전
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    // if let 버전
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
