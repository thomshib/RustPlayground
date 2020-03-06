

trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str ;
    fn talk(&self){
        println!("{} cannot talk",self.name());
    }
}

struct Human{
    name: &'static str
}

impl Animal for Human{
    fn create(name: &'static str) -> Human{
        Human{name:name}
    }
    fn name(&self) ->&'static str{
        self.name
    }
    fn talk(&self){
        println!("{} says hello",self.name());
    }
}

struct Cat{
    name: &'static str
}

impl Animal for Cat{
    fn create(name: &'static str) -> Cat{
        Cat{name:name}
    }
    fn name(&self) ->&'static str{
        self.name
    }
    fn talk(&self){
        println!("{} says meow",self.name());
    }
}

trait Summable<T> {
    fn sum(&self ) -> T;
 }

 impl Summable<i32> for Vec<i32>{
     fn sum(&self) -> i32{
         let mut result = 0;
         for x in self {
             result += *x;
         }
         return result;

     }
 }

pub fn traits(){
    
    //OOPS concept

    let h = Human{name:"shibu"};
    h.talk();

    let c = Cat{name:"tiny"};
    c.talk();

    let nh:Human = Animal::create("john");
    nh.talk();

    //creating an extension method for Vector

    let a = vec![1,2,3];
    println!("sum = {} ", a.sum());
}