#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::fmt;

struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    fn new() -> Vector {
        Vector {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

struct Matrix {
    vals: Vec<f64>,
}

impl Matrix {
    fn new() -> Self {
        // identity matrix:
        //
        // 1, 0, 0, 0
        // 0, 1, 0, 0
        // 0, 0, 1, 0
        // 0, 0, 0, 1
        //
        // "column orientation":
        //
        // x increases downward, y increases rightward
        //
        // (0, 0) top left
        // (3, 0) bottom left
        // (0, 3) top right
        // (3, 3) bottom right

        let mut m = Matrix { vals: vec![0.; 16] };

        m.setval(0, 0, 1. as f64);
        m.setval(0, 1, 0. as f64);
        m.setval(0, 2, 0. as f64);
        m.setval(0, 3, 0. as f64);

        m.setval(1, 0, 0. as f64);
        m.setval(1, 1, 1. as f64);
        m.setval(1, 2, 0. as f64);
        m.setval(1, 3, 0. as f64);

        m.setval(2, 0, 0. as f64);
        m.setval(2, 1, 0. as f64);
        m.setval(2, 2, 1. as f64);
        m.setval(2, 3, 0. as f64);

        m.setval(3, 0, 0. as f64);
        m.setval(3, 1, 0. as f64);
        m.setval(3, 2, 0. as f64);
        m.setval(3, 3, 1. as f64);

        m
    }

    fn translation_m(x: f64, y: f64, z: f64) -> Self {
        let mut m = Matrix::new();

        m.setval(0, 3, x);
        m.setval(1, 3, y);
        m.setval(2, 3, z);

        m
    }

    fn rotation_x_m(ax: f64) -> Self {
        let mut m = Matrix::new();

        m.setval(1, 1, (-ax).cos());
        m.setval(1, 2, -(-ax).sin());
        m.setval(2, 1, (-ax).sin());
        m.setval(2, 2, (-ax).cos());

        m
    }

    fn rotation_y_m(ay: f64) -> Self {
        let mut m = Matrix::new();

        m.setval(0, 0, (-ay).cos());
        m.setval(0, 2, (-ay).sin());
        m.setval(2, 0, -(-ay).sin());
        m.setval(2, 2, (-ay).cos());

        m
    }

    fn rotation_z_m(az: f64) -> Self {
        let mut m = Matrix::new();

        m.setval(0, 0, (-az).cos());
        m.setval(0, 1, -(-az).sin());
        m.setval(1, 0, (-az).sin());
        m.setval(1, 1, (-az).cos());

        m
    }

    fn scaling_m(sx: f64, sy: f64, sz: f64) -> Self {
        let mut m = Matrix::new();

        m.setval(0, 0, sx);
        m.setval(1, 1, sy);
        m.setval(2, 2, sz);

        m
    }

    fn transpose(&mut self) -> &Self {
        for i in 0..4 {
            for j in 0..4 {
                let val = self.get(j, i);
                self.setval(i, j, val);
            }
        }

        self
    }

    fn setval(&mut self, row: usize, col: usize, val: f64) {
        let dim: usize = 4;
        let i = row * dim + col;
        self.vals[i as usize] = val
    }

    fn get(&self, row: usize, col: usize) -> f64 {
        let dim = 4;
        let i = row * dim + col;
        return self.vals[i as usize];
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // we need a string literal here
        write!(
            f,
            "\n {} {} {} {} \n {} {} {} {} \n {} {} {} {} \n {} {} {} {} ",
            self.get(0, 0),
            self.get(0, 1),
            self.get(0, 2),
            self.get(0, 3),
            self.get(1, 0),
            self.get(1, 1),
            self.get(1, 2),
            self.get(1, 3),
            self.get(2, 0),
            self.get(2, 1),
            self.get(2, 2),
            self.get(2, 3),
            self.get(3, 0),
            self.get(3, 1),
            self.get(3, 2),
            self.get(3, 3)
        )
    }
}


fn cross_product(a: &Vector, b: &Vector) -> Vector {
    let v = Vector {
        x: (a.y * b.z) - (b.y * a.z),
        y: (a.z * b.x) - (b.z * a.x),
        z: (a.x * b.y) - (b.x * a.y),
    };
    v
}

fn dot_product(a: &Vector, b: &Vector) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

fn multiply_m(m1: &Matrix, m2: &Matrix) -> Matrix {
    let mut m = Matrix::new();

    for i in 0..4 {
        for j in 0..4 {
            let val = m1.get(i, 0) * m2.get(0, j) + m1.get(i, 1) * m2.get(1, j)
                + m1.get(i, 2) * m2.get(2, j) + m1.get(i, 3) * m2.get(3, j);
            m.setval(i, j, val)
        }
    }
    m
}

fn transform_v(v: &Vector, m: &Matrix) -> Vector {
    let ret = Vector::new();
    let mut vals = vec![0; 4];
    
    for i in 0..4 {
        //vals[i] = 

    }
    ret
}

fn main() {
    let a = Vector {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let b = Vector {
        x: 2.0,
        y: 2.0,
        z: 3.0,
    };
    let result = dot_product(&a, &b);
    println!("It's a dot product, everybody: {}", result);
    let result2 = cross_product(&a, &b);
    println!("It's a cross product, everybody: {}", result2);

    let m1 = Matrix::new();
    println!("It's the identity matrix, everybody: {}", m1);
    let m2 = Matrix::translation_m(3., 3., 3.);
    println!("It's the translation matrix, everybody: {}", m2);
    let m3 = Matrix::scaling_m(3., 3., 3.);
    println!("It's the scaling matrix, everybody: {}", m3);
}
