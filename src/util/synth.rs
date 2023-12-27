/**
 * 음성 합성 관련 보조 함수
 */

// 샘플 2개를 mix 한다
pub fn mix_two_samples(s1: f64, s2: f64) -> f64 {
    if s1 > 0.0 && s2 > 0.0 {
        return (s1 + s2) - (s1 * s2);
    } else if s1 < 0.0 && s2 < 0.0 {
        return (s1 + s2) + (s1 * s2);
    } else {
        return s1 + s2;
    }
}

// 샘플 여러 개를 mix 한다
pub fn mix_samples(samples: &[f64]) -> f64 {
    let mut result: f64 = 0.0;
    for sample in samples {
        result = mix_two_samples(result, *sample);
    }
    return result;
}

// 선형 보간
// 점 A(v1), B(v2) 에 대하여
// 선분 AB를 t : 1 - t (0 <= t <= 1) 로 내분하는 지점을 반환
pub fn lerp(v1: f64, v2: f64, mut t: f64) -> f64 {
    t = t.max(0.0).min(1.0);
    return (1.0 - t) * v1 + t * v2;
}