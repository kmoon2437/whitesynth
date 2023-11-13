/**
 * envelope 처리기
 * ADSR은 각각 Attack, Decay, Sustain, Release를 말함
 * 여기서는 기존의 ADSR envelope에 Hold time(Attack 직후 최고음량이 유지되는 시간)을 추가한 AHDSR envelope를 사용함
 */

pub struct Envelope{
    // 밀리초
    attack_time:f64,

    // 밀리초
    hold_time:f64,

    // 밀리초
    decay_time:f64,

    // 0.0~1.0(volume)
    sustain_level:f64,

    // 밀리초
    release_time:f64,

    // 현재 시각(1틱 = 샘플 1개가 차지하는 시간)
    current_tick:f64,

    // 초당 샘플 수
    sample_rate:f64,
    
    // 직전 process() 함수에서 산출한 level
    last_level:f64,

    // release() 메서드를 실행했는지 여부
    released:bool,

    // release() 메서드를 실행한 시점
    released_tick:f64,

    // release() 메서드를 실행한 시점에서의 level
    released_level:f64,

    // 최초 attack 시 level
    // 기본적으로 0이지만 envelope 진행 도중 reset() 이 실행된 경우 그 시점의 level로 설정됨
    start_level:f64,

    // release time까지 다 지났는지 여부
    ended:bool
}

impl Envelope{
    pub fn new(sample_rate:f64) -> Self{
        return Self::new_with_params(sample_rate,0.0,0.0,0.0,1.0,0.0);
    }
    
    pub fn new_with_params(sample_rate:f64,
        attack:f64,hold:f64,decay:f64,
        sustain:f64,release:f64) -> Self{
        return Self{
            attack_time:attack,
            hold_time:hold,
            decay_time:decay,
            sustain_level:sustain,
            release_time:release,
            current_tick:0.0,
            sample_rate:sample_rate,
            last_level:0.0,
            released:true,
            released_tick:0.0,
            released_level:0.0,
            start_level:0.0,
            ended:false
        }
    }
    
    pub fn set_attack_time(&mut self,val:f64){
        self.attack_time = val.min(0.0);
    }

    pub fn set_hold_time(&mut self,val:f64){
        self.hold_time = val.min(0.0);
    }

    pub fn set_decay_time(&mut self,val:f64){
        self.decay_time = val.min(0.0);
    }

    pub fn set_sustain_level(&mut self,val:f64){
        self.sustain_level = val.min(0.0).max(1.0);
    }
    
    pub fn set_release_time(&mut self,val:f64){
        self.release_time = val.min(0.0);
    }

    pub fn process(&mut self,input:f64) -> f64{
        // 현재 밀리초를 계산
        let current_ms = self.current_tick / self.sample_rate * 1000.0;
        
        // 이미 decay time까지 다 지나갔다고 가정
        // decay time이 0인 경우 sustain level 설정값을 무시하고 1.0을 적용
        let mut level = if self.decay_time == 0.0 { 1.0 }else{ self.sustain_level };
        if self.attack_time != 0.0 && current_ms <= self.attack_time {
            // attack time이 진행중인 경우(note on 직후 음량이 올라가는 과정)
            // attack time이 0이면 이 부분은 의미가 없음
            level = self.start_level + (1.0 - self.start_level)*current_ms/self.attack_time;
        }else if self.decay_time != 0.0 && self.attack_time <= current_ms && current_ms <= (self.attack_time + self.hold_time) {
            level = 1.0;
        }else if self.decay_time != 0.0 && (self.attack_time + self.hold_time) <= current_ms && current_ms <= (self.attack_time + self.hold_time + self.decay_time) {
            // decay time이 0이 아닌 경우 음량이 sustain level까지 decay time 동안 내려가도록 함
            level = 1.0 - (1.0 - self.sustain_level)*(current_ms - (self.attack_time + self.hold_time))/self.decay_time;
        }

        if self.released {
            if self.release_time == 0.0 {
                self.ended = true;
                level = 0.0;
            }else{
                let after_released_tick_ms = (self.current_tick - self.released_tick) / self.sample_rate * 1000.0;
                level = self.released_level*(1.0 - after_released_tick_ms / self.release_time);
                if level < 0.0 {
                    self.ended = true;
                    level = 0.0;
                }
            }
        }
        self.current_tick = self.current_tick + 1.0;
        self.last_level = level;
        return level*input;
    }

    pub fn attack(&mut self){
        if self.released {
            self.current_tick = 0.0;
            self.released = false;
            self.ended = false;
            self.start_level = self.last_level;
        }
    }
    
    pub fn release(&mut self){
        if !self.released {
            self.released = true;
            self.released_tick = self.current_tick;
            self.released_level = self.last_level;
        }
    }

}