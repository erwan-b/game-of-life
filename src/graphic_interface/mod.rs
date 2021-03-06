use std::time;
mod constants;
mod camera;
mod im_gui_wrapper;

use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};
use ggez::event::{EventHandler};
use ggez::event::MouseButton;

use crate::board::Board;
use constants::Constants;
use camera::Camera;
use im_gui_wrapper::ImGuiWrapper;
use std::time::Duration;
use crate::graphic_interface::im_gui_wrapper::UiButton;
use crate::create_file_from_map;

/// `MyGame` describe the game graphic_interface logic
/// It contain:
/// <p> - some static mesh  </p>
/// <p> - some static mesh  </p>
/// <p> - data about the refresh rate and games constants  </p>
pub struct MyGame {
    board: Box<Board>,
    camera: Camera,
    constants: Constants,

    img_wrapper: ImGuiWrapper,
    cell_mesh: graphics::Mesh,
    line_h: graphics::Mesh,
    line_w: graphics::Mesh,

    is_clicking: bool,
    as_move: bool,
    last_refresh : time::Instant,
    game_step: i64,
    play: bool
}

/// The impl is here to define our graphic_interface logic called by the `EventHandler`
/// It important to kipp it split from the the rest
impl MyGame {
    pub fn create_line_mesh(ctx: &mut Context, w: f32, h: f32) -> (graphics::Mesh, graphics::Mesh) {
        let color = [0.3, 0.3, 0.3, 1.0].into();


        let line_h = graphics::Mesh::new_line(
            ctx, &[ Point2{ x: 0.0, y: 0.0 as f32}, Point2{ x: w, y: 0.0 } ],
            1.0,
            color,
        ).unwrap();

        let line_w = graphics::Mesh::new_line(
            ctx, &[ Point2{x: 0.0 as f32, y: 0.0}, Point2{x: 0.0, y: h} ],
            1.0,
            color,
        ).unwrap();
        (line_h, line_w)
    }

    fn create_cell_mesh(ctx: &mut Context, camera: &Camera) -> graphics::Mesh {
        let cell_mesh_rect = graphics::Rect::new(0.0, 0.0, camera.get_cell_size(), camera.get_cell_size());

        match graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            cell_mesh_rect,
            graphics::Color::from_rgb(255, 51, 255)
        ) {
            Ok(mesh) => mesh,
            Err(_e) => panic!("Could not create cell_mesh")
        }
    }

    pub fn new(ctx: &mut Context, board: Box<Board>) -> Self {
        let (w, h) = graphics::size(ctx);
        let (board_h, board_w) = board.get_size();
        let (line_h, line_w) = MyGame::create_line_mesh(ctx, w, h);

        let camera = Camera::new(
            Point2{x: (board_w / 2 - 10) as f32, y: (board_h / 2 - 10) as f32},
            Point2{x: w , y: h});
        let img = ImGuiWrapper::new(ctx);

        MyGame {
            board,
            constants: Constants::new(Duration::new(1, 0)),
            cell_mesh: Self::create_cell_mesh(ctx, &camera),
            img_wrapper: img,
            line_h,
            line_w,
            camera,
            is_clicking: false,
            as_move: false,
            play: false,
            game_step: 0,
            last_refresh: time::Instant::now()
        }
    }

    fn save_map(&self) {
        create_file_from_map(&self.board.board_to_string(), "");
    }

    fn save_init_map(&self) {
        create_file_from_map(&self.board.initial_board_to_string(), "");
    }

    fn prev(&mut self) {
        if self.constants.turns > 0 {
            self.board.prev();
            self.constants.turns -= 1;
        }
    }

    fn next(&mut self) {
        self.board.next();
        self.constants.turns += 1;
    }

    /// Draw each line limitation of the board
    fn draw_line(&self, ctx: &mut Context) -> GameResult<()> {
        self.camera.line_to_show().iter()
            .fold(Ok(()), | _acc, pixel|{
                graphics::draw(ctx, &self.line_w, ( pixel.screen_pos, ))?;
                graphics::draw(ctx, &self.line_h, ( pixel.screen_pos, ))
            })
    }

    /// Draw the living cells on the board
    fn draw_board(&self, ctx: &mut Context) -> GameResult<()> {
        self.camera.active_cells_to_show(self.board.get_leaving_cells()).iter()
            .fold(Ok(()), | _acc, pixel|
                graphics::draw(ctx, &self.cell_mesh,
                    graphics::DrawParam::default().dest(pixel.screen_pos))
            )
    }

    fn update_button(&mut self) {
        match self.img_wrapper.get_last_button() {
            Some(UiButton::Next) => { self.game_step += 1;}
            Some(UiButton::Prev) => { self.game_step -= 1;}
            Some(UiButton::Stop) => { self.play = false; }
            Some(UiButton::Play) => { self.play = true; }
            Some(UiButton::SaveMap) => { self.save_map(); }
            Some(UiButton::SaveInitMap) => { self.save_init_map(); }
            _ => {}
        }
        self.constants.refresh_rate = self.img_wrapper.get_time_per_step();
    }
}

