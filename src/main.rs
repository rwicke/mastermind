use std::io;

fn main() -> io::Result<()> {
    println!("input data:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let mut iter = buffer.split_whitespace();
    let secret = iter.next().unwrap();
    let count : &str = iter.next().unwrap();
    let count : u32 = count.parse().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let mut iter = buffer.split_whitespace();

    println!();
    println!("answer:");
    for _i in 0..count {
        let guess = iter.next().unwrap();
        let (x, y) = evaluate_guess(&secret, &guess);
        print!("{}-{} ", x, y);
    }
    println!();

    Ok(())
}

fn evaluate_guess(secret : &str, guess : &str) -> (u32, u32) {
    let mut contained = 0;
    for ch in guess.chars() {
        if secret.contains(ch) {
            contained = contained + 1;
        }
    }

    let mut correct_pos = 0;

    for i in 0..secret.len() {
        if guess.chars().nth(i) == secret.chars().nth(i) {
           correct_pos = correct_pos + 1; 
        }
    }

    (correct_pos, contained - correct_pos)
}



