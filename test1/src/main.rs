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
    const N: usize = 7;
    //this creates a vector containing only zeros where the position of the various nodes will go
    let mut mesh: na::SVector<f32, N> = na::SVector::zeros();


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
    let n = u16::try_from(N).unwrap(); //disposable N for iterator
    let f32_n = f32::try_from(n).unwrap();
    let basic_element: f32 = l/(f32_n - 1.0);


    // this matrix contains in each row area and material properties of each element
    let mut cpe: na::SMatrix<f32, N, 2> = na::SMatrix::zero();

    for i in 0..n {
        let i_f32 = f32::try_from(i).unwrap();
        let mut i_u: usize = i as usize;
        mesh[i_u] = i_f32*basic_element;
        cpe[(i_u, 0)] = S;
        cpe[(i_u, 2)] = E;
    }

    //being a single horizontal rod element all the nodes are located on a line  with coordinates
    //given by the mesh vector. In a more rigorous way a connectivity matrix would need to be
    //implemented
    for (key, value) in &p {
        //needed to fix the dimension of ib in order to not have a dynamic vector
        //in this case may be more convoluted, but in more realistic cases the memory saving may be
        //worth the few line of codes added
        println!("");

    }
    let mut ib: na::SVector<f32, N> = na::SVector::zeros();
    for (key, value) in &p {
        match key {
            String::from("Vertical") => {
                for i in 0..n {
                    let mut i_u: usize = i as usize;
                    if mesh[i_u] == *value {
                        println!("okay")
                    }
                }
            }
            String::from("Horizontal") => {
                println!("okay")
            }
            _ => println!("please insert at least one (1) valid constraint")
        }
    }
}
