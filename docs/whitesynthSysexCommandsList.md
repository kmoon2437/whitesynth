# whitesynth 확장 Sysex 명령 목록
파일이 길어질 거 같아서 여기서 따로 설명한다.

## 분류

| 파라미터 번호 | 설명 |
| :---: | --------- |
| `00 00 nn nn` | 시스템 설정(리셋 sysex라던가) |
| `01 00 nn nn` | 기본 설정 |
| `02 00 nn nn` | 기본 이펙트 관련 설정 |
| `02 01 nn nn` | Common multi effect 관련 설정 |
| `03 0x nn nn` | 채널별 설정(x번): Multi effect 관련 설정 |

## 추가사항
설명을 어떻게 해석할지에 대해서는 [여기](./sysexCommands.md)의 "추가사항" 부분을 참조하라.

GM Sysex에서는 일부 little endian을 사용하는 경우가 있는 것과 달리 여기서는 기본적으로 big endian만을 사용한다.

## 목록

### 기본 설정

#### System reset - `00 00 00 7F`
기본 모드로 리셋을 한다.

- 전체 Sysex 형태: `F0 7D 10 0B 15 00 00 00 7F F7`
- 데이터: 없음

#### Master tune - `01 00 00 00`
cent 단위로 전체 음높이를 조율한다. 1키 = 100cent이다. GM의 Master fine tuning과 연동되지만 실제 값에 반영될 때의 공식이 다르니 주의할 것.

- 전체 Sysex 형태: `F0 7D 10 0B 15 00 01 00 00 mm ll F7`
- 데이터: `mm ll`
  - `mm ll`: -102.4 - 0 - 102.3875 cent (0 - 8192 - 16383), 간격 = 0.0125 cent
    - 기본값: 0

#### Master volume - `01 00 00 01`
최종 출력의 음량을 설정한다. GM의 Master volume과 연동된다.

- 전체 Sysex 형태: `F0 7D 10 0B 15 00 01 00 01 mm ll F7`
- 데이터: `mm ll`
  - `mm ll`: 0 - 16383
    - 기본값: 16383

#### Master key shift - `01 00 00 02`
전체 음높이를 1키 단위로 조절한다. GM의 Master coarse tuning과 연동된다.

- 전체 Sysex 형태: `F0 7D 10 0B 15 00 01 00 02 kk F7`
- 데이터: `kk`
  - `kk`: -24 - 0 - +24 (40 - 64 - 88)
    - 기본값: 0

#### Master pan - `01 00 00 03`
최종 출력의 좌우 음향을 조절한다.

- 전체 Sysex 형태: `F0 7D 10 0B 15 00 01 00 03 kk F7`
- 데이터: `kk`
  - `kk`: L63(1) - Center(64) - R63(127)
    - 기본값: Center(64)

### 기본 이펙트 관련 설정
이 부분은 확정된 것이 아니며, 변경될 수 있다.

<!--#### Remove reverberation - `02 00 00 00`
reverb/delay 이펙트로 인해 volume을 줄이거나 all sound off를 해도 잔향이 남는 경우가 있는데, 이러한 잔향을 깔끔히 없앤다.
synth 구조상 이걸 채널별로 하는 것은 불가능하다.

- 값 형태: `0000 0000`
- 값: 0 (`0x00`)
- 기본값: 없음 -->

#### Reverb preset - `02 00 01 00`
이걸 설정하면 나머지 reverb 파라미터들이 자동으로 맞춰진다.

- 전체 Sysex 형태: `F0 7D 10 0B 15 02 00 01 00 kk F7`
- 데이터: `kk`
  - `kk`: Room 1(0),Room 2(1),Room 3(2),Hall 1(3),Hall 2(4),Plate(5),Delay(6),Panning delay(7)
    - 기본값: Hall 2(4)

#### Reverb room type - `02 00 01 01`
reverb 이펙트용 방의 종류를 지정한다.

- 전체 Sysex 형태: `F0 7D 10 0B 15 02 00 01 01 kk F7`
- 데이터: `kk`
  - `kk`: Room(0),Hall(1),Plate(2)
    - 기본값: Hall(1)

설명: Room은 평범한 방(밴드 합주 연습실 정도),Hall은 흔한 공연장,Plate는 구식 철판 리버브다.

#### Reverb room size - `02 00 01 02`
reverb 이펙트용 방의 용적(크기)을 지정한다.

- 전체 Sysex 형태: `F0 7D 10 0B 15 02 00 01 02 kk F7`
- 데이터: `kk`
  - `kk`: 0 - 127

#### Reverb pre-lowpass-filter - `02 00 01 03`

- 전체 Sysex 형태: `F0 7D 10 0B 15 02 00 01 03 kk F7`
- 데이터: `kk`
  - `kk`: 0 - 7
    - 기본값: 4

