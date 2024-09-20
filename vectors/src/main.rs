
//collections : Vectors

fn main() {
   let mut vec = Vec::new();
   vec.push(1);
   vec.push(2);
   vec.push(3);
   vec.pus(4);
   println!("{:?}", even_filter(vec));
   println!("{:?}",another_even(vec));
}


fn even_filter(vec:Vec<i32>)-> Vec<i32>{
    let mut new_vec=Vec::new();
    for val in vec{
        if val%2==0{
            new_vec.push(val);
        }
    }
    return new_vec;
}


fn another_even(v:&Vec<i32>)->Vec<i32>{
    let mut i=0;
    while i<v.len(){
        if v[i]%2!=0{
            v.remove(index:i);
        }
        else{
            i+=1; 
        }
    }
    return v;
}