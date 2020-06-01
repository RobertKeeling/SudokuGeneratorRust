use std::time::Instant;
use rayon::prelude::*;

use rand::Rng;

fn generate_complete_board() -> [[u8; 9]; 9] {
    let mut board: [[u8; 9]; 9] = [[0; 9]; 9];
    for i in 0..9{
        for j in 0..9{
            let v = i as u8;
            let u = j as u8;
            board[i][j] = (v*3+u+v/3)%9+1;
        }
    }
    for _ in 0..100{
        shift_rows(&mut board);
        shift_cols(&mut board);
    }
    return board;
}

fn print_board(board: [[u8; 9]; 9]) {
    for (i, row) in board.iter().enumerate(){
        let mut string = String::new();
        for (j, cell) in row.iter().enumerate(){
            string += &cell.to_string();
            if j%3==2 {
                string += " ";
            }
        }
        println!("{}", string);
        if i%3==2 {
            println!("");
        }
    }
    println!("");
}

fn shift_cols(board: &mut [[u8; 9]; 9]) {
    let mut rng = rand::thread_rng();
    let r:i8 = rng.gen_range(0, 9);
    let poss: [i8; 2];
    match r {
        0|3|6 => poss = [1, 2],
        1|4|7 => poss = [-1, 1],
        2|5|8 => poss = [-2, -1],
        _ => poss = [0, 0],
    }
    if let Some(dec) = rng.choose(&poss) {
        for i in 0..9 {
            let saved = board[i][r as usize];
            let v = (r + dec) as usize;
            board[i][r as usize] = board[i][v];
            board[i][v] = saved;
        }
    }
}

fn shift_rows(board: &mut [[u8; 9]; 9]) {
    let mut rng = rand::thread_rng();
    let r:i8 = rng.gen_range(0, 9);
    let poss: [i8; 2];
    match r {
        0|3|6 => poss = [1, 2],
        1|4|7 => poss = [-1, 1],
        2|5|8 => poss = [-2, -1],
        _ => poss = [0, 0],
    }
    if let Some(dec) = rng.choose(&poss) {
        for i in 0..9 {
            let saved = board[r as usize][i];
            let v = (r + dec) as usize;
            board[r as usize][i] = board[v][i];
            board[v][i] = saved;
        }
    }
}

fn board_correct(board: [[u8; 9]; 9]) -> bool {
    let mut correct = true;
    let box_values: [[i8; 3]; 3] = [[0, 1, 2], [-1, 0, 1], [-2, -1, 0]];
    for i in 0..9 {
        for j in 0..9 {
            let v = board[i][j];
            for k in 0..9 {
                correct = correct&&(board[i][k]!=v||k==j);
                correct = correct&&(board[k][j]!=v||k==i);
            }
            let box_row = box_values[i%3];
            let box_col = box_values[j%3];
            for k in box_row.iter() {
                for l in box_col.iter() {
                    let r = (i as i8 + k) as usize;
                    let c = (j as i8 + l) as usize;
                    correct = correct&&(board[r][c]!=v||(*k==0&&*l==0));
                }
            }
        }
    }
    return correct;
}

fn find_tiles(board: [[u8; 9]; 9], f: &dyn Fn(u8) -> bool) -> Vec<[usize; 2]> {
    let mut vec: Vec<[usize; 2]> = Vec::new();
    for (i, row) in board.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if f(*cell) {
                vec.push([i, j]);
            }
        }
    }
    return vec;
}

fn find_empty_tiles(board: [[u8; 9]; 9]) -> Vec<[usize; 2]> {
    find_tiles(board, &|x: u8| -> bool {x==0})
}

fn find_full_tiles(board: [[u8; 9]; 9]) -> Vec<[usize; 2]> {
    find_tiles(board, &|x: u8| -> bool {x!=0})
}

fn find_box_increments(r: usize, c: usize) -> [[usize; 2]; 8] {
    let mut box_increments: [[usize; 2]; 8] = [[0; 2]; 8];
    let box_values: [[i8; 3]; 3] = [[0, 1, 2], [-1, 0, 1], [-2, -1, 0]];
    let box_row = box_values[r%3];
    let box_col = box_values[c%3];
    let mut count = 0;
    for i in box_row.iter() {
        for j in box_col.iter() {
            if *i!=0 || *j!=0 {
                box_increments[count] = [(r as i8 + i) as usize, (c as i8 + j) as usize];
                count += 1;
            }
        }
    }
    return box_increments;
}

