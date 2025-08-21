/*
Traits são usados para definir comportamentos comuns entre diferentes tipos genéricos.
*/

struct NewsArcticle {
    author: String,
    headline: String,
    content: String
}

impl Summary for NewsArcticle {
    fn sum_author(&self) -> String {
        format!("{}", self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

impl Summary for Tweet {
    fn sum_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content) //Sobreposição da função pelo Struct Tweet
    }
}

pub trait Summary {
    fn sum_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read More from {}...)", self.sum_author()) // Corpo padrão da função
    }
}

fn main() {
    let news = NewsArcticle {
        author: String::from("Renan"),
        headline: String::from("Pesquisa UFSCAR!"),
        content: String::from("Lorem Ipsum...")
    };

    let tweet = Tweet {
        username: String::from("TaldoCrey_"),
        content: String::from("Hello, World!"),
        reply: false,
        retweet: false
    };

    println!("{}\n{}", tweet.summarize(), news.summarize());

    notify(&news);
}

/* fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize()); Funciona, mas há uma sintaxe melhor.
} */

fn notify<T>(item: &T) 
    where T: Summary
{ //O tipo genérico T está limitado para receber apenas tipos que possuem o traço Summary
    println!("Breaking News! {}", item.summarize());
}
