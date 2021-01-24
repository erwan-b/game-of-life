use ggez::event::{ MouseButton };
use ggez::graphics;
use ggez::Context;

use gfx_core::{handle::RenderTargetView, memory::Typed};
use gfx_device_gl;

use imgui::*;
use imgui_gfx_renderer::*;

use std::time::{Instant, Duration};
use std::ops::RangeInclusive;

/// Describe the state of the mouse on a frame
#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct MouseState {
    pos: (i32, i32),
    /// mouse buttons: (left, right, middle)
    pressed: (bool, bool, bool),
    wheel: f32,
    wheel_h: f32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum UiButton {
    NEXT,
    PREV,
    STOP,
    PLAY,
}

pub struct ImGuiWrapper {
    pub imgui: imgui::Context,
    pub renderer: Renderer<gfx_core::format::Rgba8, gfx_device_gl::Resources>,

    last_frame: Instant,
    mouse_state: MouseState,
    last_button: Option<UiButton>,
    time_per_step: Duration
}


/// This ImGuiWrapper will be the class that encapsulates all
/// imgui functionality.
impl ImGuiWrapper {
    /// This is going to take a ggez context and give us
    /// back a fresh instance of the wrapper.
    pub fn new(ctx: &mut Context) -> Self {
        let mut imgui = imgui::Context::create();
        let (factory, _, _, _, _) = graphics::gfx_objects(ctx);
        let renderer = Renderer::init(&mut imgui, &mut *factory, Shaders::GlSl400).unwrap();


        Self {
            imgui,
            renderer,
            last_frame: Instant::now(),
            mouse_state: MouseState::default(),
            last_button: None,
            time_per_step: Duration::new(1, 0)
        }
    }

    // This is what we will call on every render iteration
    // to render the imgui bits on top of our game.
    pub fn render(&mut self, ctx: &mut Context,  hidpi_factor: f32, play: bool) {
        // Update mouse
        self.update_mouse();

        // Create new frame
        let now = Instant::now();
        self.last_frame = now;
        self.last_button = None;

        let (draw_width, draw_height) = graphics::drawable_size(ctx);
        self.imgui.io_mut().display_size = [draw_width, draw_height];
        self.imgui.io_mut().display_framebuffer_scale = [hidpi_factor, hidpi_factor];

        let ui = self.imgui.frame();
        {
            let mut slider = self.time_per_step.as_millis() as u64;
            let s = Slider::new(
                im_str!("step time, in millisecond"),
                RangeInclusive::new(0, 2000)
            );

            let mut click_button = None;
            let (w, h) = graphics::size(ctx);

            // Window
            Window::new(im_str!("Hello world"))
                .menu_bar(false).title_bar(false).movable(false)
                .resizable(false).size([w, 100.0], Condition::Always)
                .position([0.0, h - 100.0], Condition::Always)
                .build(&ui, || {
                    s.build(&ui, &mut slider);
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    if ui.button(im_str!("|<"),  [20.0, 20.0]) {
                        click_button = Some(UiButton::PREV);
                    }
                    ui.same_line(32.0);
                    if !play && ui.button(im_str!("|>"),  [20.0, 20.0]) {
                        click_button = Some(UiButton::PLAY);
                    }
                    if play && ui.button(im_str!("||"),  [20.0, 20.0]) {
                        click_button = Some(UiButton::STOP);
                    }
                    ui.same_line(54.0);
                    if ui.button(im_str!(">|"),  [20.0, 20.0]) {
                        click_button = Some(UiButton::NEXT);
                    }
                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });
            self.last_button = click_button;
            self.time_per_step = Duration::from_millis(slider as u64)
        }

        // Render
        let (factory, _, encoder, _, render_target) = graphics::gfx_objects(ctx);
        let draw_data = ui.render();
        self.renderer.render(&mut *factory, encoder,
                &mut RenderTargetView::new(render_target.clone()),
                draw_data).unwrap();

    }

    pub fn get_time_per_step(&self) -> Duration {
        self.time_per_step
    }

    pub fn get_last_button(&self) -> Option<UiButton> {
       self.last_button
    }

    fn update_mouse(&mut self) {
        self.imgui.io_mut().mouse_pos = [self.mouse_state.pos.0 as f32, self.mouse_state.pos.1 as f32];

        self.imgui.io_mut().mouse_down = [
            self.mouse_state.pressed.0,
            self.mouse_state.pressed.1,
            self.mouse_state.pressed.2,
            false,
            false,
        ];

        self.imgui.io_mut().mouse_wheel = self.mouse_state.wheel;
        self.mouse_state.wheel = 0.0;

        self.imgui.io_mut().mouse_wheel_h = self.mouse_state.wheel_h;
        self.mouse_state.wheel_h = 0.0;
    }

    pub fn update_mouse_pos(&mut self, x: f32, y: f32) {
        self.mouse_state.pos = (x as i32, y as i32);
    }

    pub fn update_mouse_up(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.mouse_state.pressed.0 = false,
            MouseButton::Right => self.mouse_state.pressed.1 = false,
            MouseButton::Middle => self.mouse_state.pressed.2 = false,
            _ => ()
        }
    }

    pub fn update_mouse_down(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => self.mouse_state.pressed.0 = true,
            MouseButton::Right => self.mouse_state.pressed.1 = true,
            MouseButton::Middle => self.mouse_state.pressed.2 = true,
            _ => ()
        }
    }

    pub fn update_text(&mut self, val: char) {
        self.imgui.io_mut().add_input_character(val);
    }

    pub fn update_scroll(&mut self, x: f32, y: f32) {
        self.mouse_state.wheel += y;
        self.mouse_state.wheel_h += x;
    }
}