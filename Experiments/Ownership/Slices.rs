pub fn main() {
    // return the first word of a sentence, like "hello erge fwefr gerggegkerngkje rge" -> "hello"
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes(); // To check if the String has a space, we convert it to an array of bytes.

        // returns a slice of each character as a number: its byte code.
        // 32 means ' ' a.k.a a space.
        println!("{:?}", bytes);

        for (i, &item) in bytes.iter().enumerate() { // next we iterate over each byte.
            if item == b' ' { // b' ' means take the binary value of the string, so in this case 32.
                return i; // when we found the space, return where in the slice the space is.
            }
            // if the space is not found, proceed
        }

        s.len() // return the length of the entire word if it doesn't contain a space.
    }
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""

    // Because word word isn't connected to the state of s at all, word still contans the value 5 after s is cleared.
    println!("{}", word);


    // There is a better way of doing this.

    // x -> index 0-11
    let x = String::from("hello world");

    // hello -> index 0-5
    let hello = &x[0..5];
    // world -> index 6-11
    let world = &x[6..11];

    // This can also be written as:
    let hello_world = &x[0..x.len()];
    // or:
    let hello_world = &x[..];

    let world = &x[6..x.len()];
    // or even:
    let world = &x[6..];


    // Let's rewrite out utility we made earlier.
    fn first_word2(s: &String) -> &str {
        let bytes = s.as_bytes(); // convert to bytes again

        for (i, &item) in bytes.iter().enumerate() { // loop through each slice
            if item == b' ' {
                return &s[0..i]; // if a space was found, return the slice from the start of the index -> the index of the space.
            }
        }

        &s[..] // else just return the entire thing.
    }

    let mut y = String::from("hello world");

    let word = first_word2(&y);

    // y.clear(); error! Because we need it or something

    println!("the first word is: {}", word);

    let list = [4,6,634,43,543,5432];
    let slice_of_list = &list[3..5];
    println!("{:?}", slice_of_list);

}