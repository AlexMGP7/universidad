use std::collections::VecDeque;
use crate::cart;
pub struct Cola{
    cola:VecDeque<cart::Carrito>,
    nom_cajera:String,
    carritos_pendientes:u32,
    dinero_recaudado:f64,
    frente:u32,
    fondo:u32,
}

impl Cola {
    fn cola_nueva(nom_cajera:String ,carritos_pendientes:u32,dinero_recaudado:f64) -> Self{
        Cola { cola: (VecDeque::new()), nom_cajera: (nom_cajera), carritos_pendientes: (carritos_pendientes), dinero_recaudado: (dinero_recaudado) }
    }

    fn encolar(&mut self, carro_a_insertar: cart::Carrito){
        self.cola.push_back(carro_a_insertar)
    }

    fn desencolar(&mut self) -> Option<cart::Carrito>{
        self.cola.pop_front()
    }

    fn vacia(&mut self) -> bool{
        self.cola.is_empty()
    }

    fn frente(&mut self) -> Option<&mut cart::Carrito>{
        self.cola.front_mut()
    }

    fn fondo(&mut self) -> Option<&mut cart::Carrito>{
        self.cola.back_mut()
    }

    }

pub fn queue(){
    let mut cola1 = Cola::cola_nueva("alejandra".to_string(), 2, 50.3)
    let mut carro1=cart::Carrito::carrito_nuevo();
    let m=2.3;
    let n="pan";
    carro1.push(cart::Producto::producto_nuevo(m, n.to_string()));
    cola1.encolar(carro1);

    let mut valor=cola1.frente().unwrap();
    let mut cimacarro1=valor.cima().unwrap();
    cimacarro1[0]
    println!("{}",cimacarro1[0]);
}
