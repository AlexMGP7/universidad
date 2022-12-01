use std::collections::VecDeque;

use super::cart::Carrito;
#[derive(Debug)]
pub struct Cajera{
    pub nom_cajera:String,
    pub dinero_recaudado:f32,
    pub cola: VecDeque<Carrito>
}

impl Cajera{

    #[allow(dead_code)]
    pub fn cajera_nueva(nom_cajera:String,dinero_recaudado:f32) -> Self{
        Cajera{ nom_cajera, dinero_recaudado, cola:VecDeque::new() }
    }

    pub fn encolar(&mut self, carrito:Carrito){
        self.cola.push_back(carrito);
    }

    pub fn desencolar(&mut self){
        self.cola.pop_front();
    }

    #[allow(dead_code)]
    pub fn frente(&mut self) -> Option<&Carrito>{
        self.cola.front()
    }

    #[allow(dead_code)]
    pub fn fondo(&mut self) -> Option<&Carrito>{
        self.cola.back()
    }
}