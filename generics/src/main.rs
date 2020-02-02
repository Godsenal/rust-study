struct Point<T> {
    x: T,
    y: T,
}
// Generic 타입을 갖는 Point<T> 에 대해 impl 할 수 있다.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// impl 뒤에 제네릭을 명시해주지 않으면 모든 제네릭 타입에 해당 메서드를 제공하는 것이아니라,
// f32 타입에만 해당 메서드를 제공하게 된다.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointTwo<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointTwo<T, U> {
    // 스트럭트의 제네릭 타입과 메서드의 제네릭 타입이 같을 필요는 없다.
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest<T>(list: &Vec<T>) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
