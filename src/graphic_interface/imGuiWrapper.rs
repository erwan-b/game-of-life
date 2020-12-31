// This ImGuiWrapper will be the class that encapsulates all
// imgui functionality.
impl ImGuiWrapper {
    // This is going to take a ggez context and give us
    // back a fresh instance of the wrapper.
    pub fn new(ctx: &mut Context) -> Self {
        // ...
    }

    // This is what we will call on every render iteration
    // to render the imgui bits on top of our game.
    pub fn render(&mut self, ctx: &mut Context) {
        // ...
    }

    // This is how we'll update the mouse position. The UI needs
    // to be aware of the position so it can display a different color
    // when you hover over a button etc.
    pub fn update_mouse_pos(&mut self, x: i32, y: i32) {
        // ...
    }

    // This is how we'll tell imgui a mouse button has been pressed
    // or released. The 3 bools here are counterintuitively actually
    // 1. has the left mouse button been pressed
    // 2. has the right mouse button been pressed
    // 3. has the middle mouse button been pressed
    pub fn update_mouse_down(&mut self, pressed: (bool, bool, bool)) {
        // ...
    }

    // We could also handle mouse wheels for scrolling or
    // key events in a similar fashion, I'm hoping you get
    // the gist.
}