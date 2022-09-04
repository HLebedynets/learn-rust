fn main() {
   
    //???????????????????????????????????????????????????????????????
    let mut s = String::from("hello"); // :: is a separator, from ???

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    let s1 = String::from("hello");
    let s2 = s1;
    //Data is not copied, in this situation we copy pointer, length and capacity of s1

    /* println!("{}, world!", s1); does not work because we "moved" s1 to s2 and s1 was invalidated */ 
    
    let s3 = String::from("hello, boys");
    let s4 = s3.clone();

    takes_ownership(s2);

    println!("s3 = {}, s4 = {}", s3, s4); //This code works because we made copied not only stack data, but also heap data
    /* println!("s2 = {}", s2); does not work because we try to borrow moved value */

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y); //We don't need to clone x to make code works, because integers have a known size at compile time, and stored entirely on the stack

    makes_copy(x);

    let v1 = gives_ownership();
    let v2 = String::from("hello, guys");
    let v3 = takes_and_gives_back(v1);
    let (v4, len) = calculate_length(v3);
    println!("The length of '{}' is {}.", v4, len);
    
    //??????????????????????????????????????????????????????????????????????????
    let length = calculate_length_2(&v2); // If i init "len" there is no error??
    println!("The length of '{}' is {}.", v2, length);

    let mut s5 = String::from("hi");
    
    let r1 = &s5; // no problem
    let r2 = &s5; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    change(&mut s5); // Without "mut" code won't work, because variable is immutable, so is reference.
    let mut r3 = &mut s5; // no problem
    change(&mut r3);
    println!("{}", r3);

    let reference_to_nothing = dangle();

    let some_words = String::from("hello world");
    let hello = &some_words[..5];
    let world = &some_words[6..]; /*slices examples*/
    let word = first_word(&some_words);
    println!("the first word is: {}", word);

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(&my_string_literal);
}

fn takes_ownership(name1: String) {
    println!("{}", name1);
}

fn makes_copy(name2: i32) {
    println!("{}", name2);
}

fn gives_ownership() -> String { 
    let name1 = String::from("yours");
    name1
}

fn takes_and_gives_back(variable: String) -> String {
    variable
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn calculate_length_2(string: &String) -> usize {
    string.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("hello"); //dangling pointer?
    s
}

fn first_word(s: &str) -> &str {//it allows to use function on both &String and &str values
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {//why reference is here?
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn cringe {
    println!("cringe");
}
fn new {
    
}