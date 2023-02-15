fn main() {
    // check1();
    // check2();
    check3();
}

//Ownership
fn check1() {
    let mut x = 5;
    let y = &mut x;
    *y += 1;
    println!("{}", x);
    // y += 1; //fails, you need to dereference y
    println!("{}", x);
}

fn check2(){
    let s1 = String::from("hello");
    assert!(s1 == "hello");
    println!("s1 is {}", s1);
}

//structs
struct Point {
    x: i32,
    y: i32,
    tag : String,
}

fn check3() {
    let p1 = Point { x: 5, y: 10 , tag : "p1".to_string() };
    let p2 = Point { ..p1 };
    print!("{}", p1.x); //no error since p1.x implements copy
    print!("{}", p1.tag); //error since p1.tag does not implement copy
}