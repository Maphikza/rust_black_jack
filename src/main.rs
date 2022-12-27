use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

fn main() {
    println!("Enter the card.");
    let mut card = String::new();

    io::stdin()
        .read_line(&mut card)
        .expect("Could not readline");

    let card = card.trim().parse().expect("Make sure it is a string");

    println!("Card value is: {}", value_of_cards(card));
    println!("Higher card is: {}", higher_card())
}

fn value_of_cards(card: String) -> i32 {
    let mut cards = HashMap::new();
    cards.insert(String::from("J"), 10);
    cards.insert(String::from("Q"), 10);
    cards.insert(String::from("K"), 10);
    cards.insert(String::from("A"), 1);
    cards.insert(String::from("2"), 2);
    cards.insert(String::from("3"), 3);
    cards.insert(String::from("4"), 4);
    cards.insert(String::from("5"), 5);
    cards.insert(String::from("6"), 6);
    cards.insert(String::from("7"), 7);
    cards.insert(String::from("8"), 8);
    cards.insert(String::from("9"), 9);
    cards.insert(String::from("10"), 10);

    let chosen_card = String::from(card);
    let picked_card = cards.get(&chosen_card).copied().unwrap_or(0);
    return picked_card;
}

fn higher_card() -> String {
    println!("Choose Card one: ");

    let mut card_one = String::new();
    io::stdin()
        .read_line(&mut card_one)
        .expect("It's supposed to be a string.");
    let card_one: String = card_one.trim().parse().expect("Make sure it's a String.");

    println!("Choose card two: ");

    let mut card_two = String::new();
    io::stdin()
        .read_line(&mut card_two)
        .expect("It's supposed to be a string");
    let card_two: String = card_two.trim().parse().expect("Make sure it's a String.");

    let mut cards = HashMap::new();
    cards.insert(String::from("J"), 10);
    cards.insert(String::from("Q"), 10);
    cards.insert(String::from("K"), 10);
    cards.insert(String::from("A"), 1);
    cards.insert(String::from("2"), 2);
    cards.insert(String::from("3"), 3);
    cards.insert(String::from("4"), 4);
    cards.insert(String::from("5"), 5);
    cards.insert(String::from("6"), 6);
    cards.insert(String::from("7"), 7);
    cards.insert(String::from("8"), 8);
    cards.insert(String::from("9"), 9);
    cards.insert(String::from("10"), 10);

    let first_card = String::from(&card_one);
    let second_card = String::from(&card_two);
    let first_picked_card = cards.get(&card_one).copied().unwrap_or(0);
    let second_picked_card = cards.get(&card_two).copied().unwrap_or(0);

    match &first_picked_card.cmp(&second_picked_card) {
        Ordering::Equal => first_card + &second_card,
        Ordering::Greater => first_card,
        Ordering::Less => second_card,
    }
}
