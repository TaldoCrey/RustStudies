use std::io;

fn main() {
    
    loop {
        let mut nome = String::new();
        io::stdin().read_line(&mut nome).expect("Failed to read name");
        let nome = nome.trim();
        if nome == "Renan" {
            println!("Um ótimo dia senhor!!");
        } else {
            println!("Olá, {}", nome);
        }

        let mut idade = String::new();

        io::stdin().read_line(&mut idade).expect("Failed to read age");

        let idade = idade.trim().parse().unwrap_or(0);

        match idade {
            s if s < 18 => println!("Você é menor de idade. Saia daqui!"),
            s if s > 18 => println!("Você é maior de idade. Mas está muito velho para esse programa."),
            18 => println!("Como você ta?"),
            _ => println!("Você é humano?")
        };

        println!("Para sair desse programa você deve digitar 'TERMINATE'.");
        let mut cmd = String::new();

        io::stdin().read_line(&mut cmd).expect("Failed to read final command");

        let cmd = cmd.trim();

        { //Novo escopo artificial. Bom pra fazer debug.
            let mut cmd = cmd;
            println!("Olha que legal:");
            dbg!(cmd);
            cmd = "abusdjaksd";
            dbg!(cmd);
        }

        println!("Pegadinha do malandro.");

        if cmd.trim() == "TERMINATE" {
            break;
        } else {
            println!("Oh shit... Here we go again!!");
        }
    
    }

    println!("Tchauzinho!!!");

}
