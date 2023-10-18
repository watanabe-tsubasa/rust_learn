pub fn run () {
  let st1 = String::from("x");
  let st2 = String::from("y");
  let longer = get_longer(&st1, &st2);
  println!("{}", longer);

  let st3 = String::from("x");
  let res;
  {
    let st4 = String::from("y");
    res = get_longer(&st3, &st4);
  }
  println!("{}", res);
}

fn get_longer<'a> (x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}