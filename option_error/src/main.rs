/* option 变体
enum Option<T> {
    None,     // The value doesn't exist
    Some(T),  // The value exists
}
 None 和 Some 不是类型，而是 Option<T> 类型的变体。 这表示在其他功能中，
 函数不能使用 Some 或 None 作为参数，而只能使用 Option<T> 作为参数
 */

/* 模式匹配
Rust 中提供了一个功能强大的运算符，称为 match。 可利用该运算符，通过提供模式来控制程序流。
当 match 找到匹配的模式时，它会运行随该模式一起提供的代码， Option 值使用 match 表达式，
并为它的每个变体定义一个操作过程。 Rust 将这些分支称为“match arm” 。
match arm 必须涵盖输入类型可能具有的每个可能值。 如果你尝试根据非详尽模式列表进行匹配，则会出现编译器错误。*/

//匹配单个值时可以使用if let，这样就不用想 match arm 那样涵盖所有可能值。

/*
可以使用 unwrap 方法直接访问 Option 类型的内部值。 如果变体是 None，会 panic。
expect 方法的作用与 unwrap 相同，但它提供由第二个参数提供的自定义 panic 消息。 */

fn main() {
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first);

    // pick the third item:
    let third = fruits.get(2);
    println!("{:?}", third);

    // pick the 99th item, which is non-existent:
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);

    //模式匹配
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            //Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }

    //if let 匹配
    let a_number: Option<u8> = Some(7);
    if let Some(7) = a_number {
        println!("That's my lucky number!");
    }
    //unwarp 和 expect
    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");

    // let empty_gift: Option<&str> = None;
    // assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!

    let a = Some("value");
    assert_eq!(a.expect("fruits are healthy"), "value");

    // let b: Option<&str> = None;
    // b.expect("fruits are healthy"); // panics with `fruits are healthy`

    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "cat");
}
