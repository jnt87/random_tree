use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

fn create_random_tree(base: &PathBuf, rng: &mut ChaChaRng, depth: usize) {
    if depth == 0 {
        return;
    }
    
    let num_dirs = rng.random_range(1..=3);
    let num_files = rng.random_range(1..=3);

    for i in 0..num_dirs {
        let dir_name = format!("dir_{}", i);
        let dir_path = base.join(&dir_name);
        create_dir_all(&dir_path).expect("Failed to create directory");

        create_random_tree(&dir_path, rng, depth - 1);
    }

    for i in 0..num_files {
        let file_name = format!("file_{}.txt", i);
        let file_path = base.join(&file_name);
        let mut file = File::create(&file_path).expect("Failed to create file");

        let content = format!("Random content {}\n", rng.gen::<u32>());
        file.write_all(content.as_bytes()).expect("Failed to write to file");
    }

}

fn main() {
    let base_dir = PathBuf::from("random_tree");
    
    let seed = 7;
    let mut rng = ChaChaRng::seed_from_u64(seed);

    create_random_tree(&base_dir, &mut rng, 3);
    println!("Random directory tree created at {:?}", base_dir);
}
