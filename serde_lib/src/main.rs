// 사용 모듈 import
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::{collections::BTreeMap, fs};

// 프레임 우선순위 클래스(IEEE 802.1Q)
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum TrafficClass {
    A,
    B,
    C,
    BestEffort,
}

// 스트림 예약(SR-P) 파라미터
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SrpParameter {
    max_frame_size: u32,      // 바이트
    max_interval_frames: u32, // 주기당 전송 가능한 프레임 수
    accumulated_latency_ns: u64,
    traffic_class: TrafficClass,
}

// 게이트 마스크: 8개 우선순위 큐의 gate open/close
bitflags::bitflags! {
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(transparent)]
    struct GateMask: u8 {
        const P0 = 0b0000_0001;
        const P1 = 0b0000_0010;
        const P2 = 0b0000_0100;
        const P3 = 0b0000_1000;
        const P4 = 0b0001_0000;
        const P5 = 0b0010_0000;
        const P6 = 0b0100_0000;
        const P7 = 0b1000_0000;
    }
}

// 게이트 컨트롤 리스트(GCL) 엔트리(IEEE 802.1Qbv)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GateControlEntry {
    time_ns: u64,          // 오프셋(ns)
    interval_ns: u64,      // 지속시간(ns)
    gate_mask: GateMask,   // 열릴 큐
}

// TSN 스케줄(포트 단위)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PortSchedule {
    port_id:        u8,
    base_time_ns:   u64,
    cycle_time_ns:  u64,
    gcl:            Vec<GateControlEntry>,
}

// 전체 스위치의 설정
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SwitchConfig {
    #[serde_as(as = "DisplayFromStr")]
    switch_id: uuid::Uuid,

    generated_at: DateTime<Utc>,
    precision_ns: u32,   // 네트워크 시계 정확도

    // 스트림 ID → SR-P 파라미터
    streams: BTreeMap<String, SrpParameter>,

    // 포트별 GCL
    schedules: Vec<PortSchedule>,
}

fn main() -> anyhow::Result<()> {
    // 1) 샘플 구성 -------------------------------------------------------------
    let cfg = sample_config();

    // 2) 직렬화 결과를 JSON 파일로 저장 ------------------------------------------------------
    let json = serde_json::to_string_pretty(&cfg)?;
    fs::write("tsn_config.json", &json)?;
    println!("\n직렬화 결과: tsn_config.json 저장 완료");

    // 3) 다시 읽어 역직렬화 ----------------------------------------------------
    let raw = fs::read_to_string("tsn_config.json")?;
    let parsed: SwitchConfig = serde_json::from_str(&raw)?;
    println!("\n역직렬화 결과:\n{parsed:#?}");
    Ok(())
}

// 샘플 설정 데이터
fn sample_config() -> SwitchConfig {
    use uuid::Uuid;

    let mut streams = BTreeMap::new();
    streams.insert(
        "stream_0".into(),
        SrpParameter {
            max_frame_size: 124,
            max_interval_frames: 1,
            accumulated_latency_ns: 500_000, // 0.5 µs
            traffic_class: TrafficClass::A,
        },
    );
    streams.insert(
        "stream_video".into(),
        SrpParameter {
            max_frame_size: 1500,
            max_interval_frames: 4,
            accumulated_latency_ns: 3_000_000, // 3 µs
            traffic_class: TrafficClass::B,
        },
    );

    let gcl_port1 = vec![
        GateControlEntry { time_ns: 0,       interval_ns: 1_000_000,  gate_mask: GateMask::P7 }, // 프리사이즈 제어
        GateControlEntry { time_ns: 1_000_000, interval_ns: 7_000_000, gate_mask: GateMask::P0 | GateMask::P1 },
    ];

    let gcl_port2 = vec![
        GateControlEntry { time_ns: 0, interval_ns: 8_000_000, gate_mask: GateMask::P3 | GateMask::P4 },
    ];

SwitchConfig {
        switch_id:    Uuid::new_v4(),
        generated_at: Utc::now(),
        precision_ns: 50, // ±50 ns
        streams,
        schedules: vec![
            PortSchedule {
                port_id: 1,
                base_time_ns: 0,
                cycle_time_ns: 8_000_000,
                gcl: gcl_port1,
            },
            PortSchedule {
                port_id: 2,
                base_time_ns: 0,
                cycle_time_ns: 8_000_000,
                gcl: gcl_port2,
            },
        ],
    }
}