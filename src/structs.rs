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