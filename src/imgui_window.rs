use imgui::{Context, DrawData, Window, WindowFlags, WindowToken};
use xplm;

struct Vertex {
    position: [f32; 2],
}

struct ImguiWindow {
    left: u16,
    top: u16,
    right: u16,
    bottom: u16,
    xplm_graphics_state: xplm::draw::GraphicsState,
    window_title: String,
}

impl ImguiWindow {
    fn new(
        left: u16,
        top: u16,
        right: u16,
        bottom: u16,
        xplm_graphics_state: xplm::draw::GraphicsState,
        window_title: String,
    ) -> ImguiWindow {
        ImguiWindow {
            left,
            top,
            right,
            bottom,
            xplm_graphics_state,
            window_title,
        }
    }

    /**
     * 1) Transfer XP window geometry to ImGui
     * 2) Construct the window
     * 3) Handle window focus
     */

    fn update_ui(&self, ctx: &mut imgui::Context, xplane_window: &xplm::window::Window) {
        //returns tuple of (left, top, right, bottom)
        let geometry = xplane_window.geometry().into_left_top_bottom_right();
        let window_width = geometry.2 - geometry.0;
        let window_height = geometry.1 - geometry.3;

        let ui = ctx.frame();

        let window = Window::new(String::from(&self.window_title));
        window.title_bar(false);
        window.resizable(false);
        window.collapsible(false);
        let token = window.begin(&ui).unwrap();

        {
            //build interface here
        }

        token.end();

        //TODO: handle window focus
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
        }
    }
}
