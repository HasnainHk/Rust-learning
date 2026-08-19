pub fn mut_borrow(){
    let mut name1:String=String::from("Hello");
    println!("before  {},",name1);
    having_borrow(&mut name1);
    println!("after  -{},",name1);
}

fn having_borrow(find:&mut String){
    find.push_str("world");
   
}