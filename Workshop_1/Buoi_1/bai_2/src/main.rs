use std::io;

fn main() {
    let mut s: String = String::new();
    let mut _ch: char;
    io::stdin().read_line(&mut s).expect("Failed to read line");
    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("{}", input.len());
        if input.len() == 2 {
            _ch = input.chars().nth(0).unwrap();
            break;
        }
    }

    let mut count = 0;
    let mut count_dup: i32 = 0;
    let mut del: Vec<i32> = Vec::new();
    for char in s.chars() {
        if char == _ch {
            count_dup += 1;
        }
        count += 1;
    }
    s = s.replace(_ch.to_owned(), "");
    println!("{} {}", count_dup, s);
}
