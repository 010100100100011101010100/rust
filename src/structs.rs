struct User{
    first:str,
    last:str,
    age:i32
}


//impl keyword for implementing struct functtions
impl User{
    fn print(&self) {
        println!("{}",self.first +" "+self.last);
    }

}


//use enum when we know that there are only some fixed number of types of variables.
// In this case, direction can only have 4 values and we recommend to use enums
enum Direction{
    North,
    South,
    East,
    West
}
