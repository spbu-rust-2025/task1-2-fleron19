use std::io;

fn main() {
    let mut sum = 0;
    let mut input = String::new();
    let mut ok = true;
    let mut number;
    let _ = io::stdin().read_line(&mut input);
    number = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            ok = false;
            0
        }
    };
    while number != -1 {
        sum += number;
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        number = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                ok = false;
                0
            }
        }
    }
    if ok {
        println!("{}", sum);
    } else {
        println!("NaN");
    }
}
