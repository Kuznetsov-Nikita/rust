#![forbid(unsafe_code)]

pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut bytes_strs = vec![];

    for str in &strs {
        bytes_strs.push(str.as_bytes());
    }

    let mut bytes_lcp = vec![];
    let mut index = 0;

    'outer: loop {
        let mut byte = None;

        for bytes_str in &bytes_strs {
            if index == bytes_str.len() {
                break 'outer;
            }

            match byte {
                None => byte = Some(bytes_str[index]),
                Some(byte) if byte != bytes_str[index] => break 'outer,
                _ => continue,
            }
        }
        if let Some(byte) = byte {
            bytes_lcp.push(byte);
        }

        index += 1;
    }

    let mut iter = strs[0].char_indices();
    let mut prev_length = 0;
    let mut length = 0;

    while length <= bytes_lcp.len() {
        prev_length = length;

        let char = iter.next();
        if char.is_none() {
            prev_length = bytes_lcp.len();
            break;
        }

        length = char.unwrap().0;
    }

    strs[0][0..prev_length].to_string()
}
