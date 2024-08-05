pub mod interpolation;

/** dBFS => 원래 값으로 변환 */
pub fn from_dbfs(dbfs: f64) -> f64 {
    return 10.0_f64.powf(dbfs / 20.0);
}

/** 원래 값 => dBFS로 변환 */
pub fn to_dbfs(val: f64) -> f64 {
    return 20.0 * val.log10();
}

// vec의 원소를 하나하나 지워가면서 반복
pub struct VecIter<T> {
    vec: Vec<T>
}

impl<T> VecIter<T> {
    pub fn new(vec: Vec<T>) -> Self {
        return Self { vec: vec };
    }
}

impl<T> std::iter::Iterator for VecIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.vec.len() == 0 {
            return None;
        } else {
            return Some(self.vec.remove(0));
        }
    }
}

// Result<T,E> 형의 iterator에서
// borrow 문제를 해결하면서 unwrap 없이 Result를 벗김
pub fn unwrap_result_iter<T, E, I: std::iter::Iterator<Item = Result<T, E>>>(
    iter: I
) -> Result<VecIter<T>, E> {
    let mut vec: Vec<T> = vec![];
    for child in iter {
        match child {
            Ok(a) => vec.push(a),
            Err(e) => return Err(e)
        }
    }
    return Ok(VecIter::new(vec));
}