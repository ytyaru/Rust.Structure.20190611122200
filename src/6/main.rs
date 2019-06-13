/*
 * 構造体要素のライフタイム。
 * CreatedAt: 2019-06-11
 */
fn main() {
    let user1 = User { name: "NAME", email: String::from("EMAIL") };
    println!("{:?}", user1);
}
#[derive(Debug)]
struct User {
    name: &str, // error[E0106]: missing lifetime specifier
    email: String,
}
