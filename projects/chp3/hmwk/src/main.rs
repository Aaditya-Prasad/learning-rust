use std::io;

fn main() {
    println!("Please provide a temperature!");
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: u32 = temp.trim().parse().expect("Please type a number!");

    println!("Please provide a unit!");
    let mut typ = String::new();

    io::stdin()
        .read_line(&mut typ)
        .expect("Failed to read line");

    println!("{}", convert(temp, typ));

}

fn convert (temp: u32, typ: String) -> f64 {
    let temp: f64 = temp as f64;
    if !"F".eq(&typ) {
        return temp * 9.0/5.0 + 32.0
    }
    (temp - 32.0) * 5.0/9.0

}
