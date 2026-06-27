fn main(){
    let sentence = String::from("Hello guys this is krishna");
    let first_letter = get_first_letter(sentence);
    for i in 1..100 {
        println!("hello {}",i)
    }
    println!("First letter is {}",first_letter);
}

fn get_first_letter(sentence:String) -> String {
    let mut ans = String::from("");
    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}