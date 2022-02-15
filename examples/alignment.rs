extern crate textflow;

use textflow::align;
use textflow::Alignment::*;

fn main() {
    let text = "Academy City. This was a place that was developed from the undeveloped land of western Tokyo. It was one-third the size of Tokyo, and there was a tall wall surrounding it. Eighty percent of its population of 2.3 million consisted of students. Besides being the pinnacle city of academia and the ubiquitous all-around study of science and technology, there was another side to it - the esper development institution that was achieved through artificial means and scientific processes.";

    println!("LEFT-ALIGNED ===================================\n");
    println!("{}", align(text, LEFT, 48));

    println!("\n\nRIGHT-ALIGNED ==================================\n");
    println!("{}", align(text, RIGHT, 48));

    println!("\n\nCENTERED =======================================\n");
    println!("{}", align(text, CENTER, 48));

    println!("\n\nJUSTIFIED ======================================\n");
    println!("{}", align(text, JUSTIFY, 48));
}
