use lif::Neuron;
use network::Network;
use ndarray::Array2;
use ndarray::prelude::*;

pub mod lif;
pub mod layer;
pub mod network;

pub fn main() {

    let mut neurons1 = Vec::new();
    let neurone_11 = Neuron::new(0.6, 0.45, 1.53, 1.2); 
    let neurone_12 = Neuron::new(0.6, 0.4, 1.6, 1.1);  
    let neurone_13 = Neuron::new(0.68, 0.35, 1.52, 1.3); 
    neurons1.push(neurone_11);
    neurons1.push(neurone_12);
    neurons1.push(neurone_13);
    

    let mut neurons2 = Vec::new();
    let neurone21 = Neuron::new(0.6, 0.53, 1.45, 1.2); 
    let neurone22 = Neuron::new(0.7, 0.5, 1.44, 1.1); 

    neurons2.push(neurone21);
    neurons2.push(neurone22);
    

    let mut neurons3 = Vec::new();
    let neurone31 = Neuron::new(0.77, 0.5, 1.6, 1.2);
    let neurone32 = Neuron::new(0.7, 0.47, 1.52, 1.3);
    let neurone33 = Neuron::new(0.8, 0.5, 1.49, 1.1);

    neurons3.push(neurone31);
    neurons3.push(neurone32);
    neurons3.push(neurone33);

    //Creiamo le matrici dei pesi Intra e Inter Layer
    let intra1: Array2::<f64> =  array![[0.0, 0.6, 0.4], [1.0, 0.0, 0.7], [-0.2, 0.62, 0.0]];
    let inter1 =  Array2::from_shape_vec((3, 1), vec![1.0, 1.0, 1.0]).unwrap();

    let intra2: Array2::<f64> = array![[0.0, 0.9],[0.8, 0.0]]; //2x2
    let inter2: Array2::<f64> = array![[0.7, 0.8, 0.89], [0.9, 0.65, 0.88]]; //2x3

    let intra3: Array2::<f64> = array![[0.0, 0.7, 0.4],[1.0, 0.0, 0.8],[0.9, -0.1, 0.0]]; //3x3
    let inter3: Array2::<f64> = array![[0.8, 0.7], [0.8, 0.7], [0.86, 0.75]]; //3x2
    
    
    //Creiamo la rete aggiungendo i vari Layer
    let mut network = Network::new();
    network.add_layer(neurons1, inter1, intra1);
    network.add_layer(neurons2, inter2, intra2);
    network.add_layer(neurons3, inter3, intra3);

    //vettore degli imput al primo layer
    let spike_m: Array2::<f64> = array![[1.0, 0.0, 1.0],[0.0, 1.0, 1.0],[1.0, 1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 1.0], [1.0, 1.0, 0.0]];

    //vettore dei tempi
    let tempi = vec![1.5, 2.0, 3.0, 5.0, 6.0, 7.5, 9.0];

    let mut count = 0;
    for ts in tempi{
        print!("\nTempo = {}\n", ts);
        network.aggiorna_neuroni(ts, spike_m.row(count).to_vec()); //Propagazione dello spike, e aggiornamento dei valori, all'interno della rete
        count+=1;
    }

}