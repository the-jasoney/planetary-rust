use super::object::Object;
use super::vec2::Vec2;
use crate::vec2;

const GRAVITATIONAL_CONST: f64 = 500.0;
const TRAJECTORY_RESOLUTION: u64 = 2;

#[derive(Clone)]
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

    pub fn destroy_collisions(&mut self) -> Vec<[usize; 2]>{
        let mut collisions: Vec<[usize; 2]> = vec![];

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
                        collisions.push([idx, jdx]);
                        if i.constant_pos == true {
                            self.objects.remove(jdx);
                        } else if j.constant_pos == true {
                            self.objects.remove(idx);
                        } else {
                            i.mass = i.mass + j.mass;
                            i.velocity = (i.velocity * i.mass + j.velocity * j.mass) / (i.mass + j.mass);
                            self.objects.remove(jdx);
                        }
                        continue 'l;
                    }
                }
            }
            break 'l;
        }

        collisions
    }

    pub fn solve_all(&mut self, dt: f64) {
        self.solve_accelerations();
        self.solve_euler(dt);
        self.destroy_collisions();
    }

    pub fn center_of_mass(&self) -> Vec2 {
        let mut massxpos = vec2!();
        let mut masses = 0.0;
        for i in &self.objects {
            massxpos += i.position * i.mass;
            masses += i.mass;
        }
        massxpos/masses
    }

    pub fn trajectory(&self, mut object: Object, t: u64) -> Vec<Vec2> {
        //println!("calculating trajectory for object: {:#?}", object);

        let mut locations: Vec<Vec2> = vec![];
        let mut old_position: Vec2 = object.position;

        if object.constant_pos {
            return locations;
        }

        'f: for _ in 0..t*TRAJECTORY_RESOLUTION {

            let dt = 1.00/TRAJECTORY_RESOLUTION as f64;

            let mut acceleration: Vec2 = vec2!();
            // calculate gravitational pulls

            for i in &self.objects {
                let term = (GRAVITATIONAL_CONST * object.mass * i.mass * (i.position - object.position)) /
                    (
                        (i.position - object.position).magnitude() *
                        (i.position - object.position).magnitude() *
                        (i.position - object.position).magnitude()
                    ).abs();

                let radius = i.mass.sqrt();
                let midpoint = vec2!(
                    (old_position + object.position).x / 2.0,
                    (old_position + object.position).y / 2.0
                );
                let dist = Vec2::dist_scalar(midpoint, i.position);

                if
                    (object.position - i.position).abs().magnitude() < (object.mass.sqrt() + i.mass.sqrt()) ||
                    dist < radius
                {
                    break 'f;
                }

                acceleration += term;
            }
            old_position = object.position;
            object.velocity += acceleration/object.mass * dt;
            object.position += object.velocity * dt;

            locations.push(old_position);
        }
        //println!("locations {:#?}", locations);
        locations

    }
}
