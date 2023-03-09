# Insertion effect
Insertion effect는 지정된 채널에 갖가지 이펙트(distortion,overdrive 등등등 뭐가 많음)를 넣을 수 있는 기능으로, 넣은 이펙트에 따라 정말 별의별 소리를 다 낼 수 있다. lrsynth에서는 1포트당 16개까지 사용 가능하며, 16번째 이펙터는 Roland GS sysex 로도 제어할 수 있다.

## `0x24` - `0x2F` 파라미터에 대해
`0x24` - `0x2F` 파라미터는 이펙터의 종류에 상관없이 공통으로 들어가는 파라미터이다.

| 파라미터 번호 | 이름 | 값 | 기본값 | 설명 |
| --- | -------- | ----- | --- | ------------ |
| `0x24` | Level | 0 - 127 | 이펙트 종류에 따라 변화 | 최종 출력 level을 조정한다. |
| `0x25` | Audio channel | Left(63) - Both(64) - Right(65) | Both(64) | 이펙트를 왼쪽에만 적용할지 오른쪽에만 적용할지 둘 다 적용할지 결정한다. |
| `0x26` | Pan | L63(1) - Center(64) - R63(127) | Channel에 따라 변화(Left: L63(0),Right: R63(127),Both: Center(64)) | 이펙트를 적용하는 쪽의 pan을 조절하며, Both이면 평범한 pan과 동일하게 작동한다. |
| `0x27` | Mode | 0 - 2 | 1(GS sysex로 이펙트 설정시 2로 고정) | 어느 시점에서 이 이펙트를 처리할지 설정한다.(아래 참고) |
| `0x28` | Reverb send level | 0 - 127 | 40 | post-process mode에서의 reverb send level을 설정한다. pre-process와 normal mode에서는 개별 채널의 `CC#91` 을 사용한다. |
| `0x29` | Chorus send level | 0 - 127 | 0 | post-process mode에서의 chorus send level을 설정한다. pre-process와 normal mode에서는 개별 채널의 `CC#93` 을 사용한다. |
| `0x2A` | Delay send level | 0 - 127 | 0 | post-process mode에서의 delay send level을 설정한다. pre-process와 normal mode에서는 개별 채널의 `CC#94` 를 사용한다. |
| `0x2B` | EQ on/off | 0 - 127 | 0 | post-process mode에서의 EQ 사용 여부를 설정한다. pre-process와 normal mode에서는 Sysex를 통해 채널별로 설정할 수 있다. |
| `0x2C` | (Reserved) | N/A | N/A | 추후 사용할 경우를 대비해 비워놓은 곳 |
| : | : | : | : | : |
| `0x2F` | (Reserved) | N/A | N/A | 추후 사용할 경우를 대비해 비워놓은 곳 |

### `0x27` 파라미터에 대해
값이 0인 경우는 pre-process mode에 해당하며, volume 처리와 filter(cutoff: `CC#74`,resonance: `CC#71`) 처리 이전에 이펙트를 먼저 처리한다.

값이 1인 경우는 normal mode에 해당하며, volume 처리는 똑같이 하나 이펙트를 처리한 **뒤에** filter(cutoff,resonance) 를 처리한다.

위 두 경우에는 이펙트를 각 채널별로 따로 처리하며, 이펙트 적용 후에 volume을 적용하기 때문에 `CC#7` 값이 해당 채널의「최종」 출력 볼륨으로 적용된다.

값이 2인 경우는 post-process mode에 해당하며, volume 처리까지 모두 한 다음 같은 이펙트가 적용된 소리들을 다 합친 다음 이펙트 처리를 한다.
때문에 `CC#7` 로도 최종 출력 볼륨을 조정할 수 없으며, 해당 이펙트를 가장 마지막으로 처리하고 다음에 나오는 이펙트는 켜져 있어도 처리하지 않는다.

roland scva의 경우에는 여기서의 post-process mode와 비슷한 형태로 처리한다.

## 이펙트 종류
뭐가 엄청 많지만 사실 별로 쓰는건 없어서 많이 쓰는 것부터 구현할 예정이다.  
~~취소선~~이 쳐진 건 구현 예정에 없거나 구현이 안 됐다는 뜻이다.

공식을 적어 놓은 경우, 해당 공식은 일반 값(얘를 x라 함)을 sysex용 값으로 변환하는 공식이다.

### None - `00 00 00`
기본값. 아무것도 적용하지 않는다.

### Stereo EQ - `41 01 00`
4 band 이퀄라이저를 적용한다.

#### 파라미터
공통 파라미터 기본값
- `0x24` (Level): 127

