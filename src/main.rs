use std::io;
use std::io::Write;
use std::fs;
use rand::Rng;

// TODO: trim words to the first 5 characters
// TODO: do a better check if letter is in multiple positions

fn main()
{
  println!("RWORDLE!");


  let mut buffer = String::new();

  let mut n_try = 1;

  /* Read words file and push them to vector */
  let mut valid_words = Vec::new();
  let txt = fs::read_to_string("vendor/valid_words.txt").unwrap();

  for w in txt.lines()
  {
    valid_words.push(w.trim_end().to_uppercase());
  }

  /* Get a random position within vector to be the right word */
  let num = rand::thread_rng().gen_range(0..valid_words.len());
  let word = valid_words.get(num).unwrap().clone();

  /*
   * A simple loop that checks if typed word is within vector with valid words.
   * If true, it goes by each letter and checks its position.
   */

  loop
  {
    print!("Enter guess number {}: ", n_try);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();

    let s_word = buffer.trim_end().to_uppercase();

    if !valid_words.contains(&s_word)
    {
      println!("{} is not a valid word. Retry.", s_word);
      buffer.clear(); // clear buffer because read_line appends to it
      continue;
    }

    if s_word == word
    {
      println!("CONGRATULATIONS!!!!");
      println!("{}/5", n_try);
      return;
    }

    for idx in 0..s_word.chars().count()
    {
      let l = s_word.chars().nth(idx).unwrap();
      /*
       * Bold if correct position
       * Italic if wrong position
       * Strikethrough if word does not contain it
       *
       *https://askubuntu.com/questions/528928/how-to-do-underline-bold-italic-strikethrough-color-background-and-size-i
       * https://github.com/rust-lang/rust/issues/30491
       */
      if word.contains(l)
      {
        if s_word.find(l) == word.find(l) && word.find(l) == Some(idx)
        {
          print!("\x1b[3m{}\x1b[0m", l);
        }
        else
        {
          print!("\x1b[4m{}\x1b[0m", l);
        }
      }
      else
      {
        print!("\x1b[9m{}\x1b[0m", l);
      }
    }
    // Since we've used print! macro we need to flush stdout
    io::stdout().flush().unwrap();
    println!();
    buffer.clear(); // clear buffer because read_line appends to it

    n_try += 1;
    if n_try == 6
    {
      println!("Better luck next time! Word was: {}", word);
      break;
    }
  }

}
