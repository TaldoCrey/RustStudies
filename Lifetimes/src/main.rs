fn main() {
    /* let r;              // ---------------+------- 'r was born
                              
    {                         
        let x = 5;       // - x was born
        r = &x;                                         // This code generates an error because of x lifetime
    }                         // - x dies here

    println!("r: {}", r);  */
                            // ----------------------- 'r dies here

    let x = 5;

    let r = &x;

    println!("r: {}", r); //Esse código não gera erro, pois o lifetime de x acaba junto com o de r.

    let string1 = String::from("abcd");
    let res;
    {
        let string2 = String::from("xyz"); //O lifetime de string2 é menor que o de string1

        res = bigger(string1.as_str(), string2.as_str()); //O lifetime de res acaba junto com o fim do escopo
        println!("A maior string é: {}", res); 
    }
    //println!("A maior string é: {}", res); //Descomentar essa linha mostra um erro
                                             //Pois string2 morre ao fim do escopo, e com isso, caso res
                                             // Seja utilizado fora do escopo, ele apontará para um valor inválido


    let txt = String::from("Lorem Ipsum. Dolor");
    let first = txt.split('.').next().expect("Could not find the split specifier.");
    let i = ImportantExcerpt {
        part : first,
    }; //Se tentarmos usar i depois que first sair de escopo, teremos um erro de lifetime.

    let s: &'static str = "I have static lifetime"; //E isso significa que a referência vive durante todo o programa
                                                    //Toda String é um lifetime static.

}                             

//&i32 referencia normal
//&'a i32 referência com lifetime explicito
//&'a mut i32 referência mutável com lifetime explicito

fn bigger<'x>(a: &'x str, b: &'x str) -> &'x str { //O lifetime de retorno é sempre o menor
    if a.len() > b.len() {                         //O lifetime de retorno sempre deve estar atrelado ao lifetime
        a                                          //   de um dos parâmetros. Pois isso viola as Borrowing Rules.
    } else {
        b
    }
}

struct ImportantExcerpt<'j> {
    part: &'j str,
}

/*

Life Elision Rules----------------
1. Todo parametro que é uma referência tem seu próprio parametro lifetime

2. Se em exatamente um parametro lifetime de entrada, esse lifetime é associado a TODAS os lifetimes de saída

3. Se temos multiplos lifetimes de entrada, mas um deles é &self ou &mut self, o lifetime associado a self é associado
    a todos os lifetimes de saída

*/

fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item ) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..1];
        }
    }

    &s[..]
} 

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&'a self, announcement: &str) -> &'a str { //Não precisariamos especificar os lifetimes nos 
        println!("Attention please: {}", announcement);       //parametros de entrada e saida por conta da regra 3.
        self.part
    }
}