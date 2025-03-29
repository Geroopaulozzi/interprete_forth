fn main() {
    println!("Hello, world!");
}

struct Pila {
    pila: Vec<i16>,
    max: usize,
}

impl Pila {
    fn new(max: usize) -> Self {
        Pila {
            pila: Vec::with_capacity(max),
            max,
        }
    }

    fn agregar_valor(&mut self, valor: i16) {
        self.pila.push(valor)
    }
    fn sacar_valor(&mut self) -> i16 {
        if self.pila.len() > 0 {
            self.pila.pop()
        }
    }
}