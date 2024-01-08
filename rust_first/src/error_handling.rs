use std::path::MAIN_SEPARATOR;

pub fn run() {
  let res_1 = division_opition(10.0, 2.0);
  let res_0 = division_opition(10.0, 0.0);
  match res_1 {
    Some(x) => println!("res_1 is {}", x),
    None => println!("Not allowed.")
  }
  match res_0 {
    Some(x) => println!("res_0 is {}", x),
    None => println!("Not allowed.")
  }

  let res_2 = division_result(5.0, 2.0);
  let res_0 = division_result(5.0, 0.0);
  match res_2 {
    Ok(x) => println!("result is {}", x),
    Err(e) => println!("error {}", e)
  }
  match res_0 {
    Ok(x) => println!("result is {}", x),
    Err(e) => println!("error {}", e)
  }

  let a = [1, 2, 3];
  let res_a = sum(&a);
  match res_a {
    Some(x) => println!("{}", x),
    None => println!("None.")
  }
  let a_2 = [1, 2];
  let res_none = sum(&a_2);
  match res_none {
    Some(x) => println!("{}", x),
    None => println!("None.")
  }
}

fn division_opition (x: f64, y: f64) -> Option<f64> {
  if y == 0.0 {
    None
  } else {
      Some(x / y)
  }
}

fn division_result (x: f64, y: f64) -> Result<f64, String> {
  if y == 0.0 {
    Err(String::from("Not allowed."))
  } else {
    Ok(x / y)
  }
}

fn sum (a: &[i32]) -> Option<i32> {
  let a0 = a.get(0)?;
  let a1 = a.get(1)?;
  let a2 = a.get(2)?;
  Some(a0 + a1 + a2)
}