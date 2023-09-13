// Silence some warnings that could distract from the exercise
#![allow(unused_variables, unused_mut, dead_code)]

// Someone is shooting arrows at a target.  We need to classify the shots.
//
// 1a. Create an enum called `Shot` with variants:
// - `Bullseye`
// - `Hit`, containing the distance from the center (an f64)
// - `Miss`
//
// You will need to complete 1b as well before you will be able to run this program successfully.

use std::iter::Map;
use std::slice::Iter;

enum Shot {
    Bullseye,
    Hit(f64),
    Miss,
}

impl Shot {
    // Here is a method for the `Shot` enum you just defined.
    fn points(&self) -> i32 {
        // 1b. Implement this method to convert a Shot into points
        // - return 5 points if `self` is a `Shot::Bullseye`
        // - return 2 points if `self` is a `Shot::Hit(x)` where x < 3.0
        // - return 1 point if `self` is a `Shot::Hit(x)` where x >= 3.0
        // - return 0 points if `self` is a Miss
        match *self {
            Shot::Bullseye => 5,
            Shot::Hit(d) if d < 3.0 => 2,
            Shot::Hit(d) => 1 ,
            Shot::Miss => 0
        }
    }
}

fn main() {
    // Simulate shooting a bunch of arrows and gathering their coordinates on the target.
    let arrow_coords: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<Shot> = Vec::new();

    // 2. For each coord in arrow_coords:
    //
    //   A. Call `coord.print_description()`
    //   B. Append the correct variant of `Shot` to the `shots` vector depending on the value of
    //   `coord.distance_from_center()`
    //      - Less than 1.0 -- `Shot::Bullseye`
    //      - Between 1.0 and 5.0 -- `Shot::Hit(value)`
    //      - Greater than 5.0 -- `Shot::Miss`
    for coords in arrow_coords.iter() {
        coords.print_description();
        let d = coords.distance_from_center();
        shots.push(
            if d < 1.0 { Shot::Bullseye } else if d > 5.0 { Shot::Miss } else { Shot::Hit(d) })
    }

    fn shots2(arrow_coords_iter: Iter<Coord>) -> Map<Iter<Coord>, fn(&Coord) -> Shot> {
        arrow_coords_iter.map(|coords| {
            coords.print_description();
            let d = coords.distance_from_center();
            if d < 1.0 { Shot::Bullseye } else if d > 5.0 { Shot::Miss } else { Shot::Hit(d) }
        })
    }

    let mut total = 0;
    // 3. Finally, loop through each shot in shots and add its points to total
    for shot in shots {
        total += shot.points()
    }

    let total2 = shots2(arrow_coords.iter()).fold(0, |t, shot| t + shot.points());
    let total3 = shots2(arrow_coords.iter()).map(|shot| shot.points()).sum();

    println!("Final point total is: {}", total);
    println!("Final point total (2) is: {}", total2);
    println!("Final point total (3) is: {}", total2);
    assert_eq!(total, total2);
    assert_eq!(total, total3);
}

// A coordinate of where an Arrow hit
#[derive(Debug)]
struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    fn distance_from_center(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
    fn print_description(&self) {
        println!(
            "coord is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y);
    }
}

// Generate some random coordinates
fn get_arrow_coords(num: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();
    for _ in 0..num {
        let coord = Coord {
            x: (rand::random::<f64>() - 0.5) * 12.0,
            y: (rand::random::<f64>() - 0.5) * 12.0,
        };
        coords.push(coord);
    }
    coords
}