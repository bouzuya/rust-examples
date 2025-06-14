/// 集約1
///
/// simple_mermaid を使って状態遷移図を指定する例のための集約
///
/// # State Diagram
#[doc = simple_mermaid::mermaid!("aggregate1.mmd")]
pub struct Aggregate1(Aggregate1State);

impl Aggregate1 {
    pub fn new() -> Self {
        Self(Aggregate1State::Created)
    }

    pub fn update(&self) -> Self {
        Self(Aggregate1State::Updated)
    }

    pub fn state(&self) -> String {
        match self.0 {
            Aggregate1State::Created => "created!".to_owned(),
            Aggregate1State::Updated => "updated!".to_owned(),
        }
    }
}

enum Aggregate1State {
    Created,
    Updated,
}
