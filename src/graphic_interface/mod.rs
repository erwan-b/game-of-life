use std::time;
mod constants;

use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};
use ggez::event::{EventHandler};
use ggez::event::MouseButton;

use crate::board::Board;
use crate::graphic_interface::constants::Constants;
use crate::board::cell::STATUS;

/// `MyGame` describe the game graphic_interface logic
pub struct MyGame {
    last_refresh : time::Instant,
    board: Box<Board>,
    cell_mesh: graphics::Mesh,
    constants: Constants,
}

/// The impl is here to define our graphic_interface logic called by the `EventHandler`
/// It important to kipp it split from the the rest
impl MyGame {
    pub fn create_mesh(ctx: &mut Context, constants: Constants) -> graphics::Mesh {
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
        let c = Constants::new(15.0, 4.0, 1.0);
        MyGame {
            last_refresh: time::Instant::now(),
            board,
            cell_mesh: MyGame::create_mesh(ctx, c),
            constants: c,
        }
    }

    pub fn next(&mut self) {
        self.board.apply_on_all();

        println!("{}", self.board.get_line(0));
        println!("{}", self.board.get_line(1));
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
        let cell_h = (h / self.constants.cell_size) as i32;
        let cell_w = (w / self.constants.cell_size) as i32;


        (0..cell_w).for_each(|y| {
            (0..cell_h).for_each(|x| {
                let dest = Point2{x: (x as f32 * self.constants.cell_size) as f32, y: (y as f32 * self.constants.cell_size) as f32};
                let bounds = graphics::DrawParam::default().dest(dest);
                if self.board.get_cell_status(x, y) == STATUS::ALIVE {
                    let _res = graphics::draw(ctx, &self.cell_mesh, bounds);
                }
            });
        });

        graphics::present(ctx)
    }

    /// We need to track the mouse event to set a cell alive if the mouse is click on a valid cell.
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        println!("button down");
    }
}
