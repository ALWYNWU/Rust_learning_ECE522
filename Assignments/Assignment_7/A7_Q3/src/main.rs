use rayon::prelude::*;
fn main() {
    let quote = "Some are born great, some achieve greatness, and some have greatness
thrust upon them.".to_string();
    find_words(quote,'s');
}
fn find_words(quote:String, ch:char){

    let words: Vec<_> = quote.split_whitespace().collect();
    let words_with_ch: Vec<_> = words.par_iter().filter(|word| word.contains(ch)).collect();
    println!("The following words contain the letter {:?}: {:?}", ch, words_with_ch);
}

