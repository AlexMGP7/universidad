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
    pub fn encolar(&mut self, carrito:Carrito){ //Funcion para colas enconlar(Poner en la fila al carrito)
        self.linea.push_back(carrito);
    }

    #[allow(dead_code)]
    pub fn desencolar(&mut self){ //Funcion para colas desencolar(Quitar de la fila al carrito)
        self.linea.pop_front();
    }

    #[allow(dead_code)]
    pub fn frente(&mut self) -> Option<&Carrito>{//Lo que valga el frente en la cola en ese momento
        self.linea.front()
    }

    #[allow(dead_code)]
    pub fn fondo(&mut self) -> Option<&Carrito>{//Lo que valga el fondo en la cola en ese momento
        self.linea.back()
    }
    #[allow(dead_code)]
    pub fn carritos_en_espera(&mut self) -> usize{//Los carritos en espera de la cajera
        self.linea.len()
    }

    #[allow(dead_code)]
    pub fn sumar_dinero(&mut self, carrito:Carrito){//Sumar el dinero de un carro y agregarlo a total recaudado
        let monto_total_carrito=sumar_productos(carrito);//Se saca el monto total del carrito sumando los productos
        self.dinero_recaudado=self.dinero_recaudado+monto_total_carrito;//Se van sumando al dinero recaudado
        self.desencolar();//Se desencola el carrito

        pub fn sumar_productos(mut carrito:Carrito) -> f32{//Se suman los productos del carro (monto total)
            let mut suma=carrito.pop().unwrap().precio;//Se saca el producto con .pop y se guarda en suma
            loop {
                if carrito.cant_productos()==0{
                    return suma;//Eso lo hace hasta que la cantidad de productos en el carrito sea 0
                }
                suma += carrito.pop().unwrap().precio;//Si hay otro producto, se suma a los anteriores
            }
        }
    }
}