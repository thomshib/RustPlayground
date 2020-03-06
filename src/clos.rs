

pub fn closures(){

    let plus_one = |x:i32| -> i32 { x + 1};

    let a = 1;

    println!("{} + 1 = {}", a, plus_one(a));


    //clousures

    let mut two = 2;

    let plus_two = |x|{
        let mut z = x;
        z += two;
        z
    };

    println!("{} + {} = {}", 3,two,plus_two(3));

    let borrow_two = &mut two;

    println!("{}", borrow_two);

    //by reference &mut x

    let plus_three = |x: &mut i32 | *x += 3;

    let mut f = 12;
    plus_three(&mut f);
    println!("{}", f);


    
}