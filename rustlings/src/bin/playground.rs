fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");


 
}

fn change(some_string: &mut String) {
    some_string.push_str(" yay");
}