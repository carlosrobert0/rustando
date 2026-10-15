pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn try_new(value: i32) -> Result<Guess, String> {
        if !(1..=100).contains(&value) {
            return Err(format!("valor fora do intervalo 1..=100: {value}"));
        }

        Ok(Guess { value })
    }

    pub fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("o valor precisa estar entre 1 e 100, recebi {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}