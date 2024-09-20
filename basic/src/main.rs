//include is use in rust
use std::fs::read_to_string;

fn main(){
    let _ans1=is_even(10);
    println!("{}",_ans1);
    let _ans2=fib(7);
    let name=String::from("Rasesh");
    println!("{}",_ans2);
    let _ans3=get_len(name);
    println!("the length of the string is {}",_ans3);
    let circle=Shape::Circle(1.0);
    println!("{}",print_area(circle));


    let index=find_first_a(String::from("rasesh"));

    match index{
        CustomOption::Some(value)=> println!("The index is {}", value),
        CustomOption::None=> println!("a not found in the string")
    }




    let result=fs::read_to_string("a.txt");
    match result {
        Ok(data)=> println!(data),
        Err(err)=>println!("The error is {:?}",err)
    }

} 


//i32,i64 is signed number
fn is_even(num:i32)->bool{
    if num%2==0{
        return true;
    }
    return false;
}


fn fib(num:i32)->i32{
    let mut first=0;
    let mut second=1;
    if num==0{
        return 0;
    }
    if num==1{
        return 1;
    }
    for _i in 1..num -2 {
        let temp=second;
        second=second+first;
        first=temp;
    }
    return second;
}


fn get_len(str:String)->usize{
    //implicit return and that's why no ; and return keyword
    str.chars().count()
}


//enum with asociated values
enum Shape{
    Circle(f64),
    Rectangle(f64,f64)
}



//we can get the values of the associated values in enum with pattern matching 
//match keyword
fn print_area(shape:Shape) -> f64 {
     match shape{
        Shape::Rectangle(a,b)=>a*b,
        Shape::Circle(r)=> std::f64::consts::PI * r* r,
    }
    //this has implicit return, no semi colon in the end
}





enum CustomOption{
    Some(i32),
    None,
}


//option enum with Some and None for return None or any specified type (in this example I have created a custom option)
//<t> is generic type
fn find_first_a(string:String)->CustomOption{
    for(index,char) in string.chars().enumerate(){
        if char=='a'{
            return CustomOption::Some(index as i32);
        }
    }
    return CustomOption::None;
}


//result enum is used for error handling 
//result enum can have two values OK and ERROR

//function to read the contents of another file, it will return an enum of Type RESULT

