use super::Solution;

impl Solution {
    /// # [289. Game of Life](https://leetcode.com/problems/game-of-life/description/?envType=study-plan-v2&envId=top-interview-150)
    ///
    /// According to<a href="https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life" target="_blank">Wikipedia's article</a>: "The <b>Game of Life</b>, also known simply as <b>Life</b>, is a cellular automaton devised by the British mathematician John Horton Conway in 1970."
    ///
    /// The board is made up of an `m x n` grid of cells, where each cell has an initial state: <b>live</b> (represented by a `1`) or <b>dead</b> (represented by a `0`). Each cell interacts with its <a href="https://en.wikipedia.org/wiki/Moore_neighborhood" target="_blank">eight neighbors</a> (horizontal, vertical, diagonal) using the following four rules (taken from the above Wikipedia article):
    ///
    /// - Any live cell with fewer than two live neighbors dies as if caused by under-population.
    /// - Any live cell with two or three live neighbors lives on to the next generation.
    /// - Any live cell with more than three live neighbors dies, as if by over-population.
    /// - Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
    ///
    /// The next state is created by applying the above rules simultaneously to every cell in the current state, where births and deaths occur simultaneously. Given the current state of the `m x n` grid `board`, return the next state.
    ///
    /// **Example 1:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/12/26/grid1.jpg" style="width: 562px; height: 322px;">
    ///
    /// ```txt
    /// Input: board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]
    /// Output: [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]
    /// ```
    ///
    /// **Example 2:**
    /// <img alt="" src="https://assets.leetcode.com/uploads/2020/12/26/grid2.jpg" style="width: 402px; height: 162px;">
    ///
    /// ```txt
    /// Input: board = [[1,1],[1,0]]
    /// Output: [[1,1],[1,1]]
    /// ```
    ///
    /// **Constraints:**
    ///
    /// - `m == board.length`
    /// - `n == board[i].length`
    /// - `1 <= m, n <= 25`
    /// - `board[i][j]` is `0` or `1`.
    ///
    /// **Follow up:**
    ///
    /// - Could you solve it in-place? Remember that the board needs to be updated simultaneously: You cannot update some cells first and then use their updated values to update other cells.
    /// - In this question, we represent the board using a 2D array. In principle, the board is infinite, which would cause problems when the active area encroaches upon the border of the array (i.e., live cells reach the border). How would you address these problems?
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        for i in 0..m {
            for j in 0..n {
                let v = board[i as usize][j as usize];
                match v {
                    0 => {
                        let mut count = 0;
                        for k in i - 1..=i + 1 {
                            for l in j - 1..=j + 1 {
                                if k < 0 || l < 0 || k >= m || l >= n {
                                    continue;
                                }
                                if board[k as usize][l as usize] >= 1 {
                                    count += 1;
                                }
                            }
                        }
                        if count == 3 {
                            board[i as usize][j as usize] = -1;
                        }
                    }
                    1 => {
                        let mut count = 0;
                        for k in i - 1..=i + 1 {
                            for l in j - 1..=j + 1 {
                                if k < 0 || l < 0 || k >= m || l >= n {
                                    continue;
                                }
                                if board[k as usize][l as usize] >= 1 {
                                    count += 1;
                                }
                            }
                        }
                        count -= 1;
                        if count != 2 && count != 3 {
                            board[i as usize][j as usize] = 2;
                        }
                    }
                    _ => {}
                }
            }
        }
        for row in board {
            row.iter_mut().for_each(|x| {
                if *x == 2 {
                    *x = 0;
                } else if *x == -1 {
                    *x = 1;
                }
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        Solution::game_of_life(&mut board);
        assert_eq!(
            board,
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]]
        );
    }

    #[test]
    fn example2() {
        let mut board = vec![vec![1, 1], vec![1, 0]];
        Solution::game_of_life(&mut board);
        assert_eq!(board, vec![vec![1, 1], vec![1, 1]]);
    }
}
