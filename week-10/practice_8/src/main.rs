fn main() {

    for num1 in 8..10 {
        for num2 in 16..17 {
              for num3 in 15..17 {

                let val = num1 + num2 + num3;
                println!("{}", val);
              }
        }
    }
}
