mod char_set;
use char_set::*;
mod phoneme_set;
fn main() {
    let fl = CharSet::new();
    for _ in 0..10{
        println!("{}",fl.get_random_sentence());
    }
}
