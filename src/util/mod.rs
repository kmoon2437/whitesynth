/**
 * 각종 보조 함수
 */

pub mod midi;

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

pub fn mix_samples(s1:f64,s2:f64) -> f64{
    if s1 > 0.0 && s2 > 0.0 {
        return (s1+s2)-(s1*s2);
    }else if s1 < 0.0 && s2 < 0.0 {
        return (s1+s2)+(s1*s2);
    }else{
        return s1+s2;
    }
}