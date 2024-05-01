/*
In Rust, every program must have a main function, which serves as the entry point for the program. The main function is where the execution of the program begins.
*/

fn main() {
    // 0-10 => 0 to 9(n-1)
    for i in 0..20 {
        println!("{}", i);
    }
    // arrays,maps,strings
    let _sentence: String = String::from("my name is jay");
    let first_word = get_first_word(_sentence);
    print!("First word is: {}", first_word);


    // for loop
    let n:i32 = 1000;
    for i in 0..n  {
        println!("Hello world! {}",i);
    }
    let a:i32 =10;
    let b:i32 = 20;
    let sum:i32 = do_sum(a, b);
    println!("Sum of {} and  {} is {}",a,b,sum);
}
//////////////////////////////////////////////////////////////////////
// FUNCTIONS
// Return type is must to give, can't be inferred
// fn  fn name () -> return_type{}

fn do_sum(a:i32,b:i32)-> i32 {
    return a+b;
}



fn get_first_word(sentence: String) -> String {
    let mut ans: String = String::from("");

    for char in sentence.chars() {
        ans.push(char);
        if char == ' ' {
            break;
        }
    }
    return ans;
}
