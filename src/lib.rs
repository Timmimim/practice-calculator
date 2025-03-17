use std::{f64::INFINITY};

use num_traits::{self, Pow};

pub fn factorial (number:u64) -> u64 {
    if number < 2 {
        1
    } else {
        number * factorial(number - 1)
    }
}

#[derive(Debug)]
pub enum MathError {
    DivisionByZero, 
    NonPositiveLogarithm,
    NegativeSquareRoot,
}

pub type MathResult = Result<f64, MathError>;

pub struct Calculator {
    param_a : f64,
    param_b : f64,
}
impl Calculator {
    // constructor
    /**
     * Overdid it here; NumCast appears incompatible with Option type
     */
    // pub fn new<T : num_traits::NumCast, S : num_traits::NumCast>(param_a:T, param_b:Option<S>) -> Self {
    //     Self {
    //         param_a : num_traits::cast(param_a).unwrap(),
    //         param_b : match param_b {
    //             Some(value) => num_traits::cast(value).unwrap(),
    //             None => INFINITY,
    //         }
    //     }
    // }
    pub fn new(param_a:f64, param_b:Option<f64>) -> Self {
        Self {
            param_a : num_traits::cast(param_a).unwrap(),
            param_b : match param_b {
                Some(value) => num_traits::cast(value).unwrap(),
                None => INFINITY,
            }
        }
    }

    pub fn get_param_a(&self) -> f64 {
        self.param_a
    }

    pub fn get_param_b(&self) -> f64 {
        self.param_b
    }

    pub fn add(&self) -> f64 {
        assert_ne!(self.param_b, INFINITY, "no second parameter given, or passed as INFINITY");
        self.param_a + self.param_b
    }

    pub fn subtract(&self) -> f64 {
        assert_ne!(self.param_b, INFINITY, "no second parameter given, or passed as INFINITY");
        self.param_a - self.param_b
    }

    pub fn multiply(&self) -> f64 {
        assert_ne!(self.param_b, INFINITY, "no second parameter given, or passed as INFINITY");
        self.param_a * self.param_b
    }

    pub fn divide(&self) -> MathResult {
        assert_ne!(self.param_b, INFINITY, "no second parameter given, or passed as INFINITY");
        if self.param_b == 0. {
            Err(MathError::DivisionByZero)
        } else {
            Ok(self.param_a / self.param_b)
        }
    }

    pub fn modulo(&self) -> Result<f64,&str> {
        assert_ne!(self.param_b, INFINITY, "no second parameter given, or passed as INFINITY");
        if self.param_b == 0.0 {
            return Err("X mod 0 is undefined")
        }
        Ok(self.param_a % self.param_b)
    }

    pub fn facto(&self) -> f64 {
        let param_a_as_int : u64 = self.param_a as u64;
        assert_eq!(self.param_a, param_a_as_int as f64, "factorial is only implemented for positive int types (no decimals)");
        let res = factorial(param_a_as_int);
        res as f64
    }

    pub fn square(&self) -> f64 {
        self.param_a.pow(2)
    }

    pub fn pow(&self) -> f64 {
        assert_ne!(self.param_b, INFINITY, "no second parameter given, or passed as INFINITY");
        self.param_a.pow(self.param_b)
    }

    pub fn sqrt(&self) -> MathResult {
        if self.param_a < 0. {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(f64::sqrt(self.param_a))
        }
    }

}