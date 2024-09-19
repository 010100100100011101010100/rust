fn main() {
    create_string();
    
}

//ownership
fn create_string(){
    let mut name:String=String::from("this is");
    //owner gets moved
    // let name2= nbame;
    change(name2 : &mut name);
    

    //this will also dereference nbame pointer and the new pointer will be name2 since in the function name2=nbame
    // another_move_example(nbame);


    // another_move_example(nbame.clone());
    change(name2:&mut String);
    println!("{}",name);

    fn change(name2: &mut &String){
        name2.push_str(" gautam");
    }

}


fn another_move_example(name2:String){
    println!("{}",name2);
}



