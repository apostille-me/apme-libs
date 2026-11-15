//! Stable contracts for Apostille Me.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Actor {
    pub id: String,
    pub tenant_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventEnvelope {
    pub id: String,
    pub kind: String,
    pub occurred_at: String,
    pub actor: Option<Actor>,
}

impl EventEnvelope {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.id.trim().is_empty() { return Err("event id is required"); }
        if !valid_kind(&self.kind) { return Err("event kind must be lowercase and namespaced"); }
        if self.occurred_at.trim().is_empty() { return Err("occurred_at is required"); }
        Ok(())
    }
}

pub fn valid_kind(value: &str) -> bool {
    let mut parts = value.split('.');
    let valid_part = |part: &str| {
        matches!(part.as_bytes().first(), Some(b'a'..=b'z'))
            && part.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || byte == b'_'
                    || byte == b'-'
            })
    };

    let Some(first) = parts.next() else {
        return false;
    };
    if !valid_part(first) {
        return false;
    }
    let mut has_namespace = false;
    for part in parts {
        has_namespace = true;
        if !valid_part(part) {
            return false;
        }
    }
    has_namespace
}

pub const PRODUCT: &str = "apostille-me";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_namespaced_kinds() {
        assert!(valid_kind("case.created"));
        assert!(!valid_kind("CaseCreated"));
        assert!(!valid_kind("created"));
        assert!(!valid_kind("1case.created"));
        assert!(valid_kind("case_record.created-v2"));
        for invalid in [
            ".case", "case.", "case..created", "case.1created", "Case.created",
            "case. created",
        ] {
            assert!(!valid_kind(invalid), "accepted invalid namespace: {invalid}");
        }
    }
}
