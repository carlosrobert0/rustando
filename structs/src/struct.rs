struct AlwaysEqual;

struct User {
    id: u32,
    first_letter_name: char,
    active: bool,
    experience_years: f64,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn main() {
    let always_equal = AlwaysEqual;

    let user = User {
        id: 1,
        first_letter_name: 'A',
        active: true,
        experience_years: 5.0,
    };

    let user_b = User {
        id: 2,
        first_letter_name: 'B',
        ..user
    };

    let mut user_mut = User {
        id: 3,
        first_letter_name: 'C',
        active: false,
        experience_years: 5.0,
    };

    let user_builded = build_user(4, 'D', true, 2.0);

    let user_new = User::new(&user);

    user_mut.active = true;

    user_mut.experience_years += 3.0;

    let black = Color(0, 0, 0);

    let origin = Point(0, 0, 0);

    let user_new = User::new(5, 'E', true, 1.0);

    let user_can_hold = user.can_hold(&user_new);

    let user_builded_with_tuple = build_user_with_tuple((6, 'F', true, 4.0));

    let user_builded_with_struct = build_user_with_struct(&User {
        id: 6,
        first_letter_name: 'F',
        active: true,
        experience_years: 4.0,
    });
}

fn build_user(id: u32, first_letter_name: char, active: bool, experience_years: f64) -> User {
    User {
        id,
        first_letter_name,
        active,
        experience_years,
    }
}

impl User {
    fn new(id: u32, first_letter_name: char, active: bool, experience_years: f64) -> Self {
        Self {
            id,
            first_letter_name,
            active,
            experience_years,
        }
    }

    fn can_hold(&self, other: &User) -> bool {
        self.experience_years > other.experience_years
    }
}

fn build_user_with_tuple(user: (u32, char, bool, f64)) -> User {
    User {
        id: user.0,
        first_letter_name: user.1,
        active: user.2,
        experience_years: user.3,
    }
}

fn build_user_with_struct(user: &User) -> User {
    User {
        id: user.id,
        first_letter_name: user.first_letter_name,
        active: user.active,
        experience_years: user.experience_years,
    }
}
