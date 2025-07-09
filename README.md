## 구조

- `heap_sort_demo`: Heap Sort 실행 바이너리
- `serde_lib`: tsn_config.json을 생성하는 직렬화/역직렬화 실행 바이너리

## 실행 방법

```bash
# 전체 빌드
cargo build --workspace

# Heap sort 실행
cargo run -p heap_sort_demo

# tsn_config.json 생성(직렬화) 및 역직렬화
cargo run -p tsn_controller_config
