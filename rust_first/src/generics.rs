struct Point<T, U>{
  x: T,
  y: U,
}

impl<T, U>  Point<T, U>{
    fn mixup<V, W> (self, other: Point<V, W>) -> Point<T, W> {
      Point { 
        x: self.x,
        y: other.y
      }
    }
}

pub fn run() {
  let number_list = vec![1, 2, 3, 5, 12];
  let largest_number = largest(number_list);
  println!("{}", largest_number);

  let char_list = vec!['a', 'b', 'c', 'd', 'e'];
  println!("{}", largest(char_list));

  let p1 = Point { x: 1, y: 2 };
  let p2 = Point { x:1.2, y:2.4 };
  let p3 = p1.mixup(p2);
  println!("{}, {}", p3.x, p3.y);
}

fn largest<T: PartialOrd + Copy>(list:Vec<T>) -> T{
  let mut largest = list[0];
  for i in list {
    if largest < i {
      largest = i;
    }
  }
  largest
}