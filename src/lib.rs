use num_bigint::BigInt;
use num_traits::One;

pub struct SFib {
    past: Vec<BigInt>,
    current: BigInt,
    next: BigInt,
    index: usize,
}

impl Default for SFib {
    fn default() -> Self {
        Self {
            past: Vec::from([
                BigInt::ZERO
            ]),
            current: BigInt::one(),
            next: BigInt::from(2),
            index: 0,
        }
    }
}

impl SFib {
    fn put_next(&mut self) {
        let mut next: BigInt = BigInt::ZERO;
        for n in &self.past {
            next += n;
        }
        next += self.current.clone() + self.next.clone();
        self.past.push(self.current.clone());
        (self.current, self.next) = (self.next.clone(), next);
    }
    fn get(index: usize) -> Result<BigInt,()> {
        for (other_index, n) in Self::default().enumerate() {
            if index == other_index {
                return Ok(n);
            }
        }

        Err(())
    }
}

impl Iterator for SFib {
    type Item = BigInt;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 0 {
            let mut checked: bool = true;
            for n in &self.past {
                if !n.checked_add(&self.current).is_some() | !n.checked_add(&self.next).is_some() {
                    checked = false;
                    break;
                }
            }
            if checked && self.current.checked_add(&self.next).is_some() {
                self.put_next();
                self.index += 1;
                return Some(self.current.clone());
            }
        } else {
            self.index += 1;
            return Some(self.current.clone());
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        for (index, n) in SFib::default().enumerate() {
            println!("{}. {}",index + 1, n);
            if index > 30 {
                break;
            }
        }
    }

    #[test]
    fn get() {
        println!("{}",SFib::get(123).unwrap());
    }
}
