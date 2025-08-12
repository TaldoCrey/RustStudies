
struct User {
    username: String,
    email: String,
    sing_in_count: u64,
    active: bool,
}
#[derive(Debug)]
struct Point(i32, i32, i32);
#[derive(Debug)]
struct Color(i32, i32, i32);

impl User { //Método de struct [User.função()]
    fn deactivate(&mut self) {
        self.active = false;
    }

    fn sum_count(&mut self, amount:u64) {
        self.sing_in_count = self.sing_in_count + amount;
    }
}

impl User { //Função de struct [User::função()]
    fn admin() -> User {
        User {
            username: String::from("admin"),
            email: String::from("admin@admin.com"),
            sing_in_count: 1,
            active: true
        }
    }
}
// ------------ Struct ----------
// Praticamente igual aos structs em C
fn main() {
    let mut user1 = User {
        username: String::from("teste123"),
        email: String::from("teste@hotmail.com"),
        active: true,
        sing_in_count: 1
    };

    let name = user1.username;
    user1.username = String::from("testado456");
    println!("Stored name: {} | Strcut name {}", name, user1.username);

    let mut user2 = build_user(String::from("user2@hotmail.com"), String::from("SecTeste123"));

    let mut user3 = User {
        username: String::from("roberto"),
        email: String::from("robert@hotmail.com"),
        ..user2 //Username e email, serão preenchidos conforme o que especificamos. O resto dos atributos
        //        será preenchido copiando os valores do user2.
    };

    user3.deactivate();
    println!("user3 state: {}", user3.active);

    user2.sum_count(20);
    println!("user2 has logged in {} time(s)!", user2.sing_in_count);

    let admin = User::admin();

    println!("Username: {}\nEmail: {}\nTimes Logged: {}\nAccount current state: {}", admin.username, admin.email, admin.sing_in_count, admin.active);

    //Ambos códigos abaixos são tuplas, porem, podemos tipá-las em rust para garantir uma precisão a mais em nossas funções.
    let blue = Color(0, 0, 255);
    let origin = Point(0, 0, 0);
    println!("Both are tuples of same input types, but the tuples type are different!");
    dbg!(blue);
    dbg!(origin);

}

fn build_user(email:String, username:String) -> User {

    User {
        email, //equivale a email: email,
        username, //equivale a username: username
        sing_in_count: 1,
        active: true,
    } //Retorna o struct novo

}