| 파라미터 번호 | 이름 | 값 길이 | 값 | 기본값 | 설명 |
| --- | -------- | - | ----- | --- | ------------ |
| `0x00` | Low freq | 1 | 200Hz,400Hz(N/A) | N/A | 저음역대의 기준 주파수를 설정한다. |
| `0x01` | Low gain | 1 | -12 - +12(dB)<br>(공식: x + 64) | 0 | 저음역대의 gain을 조정한다. |
| `0x02` | High freq | 1 | 4kHz,8kHz(N/A) | N/A | 고음역대의 기준 주파수를 설정한다. |
| `0x03` | High gain | 1 | -12 - +12(dB)<br>(공식: x + 64) | 0 | 고음역대의 gain을 조정한다. |
| `0x04` | Mid1 freq | 2 | 200 - 8000(Hz) | N/A | 중간 음역대 1의 기준 주파수를 설정한다. |
| `0x06` | Mid1 gain | 1 | -12 - +12(dB)<br>(공식: x + 64) | 0 | 중간 음역대 1의 gain을 조정한다. |
| `0x07` | Mid2 freq | 2 | 200 - 8000(Hz) | N/A | 중간 음역대 2의 기준 주파수를 설정한다. |
| `0x09` | Mid2 gain | 1 | -12 - +12(dB)<br>(공식: x + 64) | 0 | 중간 음역대 2의 gain을 조정한다. |

### Overdrive - `41 01 10`
일렉기타의 overdrive 이펙트를 적용한다.

#### 파라미터
공통 파라미터 기본값
- `0x24` (Level): 96

| 파라미터 번호 | 이름 | 값 | 기본값 | 설명 |
| --- | -------- | ----- | --- | ------------ |
| `0x00` | Drive | 0 - 127 | 48 (`0x30`) | drive의 정도를 조정한다. |
| `0x01` | Amp type | Small,Combo,Stack,Tube<br>(`0x00`,`0x01`,`0x02`,`0x03`) | Combo (`0x01`) | 기타 앰프 종류를 선택한다.[^GuitarAmpType] |
| `0x02` | Amp on/off | off,on<br>(`0x00`,`0x01`) | on (`0x01`) | 기타 앰프를 켜고 끈다. |
| `0x10` | EQ Low(200Hz) gain | -12 - +12(dB)<br>(공식: x + 64) | 0 | 저음역대의 EQ gain을 조정한다. |
| `0x11` | EQ High(4kHz) gain | -12 - +12(dB)<br>(공식: x + 64) | 0 | 고음역대의 EQ gain을 조정한다. |
| `0x12` | Pan | Auto - L63 - Center - R63<br>(`0x00` - `0x01` - `0x40` - `0x7F`) | Auto (`0x00`) | 이 값이 0 (`0x00`) 이 아닐 경우 채널의 control change의 pan 설정값을 무시하고 여기서 설정한 pan 값에 강제로 맞춘다. |

### Distortion - `41 01 11`
일렉기타의 distortion 이펙트를 적용한다.

#### 파라미터
공통 파라미터 기본값
- `0x24` (Level): 84

| 파라미터 번호 | 이름 | 값 | 기본값 | 설명 |
| --- | -------- | ----- | --- | ------------ |
| `0x00` | Drive | 0 - 127 | 76 (`0x4C`) | drive의 정도를 조정한다. |
| `0x01` | Amp type | Small,Combo,Stack,Tube<br>(`0x00`,`0x01`,`0x02`,`0x03`) | Tube (`0x03`) | 기타 앰프 종류를 선택한다.[^GuitarAmpType] |
| `0x02` | Amp on/off | off,on<br>(`0x00`,`0x01`) | on (`0x01`) | 기타 앰프를 켜고 끈다. |
| `0x10` | EQ Low(200Hz) gain | -12 - +12(dB)<br>(공식: x + 64) | 0 | 저음역대의 EQ gain을 조정한다. |
| `0x11` | EQ High(4kHz) gain | -12 - +12(dB)<br>(공식: x + 64) | -8 (`0x38`) | 고음역대의 EQ gain을 조정한다. |
| `0x12` | Pan | Auto - L63 - Center - R63<br>(`0x00` - `0x01` - `0x40` - `0x7F`) | Auto (`0x00`) | 이 값이 0 (`0x00`) 이 아닐 경우 채널 cc의 pan 설정값을 무시하고 여기서 설정한 pan 값에 강제로 맞춘다. |

### Amp simulator - `00 01 00`
앰프만 쓰는 거. 여기서도 드라이브를 넣을 수는 있다.

#### 파라미터
공통 파라미터 기본값
- `0x24` (Level): 112

