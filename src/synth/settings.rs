/**
 * synth 설정
 */

/**
 * synth를 만들 때 고정되는 설정
 * 이걸 다르게 하려면 무조건 Synth 개체를 다시 만들어야 함
 */
pub struct SynthCreateSettings {
    // 생성 가능한 최대 보조 스레드 수 설정
    pub(crate) max_worker_threads: usize, // 최소 1 (기본값 = 1)

    // 노트 1개의 최소 길이
    // note on 하자마자 바로 note off를 시전해도 최소한 이만큼은 소리가 유지됨
    pub(crate) min_note_length: i32, // 1 - 65535 (밀리초). 기본값 = 10

    // 최대 동시 발음 수
    // 256 이상 512 이하로 설정하는 걸 권장
    // (256 이하로는 부족할 수 있음. roland scva 써보니까 알겠더라)
    // (그렇다고 너무 크면 성능에 악영향을 미칠 수 있음)
    pub(crate) polyphony: usize, // 1 - 65535 (기본값 = 384)

    // 성능 최적화를 위해 일정 크기의 버퍼를 한 번에 렌더링하게 되는데, 이 버퍼의 크기를 설정
    // 함수 호출할 때 비용이 있어서 그 비용을 줄여야 더 빨라짐
    pub(crate) render_buffer_size: usize, // 1 - 2048 (기본값 = 128)
}

impl Default for SynthCreateSettings {
    fn default() -> Self {
        return Self {
            max_worker_threads: 1,
            min_note_length: 10,
            polyphony: 384,
            render_buffer_size: 128
        };
    }
}

impl SynthCreateSettings {
    pub fn new() -> Self {
        return Default::default();
    }

    pub fn set_max_worker_threads(&mut self, val: usize) {
        self.max_worker_threads = val.min(1);
    }

    pub fn set_min_note_length(&mut self, val: i32) {
        self.min_note_length = val.max(1).min(65535);
    }

    pub fn set_polyphony(&mut self, val: usize) {
        self.polyphony = val.max(1).min(65535);
    }

    pub fn set_render_buffer_size(&mut self, val: usize) {
        self.render_buffer_size = val.max(1).min(2048);
    }
}

/**
 * 최대 동시 발음 수를 초과하는 경우에
 * 없앨 소리의 우선순위 점수를 계산하는 것과 관련된 설정
 * 최대 동시 발음 수를 초과하는 note on 이벤트 발생 시에
 * 이 설정에 기초하여 계산된 우선순위 값이 가장 낮은 소리부터 없앰
 * 각각은 그냥 점수이므로 특별한 상/하한은 없음
 */
pub struct VoiceOverflowPriorityScoreSettings {
    /* (일단 처음에는 0점에서 시작) */

    // 우선 이 값을 소리가 지속된 시간(초 단위)으로 나눠 점수에 더함
    pub age: f64, // 기본값 = 1000
    
    // 그런 다음 이 값에 그 순간 소리의 음량(-1.0 - 1.0)을 곱해 점수에 더함
    pub volume: f64, // 기본값 = 500
    
    // 타악기는 보통 잘 들리므로 드럼 채널인 경우 이 값을 점수에 더함
    pub percussion: f64, // 기본값 = 4000

    // 어차피 원래 곧 없어질 소리는 감점을 함
    pub released: f64, // 기본값 = -2000

    // 다만 서스테인 페달로 연명하는 경우에는 좀 더 약하게 감점을 함
    pub sustained: f64 // 기본값 = -1000
}

impl VoiceOverflowPriorityScoreSettings {
    fn new() -> Self {
        return Self {
            age: 1000.0,
            volume: 500.0,
            percussion: 4000.0,
            released: -2000.0,
            sustained: -1000.0
        };
    }
}

/**
 * 렌더링 중 실시간으로 수정이 가능한 설정
 */
pub struct SynthSettings {
    // sysex용 장치 ID
    // sysex 메세지에 명시된 ID와 비교해 일치하는 메세지만 처리함
    // 장치 ID가 127(`0x7F`)인 메세지인 경우는 무조건 처리함
    pub(crate) device_id: u8, // 0 - 126
    
    // 출력 게인
    pub(crate) output_gain: f64, // 0.0 - 20.0 (기본값 = 1.0)

    // 최대 동시 발음 수 초과 시 관련 설정(위에 참조)
    pub overflow: VoiceOverflowPriorityScoreSettings
}

impl SynthSettings {
    pub fn new() -> Self {
        return Self {
            device_id: 0x10,
            output_gain: 1.0,
            overflow: VoiceOverflowPriorityScoreSettings::new()
        };
    }

    pub fn set_device_id(&mut self, id: u8) {
        self.device_id = id.max(0).min(126);
    }

    pub fn set_output_gain(&mut self, gain: f64) {
        self.output_gain = gain.max(0.0).min(20.0);
    }
}