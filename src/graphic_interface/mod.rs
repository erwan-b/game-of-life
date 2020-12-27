use std::time;
mod constants;
mod camera;

use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};
use ggez::event::{EventHandler};
use ggez::event::MouseButton;

use crate::board::Board;
use crate::graphic_interface::constants::Constants;
use crate::graphic_interface::camera::Camera;
use std::time::Duration;

/// `MyGame` describe the game graphic_interface logic
/// It contain:
/// <p> - some static mesh  </p>
/// <p> - some static mesh  </p>
/// <p> - data about the refresh rate and games constants  </p>
pub struct MyGame {
    board: Box<Board>,
    camera: Camera,

    cell_mesh: graphics::Mesh,
    line_h: graphics::Mesh,
    line_w: graphics::Mesh,

    last_refresh : time::Instant,
    constants: Constants,
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

    fn create_cell_mesh(ctx: &mut Context, constants: Constants) -> graphics::Mesh {
        let cell_mesh_rect = graphics::Rect::new(0.0, 0.0, constants.cell_size, constants.cell_size);

        match graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(),
                                            cell_mesh_rect, graphics::Color::from_rgb(255, 51, 255),
        ) {
            Ok(mesh) => mesh,
            Err(_e) => panic!("Could not create cell_mesh")
        }
    }

    pub fn new(ctx: &mut Context, board: Box<Board>) -> Self {
        let constants = Constants::new(
            16.0,
            4.0,
            Duration::new(1, 0)
        );
        let (line_h, line_w) = MyGame::create_line_mesh(ctx);
        let (w, h) = graphics::size(ctx);

        MyGame {
            last_refresh: time::Instant::now(),
            board,
            cell_mesh: MyGame::create_cell_mesh(ctx, constants),
            line_h,
            line_w,
            constants,
            camera: Camera::new(Point2{x: 0, y: 0}, Point2{x: w as usize, y: h as usize})
        }
    }

    pub fn next(&mut self) {
        self.board.apply_on_all();
    }

    /// Draw each line limitation of the board
    fn draw_line(&self, ctx: &mut Context) -> GameResult<()> {
        self.camera.size_shown_iter().filter(|(x, _y)| *x == 0 )
        .fold(Ok(()), | _acc, (_x, y)| {
            graphics::draw(ctx, &self.line_w, (Point2 { x: *y as f32, y: 0.0 }, ))?;
            graphics::draw(ctx, &self.line_h, (Point2 { x: 0.0, y: *y as f32 }, ))
        })
    }

    /// Draw the living cells on the board
    fn draw_board(&self, ctx: &mut Context) -> GameResult<()> {
        let step = self.constants.cell_size as usize;

        self.camera.size_shown_iter()
            .filter_map(|(x, y)|
                if self.board.get_cell_or_dead((*x / step) as i32, (*y / step) as i32).is_alive() {
                    Some((x, y))
                } else {
                    None
                }
            )
            .fold(Ok(()), | _acc, (&x, &y)| {
                graphics::draw(ctx, &self.cell_mesh, graphics::DrawParam::default().dest(Point2 {
                x: x as f32,
                y: y as f32
            }))
        })
    }
}

/// Define the `EventHandler` to mange the ggez lib events
impl EventHandler for MyGame {
    /// Update the cells there.
    /// There for we call the board function that return a new one with the rules applied on all cells.
    /// TODO may be put the update part in a thread so we can have a huge board
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let duration = time::Instant::now() - self.last_refresh;

        if  duration > self.constants.refresh_rate  {
            self.last_refresh = time::Instant::now();
            self.next()
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
        graphics::present(ctx)
    }

    /// We need to track the mouse event to set a cell alive if the mouse is click on a valid cell.
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) { }

    /// Called when the user resizes the window, or when it is resized
    fn resize_event(&mut self, ctx: &mut Context, _w: f32, _h: f32) {
        let (line_h, line_w) = MyGame::create_line_mesh(ctx);
        self.line_h = line_h;
        self.line_w = line_w;
    }
}
