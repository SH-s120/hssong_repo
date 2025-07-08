## 구조

- `heap_sort_demo`: Heap Sort 실행 바이너리
- `serde_lib`: tsn_config.json을 생성하는 직렬화/역직렬화 실행 바이너리
- `serde`: 커스터마이징된 Serde (path patch로 참조 중)

## 실행 방법

```bash
# 전체 빌드
cargo build --workspace

# 정렬 예제 실행
cargo run -p heap_sort_demo

# tsn_config.json 생성
cargo run -p tsn_controller_config