

pub fn increase_value(x: &mut i32){

     *x += 1;

    
}


pub fn functions(){

    let mut z = 1;
    increase_value(&mut z);
    println!("{}", z);
}