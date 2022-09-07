use std::collections::HashSet;

// static ALL_NUMBERS: &'static [usize] = &[1, 2, 3, 4, 5, 6, 7, 8, 9];

// struct SolverCell<'a> {
//     column_set: &'a mut HashSet<usize>,
//     row_set: &'a mut HashSet<usize>,
//     square_set: &'a mut HashSet<usize>,
//     value: Option<usize>,
// }

fn print_board(board: &[[usize; 9]; 9]) {
    for row in board.iter() {
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}

fn main() {
    // println!("Hello, world!");
    // let test = HashSet::from([1, 2, 3]);
    let all_numbers: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut columns: Vec<HashSet<usize>> = vec![
        HashSet::from_iter(all_numbers.clone().into_iter()),
        HashSet::from_iter(all_numbers.clone().into_iter()),
        HashSet::from_iter(all_numbers.clone().into_iter()),
        HashSet::from_iter(all_numbers.clone().into_iter()),
        HashSet::from_iter(all_numbers.clone().into_iter()),
        HashSet::from_iter(all_numbers.clone().into_iter()),
        HashSet::from_iter(all_numbers.clone().into_iter()),
        HashSet::from_iter(all_numbers.clone().into_iter()),
        HashSet::from_iter(all_numbers.clone().into_iter()),
    ];
    let mut rows: Vec<HashSet<usize>> = vec![
        HashSet::from_iter(all_numbers.clone().into_iter()),
        HashSet::from_iter(all_numbers.clone().into_iter()),
        HashSet::from_iter(all_numbers.clone().into_iter()),
        HashSet::from_iter(all_numbers.clone().into_iter()),
        HashSet::from_iter(all_numbers.clone().into_iter()),
        HashSet::from_iter(all_numbers.clone().into_iter()),
        HashSet::from_iter(all_numbers.clone().into_iter()),
        HashSet::from_iter(all_numbers.clone().into_iter()),
        HashSet::from_iter(all_numbers.clone().into_iter()),
    ];
    let mut squares: Vec<Vec<HashSet<usize>>> = vec![
        vec![
            HashSet::from_iter(all_numbers.clone().into_iter()),
            HashSet::from_iter(all_numbers.clone().into_iter()),
            HashSet::from_iter(all_numbers.clone().into_iter()),
        ],
        vec![
            HashSet::from_iter(all_numbers.clone().into_iter()),
            HashSet::from_iter(all_numbers.clone().into_iter()),
            HashSet::from_iter(all_numbers.clone().into_iter()),
        ],
        vec![
            HashSet::from_iter(all_numbers.clone().into_iter()),
            HashSet::from_iter(all_numbers.clone().into_iter()),
            HashSet::from_iter(all_numbers.clone().into_iter()),
        ],
    ];
    // let mut solver_board: Vec<Vec<SolverCell>> = vec![vec![]];

    let mut board = [
        [5, 9, 0,   0, 8, 4,   0, 0, 0],
        [0, 4, 0,   0, 0, 0,   0, 6, 9],
        [0, 0, 0,   1, 9, 0,   8, 4, 0],

        [0, 0, 0,   2, 3, 6,   4, 5, 8],
        [3, 0, 2,   0, 0, 0,   6, 0, 0],
        [0, 0, 0,   0, 0, 0,   3, 0, 0],

        [0, 2, 0,   3, 0, 7,   9, 0, 4],
        [0, 3, 0,   0, 1, 0,   0, 0, 0],
        [6, 0, 0,   5, 4, 0,   0, 3, 0],
    ];

    for row in 0..9 {
        for column in 0..9 {
            let value = board[row][column];
            if value != 0 {
                let row_set = &mut rows[row];
                let column_set = &mut columns[column];
                let square_set = &mut squares[row / 3][column / 3];
                row_set.remove(&value);
                column_set.remove(&value);
                square_set.remove(&value);
            }
        }
    }

    // println!("{:?}", board);

    print_board(&board);
    println!();

    loop {
        let mut changed = false;
        for row in 0..9 {
            for column in 0..9 {
                let value = board[row][column];
                if value == 0 {
                    let row_set = &mut rows[row];
                    let column_set = &mut columns[column];
                    let square_set = &mut squares[row / 3][column / 3];
                    let mut possible_values = row_set.clone();
                    possible_values.retain(|&x| column_set.contains(&x) && square_set.contains(&x));
                    if possible_values.len() == 1 {
                        board[row][column] = *possible_values.iter().next().unwrap();
                        row_set.remove(&board[row][column]);
                        column_set.remove(&board[row][column]);
                        square_set.remove(&board[row][column]);
                        println!("{}", board[row][column]);
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    print_board(&board);
}
