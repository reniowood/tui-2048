pub mod block {
    use rand::Rng;

    pub fn random() -> u32 {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1, 3) * 2;

        value
    }
}
