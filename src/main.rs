mod absurd;
mod almost_swapped;
mod approx_constant;
mod with_coverage;
mod too_args;


fn main()  {
    println!("Let s raise some issues !");
    absurd::demo();
    almost_swapped::demo();
    approx_constant::demo();
    with_coverage::add(40, 2);
    let _result = too_args::demo(1, 2, 3, 4, 5, 6, 7,8, 9);
}
