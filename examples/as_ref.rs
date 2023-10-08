fn main() {
    let x = String::from("this is a test");
    let y = "this i sa tes";
    let r: &str = y.as_ref();
    let t: &str = x.as_ref();
}