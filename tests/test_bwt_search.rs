use rlfm::bwt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_search() {
        let x = bwt::make_bwt("simple test with some words".to_owned());

        assert_eq!(4, x.find_num_occurences("s".to_owned()));
    }

}