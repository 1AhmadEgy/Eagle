use eagle_core::{CognitiveState, Goal, GoalStatus, MemoryKind, SelfModel};

#[test]
fn self_model_supports_observe_reason_act_verify_cycle() {
    let mut model = SelfModel::new("eda", "1.0.0");

    model.set_state(CognitiveState::Observing);
    model.record_memory(MemoryKind::Observation, "validation failure observed", 95);

    model.set_state(CognitiveState::Reasoning);
    model.record_memory(
        MemoryKind::Decision,
        "repair is bounded to source files",
        90,
    );

    model.set_state(CognitiveState::Acting);
    model.record_memory(MemoryKind::Action, "applied minimal repair", 85);

    model.set_state(CognitiveState::Verifying);
    model.record_memory(MemoryKind::Outcome, "verification passed", 100);

    let snapshot = model.introspect();

    assert_eq!(snapshot.state, CognitiveState::Verifying);
    assert_eq!(snapshot.memory_size, 4);
    assert_eq!(
        snapshot
            .last_memory
            .as_ref()
            .map(|event| event.summary.as_str()),
        Some("verification passed")
    );
}

#[test]
fn goals_and_limits_are_visible_to_introspection() {
    let mut model = SelfModel::new("eda", "1.0.0");
    model.add_goal(Goal {
        id: 7,
        title: "Preserve security invariants".into(),
        priority: 100,
        status: GoalStatus::Active,
    });
    model.add_limitation("AI output is untrusted until verified");

    let snapshot = model.introspect();

    assert_eq!(snapshot.goals[0].priority, 100);
    assert_eq!(
        snapshot.limitations,
        vec!["AI output is untrusted until verified"]
    );
}
