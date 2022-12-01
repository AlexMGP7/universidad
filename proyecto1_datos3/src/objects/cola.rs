use std::collections::VecDeque;
use super::cart::Carrito;

#[allow(dead_code)]
pub struct Queue{
    pub cola:VecDeque<Carrito>,
    pub nom_cajera:String,
    pub carritos_pendientes:String,
    pub dinero_recaudado:String,

}

impl Queue {
    #[allow(dead_code)]
    pub fn cola_nueva(nom_cajera:String ,carritos_pendientes:String,dinero_recaudado:String) -> Self{
        Queue { cola: (VecDeque::new()), nom_cajera: (nom_cajera), carritos_pendientes: (carritos_pendientes), dinero_recaudado: (dinero_recaudado) }
    }

    #[allow(dead_code)]
    pub fn encolar(&mut self, carro_a_insertar: Carrito){
        self.cola.push_back(carro_a_insertar)
    }

    #[allow(dead_code)]
    pub fn desencolar(&mut self) -> Option<Carrito>{
        if !self.vacia(){
            self.cola.pop_front()
        }
        else{
            println!("No se puede desencolar, la cola esta vacia");
            return None;
        }
    }

    pub fn vacia(&mut self) -> bool{
        self.cola.is_empty()
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

/* pub fn queue(){ //Pruebas
    let mut cola1 = Queue::cola_nueva("alejandra".to_string(), "2".to_string(), "50.3".to_string());
    let mut carro1=cart::Carrito::carrito_nuevo();
    let carro11=cart::Carrito::carrito_nuevo();
    let m=2.3;
    let n="pan";
    let m11=5.4;
    let n11="azucar";
    carro1.push(cart::Producto::producto_nuevo(m, n.to_string()));
    carro1.push(cart::Producto::producto_nuevo(m11, n11.to_string()));
    cola1.encolar(carro1);
    let valor=cola1.frente().unwrap();
    let cimacarro1=valor.cima().unwrap();
    println!("{} {}",cimacarro1.n_producto, cimacarro1.p_precio);
    cola1.encolar(carro11);
    println!("el nombre de la cajera es {}",cola1.nom_cajera);
    let valor=cola1.frente().unwrap();
    let cimacarro1=valor.cima().unwrap();
    println!("{} {}",cimacarro1.n_producto, cimacarro1.p_precio);
    let mut cola2 = Queue::cola_nueva("maria".to_string(), 3, 23.3);
    let mut carro2=cart::Carrito::carrito_nuevo();
    let m2=4.4;
    let n2="huevo";
    carro2.push(cart::Producto::producto_nuevo(m2, n2.to_string()));
    cola2.encolar(carro2);
    println!("el nombre de la cajera es {}",cola2.nom_cajera);
    let valor2=cola2.frente().unwrap();
    let cimacarro2=valor2.cima().unwrap();
    println!("{} {}",cimacarro2.n_producto, cimacarro2.p_precio);
}
 */