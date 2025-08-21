use SuperTraits::{Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing SelectBox!");
    }
}

fn main() {
    let screen = Screen {components: vec![
         Box::new(SelectBox {
            width: 100,
            height: 100,
            options: vec![
                String::from("Sim"),
                String::from("Não"),
                String::from("Cancelar")
            ]
         }),
         Box::new(Button {
            width: 100,
            heigh: 50,
            label: String::from("Apply")
         })
    ]};
}
