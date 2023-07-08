use crate::utility::utility::{Player, Movement};
use std::io::{self, stdin};
mod plate;
mod utility;
use std::io::{stdout, Read};
use termion::async_stdin;
use termion::input::TermRead;

fn main() {
    let mut plt: Vec<Vec<i32>>;
    plt = plate::read_map();
    println!("{:?}", plt);
    
    let mut player1:Player=Player{
        body:vec![(Movement::Left,4,2)],
        score: 0,
        name:String::new(),
        alive: true,
    };

    println!("Enter Your Name:");
    let mut s: String=String::new();
    io::stdin().read_line(&mut s);
    player1.name=s;

    println!("your name is {} and your score is {}",player1.name,player1.score);
    while player1.alive==true{
        let stdin = async_stdin();
        
        for key in stdin.keys() {
            if let Ok(key) = key {
                if let Some(character) = key.to_lowercase().next() {
                    if character == 'q' {
                        break;
                    }
                }
            }
        }
    }

}
