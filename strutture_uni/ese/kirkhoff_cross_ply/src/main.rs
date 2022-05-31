use utils;

fn main() {
/*
    let E_t: f32 = 8.0*10.0_f32.powi(9);
    let E_l: f32 = 1.11*10.0_f32.powi(11);
    let G: f32 = 3.0*10.0_f32.powi(9);
    let nu_lt: f32 = 0.33;
    let nu_tl: f32 = nu_lt*E_t/E_l;
    let theta: f32 = 0.0;
*/

    println!("Ok");
    let test1 = utils::Plate4::new_omogeneous();
    let d = test1.b_mat();
    println!("{}", d);

}
