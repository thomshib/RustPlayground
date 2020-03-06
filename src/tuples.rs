

pub fn tuples(){


    let x = 3;
    let y = 4;

    let result = sum_and_product(x,y);
    println!("result = {:?}", result);

    //destructuring

    let (a,b) = result;

    println!(" a = {}; b = {}", a, b);
}

pub fn sum_and_product(x:i32, y: i32) -> (i32,i32){

    return ( x+y, x*y );

}