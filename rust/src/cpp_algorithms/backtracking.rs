use super::Timer;

fn graph_colouring_is_safe<const N: usize>(graph: &[[i32; N]; N], colored: usize, color: usize, colors: &[usize; N]) -> bool {
    for vertex in 0..colored {
        // connected
        if vertex != colored && graph[colored][vertex] == 1 {
            if color == colors[vertex] {
                return false;
            }
        }
    }
    true
}

pub fn graph_colouring<const N: usize>(graph: &[[i32; N]; N], colors: &mut [usize; N], colored: usize, m: usize) -> bool {
    if colored == N { return true; }

    for color in 1..m+1 {
        if graph_colouring_is_safe(graph, colored, color, colors) {
            colors[colored] = color;
            graph_colouring(graph, colors, colored+1, m);
            if graph_colouring(graph, colors, colored+1, m) {
                return true;
            }
            colors[colored] = 0; // backtracking
        }
    }

    false
}

fn sudoku_display<const N: usize>(mat: &[[i32; N]; N], indicator: &[[i32; N]; N]) {
    for i in 0..N {
        for j in 0..N {
            if indicator[i][j] != 0 {
                print!("{} ", mat[i][j]);
            } else {
                print!("\x1b[93m{}\x1b[0m ", mat[i][j]);
            }

            if (j + 1) % 3 == 0 {
                print!("\t");
            }
        }
        if (i + 1) % 3 == 0 {
            print!("\n");
        }
        print!("\n");
    }
}

fn sudoku_is_possible<const N: usize>(mat: &[[i32; N]; N], row: usize, col: usize, no: i32) -> bool {
    for i in 0..N {
        if mat[i][col] == no { return false; }
    }

    for j in 0..N {
        if mat[row][j] == no { return false; }
    }

    let row_id = row / 3;
    let col_id = col / 3;

    for i in 0..3 {
        for j in 0..3 {
            if mat[3 * row_id + i][3 * col_id + j] == no { return false; }
        }
    }

    true
}

pub fn sudoku_solve<const N: usize>(mat: &mut [[i32; N]; N], row: usize, col: usize) -> bool {
    if row >= N { return true; }
    if col >= N { return sudoku_solve(mat, row + 1, 0); }
    if mat[row][col] != 0 { return sudoku_solve(mat, row, col + 1); }

    for no in 1..10 {
        if sudoku_is_possible(mat, row, col, no) {
            mat[row][col] = no;

            if sudoku_solve(mat, row, col+1) {
                return true;
            }
        }

    }
    mat[row][col] = 0; // backtracing ...

    false
}

pub fn rat_maze_solve<const N: usize>(curr_row: usize, curr_col: usize, maze: &[[isize; N]; N], sol: &mut [[isize; N]; N]) -> bool {
    sol[curr_row][curr_col] = 1;
    if curr_row >= N - 1 && curr_col >= N - 1 { return true; }

    if curr_row + 1 < N && maze[curr_row+1][curr_col] == 1 && rat_maze_solve(curr_row+1, curr_col, maze, sol) {
        return true;
    }
    if curr_col + 1 < N && maze[curr_row][curr_col+1] == 1 && rat_maze_solve(curr_row, curr_col+1, maze, sol) {
        return true;
    }

    sol[curr_row][curr_col] = 0;

    false
}

fn n_queens_display<const N: usize>(board: &[[i32; N]; N]) {
    for row in 0..N {
        for col in 0..N {
            print!("{} ", board[row][col]);
        }
        print!("\n");
    }
    print!("\n");
}

fn n_queens_is_safe<const N: usize>(board: &[[i32; N]; N], col: usize, row: usize) -> bool {
    for c in 0..col {
        if board[row][c] == 1 { return false; }
    }

    for s in 1..col+1 {
        if s <= row && board[row-s][col-s] == 1 { return false; }
        if s + row < N && board[row+s][col-s] == 1 { return false; }
    }

    true
}

pub fn n_queens_solve<const N: usize>(board: &mut [[i32; N]; N], col: usize) {
    if col >= N {
        n_queens_display(board);
        return;
    }

    for row in 0..N {
        if n_queens_is_safe(board, col, row) {
            board[row][col] = 1;
            n_queens_solve(board, col+1);

            board[row][col] = 0; // backtrace
        }
    }
}

