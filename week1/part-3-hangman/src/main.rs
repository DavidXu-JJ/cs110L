// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn output(already_guessed:& Vec<char>,history:& String, chance: &mut i32){
    print!("The word so far is "); 
    for i in 0..already_guessed.len(){
        print!("{}",already_guessed[i]);
    }
    print!("\n");
    println!("You have guessed the following letters:{}",history);
    println!("You have {} guesses left",chance);
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);
    let mut already_guessed:Vec<char> = Vec::new(); 
    for i in secret_word_chars.iter(){
        already_guessed.push('-');
    }
    let mut left = secret_word_chars.len(); 
    let mut history:String = String::new(); 
    let mut chance:i32 = 5;
    while left != 0{
        if chance==0{
            break;
        }
        output(&already_guessed, &history,&mut chance); 
        print!("Please guess a letter: ");
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        let mut _correct:bool =false;
        let guess_char : Vec<char> = guess.chars().collect();
        history.push(guess_char[0]);
        for i in 0..secret_word.len(){
            if already_guessed[i]=='-' && secret_word_chars[i]==guess_char[0]{
                already_guessed[i]=guess_char[0];
                left-=1;
                _correct=true;
                break;
            }
        }
        if _correct==false{
            chance-=1;
        }
    }
    if left==0{
        println!("Congratulations you guessed the secret word:{}",secret_word);
    }else{
        println!("Sorry, you ran out of guesses!");
    }
    // Your code here! :)
}
