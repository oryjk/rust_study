use std::io;

pub fn hello()
{
    let foo = 10;
    let bar = foo;
    let mut guess = String::new();
    let i = io::stdin().read_line(&mut guess).expect("无法读取行");
    println!("guess {} , {}", guess, i);

    println!("bar  = {}", bar);


    println!("Hello World!!!")
}
