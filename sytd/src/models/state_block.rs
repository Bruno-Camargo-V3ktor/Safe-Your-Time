use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StateBlock {
    Idle,
    InProgress,
    Paused,
    Finished,
}
