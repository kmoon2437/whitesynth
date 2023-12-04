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

pub fn mix_two_samples(s1:f64,s2:f64) -> f64{
    if s1 > 0.0 && s2 > 0.0 {
        return (s1+s2)-(s1*s2);
    }else if s1 < 0.0 && s2 < 0.0 {
        return (s1+s2)+(s1*s2);
    }else{
        return s1+s2;
    }
}

pub fn mix_samples(samples:&[f64]) -> f64{
    let mut result:f64 = 0.0;
    for sample in samples {
        result = mix_two_samples(result,*sample);
    }
    return result;
}

// 선형 보간
// 점 A(v1),B(v2) 에 대하여
// 선분 AB를 t : 1 - t (0 <= t <= 1) 로 내분하는 지점을 반환
pub fn lerp(v1:f64,v2:f64,mut t:f64) -> f64{
    t = t.max(0.0).min(1.0);
    return (1.0 - t) * v1 + t * v2;
}