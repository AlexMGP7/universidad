use super::products::Producto;

pub struct Carrito{     //Se crea la estructura Carrito (Equivalente a la pila hablando de un caso general)
    pub carrito:Vec<Producto>,  //El contenedor de productos del carrito como vector
    pub max_productos:u32,      //El maximo de productos para el carrito
    pub tope:u32,               //El tope actual del carrito (Se utiliza mas o menos como un indice)
}

#[allow(dead_code)]
impl Carrito {      //Las implementaciones del carrito
    pub fn carrito_nuevo() -> Self{     //Funcion para crear un carrito nuevo
        Carrito { carrito: (Vec::new()), max_productos: (15), tope: (0) }
        //Se crea un vector (carrito) nuevo para meter los productos, se establece los maximos de productos
        //Y se empieza del tope 0 (aunque se sumara a 1 con el push)
    }

    pub fn pop(&mut self) -> Option<Producto>{ //Funcion para eliminar el elemento en el tope, y devuelve su valor a la vez
        if !self.vacia(){   //Si la pila no esta vacia entonces se resta uno al tope y se elimina el ultimo elemento pusheado
            self.tope-=1;
            return self.carrito.pop();
        }
        else {
            println!("No se puede sacar el producto, carrito lleno");
            return None;
        }

    }

    pub fn llena(&self) -> bool {       //Funcion para saber si el carrito esta lleno de productos
        if self.tope >= self.max_productos{
            true
        }
        else {
            false
        }
        }

    pub fn push(&mut self, producto_a_insertar: Producto){  //Funcion para agregar un producto al carrito por el tope
        if self.llena(){
            println!("No se puede agregar mas productos, carrito lleno")
        }
        else{
            self.carrito.push(producto_a_insertar);
            self.tope+=1;
        }
    }

    pub fn vacia(&self) -> bool {   //Funcion para saber si el carrito no tiene ningun producto
        self.carrito.is_empty()
    }

    pub fn cant_productos(&self) -> usize {     //Funcion para saber cuantos productos hay
        self.carrito.len()                  //No le veo mucha utilidad por ahora, pero se me ocurrio y es mejor tenerla :D
    }

    pub fn cima(&self) -> Option<&Producto> {   //Funcion que devuelve el producto que esta en la cima
        self.carrito.last()
    }
}

/* pub fn cart(){  //Pruebas
    let mut carrito1=Carrito::carrito_nuevo();
    let m=2.3;
    let n="pan";
    carrito1.push(Producto::producto_nuevo(m, n.to_string()));
    let producto_en_la_cima=carrito1.cima().unwrap();
    println!("{} {}",producto_en_la_cima.n_producto, producto_en_la_cima.p_precio);
    println!("El tope actualmente es: {}",carrito1.tope);
    let m=4.4;
    let n="huevo";
    carrito1.push(Producto::producto_nuevo(m, n.to_string()));
    let producto_en_la_cima=carrito1.cima().unwrap();
    println!("{} {}",producto_en_la_cima.n_producto, producto_en_la_cima.p_precio);
    println!("El tope actualmente es: {}",carrito1.tope);
    let cant_productos=carrito1.cant_productos();
    println!("Actualmente hay {} productos en el carrito",cant_productos);
    let borrar=carrito1.pop().unwrap();
    println!("Sacando al producto {}",borrar.n_producto);
    let producto_en_la_cima=carrito1.cima().unwrap();
    println!("{} {}",producto_en_la_cima.n_producto, producto_en_la_cima.p_precio);

    while !carrito1.vacia(){

        let m=4.4;
        let n="huevo";
        carrito1.push(Producto::producto_nuevo(m, n.to_string()));

        let borrar=carrito1.pop().unwrap();
        println!("Sacando al producto {}",borrar.n_producto);
        let borrar=carrito1.pop().unwrap();
        println!("Sacando al producto {}",borrar.n_producto);

    }

} */