/// Define the `EventHandler` to mange the ggez lib events
impl EventHandler for MyGame {
    /// Update the cells there.
    /// There for we call the board function that return a new one with the rules applied on all cells.
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.update_button();

        let duration = time::Instant::now() - self.last_refresh;
        if duration > self.constants.refresh_rate && self.play {
            self.game_step += 1;
        }

        if self.game_step > 0 {
            self.last_refresh = time::Instant::now();
            self.next();
            self.game_step -= 1;
        } else if self.game_step < 0 {
            self.prev();
            self.game_step += 1;
        }
        Ok(())
    }

    /// Draw the board on the screen.
    /// The defined size of the cells we be translate to showed size with de zoom ratio
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        if self.camera.get_zoom_ratio() != self.img_wrapper.get_zoom_ratio() {
            self.camera.set_zoom_ratio(self.img_wrapper.get_zoom_ratio());
            self.cell_mesh = Self::create_cell_mesh(ctx, &self.camera);
        }
        self.draw_board(ctx)?;
        self.draw_line(ctx)?;
        self.img_wrapper.render(ctx, 2.0, self.play);

        graphics::present(ctx)
    }

    /// We need to track the mouse event to set a cell alive if the mouse is click on a valid cell.
    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.img_wrapper.update_mouse_pos(x, y);
        self.img_wrapper.update_mouse_down(button);

        let (_w, h) = graphics::size(ctx);
        if y <= h - 100.0 {
                self.is_clicking = true;
        }
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.img_wrapper.update_mouse_pos(x, y);
        self.img_wrapper.update_mouse_up(button);
        let (_w, h) = graphics::size(ctx);

        if y <= h - 100.0 && !self.as_move {
            let  (w, h) = self.camera.board_pos_from_screen_pos((x, y));

            self.board.inverse_cell(w as i32, h as i32);
        }
        self.as_move = false;
        self.is_clicking = false;
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
        self.img_wrapper.update_mouse_pos(x, y);
        self.as_move = true;
        let (_w, h) = graphics::size(ctx);

        if self.is_clicking && y <= h - 100.0 {
            self.camera.move_pos(Point2{x: dx, y: dy});
        }
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.img_wrapper.update_scroll(x, y);
    }

    /// Called when the user resizes the window, or when it is resized
    fn resize_event(&mut self, ctx: &mut Context, w: f32, h: f32) {
        graphics::set_screen_coordinates(ctx, graphics::Rect{x: 0.0, y: 0.0, w, h}).unwrap();

        let (line_h, line_w) = MyGame::create_line_mesh(ctx, w, h);
        self.line_h = line_h;
        self.line_w = line_w;
    }
}
