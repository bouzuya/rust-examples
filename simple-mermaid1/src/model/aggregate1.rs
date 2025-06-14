/// 集約1
///
/// simple_mermaid を使って状態遷移図を指定する例のための集約
///
/// # State Diagram
#[doc = simple_mermaid::mermaid!("aggregate1.mmd")]
pub struct Aggregate1(Aggregate1State);

enum Aggregate1State {
    Created,
    Updated,
}
