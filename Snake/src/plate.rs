use std::fs;
use std::process::Command;
use crate::utility::utility;
use utility::Player;
use utility::Movement;
use rand::Rng;


pub fn clear_terminal() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd").args(&["/C", "cls"]).status();
    } else {
        let _ = Command::new("clear").status();
    }
}

pub fn read_map() -> Vec<Vec<i32>> {
    let file_path = "assets/ensem.txt";
    let mut contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut width = 0;
    let mut height = 0;
    for i in contents.char_indices() {
        if i.1 == '\n' {
            width = i.0;
            height += 1;
        }
    }
    width = ((width + 1) / height) - 1;

    let mut plt: Vec<Vec<i32>> = vec![vec![0; width]; height];

    let mut j = 0;
    for i in contents.char_indices() {
        if i.1 == '\n' {
            j += 1;
            continue;
        }
        if i.1 == '#' {
            plt[j][i.0 - j * (width + 1)] = 1
        } else {
            plt[j][i.0 - j * (width + 1)] = 0
        }
    }
    plt
}

pub fn spawn(map: Vec<Vec<i32>>,ply:Player){
    let mut rng = rand::thread_rng();
    let random_row: i32 = rng.gen_range(1..=map.len() as i32);
    let random_col: i32 = rng.gen_range(1..=map[0].len() as i32);
}

pub fn print_screen(map: Vec<Vec<i32>>,ply:Player) {
    clear_terminal();
}
