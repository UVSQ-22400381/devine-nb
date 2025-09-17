use std::io;

fn main() {
    println!("Devine le nombre");

    let nb = rand::random_range(0..=100);
    let mut nb_essai = 0;

    loop {
        nb_essai += 1;
        println!("Saisissez votre essai :");
        let mut essai = String::new();
        if io::stdin().read_line(&mut essai).is_err() {
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
