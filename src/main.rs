use std::io;

fn main() {
    println!("Bienvenue dans la calculatrice !");
    
    // Demander le premier nombre
    println!("Entrez le premier nombre :");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Erreur de lecture");
    let num1: f64 = match input1.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Veuillez entrer un nombre valide !");
            return;
        }
    };

    // Demander le deuxième nombre
    println!("Entrez le deuxième nombre :");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Erreur de lecture");
    let num2: f64 = match input2.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Veuillez entrer un nombre valide !");
            return;
        }
    };

    // Demander l'opération
    println!("Entrez une opération (+, -, *, /) :");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Erreur de lecture");
    let operation = operation.trim();

    // Calculer le résultat
    let result = match operation {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Division par zéro impossible !");
                return;
            }
            num1 / num2
        },
        _ => {
            println!("Opération non reconnue !");
            return;
        }
    };

    // Afficher le résultat
    println!("Le résultat de {} {} {} est : {}", num1, operation, num2, result);
}
