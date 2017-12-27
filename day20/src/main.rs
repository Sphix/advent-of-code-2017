#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;

use failure::Error;
use std::str::FromStr;
use std::cmp::Ordering;
use std::ops::AddAssign;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Coordinates {
    x: i64,
    y: i64,
    z: i64,
}

impl Coordinates {
    fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Ord for Coordinates {
    fn cmp(&self, other: &Coordinates) -> Ordering {
        self.manhattan_distance().cmp(&other.manhattan_distance())
    }
}

impl PartialOrd for Coordinates {
    fn partial_cmp(&self, other: &Coordinates) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl AddAssign for Coordinates {
    fn add_assign(&mut self, other: Coordinates) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Particle {
    position: Coordinates,
    velocity: Coordinates,
    acceleration: Coordinates,
}

impl Particle {
    fn tick(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }
}

impl Ord for Particle {
    fn cmp(&self, other: &Particle) -> Ordering {
        let ord = self.acceleration.cmp(&other.acceleration);
        match ord {
            Ordering::Less | Ordering::Greater => return ord,
            _ => (),
        }
        let ord = self.velocity.cmp(&other.velocity);
        match ord {
            Ordering::Less | Ordering::Greater => return ord,
            _ => (),
        }
        self.position.cmp(&other.position)
    }
}

impl PartialOrd for Particle {
    fn partial_cmp(&self, other: &Particle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Particle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        named!(integer<&str, i64>, map!(
                pair!(
                    map!(opt!(tag!("-")),
                        |sign| if sign.is_some() { -1 } else { 1 }),
                    map_res!(nom::digit, str::parse::<i64>)),
                |(sign, val)| sign * val));

        named!(coordinates<&str, Coordinates>,
               do_parse!(tag!("<") >> x: integer >> tag!(",")
                                   >> y: integer >> tag!(",")
                                   >> z: integer >> tag!(">")
                                   >> (Coordinates { x: x, y: y, z: z })));

        do_parse!(
            s,
            p: delimited!(tag!("p="), coordinates, tag!(","))
                >> v: ws!(delimited!(tag!("v="), coordinates, tag!(",")))
                >> a: ws!(preceded!(tag!("a="), coordinates)) >> (Particle {
                position: p,
                velocity: v,
                acceleration: a,
            })
        ).to_result()
            .map_err(|e| format_err!("{}", e))
    }
}

fn parse_input(input: &str) -> Result<Vec<Particle>, Error> {
    input.split('\n').map(str::parse::<Particle>).collect()
}

fn closest_to_root(particles: &[Particle]) -> Option<usize> {
    let min = particles.iter().min()?;
    particles.iter().position(|p| p == min)
}

fn remove_collisions(particles: &mut Vec<Particle>) {
    fn position_sort(a: &Particle, b: &Particle) -> Ordering {
        let a = a.position;
        let b = b.position;
        let ord = a.x.cmp(&b.x);
        match ord {
            Ordering::Less | Ordering::Greater => return ord,
            _ => (),
        }
        let ord = a.y.cmp(&b.y);
        match ord {
            Ordering::Less | Ordering::Greater => return ord,
            _ => (),
        }
        a.z.cmp(&b.z)
    }
    particles.sort_unstable_by(position_sort);

    let mut i = 0;
    while i != particles.len() {
        let pos = particles[i].position;
        let count = particles
            .iter()
            .skip(i)
            .take_while(|particle| particle.position == pos)
            .count();
        if count > 1 {
            particles.drain(i..(i + count));
        } else {
            i += 1;
        }
    }
}

fn collide_particles(mut particles: &mut Vec<Particle>, iters: usize) {
    (0..iters).for_each(|_| {
        particles.iter_mut().for_each(Particle::tick);
        remove_collisions(&mut particles);
    });
}

fn main() {
    let input = include_str!("input.txt");
    let input = input.trim();
    let mut particles = parse_input(&input).expect("parse");

    println!("Result 1: {}", closest_to_root(&particles).unwrap());

    collide_particles(&mut particles, 1_000);
    println!("Result 2: {}", particles.len());
}
