struct NodoPila<String>{ //Se crea el nodo de la pila con valores unicamente string
    p_precio: String, //El precio del producto
    p_nombre: String, //El nombre del producto
}
impl NodoPila<String> { //Los metodos del nodo de la pila

    fn nuevo(precio:String,nombre:String)->Self { //Funcion que crea un nuevo NodoPila (producto)
        NodoPila{p_precio:precio, p_nombre:nombre} //A la izquierda va el p_precio por parametros y a la derecha el nombre del producto
    }
}
struct Pila<NodoPila>{ //Se crea la pila del nodo de la pila
    top: usize, //El tope de la pila, lo usamos como un indice, no contiene informacion
    size:usize, //El tamaño maximo de la pila
    array: Vec<NodoPila>, //El arreglo de vectores de la pila
}
impl Pila<NodoPila<String>> { //Los metodos de la pila (tiene su nodo de string)
    fn nueva(size:usize)->Self { //Metodo para crear una pila nueva del tamaño indicado por parametro
        let mut i = 0; //Contador
        let mut arry=Vec::new(); //El arreglo de la pila como vector
        while i<size.clone(){ //Se rellena todo el arreglo de la pila, o se "inicializa"
            arry.push(NodoPila{p_precio:"".to_string(),p_nombre:"".to_string()});
            i=i+1;
        }
        Pila { top: 1, size: size, array:arry}
    }
    fn llena(&mut self)->bool{ //Metodo booleano para saber si la pila esta llena
        if self.top >=self.size{ //Si el tope es mayor o igual que el tamaño de la pila, entonces esta llena
            true
        }else {
            false
        }
    }
    fn vacio(&mut self)->bool { //Metodo booleano para saber si la pila esta vacia
        if self.top==0{ //Si el tope vale 0, entonces la pila esta vacia
            true
        }else {
            false
        }
    }
    fn push(&mut self, valor: NodoPila<String>) { //Metodo para hacer push a la pila con un elemento
        if !self.llena(){ //Si la pila no esta llena...
            self.array[self.top]=valor; //En la posicion del arreglo del tope, se agrega el elemento nuevo
            self.top +=1; //A el tope se le suma uno

        }else {
            print!("Stack overflow");
        }
    }
    fn cima(&mut self)-> &NodoPila<String> { //Metodo para obtener el valor de la cima de la pila
                                            //Devuelve el nodo de la pila que seria siempre el tope
        &self.array[self.top-1]

    }
    fn pop(&mut self) { //Metodo para eliminar el elemento ope de la pila
        if !self.vacio() { //Si no esta vacia la pila...
            self.top -= 1; //A el tope se le resta uno
        }else {
            println!("stack overflow");
        }
    }

}
fn main() {
    let mut m = Pila::nueva(15);
    let p="20".to_string();
    let n="harina".to_string();
    m.push(NodoPila::nuevo(p, n));
    let p="10".to_string();
    let n="azucar".to_string();
    m.push(NodoPila::nuevo(p, n));

    let f = m.cima();
    println!("{}, {}",f.p_precio,f.p_nombre);
    m.pop();

    let f = m.cima();

    println!("{}, {}",f.p_precio,f.p_nombre);
}

