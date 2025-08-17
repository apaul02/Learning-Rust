fn main() {
    let s = String::from("hello world");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);
    println!("After adding keyword, s is '{}'.", s);
    let word = first_word(&s);
    println!("{} is the first word", word);
}
fn calculate_length(s: &str) -> usize{
    s.len()
}
fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        println!("{} {}", i, item);
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]

} 
