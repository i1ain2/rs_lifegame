use std::io;

/*
 * - [ ] 幅高さを可変にする
 * - [ ] 表示がterminal末尾になっちゃってるので変更する
 * - [ ] 各セルをセルっぽくする
 */

fn print_cells(cells: &[[usize; 32]; 32]) -> () {
    for y in 0..32 {
        for x in 0..32 {
            let c = if cells[y][x] == 1 { "#" } else { "." };
            print!("{} ", c);
        }
        println!();
    }
}

fn lifegame(cells: &[[usize; 32]; 32]) -> [[usize; 32]; 32] {
    let mut next = [[0; 32]; 32];

    for y in 0..32 {
        for x in 0..32 {
            // 周囲のセルの生死を取得
            let up = if 0 < y { y - 1 } else { 31 };
            let down = if y < 31 { y + 1 } else { 0 };
            let left = if 0 < x { x - 1 } else { 31 };
            let right = if x < 31 { x + 1 } else { 0 };

            let lives = cells[up][left]
                + cells[up][x]
                + cells[up][right]
                + cells[y][left]
                + cells[y][right]
                + cells[down][left]
                + cells[down][x]
                + cells[down][right];

            // 次世代の生死判断
            if cells[y][x] == 1 {
                if 2 <= lives && lives <= 3 {
                    next[y][x] = 1;
                } else {
                    next[y][x] = 0;
                }
            } else {
                if lives == 3 {
                    next[y][x] = 1;
                }
            }
        }
    }
    return next;
}

// TODO: 一番上まで持って行きたい
// 制御文字を端末に送信してるのはわかるが、詳細は知らないので調べる
fn clear_terminal() -> () {
    print!("{}[2J", 27 as char);
}

fn main() {
    clear_terminal();
    let mut cells = [[0; 32]; 32];

    // 初期状態
    // ブリンカー
    // cells[10][10] = 1;
    // cells[11][10] = 1;
    // cells[12][10] = 1;
    // グライダー
    cells[10][10] = 1;
    cells[10][11] = 1;
    cells[10][12] = 1;
    cells[11][10] = 1;
    cells[12][12] = 1;
    print_cells(&cells);

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read user input");
        if input != "\n" {
            break;
        }

        clear_terminal();
        cells = lifegame(&cells);
        print_cells(&cells);
    }
}
