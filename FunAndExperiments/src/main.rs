use std::io;
use rand::seq::IndexedRandom;


fn main() {
    loop {
        let mut opt = String::new();
        println!("Escolha uma opção: ");
        println!("[1] Simulador de Dados");
        println!("[0] Sair");

        io::stdin().read_line(&mut opt).expect("Failed to read option.");
        let opt = opt.trim();

        let opt:i32 = match opt.parse() {
            Ok(v) => v,
            Err(_) => {
                -1
            }
        };
        
        if opt == 0 {
            break;
        } else if opt == -1 {
            continue;
        } else if opt == 1 {
            dice_sim();
        }
        
    }
}

enum Dice {
    D3,
    D4,
    D6,
    D10,
    D12,
    D20
}

fn dice_sim() {
    loop{
        println!("Escolha qual tipo de dado você deseja: ");
        println!("[1]D3\t[2]D4\n[3]D6\t[4]D10\n[5]D12\t[6]D20");

        let mut chosen_dice = String::new();
        io::stdin().read_line(&mut chosen_dice).expect("Failed to read chosen Dice");
        let my_dice: Dice = match chosen_dice.trim().parse() {

            Ok(v) => {
                match v {
                    1 => Dice::D3,
                    2 => Dice::D4,
                    3 => Dice::D6,
                    4 => Dice::D10,
                    5 => Dice::D12,
                    6 => Dice::D20,
                    _ => {
                        println!("You have chosen such a weird option... We'll send you the most common one!");
                        Dice::D6
                    }
                }
            },
            Err(_) => {
                println!("I think you are a litte confuse... I'm sending you the common one!");
                Dice::D6
            }
        };

        let range = match my_dice {
            Dice::D3 => 1..4,
            Dice::D4 => 1..5,
            Dice::D6 => 1..7,
            Dice::D12 => 1..13,
            Dice::D10 => 1..11,
            Dice::D20 => 1..21
        };

        let mut numbers: Vec<i32> = Vec::new();

        for i in range {
            numbers.push(i);
        }

        loop {
            let mut rng = rand::rng();
            let random_number = numbers.choose(&mut rng).expect("Something went wrong on l85.");

            let dice = match my_dice {
                Dice::D3 => String::from("D3"),
                Dice::D4 => String::from("D4"),
                Dice::D6 => String::from("D6"),
                Dice::D12 => String::from("D12"),
                Dice::D10 => String::from("D10"),
                Dice::D20 => String::from("D20")
            };

            println!("Você rolou um {} e obteve {}!", dice, random_number);
            println!("Deseja re-rolar? [1] Sim / [2] Não");
            let mut reroll = String::new();
            io::stdin().read_line(&mut reroll).expect("Failed on reading reroll choice!");
            let reroll = match reroll.trim().parse() {
                Ok(v) => v,
                Err(_) => 2
            };

            match reroll {
                1 => continue,
                _ => break
            }
        }

        println!("Você deseja escolher outro tipo de dado? [1] Sim / [2] Não");
        let mut type_change = String::new();
        io::stdin().read_line(&mut type_change).expect("Failed on reading type_change choice!");
        let type_change = match type_change.trim().parse() {
            Ok(v) => v,
            Err(_) => 2
        };

        match type_change {
            1 => continue,
            _ => break
        }
    }
}