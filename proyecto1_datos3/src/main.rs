use objects::{products::Producto, cart::Carrito, cashiers::Cajera};
mod objects;
fn main(){
    let producto=Producto::producto_nuevo(3.2, "huevo".to_string());
    println!("{:?}",producto);
    let mut carrito=Carrito::carrito_nuevo();
    carrito.push(producto);
    println!("{:?}",carrito);
    let mut cajera=Cajera::cajera_nueva("yuleitsis".to_string(), 3.2);
    cajera.encolar(carrito);
    println!("{:?}",cajera.frente());
    println!("{:?}",cajera);
}