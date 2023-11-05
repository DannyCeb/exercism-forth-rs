fn main() {
    let mut f = forth::Forth::new();
    println!("{:#?}", f.eval(": 1 2 ;"));
}
