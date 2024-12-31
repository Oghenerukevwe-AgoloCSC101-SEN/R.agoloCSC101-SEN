fn main() {
    
    let mut game:i32 = 25;
    let mut two:i32 = 25;

    if game > 0 {
        game += 3;
        two -=2;
        let grass = game+two;
        game = grass / 2;
        two = game * 3;
        println!("grass, game and two are {},{},{}",grass, game, two);
    }
}
