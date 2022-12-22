use crate::legality::formatting::legality_check_strings;
use crate::legality::{CheckIdentifier, Severity};

#[derive(Clone)]
pub struct CheckResult {
    pub judgement: Severity,
    pub comment: String,
    pub identifier: CheckIdentifier,
}

impl CheckResult {
    pub fn valid(&self) -> bool {
        self.judgement != Severity::Invalid
    }

    pub fn rating(&self) -> String {
        self.judgement.description()
    }
}

impl From<CheckIdentifier> for CheckResult {
    fn from(value: CheckIdentifier) -> Self {
        Self {
            judgement: Severity::Valid,
            comment: legality_check_strings::L_A_VALID.lock().unwrap().clone(),
            identifier: value,
        }
    }
}
