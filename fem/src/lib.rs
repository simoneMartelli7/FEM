//use std::collections::HashMap;
use std::io;

/*in all this code the following reference frame will be used :
           z
            |_
              x
     y is oriented such as to be Z x X = Y


     All units must be in SI: so length are in meters, angles in radians, pressures in Pascals etc.
 */


/*struct Structure {
    /* This struct contains all the parameters needed to calculate
    apply the fem to the structure
        */
}
*/

fn read_input() -> String {
    let mut buffer = String::new();
    io.stdin().
        read_line(&mut buffer).
        expect("Failed to read line");
    buffer
}

pub struct Rod {
    /* this struct represent a rod element:
        length: the length of the rod;
        alpha: angle relative to the positive x direction
        supports: hash map with position and type of support
     */
    length: u32,
    alpha: f32,
    supports: HashMap<f32, String>,
    //beta: f32,
    //gamma: f32,
}
impl Rod {
    pub fn new(&mut self) -> Rod {
        let mut l: u32;
        let mut a: f32;

        println!("Insert dimensions: ");
        print!("Length: ");
        let l: u32 = read_input();
        print!("\nAlpha: ");
        let a: f32 = read_input();

        Rod {length: l, alpha: a, supports: self.supports}
    }
    //changes the length of the rod
    pub fn change_l(&mut self, new_length: u32) {
        self.length = new_length;
    }
    //changes the direction of the rod
    pub fn change_a(&mut self, new_alpha: f32) {
        self.alpha = new_alpha;
    }

    pub fn insert_support() {
        TODO!()
    }
}
pub struct Beam {
    /* same as rod, the loads will change */
    length: u32,
    alpha: f32,
    beta: f32,
    gamma: f32,
}

