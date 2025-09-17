use rand;
use std::io;

fn main() {
    println!("Devine le nombre");

    let nb = rand::random_range(0..=100);
    let mut nb_essai = 0;

    loop {
        nb_essai += 1;
        println!("Saisissez votre essai :");
        let mut essai = String::new();
        if let Err(_) = io::stdin().read_line(&mut essai) {
            println!("Saisie invalide");
            continue;
        }

        match essai.trim().parse::<u32>() {
            Ok(x) => match x.cmp(&nb) {
                std::cmp::Ordering::Less => println!("Trop petit"),
                std::cmp::Ordering::Equal => {
                    println!("Trouvé, {x} était le bon nombre.\nIl a fallu {nb_essai} essai(s)");
                    break;
                }
                std::cmp::Ordering::Greater => println!("Trop grand"),
            },
            Err(_) => println!("Saisie invalide"),
        }
    }
}
