extern crate textflow;

use textflow::utils::invert_2d_vec;

fn main() {
    let mut v = vec![vec![1, 9], vec![8], vec![2, 6], vec![3, 5]];

    print_vec_no_depth(&v);

    invert_2d_vec(&mut v);

    print_vec_no_depth(&v);

    invert_2d_vec(&mut v);

    print_vec_no_depth(&v);
}

fn print_vec_no_depth<T: std::fmt::Debug>(v: &Vec<T>) {
    println!("[");
    for item in v.iter() {
        println!("  {:?},", item);
    }
    println!("]");
}
