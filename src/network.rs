use crate::{layer::Layer, lif::Neuron};
use ndarray::Array2;
use std::sync::{Arc, Mutex};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Network<Layer> { 
    pub(crate) layers: Vec<Layer>, //Vettore con tutti i layer della rete
    pub(crate) num_layers : usize, //Numero di layers
}

impl Network<Layer<Neuron>>{
    pub fn new() -> Self{
        Network { layers: Vec::new(), num_layers: 0 }
    }

    pub fn add_layer(&mut self, neurons: Vec<Neuron>, interlayer_weights: Array2<f64>, intralayer_weights: Array2<f64>) { 
        let l = Layer::new(neurons.clone(), intralayer_weights.clone(), interlayer_weights.clone());
        self.num_layers += 1;
        self.layers.push(l);
    }

    pub fn aggiorna_neuroni (&mut self, ts : f64, spike : Vec<f64>) -> Vec<f64>{
        let mut s = Vec::new();
        
        for i in 0..self.num_layers{ //for sui layer
            if i==0 {                

                let temp = Arc::new(Mutex::new(Vec::<f64>::new())); //garantisco la mutua esclusione sul vettore degli spike
                let mut vt = Vec::new(); //vettore dei thread
                for m in 0..self.layers.get(i).unwrap().num_neuroni(){ //for sui neuroni del layer
                    let temp = temp.clone();
                    let mut primo_layer = self.layers.get(i).unwrap().clone();
                    let spike_temp = spike.clone();
                    
                    vt.push(std::thread::spawn(move || {                                                            //qui invece della weighted sum ho il valore di input
                        temp.lock().unwrap().push(primo_layer.get_neuroni_mut(m).unwrap().clone().potential_evolution(*spike_temp.get(m).unwrap(),ts));
                    })); //crea un thread per ogni Neuron del layer, aggiorna il potenziale di membrana e ritorna lo spike del Neuron 
                }
                for v in vt{ //aspettiamo le terminazioni dei thread
                    v.join().unwrap();
                }

                s = temp.lock().unwrap().to_vec(); //output
                drop(temp);

                for ciclo in 0..s.len(){ //stampa del risultato per ogni tempo
                    print!("{}", s.get(ciclo).unwrap());
                    print!("  ");
                }
                print!("\n");

                let internal_temp = Arc::new(Mutex::new(vec![0.0; self.layers.get(i).unwrap().num_neuroni()]));  //vettore dei contributi degli spike provenienti dagli altri neuroni dello stesso layer
                let mut vet_internal_spike =  Vec::new(); //vettore dei thread

                for n in 0..self.layers.get(i).unwrap().num_neuroni(){ //for sui neuroni del layer
                    let internal_temp = internal_temp.clone();
                    let layer_temp = self.layers.get(i).unwrap().clone();
                    let temporaneo = s.clone();
                    vet_internal_spike.push(std::thread::spawn(move || {
                        for m in 0..layer_temp.num_neuroni(){                    
                            internal_temp.lock().unwrap()[m] = temporaneo.get(n).unwrap() * layer_temp.get_intralayer_weight(n, m).unwrap();
                        }  //creo un thread per ogni Neuron che aggiunge i nuovi spike degli altri nueroni del proprio layer, pesati secondo la matrice di intralayer_weights
                    }));
                }
                for v in vet_internal_spike{ //aspettiamo le terminazioni dei thread
                    v.join().unwrap();
                }

                let lun;
                {
                    lun = internal_temp.lock().unwrap().to_vec().len(); 
                }
                for j in 0..lun{ //for sui neuroni del layer
                    self.layers.get(i).unwrap().neuroni.get(j).unwrap().aggiornamento_internal(*internal_temp.lock().unwrap().to_vec().get(j).unwrap()); 
                }   //sommo alla v_mem il contibuto degli spike provenienti dagli altri neuroni dello stesso layer
                drop(internal_temp);
            }    
            else{      
                let temp = Arc::new(Mutex::new(Vec::<f64>::new())); //garantisco la mutua esclusione sul vettore degli spike 
                let mut vt = Vec::new(); //vettore dei thread
                for n in 0..self.layers.get(i).unwrap().num_neuroni(){ //ciclo sui neuroni del layer
                    let mut layer_temp = self.layers.get(i).unwrap().clone();
                    let layer_temp_p = self.layers.get(i-1).unwrap().clone(); //qui abbiamo bisogno anche del layer precedente
                    let temp = temp.clone();
                    let temporaneo = s.clone();  //spike del layer precedente

                    vt.push(std::thread::spawn(move||{
                        let mut tot = 0.0; //weighted sum
                        for m in 0..layer_temp_p.num_neuroni(){ //ciclo sui neuroni del layer PRECEDENTE
                            tot = tot + temporaneo.get(m).unwrap() * layer_temp.get_interlayer_weight(n,m).unwrap(); //aggiorno il valore dello spike con i pesi del layer precedente
                        }
                        temp.lock().unwrap().push(layer_temp.get_neuroni_mut(n).unwrap().clone().potential_evolution(tot, ts)); 
                    })); //creo un thread per ogni neurone che, calcola la weighted_sum del neurone, aggiorna il potenziale di membrana e ritorna lo spike del Neuron
                }
                for v in vt{
                    v.join().unwrap();
                }
                

                //stesso procedimento
                s = temp.lock().unwrap().to_vec();
                drop(temp);
                
                let internal_temp = Arc::new(Mutex::new(vec![0.0; self.layers.get(i).unwrap().num_neuroni()]));
                let mut vet_internal_spike =  Vec::new();
                for n in 0..self.layers.get(i).unwrap().num_neuroni(){
                    let internal_temp = internal_temp.clone();
                    let layer_temp = self.layers.get(i).unwrap().clone();
                    let temporaneo = s.clone();
                    vet_internal_spike.push(std::thread::spawn(move || {
                        for m in 0..layer_temp.num_neuroni(){                  
                            internal_temp.lock().unwrap()[m] = temporaneo.get(n).unwrap() * layer_temp.get_intralayer_weight(n, m).unwrap();
                        }
                    }));
                    
                }
                for v in vet_internal_spike{
                    v.join().unwrap();
                }
                
                let lun;
                {
                    lun = internal_temp.lock().unwrap().to_vec().len();
                }
                for j in 0..lun{
                    self.layers.get(i).unwrap().neuroni.get(j).unwrap().aggiornamento_internal(*internal_temp.lock().unwrap().to_vec().get(j).unwrap());
                }
                drop(internal_temp);

                for indice in 0..s.len(){
                    print!("{}", s.get(indice).unwrap());
                    print!("  ");
                }
                print!("\n");
            }  
        }
        return s
    }       
}       