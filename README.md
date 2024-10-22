# whitesynth
소프트웨어 미디음원 라이브러리

개발중. 많이 불안정함

## 목표
- 라즈베리파이로 노래방 기계를 하나 만들어 보려는데, 거기서 돌릴 roland sc-8820 호환 미디음원이 없어서 그냥 직접 만들자 하고 개발 시작한 게 이것
- 근데 2024년 9월 1일에 SCVA가 단종이 돼서 언젠간 윈도우 본컴에서도 못쓰게 될 수 있음. 그러니까 roland에서 따로 비슷한 가상악기를 내오지 않는 한 무조건 이걸 완성해 내야 됨

## 구현 예정
- 일단 미디음원의 기본기능(note on/off, control change, pitch bend 같은 것들)
- sc 8820(사실상 scva)에서 지원하는것들 중 대강 70%는 만들 예정
- 배리에이션 이펙트(variation effect) 1포트당 16개(scva efx 호환용) + 채널 1개당 멀티 이펙트(multi effect) 16개
  - <https://www.utsbox.com/?p=3302> 이 플러그인에서 지원하는 이펙트 중 일부는 multi effect 형태로 구현할 수도 있음
- [여기](https://github.com/kmoon2437/whitesynth-docs)에 어떻게 만들지 사용설명서 형태로 문서를 만들어 놓았음. 완성된 게 아니므로 자주 바뀔 수 있음

## 기타
- 「Roland」,「GS」(**G**eneral **S**tandard),「Sound Canvas」는 Roland 사가 등록한 상표다.
  - 한 가지 재밌는 사실은 **「EFX」도 Roland 사가 등록한 상표라는 것이다!**

## 참고 문헌
- 각종 이펙트 처리 관련: <https://utsbox.com>