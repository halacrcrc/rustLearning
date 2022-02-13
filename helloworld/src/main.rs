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
    println!("{}, {}, {}", xiaohaoliu.name, xiaohaoliu.age, xiaohaoliu.grade);


    //enum
    let click = MouseClick{x: 100, y: 150};             
    let keys = KeyPress(String::from("Ctrl+"), 'N');  

    let we_load = WebEvent::WEload(true);   
    let we_click = WebEvent::WEClick(click); 
    let we_key = WebEvent::WEKeys(keys);

    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click,we_key);

}

//struct
struct Student { name: String, age: i32, grade: i32}

//enum , 通过为枚举中的每个变量定义单独的结构，可以直接访问特定变量的数据

#[derive(Debug)]                           //通过 #[derive(Debug)] 语法可以在代码执行期间查看某些在标准输出中无法查看的值。 要使用 println! 宏查看调试数据，用语法 {:#?} 以可读的方式格式化数据。
struct KeyPress(String, char);             //元组变体

#[derive(Debug)]
struct MouseClick{x: i64, y: i64}          //结构变体

#[derive(Debug)]
enum WebEvent {
    WEload(bool),                          //简单变体
    WEClick(MouseClick),
    WEKeys(KeyPress)
}




