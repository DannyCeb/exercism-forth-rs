fn main() {
    /*
    let mut f = forth::Forth::new();
    println!("{:#?}", f.eval(": foo 5 ;"));
    println!("{:#?}", f.eval(": bar foo ;"));
    println!("{:#?}", f.eval(": foo 6 ;"));
    println!("{:#?}", f.eval("bar foo"));
    println!("{:#?}", f.stack());*/

    println!("====================================");

    let mut f = forth::Forth::new();
    println!("{:#?}", f.eval(": foo 10 ;"));
    println!("{:#?}", f.eval(": foo foo 1 + ;"));
    println!("{:#?}", f.eval("foo"));
    println!("{:#?}", f.custom_instructions);
    println!("{:#?}", f.stack());
}
