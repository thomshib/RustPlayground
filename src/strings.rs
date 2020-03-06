pub fn strings(){

    //&str
    let s:&'static str = "hello world";

    for c in s.chars().rev(){
        println!("{}",c);
    }


    //heap allocated string

    let mut letters = String::new();
    let mut a = 'a' as u8;

    while a <= ('z' as u8){
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    } 

    println!("{:?}", letters);

}