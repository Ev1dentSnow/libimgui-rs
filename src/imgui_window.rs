use imgui::DrawData;
use xplm;
use util;

struct Vertex {
    position: [f32; 2],
}

struct ImguiWindow {
    left: u16,
    top: u16,
    right: u16,
    bottom: u16,
    xplm_graphics_state: xplm::draw::GraphicsState,
}

impl ImguiWindow {
    fn new(left: u16, top: u16, right: u16, bottom: u16, xplm_graphics_state: xplm::draw::GraphicsState, ) -> ImguiWindow {
        ImguiWindow {
            left,
            top,
            right,
            bottom,
            xplm_graphics_state,
        }
    }

    fn render_ui(&mut self, imgui_draw_data: DrawData) {
        self.xplm_graphics_state.fog = false;
        self.xplm_graphics_state.textures = 1;

        self.xplm_graphics_state.lighting = false;
        self.xplm_graphics_state.alpha_testing = true;
        self.xplm_graphics_state.alpha_blending = true;
        self.xplm_graphics_state.depth_testing = false;
        self.xplm_graphics_state.depth_writing = false;

        // Render command lists
        for (i, list) in imgui_draw_data.draw_lists().enumerate() {
            let vtx_buffer = list.vtx_buffer();
            let idx_buffer = list.idx_buffer();

            let boxel = crate::util::translate_imgui_to_boxels();

        }
    }
}