#### Reverb level - `02 00 01 04`
reverb 출력음의 level을 설정한다.

- 전체 Sysex 형태: `F0 7D 10 0B 15 02 00 01 04 kk F7`
- 데이터: `kk`
  - `kk`: 0 - 127
    - 기본값: 64

#### Reverb time - `02 00 01 05`
reverb 이펙트로 인해 잔향이 남는 시간을 설정한다.

- 전체 Sysex 형태: `F0 7D 10 0B 15 02 00 01 05 kk F7`
- 데이터: `kk`
  - `kk`: 0 - 127
    - 기본값: 64

#### Reverb delay feedback - `02 00 01 06`
reverb 이펙트에서 delay로 인해 소리가 울리는 횟수를 지정한다.

- 전체 Sysex 형태: `F0 7D 10 0B 15 02 00 01 06 kk F7`
- 데이터: `kk`
  - `kk`: 0 - 127
    - 기본값: 0

#### Reverb pre-delay time - `02 00 01 07`
reverb 이펙트에서 소리가 울리기 시작하는 시간을 설정한다.

- 전체 Sysex 형태: `F0 7D 10 0B 15 02 00 01 07 kk F7`
- 데이터: `kk`
  - `kk`: 0 - 127 (ms)
    - 기본값: 0

### 채널별 Multi effect 관련 설정

#### Common multi effect type - `02 01 1x 00`
해당 이펙트 종류를 설정한다. 이걸 설정하면 모든 파라미터의 값이 이펙트별 기본값으로 지정된다.  
종류에 대해서는 [여기](./multiEffects.md)를 참조하라.

- 전체 Sysex 형태: `F0 7D 10 0B 15 02 01 0x 01 kk ll mm F7`
  - `x` = 이펙터 번호 (`0x0` - `0xF`)
- 데이터: `kk ll mm`
  - `kk ll mm`: `0x00 0x00 0x00` - `0x7F 0x7F 0x7F`
    - 기본값: `0x00 0x00 0x00` (None)

#### Common multi effect parameter - `02 01 2x pp`
해당 이펙트 파라미터를 설정한다.

- 전체 Sysex 형태: `F0 7D 10 0B 15 02 01 2y pp kk ... kk F7`
  - `x` = 이펙터 번호 (`0x0` - `0xF`)
  - `pp` = 파라미터 번호 (`0x00` - `0x2F`)
- 데이터: `kk ... kk`
  - `kk ... kk`: 몇개가 될지, 값의 범위가 어떻게 될지는 모름

### 채널별 Multi effect 관련 설정

#### Multi effect unit - `03 0x 0y 00`
해당 채널이 Common multi effect의 이펙터를 사용할 것인지 이 채널에서 따로 이펙터를 설정할 것인지 설정한다.  
전자의 경우(`kk` = `0x7F` 인 경우) 종류와 파라미터도 그쪽을 따라가며, 후자의 경우(`kk` = `0x00` - `0x0F` 인 경우) 아래의 채널별 Multi effect 설정을 통해 종류와 파라미터를 설정할 수 있다.

- 전체 Sysex 형태: `F0 7D 10 0B 15 03 0x 0y 00 kk F7`
  - `x` = 채널 번호 (`0x0` - `0xF`)
  - `y` = 이펙터 번호 (`0x0` - `0xF`)
- 데이터: `kk`
  - `kk`: `0x00` - `0x0F` (Common multi effect), `0x7F` (Channel multi effect)
    - 기본값: `0x7F` (Channel multi effect)

#### Channel multi effect type - `03 0x 1y 00`
해당 채널에서 사용되는 이펙트 종류를 설정한다. 이걸 설정하면 모든 파라미터의 값이 이펙트별 기본값으로 지정된다.  
종류에 대해서는 [여기](./multiEffects.md)를 참조하라.

- 전체 Sysex 형태: `F0 7D 10 0B 15 03 0x 1y 00 kk ll mm F7`
  - `x` = 채널 번호 (`0x0` - `0xF`)
  - `y` = 이펙터 번호 (`0x0` - `0xF`)
- 데이터: `kk ll mm`
  - `kk ll mm`: `0x00 0x00 0x00` - `0x7F 0x7F 0x7F`
    - 기본값: `0x00 0x00 0x00` (None)

#### Channel multi effect parameter - `03 0x 2y pp`
해당 채널에서 사용되는 이펙트 파라미터를 설정한다.

- 전체 Sysex 형태: `F0 7D 10 0B 15 03 0x 2y pp kk ... kk F7`
  - `x` = 채널 번호 (`0x0` - `0xF`)
  - `y` = 이펙터 번호 (`0x0` - `0xF`)
  - `pp` = 파라미터 번호 (`0x00` - `0x2F`)
- 데이터: `kk ... kk`
  - `kk ... kk`: 몇개가 될지, 값의 범위가 어떻게 될지는 모름