use rand::{Rng, distributions::{StandardNormal, Uniform}};
fn main() {
    let mut rng = rand::thread_rng();
    let normal = StandardNormal.new().unwrap();
    let uniform = Uniform::from(1..=6).unwrap();
    for _ in 0..5 {
        println!("{:?}", normal.sample(&mut rng));
        println!("{}", uniform.sample(&mut rng));
    }
}
