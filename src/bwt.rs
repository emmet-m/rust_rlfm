use std::collections::HashMap;

extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub struct BWT {
    // The BWT transform of a given input string
    transform: String,
    // the C table of our transform. given bb = `sort(transform)`, 
    // the table maps a char to it's index in bb
    c_table: HashMap<char, u64>,
    // The maps a char to the number of times it occurs in `transform`
    amount_of: HashMap<char, u64>,
    // The position of the last char of the original string in `transform`
    last_char_pos: u64
}

pub fn make_bwt(from: String) -> BWT {
    let mut suffixes: Vec<String> =  vec![String::new(); from.len()];
    let mut rot_str: String = from.clone();

    // make a matrix of rotated strings
    for i in 0..from.len() {
        suffixes[i] = rot_str.clone();
        let prefix_char: String = rot_str.chars().nth(0).unwrap().to_string();
        let suffix_str: String  = rot_str[1..from.len()].to_string();
        rot_str = suffix_str + &prefix_char;
    }

    // sort it
    suffixes.sort();

    // Take the last column of the matrix
    let mut bwt_chars: String = String::new();
    for i in 0..suffixes.len() {
        let ch = suffixes[i].chars().nth(from.len() - 1).unwrap();
        bwt_chars.push(ch);
    }

    // Sort it to get all characters in order
    let mut bwt_sorted: Vec<char> = (&bwt_chars[..]).chars().collect();
    bwt_sorted.sort_by(|a, b| a.cmp(b));

    // Record the starting position of each charater
    let mut c_table = HashMap::new();
    let mut amount_of = HashMap::new();
    for i in 0..bwt_sorted.len() {
        c_table.entry(bwt_sorted[i]).or_insert(i as u64);
        match amount_of.get(&bwt_sorted[i]) {
            Some(&x) => amount_of.insert(bwt_sorted[i], x + 1),
            _ => amount_of.insert(bwt_sorted[i], 1)
        };
    }

    // Get the position of the last character in the original string
    let mut last_char_pos = 0;
    for i in 0..suffixes.len() {
        if suffixes[i] == from {
            last_char_pos = i as u64;
            break;
        }
    }

    BWT {
        transform: bwt_chars,
        c_table,
        amount_of,
        last_char_pos
    }
}

struct SearchState {
    // The current char we are searching for
    curr_char: char,
    // The lowest index we have searched for
    lo: u64,
    // The highest index we have searched for
    hi: u64,
    // The amount of `curr_char` before index lo 
    occ_lo: u64,
    // The amount of `curr_char` after index lo 
    occ_hi: u64,
    // Did the search fail?
    did_fail: bool
}

impl BWT {

    /**
     * Computes the rank of a given character before an index.
     * The rank of a character on an index is the amount of times
     *  that character occurs before the given index.
     */
    fn _rank(&self, c: char, ind: u64) -> u64 {
        let mut count = 0;
        for i in self.transform.chars() {
            if c == i {
                count += 1;
            }
        }

        count
    }

    /** 
     * Finds the amount of characters `goal` that occurs before state.curr_char,
     * and updates the state accordingly 
     */
    fn _reverse_search(&self, state: &SearchState, goal: char) -> SearchState {
        let mut new_state = SearchState {
            curr_char: goal,
            lo: 0,
            hi: 0,
            occ_lo: 0,
            occ_hi: 0,
            did_fail: false
        };

        // Check if character exists in our transform somewhere
        if self.amount_of.get(&goal).is_none() {
            new_state.did_fail = true;
            return new_state;
        }

        // Update the lo and hi occurences
        new_state.occ_lo = self._rank(goal, state.lo);
        new_state.occ_hi = self._rank(goal, state.hi);

        // Update lo and hi according to matches
        new_state.lo = self.c_table[&goal] + new_state.occ_lo;
        new_state.hi = self.c_table[&goal] + new_state.occ_hi;

        if new_state.lo >= new_state.hi {
            new_state.did_fail = true;
        }

        new_state
    }
    
    pub fn find_num_occurences(&self, pattern: String) -> u64 {
        // The empty string matches all the gaps between characters,
        // including before the first character and after the last character
        if pattern.len() == 0 {
            self.transform.len() + 1;
        }
        
        let rev_pattern = pattern.graphemes(true).rev().collect::<String>();
        let mut it = rev_pattern.chars().peekable();
        let mut c: char = it.next().unwrap();
        let mut state = SearchState {
            curr_char: c,
            lo: self.c_table[&c],
            hi: self.c_table[&c] + self.amount_of[&c],
            occ_lo: 0, // will be set in the next search
            occ_hi: 0,
            did_fail: false
        };

        while it.peek().is_some() {
            c = it.next().unwrap();
            state = self._reverse_search(&state, c);
            if state.did_fail {
                return 0;
            }
        }

        state.hi - state.lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bwt_banana() {
        let ban_bwt = make_bwt("banana".to_string());

        assert_eq!("nnbaaa", ban_bwt.transform);

        assert_eq!(3, ban_bwt.last_char_pos);

        assert_eq!(Some(&(0 as u64)), ban_bwt.c_table.get(&'a'));
        assert_eq!(Some(&(3 as u64)), ban_bwt.c_table.get(&'b'));
        assert_eq!(Some(&(4 as u64)), ban_bwt.c_table.get(&'n'));

        assert_eq!(Some(&(3 as u64)), ban_bwt.amount_of.get(&'a'));
        assert_eq!(Some(&(1 as u64)), ban_bwt.amount_of.get(&'b'));
        assert_eq!(Some(&(2 as u64)), ban_bwt.amount_of.get(&'n'));
   }

    #[test]
    fn test_bwt_missisippi() {
        let mis_bwt = make_bwt("mississippi".to_string());
        assert_eq!("pssmipissii", mis_bwt.transform);

        assert_eq!(4, mis_bwt.last_char_pos);

        assert_eq!(Some(&(0 as u64)), mis_bwt.c_table.get(&'i'));
        assert_eq!(Some(&(5 as u64)), mis_bwt.c_table.get(&'p'));

        assert_eq!(Some(&(4 as u64)), mis_bwt.amount_of.get(&'s'));
        assert_eq!(Some(&(1 as u64)), mis_bwt.amount_of.get(&'m'));
    }
}
