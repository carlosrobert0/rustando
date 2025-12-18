#[derive(Debug)]
struct AlwaysEqual;
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: u32,
}

impl User {
    fn age_more_than_18(&self) -> bool {
        self.age > 18
    }

    fn new() -> User {
        Self {
            name: String::from("carlos"),
            email: String::from("carlos@gmail.com"),
            age: 28,
        }
    }
}

fn main() {
    let user = User::new();
    let name_is_equal_carlos = AlwaysEqual;

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Cor: {:#?}", black);
    println!("Ponto: {:#?}", origin);

    println!("User: {:#?}", user);
    println!("Usuário é maior de idade? {}", user.age_more_than_18());
}

fn build_user(user: &User) -> User {
    User {
        name: user.name.clone(),
        email: user.email.clone(),
        age: user.age,
    }
}