| 파라미터 번호 | 이름 | 값 | 기본값 | 설명 |
| --- | -------- | ----- | --- | ------------ |
| `0x00` | Drive | 0 - 127 | 0 (`0x00`) | drive의 정도를 조정한다. |
| `0x01` | Amp type | Small,Combo,Stack,Tube<br>(`0x00`,`0x01`,`0x02`,`0x03`) | Tube (`0x03`) | 기타 앰프 종류를 선택한다.[^GuitarAmpType] |
| `0x10` | EQ Low(200Hz) gain | -12 - +12(dB)<br>(공식: x + 64) | 0 | 저음역대의 EQ gain을 조정한다. |
| `0x11` | EQ High(4kHz) gain | -12 - +12(dB)<br>(공식: x + 64) | 0 | 고음역대의 EQ gain을 조정한다. |
| `0x12` | Pan | Auto - L63 - Center - R63<br>(`0x00` - `0x01` - `0x40` - `0x7F`) | Auto (`0x00`) | 이 값이 0 (`0x00`) 이 아닐 경우 채널 cc의 pan 설정값을 무시하고 여기서 설정한 pan 값에 강제로 맞춘다. |

### ~~Tremolo - `41 01 25`~~
이름 그대로이다.

#### 파라미터
공통 파라미터 기본값
- `0x24` (Level): 127

| 파라미터 번호 | 이름 | 값 | 기본값 | 설명 |
| --- | -------- | ----- | --- | ------------ |
| `0x00` | Mod wave | 0 - 4 | N/A | modulation의 종류를 설정한다.<br>0 = Triangle wave<br>1 = Square wave<br>2 = Sine wave<br>3 = Saw wave<br>4 = 뒤집어진 Saw wave |
| `0x01` | Mod rate | 0.05 - 10.00(Hz)<br>(5 - 1000(공식: 100*x), `0x00 0x05` - `0x07 0x68`) | N/A | modulation rate를 설정한다. |
| `0x02` | Mod depth | 0 - 127 | N/A | modulation depth를 설정한다. |
| `0x10` | EQ Low(200Hz) gain | -12 - +12(dB)<br>(공식: x + 64) | 0 | 저음역대의 EQ gain을 조정한다. |
| `0x11` | EQ High(4kHz) gain | -12 - +12(dB)<br>(공식: x + 64) | 0 | 고음역대의 EQ gain을 조정한다. |

### ~~Compressor - `41 01 30`~~
일렉기타용 컴프레서. 가장 큰 소리와 가장 작은 소리의 볼륨을 특정한 기준점에 맞춤으로써 볼륨의 격차(다이나믹 레인지)를 줄인다.

#### 파라미터
공통 파라미터 기본값
- `0x24` (Level): 127

| 파라미터 번호 | 이름 | 값 | 기본값 | 설명 |
| --- | -------- | ----- | --- | ------------ |
| `0x00` | Attack | 0 - 127 | N/A | threshold를 넘는 소리가 줄어드는 시간을 조정한다. |
| `0x01` | Sustain | 0 - 127 | N/A | threshold에 못 미치는 소리가 커지는 시간을 조정한다. GS sysex로 이 값을 설정하면 Threshold(`0x03`) 값도 같은 값(x)으로 설정되며, Sustain 값은 127 - x 로 설정된다. |
| `0x02` | Makeup gain | 0 - +18<br>(공식: x) | N/A | 출력 gain을 조정한다. |
| `0x03` | Threshold | 0 - 127 | N/A | "특정한 기준점에 맞춤으로써"에서 "특정한 기준점"을 설정한다. |
| `0x04` | Ratio | 0 - 100 | N/A | threshold를 a(a != 0)만큼 넘은 볼륨을 a*((100 - x)/100) 만큼 줄일 때 x를 정한다.(일반적인 컴프레서에서의 방식대로 표기하면 100 : x 이다) |
| `0x05` | Release | 0 - 127 | N/A | threshold에서 멀어진 볼륨이 다시 threshold에 가까워졌을 때 줄이거나 키운 볼륨을 원상복구하는 시간을 조정한다. |
| `0x10` | EQ Low(200Hz) gain | -12 - +12(dB)<br>(공식: x + 64) | 0 | 저음역대의 EQ gain을 조정한다. |
| `0x11` | EQ High(4kHz) gain | -12 - +12(dB)<br>(공식: x + 64) | 0 | 고음역대의 EQ gain을 조정한다. |
| `0x12` | Pan | Auto - L63 - Center - R63<br>(`0x00` - `0x01` - `0x40` - `0x7F`) | Auto (`0x00`) | 이 값이 0 (`0x00`) 이 아닐 경우 채널 cc의 pan 설정값을 무시하고 여기서 설정한 pan 값에 강제로 맞춘다. |

### Limiter - `41 01 31`
이름에서 알 수 있듯이 볼륨이 일정 수준 이상으로 올라가지 않도록 볼륨을 조절한다.

