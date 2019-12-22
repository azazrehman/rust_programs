fn main() {
    println!("Hello, world!");
    let s=String::from("Hellow")
    println!("Data: {}",takes_ownership(s));
}
fn takes_ownership(some_string: String) -> String
{
    println!("Function: {}",some_string);
    some_string.push_str( " Ali");
    some_string
}
