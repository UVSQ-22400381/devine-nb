use std::io;

fn main() {
    println!("Devine le nombre");

    let nb = 10;
    let mut essai = String::new();

    loop {
        if let Err(_) = io::stdin().read_line(&mut essai) {
            println!("Saisie invalide");
            continue;
        }

        match essai.parse::<i32>() {
            Ok(x) => match x.cmp(&nb) {
                std::cmp::Ordering::Less => println!("Trop petit"),
                std::cmp::Ordering::Equal => {
                    println!("Trouvé");
                    break;
                }
                std::cmp::Ordering::Greater => println!("Trop grand"),
            },
            Err(_) => println!("Saisie invalide"),
        }
    }
}
