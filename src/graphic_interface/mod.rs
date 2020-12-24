use std::time;
mod constants;

use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};
use ggez::event::{EventHandler};
use ggez::event::MouseButton;

use crate::board::Board;
use crate::graphic_interface::constants::Constants;

/// `MyGame` describe the game graphic_interface logic
pub struct MyGame {
    last_refresh : time::Instant,
    board: Box<Board>,
    cell_mesh: graphics::Mesh,
    h_l: graphics::Mesh,
    w_l: graphics::Mesh,
    constants: Constants,
}

/// The impl is here to define our graphic_interface logic called by the `EventHandler`
/// It important to kipp it split from the the rest
impl MyGame {
    pub fn create_line_mesh(ctx: &mut Context) -> (graphics::Mesh, graphics::Mesh) {
        let color = [0.3, 0.3, 0.3, 1.0].into();
        let (w, h) = graphics::size(ctx);


        let h_l = graphics::Mesh::new_line(
            ctx, &[ Point2{ x: 0.0, y: 0.0 as f32}, Point2{ x: w, y: 0.0 } ],
            1.0,
            color,
        ).unwrap();

        let w_l = graphics::Mesh::new_line(
            ctx, &[ Point2{x: 0.0 as f32, y: 0.0}, Point2{x: 0.0, y: h} ],
            1.0,
            color,
        ).unwrap();
        (h_l, w_l)
    }

    fn create_cell_mesh(ctx: &mut Context, constants: Constants) -> graphics::Mesh {
        let cell_mesh_rect = graphics::Rect::new(0.0, 0.0, constants.cell_size, constants.cell_size);
        let cell_mesh = match graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            cell_mesh_rect,
            graphics::Color::from_rgb(255, 51, 255),
        ) {
            Ok(mesh) => mesh,
            Err(_e) => panic!("Could not create cell_mesh")
        };
        cell_mesh
    }

    pub fn new(ctx: &mut Context, board: Box<Board>) -> Self {
        let constants = Constants::new(16.0, 4.0, 1.0);
        let (h_l, w_l) = MyGame::create_line_mesh(ctx);

        MyGame {
            last_refresh: time::Instant::now(),
            board,
            cell_mesh: MyGame::create_cell_mesh(ctx, constants),
            h_l,
            w_l,
            constants,
        }
    }

    pub fn next(&mut self) {
        self.board.apply_on_all();
    }

    fn draw_board(&self, ctx: &mut Context, w: f32, h: f32) -> GameResult<()> {
        let step = self.constants.cell_size as usize;

        (0..w as usize).step_by(step).fold(Ok(()), |_acc, y| -> GameResult<()>  {
            (0..h as usize).step_by(step).fold(Ok(()), |_acc, x| -> GameResult<()>  {
                let dest = Point2 {
                    x: x as f32,
                    y: y as f32
                };

                if self.board.get_cell_or_dead(
                    (x as f32 / self.constants.cell_size) as i32,
                    (y as f32 / self.constants.cell_size) as i32).is_alive() {
                    graphics::draw(ctx, &self.cell_mesh, graphics::DrawParam::default().dest(dest))
                } else {
                    Ok(())
                }
            })
        })
    }

    fn draw_grid(&self, ctx: &mut Context, w: f32, h: f32) -> GameResult<()> {
        let step = self.constants.cell_size as usize;
        let max = if w > h { w } else { h };

        (0..max as i32).step_by(step).fold(Ok(()), | _acc, y| {
            graphics::draw(ctx, &self.w_l, (Point2 { x: y as f32, y: 0.0 }, ))?;
            graphics::draw(ctx, &self.h_l, (Point2 { x: 0.0, y: y as f32 }, ))
        })
    }
}

/// Define the `EventHandler` to mange the ggez lib events
impl EventHandler for MyGame {
    /// Update the cells there.
    /// There for we call the board function that return a new one with the rules applied on all cells.
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let duration = time::Instant::now() - self.last_refresh;

        if  duration.as_secs() > 1 {
            self.last_refresh = time::Instant::now();
            self.next()
        }
        Ok(())
    }

    /// Draw the board on the screen.
    /// The defined size of the cells we be translate to showed size with de zoom ratio
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        let (w, h) = graphics::size(ctx);
        self.draw_board(ctx, w, h)?;
        self.draw_grid(ctx, w, h)?;

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
    fn resize_event(&mut self, ctx: &mut Context, w: f32, h: f32) {
        let (h_l, w_l) = MyGame::create_line_mesh(ctx);
        self.h_l = h_l;
        self.w_l = w_l;
    }
}
