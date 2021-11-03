use super::conjunto::Conjunto;

pub fn solucion_greedy(mut conj: Conjunto)      -> Conjunto{
    let mut i = 0;
    let mut dist = 0.0;
   let mut distancias_medias: Vec<f64> = vec![0.0;conj.get_tamanio() as usize];
    let mut nodo = 0;

    for (par, valor) in &conj.get_tablas_distancia() {
        distancias_medias[par.0 as usize] += valor;
        distancias_medias[par.1 as usize] += valor;
    }

    for a in 0..distancias_medias.len(){
        if dist <= distancias_medias[a] {
            nodo = a;
            dist = distancias_medias[a];
        }
    }

    conj.add_nodo(nodo as i32);
    conj.elimina_nodo_restantes(nodo as i32);

    while conj.get_tamanio_sol() > conj.get_solucion().len() as i32 {
        let mut nodo_in: i32 = conj.get_no_seleccionados()[0];
        let mut distancia: f64 = 0.0;

        for no_selec in &conj.get_no_seleccionados() {
            dist = 0.0;
            for selec in &conj.get_solucion() {
                dist += conj.get_distancia(*no_selec, *selec);
            }
            if dist >= distancia {
                nodo_in = *no_selec;
                distancia = dist;
            }
        }

        conj.add_nodo(nodo_in);
        conj.elimina_nodo_restantes(nodo_in as i32);

    }

    return conj;

}

/*

    pub fn solucion_bl(&mut self){

        self.solucion_greedy();
        let mut continuar = true;
        let mut salir = false;
        let mut iter = 0;

        while continuar {

            let div_ini = self.calcula_diversidad();
            iter += 1;

            for nodo_sol in &mut self.solucion {
                for nodo_no_sel in &mut self.no_seleccionados {
                    iter += 1;

                    self.intercambia(*nodo_sol, *nodo_no_sel);

                    let div_nueva = *self.calcula_diversidad();

                    if div_nueva < div_ini {
                        self.intercambia(*nodo_no_sel, *nodo_sol);
                    }
                    else{
                        salir = true;
                        break;
                    }

                }
                if salir || iter <= 10000{
                    break;
                }
                else{
                    continuar = false;
                }
            }

        }

        println!("La solucion es: {:?}", self.solucion);
        println!("La diversidad es: {}", self.calcula_diversidad());
    }
*/