pub fn minimax_solve<const N: usize>(depth: usize, node_index: usize, is_max: bool, scores: &[i32; N], height: usize) -> i32 {
    if depth == height { return scores[node_index]; }

    let v1 = minimax_solve(depth+1, node_index*2, !is_max, scores, height);
    let v2 = minimax_solve(depth+1, node_index*2+1, !is_max, scores, height);

    if is_max { v1.max(v2) } else { v1.min(v2) }
}

fn knight_tour_is_safe(nx: isize, ny: isize, n: usize, sol: &Vec<Vec<isize>>) -> bool {
    return (nx >= 0) && (ny >= 0) && (nx < n as isize) && (ny < n as isize) && (sol[nx as usize][ny as usize] == -1)
}

pub fn knight_tour_solve(rank: usize, sx: isize, sy: isize, sol: &mut Vec<Vec<isize>>, moves: &Vec<(isize, isize)>) -> bool {
    let n = moves.len();
    if n*n == rank { return true; }

    for (mx, my) in moves.iter() {
        let nx = sx + mx;
        let ny = sy + my;
        if knight_tour_is_safe(nx, ny, n, sol) {
            sol[nx as usize][ny as usize] = rank as isize;
            if knight_tour_solve(rank+1, nx, ny, sol, moves) {
                return true;
            } else {
                sol[nx as usize][ny as usize] = -1;
            }
        }
    }

    false
}

fn knight_tour_is_safe_const<const N: usize>(nx: isize, ny: isize, sol: &[[isize; N]; N]) -> bool {
    return (nx >= 0) && (ny >= 0) && (nx < N as isize) && (ny < N as isize) && (sol[nx as usize][ny as usize] == -1)
}

