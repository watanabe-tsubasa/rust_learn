pub fn run() {
  let s1 = String::from("hello");
  let s2 = s1;
  println!("{}", s2);

  let i1 = 1;
  let i2 = i1;
  println!("{} {}", i1, i2);
  println!("Stack address of i1 is {:p}", &i1);
  println!("Stack address of i2 is {:p}", &i2);

  let sl1 = "leteral";
  let sl2 = sl1;
  println!("{} {}", sl1, sl2);
  println!("Stack address of sl1 is {:p}", &sl1);
  println!("Stack address of sl2 is {:p}", &sl2);

  let s3 = String::from("hello");
  let s4 = s3.clone();
  println!("Stack address of s3 is {:?}", &s3.as_ptr());
  println!("Stack address of s4 is {:?}", &s4.as_ptr());

  let s5 = String::from("hello");
  println!("Stack address of s5 is {:p}", &s5);
  println!("Heap address of s5: {:?}", s5.as_ptr());
  println!("Len is :{}", s5.len());
  println!("Cap is: {}", s5.capacity());
  take_ownership(s5); 

  let s6 = String::from("hello");
  println!("stack address of s6 is {:p}", &s6);
  println!("Heap memory address of hello is {:?}", s6.as_ptr());
  println!("Len of s6 is {}", s6.len());
  let s7 = take_giveback_ownership(s6);
  println!("stack address of s7 is {:p}", &s7);
  println!("Heap memory address of hello is {:?}", s7.as_ptr());
  println!("Len of s7 is {}", s7.len());

  let s8 = String::from("hello");
  let len = calculate_length(&s8);
  println!("The length of {} is {}", s8, len);

  let mut s9 = String::from("hello");
  change(&mut s9);
  println!("{}", s9);

  let s10 = String::from("hello");
  let r1 = &s10;
  let r2 = &s10;
  println!("s10: {}, r1: {}, r2: {}", s10, r1, r2);

  // let mut s10 = String::from("hello");
  // let r1 = &s10;
  // let r2 = &mut s10; // mutable, immutableな参照は共存できない
  // println!("{} {} ", r1, r2)

  let mut s11 = String::from("hello");
  let r1 = &mut s11;
  println!("{}", r1);
  println!("{}", s11);

  let mut s12 = String::from("hello");
  let r1 = &s12;
  let r2 = &s12;
  println!("{} {} ", r1, r2);
  let r3 = &mut s12;
  *r3 = String::from("hello_update");
  println!("{}", r3);

}

fn take_ownership(s: String) {
  println!("Stack address of s is {:p}", &s);
  println!("Heap address of s: {:?}", s.as_ptr());
  println!("Len is :{}", s.len());
  println!("Cap is: {}", s.capacity());
  println!("{}", s);
}

fn take_giveback_ownership(s: String) -> String {
  s
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn change(s: &mut String) {
  s.push_str("_world");
}

