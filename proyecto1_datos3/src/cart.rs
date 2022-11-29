struct Producto{
    n_producto:String,
    p_precio:f64
}

impl Producto {
    fn producto_nuevo(precio:f64, producto:String)->Self{ Producto {
        n_producto:producto, p_precio:precio,
    } }
}

struct Carrito{
    carrito:Vec<Producto>,
    max_productos:u32,
    tope:u32,
}

impl Carrito {
    fn carrito_nuevo() -> Self{
        Carrito { carrito: (Vec::new()), max_productos: (15), tope: (0) }
    }

    fn pop(&mut self) -> Option<Producto>{
        if !self.vacia(){
            self.tope-=1;
            return self.carrito.pop();
        }
        else {
            println!("No se puede sacar el producto, carrito lleno");
            return None;
        }

    }

    fn llena(&self) -> bool {
        if self.tope >= self.max_productos{
            true
        }
        else {
            false
        }
        }

    fn push(&mut self, producto_a_insertar: Producto){
        if self.llena(){
            println!("No se puede agregar mas productos, carrito lleno")
        }
        else{
            self.carrito.push(producto_a_insertar);
            self.tope+=1;
        }
    }

    fn vacia(&self) -> bool {
        self.carrito.is_empty()
    }

    fn cant_productos(&self) -> usize {
        self.carrito.len()
    }

    fn cima(&self) -> Option<&Producto> {
        self.carrito.last()
    }
}

pub fn cart(){
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

}