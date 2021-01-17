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
use crate::board::cell::{STATUS, Cell};

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

    last_refresh : time::Instant,
    game_step: i64,
    play: bool
}

/// The impl is here to define our graphic_interface logic called by the `EventHandler`
/// It important to kipp it split from the the rest
impl MyGame {
    pub fn create_line_mesh(ctx: &mut Context) -> (graphics::Mesh, graphics::Mesh) {
        let color = [0.3, 0.3, 0.3, 1.0].into();
        let (w, h) = graphics::size(ctx);


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
        let cell_mesh_rect = graphics::Rect::new(0.0, 0.0, camera.cell_size, camera.cell_size);

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
        let pos = board.nb_row() / 2 - 10;
        let (line_h, line_w) = MyGame::create_line_mesh(ctx);

        let camera = Camera::new(Point2{x: pos, y: pos}, Point2{x: w as usize, y: h as usize});
        let img = ImGuiWrapper::new(ctx);

        MyGame {
            board,
            constants: Constants::new(Duration::new(1, 0)),
            cell_mesh: MyGame::create_cell_mesh(ctx, &camera),
            img_wrapper: img,
            line_h,
            line_w,
            camera,
            play: false,
            game_step: 0,
            last_refresh: time::Instant::now()
        }
    }

    pub fn next(&mut self) {
        self.board.apply_on_all();
    }

    /// Draw each line limitation of the board
    fn draw_line(&self, ctx: &mut Context) -> GameResult<()> {
        self.camera.size_shown_iter().filter(|&pixel| pixel.screen_pos.x ==  0)
        .fold(Ok(()), | _acc, pixel|{
            graphics::draw(ctx, &self.line_w, (Point2 { x: pixel.screen_pos.y as f32, y: 0.0 }, ))?;
            graphics::draw(ctx, &self.line_h, (Point2 { x: 0.0, y: pixel.screen_pos.y as f32 }, ))
        })
    }

    /// Draw the living cells on the board
    fn draw_board(&self, ctx: &mut Context) -> GameResult<()> {
        let step = self.camera.cell_size as usize;

        self.camera.size_shown_iter()
            .filter_map(|pixel|
                if self.board.get_cell_or_dead(
                    (pixel.board_pixel_pos.x as f32 / self.camera.cell_size) as i32,
                    (pixel.board_pixel_pos.y as f32 / self.camera.cell_size) as i32,).is_alive() {
                    Some(pixel)
                } else {
                    None
                })
            .fold(Ok(()), | _acc, pixel| {
                graphics::draw(ctx, &self.cell_mesh, graphics::DrawParam::default().dest(Point2 {
                    x: pixel.screen_pos.x as f32,
                    y: pixel.screen_pos.y as f32
                }))
            })
    }

    fn update_button(&mut self) {
        match self.img_wrapper.get_last_button() {
            Some(UiButton::NEXT) => { self.game_step += 1;}
            Some(UiButton::PREV) => { self.game_step -= 1;}
            Some(UiButton::STOP) => { self.play = false; }
            Some(UiButton::PLAY) => { self.play = true; }
            _ => {}
        }
        self.constants.refresh_rate = self.img_wrapper.get_time_per_step();
    }
}

/// Define the `EventHandler` to mange the ggez lib events
impl EventHandler for MyGame {
    /// Update the cells there.
    /// There for we call the board function that return a new one with the rules applied on all cells.
    /// TODO may be put the update part in a thread so we can have a huge board
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
        }
        Ok(())
    }

    /// Draw the board on the screen.
    /// The defined size of the cells we be translate to showed size with de zoom ratio
    /// TODO The draw take a lot of CPU  !!!!
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        self.draw_board(ctx)?;
        self.draw_line(ctx)?;
        self.img_wrapper.render(ctx, 2.0, self.play);

        graphics::present(ctx)
    }

    /// We need to track the mouse event to set a cell alive if the mouse is click on a valid cell.
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        let (w, h) = ((x / 16.0) as i32, (y / 16.0) as i32);
        match self.board.get_cell(w, h) {
            Some(cell) => self.board.set_cell(w, h, cell.status.inverse()),
            _ => None
        };
        self.img_wrapper.update_mouse_pos(x, y);
        self.img_wrapper.update_mouse_down(button);
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.img_wrapper.update_mouse_pos(x, y);
        self.img_wrapper.update_mouse_up(button);
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.img_wrapper.update_mouse_pos(x, y);
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
        self.img_wrapper.update_scroll(x, y);
    }

    /// Called when the user resizes the window, or when it is resized
    fn resize_event(&mut self, ctx: &mut Context, _w: f32, _h: f32) {
        let (line_h, line_w) = MyGame::create_line_mesh(ctx);
        self.line_h = line_h;
        self.line_w = line_w;
    }
}
