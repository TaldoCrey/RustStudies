fn main() {

    let vetor = vec![1, 5, 2, 81, 2, 1];

    let max = get_max(vetor);

    println!("Max = {}", max);

    let vetor = vec!["u", "y", "z", "x", "A"];

    let max_char = get_max(vetor);

    println!("Max Char = {}", max_char);

    let p1 = Points {x: 5, y: 10};
    let p2 = Points {x: 10.0, y: 20.0};
    let p3 = Points {x: 5, y: 10.0};

    println!("{}", p1.x());
    p2.y();

    let np = p2.mix(p3);

    println!("x = {} | y = {}", np.x, np.y);


}

fn get_max<T: PartialOrd + Copy>(v: Vec<T>) -> T { //<T> informa que a função recebe um tipo genérico T
                                                   // Para restringir quais tipos podem ser atribuidos à função
                                                   // Usamos traços (No caso Partial Ord e Copy são exemplos)
    let mut max = v[0];

    for n in v {
        if n > max {
            max = n;
        }
    }

    max

}

struct Points <T, U> {
    x: T,
    y: U
}

impl<A, B> Points<A, B> {
    fn x(&self) -> &A {
        &self.x
    }

    fn mix<C, D>(self, other: Points<C, D>) -> Points<A, C> {
        Points {
            x: self.x,
            y: other.x
        }
    }
}

impl Points<f64, f64> {
    fn y(&self) -> () {
        println!("X = {}", &self.x);
    }
}



/*

Em resumo, Generics Sâo usados para abstrair tipos de váriaveis, funções, structs, enums e blocos de implementação,
reduzindo a duplicação de código sem prejudicar a performance.
*/