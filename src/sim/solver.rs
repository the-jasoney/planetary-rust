use super::object::{Object, self};
use super::vec2::Vec2;
use crate::vec2;

const GRAVITATIONAL_CONST: f64 = 1000.0;

pub struct Solver {
    pub objects: Vec<Object>
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            objects: vec![]
        }
    }

    pub fn add_object(&mut self, position: Vec2, velocity: Vec2, constant_pos: bool, mass: f64) {
        self.objects.push(Object {
            position,
            velocity,
            mass,
            constant_pos,
            acceleration: vec2!()
        })
    }

    pub fn solve_accelerations(&mut self) {
        let mut sums: Vec<Vec2> = vec![];

        for idx in 0..self.objects.len() {
            let mut sum = vec2!();
            for jdx in 0..self.objects.len() {
                if idx == jdx {
                    continue;
                }

                let (i, j) =
                    if idx < jdx {
                        let (head, tail) = self.objects.split_at_mut(idx + 1);
                        (&mut head[idx], &mut tail[jdx - idx - 1])
                    } else {
                        let (head, tail) = self.objects.split_at_mut(jdx + 1);
                        (&mut tail[idx - jdx - 1], &mut head[jdx])
                    };

                if i.constant_pos {
                    continue;
                }

                let term = (GRAVITATIONAL_CONST * i.mass * j.mass * (j.position - i.position)) /
                    (
                        (j.position - i.position).magnitude() *
                        (j.position - i.position).magnitude() *
                        (j.position - i.position).magnitude()
                    ).abs();
                sum += term;
            }
            sums.push(sum);
        }

        //println!("{:?}", sums);

        for (object, acceleration) in self.objects.iter_mut().zip(sums.iter()) {
            object.acceleration = *acceleration;
        }
    }

    pub fn solve_euler(&mut self, dt: f64) {
        for i in &mut self.objects {
            if i.constant_pos {
                continue;
            }

            i.velocity += i.acceleration/i.mass * dt;
            i.position += i.velocity * dt;
        }
    }

    pub fn destroy_collisions(&mut self) {
        'l: loop {
            for idx in 0..self.objects.len() {
                for jdx in 0..self.objects.len() {
                    if idx == jdx {
                        continue;
                    }

                    let (i, j) =
                        if idx < jdx {
                            let (head, tail) = self.objects.split_at_mut(idx + 1);
                            (&mut head[idx], &mut tail[jdx - idx - 1])
                        } else {
                            let (head, tail) = self.objects.split_at_mut(jdx + 1);
                            (&mut tail[idx - jdx - 1], &mut head[jdx])
                        };

                    if (i.position - j.position).abs().magnitude() < (i.mass.sqrt() + j.mass.sqrt()) {
                        if i.constant_pos == true {
                            self.objects.remove(jdx);
                        } else if j.constant_pos == true {
                            self.objects.remove(idx);
                        } else {
                            if idx > jdx {
                                self.objects.remove(idx);
                                self.objects.remove(jdx);
                            } else {
                                self.objects.remove(jdx);
                                self.objects.remove(idx);
                            }
                        }
                        continue 'l;
                    }
                }
            }
            break 'l;
        }
    }

    pub fn solve_all(&mut self, dt: f64) {
        self.solve_accelerations();
        self.solve_euler(dt);
        self.destroy_collisions();
    }

    pub fn advance(&mut self, object: Object, t: u64) -> Vec<Vec2> {
        let txr = t*5;

        let mut positions: Vec<Vec2> = vec![];

        for k in 0..txr {
            let bv = (1/5) as f64;

            self.objects.push(object);
            let idx = self.objects.len() - 1;

            println!("{}", self.objects.len());
            if self.objects.len() > 1 {
                for jdx in 0..self.objects.len() - 1 {
                    println!("{}, {}", idx, jdx);

                    let didxjdx = idx - jdx;
                    let idxmjdx = if didxjdx > 0 {didxjdx - 1} else {didxjdx};

                    let (head, tail) = self.objects.split_at_mut(jdx + 1);
                    let (i, j) = (&mut tail[idxmjdx], &mut head[jdx]);

                    if jdx != idx {
                        i.acceleration =
                        (GRAVITATIONAL_CONST * i.mass * j.mass * (j.position - i.position)) /
                        (
                            (j.position - i.position).magnitude() *
                            (j.position - i.position).magnitude() *
                            (j.position - i.position).magnitude()
                        ).abs()
                    }
                }
            }

            let mut i = self.objects[idx];
            i.velocity += i.acceleration/i.mass * bv;
            i.position += i.velocity * bv;

            positions.push(i.position);
        }
        self.objects.pop();

        return positions;
    }
}
