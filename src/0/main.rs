/*
 * 構造体。
 * CreatedAt: 2019-06-11
 */
fn main() {
    let user = User {name: String::from("Yamada"), email: String::from("abc@def")};
    println!("{:?}", user);
}
#[derive(Debug)]
struct User {
    name: String,
    email: String,
}
