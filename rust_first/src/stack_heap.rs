enum List { 
  Node(i32, Box<List>),
  Nil,
}

pub fn run() {
  let t1:  (i64, String) = (10, String::from("hello"));
  println!("stack address of tuple data is: {:p}", &t1);
  println!("heap memory address of t1.1: {:?}", t1.1.as_ptr());
  println!("len of t1.1: {:?}", t1.1.len());
  println!("capacity of t1.1: {:?}", t1.1.capacity());
  let mut b1 = Box::new(t1);
  (*b1).1 += "world";
  println!("{} {}", b1.0, b1.1);
  println!("stack address of box pointer is: {:p}", &b1);
}