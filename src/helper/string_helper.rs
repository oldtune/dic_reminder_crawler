pub fn join_split<'a, P>(some_iter: P, separator: char) -> String
where
    P: Iterator<Item = &'a str>,
{
    let mut result = "".to_string();

    for string in some_iter {
        result.push('+');
        result.push_str(string);
    }

    result
}
