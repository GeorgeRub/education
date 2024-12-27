extern crate tensorflow;

use tensorflow::Tensor;

fn main() {
    let mut x = Tensor::new(&[2]);

    x[0] = 3.0f32;
    x[1] = 2.0f32;

    println!("{:?}", x);
}