 pub fn find_empty(){
     let f1:String=String::from("Hasnian");
     let search:bool=check_empty(&f1);
    println!("Value is {}",search);
 }
 fn check_empty(find:&String)->bool{
 return find.is_empty();
}