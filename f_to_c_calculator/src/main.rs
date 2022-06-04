use std::io;

fn main() {
    loop {
        println!("Enter the temperature you want to convert: ");

        let mut c = String::new();

        io::stdin().read_line(&mut c).expect("Failed to read line");

        // convert the string to a number and check it's valid
        let c: f32 = if let Ok(num) = c.trim().parse() {
            num
        } else {
            eprintln!("We need a number");
            continue;
        };
        println!("{}", convert(c));
        break;
    }
}

fn convert(deg_f: f32) -> f32 {
    (5.0 / 9.0) * (deg_f - 32.0)
}
