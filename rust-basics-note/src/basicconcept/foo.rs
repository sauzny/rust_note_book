

pub fn foo01(){
    let name = "trump";
    let age = 60;

    print!("my name is {} and ", name);
    println!("my age is {}", age);

    println!("Nobody knows more about {} than I do! you're {1} {1} {1}!", "rust", "fired");

    println!("格式字符串中通过 {{{{ 和 }}}} 分别转义代表 {{ 和 }}");
}

pub fn foo02(){
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

pub fn foo03(){
    // TODO
    // 常量的demo
}

pub fn foo04(){
    let x = 5;
    println!("The value of x is: {}", x);
    // 不可变量 不能直接再次赋值
    //x = 6;
    let x = 6;
    println!("The value of x is: {}", x);
}

pub fn foo05(){
    let spaces = "   ";
    let spaces = spaces.len();
}

pub fn foo06(){
    let mut spaces = "   ";
    // 不能修改 可变变量的类型
    //spaces = spaces.len();
}