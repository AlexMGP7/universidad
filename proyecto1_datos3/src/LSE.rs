#[derive(Clone)]
enum Lista<T> {
    None,
    Cola { item: T },
    Lista { item: T, next: Box<Lista<T>> }
}

#[derive(Clone)]
struct Cursor<T> {
    curr: Lista<T>
}

impl<T> Lista<T> where T: Copy {
    pub fn nuevo() -> Self {
        Self::None
    }
    pub fn eliminar(&mut self) -> Option<T> {
        match self {
            Self::None => None,
            Self::Cola { item } => {
                let item = *item;
                self.a_nada();
                Some(item)
            },
            Self::Lista { item, next } => {
                let mut n = Box::new(Self::None);
                let item = *item;
                std::mem::swap(next, &mut n);
                self.a_siguiente(*n);
                Some(item)
            }
        }
    }

    pub fn insertar(&mut self, x: T) {
        match self {
            Self::None => self.hacia_atras(x),
            Self::Cola { .. } => self.a_lista(x),
            Self::Lista { next, .. } => next.insertar(x)
        };
    }

    fn a_nada(&mut self) { *self = std::mem::replace(self, Lista::None); }

    fn hacia_atras(&mut self, it: T) {
        *self = match self {
            Self::None => Self::Cola { item: it },
            Self::Lista { item:_, next:_ } => Self::Cola { item: it },
            _ => panic!("Supplied value was not of correct type or variant.")
        }
    }

    fn a_siguiente(&mut self, nxt: Lista<T>) {
        *self = nxt;
    }

    fn a_lista(&mut self, x: T) {
        *self = match self {
            Self::Cola { item } => {
                Self::Lista { item: *item, next: Box::new(Self::Cola { item: x }) }
            },
            _ => { panic!("no se pudo enviar a la lista"); }
        };
    }
}

impl<T> IntoIterator for Lista<T> where T: Copy {
    type Item = T;
    type IntoIter = Cursor<T>;

    fn into_iter(self) -> Self::IntoIter {
        Cursor {
            curr: self
        }
    }
}

impl<T> Iterator for Cursor<T> where T: Copy {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let nxt = match self.curr {
            Lista::None => None,
            Lista::Cola { item } => {
                self.curr = Lista::None;
                Some(item)
            },
            Lista::Lista { item, ref mut next } => {
                let mut n = Box::new(Lista::None);
                std::mem::swap(next, &mut n);
                self.curr = *n;
                Some(item)
            }
        };
        nxt
    }
}


pub fn Test() {

    let mut lista: Lista<i32> = Lista::nuevo();
    let mut lista2: Lista<i32> = Lista::nuevo();

    lista.insertar(1);
    lista.insertar(2);
    lista.insertar(3);
    lista.insertar(4);

    lista2.insertar(10);
    lista2.insertar(20);
    lista2.insertar(30);

    println!("{}", lista2.eliminar().unwrap());
    println!("{}", lista2.eliminar().unwrap());
    println!("{}", lista2.eliminar().unwrap());
    println!("---");

    for i in lista.clone() {
        println!("{}", i);
    }

    for i in lista.clone().into_iter().map(|x| x * 2) {
        println!("{}", i)
    }

    for (i, x) in lista.into_iter().enumerate() {
        println!("iter2: {}, {}", i, x);
    }
}