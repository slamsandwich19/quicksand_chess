mod engine;

// third party
use cozy_chess::util::parse_uci_move;
use cozy_chess::Board;

// project
//use engine::engine::Engine;
use engine::engine::Engine;

// std
use std::io;

fn uci() {
    println!("id name Quicksand");
    println!("id author BigSauce");
    println!("uciok");
}

fn isready() {
    println!("readyok");
}

fn apply_moves(board: &Board, moves: &[&str]) -> Board {
    let mut new_board = board.clone();

    for move_str in moves {
        match parse_uci_move(&new_board, move_str) {
            Ok(mv) => new_board.play(mv),
            Err(_) => {println!("info error malformed move"); break},
        }
    }
    new_board
}

fn position(command: &Vec<&str>) -> Board {
    if command.len() < 2 {
        return Board::default();
    }

    match command[1] {
        "startpos" => {
            let board = Board::default();
            if command.len() > 3 && command[2] == "moves" {
                apply_moves(&board, &command[3..])
            } else {
                board
            }
        }
        "fen" => {
            // Find where "moves" starts (if present) to separate the FEN tokens
            let moves_idx = command.iter().position(|&r| r == "moves");
            let fen_end = moves_idx.unwrap_or(command.len());
            
            let fen_str = command[2..fen_end].join(" ");
            let board = fen_str.parse::<Board>().unwrap_or_default();

            if let Some(idx) = moves_idx {
                apply_moves(&board, &command[idx + 1..])
            } else {
                board
            }
        }

        _ => Board::default()
    }
}

fn go(engine: &mut Engine, board: &Board, command: &Vec<&str>) {
    match command[1] {
        "depth" => {
            let expected_depth = command[2].parse::<i32>();
            if let Some(best_move) = engine.get_best_move(board, expected_depth.unwrap()) {
                println!("bestmove {}", best_move);
            }
        }
        _ => {
            if let Some(best_move) = engine.get_best_move(board, 3) {
                println!("bestmove {}", best_move);
            }
        }
    }
}

fn main() {
    let mut engine = Engine::new();
    let mut board = Board::default();

    loop {
        // read input
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }
        let command: Vec<&str> = input.split_whitespace().collect();

        if command.is_empty() {
            continue;
        }

        match command[0] {
            "uci" => uci(),
            "isready" => isready(),
            "position" => board = position(&command),
            "go" => go(&mut engine, &board, &command),
            "quit" => break,
            _ => {}
        }
    }
}