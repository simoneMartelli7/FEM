extern crate nalgebra as na;

pub const pi: f32 = std::f32::consts::PI;

pub fn lam(theta: f32) -> na::Matrix3::<f32>{
    let c:f32 = theta.cos();
    let s:f32 = theta.sin();

    let mut lam: na::Matrix3::<f32> = na::Matrix3::new(c.powi(2), s.powi(2), -2.0*c*s,
                                                           s.powi(2), c.powi(2), 2.0*c*s,
                                                           c*s, -c*s, c.powi(2)-s.powi(2));

    lam
}

pub fn q_p(E1: f32, E2: f32, nu1: f32, nu2: f32, G: f32) -> na::Matrix3::<f32> {
    let mut q_p: na::Matrix3::<f32> = na::Matrix3::new(E1, nu1*E2, 0.0,
                                                           nu1*E2, E2, 0.0,
                                                            0.0, 0.0, G);
    1.0/(1.0-nu1*nu2) * q_p
}


pub struct stratum{
    E1: f64,  //Young Modulus in the fibre direction
    E2: f64,  //Young Modulus in the direction trasversal to the fibre
    nu1: f64, //poisson coefficients
    nu2: f64,
    G: f64,   //shear coefficient
    theta: f64, //angle of the fibre
    q: na::Matrix3::<f32>,

}

impl stratum {
    pub fn q_p(& mut self) {
        q_p(&*self.E1, &*self.E2, &*self.nu1, &*self.nu2, &*self.G);
        lam = lam(&*self.theta);
        self.q = lam.inverse() * q_p * lam.inverse().transpose();
    }
}