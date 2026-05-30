// ANSI 16-color escape sequences — palette-aware (terminal theme defines the actual colors)
pub const GRN: &str = "\x1b[32m"; // color 2
pub const BLU: &str = "\x1b[34m"; // color 4
pub const SKY: &str = "\x1b[36m"; // color 6
pub const MVE: &str = "\x1b[35m"; // color 5
pub const YEL: &str = "\x1b[33m"; // color 3
pub const PCH: &str = "\x1b[93m"; // color 11 (bright yellow — closest to orange/peach in ANSI16)
pub const DIM: &str = "\x1b[2m"; // SGR dim
pub const SUB: &str = "\x1b[37m"; // color 7
pub const RST: &str = "\x1b[0m";
pub const BLD: &str = "\x1b[1m";
