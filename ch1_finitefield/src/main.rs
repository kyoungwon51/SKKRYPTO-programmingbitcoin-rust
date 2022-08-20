//extern crate num;

//use std::intrinsics::powif32;
//use std::intrinsics::powif64;
use std::ops::*;
use num::pow;
use num::One;

//impl One for usize;
#[derive(Debug, Clone, Copy)]
struct FieldElement {
    num: i32,
    prime: i32,
}

// impl FieldElement {
//     pub fn new() -> Self {

//     }
// }

// FieldElement + FieldElement
impl Add<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn add(self, other:Self) -> Self {
        Self {
            num: (self.num + other.num) % self.prime,
            prime: self.prime 
        }
    }
}
// FieldElement - FieldElement
impl Sub<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn sub(self, other:Self) -> Self {
        Self {
            num: (self.num - other.num) % self.prime + self.prime,
            prime: self.prime 
        }
    }
}


// FieldElement * FieldElement
impl Mul<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn mul(self, other:Self) -> Self {
        Self {
            num: (self.num * other.num) % self.prime,
            prime: self.prime 
        }
    }
}
//impl One for FieldElement {}
// impl FieldElement {
//     pub fn pow<FieldElement: Clone + One + Copy>(&mut self, exp: usize) -> Self {
//         Self { 
//             num: pow(self.num, exp) % self.prime,
//             prime: self.prime 
//         }
//     }
// }

// FieldElement / FieldElement
impl Div<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn div(self, other:Self) -> Self {
        Self {
            num: (self.num / other.num) % self.prime,
            prime: self.prime 
        }
    }
}

impl One for FieldElement {
    fn one() -> Self {
        Self {
            num: 1,
            prime: 1
        }
    }
}

// impl FieldElement {
//     fn pow2(&self, exp: i32) -> Self {
//         if exp > 0 {
//             Self {
//                 num: pow(self.num, exp),
//                 prime: self.prime
//             }
//         }
//         else {
//             Self {
//                 num: pow(&self, exp),
//                 prime: self.prime
//             }
//         }
//     }
// }

fn main() {
    let a = FieldElement{num: 2, prime: 31};
    let b = FieldElement{num: 19, prime: 31};
    println!("{:#?}", a + b);
    println!("{:#?}", a - b);
    println!("{:#?}", a * b);
    println!("{:#?}", pow(a, 3));
}
