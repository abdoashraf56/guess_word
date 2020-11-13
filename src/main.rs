use std::io;
use std::cmp::Ordering;
use rand::Rng ;


fn main() {

    //Lists of guessed words 
    let words : [&str ; 6] = [
        "eat" ,
        "Hypo" ,
        "lunch" ,
        "egypt" ,
        "coopreation" ,
        "rice"
    ];

    //Get random word and its length
    let random  = rand::thread_rng().gen_range(0,6);
    let  word : &str = words[random];
    let word_length : usize = word.len();

    //Define the prefix
    let prefix : &str = "*";

    //Get how many times to try from the lenght of word
    let mut attemp = word_length.clone();

    //Create the guess teample from the word
    let mut guess = vec![prefix;word_length];

    //Create the Stdin to get user data
    let std = io::stdin();

    println!("You word is {}" , guess.join(prefix));

    
    loop {
        //Check if user consume his all attemps
        if attemp == 0 {
            println!("You Loss");
            println!("Your word was {}" ,word);
            break;
        }

        //Check if user guess all the letters in word 
        if guess.contains(&prefix) == false {
            println!("You Win");
            break;
        }
        
        
        //Gettting user input and clear it
        let mut ch = String::new();
        std.read_line(&mut ch).expect("Error in read");
        let c = ch.trim();

        //Looking for the letter in word
        let match_cases: Vec<_> = word.match_indices(c).collect();
        
        //Check if letter in the word or not 
        if match_cases.len() > 0 {
            //Replace the prefix with letter
            for elem in match_cases {
                guess[elem.0] = elem.1;
            }
            println!("{}", guess.join(""));
        } else {
            //Notify user he is worng guest and decrase his attemps
            println!("No match letter found please guess again");
            attemp -= 1 ;
            println!("You have {} attemps left" , attemp);
        }
    }
}
