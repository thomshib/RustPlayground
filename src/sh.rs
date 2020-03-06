

use std::mem;

struct Point {
    x: f64,
    y: f64

}

fn origin() -> Point{
   return Point{x: 0.0,y: 0.0};
}
pub fn stack_and_heap(){

    //stack allocate
    let p1 = origin();

    //heap allocate
    let p2 = Box::new(origin());

    println!("p1 occupies {} size", mem::size_of_val(&p1));
    println!("p2 occupies {} size", mem::size_of_val(&p2));

    //unboxing; get the value of p2
    let p3 = *p2;
    println!("p3.x = {}", p3.x);
    println!("p3 occupies {} size", mem::size_of_val(&p3));

    

    

}