fn main() {
    {
        let s = "hello";
        println!("{}", s);
    }

    {
        let mut s = String::from("hello");
        println!("{}", s);
        s.push_str(" world");
        println!("{}", s);
    }
}
