use std::collections::HashMap;

fn get_alphabet() -> HashMap<u8, &'static str> {
    HashMap::from([
        (0, "A"),
        (1, "B"),
        (2, "C"),
        (3, "D"),
        (4, "E"),
        (5, "F"),
        (6, "G"),
        (7, "H"),
        (8, "I"),
        (9, "J"),
        (10, "K"),
        (11, "L"),
        (12, "M"),
        (13, "N"),
        (14, "O"),
        (15, "P"),
        (16, "Q"),
        (17, "R"),
        (18, "S"),
        (19, "T"),
        (20, "U"),
        (21, "V"),
        (22, "W"),
        (23, "X"),
        (24, "Y"),
        (25, "Z"),
        (26, "a"),
        (27, "b"),
        (28, "c"),
        (29, "d"),
        (30, "e"),
        (31, "f"),
        (32, "g"),
        (33, "h"),
        (34, "i"),
        (35, "j"),
        (36, "k"),
        (37, "l"),
        (38, "m"),
        (39, "n"),
        (40, "o"),
        (41, "p"),
        (42, "q"),
        (43, "r"),
        (44, "s"),
        (45, "t"),
        (46, "u"),
        (47, "v"),
        (48, "w"),
        (49, "x"),
        (50, "y"),
        (51, "z"),
        (52, "0"),
        (53, "1"),
        (54, "2"),
        (55, "3"),
        (56, "4"),
        (57, "5"),
        (58, "6"),
        (59, "7"),
        (60, "8"),
        (61, "9"),
        (62, "+"),
        (63, "/"),
    ])
}

fn main() {
    let bytes = "stacippalippalunga".as_bytes();
}

fn base64_encode(input: &str) -> String {
    let bytes = &input.as_bytes();
    let mut result = String::new();
    let mut offset = 0;

    while offset < bytes.len() {
        let chunk = if offset + 3 > bytes.len() {
            &bytes[offset..bytes.len()]
        } else {
            &bytes[offset..offset + 3]
        };

        let chunk_str = match chunk.len() {
            0 => "0".to_string(),
            1 => "1".to_string(),
            2 => "2".to_string(),
            _ => decode_triplet(chunk),
        };

        result.push_str(chunk_str.as_ref());

        offset += 3;
    }

    result
}

fn decode_triplet(bytes: &[u8]) -> String {
    let alphabet = get_alphabet();
    println!("{:08b} {:08b} {:08b}", bytes[0], bytes[1], bytes[2]);

    let first_byte = bytes[0];
    let first_sextet = first_byte >> 2;
    let first_remainder = first_byte & 0b00000011;

    let second_byte = bytes[1];
    let second_sextet = (second_byte >> 4) | (first_remainder << 4);
    let second_remainder = second_byte & 0b00001111;

    let third_byte = bytes[2];
    let third_sextet = (third_byte >> 6) | (second_remainder << 2);
    let fourth_sextet = third_byte & 0b00111111;

    format!(
        "{}{}{}{}",
        alphabet.get(&first_sextet).unwrap(),
        alphabet.get(&second_sextet).unwrap(),
        alphabet.get(&third_sextet).unwrap(),
        alphabet.get(&fourth_sextet).unwrap()
    )
}

#[cfg(test)]
mod test {
    use crate::base64_encode;

    #[test]
    fn base_string_encode() {
        let plain = "hello";
        let expected = "aGVsbG8=".to_string();
        let encoded = base64_encode(plain);

        assert_eq!(encoded, expected);
    }
}
