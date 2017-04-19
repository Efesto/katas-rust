fn main() {
    let numbers = 0..100;

    for number in numbers {
        match (number%3, number%5)  {
            (0, 0) => println!("fizzbuzz"),
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _ => println!("{:?}", number),
        }
    }
}