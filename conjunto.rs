use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fs::File;
use std::io::{BufRead, BufReader};
//
//Structura Conjunto
//
pub struct Pair{
    a:                  i32,
    b:                  i32,
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {self.a == other.a && self.b == other.b}
}

impl Eq for Pair{}

impl Hash for Pair{
    fn hash<H: Hasher>(&self, state: &mut H){
        self.a.hash(state);
        self.b.hash(state);
    }
}

pub struct Conjunto{
    tamanio:            i32,
    tamanio_sol:        i32,
    solucion:           Vec<i32>,
    no_seleccionados:   Vec<i32>,
    tablas_distancia:   HashMap<Pair, f64>,
}

impl Conjunto {
    pub fn new(fichero: &String)        -> Conjunto {
        let mut nodos: Vec<i32> = Vec::new();
        let mut tam_conj: i32 = 0;
        let mut tam: i32 = 0;

        let mut tab: HashMap<Pair, f64> = HashMap::new();
        let filename = fichero;
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for(index, line) in reader.lines().enumerate(){
            let line = line.unwrap();

            if index != 0 {
                let split = line.split(" ");
                let vec: Vec<&str> = split.collect();
                let par = Pair{a: vec[0].parse::<i32>().unwrap(), b: vec[1].parse::<i32>().unwrap()};
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

    pub fn get_tamanio(&self)   -> i32 {
        return self.tamanio;
    }

    pub fn get_solucion(self)  -> Vec<i32> {
        return self.solucion;
    }

    pub fn add_nodo(&mut self, nodo: i32){
        self.solucion.push(nodo);
    }

    pub fn elimina_nodo_restantes(&mut self, nodo: i32){
        if let Some(index) = self.no_seleccionados.iter().position(|value| *value == nodo) {
            self.no_seleccionados.swap_remove(index);
        }
    }

    pub fn get_valor(&mut self, val_a: i32, val_b: i32) -> f64{
        let busca = Pair{a: val_a, b: val_b};

        for (par, valor) in &self.tablas_distancia{
            if par.a == busca.a && par.b == busca.b {
                return *valor;
            }
        }

        return f64::NAN;
    }

    pub fn solucion_greedy(&mut self){
        let mut i = 0;
        let mut dist = 0.0;
        let mut distancias_medias: Vec<f64> = Vec::new();
        let mut nodo = 0;

        while distancias_medias.len() <= self.tamanio as usize {
            for ( par, valor ) in &self.tablas_distancia {
                if par.a == i || par.b == i {
                    dist += valor;
                }
            }
            distancias_medias.push( dist/(self.tamanio as f64));
            dist=0.0;
            i+=1;
        }

        for a in 0..distancias_medias.len(){
            if dist <= distancias_medias[a] {
                nodo = a;
                dist = distancias_medias[a];
            }
        }

        self.add_nodo(nodo as i32);
        self.elimina_nodo_restantes(nodo as i32);

        while self.tamanio_sol > self.solucion.len() as i32 {
            let mut nodo_in: i32 = self.no_seleccionados[0];
            let mut a_aux: i32;
            let mut b_aux: i32;
            let mut distancia = 0.0;

            for no_selec in &self.no_seleccionados {
                dist = 0.0;
                for selec in &self.solucion {
                    if *no_selec < *selec {
                        //Esto se hace puesto que la tabla de distancias
                        //ignora lo valores repetidos y tienen prioridad los
                        //valores bajos, es decir aparece la entrada (0 1), pero
                        //no la (1 0).
                        //
                        //Entonces de esta forma nos aseguramos que al buscar
                        //el valor de la clave siempre esté bien ordenado
                        a_aux = *no_selec;
                        b_aux = *selec;
                    }
                    else{
                        a_aux = *selec;
                        b_aux = *no_selec;
                    }
                    let pareja = Pair{a: a_aux, b: b_aux};
                    dist += self.tablas_distancia.get(&pareja).unwrap();
                }
                if dist >= distancia {
                    nodo_in = *no_selec;
                    distancia = dist;
                }
            }

            self.add_nodo(nodo_in);
            self.elimina_nodo_restantes(nodo_in as i32);

        }

        println!("La solucion es: {:?}", self.solucion);



    }

}
