fn main() {
    
    let mut lab = 15;
    let mut class = 50;
    let mut min = 4;
    let mut max = 7;

    while lab < class {
        lab+=min;
        class-=max;
        println!("The value of class = {}",class);
    }
}
