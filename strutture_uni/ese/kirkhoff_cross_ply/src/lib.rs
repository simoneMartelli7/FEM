extern crate nalgebra as na;
use std::io;

////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////   GENERAL UTILITIES   /////////////////////////////////////////////////////
pub const pi: f32 = std::f32::consts::PI;
pub fn read_f32(a: Option<String>) -> f32 {
    println!("{}", a.unwrap_or("Insert input: ".to_string()));
    let mut buffer = String::new();
    io::stdin().
        read_line(&mut buffer).
        expect("Failed to read line");
    let buffer2: f32 = buffer.trim().parse().unwrap();
    buffer2
}

//returns the rotation matrix given the rotation angle theta
pub fn lam(theta: f32) -> na::Matrix3::<f32>{
    let c:f32 = theta.cos();
    let s:f32 = theta.sin();

    let lam: na::Matrix3::<f32> = na::Matrix3::new(c.powi(2), s.powi(2), -2.0*c*s,
                                                           s.powi(2), c.powi(2), 2.0*c*s,
                                                           c*s, -c*s, c.powi(2)-s.powi(2));

    lam
}


// returns the rigidity matrix
pub fn q_p(E1: f32, E2: f32, nu1: f32, nu2: f32, G: f32) -> na::Matrix3::<f32> {
    let  q_p: na::Matrix3::<f32> = na::Matrix3::new(E1, nu1*E2, 0.0,
                                                           nu1*E2, E2, 0.0,
                                                            0.0, 0.0, G * (1.0 + nu1*nu2));
    1.0/(1.0-nu1*nu2) * q_p
}

////////////////////////////////////////////////////////////////////////////////////////////////////


// represent a stratum in a plate
pub struct Stratum{
    E1: f32,  //Young Modulus in the fibre direction
    E2: f32,  //Young Modulus in the direction trasversal to the fibre
    nu1: f32, //poisson coefficients
    nu2: f32,
    G: f32,   //shear coefficient
    theta: f32, //angle of the fibre
    q: na::Matrix3::<f32>,
    zt: f32,
    zb: f32,

}

impl Stratum {

    pub fn print_e1(&self) {
        println!("Longitudinal Young Modulus: {}", self.E1)
    }
    //creates a Stratum struct from user input
    pub fn new() -> Stratum {

        println!("Please insert the relevant mechanical characteristics when prompted.\nOnly SI Units!!\n");

        //reads inputs from user
        let e1: f32 = read_f32(Some("Longitudinal Young Modulus: ".to_string()));
        let e2: f32 = read_f32(Some("Trasversal Young Modulus: ".to_string()));
        let nu1: f32 = read_f32(Some("l-t Poisson Modulus: ".to_string()));
        let g: f32 = read_f32(Some("Shear Modulus: ".to_string()));
        let nu2: f32 = nu1 * e1/e2;
        let theta: f32 = read_f32(Some("Lamination angle: ".to_string()));

        let qp = q_p(e1, e2, nu1, nu2, g);
        let lam = lam(theta).pseudo_inverse(0.00001).unwrap();
        let q = lam * qp * lam.transpose();

        let zb: f32 = read_f32(Some("z bottom: ".to_string()));
        let zt: f32 = read_f32(Some("z top: ".to_string()));

        Stratum { E1: e1, E2: e2, nu1: nu1, nu2: nu2, G: g, theta: theta, q: q, zb: zb, zt: zt}

    }
    pub fn new_from_stratum(v: &Vec<f32>) -> Stratum  {
        let zb: f32 = read_f32(Some("z bottom: ".to_string()));
        let zt = read_f32(Some("z top: ".to_string()));
        let theta: f32 = read_f32(Some("Lamination angle: ".to_string()));
        let qp: na::Matrix3::<f32> = q_p(v[0], v[1], v[2], v[3], v[4]);
        let lam = lam(theta).pseudo_inverse(0.00001).unwrap();
        let q = lam * qp * lam.transpose();
        let s_new = Stratum {E1: v[0], E2: v[1], nu1: v[2], nu2: v[3], q: q, zb: zb, zt: zt, G: v[4], theta: theta};
        s_new
    }


    // the following are three functions only needed in order to not have redundant code in the
    // functions of the same name for the Plate4 struct
    pub fn a_mat(&self) -> na::Matrix3::<f32> {
        let h: f32 = self.zt - self.zb;
        let a = self.q * h;
        a
    }
    pub fn b_mat(&self) -> na::Matrix3::<f32> {
        let b = self.q * (self.zt.powi(2) - self.zb.powi(2));
        b
    }
    pub fn d_mat(&self) -> na::Matrix3::<f32> {
        let d = self.q * (self.zt.powi(3) - self.zb.powi(3));
        d
    }
}

pub struct Plate4{
    // s_i are the single strata of the plate, each is a Stratum, a, b and h are the dimensions
    s1: Stratum,
    s2: Stratum,
    s3: Stratum,
    s4: Stratum,
    h: f32
    //theta: na::SVector<f32, 4>
}

impl Plate4 {
    pub fn new() -> Plate4 {
        println!("\nFirst stratum: ");
        let mut s1: Stratum = Stratum::new();
        println!("\nSecond stratum: ");
        let mut s2: Stratum = Stratum::new();
        println!("\nThird stratum: ");
        let mut s3: Stratum = Stratum::new();
        println!("\nFourth stratum: ");
        let mut s4: Stratum = Stratum::new();

        let h: f32 = s4.zt - s1.zb;

        Plate4 { s1: s1, s2: s2, s3: s3, s4: s4, h: h}
    }

    //creates a plate where each strata is of the same material and only the lamination angles are
    //different
    pub fn new_omogeneous() -> Plate4 {
        println!("Please insert material charachteristics: ");
        let s1: Stratum = Stratum::new();

        // temporary variables for use in new_from_stratum
        let mut temp = vec![];
        temp.push(s1.E1);
        temp.push(s1.E2);
        temp.push(s1.nu1);
        temp.push(s1.nu2);
        temp.push(s1.G);

        println!("Second stratum: ");
        let s2: Stratum = Stratum::new_from_stratum(&temp);
        println!("Third stratum: ");
        let s3: Stratum = Stratum::new_from_stratum(&temp);
        println!("Fourth stratum: ");
        let s4: Stratum = Stratum::new_from_stratum(&temp);

        let h: f32 = 4.0*(s1.zt- s1.zb);

        Plate4 { s1: s1, s2: s2, s3: s3, s4: s4, h: h }
    }

    pub fn omogeneous_from_var(e1: f32, e2: f32, nu1: f32, nu2: f32, g: f32, theta: Vec<f32>, h: f32) -> Plate4 {
       todo!()
    }


    pub fn a_mat(&self) -> na::Matrix3::<f32> {
        let a = self.s1.a_mat() + self.s2.a_mat() + self.s3.a_mat() + self.s4.a_mat();
        a
    }

    pub fn b_mat(&self) -> na::Matrix3::<f32> {
        let b = 0.5 * ( self.s1.b_mat() + self.s2.b_mat() + self.s3.b_mat() + self.s4.b_mat());
        b
    }

    pub fn d_mat(&self) -> na::Matrix3::<f32> {
        let d = (self.s1.d_mat() + self.s2.d_mat() + self.s3.d_mat() + self.s4.d_mat()) / 3.0;
        d
    }

}





