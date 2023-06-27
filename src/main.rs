#[allow(dead_code)]
#[allow(unused)]

fn main() { 
    let x = 5; 
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2);

    // interger type max and min values
    println!(" i8 has the min value of {}.", i8::min_value());
    println!(" i8 has the max value of {}.", i8::max_value());
    println!(" i16 has the min value of {}.", i16::min_value());
    println!(" i16 has the max value of {}.", i16::max_value());
    println!(" i32 has the min value of {}.", i32::min_value());
    println!(" i32 has the max value of {}.", i32::max_value());
    println!(" i64 has the min value of {}.", i64::min_value());
    println!(" i64 has the max value of {}.", i64::max_value());
    println!(" i128 has the min value of {}.", i128::min_value());
    println!(" i128 has the max value of {}.", i128::max_value());

    // Arvutustehted

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplaction
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    // remainder
    let remainder = 43 % 5;

    // booleans
    let t = true;

    let f: bool = false;

    // Sõned 
    
    // Note that we specify 'char' literals with single quotes, as opposed to string
    // literals, which use double quotes. Rust's 'char' type is four bytes in size
    // and represents a Unicode Scalar Value, which mean it can represent a lot more
    // than just ASCII. Accented letters; Chinese, Japanese, and Korean characters;
    // emoji; and zero-width spaces are all valid 'char' values in Rust. Unicode 
    // Scalar Values range from 'U+0000' to U+E000 to U+10FFFF inclusive. 
    // However, a “character” isn’t really a concept in Unicode, so your 
    // human intuition for what a “character” is may not match up with what a char 
    // is in Rust. We’ll discuss this topic in detail in 
    // “Storing UTF-8 Encoded Text with Strings” in Chapter 8.
    
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Liittüübid

    // tuples - kõik elemendid võivad olla erinevad
    let tup:(i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is {y}");
    println!("The value of x is {}", tup.1);

    // arrays - kõik elemendid peavad olema samad.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];
    
    println!("The value of b's first element is: {}", b[0]);
}
