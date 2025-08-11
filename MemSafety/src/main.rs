

// ---------- Ownership Rules ------------
// 1. Todo valor em Rust tem uma váriavel dona.
// 2. Só pode existir um dono por vez.
// 3. Quando o dono sai do escopo, o valor é descartado.
fn main() {
    let s = String::from("Roberto");
    println!("Outside: {}", s);

    { //Criamos um escopo artificial.
        //s não existe daqui pra tras
        let s  = 1; //s agora existe dentro deste escopo.
        //podemos efetuar operações e processos utilizando s
        println!("Inside: {}", s);
    } //ao fim do escopo, o valor de s será descartado pois o dono de s sairá do escopo.
    // Como s estava definida antes do escopo, seu valor volta a ser o original, e o valor/modificações feitas dentro
    //      do escopo serão todas descartadas.
    println!("Outside (again): {}", s);

    //Ficar atento ao passar váriaveis como parâmetros e afins, pois a varíavel passada por parametro recebe um dono
    //  que será descartado ao fim do escopo da função. Além disso, tomar cuidado ao passar váriaveis como valor de
    //  outras variáveis, pois, ela passa de dona para objeto, e isso afeta o Borrowing system.

    //IMPORTANTE -> Valores numéricos são copiados, e STRINGS vão dar erro no Borrowing system pois são armazenadas
    //  no heap. String = ponteiro.

    /*
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of {} is {}", s1, len);
     */ //Isso é um exemplo de uma função que daria erro no Borrowing System. Vamos ver como arrumá-la com ponteiros.

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len); //Aqui teríamos um erro em s1 (caso não utilizassemos ponteiros)
    // O Borrowing system nos acusa de estar pegando emprestado o valor de uma váriavel que já foi dado a outro dono:
    //  o parametro da função calculate_length. E sendo assim, ao fim do escopo da função, o valor do parametro foi
    //  descartado. Por isso o erro. Utilizando ponteiros, o parametro da função recebe o endereço da memória e não o
    //  valor da váriavel. Dessa forma, apenas o endereço da memória é descartado pela função ao fim de seu escopo,
    //  e não o valor da variável.


    // - Passar valores por referencia é chamado de BORROWING, pois emprestamos o valor de uma variável sem ceder
    //   a posse sobre ela. Não podemos emprestar uma váriavel usando uma referência mutável duas vezes. 
    //   Referências imutáveis podem ser emprestadas várias vezes. Vantagem: Não tem como um ponteiro ler um valor
    //   enquanto outro ponteiro o modifica simultâneamente.

    let mut s = String::from("Pega"); //Aqui definimos s como uma váriavel mutável

    let r1 = &s; //Referência imutável de s
    let r2 = &s; //Referência imutável de s
    println!("r1 -> {}|r2 -> {}", r1, r2); //r1 e r2, referências imutáveis de s, são utilizadas aqui, e somente aqui.
    //Aqui, r1 e r2 já estão fora de escopo. E isso possibilita que r3 seja utilizado como uma referência mutável de s.
    let r3 = &mut s;
    //let r4 = &s; caso esta linha não estivesse comentada, teriamos um erro de empréstimo, pois r3 ainda está em escopo,
    //e não podemos ter uma referência imutável enquanto uma referência mutável está em escopo.
    r3.push_str("aaaa");
    println!("r3 -> {}", r3);


    // ---------- Regras de Referenciação --------------
    /*
        1. Você pode ter apenas uma referência mutável ou incontáveis imutáveis dentro de um mesmo escopo
        2. Referências tem sempre que ser válidas!
     */

    //Para pegarmos pedaços de strings e arrays, usamos um slice:

    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2]; //Aqui pegaremos um pedaço de a: o index 0 e 1.
    println!("Slice values: (Knowing a = [1, 2, 3, 4, 5]):");
    for (i, n) in slice.iter().enumerate() {
        println!("#{} -> {}", i, n);
    }
}

fn calculate_length(s: &String) -> usize { //Parametro por referência é imutável a não ser que tornemos ele mutável com 'mut'.
    let length = s.len();
    length //retorna a length da string
}

/*
fn calculate_length(s) -> (String, usize) {

    let length = s.len();
    return (s, length);

}
*/