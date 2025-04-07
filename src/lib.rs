// Call librairie
use rand::Rng;

// Choose randomly
pub fn pick_word(list_of_string: Vec<&str>)->&str{
  let mut rng = rand::thread_rng();
  let length = list_of_string.len();
  let index = rng.gen_range(1..=length-1);
  list_of_string[index]
}

 // src/tests.rs
 #[cfg(test)]
 mod tests {
     use super::pick_word;

     #[test]
     fn test_pick_word() {
         let words = vec!["histoire", "canard", "feu", "anglais"];
         let picked_word = pick_word(words.clone());

         // Check if the picked word is in the original list
         assert!(words.contains(&picked_word));
     }
 }