pub fn knight_tour_solve_const<const N: usize>(rank: usize, sx: isize, sy: isize, sol: &mut [[isize; N]; N], moves: &[(isize, isize); N]) -> bool {
    if N * N == rank { return true; }
    for (mx, my) in moves.iter() {
        let nx = sx + mx;
        let ny = sy + my;
        if knight_tour_is_safe_const(nx, ny, sol) {
            sol[nx as usize][ny as usize] = rank as isize;

            if knight_tour_solve_const(rank+1, nx, ny, sol, moves) {
                return true;
            } else {
                sol[nx as usize][ny as usize] = -1;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_colouring_sample() {
        // Create following graph and test whether it is 3 colorable
        // (3)---(2)
        // |   / |
        // |  /  |
        // | /   |
        // (0)---(1)

        const v: usize = 4;
        let graph = [
            [0, 1, 1, 1],
            [1, 0, 1, 0],
            [1, 1, 0, 1],
            [1, 0, 1, 0],
        ];
            let m = 3;
            let mut colors = [0; v];

            if graph_colouring(&graph, &mut colors, 0, m) {
                println!("solution founded: {:?}", colors);
            } else {
                println!("solution not founed...");
            }
    }

    fn sudoku_read<const N: usize>(mat: &mut [[i32; N]; N], buf: &str) {
        for (row, line) in buf.lines().enumerate() {
            if row < N {
                for (col, c) in line.split(" ").enumerate() {
                    if col < N {
                        mat[row][col] = c.parse().unwrap();
                    }
                }
            }
        }
    }

    // use example from [sudoku](http://lipas.uwasa.fi/~timan/sudoku/)
    #[test]
    fn sudoku_samples() {
        let proj_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let p_dir = proj_dir.join("assets/sudoku/puzzles");
        let s_dir = proj_dir.join("assets/sudoku/solutions");

        let mut mat = [[0; 9]; 9];
        let mut sol = [[0; 9]; 9];

        for entry in p_dir.read_dir().unwrap() {
            if let Ok(entry) = entry {
                let p = entry.path();
                let t = s_dir.join(format!("{}_s.txt", p.file_stem().unwrap().to_str().unwrap()));
                assert!(p.is_file() && t.is_file());

                let tn = format!("{}", p.display());
                let _t = Timer::new(&tn);

                // mat.clear(); sol.clear();

                let ps = std::fs::read_to_string(p).unwrap();
                let ss = std::fs::read_to_string(t).unwrap().replace("\r\n---------------------", "").replace(" |", "");

                sudoku_read(&mut mat, &ps);
                sudoku_read(&mut sol, &ss);
                // println!("{:?} \n {:?}", mat, sol);

                assert!(sudoku_solve(&mut mat, 0, 0));
                assert_eq!(mat, sol);
                // println!("{:?} \n {:?}", mat, sol);
            }
        }
    }

    fn sudoku_sample<const N: usize>(mat: &mut [[i32; N]; N]) {
        let indicator = mat.clone();

        // n_queens_display(&mat, &indicator);
        // println!(">>>>>>>>>>>>>>>>>>>");

        if sudoku_solve(mat, 0, 0) {
            println!("solution founded...");
            sudoku_display(&mat, &indicator);
        } else {
            println!("no solution can be found...");
        }
    }

    #[test]
    fn sudoku_sample01() {
        let mut mat = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        sudoku_sample(&mut mat);
    }

    #[test]
    fn sudoku_sample02() {
        let mut mat = [
            [0, 4, 0, 0, 0, 0, 1, 7, 9, ],
            [0, 0, 2, 0, 0, 8, 0, 5, 4, ],
            [0, 0, 6, 0, 0, 5, 0, 0, 8, ],
            [0, 8, 0, 0, 7, 0, 9, 1, 0, ],
            [0, 5, 0, 0, 9, 0, 0, 3, 0, ],
            [0, 1, 9, 0, 6, 0, 0, 4, 0, ],
            [3, 0, 0, 4, 0, 0, 7, 0, 0, ],
            [5, 7, 0, 1, 0, 0, 2, 0, 0, ],
            [9, 2, 8, 0, 0, 0, 0, 6, 0, ],
        ];
        sudoku_sample(&mut mat);
    }

    #[test]
    fn sudoku_sample03() {
        let mut mat = [
            [0, 0, 0, 0, 0, 3, 0, 1, 7],
            [0, 1, 5, 0, 0, 9, 0, 0, 8],
            [0, 6, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 7, 0, 0, 0],
            [0, 0, 9, 0, 0, 0, 2, 0, 0],
            [0, 0, 0, 5, 0, 0, 0, 0, 4],
            [0, 0, 0, 0, 0, 0, 0, 2, 0],
            [5, 0, 0, 6, 0, 0, 3, 4, 0],
            [3, 4, 0, 2, 0, 0, 0, 0, 0],
        ];
        sudoku_sample(&mut mat);
    }

    #[test]
    fn rat_maze_sample() {
        const size: usize = 4;
        let maze = [
            [1, 0, 1, 0],
            [1, 0, 1, 1],
            [1, 0, 0, 1],
            [1, 1, 1, 1],
        ];

        let mut sol = [[0; size]; size];
        if rat_maze_solve(0, 0, &maze, &mut sol) {
            for row in sol.iter() {
                println!("{:?}", row);
            }
        } else {
            println!("no sol found");
        }
    }

    #[test]
    fn n_queens_sample() {
        const n: usize = 4;
        let mut board = [[0; n]; n];
        n_queens_solve(&mut board, 0);
    }

    #[test]
    fn minimax_sample() {
        let scores = [90, 23, 6, 33, 21, 65, 123, 34423];
        let height = (scores.len() as f64).log2().round() as usize;

        println!("Optimal value: {}", minimax_solve(0, 0, true, &scores, height));
    }

    #[test]
    fn knight_tour_sample01() {
        let _t = Timer::new("sample01");
        const n: usize = 8;
        let moves = vec![(2, 1),  (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1)];
        let mut sol = vec![vec![-1; n]; n];
        sol[0][0] = 0;

        if !knight_tour_solve(1, 0, 0, &mut sol, &moves) {
            eprintln!("Error: solution does not exist.\n");
        } else {
            for i in 0..n {
                for j in 0..n {
                    print!("{:02} ", sol[i][j]);
                }
                print!("\n");
            }

        }
    }

    #[test]
    fn knight_tour_sample02() {
        let _t = Timer::new("sample02");
        const n: usize = 8;
        let moves = [(2, 1),  (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1)];
        let mut sol = [[-1; n]; n];
        sol[0][0] = 0;

        if !knight_tour_solve_const(1, 0, 0, &mut sol, &moves) {
            eprintln!("Error: solution does not exist.\n");
        } else {
            for i in 0..n {
                for j in 0..n {
                    print!("{:02} ", sol[i][j]);
                }
                print!("\n");
            }
        }
    }
}
