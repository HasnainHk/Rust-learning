pub fn borrow(){
    let name1:String=String::from("Hasnain KASHIF");
    having_borrow(&name1);
    println!("This is my Name AND IT IS SOMETHING{},",name1);
}

fn having_borrow(find:&String){
   println!("this is scnd{}",find);
}