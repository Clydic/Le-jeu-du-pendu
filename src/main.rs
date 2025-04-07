use unicode_segmentation::UnicodeSegmentation;
use std::{io, ops::Index};
use lib::pick_word;
mod tests;
fn replace_char_at(s: &str, index: usize, new_char: char) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    if index < chars.len() {
        chars[index] = new_char;
    }
    chars.into_iter().collect()
}
fn main() {


       let mut input = String::new();
       let  words = vec!["histoire", "canard", "feu" , "histoire", "anglais", "star", "planète", "insecte"];
       let word_picked  = pick_word(words).trim();
       let word_to_guessed = word_picked.to_string();
       let size_of_number_to_guess  = word_to_guessed.graphemes(true).count();
       let  mut word_hiddden = '*'.to_string().repeat(size_of_number_to_guess);
       let mut number_of_try = 0;
       //let mut flag : bool = true;

       while input.trim() != "q" && word_hiddden != word_to_guessed && number_of_try != 6{
              println!("Mot à découvrir, {}", word_to_guessed);
              println!("Devinez le mot vous avez 6 essaie, {}", word_hiddden);
              input.clear();
              println!("Essayez de deviner le mot, si vous voulez sortir tapez q");
              io::stdin().read_line(&mut input).expect("error: unable to read user input");
       let input_charact:Vec<char> = input.as_str().chars().collect();
           let indices: Vec<usize> = word_to_guessed.match_indices(input_charact[0])
              .map(|(index, _)| index).collect();
       if indices.len() != 0 {

              for indice in indices{
                 word_hiddden = replace_char_at(&word_hiddden, indice, input_charact[0]);
              }
       }else{
             number_of_try += 1; 
              println!("La lettre n'est pas dans le mot il vous reste {}", 6-number_of_try);
       }
              
                     
       }
       if word_hiddden == word_to_guessed{
              println!("Félicitation vous avez gagné");
       }else if input.trim() == "q" {
              println!("Vous avez annulé la partie le mot était {word_to_guessed}");
       }
       else{
              println!("Vous avez perdu");
       }
       


}
