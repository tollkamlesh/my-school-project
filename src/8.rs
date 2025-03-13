// Function to generate a random number between 1 and 10
fn rand_num() -> i32 {
    let mut rng = rand::thread_rng();
    // Generate a random number between 1 and 10
    return rng.gen_range(1, 11);
}
