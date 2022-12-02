use std::collections::VecDeque;//Usamos la libreria local VecDeque, que tiene todas las funciones de una cola
use std::{fs};//Libreria local para leer el archivo .txt entrante

#[derive(Debug)]
#[derive(Clone)]
pub struct Producto{        //Se crea la estructura producto (Equivalente a un nodo hablando en un caso general)
    pub producto:String,  //Nombre del producto
    pub precio:f32        //Precio del producto
}

impl Producto {         //Las implementaciones del producto
    #[allow(dead_code)]
    pub fn producto_nuevo(precio:f32, producto:String)->Self{
        Producto { //Funcion para crear un producto nuevo
            producto:producto, precio:precio,   //Las variables de las estructuras valdran lo que se pase aca por parametros
        }
    }
}
#[derive(Debug)]
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


#[allow(dead_code)]
pub fn leer_cajera1() -> Cajera{
    println!("Leyendo el archivo de entrada {}","QSM.txt");
    let contenido = fs::read_to_string("QSM.txt").expect("Something went wrong reading the file");

    let arr_text: Vec<&str> =contenido.lines().collect();

    let mut cajeros:Vec<&str>= Vec::new();
    for x in 0..3{
        cajeros.push(arr_text[x]);
        println!("[{}]",cajeros[x]);
    }

    let data=cajeros[0].trim().split(" ").collect::<Vec<&str>>();
    let mut arr_datos_cajero:Vec<&str>=Vec::new();
    for x in data{
        arr_datos_cajero.push(x);
    }

    return Cajera::cajera_nueva(arr_datos_cajero[0].to_string(), arr_datos_cajero[1].parse::<f32>().unwrap());

    //println!("{:?}",arr_datos_cajero);

    /* let datos_cajeros2=cajeros[1].split_whitespace();
    let mut arr_datos_cajero2:Vec<&str>=Vec::new();
    for x in datos_cajeros2{
        arr_datos_cajero2.push(x);
    }

    let datos_cajeros3=cajeros[2].split_whitespace();
    let mut arr_datos_cajero3:Vec<&str>=Vec::new();
    for x in datos_cajeros3{
        arr_datos_cajero3.push(x);
    }

    let mut carritos=Vec::new();
    for x in 3..arr_text.len(){
        carritos.push(arr_text[x]);
        println!("[{}]",carritos[x-3]);
    } */

}

fn main(){/*
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
    println!("{}",cajera.nom_cajera);
    println!("Carritos en cola de la cajera: {}",cajera.carritos_en_espera()); */

    let cajera1=leer_cajera1();
    println!("{:?}",cajera1);

}
