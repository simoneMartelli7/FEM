use std::collections::HashMap;
use std::convert::TryFrom;
extern crate nalgebra as na;
/* This is a test as a proof of concept. It's a hard coded implementation of a FEM analysis on a
   single rod element
 */

fn main() {
    //length of the rod
    let l: f32 = 1.500;

    /* supports:
        the first element is a String and represent the type of support
        the second one represent where it's located
     */
    let mut p: HashMap<String, f32> = HashMap::new();
    p.insert(String::from("Vertical"), 0.0);
    //size of the mesh: this is the number of nodes which is number of elements + 1
    const N: usize = 3;
    //this creates a vector containing only zeros where the position of the various nodes will go
    let mesh: na::SVector<f32, N> = na::SVector::zeros();


    /*applied load:
        Vec<String, f32> contains:
            String: type of load: Vertical, Horizontal, Torque
            f32:    position in which is applied
        f32: entity of the load N.B: this is a signed value: so a compression load needs to be negative
     */
    /*let mut loads: HashMap<(String, f32), f32> = HashMap::new();
    loads.insert((String::from("Horizontal"), l), -650.0);*/


    //some rod properties
    let E: f32 = 73000.0;
    let S: f32 = 0.050;

    //this next snippet of code will be the function which will create the mesh
    let n = u32::try_from(N).unwrap(); //disposable N
    let basic_element: f32 = l/n;

    for i in 0..n {
        let ni = i.as_f64();
        mesh[i] = i*basic_element;
    }

    println!("{:?}", mesh);
}
