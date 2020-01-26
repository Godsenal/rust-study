fn main() {
    let v: Vec<i32> = Vec::new(); // 비어있는 벡터 생성
    let mut v = vec![1, 2, 3]; // vec! 매크로를 이용해 초기 값을 가지고있는 벡터 생성. 타입이 Vec<i32>로 infer 된다.

    v.push(5); // vector 업데이트트

    // vector 값을 읽는 방법
    let third: &i32 = &v[2]; // []를 이용하여 읽을 수 있다.
    match v.get(2) { // get을 이용하여 읽을 수 있다. 타입은 Option<T> 가 된다.
        Some(third) => println!("third element: {}", third),
        None => println!("No third element"),
    }

    // 둘의 차이점은 해당 index에 element가 없는 경우에 나타난다.
    // let not_exist = &v[100]; 에러가 발생하고, 프로그램이 종료된다.
    let not_exist = v.get(100); // Option 타입이기 때문에 None이 나오게 되어 에러가 발생하지 않는다.

    let first = &v[0];
    // v.push(6); 여기서 에러가 발생한다.
    // 이는 벡터의 동작 원리때문인데, 벡터에 새로운 아이템이 추가되면 벡터에 새로운 메모리 공간이 필요해질 수 있고,
    // 그렇게되면 이전 값들을 전부 다른 공간으로 옮기게 된다.
    // 즉, first는 할당 해제된 메모리를 가리키게 되므로, 에러가 발생한다.
    // 이를 고치기 위해서는,
    let mut first = &v[0];
    v.push(6);
    // first를 mutable로 가져와야한다.

    // 이렇게 iterating을 할 수 있다.
    for i in &v {
        println!("{}", i);
    }

    // mutable하게 하려면 다음과 같이한다.
    // mutable reference가 가리키는 곳의 값을 바꾸기 위해서는 (*) dereference 연산자를 사용해야 한다.
    for i in &mut v {
        *i += 50;
    }

    // 여러 타입의 값을 저장하기 위해서는 enum을 사용할 수 있다.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    /* string */
    // string을 만드는 방법들. String은 UTF-8로 인코딩되어야 한다.

    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s = String::from(data);
    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    // string 추가하기
    let mut s = String::from("Hello ");
    s.push_str("World"); // string을 추가할 수 있다.
    s.push('!'); // character도 추가할 수 있다.
    
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 더이상 사용이 불가능하다.
    // + 연산자는 기본적으로 add 메서드를 사용하는데, 이는 
    // fn add(self, s: &str) -> String { 이런식으로 되어있기 때문.
    // 즉, 소유권이 이전된다.
    // 여기서 &s2의 타입은 &String인데, 받는 타입은 &str이다.
    // 여기서는 rust가 강제 타입변환을 하기 때문. (&s2 -> &s2[..])
    
    // 아무 string의 소유권도 얻지 않는 방법도 있다.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    
    // format! 매크로를 사용하면 사용하기도 편하고 읽기도 쉽다.
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // s[0] - rust에서 string은 인덱싱을 허용하지 않는다.
    // String은 Vec<u8> 의 형태를 가지고 있는데,
    let len = String::from("Здравствуйте").len();
    // 위 코드는 len이 24이다. 이는 위 문자열을 UTF-8로 인코딩하는데 드는 바이트이다.
    let hello = "Здравствуйте";
    // let answer = &hello[0]; 이렇게 인덱싱을 할 경우 З(숫자 3이 아님)에 대해서 208이란 byte가 나오게 된다. (실제로는 2 바이트씩이기 때문에 208151이 된다.)
    // 하지만 208은 우리가 원하는 문자가 아니므로 헷갈릴 수 있기 때문에 애초에 인덱싱을 허용하지 않는다.
    // 따라서 이와 같은 코드도 에러가 난다.
    // &hello[0..1] 정상적으로 동작하게 하려면,
    println!("{}", &hello[0..2]); // 이렇게 해야한다.

    for c in hello.chars() { // chars 를 사용할경우 바이트 별이 아닌 유니코드 스칼라 값마다 iterating 한다.
        println!("{}", c);
    }

    for b in hello.bytes() { // 이렇게하면 byte별로 가져올 수 있다.
        println!("{}", b);
    }

    /* Hash Map */
    // 키 벨류 형태를 가지고 있는 데이터 구조.
    // HashMap<K, V> 형태이다.

    use std::collections::HashMap;
    
    let mut scores = HashMap::new(); // 생성
    scores.insert(String::from("Blue"), 10); // 삽입
    scores.insert(String::from("Yellow"), 50);

    // 벡터의 collect 메서드를 이용해서도 생성할 수 있다.
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // collect의 반환타입은 여러가지 경우가 될 수 있으므로 rust가 타입 추측을 할 수 없다.  
    // 대신 _, _를 사용하여 벡터로 부터 타입을 추측하게하도록 한다.

    let copy_val = 30; // copy가 가능한 값
    let owned_val = String::from("Owned!"); // 소유권이 이전되는 값값

    let mut map = HashMap::new();
    map.insert(copy_val, owned_val);
    println!("copy_val: {} 이건 사용 가능", copy_val);
    // owned_val은 소유권이 이전되기 때문에 사용 불가능하다.

    // map.get을 통해 가져온 값은 Option<T> 값을 가지게 된다.
    match map.get(&copy_val) {
        Some(exist) => println!("{}", exist),
        None => println!("not exist"),
    }
    // if let 도 기억하자 ㅋ
    if let Some(exist) = map.get(&copy_val) {
        println!("{}", exist);
    }
    // iterating도 가능하다.
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    // 값을 업데이트하기 위해서는 똑같은 키에 insert를 하면 된다.
    map.insert(copy_val, String::from("Updated"));
    println!("{:?}", map);

    // 값이 없을때만 업데이트 하고 싶을 경우 entry API의 or_insert를 사용할 수 있다.
    // entry는 key를 인자로 받아서 Entry enum을 반환해준다.
    // Entry enum에 있는 or_insert 메서드는는 해당하는 키에 대한 값이 존재할 경우 해당하는 값에 대한 mutable reference를 반환하고,
    // 아닌경우 해당 키에 대한 값을 넣어준 후 그 mutable reference를 반환한다.
    map.entry(copy_val).or_insert(String::from("Updated twice"));
    map.entry(10).or_insert(String::from("new Value"));
    println!("{:?}", map);

    // 기존 값을 통해 업데이트하기
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // word가 존재하지 않을 경우에만 0을 넣어준다.
        *count += 1; // 존재하면 해당 reference가 반환되므로 거기에 1을 더해준다.
    }

    println!("{:?}", map);

    // rust의 해싱 함수는 cryptographically strong 인데, 이는 Dos 공격에 더 안전하지만, 조금 느리다.
    // 다른 해싱 함수를 사용하고 싶다면 바꿀 수 있다.
}
