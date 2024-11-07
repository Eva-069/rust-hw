// 1.1
fn main1() {
    let x: i32 = 5;
    println!("x is {}", x);
}

// 1.2
fn main2() {
    let mut x = 10;
    println!("x is {}", x);
    x = 15;
    println!("x is now {}", x);
}

// 1.3
fn main3() {
    let x: i32 = 5;
    let y: f64 = 10.0;
    println!("x is {}, y is {}", x, y);
}

// 1.4
fn main4() {
    let x: i32 = 5;
    let y: i32 = 10;
    println!("x + y = {}", x + y);
}

// 1.5
fn main5() {
    let tup = (500, 6.4, 1);
    let (_, y, _) = tup;
    println!("The value of y is: {}", y);
}

// 1.6
fn main6() {
    let arr = [1, 2, 3, 4, 5];
    println!("The first element is: {}", arr[0]);
}

// 1.7
fn main7() {
    let arr = [1, 2, 3, 4, 5];
    println!("The second element is: {}", arr[1]);
}

// 2.1
fn main8() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

// 2.2
fn main9() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

// 2.3
fn main10() {
    let mut count = 0;
    while count < 5 {
        println!("count is {}", count);
        count += 1;
    }
}

// 2.4
fn main11() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

// 2.5
fn main12() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

// 3.1
fn main13() {
    println!("{}", largest(2, 3));
}

fn largest(x: i32, y: i32) -> i32 {
    if x > y { x } else { y }
}

// 3.2
fn main14() {
    println!("5! = {}", factorial(5));
}

fn factorial(n: u32) -> u32 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}

// 3.3
fn main15() {
    println!("{}", fibonacci(5));
}

fn fibonacci(n: u32) -> u32 {
    if n < 2 { n } else { fibonacci(n - 1) + fibonacci(n - 2) }
}

// 3.4
fn main16() {
    let result = is_even(4);
    println!("Is 4 even? {}", result);
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// 3.5
fn main17() {
    let result = is_odd(3);
    println!("Is 3 odd? {}", result);
}

fn is_odd(n: i32) -> bool {
    n % 2 != 0
}

// 4.1
fn main18() {
    let x = 5;
    let y = &x;
    println!("y is {}", y);
}

// 4.2
fn main19() {
    let mut x = 5;
    let y = &mut x;
    *y += 1;
    println!("x is {}", x);
}

// 4.3
fn main20() {
    let x = 5;
    let y = &x;
    let z = &x;
    println!("y and z are {} and {}", y, z);
}

// 4.4
fn main21() {
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("x is {}", x);
}

// 4.5
fn main22() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// 5.1
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn main23() {
    let user1 = User {
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("Username is {}", user1.username);
}

// 5.2
struct Rectangle {
    width: u32,
    height: u32,
}

fn main24() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// 5.3
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main25() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}


// 5.4
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main26() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}

// 5.5
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main27() {
    let square = Rectangle::square(3);
    println!("Square dimensions: {} x {}", square.width, square.height);
}

// 6.1
enum IpAddrKind {
    V4,
    V6,
}

fn main28() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("We have two kinds of IP addresses: {:?} and {:?}", four, six);
}

// 6.2
enum IpAddr {
    V4(String),
    V6(String),
}

fn main29() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home: {:?}, loopback: {:?}", home, loopback);
}

// 6.3
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main30() {
    let msg = Message::Write(String::from("hello"));
    println!("Message: {:?}", msg);
}

// 6.4
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main31() {
    let five = Some(5);
    let six = plus_one(five);
    println!("six: {:?}", six);
}

// 6.5
fn main32() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

// 6.6
fn main33() {
    let some_number: Option<i32> = Some(7);
    if let Some(7) = some_number {
        println!("Lucky number seven!");
    }
}
// 7.1
fn main34() {
    let v = vec![1, 2, 3];
    println!("The third element is {}", v[2]);
}

// 7.2
fn main35() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("Vector contents: {:?}", v);
}

