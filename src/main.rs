use rand::Rng;
use std::char;
use std::io;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Can't Read a line");

    line
}
fn read_char() -> char {
    match read_line().trim().parse() {
        Ok(c) => c,
        Err(_) => panic!("Wrong character"),
    }
}
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn random_string(len: usize) -> Vec<char> {
    let mut random_string: Vec<char> = Vec::new();
    for _i in 0..len {
        let c = match char::from_u32(rand::thread_rng().gen_range(97, 123)) {
            Some(num) => num,
            None => '0',
        };
        random_string.push(c);
    }
    random_string
}
fn main() {
    let env = [
        "                  ",
        "                  ",
        "                  ",
        "                  ",
        "                  ",
        "                  ",
        "                  ",
        "------------------",
        "                  ",
    ];

    let secret = random_string(10);
    for i in secret.iter() {
        print!("{} ", i);
    }
    // loop {
    //     println!("Enter your guess:");
    // }
}
