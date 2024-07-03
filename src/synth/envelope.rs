/**
 * envelope 처리기
 * ADSR은 각각 Attack, Decay, Sustain, Release를 말함
 * 
 * 여기서는 기존의 ADSR envelope에다가
 * Delay time(attack() 함수 실행 후 실제 Attack이 시작되기 전까지의 시간 간격)과
 * Hold time(Attack 직후 최고음량이 유지되는 시간)
 * 을 추가한 형태의 envelope를 사용함
 */

use crate::util::from_dbfs;

#[derive(PartialEq)]
enum EnvelopeStatus {
    Waiting, // 시작 대기 중
    Delay, // 여기부터 타이머 시작
    Attack,
    Hold,
    Decay,
    Sustain,
    Released,
    Finished // 타이머 정지(다시 시작하면 0으로 돌아감)
}

pub enum EnvelopeMode {
    Normal, DLS
}

pub struct Envelope {
    // tick = 1샘플 분량의 시간
    // 어차피 처리하는 동안 sample rate는 변하지 않으므로
    // 연산량을 조금이라도 줄이기 위해 이렇게 sample 단위로 계산을 함
    delay_tick: f64,
    attack_tick: f64,
    hold_tick: f64,
    decay_tick: f64,
    sustain_level: f64,
    release_tick: f64,

    // 초당 샘플 수
    sample_rate: f64,
    
    // decay time과 release time의 기준을 설정
    // Normal => decay time은 1.0에서 sustain level까지 줄어드는 시간, release time은 sustain level에서 0.0까지 줄어드는 시간
    // DLS => decay time과 release time이 모두 1.0에서 0.0까지 줄어드는 시간이라고 가정함(DLS 표준 공식 문서 참조)
    mode: EnvelopeMode,

    // 현재 시각(1틱 = 샘플 1개가 차지하는 시간)
    current_tick: f64,

    // 현재 level
    current_level: f64,

    // 현재 적용되는 볼륨 변화량
    slope: f64,

    // decay가 시작되는 시간
    // 역시나 조금이라도 연산량을 줄이기 위해 미리 연산을 해 놓음
    decay_starts_at: f64,

    // 현재 envelope 상태
    status: EnvelopeStatus
}

impl Envelope {
    pub fn new(sample_rate: f64, mode: EnvelopeMode) -> Self {
        return Self::with_params(sample_rate, mode, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    }

    pub fn with_params(
        sample_rate: f64,
        mode: EnvelopeMode,
        delay: f64,
        attack: f64, hold: f64,
        decay: f64, sustain: f64,
        release: f64
    ) -> Self {
        let mut this = Self {
            delay_tick: 0.0,
            attack_tick: 0.0,
            hold_tick: 0.0,
            decay_tick: 0.0,
            sustain_level: 1.0,
            release_tick: 0.0,

            sample_rate, mode,
            current_tick: 0.0,
            current_level: 0.0,
            slope: 0.0,
            decay_starts_at: 0.0,
            status: EnvelopeStatus::Waiting
        };

        this.reset();
        this.set_delay_time(delay);
        this.set_attack_time(attack);
        this.set_hold_time(hold);
        this.set_decay_time(decay);
        this.set_sustain_level(sustain);
        this.set_release_time(release);

        return this;
    }

    // 모든 것을 처음 상태로 초기화
    pub fn reset(&mut self) {
        self.current_tick = 0.0;
        self.status = EnvelopeStatus::Waiting;
        self.current_level = 0.0;
        self.slope = 0.0;
    }

    pub fn set_delay_time(&mut self, val: f64) {
        self.delay_tick = self.ms2tick(val.max(0.0));
    }

    pub fn set_attack_time(&mut self, val: f64) {
        self.attack_tick = self.ms2tick(val.max(0.0));
        self.decay_starts_at = self.attack_tick + self.hold_tick;
    }

    pub fn set_hold_time(&mut self, val: f64) {
        self.hold_tick = self.ms2tick(val.max(0.0));
        self.decay_starts_at = self.attack_tick + self.hold_tick;
    }

    pub fn set_decay_time(&mut self, val: f64) {
        self.decay_tick = self.ms2tick(val.max(0.0));
    }

    pub fn set_sustain_level(&mut self, val: f64) {
        self.sustain_level = val.max(0.0).min(1.0);
    }

    pub fn set_release_time(&mut self, val: f64) {
        self.release_tick = self.ms2tick(val.max(0.0));
    }

    fn tick2ms(&self, tick: f64) -> f64 {
        return tick / self.sample_rate * 1000.0;
    }

    fn ms2tick(&self, ms: f64) -> f64 {
        return ms / 1000.0 * self.sample_rate;
    }

    // 샘플 count개분의 시간만큼 진행
    pub fn process(&mut self, count: usize) {
        if self.status == EnvelopeStatus::Waiting || self.status == EnvelopeStatus::Finished {
            return;
        }
        for _ in 0..count {
            if self.status == EnvelopeStatus::Waiting || self.status == EnvelopeStatus::Finished {
                continue;
            }

            self.current_level += self.slope;

            if self.status == EnvelopeStatus::Delay && self.current_tick >= 0.0 {
                self.slope = 1.0 / self.attack_tick;
                self.status = EnvelopeStatus::Attack;
            } else if self.status == EnvelopeStatus::Attack && self.current_level >= 1.0 {
                self.current_level = 1.0;
                self.slope = 0.0;
                self.status = EnvelopeStatus::Hold;
            } else if self.status == EnvelopeStatus::Hold && self.current_tick >= self.decay_starts_at {
                self.slope = match self.mode {
                    EnvelopeMode::Normal => (self.sustain_level - 1.0) / self.decay_tick,
                    EnvelopeMode::DLS => -1.0 / self.decay_tick
                };
                self.status = EnvelopeStatus::Decay;
            } else if self.status == EnvelopeStatus::Decay && self.current_level <= self.sustain_level {
                self.current_level = self.sustain_level;
                self.slope = 0.0;
                self.status = EnvelopeStatus::Sustain;
            } else if self.status == EnvelopeStatus::Released && self.current_level <= 0.0 {
                self.current_level = 0.0;
                self.slope = 0.0;
                self.status = EnvelopeStatus::Finished;
            }

            self.current_tick += 1.0;
        }
    }

    pub fn get_level(&self) -> f64 {
        return self.current_level;
    }

    pub fn get_log_scale_level(&mut self) -> f64 {
        if self.current_level <= 0.0 {
            return 0.0;
        } else {
            return from_dbfs(self.current_level * 100.0 - 100.0);
        }
    }

    pub fn attack(&mut self) {
        self.current_tick = -self.delay_tick;
        self.status = EnvelopeStatus::Delay;
        self.current_level = 0.0;
        self.slope = 0.0;
    }
    
    pub fn release(&mut self) {
        self.slope = match self.mode {
            EnvelopeMode::Normal => -self.sustain_level / self.release_tick,
            EnvelopeMode::DLS => -1.0 / self.release_tick
        };
        self.status = EnvelopeStatus::Released;
    }
}