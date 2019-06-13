/*
 * タプル構造体。
 * CreatedAt: 2019-06-11
 */
fn main() {
    let red = Color(255, 0, 0);
    println!("{:?}", red);
}
#[derive(Debug)]
struct Color(i32, i32, i32);

