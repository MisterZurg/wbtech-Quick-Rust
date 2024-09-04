/// Action трейт с методом say
pub trait Action {
    fn say(self);
}

/// Person структуру, которая содержит строковое имя.
pub struct Person {
    pub name: String
}

/// Реализацию трейта Action
impl Action for Person {
    fn say(self) {
        println!("Hello, {}", self.name)
    }
}

fn main() {
    let p = Person{
        name: "Mikhail Grachev".to_string()
    };

    p.say();
}