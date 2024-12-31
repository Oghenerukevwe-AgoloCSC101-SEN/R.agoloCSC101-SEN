fn main() {
    let mut wood:i32 = 35;
    bush(&mut wood);
    println!("The value of wood is: {}",wood);

}

fn bush(plank:&mut i32){
    *plank = *plank/5;
    let wood = *plank/3;
    println!("The value of plank is {}",plank);
    
}
