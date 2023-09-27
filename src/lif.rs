#[derive(Clone, Debug, Copy)]
pub struct Neuron{

    pub v_rest: f64,

    pub v_reset: f64,

    pub v_th: f64,  

    pub tau: f64,

    pub v_mem : f64,  //inizializzato a 0.0

    pub t_s_prec : f64, //inizializzato a 0.0
    
}

impl Neuron {
    pub fn new(vrest: f64, vreset: f64, vth: f64, tauu: f64) -> Neuron {
        Neuron {
            v_rest : vrest,
            v_reset : vreset,
            v_th : vth, 
            tau : tauu,
            v_mem : 0.0,
            t_s_prec : 0.0
        }
    }

    #[inline]
    pub fn potential_evolution (&mut self, weighted_sum: f64, t_s: f64) -> f64 { //funzione di calcolo dello spike del singolo neurone
        
        self.v_mem = self.v_rest + (self.v_mem - self.v_rest) * (self.t_s_prec - t_s / self.tau).exp() + weighted_sum;  //decadenza di v_mem + aggiunta weighted_sum
        //aggiornamento del valore di v_mem e confronto con il valore di threshold
        self.t_s_prec = t_s; 

        if self.v_mem > self.v_th {  //controllo superamento treshold
            self.v_mem = self.v_reset;
            1. //spike
        } else {
            0. //no spike
        }
    }

    pub fn aggiornamento_internal (mut self, input: f64){
        self.v_mem = self.v_mem + input;
    }
}