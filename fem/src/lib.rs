use std::collections::HashMap;
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

pub fn read_input() -> String {
    let mut buffer = String::new();
    io::stdin().
        read_line(&mut buffer).
        expect("Failed to read line");
    buffer
}

pub struct Supports {
    model: String,
    vin: Vec<String>,
    position: f32,
}
impl Supports {
    pub fn new(&mut self) -> Supports {
        let mut option: u8;
        let mut postion: f32;


        println!("What support would you like to insert?");
        println!("1. Appoggio");
        println!("2. Carrello");
        println!("3. Incastro");

        option = read_input().parse().unwrap();

        match option {
            1 => {
                self.model = String::from("Appoggio");
                self.vin = vec![String::from("Vertical"), String::from("Horizontal")];
            }
            2 => {
                self.model = String::from("Carrello");
                self.vin = vec![String::from("Vertical")];
            }
            3 => {
                self.model = String::from("Incastro");
                self.vin = vec![String::from("Vertical"), String::from("Horizontal"), String::from("Rotation")];
            }
            _ => {println!("Please insert a valid number!"); /*self:new() not working check reddit post on laptop*/}
        }

        println!("In what position would you like to insert it? \n(In fraction of the length or x/L)");
        postion = read_input().parse().unwrap();
        self.position = postion;
    }
}

/* TODO: 1. check reddit post on laptop for supports::new() [recursion]
         2. fix rod and supports ::new, maybe implement a Copy trait
         3. function to apply loads
         4. function to compute forces applied by the supports
 */


pub struct Rod {
    /* this struct represent a rod element:
        length: the length of the rod;
        alpha: angle relative to the positive x direction
        supports: hash map with position and type of support
     */
    length: u32,
    alpha: f32,
    supports: Supports,
}
impl Rod {
    pub fn new(&mut self) -> Rod {
        let mut l: u32;
        let mut a: f32;

        println!("Insert dimensions: ");
        print!("Length: ");
        let l: u32 = read_input().parse().unwrap();
        print!("\nAlpha: ");
        let a: f32 = read_input().parse().unwrap();

        let mut supports = Supports::new();

        Rod { length: l, alpha: a, supports: self.supports = supports}
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
        todo!()
    }
}




