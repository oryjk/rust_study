// use std::fs::File;
// use std::io;
// use std::io::Read;
//
// use rand::Rng;
//
// mod hello_world;
// mod second_module;

// fn take_ownership(s: String) {
//     println!("{}", s);
// }
//
// fn make_copy(s: i32) {}
//
// fn calculte_length(s: &String) -> u8 {
//     println!("length is {}", s.len());
//     return 2;
// }
// fn first_world(s: &String) -> usize {
//     for (index, &value) in s.as_bytes().iter().enumerate() {
//         if (value == b' ') {
//             return index;
//         }
//     }
//     s.len()
// }

// use std::intrinsics::type_id;

use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;

fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{},{},{}", x, y, z);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    let b = [3; 5];
    println!("{:?}, {}", b, b[4]);

    let y = {
        let x = 1;
        x + 3//这个会作为表达式的返回值，表达式是有返回值的，而语句没有返回值
        //如果上面加一个分号，那么返回值就是这一行，这一行什么也没有，所以返回值就是（）
    };
    println!("{:?}", y);
    /*
    函数
     */
    fn five(x: i32) -> i32 {
        x + 5
    }
    println!("{}", five(6));

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    }

    match number {
        x if x % 4 == 0 => println!("number is divisible by 4"),
        x if x % 2 == 0 => println!("number is divisible by 2"),
        x if x % 3 == 0 => println!("number is divisible by 3"),
        _ => { println!("no!!!") }
    }

    let condition = true;
    let number = if condition { 5 } else { 4 };
    println!("number is {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number = number - 1;
    }

    let a = [1, 2, 3, 4, 5, 6, 7];
    for item in a.iter() {
        println!("{}", item);
    }

    let result = for number in (1..100).rev() {
        println!("{}", number);
        break;
    };
    println!("{:?}", result);

    let mut s = String::from("hello");
    println!("{}", s);
    s.push_str(", world");
    println!("{}", s);

    let s1 = gives_ownership();
    let s2 = String::from("haha");
    let s3 = takes_and_gives_back(s2);
    println!("{}", &s3);
    println!("{}", &s1);
    //当一个在heap上的数据变量离开作用域的时候，他的值就会被drop函数清理，除非数据的所有权移动到了另外一个变量上面
    //不在heap上的变量，比如int类型的，他是在stack上的，由于占用的空间很小，或者每次赋值都是一个copy的步骤，所以不存在
    //所有权转移的情况


    /*
    1. 不能在同一个作用域里面既有可变借用，又有不可变借用
    2. 不能在同一个作用域里面同时出现多个可变的借用
    3. 可以在同一个作用域里面同时存在多个不可变借用
     */
    let mut s = String::from("hhhh hhhh");
    let name = first_name(&mut s);
    // second_name(&mut s);
    println!("{}", name);

    let s = "hello world";
    let rectangle = Rectangle { length: 10, width: 20 };
    println!("area is {}", rectangle.area());

    let x = 12;
    let xxx = match x {
        10 => 10,
        11 => {
            println!("11");
            11
        }
        _ => 0,
    };
    println!("xxx {}", xxx);

    let v = Some(8);
    if Some(8) == v {
        println!("is 8")
    }

    let vec: Vec<i32> = Vec::new();
    let mut vec1 = vec![1, 2, 3];

    match vec1.get(1) {
        Some(value) => println!("{}", value),
        None => println!("none")
    }
    let result = &vec1[1];
    vec1.push(4);
    println!("{:?}", vec1);
    // println!("{}",result);
    for i in &mut vec1 {
        *i += 50;
    }
    println!("{:?}", vec1);

    let x = "hello";
    let mut string = x.to_string();
    string.push_str("hello");
    let x1 = string + x;
    // println!("{}",string);

    for xx in "王睿".chars() {
        println!("{}", xx);
    }

    for xx in "王睿".bytes() {
        println!("{}", xx);
    }

    let x2 = &"王睿"[0..3];
    println!("{}", x2);

    let mut map = HashMap::new();
    map.insert("hello", 12);
    let entry = map.entry("wang");
    println!("{:?}", entry);
    let entry1 = map.entry("hello");
    println!("{:?}", entry1);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);

    let v = vec![1, 2, 3, 4];
    // println!("{}", v[99]);

    let result = File::open("launch.json");
    println!("{:?}", result);
    let f = match result {
        Ok(file) => file,
        Err(error) => {
            panic!("{}", error);
        }
    };

    println!("{:?}", f);

    let arr = [1; 3];
    // let [q,w]=arr;
    let [q, w, ..] = arr;
    let [q, w, e] = arr;
    println!("{}-{}-{}", q, w, e);
    let xxx = Rectangle {
        width: 20,
        length: 30,
    };
    let Rectangle { width, length } = Rectangle {
        width: 40,
        length: 30,
    };

    let Rectangle { width, .. } = xxx;
    println!("{}-", width);
    println!("{}-{}", width, length);

    //栈中的数据会发生复制，而不会转移所有权：
    let s1 = "hello";
    let s2 = s1;
    println!("{}-{}", s1, s2);

    //存放在堆中的数据会发生所有权的转移：
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);
    // println!("{}",s1); 这句话会报错 borrow of moved value: `s1`，因为s1的所有权已经转移给s2了

    //对于单个作用域中的数据来说，一次只能声明一个可变引用：
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    // let s3 = &mut s1;//单个作用域中，都是在main函数的作用域中
    println!("{}", s2);
    // println!("{}",s3); cannot borrow `s1` as mutable more than once at a time

    /*
    引用的规则:
        1. 在任何一段给定的时间里，你要么只能拥有一个可变引用，要么只能拥有任意数量的不可变引用，试想一下如果数据改变了，那么不可变的变量的不可变特性是否还有意义。
        2. rust会保证引用总是有效的。
     */
}

struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
}

fn first_name(s: &String) -> &str {
    &s[..]
}

fn second_name(s: &mut String) {
    s.clear();
}

fn gives_ownership() -> String {
    String::from("hello")
}

fn takes_and_gives_back(string: String) -> String {
    string
}

// let coin = Coin::Xxxx;
// match coin {
//     Coin::Xxxx => println!("{}", "Xxxx"),
//     Coin::Yyyy => println!("{}", "Yyyy"),
// }
//
// if let Coin::Xxxx = coin {
//     println!("{}", "It is Xxxx")
// }
//
// enum Coin {
//     Xxxx,
//     Yyyy,
// }
// let rect = Rectangle {
//     width: 20,
//     length: 30,
// };
// println!("area is {}", rect.area());
// println!("square is {:?}", Rectangle::square(10));
//
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     length: u32,
// }
//
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.length * self.width
//     }
//
//     fn square(size:u32)->Rectangle{
//         Rectangle{
//             width:size,
//             length:size
//         }
//     }
// }
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// let color = Color(232, 21, 32);
// println!("{}", color.1);
// let user = User {
//     age: 21,
//     username: String::from("WangRui"),
// };
// println!("user -> {}", user.username);
//
// struct User {
//     username: String,
//     age: i8,
// }
// let s = String::from("hello world");
// let hello = &s[..5];//equal &s[0..5]
// let world = &s[6..];//equal &s[6..11]
// println!("hello = {}, world = {}", hello, world);


// let index = first_world(&s);
// println!("first world index {}", index);

// let mut s = String::from("hello");
// {
//     let s1 = &mut s;
//     s1.push_str(", world");
// }
// let s2 = &mut s;
// s2.push_str(" ,WangRui");
// println!("s2 = {}", s2)

// calculte_length(&s);
// println!("s 还在 {}", s);

// let s = String::from("hello");
// take_ownership(s);

// let i = 10;
// make_copy(i);
// println!("{}", i)
// println!("{}",s);

// let s1=String::from("hello");
// let s2=s1.clone();
// println!("{},{}",s1,s2);
// let mut s = String::from("hello");
// s.push_str(", world");
// println!("{}",s);
// let s="hello";
// for number in (1..4).rev() {
//     println!("{}", number)
// }

// let a = [1, 2, 3, 4, 5];
//
// for x in a {
//     println!("{}", x);
// }
//
// for element in a.iter() {
//     println!("{}", element);
// }

// let mut counter = 0;
// let result = loop {
//     counter = counter + 1;
//
//     println!("hello");
//
//     if (counter == 10) {
//         break counter;
//     }
// };
// println!("{}",result)

