use objects::{products::Producto, cart::Carrito, cashiers::Cajera};
mod objects;
fn main(){
    let producto=Producto::producto_nuevo(3.2, "huevo".to_string());
    let producto2=Producto::producto_nuevo(2.8, "pan".to_string());
    let producto3=Producto::producto_nuevo(3.2, "huevo".to_string());
    let producto4=Producto::producto_nuevo(2.8, "pan".to_string());
    println!("Creando producto 1:");
    println!("{:?}",producto);
    println!("Creando producto 2:");
    println!("{:?}",producto2);
    println!("Creando producto 3:");
    println!("{:?}",producto3);
    println!("Creando producto 4:");
    println!("{:?}",producto4);
    let mut carrito=Carrito::carrito_nuevo();
    let mut carrito2=Carrito::carrito_nuevo();
    carrito.push(producto);
    carrito.push(producto2);
    carrito2.push(producto3);
    carrito2.push(producto4);
    println!("Creando carrito nuevo e introduciendo los productos: ");
    println!("{:?}",carrito);
    let mut cajera=Cajera::cajera_nueva("yuleitsis".to_string(), 0.0);
    cajera.encolar(carrito.clone());
    println!("Agregando el carrito con productos a la cola de una cajera: ");
    println!("{:?}",cajera);
    cajera.sumar_dinero(carrito.clone());
    println!("La suma de los precios del producto del carro de la primera cajera: {}",cajera.dinero_recaudado);
    println!("Despues de sumar y agregar al recaudo de la cajera, el carro se saca de la cola(se atiende): ");
    println!("{:?}",cajera);
    println!("Carritos en cola de la cajera 1: {}",cajera.carritos_en_espera())
}