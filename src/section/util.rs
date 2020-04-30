use crate::*;

pub fn build_string_table(names: Vec<&str>) -> Vec<u8> {
    // ELFの文字列テーブルは null-byte + (name + null-byte) * n という形状に
    // それに合うようにバイト列を構築.
    let mut string_table: Vec<u8> = vec![0x00];

    for name in names {
        for byte in name.as_bytes() {
            string_table.push(*byte);
        }
        string_table.push(0x00);
    }

    // アラインメントの調整
    let md = string_table.len() % 4;
    for _ in 0..(4 - md) {
        string_table.push(0x00);
    }

    string_table
}
