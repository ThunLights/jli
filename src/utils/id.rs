use rand::distributions::{Alphanumeric, DistString};

pub fn generate_id(id_size: usize) -> String {
	let mut rng = rand::thread_rng();
	Alphanumeric.sample_string(&mut rng, id_size)
}
