
fn main() {
    let a:i8 = 10;
    let b:i8 = 20;
    print!("Before swap A {}, B {}",a,b);
    let a:i8 = a +b;
    let b:i8 = a - b;
    let ai:i8 = a - b;
    print!("After swap A {}, B {}",a,b)

}