
//  cargo run --release

use std::process::exit;

fn main() {

    
    #[allow(unused_mut)]
    let mut term_w;
    #[allow(unused_mut)]
    let mut term_h;

    if let Some((w, h)) = term_size::dimensions() {
        term_w = w;
        term_h = h;
    } else {
        println!("Unable to get term size.");
        exit(-1);
    }

    //println!("Width: {}\nHeight: {}", term_w, term_h);

    //print!("\x1B[2J\x1B[1;1H");

    let mut n = 0;
    while n < term_h {
        let mut m = 0;
        while m < term_w {
            print!("#");
            m = m + 1;
        }
        n = n + 1;
    }
    return();
}
