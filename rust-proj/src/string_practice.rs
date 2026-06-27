
fn main(){
    let name = String::from("Sai Krishna");
    println!("hello, {}",name);

    let myletter = name.chars().nth(80);
    match myletter {
        Some(c)=>println!("{}",c),
        None=>println!("It was not a word"),
    };
    println!("{}",myletter.unwrap());

}