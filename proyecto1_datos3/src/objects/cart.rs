use super::products::Producto;
#[derive(Debug)]
#[derive(Clone)]
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

    #[allow(dead_code)]
    pub fn push(&mut self, producto_a_insertar: Producto){  //Funcion para agregar un producto al carrito por el tope
        if self.llena(){
            println!("No se puede agregar mas productos, carrito lleno")
        }
        else{
            self.carrito.push(producto_a_insertar);
            self.tope+=1;
        }
    }

    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<Producto>{ //Funcion para eliminar el elemento en el tope, y devuelve su valor a la vez
        if !self.vacia(){self.tope-=1;return self.carrito.pop();   //Si la pila no esta vacia entonces se resta uno al tope y se elimina el ultimo elemento pusheado
            }else {println!("No se puede sacar el producto, carrito lleno");return None;}
    }

    #[allow(dead_code)]
    pub fn llena(&self) -> bool {       //Funcion para saber si el carrito esta lleno de productos
        if self.tope >= self.max_productos{true}else {false}
    }

    #[allow(dead_code)]
    pub fn vacia(&self) -> bool {   //Funcion para saber si el carrito no tiene ningun producto
        self.carrito.is_empty()
    }

    #[allow(dead_code)]
    pub fn cant_productos(&self) -> usize {     //Funcion para saber cuantos productos hay
        self.carrito.len()                  //No le veo mucha utilidad por ahora, pero se me ocurrio y es mejor tenerla :D
    }

    #[allow(dead_code)]
    pub fn cima(&self) -> Option<&Producto> {   //Funcion que devuelve el producto que esta en la cima
        self.carrito.last()
    }
}

/* pub fn cart(){  //Pruebas
*/