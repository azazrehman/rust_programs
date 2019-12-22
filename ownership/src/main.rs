fn main() {
    //println!("Hello, world!");
    let get = giveback();
    println!("Hello, {}",get);

    let batch3= String::from("IOT");
    let assignment=receiver(batch3);
    println!("Assignment {}",assignment);
    println!("Batch 3 {}",batch3);
}

fn giveback()->String
{
    let name= String::from("Azaz");
    name    
}

fn receiver(morning: String)->String
{
    println!("Shift Reciever {}",morning);
    morning
}