// 7.3
fn main36() {
    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{}", i);
    }
}

// 7.4
fn main37() {
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
    }
    println!("Updated vector: {:?}", v);
}

// 7.5
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main38() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("Spreadsheet row: {:?}", row);
}

// 8.1
use std::collections::HashMap;

fn main39() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("Scores: {:?}", scores);
}

// 8.2
fn main40() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("Scores: {:?}", scores);
}

// 8.3
fn main41() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("Updated scores: {:?}", scores);
}

// 8.4
fn main42() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word counts: {:?}", map);
}

// 9.1
fn main43() {
    let greeting_file_result = std::fs::File::open("hello.txt");
    match greeting_file_result {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(e) => println!("Failed to open file: {:?}", e),
    }
}

// 9.2
use std::fs::File;
use std::io::ErrorKind;

fn main44() {
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

// 9.3
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main45() {
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Failed to read username: {:?}", e),
    }
}

// 9.4
use std::fs;

fn read_username_from_file2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main46() {
    match read_username_from_file2() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Failed to read username: {:?}", e),
    }
}

// 10.1
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main47() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&number_list));
}

// 10.2
struct Point<T> {
    x: T,
    y: T,
}

fn main48() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("Points: ({}, {}), ({}, {})", integer_point.x, integer_point.y, float_point.x, float_point.y);
}

// 10.3
struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

fn main49() {
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!("Mixed point: ({}, {})", integer_and_float.x, integer_and_float.y);
}

// 11.1
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests1 {
    use super::*;
    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(2), 4);
    }
}

// 11.2
fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests2 {
    use super::*;
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}

// 11.3
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests3 {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(larger.can_hold(&smaller));
    }
}

// 12.1
use std::env;

fn main50() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}

// 12.2
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}
// 12.1
use std::env;
use std::fs;

fn main51() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}

// 12.2
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

// 13.1
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main52() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", rect.area());
}

// 13.2
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main53() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// 13.3
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main54() {
    let sq = Rectangle::square(3);
    println!("Square: {:?}", sq);
}

// 14.1
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main55() {
    let mut collection = AveragedCollection {
        list: vec![1, 2, 3],
        average: 2.0,
    };

    collection.add(4);
    println!("Average after adding 4: {}", collection.average());

    collection.remove();
    println!("Average after removing last element: {}", collection.average());
}

// 15.1
pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button: {}", self.label);
    }
}

fn main56() {
    let button = Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
    };

    button.draw();
}

// 15.2
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main57() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// 15.3
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a select box with options:");
        for option in &self.options {
            println!("- {}", option);
        }
    }
}

fn main58() {
    let select_box = SelectBox {
        width: 75,
        height: 20,
        options: vec![
            String::from("Yes"),
            String::from("No"),
            String::from("Maybe"),
        ],
    };

    select_box.draw();
}
// 12.1
use std::env;
use std::fs;

fn main51() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}

// 12.2
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

// 13.1
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main52() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", rect.area());
}

// 13.2
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main53() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

// 13.3
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main54() {
    let sq = Rectangle::square(3);
    println!("Square: {:?}", sq);
}

// 14.1
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main55() {
    let mut collection = AveragedCollection {
        list: vec![1, 2, 3],
        average: 2.0,
    };

    collection.add(4);
    println!("Average after adding 4: {}", collection.average());

    collection.remove();
    println!("Average after removing last element: {}", collection.average());
}

// 15.1
pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button: {}", self.label);
    }
}

fn main56() {
    let button = Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
    };

    button.draw();
}

// 15.2
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn main57() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// 15.3
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a select box with options:");
        for option in &self.options {
            println!("- {}", option);
        }
    }
}

fn main58() {
    let select_box = SelectBox {
        width: 75,
        height: 20,
        options: vec![
            String::from("Yes"),
            String::from("No"),
            String::from("Maybe"),
        ],
    };

    select_box.draw();
}

