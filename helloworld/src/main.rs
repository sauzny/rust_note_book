use std::io;
use rand::Rng;

mod foo {
    pub fn foo01(){
        let name = "trump";
        let age = 16;
    
        print!("my name is {} ", name);
        println!("my age is {}", age);
    
        println!("Nobody knows more about {} than I do! you're {1} {1} {1}!", "rust", "fired");
    
        println!("格式字符串中通过 {{{{ 和 }}}} 分别转义代表 {{ 和 }}");
        println!("{{");
        println!("}}");
    }

    pub fn foo02(){
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1, 101);
    
        println!("The secret number is: {}", secret_number);
    
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        std::io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        println!("You guessed: {}", guess);
    }
}

fn main() {
    foo.foo01();
    foo::foo02();
}
