use crate::read_input::read_input;
/*
 *"Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!

As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if you could help her with her word search (your puzzle input). She only has to find one word: XMAS.

This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:

..X...
.SAMX.
.A..A.
XMAS.S
.X....

The actual word search will be full of letters instead. For example:

MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX

In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where letters not involved in any XMAS have been replaced with .:

....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX

Take a look at the little Elf's word search. How many times does XMAS appear? */
fn find_word(
    crossword: &Vec<Vec<char>>,
    (y, x): (usize, usize),
    (pos_y, pos_x): (isize, isize),
    remaining_chars: Vec<char>,
) -> bool {
    if remaining_chars.len() == 0 {
        return true;
    }
    let new_y = y as isize + pos_y;
    let new_y = if new_y >= 0 && (new_y as usize) < crossword.len() {
        new_y as usize
    } else {
        return false;
    };
    let new_x = x as isize + pos_x;
    let new_x = if new_x >= 0 && (new_x as usize) < crossword[y].len() {
        new_x as usize
    } else {
        return false;
    };
    let next_letter = crossword[new_y][new_x];
    let next_char = remaining_chars[0];
    if next_letter == next_char {
        return find_word(
            crossword,
            (new_y, new_x),
            (pos_y, pos_x),
            remaining_chars[1..remaining_chars.len()].into(),
        );
    }
    return false;
}
pub fn find_xmas() -> i64 {
    const SURROUND: [(isize, isize); 4] = [
        // (-1, -1), (0, -1), (1, -1),
        // (-1, 0),
        /*(0,0),*/ (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
    const SAMX: [char; 4] = ['S', 'A', 'M', 'X'];
    let crossword: Vec<Vec<char>> = read_input("./inputs/day4.txt")
        .iter()
        .map(|row| row.chars().collect())
        .collect();
    let mut word_count = 0;
    for y in 0..crossword.len() {
        for x in 0..crossword[y].len() {
            let curr_letter = crossword[y][x];
            for dir in SURROUND {
                if curr_letter == XMAS[0] {
                    let xmas_exists = find_word(&crossword, (y, x), dir, XMAS[1..=3].into());
                    if xmas_exists {
                        word_count += 1;
                    }
                } else if curr_letter == SAMX[0] {
                    let samx_exists: bool = find_word(&crossword, (y, x), dir, SAMX[1..=3].into());
                    if samx_exists {
                        word_count += 1;
                    }
                }
            }
        }
    }
    word_count
}
/*
 *The Elf looks quizzically at you. Did you misunderstand the assignment?

Looking for the instructions, you flip over the word search to find that this isn't actually an XMAS puzzle; it's an X-MAS puzzle in which you're supposed to find two MAS in the shape of an X. One way to achieve that is like this:

M.S
.A.
M.S

Irrelevant characters have again been replaced with . in the above diagram. Within the X, each MAS can be written forwards or backwards.

Here's the same example from before, but this time all of the X-MASes have been kept instead:

.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........

In this example, an X-MAS appears 9 times.

Flip the word search from the instructions back over to the word search side and try again. How many times does an X-MAS appear? */
const MAS: [char; 3] = ['M', 'A', 'S'];
const SAM: [char; 3] = ['S', 'A', 'M'];
fn find_diag(crossword: &Vec<Vec<char>>, (y, x): (usize, usize), dir: (isize, isize)) -> bool {
    if x >= crossword[y].len() {
        return false;
    }
    let curr_letter = crossword[y][x];

    if curr_letter == MAS[0] {
        return find_word(&crossword, (y, x), dir, MAS[1..=2].into());
    } else if curr_letter == SAM[0] {
        return find_word(&crossword, (y, x), dir, SAM[1..=2].into());
    }
    return false;
}
pub fn find_x_mas() -> i64 {
    let crossword: Vec<Vec<char>> = read_input("./inputs/day4.txt")
        .iter()
        .map(|row| row.chars().collect())
        .collect();
    let mut word_count = 0;
    for y in 0..crossword.len() {
        for x in 0..crossword[y].len() {
            let left_dir = (1, 1);
            let left_diag = find_diag(&crossword, (y, x), left_dir);
            let right_dir = (1, -1);
            let right_diag = find_diag(&crossword, (y, x + 2), right_dir);
            if left_diag && right_diag {
                word_count += 1;
            }
        }
    }
    word_count
}
