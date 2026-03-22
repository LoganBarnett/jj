pub struct BuildRef {
  pub number: u64,
  pub url: String,
}

pub enum BuildStatus {
  Running,
  Success,
  Failure,
  Aborted,
  Unstable,
  Unknown(String),
}

// Newtype so the meaning is explicit at call sites (std::process::exit).
pub struct BuildExitCode(pub i32);

impl BuildStatus {
  pub fn exit_code(&self) -> BuildExitCode {
    match self {
      BuildStatus::Success => BuildExitCode(0),
      BuildStatus::Failure => BuildExitCode(1),
      BuildStatus::Aborted => BuildExitCode(2),
      BuildStatus::Unstable => BuildExitCode(3),
      BuildStatus::Running => BuildExitCode(1),
      BuildStatus::Unknown(_) => BuildExitCode(4),
    }
  }
}
