
use num_cpus;



/// Returns a usize representing the length that the sequences should have after padding/trimming
/// or 0 for no padding/trimming
///
/// if pad_length = -1 searches for the shortest sequence
/// if pad_length = -2 searches for the longest sequence
pub fn get_length(sequences: &Vec<&str>, pad_length: i128) -> usize {


    let mut length= sequences[0].len();


    if pad_length < -2 {

        panic!("Invalid padding length. Here are the available options:
         -2 to pad to longest sequence; 
         -1 to pad to the shortest sequence;
          0 for no padding;
          any positive number to choose the maximum length you want your sequences to be.");
        
    }

    else if pad_length == 0 {

        length= 0;
    }
    

    //padd to shortest
    else if pad_length == -1 {

        for seq in sequences.iter() {

            if seq.len() < length {
            
            length = seq.len();

            }
        }
    }

    //pad to longest
    else if pad_length == -2 {

        for seq in sequences.iter() {

            if seq.len() > length {
            
                length = seq.len();

            }
        }   
    }


    else {

        length= pad_length as usize;
    }

    return length
}


/// Returns the number of threads to use.
/// 
/// if n_jobs = 0; number of threads = number of cpus
pub fn check_nb_cpus(n_jobs: i16) -> usize {

    let nb_cpus;

    if n_jobs == 0 {

        nb_cpus= num_cpus::get_physical();
    }

    else if n_jobs < 0 {

       panic!("Cannot have a negative number of cpu. Use 0 to use every cpus or input the number of desired cpus")

    }

    else {

        nb_cpus= n_jobs as usize;
    }

    nb_cpus
}