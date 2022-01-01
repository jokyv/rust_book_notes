fn main() {
    // TRAITS
    let tweet = Tweet {
        username: String::from("John"),
        content: String::from("Hello world this is Rust!"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        author: String::from("John K"),
        content: String::from("this is a news article - test"),
        location: String::from("Singapore"),
        headline: String::from("You are welcome!"),
    };
    println!("1 new article: {}", article.summarize());

    // GENERICS
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    let result1 = largest(&number_list);
    println!("the largest number is {}", result);
    println!("the largest number is {}", result1);

    let char_list = vec!['t', 'w', 'i', 't', 'c', 'h'];
    let result = largest_char(&char_list);
    let result1 = largest(&char_list);
    println!("the largest number is {}", result);
    println!("the largest number is {}", result1);

    // both are the same type
    // if it were different type then compiler would complain
    let _integer = Point { x: 0, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };

    // PointMultiple struct can handle two different type or same type
    let _integer = PointMultiple { x: 0, y: 10 };
    let _float = PointMultiple { x: 1.0, y: 4 };
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// This would override the default implementation of summarize
// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.content, self.author, self.location)
//     }
// }

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}

// can be different type
struct PointMultiple<T, U> {
    x: T,
    y: U,
}

// same type
struct Point<T> {
    x: T,
    y: T,
}

// function with generic type
// can take any type
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// only works with i32 lists
// function that finds the largest i32
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// function that finds the largest char
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
