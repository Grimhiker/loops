fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}", number);

        number -= 1;
    }

    println!("Liftoff");

    another_function ();
 
fn  another_function() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
        println!("LIFTOFF!!!");
    }
}
