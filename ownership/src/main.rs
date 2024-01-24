fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    let s1: String = String::from("hello");
    let s2 = s1;

    // println!("{}", s1); 컴파일 에러

    let s1: String = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    //***************************************

    let s = String::from("Hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    // ******** reference *******************
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);


    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    // ************** Slice *********************

    let mut s = String::from("hello world");
    let word = first_word(&s);


}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
