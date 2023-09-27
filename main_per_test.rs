//Giorgio ->
let mut neurons1 = Vec::new();
    let neurone_11 = Neuron::new(0.6, 0.45, 1.03, 1.2); 
    let neurone_12 = Neuron::new(0.6, 0.4, 1.6, 1.1);
    neurons1.push(neurone_11);
    neurons1.push(neurone_12);



    let mut neurons2 = Vec::new();
    let neurone21 = Neuron::new(0.6, 0.53, 1.25, 1.2); 
    let neurone22 = Neuron::new(0.7, 0.5, 1.14, 1.1); 
    neurons2.push(neurone21);
    neurons2.push(neurone22);

     
    let mut neurons3 = Vec::new();
    let neurone31 = Neuron::new(0.77, 0.5, 1.2, 1.2);
    neurons3.push(neurone31);


    let intra1: Array2::<f64> =  array![[0.0, 0.7],[0.6, 0.0]];
    let inter1 =  Array2::from_shape_vec((2, 1), vec![1.0, 1.0]).unwrap();

    let intra2: Array2::<f64> = array![[0.0, 0.9],[0.8, 0.0]]; //2x2
    let inter2: Array2::<f64> = array![[0.7, 0.8], [0.9, 0.75]]; //2x2

    let intra3: Array2::<f64> = array![[0.0]]; //1x1
    let inter3: Array2::<f64> = array![[0.8, 0.9]]; //1x2

    let mut network = Network::new();
    network.add_layer(neurons1, inter1, intra1);
    network.add_layer(neurons2, inter2, intra2);
    network.add_layer(neurons3, inter3, intra3);

    let spike_m: Array2::<f64> = array![[1.0, 0.0]];

    let count = 0;
    let ts = 1.0; 
        print!("\nTempo = {}\n", ts);
        network.aggiorna_neuroni(ts, spike_m.row(count).to_vec());

//*-----------------------------------------------------------------------------*

//Davide ->

 let mut neurons1 = Vec::new();
    let neurone_11 = Neuron::new(0.7, 0.65, 1.23, 1.2); 
    let neurone_12 = Neuron::new(0.82, 0.4, 1.44, 1.1);
    neurons1.push(neurone_11);
    neurons1.push(neurone_12);



    let mut neurons2 = Vec::new();
    let neurone21 = Neuron::new(0.55, 0.53, 1.05, 1.22); 
    neurons2.push(neurone21);

     
    let mut neurons3 = Vec::new();
    let neurone31 = Neuron::new(0.77, 0.55, 1.2, 1.2);
    let neurone32 = Neuron::new(0.4, 0.87, 1.32, 1.3);
    neurons3.push(neurone31);
    neurons3.push(neurone32);


    let intra1: Array2::<f64> =  array![[0.0, 0.5],[0.5, 0.0]];
    let inter1 =  Array2::from_shape_vec((2, 1), vec![1.0, 1.0]).unwrap();

    let intra2: Array2::<f64> = array![[0.0]]; //1x1
    let inter2: Array2::<f64> = array![[0.8, 0.99]]; //1x2

    let intra3: Array2::<f64> = array![[0.0, 0.88],[0.3, 0.0]]; //1x1
    let inter3: Array2::<f64> = array![[0.8], [-0.1]]; //2x1

    let mut network = Network::new();
    network.add_layer(neurons1, inter1, intra1);
    network.add_layer(neurons2, inter2, intra2);
    network.add_layer(neurons3, inter3, intra3);

    let spike_m: Array2::<f64> = array![[1.0, 0.0]];

    let count = 0;
    let ts = 1.0; 
        print!("\nTempo = {}\n", ts);
        network.aggiorna_neuroni(ts, spike_m.row(count).to_vec()); //Propagazione dello spike, e aggiornamento dei valori, all'interno della rete
    
//*-------------------------------------------------------------------------------*
//Marco -> 

    let mut neurons1 = Vec::new();
    let neurone_11 = Neuron::new(0.6, 0.69, 1.0, 1.2); 
    neurons1.push(neurone_11);


    let mut neurons2 = Vec::new();
    let neurone21 = Neuron::new(0.55, 0.53, 1.35, 1.22); 
    let neurone22 = Neuron::new(0.7, 0.5, 1.14, 1.17);
    neurons2.push(neurone21);
    neurons2.push(neurone22);

     
    let mut neurons3 = Vec::new();
    let neurone31 = Neuron::new(0.59, 0.55, 1.29, 1.2);
    let neurone32 = Neuron::new(0.66, 0.57, 1.12, 1.3);
    neurons3.push(neurone31);
    neurons3.push(neurone32);


    let intra1: Array2::<f64> =  array![[0.0]];
    let inter1 =  Array2::from_shape_vec((1, 1), vec![1.0]).unwrap();

    let intra2: Array2::<f64> = array![[0.0, 0.83],[0.66, 0.0]]; //2x2
    let inter2: Array2::<f64> = array![[0.9], [0.74]]; //2x1

    let intra3: Array2::<f64> = array![[0.0, 0.88],[0.3, 0.0]]; //1x1
    let inter3: Array2::<f64> = array![[0.8, -0.4], [-0.1, 0.77]]; //2x2

    let mut network = Network::new();
    network.add_layer(neurons1, inter1, intra1);
    network.add_layer(neurons2, inter2, intra2);
    network.add_layer(neurons3, inter3, intra3);

    let spike_m: Array2::<f64> = array![[1.0]];

    let count = 0;
    let ts = 1.0; 
        print!("\nTempo = {}\n", ts);
        network.aggiorna_neuroni(ts, spike_m.row(count).to_vec());
