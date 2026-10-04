#![forbid(unsafe_code)]

//! Deterministic self-model and introspection primitives.
//!
//! This module models the agent's operational state. It is intentionally
//! non-authoritative: it cannot authenticate, authorize, change trust state,
//! access keys, execute tools, or mutate the security policy.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CognitiveState {
    Idle,
    Observing,
    Reasoning,
    Acting,
    Verifying,
    Recovering,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GoalStatus {
    Active,
    Completed,
    Suspended,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryKind {
    Observation,
    Decision,
    Action,
    Outcome,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Goal {
    pub id: u64,
    pub title: String,
    pub priority: u8,
    pub status: GoalStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryEvent {
    pub sequence: u64,
    pub kind: MemoryKind,
    pub summary: String,
    pub confidence: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntrospectionSnapshot {
    pub agent_name: String,
    pub agent_version: String,
    pub state: CognitiveState,
    pub confidence: u8,
    pub goals: Vec<Goal>,
    pub capabilities: Vec<String>,
    pub limitations: Vec<String>,
    pub memory_size: usize,
    pub last_memory: Option<MemoryEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelfModel {
    agent_name: String,
    agent_version: String,
    state: CognitiveState,
    confidence: u8,
    goals: Vec<Goal>,
    capabilities: Vec<String>,
    limitations: Vec<String>,
    memory: Vec<MemoryEvent>,
    next_sequence: u64,
    max_memory: usize,
}

impl SelfModel {
    pub const DEFAULT_MEMORY_CAPACITY: usize = 32;

    pub fn new(agent_name: impl Into<String>, agent_version: impl Into<String>) -> Self {
        Self::with_memory_capacity(agent_name, agent_version, Self::DEFAULT_MEMORY_CAPACITY)
    }

    pub fn with_memory_capacity(
        agent_name: impl Into<String>,
        agent_version: impl Into<String>,
        max_memory: usize,
    ) -> Self {
        Self {
            agent_name: agent_name.into(),
            agent_version: agent_version.into(),
            state: CognitiveState::Idle,
            confidence: 0,
            goals: Vec::new(),
            capabilities: Vec::new(),
            limitations: Vec::new(),
            memory: Vec::new(),
            next_sequence: 1,
            max_memory: max_memory.max(1),
        }
    }

    pub fn state(&self) -> CognitiveState {
        self.state
    }

    pub fn confidence(&self) -> u8 {
        self.confidence
    }

    pub fn set_state(&mut self, state: CognitiveState) {
        self.state = state;
    }

    pub fn set_confidence(&mut self, confidence: u8) {
        self.confidence = confidence.min(100);
    }

    pub fn add_goal(&mut self, goal: Goal) {
        self.goals.push(goal);
    }

    pub fn add_capability(&mut self, capability: impl Into<String>) {
        self.capabilities.push(capability.into());
    }

    pub fn add_limitation(&mut self, limitation: impl Into<String>) {
        self.limitations.push(limitation.into());
    }

    pub fn record_memory(
        &mut self,
        kind: MemoryKind,
        summary: impl Into<String>,
        confidence: u8,
    ) -> u64 {
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);

        self.memory.push(MemoryEvent {
            sequence,
            kind,
            summary: summary.into(),
            confidence: confidence.min(100),
        });

        if self.memory.len() > self.max_memory {
            let overflow = self.memory.len() - self.max_memory;
            self.memory.drain(0..overflow);
        }

        sequence
    }

    pub fn memory(&self) -> &[MemoryEvent] {
        &self.memory
    }

    pub fn introspect(&self) -> IntrospectionSnapshot {
        IntrospectionSnapshot {
            agent_name: self.agent_name.clone(),
            agent_version: self.agent_version.clone(),
            state: self.state,
            confidence: self.confidence,
            goals: self.goals.clone(),
            capabilities: self.capabilities.clone(),
            limitations: self.limitations.clone(),
            memory_size: self.memory.len(),
            last_memory: self.memory.last().cloned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_model_starts_idle_without_memory() {
        let model = SelfModel::new("eda", "1.0.0");

        assert_eq!(model.state(), CognitiveState::Idle);
        assert_eq!(model.confidence(), 0);
        assert!(model.memory().is_empty());
    }

    #[test]
    fn confidence_is_bounded() {
        let mut model = SelfModel::new("eda", "1.0.0");

        model.set_confidence(140);

        assert_eq!(model.confidence(), 100);
    }

    #[test]
    fn memory_is_bounded_and_keeps_latest_events() {
        let mut model = SelfModel::with_memory_capacity("eda", "1.0.0", 2);

        model.record_memory(MemoryKind::Observation, "first", 80);
        model.record_memory(MemoryKind::Outcome, "second", 90);
        model.record_memory(MemoryKind::Error, "third", 20);

        assert_eq!(model.memory().len(), 2);
        assert_eq!(model.memory()[0].summary, "second");
        assert_eq!(model.memory()[1].summary, "third");
        assert_eq!(model.memory()[1].sequence, 3);
    }

    #[test]
    fn introspection_is_a_snapshot_of_operational_state() {
        let mut model = SelfModel::new("eda", "1.0.0");
        model.set_state(CognitiveState::Reasoning);
        model.set_confidence(75);
        model.add_goal(Goal {
            id: 1,
            title: "Validate a repair".into(),
            priority: 10,
            status: GoalStatus::Active,
        });
        model.add_capability("scenario generation");
        model.add_limitation("no security authority");
        model.record_memory(MemoryKind::Decision, "defer to policy", 100);

        let snapshot = model.introspect();

        assert_eq!(snapshot.agent_name, "eda");
        assert_eq!(snapshot.agent_version, "1.0.0");
        assert_eq!(snapshot.state, CognitiveState::Reasoning);
        assert_eq!(snapshot.confidence, 75);
        assert_eq!(snapshot.goals.len(), 1);
        assert_eq!(snapshot.capabilities, vec!["scenario generation"]);
        assert_eq!(snapshot.limitations, vec!["no security authority"]);
        assert_eq!(snapshot.memory_size, 1);
        assert_eq!(
            snapshot.last_memory.as_ref().map(|m| m.kind),
            Some(MemoryKind::Decision)
        );
    }

    #[test]
    fn changing_the_snapshot_does_not_change_the_model() {
        let mut model = SelfModel::new("eda", "1.0.0");
        model.add_capability("inspect");

        let mut snapshot = model.introspect();
        snapshot.capabilities.push("execute".into());

        assert_eq!(model.introspect().capabilities, vec!["inspect"]);
    }
}
