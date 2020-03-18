use std::error::Error;
#[derive(Debug)]
enum TokenType {
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    End,
    Illegal,
}
fn main() -> Result<(), Box<dyn Error>> {
    use std::env; //choose a module to start with (standard env)
    use std::fs::File; //module to open file
    use std::io::Read; //module to read to string
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut file = File::open(filename)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    let mut tokens: Vec<TokenType> = Vec::new();
    for token in file_contents.chars() {
        match token {
            '+' => tokens.push(TokenType::Plus),
            '-' => tokens.push(TokenType::Minus),
            '*' => tokens.push(TokenType::Star),
            '/' => tokens.push(TokenType::Slash),
            '^' => tokens.push(TokenType::Caret),
            '(' => tokens.push(TokenType::LParen),
            ')' => tokens.push(TokenType::RParen),
            ' ' => (),
            '\n' => tokens.push(TokenType::End), 
            _ => tokens.push(TokenType::Illegal), //println!("Error invalid input"),
        }
    }
    for token in tokens{
      match token{
          TokenType::Illegal => panic!(),
          TokenType::Plus => println!("Plus"),
          TokenType::Minus => println!("Minus"),
          TokenType::Star => println!("Star"),
          TokenType::Slash => println!("Slash"),
          TokenType::Caret => println!("Caret"),
          TokenType::LParen => println!("LParen"),
          TokenType::RParen => println!("RParen"),
          TokenType::End => (),
      }
      
    }

    Ok(())
}
