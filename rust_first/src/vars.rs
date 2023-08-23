pub mod sub_b;
pub mod sub_a;

const MAX_POINTS: i32 = 100_000;

pub fn run() {
  println!("Here is vars module.");
  // sub_a::func_a();
  // sub_b::func_b();
  let mut x = 5;
  println!("x is {}", x);
  x = 6;
  println!("x is {}", x);
  let _f1 = 0.1;

  println!("{}", usize::BITS);
  println!("{:p}", &MAX_POINTS);

  let i2: i64 = 1;
  let i3: i64 = 2;
  println!("{:p}", &i2);
  println!("{:p}", &i3);

  let y = 5;
  println!("{:p}", &y);
  let y = y + 1;
  println!("{:p}", &y);
  let y = y * 2;
  println!("{:p}", &y);
  {
    let y = 0;
    println!("{}", y);
  }
  println!("y: {}", y);
  let t1 = (500, 6.4, "dummy");
  let (x, y, z) = t1;
  println!("{}, {}, {}", x, y, z);
  println!("{}, {}, {}", t1.0, t1.1, t1.2);

  let mut t2 = ((0, 1), (2, 3));
  let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
  *x1_ptr = 5;
  *y1_ptr = -5;
  println!("{:?}", t2);

  let a1 = [1, 2, 3, 4, 5];
  let a2 = [0; 10];
  println!("{:?}, {:?}, {}, {}", a1, a2, a1[2], a2[2])
}