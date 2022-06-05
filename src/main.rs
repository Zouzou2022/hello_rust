fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = x;
    //此类固定长度的数据是放在stack上的
    println!("x = {} y = {}", x,y);

    let s1 = String::from("xiaolvzi");
    //let s2 = s1;
    let s2 = s1.clone();
    //变量数据类型是放在heap上的
    println!("s1 = {} s2 = {}", s1,s2);

}
