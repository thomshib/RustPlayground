

enum Color{
    Red,
    Blue,
    Green,
    RgbColor(u8,u8,u8),
    CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8},
}

pub fn enums(){

    let c:Color = Color::Red;

    match c{
        Color::Red => println!("r"),
        Color::Blue => println!("b"),
        Color::Green => println!("g"),
        Color::RgbColor(0,0,0)
        | Color::CmykColor{cyan:_, magenta:_,yellow:_, black:255} =>println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})", r,g,b ),
        _ => ()


    }

}