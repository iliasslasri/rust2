mod gamma;
mod image;

fn main() {
    let mut c: image::Color = Default::default();

    println!(" Defaults colors : {:?}", c.r);
    c.r = 3;
    c = c.gamma_correct();

    println!("  : {:?}", c.r);
}
