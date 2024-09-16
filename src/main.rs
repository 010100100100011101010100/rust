fn main(){
    let _ans1=is_even(10);
    println!("{}",_ans1);
    let _ans2=fib(7);
    let name=String::from("Rasesh");
    println!("{}",_ans2);
    let _ans3=get_len(name);
    println!("the length of the string is {}",_ans3);
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


struct User<'a>{
    first:&'astr,
    last:&'astr,
    age:i32
}

impl User<'_>{
    fn print(&self) {
        println!("{}",self.first +" "+self.last);
    }

}

fn main(){
    let rasesh= User{
        first:"Rasesh",
        last:"Gautam",
        age:19
    };
    rasesh.print();
}