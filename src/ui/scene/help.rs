use crate::game::COLORS;
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
        let mut bottom = bottom + size.y + 64.0;
        draw.draw_text_ex(
            font.as_ref(),
            "(Sudoku)",
            position,
            32.0,
            2.0,
            colors::DARKCYAN,
        );

        let msgs = [
            "WASD: select",
            "Cursor keys: select",
            "1~9: toggle candidate",
            "Shift + 1~9: toggle value",
            "Space: alias to toggle one value",
            "F1: help",
            "Esc: back",
        ];

        let mut width = 0f32;
        for msg in msgs {
            let size = measure_text_ex(font.as_ref(), msg, 24.0, 1.0);
            if size.x > width {
                width = size.x;
            }
        }

        for msg in msgs {
            let size = measure_text_ex(font.as_ref(), msg, 24.0, 1.0);
            let position = Vector2::new((screen.width - width) / 2.0, bottom);
            bottom += 12.0 + size.y;
            let label = Rectangle {
                x: position.x,
                y: position.y,
                width,
                height: size.y,
            };
            let tint = if label.check_collision_point_rec(mouse) {
                colors::BLACK
            } else {
                colors::DARKGRAY
            };
            draw.draw_text_ex(font.as_ref(), msg, position, 24.0, 1.0, tint);
        }

        let size = measure_text_ex(font.as_ref(), "Colours: 1 2 3 4 5 6 7 8 9", 24.0, 1.0);
        let mut position = Vector2::new((screen.width - size.x) / 2.0, bottom);
        let size = measure_text_ex(font.as_ref(), "Colours:", 24.0, 1.0);
        draw.draw_text_ex(font.as_ref(), "Colours:", position, 24.0, 1.0, COLORS[0]);
        position.x += size.x;
        for i in 1..=9 {
            let msg = format!(" {i}");
            draw.draw_text_ex(font.as_ref(), &msg, position, 24.0, 1.0, COLORS[i]);
            let size = measure_text_ex(font.as_ref(), &msg, 24.0, 1.0);
            position.x += size.x;
        }

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