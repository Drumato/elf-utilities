#[allow(clippy::same_item_push)]
/// STRTABセクション等で使われる文字列テーブル形式を生成します．
/// 具体的には，null-byte (name + null-byte)* という形式を生成します．
/// alignmentがtrueの場合，4バイトアラインメントも行います．
///
/// # Examples
///
/// ```rust
/// use elf_utilities::section;
/// let table = section::build_string_table(vec![".text", ".symtab"], false);
///
/// assert_eq!(vec![0x00, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x00, 0x2e, 0x73, 0x79, 0x6d, 0x74, 0x61, 0x62, 0x00], table);
/// ```
pub fn build_string_table(strings: Vec<&str>, alignment: bool) -> Vec<u8> {
    // ELFの文字列テーブルは null-byte + (name + null-byte) * n という形状に
    // それに合うようにバイト列を構築.
    let mut string_table: Vec<u8> = vec![0x00];

    for st in strings {
        for byte in st.as_bytes() {
            string_table.push(*byte);
        }
        string_table.push(0x00);
    }

    // アラインメントの調整
    if alignment {
        let md = string_table.len() % 4;
        for _ in 0..(4 - md) {
            string_table.push(0x00);
        }
    }

    string_table
}

/// STRTABセクション等で使われる文字列テーブル形式を生成します．
/// 具体的には，null-byte (name + null-byte)* という形式を生成します．
/// alignmentがtrueの場合，4バイトアラインメントも行います．
///
/// # Examples
///
/// ```rust
/// use elf_utilities::section;
/// let table = section::build_byte_string_table(vec![".text".as_bytes().to_vec(), ".symtab".as_bytes().to_vec()], false);
///
/// assert_eq!(vec![0x00, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x00, 0x2e, 0x73, 0x79, 0x6d, 0x74, 0x61, 0x62, 0x00], table);
/// ```
#[allow(clippy::same_item_push)]
pub fn build_byte_string_table(strings: Vec<Vec<u8>>, alignment: bool) -> Vec<u8> {
    // ELFの文字列テーブルは null-byte + (name + null-byte) * n という形状に
    // それに合うようにバイト列を構築.
    let mut string_table: Vec<u8> = vec![0x00];

    for st in strings {
        for byte in st.iter() {
            string_table.push(*byte);
        }
        string_table.push(0x00);
    }

    // アラインメントの調整
    if alignment {
        let md = string_table.len() % 4;
        for _ in 0..(4 - md) {
            string_table.push(0x00);
        }
    }

    string_table
}

#[cfg(test)]
mod section_util_tests {
    use super::*;

    #[test]
    fn build_string_table_test() {
        let table = build_string_table(vec!["foo", "bar", "baz"], true);
        assert_eq!(
            vec![
                0x00, 0x66, 0x6f, 0x6f, 0x00, 0x62, 0x61, 0x72, 0x00, 0x62, 0x61, 0x7a, 0x00, 0x00,
                0x00, 0x00
            ],
            table
        );
    }

    #[test]
    fn build_byte_string_table_test() {
        let table = vec![
            "foo".as_bytes().to_vec(),
            "bar".as_bytes().to_vec(),
            "baz".as_bytes().to_vec(),
        ];
        let table = build_byte_string_table(table, true);
        assert_eq!(
            vec![
                0x00, 0x66, 0x6f, 0x6f, 0x00, 0x62, 0x61, 0x72, 0x00, 0x62, 0x61, 0x7a, 0x00, 0x00,
                0x00, 0x00
            ],
            table
        );
    }
}
