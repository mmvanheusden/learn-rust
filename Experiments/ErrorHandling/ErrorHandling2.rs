pub fn main() {
    // We can also just throw an error when we create a variable.
    let guess_1 = 13;
    let guess_2 = 104;

    let guess_1 = Guess::new(guess_1);
    // This way by moving the check to the Guess struct we can safely assure that our guess is valid and safe to use.t
    //let guess_2= Guess::new(guess_2);
    //TODO: How to error handle this then??

    println!("{}", guess_1.value);
    //println!("{}", guess_2.value);


}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess must BE BETWEEN 1..100. Got {}",value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}