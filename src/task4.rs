pub fn borrow(){
    let name1:String=String::from("Hasnain");
    having_borrow(&name1);
    println!("This is my Name {},",name1);
}

fn having_borrow(find:&String){
   println!("this is scnd{}",find);
}