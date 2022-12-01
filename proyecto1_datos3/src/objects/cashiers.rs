use std::collections::VecDeque;//Usamos la libreria local VecDeque, que tiene todas las funciones de una cola
use super::cart::Carrito;//Exportamos los carritos en la cajera

#[derive(Debug)]//Para poder testear y ver todo los atributos que tiene el objeto de manera general
pub struct Cajera{//Creamos el objeto cajera, la cual sera nuestra "cola" hablando en terminos generales
    pub nom_cajera:String,
    pub dinero_recaudado:f32,
    pub linea: VecDeque<Carrito>//Creamos la linea de carritos de la cajera
    //(Los carritos en espera, es un metodo de la implementacion de la estructura, no un atributo)
}

impl Cajera{

    #[allow(dead_code)]
    pub fn cajera_nueva(nom_cajera:String,dinero_recaudado:f32) -> Self{//Funcion para crear una cajera nueva
        Cajera{ nom_cajera, dinero_recaudado, linea:VecDeque::new() }
    }

    #[allow(dead_code)]
    pub fn encolar(&mut self, carrito:Carrito){
        self.linea.push_back(carrito);
    }

    #[allow(dead_code)]
    pub fn desencolar(&mut self){
        self.linea.pop_front();
    }

    #[allow(dead_code)]
    pub fn frente(&mut self) -> Option<&Carrito>{
        self.linea.front()
    }

    #[allow(dead_code)]
    pub fn fondo(&mut self) -> Option<&Carrito>{
        self.linea.back()
    }
    #[allow(dead_code)]
    pub fn carritos_en_espera(&mut self) -> usize{
        self.linea.len()
    }

    pub fn sumar_dinero(&mut self, carrito:Carrito){//Sumar el dinero de un carro y agregarlo a total recaudado
        let dinero_carrito=sumar_productos(carrito);
        self.dinero_recaudado=self.dinero_recaudado+dinero_carrito;
        self.desencolar();

        pub fn sumar_productos(mut carrito:Carrito) -> f32{
            let mut suma=carrito.pop().unwrap().precio;
            loop {
                if carrito.cant_productos()==0{
                    return suma;
                }
                suma = suma + carrito.pop().unwrap().precio;
            }
        }
    }
}