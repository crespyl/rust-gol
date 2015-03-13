#![feature(core, collections, plugin)]
#![plugin(docopt_macros)]
extern crate core;
extern crate docopt;
extern crate rand;
extern crate "rustc-serialize" as rustc_serialize;

#[macro_use]
extern crate bitflags;

use std::mem;
use std::collections::BitVec;
use core::num::wrapping::OverflowingOps;
use core::iter::FromIterator;
use docopt::Docopt;
use rand::random;

use std::time::duration::Duration;
use std::old_io::timer::sleep;

docopt!(Args derive Debug, "
gol: Simulate Conway's Game of Life

Usage: gol [options]
Try 'gol --help' for more information.

Options:
    -w, --width=WIDTH           Width of the simulation grid [default: 10].
    -h, --height=HEIGHT         Height of the simulation grid [default: 10].
    -n, --generations=STEPS           Run the simulation for a given number of generations [default: 1].
    -s, --speed=SPEED           Ticks per second [default: 10]
    -d, --dead=CHAR             Use the given character to display \"dead\" cells [default: .].
    -l, --live=CHAR             Use the given character to display \"live\" cells [default: #].

    -h, --help          Show this message.
    --version           Show the version number.
",
        flag_width: usize,
        flag_height: usize,
        flag_generations: usize,
        flag_speed: usize,
        flag_dead: char,
        flag_live: char,
);

fn main() {
    let args: Args = Args::docopt()
        .version(Some(env!("CARGO_PKG_VERSION").to_string()))
        .help(true)
        .decode().unwrap_or_else(|e| e.exit());

    let (dead, live) = (args.flag_dead, args.flag_live);
    let (width, height) = (args.flag_width, args.flag_height);
    let generations = args.flag_generations;
    println!("simulating {} cells for {} generations", width*height, generations);

    let mut world = &mut BitVec::from_elem(width*height, false);
    let mut next = &mut BitVec::from_elem(width*height, false);

    for i in 0..width*height {
        world.set(i, random::<f32>() < 0.5);
    }

    println!("\n{}\ngenerations: {}", format_bitvec(world, width, height, dead, live), 0);

    // run the simulation
    for i in 0..generations+1 {
        print!("\x1b[{}A", height+2);
        print!("\x1b[{}D", width+1);
        sleep(Duration::milliseconds(1000 / args.flag_speed as i64));

        for i in 0..width*height {
            let state = cell_neighbors(i, &world, width, height, false);
            next.set(i, evolve(world.get(i).unwrap_or(false), state));
        }
        mem::swap(world, next);

        println!("{}\ngenerations: {}", format_bitvec(world, width, height, dead, live), i);
    }
}

/// Takes a number representing the eight neighbors of a cell and returns
/// whether, according to this rule, the center cell should become live
/// or dead at the next generation.
fn evolve(live: bool, neighbors: usize) -> bool {
    if live {
        2 <= neighbors && neighbors <= 3
    } else {
        neighbors == 3
    }
}

/// Get the three bits representing a single cell and its neighbors
/// If wrap is true, the first and last cells are considered neighbors, otherwise
/// any neighbors "out of bounds" are always considered to be dead.
fn cell_neighbors(cell_idx: usize, world: &BitVec, width: usize, height: usize, wrap: bool) -> usize {
    if let Some(cell) = world.get(cell_idx) {
        vec![
            cell_idx.overflowing_sub(width).0.overflowing_sub(1).0,
            cell_idx.overflowing_sub(width).0,
            cell_idx.overflowing_sub(width).0.overflowing_add(1).0,
            
            cell_idx.overflowing_sub(1).0,
            cell_idx.overflowing_add(1).0,

            cell_idx.overflowing_add(width).0.overflowing_sub(1).0,
            cell_idx.overflowing_add(width).0,
            cell_idx.overflowing_add(width).0.overflowing_add(1).0]
            .iter()
            .fold(0, |sum, &idx| if world.get(idx).unwrap_or(false) { sum+1 } else { sum })
        
    } else {
        0
    }
}

/// Simple function to print out a BitVec as '.' and '#' characters
fn format_bitvec(bv: &BitVec, width: usize, height: usize, dead: char, live: char) -> String {
    let mut res = String::new();
    for row in 0..height {
        for col in 0..width {
            res.push(if bv.get(width*row + col).unwrap_or(false) { live } else { dead });
        }
        res.push('\n');
    }
    res
}
