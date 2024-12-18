use std::io::{self,BufRead};
fn find_xmas_north(char_tab: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    if i < 3 {
        return 0;
    }
    if  char_tab[i-1][j] == 'M' &&
        char_tab[i-2][j] == 'A' &&
        char_tab[i-3][j] == 'S' {
           return 1;
    }

    return 0;
}

fn find_xmas_north_west(char_tab: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    if i < 3 || j < 3{
        return 0;
    }
    if  char_tab[i-1][j-1] == 'M' &&
        char_tab[i-2][j-2] == 'A' &&
        char_tab[i-3][j-3] == 'S' {
           return 1;
    }
    return 0;
}

fn find_xmas_north_east(char_tab: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    if i < 3 || j > char_tab.len()-4 {
        return 0;
    }
    if  char_tab[i-1][j+1] == 'M' &&
        char_tab[i-2][j+2] == 'A' &&
        char_tab[i-3][j+3] == 'S' {
           return 1;
    }
    return 0;
}

fn find_xmas_east(char_tab: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    if j > char_tab.len()-4 {
        return 0;
    }
    if  char_tab[i][j+1] == 'M' &&
        char_tab[i][j+2] == 'A' &&
        char_tab[i][j+3] == 'S' {
           return 1;
    }
    return 0;
}

fn find_xmas_west(char_tab: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    if j < 3 {
        return 0;
    }
    if  char_tab[i][j-1] == 'M' &&
        char_tab[i][j-2] == 'A' &&
        char_tab[i][j-3] == 'S' {
           return 1;
    }
    return 0;

}

fn find_xmas_south(char_tab: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    if i > char_tab.len()-4 {
        return 0;
    }
    if  char_tab[i+1][j] == 'M' &&
        char_tab[i+2][j] == 'A' &&
        char_tab[i+3][j] == 'S' {
           return 1;
    }
    return 0;
}

fn find_xmas_south_west(char_tab: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    if i > char_tab.len()-4 || j < 3 {
        return 0;
    }
    if  char_tab[i+1][j-1] == 'M' &&
        char_tab[i+2][j-2] == 'A' &&
        char_tab[i+3][j-3] == 'S' {
           return 1;
    }
    return 0;
}

fn find_xmas_south_east(char_tab: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    if i > char_tab.len()-4 || j > char_tab.len()-4 {
        return 0;
    }
    if  char_tab[i+1][j+1] == 'M' &&
        char_tab[i+2][j+2] == 'A' &&
        char_tab[i+3][j+3] == 'S' {
           return 1;
    }
    return 0;
}

fn find_x_mas(char_tab: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    if i < 1 || i > char_tab.len()-2 || j > char_tab.len()-2 || j <1 {
        return 0;
    }
    if  char_tab[i-1][j-1] == 'M' &&
        char_tab[i-1][j+1] == 'M' &&
        char_tab[i+1][j-1] == 'S' &&
        char_tab[i+1][j+1] == 'S' {
           return 1;
    }
    if  char_tab[i-1][j-1] == 'S' &&
        char_tab[i-1][j+1] == 'M' &&
        char_tab[i+1][j-1] == 'S' &&
        char_tab[i+1][j+1] == 'M' {
           return 1;
    }
    if  char_tab[i-1][j-1] == 'M' &&
        char_tab[i-1][j+1] == 'S' &&
        char_tab[i+1][j-1] == 'M' &&
        char_tab[i+1][j+1] == 'S' {
           return 1;
    }
    if  char_tab[i-1][j-1] == 'S' &&
        char_tab[i-1][j+1] == 'S' &&
        char_tab[i+1][j-1] == 'M' &&
        char_tab[i+1][j+1] == 'M' {
           return 1;
    }
    return 0;
}

fn find_xmas(char_tab: Vec<Vec<char>>) {
    let mut xmas_sum: u32 = 0;
    let mut x_mas_sum: u32 = 0;
    for (i, row) in char_tab.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'X' {
                    xmas_sum += find_xmas_north(&char_tab, i, j);
                    xmas_sum += find_xmas_south(&char_tab, i, j);
                    xmas_sum += find_xmas_west(&char_tab, i, j);
                    xmas_sum += find_xmas_east(&char_tab, i, j);
                    xmas_sum += find_xmas_north_west(&char_tab, i, j);
                    xmas_sum += find_xmas_north_east(&char_tab, i, j);
                    xmas_sum += find_xmas_south_west(&char_tab, i, j);
                    xmas_sum += find_xmas_south_east(&char_tab, i, j);
            } if *c == 'A' {
                x_mas_sum += find_x_mas(&char_tab, i, j);
            }
        }
    }
    println!("xmas_sum = {}", xmas_sum);
    println!("x_mas_sum = {}", x_mas_sum);
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut char_tab: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let sline = line.unwrap();
        let char_vec: Vec<char> = sline.chars().collect();
        char_tab.push(char_vec);
    }
    find_xmas(char_tab);
}