// let rng = rand::thread_rng().gen_range(1, 10);
//
// use hello_world::hello;
// hello();
// println!("{:?} months in a year 1.", 12);
// println!("{} months in a year 2.", 12);
// let x = 5;
//
// let y = {
//     let x = 3;
//     x + 1
// };
//
// println!("x {}", x);
// println!("y {}", y);
//
// fn five() -> i32 {
//     5
// }
// println!("{}", five());
//
// fn six() -> i32 {
//     return 6;
// }
// println!("{}", six());
//
// let s1 = String::from("hello");
// let mut s2 = &s1;
// println!("s2 {}", s2);
// let s3 = s1;
// s2 = &s3;
// println!("s2 {}", s2);
// println!("s3 {}", s3);
//
// let name = String::from("这是一段中文字符串");
// let x1 = &name[0..3];
// println!("{}", x1);
//
// let arr = [1, 2, 4, 5, 7];
// let part = &arr[0..3];
// for i in part.iter() {
//     println!("{}", i)
// }
//
// let beta = Site {
//     domain: String::from("www.betalpha.com"),
//     name: String::from("hello"),
// };
//
// let beta1 = Site {
//     domain: String::from("www.betalpha.com"),
//     ..beta
// };
//
// println!("{}", beta1.name);
//
// let color = Color(1, 2, 3);
// println!("{}", color.0);
//
// let book = Book::Papery(1001);
//
// match book {
//     Book::Papery(i) => println!("{}", i),
//     Book::Electronic { url } => println!("{}", url),
// }
//
// let option = Option::Some("hello");
// match option {
//     Option::Some(some) => println!("{}", some),
//     Option::None => println!("is none"),
// }
//
// let op: Option<&str> = Option::None;
//
// if let Option::None = op {
//     println!("op is none")
// }
// use second_module::message;
// println!("This is the main module.");
// println!("{}", message());
//
// let result = File::open("hello.txt");
// if let Ok(file) = result {
//     println!("file is ok");
// } else {
//     println!("file not exist")
// }
//
// println!("---------------------");
// let str_file = read_text_from_file("./rust_study.d");
// match str_file {
//     Ok(s) => println!("{}", s),
//     Err(e) => match e.kind() {
//         io::ErrorKind::NotFound => println!("No such file"),
//         _ => println!("Can not read the file"),
//     },
// };
// let p = Point { x: 1, y: 3 };
// println!("{}", p.x());
//
// let person = Person {
//     name: String::from("wangrui"),
//     age: 32,
// };
// println!("{}", person.describe());
// A {
//     name: X {
//         name: String::from("haha"),
//     },
// }
// .d();
//
// struct Str<'cc> {
//     content: &'cc str,
// }
// let s = Str {
//     content: "string_slice",
// };
// println!("s.content = {}", s.content);
//
// let r;
// {
//     let s1 = "rust";
//     let s2 = "ecmascript";
//     r = longer(s1, s2);
// }
// println!("{} is longer", r);
//
// let args = std::env::args();
// for arg in args {
//     println!("{}", arg);
// }
//
// let mut vector = vec![1, 2, 3, 4];
// let mut vector2 = vec![1, 2, 3, 4];
// vector.push(5);
// vector.push(6);
// vector.push(7);
// for x in &vector {
//     println!("current is {}", x);
// }
// for x in &vector {
//     println!("current is {}", x);
// }
//
// for x in &mut vector {
//     *x += 50;
//     println!("current is {}", x);
// }
//如果不加&，那么不代表引用，就代表使用他的本身，而他的本生会把所有权给println，而println的生命周期
//是在for里面，所以结束后，也就销毁了，那么for外面看到的vector就是一个空的，没有意义的
// for x in vector {
//     println!("current is {}", x);
// }
//     vector.append(&mut vector2);
//
//     for x in &vector {
//         println!("current is {}", x);
//     }
//
//     use std::thread;
//     use std::time::Duration;
//
//     thread::spawn(|| -> i8 {
//         for i in 0..5 {
//             println!("spawned thread print {}", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//         2
//     });
//
//     for i in 0..7 {
//         println!("main thread print {}", i);
//         thread::sleep(Duration::from_millis(1));
//     }
//
//     use std::sync::mpsc;
//     let (tx, rx) = mpsc::channel();
//
//     thread::spawn(move || {
//         let val = String::from("hi");
//         let val1 = String::from("hi333");
//         thread::sleep(Duration::from_secs(3));
//         tx.send(val).unwrap();
//         tx.send(val1).unwrap();
//     });
//     println!("执行到这里了，子线程还在sleep");
//     let received = rx.recv().unwrap(); //这里在等待
//     println!("Got: {}", received);
//     let received = rx.recv().unwrap(); //这里在等待
//                                        // let received = rx.recv().unwrap();//这里在等待
//     println!("Got: {}", received);
// }
//
// fn longer<'b>(s1: &'b str, s2: &'b str) -> &'b str {
//     if s2.len() > s1.len() {
//         s2
//     } else {
//         s1
//     }
//
// trait B {}
//
// trait C {}
//
// struct X {
//     name: String,
// }
//
// impl B for X {}
//
// impl C for X {}
//
// struct A<T> {
//     name: T,
// }
//
// impl<T: B + C> A<T> {
//     fn d(&self) {
//         println!("haha")
//     }
// }
//
// trait Comparable {
//     fn compare(&self, object: &Self) -> i8;
// }
//
// trait Descriptive {
//     fn describe(&self) -> String {
//         String::from("[Object]")
//     }
// }
//
// struct Person {
//     name: String,
//     age: u8,
// }
//
// impl Descriptive for Person {
//     fn describe(&self) -> String {
//         format!("{} {}", self.name, self.age)
//     }
// }
//
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
//
// fn read_text_from_file(path: &str) -> Result<String, io::Error> {
//     let mut f = File::open(path)?;
//     let mut s = String::new();
//     f.read_to_string(&mut s);
//     Ok(s)
// }
//
// enum Book {
//     Papery(u32),
//     Electronic { url: String },
// }
//
// struct Site {
//     domain: String,
//     name: String,
// }
//
// struct Color(u8, u8, u8);
