/// Shift st_bind value for constructing st_info.
pub fn setup_symbol_bind(bind: u8) -> u8 {
    bind << 4
}

/// Construct an symbol's information from st_bind and st_type.
pub fn symbol_info(bind: u8, sym_type: u8) -> u8 {
    setup_symbol_bind(bind) | sym_type
}

pub fn symbol_visibility(other: u8) -> u8 {
    other & 0x03
}