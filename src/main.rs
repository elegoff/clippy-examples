mod absurd;
mod almost_swapped;
mod approx_constant;
mod with_coverage;

fn main() {
    println!("Hello, world!");
    absurd::demo();
    almost_swapped::demo();
    approx_constant::demo();
    with_coverage::add(40, 2);
}
