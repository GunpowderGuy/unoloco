#[macro_use]
extern crate text_io;

#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate derive_getters;

extern crate rand;

mod card;
mod deck;
mod hand;

fn main() {
    let i: i32 = read!();

    println!(
        "ejemplo de input y output
    , el numero ingresado fue {}",
        i
    );

    let numCards = 5;
    p1 = hand::Hand::new();
}
