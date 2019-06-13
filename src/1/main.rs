/*
 * 構造体。
 * CreatedAt: 2019-06-11
 */
fn main() {
    let user1 = User {name: String::from("Yamada"), email: String::from("abc@def")};
//    user1.name = String::from("YAMADA"); // error[E0594]: cannot assign to field `user1.name` of immutable binding
    println!("{:?}", user1);
    let mut user2 = User {name: String::from(""), email: String::from("ghi@jkl")};
    user2.name = String::from("Suzuki");
    user2.email = String::from("ghi@jkl");
    println!("{:?}", user2);
}
#[derive(Debug)]
struct User {
    name: String,
    email: String,
}
