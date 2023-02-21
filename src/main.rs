extern crate piston_window;

mod sim;

use piston_window::*;
use piston_window::ellipse::circle;

use piston_window::Motion::{MouseCursor, MouseScroll};
use piston_window::Button::*;
use sim::solver::Solver;
use sim::object::Object;
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

    let mut solver = Solver::new();

    let mut mouse_x: f64 = 0.0;
    let mut mouse_y: f64 = 0.0;

    let mut mass: f64 = 10.0;
    let mut constant_pos: bool = false;

    let mut last_tick: Instant = Instant::now();

    let mut mouse_down_position: Option<Vec2> = None;
    let mut mouse_up_position: Option<Vec2> = None;

    let mut time_change: f64 = 0.0;

    let mut show_vectors = false;
    let mut show_center_of_mass = false;

    let mut time_scaling_factor: f64 = 2.0;

    while let Some(event) = window.next() {
        let dt: f64 = last_tick.elapsed().as_secs_f64();
        last_tick = Instant::now();
        if time_scaling_factor > 0.0 || time_change > 0.0 {
            time_scaling_factor += time_change;
        }

        solver.solve_all(dt * time_scaling_factor);
        if let Event::Input(input, _) = &event {
            if let Input::Move(x) = input {
                if let MouseCursor(pos) = x {
                    [mouse_x, mouse_y] = *pos;
                }
                if let MouseScroll([_, dy]) = *x {
                    mass += dy * 100.0;

                }
            }
            if let Input::Button(x) = *input{
                match x.button {
                    Mouse(MouseButton::Left) => {
                        if x.state == ButtonState::Press {
                            mouse_down_position = Some(Vec2::from_arr([mouse_x, mouse_y]));
                        } else if x.state == ButtonState::Release {
                            mouse_up_position = Some(Vec2::from_arr([mouse_x, mouse_y]));
                        }
                    }

                    Keyboard(Key::Delete) => {
                        solver.objects = vec![];
                    }
                    Keyboard(Key::Backspace) => {
                        if x.state == ButtonState::Press {
                            solver.objects.pop();
                        }
                    }
                    Keyboard(Key::Space) => {
                        if x.state == ButtonState::Press {
                            show_vectors = !show_vectors;
                        }
                    }
                    Keyboard(Key::Down) => {
                        if x.state == ButtonState::Press && time_scaling_factor > 0.0 {
                            time_change = -0.005;
                        } else {
                            time_change = 0.0;
                        }
                    }
                    Keyboard(Key::Up) => {
                        if x.state == ButtonState::Press {
                            time_change = 0.005;
                        } else {
                            time_change = 0.0;
                        }
                    }
                    Keyboard(Key::D0) => {
                        if x.state == ButtonState::Press {
                            time_change = 0.0;
                            time_scaling_factor = 1.0;
                        }
                    }
                    Keyboard(Key::C) => {
                        if x.state == ButtonState::Press {
                            constant_pos = !constant_pos;
                        }
                    }
                    Keyboard(Key::M) => {
                        if x.state == ButtonState::Press {
                            show_center_of_mass = !show_center_of_mass;
                        }
                    }
                    _ => {}
                };
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
            clear([0.0, 0.0, 0.0, 0.0], graphics);
            let com = solver.center_of_mass();

            if show_center_of_mass {
                possible_ellipse_drawer.draw(
                    circle(com.x, com.y, 10.0),
                    &context.draw_state,
                    context.transform,
                    graphics
                );
            }

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

                let trajectory = solver.trajectory(Object {
                    position: vec2!(x.x, x.y),
                    velocity: vec2!(mouse_x, mouse_y) - vec2!(x.x, x.y),
                    acceleration: vec2!(),
                    constant_pos,
                    mass
                }, (15.0 * time_scaling_factor) as u64);
                let mut im1 = vec2!(x.x, x.y);
                for i in trajectory {
                    possible_ellipse_drawer.draw(
                        circle(i.x, i.y, 5.0),
                        &context.draw_state,
                        context.transform,
                        graphics
                    );

                    possible_line_drawer.draw(
                        [
                            im1.x,
                            im1.y,
                            i.x,
                            i.y
                        ],
                        &context.draw_state,
                        context.transform,
                        graphics
                    );

                    im1 = i;
                }
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
