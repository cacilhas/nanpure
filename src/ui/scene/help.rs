use rscenes::prelude::*;

#[derive(Debug, Default)]
pub struct HelpScene(bool);

impl Scene for HelpScene {
    fn update(
        &mut self,
        _: (&mut RaylibHandle, &RaylibThread),
        _: f32,
        _: Option<std::rc::Rc<&mut RaylibAudio>>,
    ) -> anyhow::Result<State> {
        if self.0 {
            self.0 = false;
            Ok(State::Previous(1))
        } else {
            Ok(State::Keep)
        }
    }

    fn draw(
        &mut self,
        handle: &mut RaylibDrawHandle,
        screen: Rectangle,
        font: Option<std::rc::Rc<Font>>,
        _: Option<std::rc::Rc<&mut RaylibAudio>>,
    ) -> anyhow::Result<()> {
        let font = font.unwrap();
        let clicked =
            handle.is_mouse_button_released(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON);
        let mouse = Vector2::new(handle.get_mouse_x() as f32, handle.get_mouse_y() as f32);

        let camera = Camera2D {
            zoom: 1.0,
            ..Default::default()
        };
        let mut draw = handle.begin_mode2D(camera);

        let background_color = colors::WHEAT;
        draw.clear_background(background_color);

        let size = measure_text_ex(font.as_ref(), "Nanpure", 84.0, 2.0);
        let position = Vector2::new((screen.width - size.x) / 2.0, 0.0);
        let bottom = size.y + 16.0;
        draw.draw_text_ex(
            font.as_ref(),
            "Nanpure",
            position,
            84.0,
            2.0,
            colors::DARKCYAN,
        );

        let size = measure_text_ex(font.as_ref(), "(Sudoku)", 32.0, 1.0);
        let position = Vector2::new((screen.width - size.x) / 2.0, bottom);
        let bottom = bottom + size.y + 64.0;
        draw.draw_text_ex(
            font.as_ref(),
            "(Sudoku)",
            position,
            32.0,
            2.0,
            colors::DARKCYAN,
        );

        // TODO: help here

        let size = measure_text_ex(font.as_ref(), "Back", 24.0, 1.0);
        let position = Vector2::new(screen.width - size.x - 12.0, screen.height - size.y - 12.0);
        let back_bt = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if back_bt.check_collision_point_rec(mouse) {
            if clicked {
                self.0 = true;
            }
            colors::BLACK
        } else {
            colors::DARKGRAY
        };
        draw.draw_text_ex(font.as_ref(), "Back", position, 24.0, 1.0, tint);
        Ok(())
    }
}
