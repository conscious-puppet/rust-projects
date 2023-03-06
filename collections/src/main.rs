use std::ops::Add;

fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    println!("{:?}", v);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    };

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }
    v.push(1);
    println!("{v:?}");

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.2),
    ];

    println!("{:?}", row);

    let data = "Initial Contents";
    let s = data.to_string();
    println!("{s}");

    let s = "initial contents".to_string();
    println!("{s}");

    let s = String::from("initial contents");
    println!("{s}");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");
    let s2 = "lo";
    s.push_str(s2);
    s.push('l');
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{s3}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = String::new() + &s1 + &s2;
    let s4 = s1.add(&s2);
    println!("{s3}");
    println!("{s4}");

    let s1 = String::from("tic");
    let s2 = String::from("tak");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s1}");
    println!("{s}");

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}");

    for c in hello.chars() {
        println!("{c}");
    }

    for c in "नमस्ते".chars() {
        println!("{c}");
    }

    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{scores:?}");

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(String::from("Blue"), 50);
    println!("{scores:?}");

    let entry = scores.entry(String::from("Yellow"));
    println!("{entry:?}");

    let entry = scores.entry(String::from("Green"));
    println!("{entry:?}");

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(50);
    println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

}
