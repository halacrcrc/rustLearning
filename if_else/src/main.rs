fn main() {
   
    //数组
    let mut bytes = [11;3];                        // 11是初始值，3是数组长度, [T; size]是数组签名，T是元素类型，size是数组长度
    let _ala = ["a", "l", "a"];                   // 数组元素具有相同的数据类型且不变，数组长度固定不变；数组中仅值可变
    println!("{}", bytes[2]);
    bytes[2] = 233;
    println!("{}", bytes[2]);

    //矢量 vec![T], 声明由未知类型组成的向量                  //矢量可以增大缩小
    let nums = vec![1,2,3];
    println!("{:?}",nums);

    let mut fruit= Vec::new();
    fruit.push("apple");
    fruit.push("banana");
    fruit.push("cherry");
    println!("{:?}",fruit);
    fruit.pop();
    fruit[0]= "bear";
    println!("{:?},{:?}",fruit, fruit[0]);


    
}

