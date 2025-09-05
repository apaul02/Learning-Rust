fn main() {
    let result;
    {
        let s1 = String::from("Hellow");
        let s2 = String::from("world");
        result = longest(&s1.as_str(), &s2.as_str());
        println!("The result is {result}");
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
