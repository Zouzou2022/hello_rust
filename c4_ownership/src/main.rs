fn main() {
    println!("Hello, world!");

    //创建可变变量 mut s
    let mut s = String::from("hello");
    s.push_str(",world");
    println!("{}", s);

    //移动
    let s1 = String::from("hhhh");
    let s2 = s1;
    println!("{}", s2);
    //println!("{}", s1);会报错，s1所指变量的所有权已经给了s2

    //克隆
    let s3 = String::from("wwww");
    let s4 = s3.clone();
    println!("{}, {}", s3, s4);

    //只在栈上的数据：拷贝
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);
    //这里不涉及所有权的转移
    //原因是像整型这种在编译时已知大小的类型是存储在栈上的，所以拷贝是快速的
    //不需要在创建变量y时使x失效

    println!("--------------------------------------");

    //所有权与函数
    let t = String::from("hhahah"); //t进入作用域
    let y = 5; //y进入作用域
    takes_ownership(t); //t的值移动到函数里
    makes_copy(y); //y的值本应该移动到函数里，但是y是整型，这里其实是copy了一份
    println!("{}", y);
    //println!("{}", t);

    println!("--------------------------");

    //通过函数返回值返回所有权
    let t1 = gives_ownership();
    println!("{}", t1);
    let t2 = String::from("hello");
    println!("{}", t2);
    let t3 = takes_and_gives_back(t2);
    //println!("{}  {}", t3, t2); t2已经没有值的所有权了

    //由于之后还想使用某变量，因此不想让所有权转移，为此引入“借用”概念
    //借用------不获取所有权，但是可以使用值
    //&mut s 为可变引用，允许我们修改借来的值
    let mut u1 = String::from("hahha");
    //& 符号就是 引用，它们允许你使用值但不获取其所有权。
    let u2 = calculate_length(&u1);
    println!("the length of {} is {}", u1, u2);
    change(&mut u1);
    println!("{}", u1);


    //slice也是一类引用
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} //在这里，some_thing移出作用域并且调用drop方法，释放内存
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} //在这里，some_integer移出作用域
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str(",world");
}
