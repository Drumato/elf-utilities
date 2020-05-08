fn symbol_info_from_bind(bind: u8) -> u8 {
    bind << 4
}

pub fn symbol_info(bind: u8, sym_type: u8) -> u8 {
    symbol_info_from_bind(bind) | sym_type
}
