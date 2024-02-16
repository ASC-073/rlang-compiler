mod ast;

fn main(){
    let input = "7 + 0 / 1 * 22";
    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token(){
        tokens.push(token);
    }
    // Print the array syntax, print EVERYTHING lol
    println!("{:?}",tokens);
}