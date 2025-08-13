use std::collections::{hash_map, HashMap};
//Vetores normalmente armazenam um tipo só de informação, mas podemos contornar isso usando as variações
//de um mesmo tipo providas de um enum:

enum Color {
    Name(String),
    RGB(i32, i32, i32),
    Hex(String)
}
fn main() {
    let mut a = [1, 2, 3];
    let mut vector: Vec<i32> = Vec::new(); //Inicializa um novo vetor vazio

    vector.push(1);
    vector.push(2);

    let v2 = vec![1, 2, 3, 4, 5]; //Inicializa um vetor já com os valores de 1 a 5

    let third = &v2[2]; //unsafe method to acces vector elements.
    println!("O terceiro elemento de v2 é: {}", third);

    match v2.get(2) {
        Some(third) => println!("O terceiro elemento de v2 é: {}", third),
        None => println!("Não existe terceiro elemento em v2.")
    }

    println!("Vejamos todos os elementos de a: ");
    for e in &mut a {
        println!("{} is an element of a!", e);
        println!("Lets sum 2 in all a elements!");
        *e += 2;
    }

    for e in &a {
        println!("{} is new in a!", e);
    }

    let colors = vec![Color::RGB(255, 120, 10), Color::Name(String::from("Blue")), Color::Hex(String::from("#FFFFF"))];

    match &colors[1] {
        Color::Name(c) => println!("Its the {} color!", c),
        _ => println!("I can't figure out the name of this color!")
    }

    let mut h = HashMap::new();

    h.insert(String::from("key1"), 1);
    h.insert(String::from("k2"), 20);

    let value = h.get(&String::from("key1"));

    match value {
        Some(v) => println!("Value = {}", v),
        None => println!("None value has been loaded...")
    }

    for (key, value) in &h {
        println!("Key: {} => Value: {}", key, value);
    }

    println!("Observe que key1 e k2 já possuem seus valores preenchidos.");
    h.insert(String::from("key1"), 10); //Isso sobrepõe o valor de key1;
    match h.get(&String::from("key1")) {
        Some(v) => println!("New value of key1 = {}", v),
        None => println!("No value has been loaded!")
    }

    h.entry(String::from("k2")).or_insert(10); //Isso preenche o valor de k2 se não ouver nada já preenchido.
    match h.get(&String::from("k2")) {
        Some(v) => println!("Value of k2 => {}", v),
        None => println!("No value has been loaded!")
    }
}
