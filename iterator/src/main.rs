//Collections : Iterators


//immutable iterator -> iter()
fn main() {
    let mut v=vec![1,2,3];
    let iter=v.iter();
    for val in iter{
        println!("{}", val);
    }    



//mutable iterator -> iter_mut()

let mut v1=vec![1,2,3,4];
let mut v2=vec![1,2,3,4];
let iter_1=v1.iter_mut();
for val in iter_1{
    *val=*val+1;

}
println!("{:?}", v1);



//next() iterator
let mut iter_2=v2.iter_mut();
while let Some(val)=iter_2.next(){
    println!{"{}",val};
}




}