

pub fn slices_main(){

    let mut data =[1,2,3,4,5];
    use_slices(&mut data[1..4]);
    println!("{:?}",data);
}

pub fn use_slices(slice: &mut[i32]){

    println!("first elem  = {}, len = {}", slice[0],slice.len());
     slice[0] = 4321;
}