#[cfg(test)]
mod tests{
    use crate::neurone::{self, FullNet};

    #[test]
    fn activation_test(){
        let i1 = neurone::Neuron::new(1);
        let i2 = neurone::Neuron::new(1);
        let i3 = neurone::Neuron::new(1);
        let mut o0 = neurone::Neuron::new(1);
        assert_eq!(o0.activtion([i1,i2,i3].to_vec(), 0).activation,0.8909032)
    }
    #[test]
    fn run_frd(){
        let mut net = FullNet::new(1, 2, 1);

        let out = net.run_data([1].to_vec());
        assert_eq!(out,[0.71818227,0.71818227].to_vec())
    }
}