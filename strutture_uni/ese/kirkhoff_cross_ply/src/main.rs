use utils;
use utils::{Stratum, Plate4};
extern crate nalgebra as na;

fn main() {

    let E_t: f32 = 8.0*10.0_f32.powi(9);
    let E_l: f32 = 1.11*10.0_f32.powi(11);
    let G: f32 = 3.0*10.0_f32.powi(9);
    let nu_lt: f32 = 0.33;
    let nu_tl: f32 = nu_lt*E_t/E_l;
    let theta: f32 = 0.0;


    let test1 = utils::Plate4::new_omogeneous();
    let d = test1.b_mat();
    println!("{}", d);

/*    let s_test = Stratum {E1: E_l, E2: E_t, nu1: nu_lt, nu2: nu_tl, G: G, zb: 0.0, zt: 1.0, theta: utils::pi/2.0};
    let lam = utils::lam(0.0).pseudo_inverse(0.00001).unwrap();
    let q = lam * utils::q_p(E_l, E_t, nu_lt, nu_tl, G) * lam.transpose();

    println!("{}", q);*/
}