fn find_possibilities_for_position(board: [[u8; 9]; 9], r: usize, c: usize) -> Vec<u8> {
    let mut possibilities: Vec<u8> = Vec::new();
    let box_increments = find_box_increments(r, c);
    for i in 1..10 {
        possibilities.push(i as u8);
    }
    for i in 0..9 {
        possibilities.retain(|&x| x!=board[r][i]&&x!=board[i][c]);
    }
    for [r0, c0] in box_increments.iter() {    
        possibilities.retain(|&x| x!=board[*r0][*c0]);
    }
    return possibilities;
}

fn find_forced_moves(board: [[u8; 9]; 9]) -> Vec<[u8; 3]>{
    let empties = find_empty_tiles(board);
    let mut forced: Vec<[u8; 3]> = Vec::new();
    for [r, c] in empties.iter() {
        let mut poss = find_possibilities_for_position(board, *r as usize, *c as usize);
        if poss.iter().len()==1{
            forced.push([*r as u8, *c as u8, poss[0]]);
            continue;
        }
        for i in 0..9 {
            if board[*r][i] == 0 {
                let other_poss = find_possibilities_for_position(board, *r as usize, i);
                for j in other_poss.iter() {
                    poss.retain(|&x| x!=*j);
                }
            }
            if board[i][*c] == 0 {
                let other_poss = find_possibilities_for_position(board, i, *c as usize);
                for j in other_poss.iter() {
                    poss.retain(|&x| x!=*j);
                }
            }
        }
        let box_increments = find_box_increments(*r, *c);
        for [r0, c0] in box_increments.iter() {
            if board[*r0][*c0] == 0{
                let other_poss = find_possibilities_for_position(board, *r0, *c0);
                for j in other_poss.iter() {
                    poss.retain(|&x| x!=*j);
                }
            }
        }
        if poss.iter().len()==1{
            forced.push([*r as u8, *c as u8, poss[0]]);
        }
    }
    return forced;
}

fn make_move(board: &mut [[u8; 9]; 9]) -> bool {
    let forced = find_forced_moves(*board);
    if forced.iter().len() == 0{
        return false;
    }
    let mut rng = rand::thread_rng();
    let ran: usize = rng.gen_range(0, forced.iter().len());
    let [r, c, v] = forced[ran];
    board[r as usize][c as usize] = v;
    return true;
}

fn make_moves(board: &mut [[u8; 9]; 9]) -> bool {
    let forced = find_forced_moves(*board);
    if forced.iter().len() == 0{
        return false;
    }
    for [r, c, v] in forced.iter() {
        board[*r as usize][*c as usize] = *v;
    }
    return true;
}

fn board_possible(board: [[u8; 9]; 9]) -> bool {
    let mut clone_board = board.clone();
    while make_moves(&mut clone_board){};
    return board_correct(clone_board);
}

fn generate_puzzle(board: &mut [[u8; 9]; 9], mut tile_count: u8) -> u8{
    let tile_goal = tile_count.clone();
    let mut rng = rand::thread_rng();
    let mut tiles = find_full_tiles(*board);
    while 81-tile_count > 0 {
        let ran: usize = rng.gen_range(0, tiles.iter().len());
        let [r, c] = tiles[ran];
        tiles.retain(|&x| x!=[r, c]);
        let temp = board[r][c];
        board[r][c] = 0;
        if board_possible(*board) {
            tile_count += 1;
        }else {
            board[r][c] = temp;
        }
        if tiles.iter().len() == 0 {
            break;
        }
    }
    return tile_goal + (81 - tile_count);
}

fn main() {
    let mut vec = Vec::new();
    for i in 0..100u32 {
        vec.push(i);
    }
    let start = Instant::now();
    vec.iter().for_each(|_p| {
        let mut board = generate_complete_board();
        generate_puzzle(&mut board, 20);
    });
    let duration = start.elapsed();
    println!("In Series: {:?}", duration);
    let start0 = Instant::now();
    vec.par_iter().for_each(|_p| {
        let mut board = generate_complete_board();
        generate_puzzle(&mut board, 20);
    });
    let duration0 = start0.elapsed();
    println!("In Parallel: {:?}", duration0);
}
