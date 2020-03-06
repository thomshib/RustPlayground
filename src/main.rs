use std::mem;
extern crate  rand;
use rand::Rng;
mod sh;
mod color;
mod slices;
mod strings;
mod tuples;
mod func;
mod meth;
mod clos;
mod high;
mod trt;
 

 static mut Z:i32 = 42;

 fn main(){
     //data_types();
     //operators();
    //  unsafe{
    //     println!("the value of Z = {}" ,Z);
    //  }

     //sh::stack_and_heap();
     //for_loop();
     //match_statement();
     //color::enums();
     //options();
     //array();
     //vectors();
     //slices::slices_main();
     //strings::strings();

     //tuples:: tuples();

     //func::functions();
     //meth::methods();

     //clos::closures();

     //high::higher_order_functions();

     //trt::traits();
     random();





 }

 fn data_types(){

   let a: u8 = 235;

   println!("a = {}:", a);

   let mut b:i8  = 0;
   println!("b = {}:, size {}", b, mem::size_of_val(&b));

   b = 123;

   println!("b = {}:", b);

   let c = 123456789;

   println!("the value of c :{} and size: {}", c, mem::size_of_val(&c));

   let z: isize = 123;
   let size_of_z = mem::size_of_val(&z);

   println!("Value of z :{} of size {} in {} bit os",
    z,size_of_z,size_of_z * 8
    );

    let d:char = 'x';
    println!("the value of d :{} and size: {}", d, mem::size_of_val(&d));

    let e:f64 = 2.5;
    println!("the value of e :{} and size: {}", e, mem::size_of_val(&e));

    let g:bool = false;
    println!("the value of g :{} and size: {}", g, mem::size_of_val(&g));








}


fn operators(){

    let b = 2.5;
    let b_cubed = f64::powi(b,3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!(" {} cubed = {}, {} ^pi =  {}", b,b_cubed, b, b_to_pi);

    //bitwise
    let c = 1|2;   // | OR , & AND, ^ XOR, ! NOR
    // 01 | 10 = 11 == 3_10

    println!("1|2 = {} ", c);

    // shift operator

    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

}

fn for_loop(){

    for (pos,x) in (30..41).enumerate(){
        println!("{} : {}" ,pos,x);
    }
}

fn match_statement(){

    let country_code = 44;

    let country = match country_code{
        44 => "US",
        46 => "Sweden",
        1...999 => "Unknown",
        _ => "Invalid"
    };

    println!("the country with code  {}  is {}",  country_code,country);
}

fn options(){

    let x = 3.0;
    let y = 0.0;

    let result:Option<f64> = 
    if y != 0.0 {Some(x/y)} else {None};

    println!("{:?}", result);

    match result{
        Some(z) =>  println!("{} / {} = {}", x,y,z),
        None => println!("cannot divide {} by {}", x, y)
    }

    if let Some(z) = result {println!("z={}", z);}

}

fn array(){

    let mut a:[i32;5] = [1,2,3,4,5];

    println!(" a has {} element, first element is {}",
        a.len(), a[0]
    );

    a[0] = 321;

    println!("a[0] is {}", a[0]);

    println!("{:?}", a);

    let mtx: [[f32;3]; 2] = [
        [0.0,1.0,2.0],
        [3.0,4.0,5.0]

    ];

    for i in 0..mtx.len(){
        for j in 0..mtx[i].len(){
            if i==j{
                println!("mtx[{}][{}] = {}",i,j,mtx[i][j]);
            }
        }
    }
}

fn vectors(){

    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);
    println!("a[0] = {}", a[0]);

    let idx:usize = 0;
    println!("a[idx] = {}", a[idx]);

    //enumerate a vector
    for x in &a{
        println!("{}", x);
    }

    //handling errors

    match a.get(3){
        Some(x) => println!("Found value at a[3] as {}", x),
        None => println!("error")
    };

    a.push(44);

    while let Some(elem) = a.pop(){
        println!("{}", elem);
    }

}

fn random(){

    let mut rng = rand::thread_rng();
    
}

