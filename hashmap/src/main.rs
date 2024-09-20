// Collections : Hashmaps

use std::collections::HashMap;
fn main() {
    let mut users = HashMap::new();
    users.insert(String::from("rasesh"), 22);
    users.insert( String::from("Bhuvnesh") , 19);

    let user_data=users.get("rasesh");

    match user_data{
        Some(val)=> println!("value is  {}",val),
        None => println!("age not found"),
    }


}
    




//assignment question during the course

//function to map binary tuples inside a vector to a hashmap
fn group_by(vec:Vec<(String,i32)>)-> HashMap<String,i32>{
    let mut result=HashMap::new();
    for(key,val) in vec{
        result.insert(key,val);
    }
    return result;  
}
    
