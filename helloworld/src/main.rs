use std::string;

fn main() {
    let mut number = 10;
    print!("hello,world!\n");
    println!("{} is the best language!", "rust");
    println!("{} is the number of Messi!",number);
    number = 7;
    println!("{} is the number of Ronaldo (Maybe the best) !", number);


    //number
    let neg_number= number - 10i32;
    println!("{} is a nega number", neg_number);


    //bool
    let compare = -7i32 > 7i32;
    println!("is -7 > 7 ? {}", compare);


    //char
    let meme = '😂';
    println!("meme: {} " ,meme);


    //str
    let string1 = "ahhhhhhh";
    println!("the face of {} is {} !", string1, meme);


    //tuple&struct
    let tuple1 = ('A', 7 , '😘', true);
    println!("show tuple's second element : {}", tuple1.1);
    let xiaohaoliu = Student {name: "xiaohaoliu".to_string(), age: 20, grade: 00}; // or use String::from("xiaohaoliu") 
    println!("{}", xiaohaoliu.name);
}
struct Student { name: String, age: i32, grade: i32}


