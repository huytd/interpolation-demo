pub const WIDTH: usize = 280;
pub const HEIGHT: usize = 150;
pub const fn FROM_RGB(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
pub const FG_PIXEL: u32 = FROM_RGB(117, 62, 0);
pub const RED_PIXEL: u32 = FROM_RGB(255, 100, 120);
pub const TRACE_PIXEL: u32 = FROM_RGB(183, 183, 147);
pub const BG_PIXEL: u32 = FROM_RGB(245, 245, 240);
