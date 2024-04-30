fn main() {
    // Numbers
    /*
    Signed integers
    i8 reserve 8 bits
    i16 reserve 16 bits
    i32 reserve 32 bits
    i63
    i128

    unsigned integers
    u8
    u16
    u32
    u64

    Max number which it can store: 2^7 -1 && -2^7 (-128 to 127)

    */
    let signed_int: i32 = 2;
    let un_signed_int: u32 = 62;
    let float: f64 = 300.0002;

    // Cannot MUTATE A VARIBALE IN RUST
    // signed_int = 4;
    // for mutating use
    let mut _mrks: i8 = 10;
    for _i in 0..20 {
        _mrks = _mrks + 1;
    }
    println!("{}", _mrks);

    println!(
        "signed_int:{}, un_signed_int:{}, float:{}",
        signed_int, un_signed_int, float
    );
}
