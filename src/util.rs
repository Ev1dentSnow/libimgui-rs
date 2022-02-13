// For more on boxels: http://www.xsquawkbox.net/xpsdk/mediawiki/ScreenCoordinates
// https://developer.x-plane.com/sdk/XPLMCreateWindow_t/

use xplm::window::Window;

pub fn translate_imgui_to_boxels(inX: i32, inY: i32, xp_window_left: i32, xp_window_top: i32) -> (i32, i32) {
    let outX = xp_window_left - inX;
    let outY = xp_window_top - inY;
    (outX, outY)
}

pub fn translate_boxels_to_native(inX: u16, inY: u16) -> (u16, u16) {
    (0, 0)
}
