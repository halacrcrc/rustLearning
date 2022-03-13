//https://docs.microsoft.com/zh-cn/learn/modules/rust-memory-management/?ns-enrollment-type=LearningPath&ns-enrollment-id=learn.languages.rust-first-steps

/* 引用 
当借用任何 T 类型的值时，以下规则适用：

代码必须同时实现以下任一定义，但不能同时实现这两个定义：

一个或多个不可变引用 (&T)
恰好一个可变引用 (&mut T) */


/* 生存周期

fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} */


fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String {
    vector.push(String::from(value));
    vector.get(vector.len()-1).unwrap()

}

fn main() {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}