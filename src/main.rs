use rand::SeedableRng;
use rand_chacha::ChaChaRng;
use std::path::PathBuf;
use random_tree::{create_random_tree};

fn main() {
    let base_dir = PathBuf::from("random_tree");
    
    let seed = 7;
    let mut rng = ChaChaRng::seed_from_u64(seed);

    create_random_tree(&base_dir, &mut rng, 3);
    println!("Random directory tree created at {:?}", base_dir);
}
