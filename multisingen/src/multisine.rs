use rand::Rng;

pub fn create_discrete_multisine(fmin: u32, fmax: u32) -> Vec<f64> {

    //Declaring constats
    const PI: f64 = std::f64::consts::PI;

    let sample_num: u32 = fmax * 2;
    let norm_factor: f64 = 1.0 / (sample_num as f64);

    //Vectors for frequency, phase and basis functions
    let mut freq_vals: Vec<u32> = vec![0; fmax as usize];
    let mut time_steps: Vec<f64> = vec![0.0; sample_num as usize];
    let mut phase_vals:  Vec<f64> = vec![0.0; fmax as usize];
    let mut multisin_basis: Vec<Vec<f64>> = vec![vec![0.0; sample_num as usize]; sample_num as usize];
    let mut multisin_samples: Vec<f64> = vec![0.0; sample_num as usize];

    //Variable for random number generator
    let mut rng = rand::thread_rng();

    //Creating freqency values
    for i in fmin..=fmax {
        freq_vals[i as usize] = i;
    }

    //Creating time steps
    let timestep: f64 = 1.0 / (sample_num as f64);
    for i in 0..sample_num {
        time_steps[i as usize] = (i as f64) * timestep;
    }

    //Creating random phases between -180°and 180°
    for i in 0..fmax {
        phase_vals[i as usize] = rng.gen_range(-PI..PI);
    }

    //FIXME: Use iterators
    //Creating basis functions
    for i in 0..fmax {
        for j in 0..sample_num {
            multisin_basis[i as usize][j as usize] = ((((2.0*PI*freq_vals[i as usize] as f64)*time_steps[j as usize]) as f64) + phase_vals[i as usize]).sin();
        }
    }

    //Create the multisine vector
    for time_sample in 0..sample_num {
        for freq in 0..fmax {
            multisin_samples[time_sample as usize] += multisin_basis[freq as usize][time_sample as usize];
        }
    }
    return multisin_samples;

}