use std::vec::Vec;

fn main() {
    let n_players = 426;
    let last_marble_value = 72058;
    let mut marbles: Vec<u32> = vec![0];
    let mut current_ix = 0;
    let mut scores = vec![0; n_players];
    let mut player_ix = 0;
    for m_no in 1..(last_marble_value + 1) {
        player_ix = (player_ix + 1) % n_players;
        if m_no % 23 == 0 {
            scores[player_ix] += m_no;
            while current_ix < 7 {
                current_ix += marbles.len();
            }
            current_ix -= 7;
            let removed = marbles.remove(current_ix);
            scores[player_ix] += removed;
        } else {
            current_ix = (current_ix + 2) % marbles.len();
            marbles.insert(current_ix, m_no);
        }
    }
    let result_1 = scores.iter().max().unwrap();
    println!("Answer 1: {}", result_1);
}
