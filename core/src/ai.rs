#![forbid(unsafe_code)]

//! Deterministic AI foundation boundary.
//!
//! AI data is untrusted. This module provides typed proposals, a fixed
//! capability registry, deterministic policy validation, sanitized results,
//! and test adapters. It does not contain an LLM, provider SDK, keys,
//! cryptographic operations, or security authority.

use std::collections::{BTreeSet, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RequestId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlanId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ToolId(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvidenceId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyVersion(pub u16);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntrustedText(String);

impl UntrustedText {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedactedText(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedactionError {
    Empty,
    SecretLikeContent,
    Oversized,
}

impl RedactedText {
    pub fn from_untrusted(input: &UntrustedText) -> Result<Self, RedactionError> {
        let value = input.as_str().trim();
        if value.is_empty() {
            return Err(RedactionError::Empty);
        }
        if value.len() > 4096 {
            return Err(RedactionError::Oversized);
        }

        let lower = value.to_ascii_lowercase();
        let secret_markers = [
            "private key",
            "secret key",
            "session key",
            "api_key",
            "api-key",
            "authorization: bearer",
            "password=",
            "token=",
        ];

        if secret_markers.iter().any(|marker| lower.contains(marker)) {
            return Err(RedactionError::SecretLikeContent);
        }

        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafeArguments(BTreeSet<String>);

impl SafeArguments {
    pub fn empty() -> Self {
        Self(BTreeSet::new())
    }

    pub fn from_pairs<I, K, V>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let mut values = BTreeSet::new();
        for (key, value) in pairs {
            values.insert(format!("{}={}", key.into(), value.into()));
        }
        Self(values)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CapabilityRequest {
    ReadConversationSummary,
    SearchLocalIndex,
    CreateDraft,
    ReadConnectionDiagnostic,
    RequestRetry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestSource {
    User,
    System,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiRequest {
    pub request_id: RequestId,
    pub source: RequestSource,
    pub input: UntrustedText,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiObservation {
    UserIntent(RedactedText),
    ToolResult(SanitizedToolResult),
    SystemState(RedactedText),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiContext {
    pub request_id: RequestId,
    pub observations: Vec<AiObservation>,
    pub allowed_capabilities: BTreeSet<CapabilityRequest>,
    pub policy_version: PolicyVersion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfidenceAssessment {
    pub model_score: u8,
    pub evidence_score: u8,
    pub consistency_score: u8,
    pub rule_validation_score: u8,
    pub source_trust_score: u8,
    pub final_score: u8,
}

impl ConfidenceAssessment {
    pub fn calculate(
        model_score: u8,
        evidence_score: u8,
        consistency_score: u8,
        rule_validation_score: u8,
        source_trust_score: u8,
    ) -> Self {
        let model_score = model_score.min(100);
        let evidence_score = evidence_score.min(100);
        let consistency_score = consistency_score.min(100);
        let rule_validation_score = rule_validation_score.min(100);
        let source_trust_score = source_trust_score.min(100);

        let final_score = ((model_score as u16
            + evidence_score as u16
            + consistency_score as u16
            + rule_validation_score as u16
            + source_trust_score as u16)
            / 5) as u8;

        Self {
            model_score,
            evidence_score,
            consistency_score,
            rule_validation_score,
            source_trust_score,
            final_score,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiPlan {
    pub plan_id: PlanId,
    pub request_id: RequestId,
    pub policy_version: PolicyVersion,
    pub steps: Vec<PlanStep>,
    pub confidence: ConfidenceAssessment,
    pub requires_confirmation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanStep {
    pub capability: CapabilityRequest,
    pub arguments: SafeArguments,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SanitizedToolResult {
    pub tool_id: ToolId,
    pub success: bool,
    pub summary: RedactedText,
    pub evidence_id: Option<EvidenceId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolResult {
    pub sanitized: SanitizedToolResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvaluationOutcome {
    Accepted,
    Rejected,
    NeedsReview,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiEvaluation {
    pub plan_id: PlanId,
    pub outcome: EvaluationOutcome,
    pub confidence: ConfidenceAssessment,
    pub evidence_id: EvidenceId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SanitizedMemoryEvent {
    pub sequence: u64,
    pub kind: MemoryKind,
    pub evidence_id: Option<EvidenceId>,
    pub summary: RedactedText,
    pub confidence: ConfidenceAssessment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryKind {
    Observation,
    Decision,
    Action,
    Outcome,
    Error,
}

pub trait AiModelAdapter {
    fn generate(&self, request: &ModelRequest) -> Result<ModelResponse, ModelError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelRequest {
    pub request_id: RequestId,
    pub input: UntrustedText,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelResponse {
    pub plan: AiPlan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelError {
    Unavailable,
    InvalidOutput,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct RuleBasedAdapter;

impl AiModelAdapter for RuleBasedAdapter {
    fn generate(&self, request: &ModelRequest) -> Result<ModelResponse, ModelError> {
        let input = request.input.as_str().to_ascii_lowercase();
        let capability = if input.contains("search") {
            CapabilityRequest::SearchLocalIndex
        } else if input.contains("draft") {
            CapabilityRequest::CreateDraft
        } else if input.contains("connection") {
            CapabilityRequest::ReadConnectionDiagnostic
        } else {
            return Err(ModelError::InvalidOutput);
        };

        Ok(ModelResponse {
            plan: AiPlan {
                plan_id: PlanId(request.request_id.0),
                request_id: request.request_id,
                policy_version: PolicyVersion(1),
                steps: vec![PlanStep {
                    capability,
                    arguments: SafeArguments::empty(),
                }],
                confidence: ConfidenceAssessment::calculate(80, 0, 90, 0, 0),
                requires_confirmation: false,
            },
        })
    }
}

#[derive(Debug, Clone)]
pub struct TestAdapter {
    response: Result<ModelResponse, ModelError>,
}

impl TestAdapter {
    pub fn returning(response: ModelResponse) -> Self {
        Self {
            response: Ok(response),
        }
    }

    pub fn failing(error: ModelError) -> Self {
        Self { response: Err(error) }
    }
}

impl AiModelAdapter for TestAdapter {
    fn generate(&self, _request: &ModelRequest) -> Result<ModelResponse, ModelError> {
        self.response.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyRejection {
    WrongPolicyVersion,
    EmptyPlan,
    TooManySteps,
    UnknownCapability,
    CapabilityNotAllowed,
    DuplicateRequest,
    DuplicatePlan,
    InvalidArguments,
    UntrustedConfidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovedPlan(AiPlan);

impl ApprovedPlan {
    pub fn plan(&self) -> &AiPlan {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    Approved(ApprovedPlan),
    Rejected(PolicyRejection),
    RequiresConfirmation(AiPlan),
}

#[derive(Debug, Clone)]
pub struct AiPolicyGuard {
    policy_version: PolicyVersion,
    allowed_capabilities: BTreeSet<CapabilityRequest>,
    max_steps: usize,
    seen_requests: HashSet<RequestId>,
    seen_plans: HashSet<PlanId>,
}

impl AiPolicyGuard {
    pub fn new(
        policy_version: PolicyVersion,
        allowed_capabilities: impl IntoIterator<Item = CapabilityRequest>,
        max_steps: usize,
    ) -> Self {
        Self {
            policy_version,
            allowed_capabilities: allowed_capabilities.into_iter().collect(),
            max_steps: max_steps.max(1),
            seen_requests: HashSet::new(),
            seen_plans: HashSet::new(),
        }
    }

    pub fn validate(&mut self, context: &AiContext, plan: AiPlan) -> PolicyDecision {
        if plan.policy_version != self.policy_version
            || context.policy_version != self.policy_version
        {
            return PolicyDecision::Rejected(PolicyRejection::WrongPolicyVersion);
        }
        if plan.steps.is_empty() {
            return PolicyDecision::Rejected(PolicyRejection::EmptyPlan);
        }
        if plan.steps.len() > self.max_steps {
            return PolicyDecision::Rejected(PolicyRejection::TooManySteps);
        }
        if plan.request_id != context.request_id {
            return PolicyDecision::Rejected(PolicyRejection::InvalidArguments);
        }
        if !self.seen_requests.insert(plan.request_id) {
            return PolicyDecision::Rejected(PolicyRejection::DuplicateRequest);
        }
        if !self.seen_plans.insert(plan.plan_id) {
            return PolicyDecision::Rejected(PolicyRejection::DuplicatePlan);
        }

        for step in &plan.steps {
            if !self.allowed_capabilities.contains(&step.capability) {
                return PolicyDecision::Rejected(PolicyRejection::UnknownCapability);
            }
            if !context.allowed_capabilities.contains(&step.capability) {
                return PolicyDecision::Rejected(PolicyRejection::CapabilityNotAllowed);
            }
            if step.arguments.len() > 16 {
                return PolicyDecision::Rejected(PolicyRejection::InvalidArguments);
            }
        }

        if plan.confidence.rule_validation_score < 50
            || plan.confidence.consistency_score < 50
        {
            return PolicyDecision::Rejected(PolicyRejection::UntrustedConfidence);
        }

        if plan.requires_confirmation {
            return PolicyDecision::RequiresConfirmation(plan);
        }

        PolicyDecision::Approved(ApprovedPlan(plan))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegisteredTool {
    pub capability: CapabilityRequest,
    pub tool_id: ToolId,
    pub risk: RiskLevel,
}

#[derive(Debug, Clone)]
pub struct ToolRegistry {
    tools: Vec<RegisteredTool>,
}

impl ToolRegistry {
    pub fn new(tools: impl IntoIterator<Item = RegisteredTool>) -> Self {
        Self {
            tools: tools.into_iter().collect(),
        }
    }

    pub fn resolve(&self, capability: CapabilityRequest) -> Option<RegisteredTool> {
        self.tools.iter().copied().find(|tool| tool.capability == capability)
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxError {
    UnknownCapability,
    ExecutionDenied,
}

#[derive(Debug, Clone)]
pub struct ToolSandbox {
    registry: ToolRegistry,
}

impl ToolSandbox {
    pub fn new(registry: ToolRegistry) -> Self {
        Self { registry }
    }

    pub fn execute(&self, approved: &ApprovedPlan) -> Result<ToolResult, SandboxError> {
        let step = approved
            .plan()
            .steps
            .first()
            .ok_or(SandboxError::ExecutionDenied)?;

        let tool = self
            .registry
            .resolve(step.capability)
            .ok_or(SandboxError::UnknownCapability)?;

        let summary = RedactedText::from_untrusted(&UntrustedText::new(
            "tool executed; result intentionally sanitized",
        ))
        .map_err(|_| SandboxError::ExecutionDenied)?;

        Ok(ToolResult {
            sanitized: SanitizedToolResult {
                tool_id: tool.tool_id,
                success: true,
                summary,
                evidence_id: Some(EvidenceId(approved.plan().plan_id.0)),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context(request_id: RequestId) -> AiContext {
        AiContext {
            request_id,
            observations: Vec::new(),
            allowed_capabilities: [
                CapabilityRequest::SearchLocalIndex,
                CapabilityRequest::CreateDraft,
            ]
            .into_iter()
            .collect(),
            policy_version: PolicyVersion(1),
        }
    }

    fn plan(request_id: RequestId, plan_id: u64, capability: CapabilityRequest) -> AiPlan {
        AiPlan {
            plan_id: PlanId(plan_id),
            request_id,
            policy_version: PolicyVersion(1),
            steps: vec![PlanStep {
                capability,
                arguments: SafeArguments::empty(),
            }],
            confidence: ConfidenceAssessment::calculate(80, 80, 90, 100, 70),
            requires_confirmation: false,
        }
    }

    #[test]
    fn confidence_is_derived_and_bounded() {
        let assessment = ConfidenceAssessment::calculate(255, 200, 101, 99, 80);
        assert_eq!(assessment.model_score, 100);
        assert_eq!(assessment.evidence_score, 100);
        assert_eq!(assessment.consistency_score, 100);
        assert_eq!(assessment.final_score, 95);
    }

    #[test]
    fn policy_approves_only_fixed_allowed_capabilities() {
        let mut guard = AiPolicyGuard::new(
            PolicyVersion(1),
            [CapabilityRequest::SearchLocalIndex, CapabilityRequest::CreateDraft],
            4,
        );

        let request_id = RequestId(1);
        let decision = guard.validate(
            &context(request_id),
            plan(request_id, 1, CapabilityRequest::SearchLocalIndex),
        );

        assert!(matches!(decision, PolicyDecision::Approved(_)));
    }

    #[test]
    fn policy_rejects_capability_not_in_registry_policy() {
        let mut guard = AiPolicyGuard::new(
            PolicyVersion(1),
            [CapabilityRequest::SearchLocalIndex],
            4,
        );

        let request_id = RequestId(2);
        let decision = guard.validate(
            &context(request_id),
            plan(request_id, 2, CapabilityRequest::CreateDraft),
        );

        assert_eq!(
            decision,
            PolicyDecision::Rejected(PolicyRejection::CapabilityNotAllowed)
        );
    }

    #[test]
    fn policy_rejects_stale_policy_version() {
        let mut guard =
            AiPolicyGuard::new(PolicyVersion(2), [CapabilityRequest::SearchLocalIndex], 4);
        let request_id = RequestId(3);
        let decision = guard.validate(&context(request_id), plan(request_id, 3, CapabilityRequest::SearchLocalIndex));
        assert_eq!(
            decision,
            PolicyDecision::Rejected(PolicyRejection::WrongPolicyVersion)
        );
    }

    #[test]
    fn policy_rejects_replayed_request() {
        let mut guard =
            AiPolicyGuard::new(PolicyVersion(1), [CapabilityRequest::SearchLocalIndex], 4);
        let request_id = RequestId(4);

        assert!(matches!(
            guard.validate(
                &context(request_id),
                plan(request_id, 4, CapabilityRequest::SearchLocalIndex)
            ),
            PolicyDecision::Approved(_)
        ));

        assert_eq!(
            guard.validate(
                &context(request_id),
                plan(request_id, 5, CapabilityRequest::SearchLocalIndex)
            ),
            PolicyDecision::Rejected(PolicyRejection::DuplicateRequest)
        );
    }

    #[test]
    fn policy_rejects_untrusted_confidence() {
        let mut guard =
            AiPolicyGuard::new(PolicyVersion(1), [CapabilityRequest::SearchLocalIndex], 4);
        let request_id = RequestId(5);
        let mut candidate = plan(request_id, 5, CapabilityRequest::SearchLocalIndex);
        candidate.confidence = ConfidenceAssessment::calculate(100, 100, 10, 100, 100);

        assert_eq!(
            guard.validate(&context(request_id), candidate),
            PolicyDecision::Rejected(PolicyRejection::UntrustedConfidence)
        );
    }

    #[test]
    fn sensitive_capabilities_are_not_part_of_ai_capability_enum() {
        assert_eq!(std::mem::variant_count::<CapabilityRequest>(), 5);
    }

    #[test]
    fn secret_like_memory_content_is_rejected_before_storage() {
        let secret = UntrustedText::new("authorization: bearer abc123");
        assert_eq!(
            RedactedText::from_untrusted(&secret),
            Err(RedactionError::SecretLikeContent)
        );
    }

    #[test]
    fn tool_registry_is_fixed_and_execution_returns_sanitized_result() {
        let registry = ToolRegistry::new([RegisteredTool {
            capability: CapabilityRequest::SearchLocalIndex,
            tool_id: ToolId(1),
            risk: RiskLevel::Low,
        }]);
        assert_eq!(registry.len(), 1);

        let sandbox = ToolSandbox::new(registry);
        let mut guard =
            AiPolicyGuard::new(PolicyVersion(1), [CapabilityRequest::SearchLocalIndex], 1);
        let request_id = RequestId(6);

        let decision = guard.validate(
            &context(request_id),
            plan(request_id, 6, CapabilityRequest::SearchLocalIndex),
        );

        let approved = match decision {
            PolicyDecision::Approved(plan) => plan,
            other => panic!("unexpected decision: {other:?}"),
        };

        let result = sandbox.execute(&approved).expect("approved plan should execute");
        assert!(result.sanitized.success);
        assert_eq!(result.sanitized.tool_id, ToolId(1));
        assert_eq!(
            result.sanitized.summary.as_str(),
            "tool executed; result intentionally sanitized"
        );
    }

    #[test]
    fn model_adapter_cannot_execute_a_plan() {
        let adapter = RuleBasedAdapter;
        let response = adapter
            .generate(&ModelRequest {
                request_id: RequestId(7),
                input: UntrustedText::new("search messages"),
            })
            .expect("rule adapter should produce a proposal");

        assert_eq!(
            response.plan.steps[0].capability,
            CapabilityRequest::SearchLocalIndex
        );
    }
}
