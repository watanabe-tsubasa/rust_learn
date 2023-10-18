#[derive(Debug)]
struct User {
  username: String,
  email: String,
  login_counts: u64,
  active: bool,
}

#[derive(Debug)]
struct Rectangle {
  width: u64,
  height: u64,
}
impl Rectangle {
    fn create (width: u64, height: u64) -> Self {
      Self { width, height }
    }
    fn area (&self) {
      println!("{}", self.width * self.height);
    }
}

pub fn run() {
  let mut user1 = User {
    username: String::from("watanabe"),
    email: String::from("t.watanabe@gmail.com"),
    login_counts: 1,
    active: true,
  };

  user1.email = String::from("t.watanabe2@gmail.com");
  println!("{:#?}", user1);

  let user2 = build_user(String::from("watanabe"), String::from("t.watanabe3@gmail.com"));
  println!("{:#?}", user2);

  let rect = Rectangle::create(20, 20);
  rect.area();
  println!("{:#?}", rect);
}

fn build_user (username: String, email: String) -> User {
  User {
    username: (username),
    email: (email),
    login_counts: (1),
    active: (true)
  }
}