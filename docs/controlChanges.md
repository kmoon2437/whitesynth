# Control change 관련 파라미터
문서가 너무 길어져서 여기에 따로 적는다. 값의 공식에 대해서는 [여기](./midiImplementation.md)의 첫 부분에 적혀 있다.

Filter 관련 내용 참조:
- <https://miing95.tistory.com/22>
  - PC용 페이지 아카이브: <https://web.archive.org/web/20230118085919/https://miing95.tistory.com/22>
  - 모바일용 페이지 아카이브: <https://web.archive.org/web/20230118085755/https://miing95.tistory.com/m/22>
- <https://www.cuonet.com/bbs/board.php?bo_table=qna2&wr_id=1033642>
  - 얘는 사이트가 봇을 막아놔서 아카이브를 못함
- <https://blog.naver.com/suya309/221437120375>
  - PC용 페이지 아카이브: <https://web.archive.org/web/20230118090918/https://blog.naver.com/suya309/221437120375>
  - 모바일용 페이지 아카이브: <https://web.archive.org/web/20230118091437/https://m.blog.naver.com/suya309/221437120375>

## 일반 파라미터
이름 옆에 `CC#n` 과 같이 번호를 적어 두었다.

### Bank select - `CC#0`,`CC#32`
Program change의 128개만으로는 악기 수가 턱없이 부족하므로, 각 program 번호별로 bank 번호를 따로 지정하고, 이걸 바꿔줌으로써 기본 GM 악기 외의 다른 악기를 사용할 수 있다.

#### 형식
```
Bn 00 kk
```
또는
```
Bn 20 ll
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `kk` = bank 번호 MSB: 0 - 127(`0x00` - `0x7f`), 기본값 = 0(`0x00`)
- `ll` = bank 번호 LSB(사실상 쓸 데 없음): 0 - 127(`0x00` - `0x7f`), 기본값 = 0(`0x00`)

### Modulation - `CC#1`

#### 형식
```
Bn 01 vv
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `vv` = 값: 0 - 127(`0x00` - `0x7f`), 기본값 = 0(`0x00`)

### Portamento time - `CC#5`

#### 형식
```
Bn 05 vv
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `vv` = 값: 0 - 127(`0x00` - `0x7f`), 기본값 = 0(`0x00`)

### Data entry - `CC#6`,`CC#38`
RPN,NRPN(얘네에 대해서는 밑에 참고)의 값을 전달하는 용도로 쓰인다.

#### 형식
```
Bn 01 kk
Bn 26 ll
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `kk` = 값 MSB: 0 - 127(`0x00` - `0x7f`)
- `ll` = 값 LSB: 0 - 127(`0x00` - `0x7f`)

### Volume - `CC#7`
채널 자체의 볼륨을 설정하며, 밑에 나오는 Expression과 달리 거의 모든 이펙트가 적용된 최종 출력의 volume을 조절한다.

#### 형식
```
Bn 07 vv
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `vv` = 값: 0 - 127(`0x00` - `0x7f`), 기본값 = 100(`0x64`)

### Pan - `CC#10`
소리의 좌우 음향을 조정한다.

#### 형식
```
Bn 0A vv
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `vv` = 값: Random - L63 - Center - R63(`0x00` - `0x01` - `0x40` - `0x7f`), 기본값 = Center(`0x40`)

### Expression - `CC#11`
채널 자체의 볼륨을 설정하며, 위에 나오는 Volume과 달리 어떠한 이펙트도 적용되지 않은 원 소스의 볼륨을 조절한다.

#### 형식
```
Bn 0B vv
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `vv` = 값: 0 - 127(`0x00` - `0x7f`), 기본값 = 127(`0x7f`)

### Damper pedal(Sustain pedal) - `CC#64`
피아노의 댐퍼 페달과 같은 기능(미디에서는 서스테인 페달이라 부름)을 on off한다.

#### 형식
```
Bn 40 vv
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `vv` = 값: 값을 x라 할 때, x <= `0x3f` 이면 off, x >= `0x40` 이면 on이다. 기본값 = off(`0x00`)

### Portamento - `CC#65`
포르타멘토 기능(2개의 음을 부드럽게 이어줌)을 on off한다.

#### 형식
```
Bn 41 vv
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `vv` = 값: 값을 x라 할 때, x <= `0x3f` 이면 off, x >= `0x40` 이면 on이다. 기본값 = off(`0x00`)

### Sostenuto pedal - `CC#66`
피아노의 소스테누토 페달과 같은 기능을 on off한다.

#### 형식
```
Bn 42 vv
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `vv` = 값: 값을 x라 할 때, x <= `0x3f` 이면 off, x >= `0x40` 이면 on이다. 기본값 = off(`0x00`)

### Soft pedal - `CC#67`
피아노의 소프트 페달과 같은 기능을 on off한다.

#### 형식
```
Bn 43 vv
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `vv` = 값: 값을 x라 할 때, x <= `0x3f` 이면 off, x >= `0x40` 이면 on이다. 기본값 = off(`0x00`)

### Filter resonance - `CC#71`
Low-pass filter의 Cutoff 주파수 주변의 주파수를 부스트 시키는 정도를 설정한다.

0은 초기값이다.

#### 형식
```
Bn 47 vv
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `vv` = 값: -63 - 0 - +63(공식: x + 64), 기본값 = 0

### Release time - `CC#72`
ADSR envelope의 「R」elease time을 설정한다.

0은 초기값이다.

#### 형식
```
Bn 48 vv
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `vv` = 값: -63 - 0 - +63(공식: x + 64), 기본값 = 0

### Attack time - `CC#73`
ADSR envelope의 「A」ttack time을 설정한다.

0은 초기값이다.

#### 형식
```
Bn 49 vv
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `vv` = 값: -63 - 0 - +63(공식: x + 64), 기본값 = 0

### Low-pass filter(Cutoff) - `CC#74`
Low-pass filter의 상한 주파수를 설정한다.

0은 초기값이다.

#### 형식
```
Bn 4A vv
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `vv` = 값: -63 - 0 - +63(공식: x + 64), 기본값 = 0

### Reverb send level - `CC#91`
reverb 이펙트의 입력 volume을 설정한다.

##@# 형식
```
Bn 5B vv
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `vv` = 값: 0 - 127(`0x00` - `0x7f`), 기본값 = 40(`0x28`)

### Chorus send level - `CC#93`
chorus 이펙트의 입력 volume을 설정한다.

#### 형식
```
Bn 5D vv
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `vv` = 값: 0 - 127(`0x00` - `0x7f`), 기본값 = 0(`0x00`)

### Delay send level - `CC#94`
delay 이펙트의 입력 volume을 설정한다.

#### 형식
```
Bn 5E vv
```
- `n` = 미디 채널 번호: 1 - 16(`0x0` - `0xf`)
- `vv` = 값: 0 - 127(`0x00` - `0x7f`), 기본값 = 0(`0x00`)

