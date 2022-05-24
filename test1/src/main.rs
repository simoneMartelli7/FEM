use std::convert::TryFrom;
extern crate nalgebra as na;
/* This is a test as a proof of concept. It's a hard coded implementation of a FEM analysis on a
   single rod element
 */



fn main() {
    //length of the rod
    let l: f32 = 1.500;

    // this are the vectors for the horizontal and vertical constraint respectively
    //working on a better solution
    let mut h: Vec<f32> = Vec::new();
    let mut v: Vec<f32> = Vec::new();

    h.push(0.0);

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
    let A: f32 = 0.050;

    //this next snippet of code will be the function which will create the mesh
    let n = u16::try_from(N).unwrap(); //disposable N for iterator
    let f32_n = f32::try_from(n).unwrap();
    let basic_element: f32 = l/(f32_n - 1.0);


    // this matrix contains in each row area and material properties of each element
    let mut cpe: na::SMatrix<f32, N, 2> = na::SMatrix::zeros();

    for i in 0..n {
        let i_f32 = f32::try_from(i).unwrap();
        let mut i_u: usize = i as usize;
        mesh[i_u] = i_f32*basic_element;
        cpe[(i_u, 0)] = A;
        cpe[(i_u, 1)] = E;
    }

    //being a single horizontal rod element all the nodes are located on a line  with coordinates
    //given by the mesh vector. In a more rigorous way a connectivity matrix would need to be
    //implemented

    let temp_s: usize = h.len() + v.len();
    const S: usize = 1;

    //this is the vector of the degrees of freedom which are constrained and so are 0
    let mut ib: na::SVector<f32, S> = na::SVector::zeros();
    let mut counter: usize = 0;
    for i in h.iter() {
        ib[counter] = (i + 1.0) * 2.0;
        counter += 1;
    }
    for i in v.iter() {
        ib[counter] = ( i + 1.0 ) * 2.0 - 1.0;
        counter += 1;
    }

    if counter != S {panic!("Critical Error: please restart program")}


    let mut Fn: na::SMatrix<f32, 1, 2> = na::SMatrix::zeros();
    Fn[(0, 0)] = 14.0;
    Fn[(0, 1)] = -65000.0;

    println!("{}", Fn)

}
