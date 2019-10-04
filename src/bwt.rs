use std::collections::HashMap;

pub struct BWTString {
    pub transform: String,
    c_table: HashMap<char, u64>,
    amount_of: HashMap<char, u64>,
    last_char_pos: u64
}

pub fn make_bwt(from: String) -> BWTString {
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

    BWTString {
        transform: bwt_chars,
        c_table,
        amount_of,
        last_char_pos
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
