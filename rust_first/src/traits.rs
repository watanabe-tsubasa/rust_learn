trait Fruits {
  fn price(&self) -> u32;
}

struct Apple;
impl Fruits for Apple {
  fn price(&self) -> u32 {
    10
  }
}

struct Banana;
impl Fruits for Banana {
  fn price(&self) -> u32 {
    5
  }
}

trait Summary {
  fn summarize(&self) -> String{
    String::from("(Read more..)")
  }
}

trait Message {
  fn message(&self) -> String {
    String::from("Message")
  }
}

struct NewsArticle {
  headline: String,
  location: String ,
  author: String ,
  content: String ,
}
impl Summary for NewsArticle {
  // fn summarize(&self) -> String {
  //   format!("{}, by {} ({})", self.headline, self.author, self.location)
  // }
}
impl Message for NewsArticle {
}

struct Tweet {
  username: String,
  content: String,
  reply: bool,
  retweet: bool,
}
impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
  }
}

pub fn run () {
  let apple = Apple {};
  let banana = Banana {};
  println!("apple price is {}, banana price is {}", get_price(apple), get_price(banana));
  let tweet = Tweet{
    username: String::from("watanabe"),
    content: String::from("今日はライザップに行きました"),
    reply: true,
    retweet: true,
  };
  println!("{}", tweet.summarize());
  let news_article = NewsArticle {
    headline: String::from("ノーコードハッカソンの開催"),
    location: String::from("Japan"),
    author: String::from("watanabe"),
    content: String::from("11/4, 5 にノーコードハッカソンが開催されます。\n主催者はもう少し真面目に集客をした方が良い"),
  };
  println!("{}", news_article.summarize());
  notify(&news_article);
  notify_another(&news_article);
  // notify_another(&tweet);
}

fn get_price<T: Fruits>(fruits: T) -> u32 {
  fruits.price()
}
fn notify(item: &impl Summary) {
  println!("Breking news! {}", item.summarize());
}
fn notify_another(item: &(impl Summary + Message)) {
  println!("Breaking news! {}", item.summarize());
  println!("Message! {}", item.message());
}