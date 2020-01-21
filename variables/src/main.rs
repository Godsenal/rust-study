fn main() {
    let mut x = 4; // mut 키워드를 통해 mutable 하게 바꿀 수 있다.
    x = 5;

    const MAX_POINTS: u32 = 100_000;
    // const는 반드시 타입을 명시해주어야 한다.
    // 또한, runtime에 계산되는 값이나 함수의 반환 값은 지정될 수 없다.

    let x = 10;
    let x = 11;
    // 변수는 섀도우잉이 가능하다. 두번째 x가 첫번째 x를 가리게 된다.
    // 이는 mut 키워드로 만든 변수를 바꾸는 것과는 다르다.
    // 반드시 let을 붙여줘야 한다.

    println!("{}", x);

    let spaces = "  ";
    let spaces = spaces.len();
    // 이는 가능하다. 위 spaces와 아래 spaces는 완전히 다른 변수이다.

    // let mut spaces = "  ";
    // spaces = spaces.len();
    // 이는 불가능하다. 위 spaces와 아래 spaces의 타입이 다르다.

    println!("{}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    // 대부분의 경우 rust가 타입을 알아서 추측하지만
    // 위와 같이 String을 parse로 숫자 형태로 바꾸는 등
    // 많은 타입이 가능한 경우에는 타입을 명시해주어야 한다.

    let x: u32 = 2; // 정수 32bit
    let y: f32 = 3.0; // 소수 32bit
    // 비트를 넘는 정수가 들어갈 경우, 2의 보수에 따라
    // 만약 255가 맥스면 256은 0, 257은 1이 된다. (debug 모드에서는 에러가 난다.)

    /* Compound Type */
    let tup: (i32, f64, u8) = (500, 6.4, 1); // 튜플 타입. 선언된 후 바뀔 수 없다.
    let (x, y, z) = tup; // 이렇게 사용할 수 있고,
    let five_hundred = x.0; // 이렇게도 사용할 수 있다.

    let a = [1, 2, 3, 4, 5]; // 배열도 선언할 수 있다.
    // 튜플과 달리 모든 엘리먼트가 같은 타입을 가질 필요 없다.
    // 튜플과 같이 길이가 고정되어 있다.
    // Array는 고정된 길이와 타입을 가진 엘리먼트를 저장할때 유용하고, 스택에 저장된다.
    // 그렇지 않을때는 vector를 사용할 수 있다.
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // 타입과 길이를 정할 수 있다.
    let a: [3; 5]; // 같은 값으로(3) 정한 길이(5)만큼 초기화 할 수 있다.

    let first = a[0]; // 인덱스를 이용하여 접근 가능하다.
    // let index = 10;
    // let first = a[index]
    // 이는 compile 떄는 에러가 나지 않지만, runtime에 에러가 발생한다.
    // 다른 프로그래밍 언어들은 보통 다른 메모리를 참조하여 에러가 나지 않지만,
    // rust에서는 에러가 난다.

    another_function(); // rust에서는 보통 snake 케이스를 사용한다.
    // rust에서는 함수가 선언된 위치는 신경쓰지 않는다.
    // function project로 가보자.
}

fn another_function() {
    println!("another function.");
}