#### 파라미터
공통 파라미터 기본값
- `0x24` (Level): 127

| 파라미터 번호 | 이름 | 값 | 기본값 | 설명 |
| --- | -------- | ----- | --- | ------------ |
| `0x00` | Threshold | 0 - 127 | N/A | "일정 수준 이상으로" 에서 "일정 수준"을 설정한다. |
| `0x01` | Ratio | 0 - 100 | N/A | threshold를 a(a > 0)만큼 넘은 볼륨을 a*((100 - x)/100) 만큼 줄일 때 x를 정한다.(일반적인 컴프레서에서의 방식대로 표기하면 100 : x 이다) |
| `0x02` | Release | 0 - 127 | N/A | threshold를 넘은 볼륨이 다시 threshold 밑으로 내려갔을 때 줄인 볼륨을 원상복구하는 시간을 조정한다. |
| `0x03` | Attack | 0 - 127 | 0 | threshold를 넘는 소리가 줄어드는 시간을 조정한다. |
| `0x10` | EQ Low(200Hz) gain | -12 - +12(dB)<br>(공식: x + 64) | 0 | 저음역대의 EQ gain을 조정한다. |
| `0x11` | EQ High(4kHz) gain | -12 - +12(dB)<br>(공식: x + 64) | 0 | 고음역대의 EQ gain을 조정한다. |
| `0x12` | Pan | Auto - L63 - Center - R63<br>(`0x00` - `0x01` - `0x40` - `0x7F`) | Auto (`0x00`) | 이 값이 0 (`0x00`) 이 아닐 경우 채널 cc의 pan 설정값을 무시하고 여기서 설정한 pan 값에 강제로 맞춘다. |

### Drive 1 / 2 - `41 11 03`
Drive 계열 이펙트를 좌/우에 하나씩 적용한다. 좌,우의 이펙트 설정을 다르게 할 수 있다.

#### 파라미터
공통 파라미터 기본값
- `0x24` (Level): 127

| 파라미터 번호 | 이름 | 값 | 기본값 | 설명 |
| --- | -------- | ----- | --- | ------------ |
| `0x00` | Left:Type | Overdrive,Distortion<br>(`0x00`,`0x01`) | Overdrive (`0x00`) | 왼쪽 채널의 drive 이펙터 종류를 선택한다. |
| `0x01` | Left:Drive | 0 - 127 | 48 (`0x30`) | 왼쪽 채널의 drive의 정도를 조정한다. |
| `0x02` | Left:Amp type | Small,Combo,Stack,Tube<br>(`0x00`,`0x01`,`0x02`,`0x03`) | Combo (`0x01`) | 왼쪽 채널의 기타 앰프 종류를 선택한다.[^GuitarAmpType] |
| `0x03` | Left:Amp on/off | off,on<br>(`0x00`,`0x01`) | on (`0x01`) | 왼쪽 채널의 기타 앰프를 켜고 끈다. |
| `0x05` | Right:Type | Overdrive,Distortion<br>(`0x00`,`0x01`) | Distortion (`0x01`) | 오른쪽 채널의 drive 이펙터 종류를 선택한다. |
| `0x06` | Right:Drive | 0 - 127 | 76 (`0x4C`) | 오른쪽 채널의 drive의 정도를 조정한다. |
| `0x07` | Right:Amp type | Small,Combo,Stack,Tube<br>(`0x00`,`0x01`,`0x02`,`0x03`) | Tube (`0x03`) | 오른쪽 채널의 기타 앰프 종류를 선택한다.[^GuitarAmpType] |
| `0x08` | Right:Amp on/off | off,on<br>(`0x00`,`0x01`) | on (`0x01`) | 오른쪽 채널의 기타 앰프를 켜고 끈다. |
| `0x0F` | Left:Pan | L63 - Center - R63<br>(`0x01` - `0x40` - `0x7F`) | L63 (`0x00`) | 왼쪽 채널의 pan 값을 조절한다. |
| `0x10` | Left:Level | 0 - 127 | 96 (`0x60`) | 왼쪽 채널의 출력 level을 조정한다. |
| `0x11` | Right:Pan | L63 - Center - R63<br>(`0x01` - `0x40` - `0x7F`) | R63 (`0x7F`) | 오른쪽 채널의 pan 값을 조절한다. |
| `0x12` | Right:Level | 0 - 127 | 84 (`0x54`) | 오른쪽 채널의 출력 level을 조정한다. |

[^GuitarAmpType]: Small = 소형 앰프,Combo = 콤보 앰프,Stack = 스택 앰프,Tube = 진공관 앰프. roland sound canvas 미디음원의 설명서에서는 이들을 순서대로 Small,Built-in,2 stack,3 stack이라 한다.