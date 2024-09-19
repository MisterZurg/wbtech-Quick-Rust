use std::any::{Any, TypeId};

fn get_type_at_runtime<T: Any>(_var: &T) {
    if _var.type_id() == TypeId::of::<String>() {
        println!("String")
    } else if _var.type_id() == TypeId::of::<u128>() {
        println!("u128")
        // ...
    } else {
        println!("I donno")
    }
}

struct Zeliboba();

fn main() {
    get_type_at_runtime(&1_u128);
    get_type_at_runtime(&"mgrachev".to_string());
    get_type_at_runtime(&Zeliboba);
}