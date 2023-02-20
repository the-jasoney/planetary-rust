extern crate piston_window;

mod sim;

use piston_window::*;
use piston_window::ellipse::circle;

use piston_window::Motion::{MouseCursor, MouseScroll};
use piston_window::Button as ButtonType;
use sim::object::Object;
use sim::solver::Solver;
use sim::vec2::Vec2;
use std::time::Instant;

/// Create new fullscreen window.
pub fn create_window(w: u32, h: u32) -> piston_window::PistonWindow {
        WindowSettings::new("planetary rust", [w, h])
            .exit_on_esc(true)
            .fullscreen(true)
            .build()
            .unwrap()
}

fn main() {
    let mut window = create_window(800, 400);
    let w = window.size().width;
    let h = window.size().height;

    let mut solver = Solver::new();

    let mut mouse_x: f64 = 0.0;
    let mut mouse_y: f64 = 0.0;

    let mut mass: f64 = 10.0;
    let mut constant_pos: bool = false;

    let mut last_tick: Instant = Instant::now();

    let mut mouse_down_position: Option<Vec2> = None;
    let mut mouse_up_position: Option<Vec2> = None;

    let mut show_vectors = false;

    while let Some(event) = window.next() {
        let dt: f64 = last_tick.elapsed().as_secs_f64();
        last_tick = Instant::now();

        solver.solve_all(dt);
        if let Event::Input(input, _) = &event {
            if let Input::Move(x) = input {
                if let MouseCursor(pos) = x {
                    [mouse_x, mouse_y] = *pos;
                }
                if let MouseScroll([_, dy]) = *x {
                    mass += dy * 10.0;
                }
            }
            if let Input::Button(x) = *input{
                if x.button == ButtonType::Mouse(MouseButton::Left) { // mouse left click
                    if x.state == ButtonState::Press {
                        mouse_down_position = Some(Vec2::from_arr([mouse_x, mouse_y]));
                    }

                    if x.state == ButtonState::Release {
                        mouse_up_position = Some(Vec2::from_arr([mouse_x, mouse_y]));
                    }
                } else if
                    x.button == ButtonType::Keyboard(Key::Backspace) ||
                    x.button == ButtonType::Keyboard(Key::Delete)
                { // clear objects with backspace/delete and reset time scaling factor
                    solver.objects = vec![];
                } else if x.button == ButtonType::Keyboard(Key::C) { // space toggle vectors
                    if x.state == ButtonState::Press {
                        constant_pos = !constant_pos;
                    }
                } else if x.button == ButtonType::Keyboard(Key::Undo) {
                    println!("undo!");
                    solver.objects.pop();
                } else if x.button == ButtonType::Keyboard(Key::Space) {
                    if x.state == ButtonState::Press {
                        show_vectors = !show_vectors;
                    }
                }
            }
        }

        let main_ellipse_drawer = Ellipse::new([1.0; 4]);
        let main_line_drawer = Line::new([1.0; 4], 1.0);

        let possible_ellipse_drawer = Ellipse::new([0.5; 4]);
        let possible_const_drawer = Ellipse::new([0.75; 4]);
        let possible_line_drawer = Line::new([0.5; 4], 1.0);

        if let (Some(d), Some(u)) = (mouse_down_position, mouse_up_position) {
            solver.add_object(d, u - d, constant_pos, mass);

            mouse_down_position = None;
            mouse_up_position = None;
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0, 0.0, 0.0, 0.25], graphics);
            for i in &solver.objects {
                main_ellipse_drawer.draw(
                    circle(
                        i.position.x,
                        i.position.y,
                        i.mass.sqrt()
                    ),
                    &context.draw_state,
                    context.transform,
                    graphics
                );

                if show_vectors {
                    main_line_drawer.draw_arrow(
                        [
                            i.position.x,
                            i.position.y,
                            i.position.x + i.velocity.x,
                            i.position.y + i.velocity.y
                        ],
                        6.0,
                        &context.draw_state,
                        context.transform,
                        graphics);

                    possible_line_drawer.draw_arrow(
                        [
                            i.position.x,
                            i.position.y,
                            i.position.x + i.acceleration.x * dt,
                            i.position.y + i.acceleration.y * dt
                        ],
                        6.0,
                        &context.draw_state,
                        context.transform,
                        graphics);
                }
            }

            if let (Some(x), None) = (mouse_down_position, mouse_up_position) {
                possible_ellipse_drawer.draw(
                    circle(x.x, x.y, mass.sqrt()),
                    &context.draw_state,
                    context.transform,
                    graphics
                );

                possible_line_drawer.draw_arrow([
                    x.x,
                    x.y,
                    mouse_x,
                    mouse_y
                ], 6.0, &context.draw_state, context.transform, graphics);

                /*
                let trajectory = solver.advance(Object {
                    position: x,
                    velocity: vec2!(mouse_x - x.x, mouse_y - x.y),
                    acceleration: vec2!(),
                    constant_pos,
                    mass
                }, 20);

                for i in trajectory {
                    possible_ellipse_drawer.draw(
                        circle(i.x, i.y, 5.0),
                        &context.draw_state,
                        context.transform,
                        graphics
                    );
                }*/
            } else {
                if constant_pos {
                    possible_const_drawer.draw(
                        circle(mouse_x, mouse_y, mass.sqrt()),
                        &context.draw_state,
                        context.transform,
                        graphics
                    );
                } else {
                    possible_ellipse_drawer.draw(
                        circle(mouse_x, mouse_y, mass.sqrt()),
                        &context.draw_state,
                        context.transform,
                        graphics
                    );
                }
            }
        });
    }
}
