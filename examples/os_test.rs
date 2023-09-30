use std::env;

trait T{
    fn trait_fun() {
        println!("I'm a trait_fun")
    }
}
struct A;

impl T for A{} 

// fn test(x: dyn T) {
    
// }
fn main() {
    let x = A;
    // x::trait_fun();
    println!("{}", env::consts::OS);

}