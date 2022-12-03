//Alexander Gonzalez / 30.230.460

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
#[derive(Clone)]
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
#[derive(Clone)]
#[derive(Debug)]//Para poder testear y ver todo los atributos que tiene el objeto de manera general
pub struct Cajera{//Creamos el objeto cajera, la cual sera nuestra "cola" hablando en terminos generales
    pub nom_cajera:String,
    pub dinero_recaudado:f32,
    pub linea_texto: VecDeque<Carrito>,//Creamos la linea_texto de carritos de la cajera
    pub carritos_en_espera:u32,//Los carritos en espera
}

impl Cajera{

    #[allow(dead_code)]
    pub fn cajera_nueva(nom_cajera:String,dinero_recaudado:f32, carritos_en_espera:u32) -> Self{//Funcion para crear una cajera nueva
        Cajera{ nom_cajera, dinero_recaudado, linea_texto:VecDeque::new(), carritos_en_espera }
    }

    #[allow(dead_code)]
    pub fn encolar(&mut self, carrito:Carrito){ //Funcion para colas enconlar(Poner en la fila al carrito)
        self.linea_texto.push_back(carrito);
    }

    #[allow(dead_code)]
    pub fn desencolar(&mut self){ //Funcion para colas desencolar(Quitar de la fila al carrito)
        self.linea_texto.pop_front();
    }

    #[allow(dead_code)]
    pub fn frente(&mut self) -> Option<&Carrito>{//Lo que valga el frente en la cola en ese momento
        self.linea_texto.front()
    }

    #[allow(dead_code)]
    pub fn fondo(&mut self) -> Option<&Carrito>{//Lo que valga el fondo en la cola en ese momento
        self.linea_texto.back()
    }
    #[allow(dead_code)]
    pub fn carritos_en_espera_actuales(&mut self) -> usize{//Los carritos en espera de la cajera asignados
        self.linea_texto.len()
    }

    #[allow(dead_code)]
    pub fn sumar_dinero(&mut self, carrito:Carrito){//Sumar el dinero de un carro y agregarlo a total recaudado
        println!("La cajera {} esta atendiendo un carro...",self.nom_cajera);

        let monto_total_carrito=sumar_productos(carrito);//Se saca el monto total del carrito sumando los productos
        println!("El monto total del carro es: {}$",(monto_total_carrito*100.00).round()/100.00);
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
        println!("Ahora el dinero recaudado de la cajera {} es: {}$",self.nom_cajera,(self.dinero_recaudado*100.00).round()/100.00);
        println!("");
    }
}

fn main(){
    println!("Leyendo el archivo de entrada {}","QSM.txt");//Se lee el archivo
    let contenido = fs::read_to_string("QSM.txt").expect("Something went wrong reading the file");

    let mut arr_text: Vec<&str> =contenido.lines().collect();//Se divide en lineas
    let mut arr_text_iterable=arr_text.iter_mut();//Se convierte en iterable

    let mut cajeras=Vec::new();//Se crea el vector de cajeras

    println!("");
    println!("-----Procesando carritos en espera predefinidos en una cajero-----");
    println!("");

    for _ in 0..3 {
        let linea_texto = arr_text_iterable.next().unwrap();//Se leen las cajeras
        let vec_contenedor = linea_texto.trim().split(" ").collect::<Vec<&str>>();//Se divide por espacios

        let nom_cajera = vec_contenedor[0].to_string();
        let dinero_recaudado = vec_contenedor[1].parse().unwrap();
        let carritos_en_espera = vec_contenedor[2].parse().unwrap();

        let cajera = Cajera::cajera_nueva(nom_cajera, dinero_recaudado, carritos_en_espera);//Se crea la cajera

        cajeras.push(cajera);
    }

    for cajera in cajeras.iter_mut() {
        for _ in 0..cajera.carritos_en_espera{
            let linea_texto = arr_text_iterable.next().unwrap();

            let mut carro = Carrito::carrito_nuevo();

            for producto in linea_texto.trim().split(",") {
                let vec_contenedor = producto.trim().split(" ").collect::<Vec<&str>>();

                let producto = Producto::producto_nuevo(vec_contenedor[1].parse().unwrap(),vec_contenedor[0].to_string());
                carro.push(producto);
            }
        cajera.encolar(carro);
        }
    }
    for cajera in cajeras.iter_mut() {
        //println!("[{:?}]",cajera);
        while cajera.carritos_en_espera_actuales() > 0 {
            let carro_actual=cajera.frente().unwrap().clone();
            cajera.sumar_dinero(carro_actual);
            //println!("{}",cajera.dinero_recaudado);
        }
    }

    println!("Mostrando todas las cajeras por el momento: ");
    println!("{:?}",cajeras);

    println!("");
    println!("-----Procesando carritos sobrantes en el archivo de texto-----");
    println!("");

    for cajera in cajeras.iter_mut() {
        for _ in 0..cajera.carritos_en_espera{
            let linea_texto = arr_text_iterable.next().unwrap();

            let mut carro = Carrito::carrito_nuevo();

            for producto in linea_texto.trim().split(",") {
                let vec_contenedor = producto.trim().split(" ").collect::<Vec<&str>>();

                let producto_a_insertar = Producto::producto_nuevo(vec_contenedor[1].parse().unwrap(),vec_contenedor[0].to_string());
                carro.push(producto_a_insertar);
            }
        cajera.encolar(carro);
        }
    }

    for cajera in cajeras.iter_mut() {
        //println!("[{:?}]",cajera);
        while cajera.carritos_en_espera_actuales() > 0 {
            let carro_actual=cajera.frente().unwrap().clone();
            cajera.sumar_dinero(carro_actual);
            //println!("{}",cajera.dinero_recaudado);
        }
    }

    println!("Mostrando todas las cajeras: ");
    println!("{:?}",cajeras);

    let mut dinero_recaudado:f32=0.00;

    for x in 0..cajeras.len(){
        dinero_recaudado += cajeras[x].dinero_recaudado;
    }

    println!("");
    println!("El dinero total recaudado por el supermercado: {}$",(dinero_recaudado*100.00).round()/100.00);
    println!("Se despedira al cajero: {} por no haber recaudado dinero",cajeras[2].nom_cajera)

    //141.28

}

