fn main() {
    check1();
    check2();
}

//Ownership
fn check1() {
    let mut x = 5;
    let y = &mut x;
    *y += 1;
    println!("{}", x);
    y += 1; //fails, you need to dereference y
    println!("{}", x);
}

fn check2(){
    let s1 = String::from("hello");
    assert!(s1 == "hello");
    println!("s1 is {}", s1);
}
