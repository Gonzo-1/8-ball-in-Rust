// --- Get the tools we need ---
use std::io;                      // Lets us read and write to the console
use rand::seq::SliceRandom;       // Lets us pick a random item from a list
use std::io::Write;               // Lets us "flush" the console output

fn main() {
    println!("--- 🎱 Magic 8-Ball 🎱 ---");
    println!("Ask me any yes/no question. (Type 'quit' to exit)\n");

    // All our 8-ball answers
    let responses = [
        "It is certain.",
        "It is decidedly so.",
        "Without a doubt.",
        "Yes, definitely.",
        "You may rely on it.",
        "As I see it, yes.",
        "Most likely.",
        "Outlook good.",
        "Yes.",
        "Signs point to yes.",
        "Reply hazy, try again.",
        "Ask again later.",
        "Better not tell you now.",
        "Cannot predict now.",
        "Concentrate and ask again.",
        "Don't count on it.",
        "My reply is no.",
        "My sources say no.",
        "Outlook not so good.",
        "Very doubtful."
    ];

    // Start the random number generator
    let mut rng = rand::thread_rng();

    // Main program loop that runs forever (until "quit")
    loop {
        print!("Your question: "); // Ask the user for a question
        
        // Force the "Your question: " prompt to show up immediately
        io::stdout().flush().expect("Failed to flush stdout");

        // Create an empty string to store the user's question
        let mut question = String::new();

        // Read the user's typed line into our 'question' string
        io::stdin()
            .read_line(&mut question)
            .expect("Failed to read line."); // Stop if reading fails

        // --- Check the User's Input ---

        // Clean up the input (remove whitespace/newline) and check if the user typed "quit"
        if question.trim().to_lowercase() == "quit" {
            println!("Goodbye!");
            break; // Exit the loop
        }
        
        // Check if the user just hit Enter without typing anything
        if question.trim().is_empty() {
            println!("You must ask a question.");
            continue; // Go back to the start of the loop
        }

        // --- Give an Answer ---

        // Pick a random answer from our list
        let answer = responses
            .choose(&mut rng)
            .expect("Response list was empty!");

        // Print the chosen answer
        println!("\nThe 8-ball says: {}\n", **answer);
    }
}
