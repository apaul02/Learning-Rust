fn main() {
    let result;
    {
        let s1 = String::from("Hellow");
        let s2 = String::from("world");
        result = longest(&s1.as_str(), &s2.as_str());
        println!("The result is {result}");
    }
    let novel = String::from("Call me Ishmael. Some years ago....");
    let first_sentence = novel.split(".").next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence
    };
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}


