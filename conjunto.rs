use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

//
//Structura Conjunto
//

pub struct Conjunto{
    tamanio:            i32,
    tamanio_sol:        i32,
    solucion:           Vec<i32>,
    no_seleccionados:   Vec<i32>,
    tablas_distancia:   HashMap<(i32, i32), f64>,
}

impl Conjunto {
    //
    //Constructor:
    //Mediante un fichero inicializa nuestro conjunto
    //y tablas de distancia
    //
    pub fn new(fichero: &String)        -> Conjunto {
        let mut nodos: Vec<i32> = Vec::new();
        let mut tam_conj: i32 = 0;
        let mut tam: i32 = 0;

        let mut tab: HashMap<(i32, i32), f64> = HashMap::new();
        let filename = fichero;
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for(index, line) in reader.lines().enumerate(){
            let line = line.unwrap();

            if index != 0 {
                let split = line.split(" ");
                let vec: Vec<&str> = split.collect();
                let par = (vec[0].parse::<i32>().unwrap(), vec[1].parse::<i32>().unwrap());
                tab.insert(par, vec[2].parse::<f64>().unwrap());
            }
            else{
                let split = line.split(" ");
                let vec: Vec<&str> = split.collect();
                tam = vec[0].parse::<i32>().unwrap();
                tam_conj = vec[1].parse::<i32>().unwrap();
            }
        }

        for i in 0..tam {
            nodos.push(i as i32);
        }

        return Conjunto{tamanio: tam, tamanio_sol: tam_conj, solucion: Vec::new(), no_seleccionados: nodos, tablas_distancia: tab};
    }

    //
    //Getters
    //Métodos básicos para recuperar información
    //

    pub fn get_tablas_distancia(&self)                  -> HashMap<(i32, i32), f64> {
        return self.tablas_distancia.clone();
    }

    pub fn get_tamanio(&self)                            -> i32 {
        return self.tamanio;
    }

    pub fn get_tamanio_sol(&self)                        -> i32 {
        return self.tamanio_sol;
    }

    pub fn get_solucion(&self)                           -> Vec<i32> {
        return self.solucion.clone();
    }

    pub fn get_no_seleccionados(&self)                   -> Vec<i32> {
        return self.no_seleccionados.clone();
    }

    pub fn get_valor(&mut self, val_a: i32, val_b: i32) -> f64{
        let busca = (val_a, val_b);

        for (par, valor) in &self.tablas_distancia{
            if par.0 == busca.0 && par.1 == busca.1 {
                return *valor;
            }
        }

        return f64::NAN;
    }

    pub fn get_distancia(&self, nodo1: i32, nodo2: i32 ) -> f64 {
        let a_aux: i32;
        let b_aux: i32;

        if nodo1 < nodo2 {
            //Esto se hace puesto que la tabla de distancias
            //ignora lo valores repetidos y tienen prioridad los
            //valores bajos, es decir aparece la entrada (0 1), pero
            //no la (1 0).
            //
            //Entonces de esta forma nos aseguramos que al buscar
            //el valor de la clave siempre esté bien ordenado
            a_aux = nodo1;
            b_aux = nodo2;
        }
        else{
            a_aux = nodo2;
            b_aux = nodo1;
        }

        let pareja = (a_aux, b_aux);
        return *self.tablas_distancia.get(&pareja).unwrap();
    }

    //
    //Setters
    //Métodos básicos para modificar información
    //

    pub fn add_nodo_restantes(&mut self, nodo: i32){
        self.no_seleccionados.push(nodo);
    }

    pub fn add_nodo(&mut self, nodo: i32){
        self.solucion.push(nodo);
    }

    pub fn elimina_nodo_restantes(&mut self, nodo: i32){
        if let Some(index) = self.no_seleccionados.iter().position(|value| *value == nodo) {
            self.no_seleccionados.swap_remove(index);
        }
    }

    pub fn elimina_nodo(&mut self, nodo: i32){
        if let Some(index) = self.solucion.iter().position(|value| *value == nodo) {
            self.solucion.swap_remove(index);
        }
    }

    //
    //Método específico del problema para calcular la diversidad
    //de nuestro conjunto.
    //

    pub fn calcula_diversidad(&self) -> f64 {
        let mut distancia = 0.0;
        for nodo1 in 0..self.tamanio_sol {
            for nodo2 in nodo1+1..self.tamanio_sol {
                let a = self.solucion[nodo1 as usize];
                let b = self.solucion[nodo2 as usize];
                distancia += self.get_distancia(a, b);
            }
        }
        return distancia;
    }

    //
    //Método específico pensado para la búsqueda local
    //intercambia un nodo no seleccionado por otro nodo
    //presente en el conjunto solución
    //

    pub fn intercambia(&mut self, nodo_sol: i32, nodo_no_sel: i32){
        self.elimina_nodo_restantes(nodo_no_sel);
        self.add_nodo(nodo_no_sel);

        self.elimina_nodo(nodo_sol);
        self.add_nodo_restantes(nodo_sol);
    }

}
