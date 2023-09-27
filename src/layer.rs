use ndarray::Array2;
//use std::sync::{Arc, Mutex};
#[derive(Clone, Debug)]

pub struct Layer<Neuron> {
    pub(crate) neuroni: Vec<Neuron>, // Lista di tutti i neuroni del layer.
    
    pub(crate) interlayer_weights: Array2<f64>, //Matrice dei pesi interlayer con il layer precedente
    
    pub(crate) intralayer_weights: Array2<f64>, //Matrice quadrata dei pesi intra-layer (neuroni appartenti allo stesso layer).
}

impl<Neuron> Layer<Neuron> {
    pub fn new(neurons : Vec<Neuron>, intra_w : Array2<f64>, inter_w : Array2<f64>) -> Self{
        Self{
            neuroni : neurons,             //vettore dei neuroni appartenenti al layer
            interlayer_weights : inter_w,  //matrice dei pesi inter-layer
            intralayer_weights : intra_w,  //matrice dei pesi intra-layer         
        }
    }

    pub fn num_neuroni(&self) -> usize {
        self.neuroni.len()
    }

    pub fn get_neuroni_mut(&mut self, neuroni: usize) -> Option<&mut Neuron> {
        self.neuroni.get_mut(neuroni)
    }

    pub fn get_intralayer_weight(&self, row: usize, coloumn: usize) -> Option<&f64> {
        self.intralayer_weights.get((row, coloumn))
    }

    pub fn get_interlayer_weight(&self, row: usize, coloumn: usize) -> Option<&f64> {
        self.interlayer_weights.get((row, coloumn))
    }

}