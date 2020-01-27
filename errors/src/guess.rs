pub struct Guess {
  value: i32,
}
impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {}.", value);
    }
    Guess { value }
  }
  // getter와 같은 역할을 한다.
  // 이는 Guess 구조체의 value가 private하기 때문인데, private한 이유는
  // new를 이용하지 않고 직접적으로 값을 넣어주는 것을 방지하기 위해서이다.
  pub fn value(&self) -> i32 {
    self.value
  }
}
