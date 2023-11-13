/**
 * ring buffer
 * (대충 버퍼인데 고리모양으로 뺑뺑 도는 거)
 * 참고문헌: https://www.utsbox.com/?p=1505
 */
pub const MAX_INTERVAL_SEC:usize = 4;
pub const MAX_INTERVAL_SEC_F64:f64 = MAX_INTERVAL_SEC as f64;

pub struct RingBuffer{
    ri:usize, // 읽기 시 index
    wi:usize, // 쓰기 시 index
    
    buf:Vec<f64>, // 샘플 버퍼
    buf_length:usize // 버퍼 최대 길이
}

impl RingBuffer{
    pub fn new(sample_rate:usize) -> Self{
        let length = sample_rate * MAX_INTERVAL_SEC;
        return Self{
            ri:0,
            wi:length / 2,
            buf:vec![0.0;length],
            buf_length:length,
        }
    }
    
    // 읽기 위치와 쓰기 위치의 간격을 설정
    pub fn set_interval(&mut self,mut interval:usize){
        // interval이 buf_length를 넘거나 1미만으로 내려가지 않도록 함
        interval = interval % self.buf_length;
        if interval < 1 { interval = 1; }
        
        // 쓰기 위치를 읽기 위치로부터 interval만큼 떨어뜨리긴 하는데,
        // 그 쓰기 위치가 buf_length를 넘을 수 있으므로 넘지 않도록 함
        self.wi = (self.ri + interval) % self.buf_length;
    }
    
    // 읽기 위치로부터 i만큼 떨어진 곳으로부터 데이터를 읽음
    pub fn read(&self,i:usize) -> f64{
        let tmpi = (self.ri + i) % self.buf_length;
        return self.buf[tmpi];
    }
    
    // 쓰기 위치에 데이터를 기록
    pub fn write(&mut self,smpl:f64){
        self.buf[self.wi] = smpl;
    }
    
    // 읽기/쓰기 위치를 1씩 증가시킴
    // 최대치에 이르는 경우
    // 길이로 나눈 나머지가 0이 되므로
    // 다시 처음으로 돌아감
    // 따라서 우리가 상상하는 고리 모양이 나옴
    pub fn next(&mut self){
        self.ri = (self.ri + 1) % self.buf_length;
        self.wi = (self.wi + 1) % self.buf_length;
    }
}