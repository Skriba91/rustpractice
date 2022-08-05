
pub fn create_discrete_multisine(fmax: u32) {

    //Declaring constats
    const PI: f64 = std::f64::consts::PI;

    let sample_num: u32 = fmax * 2;
    let norm_factor: f64 = 1.0 / (sample_num as f64);

    //Vectors for frequency, phase and basis functions
    let mut freq_vals: Vec<u32> = vec![0; fmax as usize];
    let mut phase_vals:  Vec<f64> = vec![0.0; fmax as usize];
    let mut multisin_basis: Vec<Vec<f64>> = vec![vec![0.0; sample_num as usize]; sample_num as usize];
    let mut multisin_samples: Vec<f64> = vec![0.0; sample_num as usize];

    //Creating freqency values
    for i in 0..=fmax {
        
        freq_vals[i as usize] = (i as f64)/norm_factor;
    }

}