fn main() {
    let greeting: String = String::from("hello world");
    println!("{}", greeting);
    let char1: Option<char> = greeting.chars().nth(10);
    print!("char1:{}", char1.unwrap());
}
