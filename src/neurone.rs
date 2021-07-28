#[derive(Clone,Debug)]
pub struct Neuron {
    bias: f32,
    pub weights: Vec<f32>,
    pub activation: f32,
}
#[derive(Clone)]
pub struct FullNet {
    pub layers: Vec<Vec<Neuron>>, //fix     vec![Vec<Neuron>;4]  not sure how to get this to work, would be nice to but not necareccy.
}

/*
    /¯¯¯m
   i----m
    \___m

the weights between the i am m layer are owned by the i neuron. output neurones weights will be null as they will have no "connection"
*/

use rand::{distributions::Uniform, thread_rng, Rng};
use std::f32::consts::E;
#[allow(dead_code)]
impl Neuron {
    pub fn new(weights: usize) -> Neuron {
        let mut random = thread_rng(); //i assume that this is expensive so i did not put it on the same line as the weight gerneration
        let range = Uniform::new_inclusive(0.7,0.7);
        Neuron {
            bias: 0.0,
            weights:(&mut random).sample_iter(range).take(weights).collect(), //wanted to avoid using map not sure if this is the idiomatic way of doing this
            activation: 1.0,
        }
    }
    pub fn activtion(&mut self,layer: Vec<Neuron>, self_pos: usize) -> Self {       // the layer should be the previous layer ( sould have made this explicit) Fool
        self.activation = 0.0;

        for node in layer {
            
            self.activation += node.activation * node.weights[self_pos];
        }
        self.activation = 1.0 / (1.0 + E.powf(-self.activation)); // this is a sigmoid function, it is cosely bound between 2 asymtotes on y=1 and y=0 meaning it is good for "squishification"-3Blue!Brown
        self.to_owned()
    }
}

#[allow(dead_code)]
impl FullNet {
    pub fn new(inputs: usize, outputs: usize,mut hidden_layers:usize) -> Self {
        let mut net = Vec::new();

        //this automatically constructs a net of in, out and x hidden layers 2 neurons deep
        hidden_layers = if hidden_layers ==0{1}else{hidden_layers};
        net.push(FullNet::pop_vec(inputs,2));
        for _ in 0..hidden_layers-1{
            net.push(FullNet::pop_vec(2, 2));
        }
        net.push(FullNet::pop_vec(2, outputs));
        net.push(FullNet::pop_vec(outputs, 1));

        FullNet { layers: net }

    }
    pub fn run_data(&mut self,inputs:Vec<i32>)->Vec<f32>{   //returns the output as a vec of floats(the activations of the opuput layer)
        for i in 0..inputs.len(){
            self.layers[0][i].activation = inputs[i]as f32;                //load the given inputs into the input layer of the net
        }


        
        for layer in 1..self.layers.len(){                             //for every layer
            for node in 0..self.layers[layer].len(){                       //do the folowing for each node in that layer
                let  a = self.layers[layer-1].clone();         
                self.layers[layer][node].activtion(a, node);
            }

        }
        self.layers.last().unwrap().iter().map(|node|node.activation).collect()
    }

     fn _total_cost(outputs:Vec<f32>,expected:Vec<f32>)->f32{
        let mut cost = 0.0;
        for i in 0..outputs.len(){
            cost += (outputs[i]-expected[i]).powf(2.0)
        }
        cost
    }

    /*
    input layer -- hidden later(s) -- output layer

    if the input is the layer you are creating then len is the number
    of inputs and the "next_layer" is the length the the presceding hidden layer
    */
    #[allow(dead_code)]
    fn pop_vec(len: usize, next_len: usize) -> Vec<Neuron> {
        let mut sub_net = Vec::new();
        for _ in 0..len {
            sub_net.push(Neuron::new(next_len));
        }
        sub_net
    }
}
//inputs:usize,outputs:usize,hiddenLayersLen:usize


