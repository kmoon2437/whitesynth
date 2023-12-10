/**
 * 각종 보조 함수
 */

pub mod midi;
pub mod synth;

// 배열 비교
pub fn compare_array<T: std::cmp::PartialEq>(arr1: &[T],arr2: &[T]) -> bool{
    if arr1.len() != arr2.len() {
        return false;
    }
    
    for i in 0..arr1.len() {
        if arr1[i] != arr2[i] {
            return false;
        }
    }
    
    return true;
}

// vec의 원소를 하나하나 지워가면서 반복
pub struct Iter<T>{
    vec:Vec<T>
}

impl<T> Iter<T>{
    pub fn new(vec:Vec<T>) -> Self{
        return Self{ vec:vec };
    }
}

impl<T> std::iter::Iterator for Iter<T>{
    type Item = T;
    fn next(&mut self) -> Option<T>{
        if self.vec.len() == 0 {
            return None;
        }else{
            return Some(self.vec.remove(0));
        }
    }
}

// Result<T,E> 형의 iterator에서
// unwrap() 없이 Result를 벗김
pub fn unwrap_result_iter<T,E,I:std::iter::Iterator<Item = Result<T,E>>>(iter:I) -> Result<Iter<T>,E>{
    let mut vec:Vec<T> = vec![];
    for child in iter {
        match child {
            Ok(a) => vec.push(a),
            Err(e) => return Err(e)
        }
    }
    return Ok(Iter{ vec:vec });
}