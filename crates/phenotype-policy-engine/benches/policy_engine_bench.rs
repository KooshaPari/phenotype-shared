//! Benchmarks for phenotype-policy-engine

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use phenotype_policy_engine::{
    context::EvaluationContext,
    engine::PolicyEngine,
    policy::Policy,
    rule::{Rule, RuleType},
};

fn bench_engine_new(c: &mut Criterion) {
    c.bench_function("policy_engine_new", |b| {
        b.iter(|| PolicyEngine::new());
    });
}

fn bench_engine_with_policies(c: &mut Criterion) {
    let policies = vec![
        Policy::new("policy1"),
        Policy::new("policy2"),
        Policy::new("policy3"),
        Policy::new("policy4"),
        Policy::new("policy5"),
    ];
    c.bench_function("policy_engine_with_policies_5", |b| {
        b.iter(|| PolicyEngine::with_policies(black_box(policies.clone())));
    });
}

fn bench_engine_add_policy(c: &mut Criterion) {
    let engine = PolicyEngine::new();
    let policy = Policy::new("test_policy");
    c.bench_function("policy_engine_add_policy", |b| {
        b.iter(|| engine.add_policy(black_box(policy.clone())));
    });
}

fn bench_engine_evaluate_single_policy(c: &mut Criterion) {
    let engine = PolicyEngine::new();
    let rule = Rule::new(RuleType::Allow, "status", "^active$");
    let policy = Policy::new("status_policy").add_rule(rule);
    engine.add_policy(policy);

    let mut ctx = EvaluationContext::new();
    ctx.set_string("status", "active");

    c.bench_function("policy_engine_evaluate_single", |b| {
        b.iter(|| engine.evaluate_policy(black_box("status_policy"), black_box(&ctx)).unwrap());
    });
}

fn bench_engine_evaluate_all(c: &mut Criterion) {
    let engine = PolicyEngine::new();
    engine.add_policy(Policy::new("policy1").add_rule(Rule::new(
        RuleType::Allow,
        "status",
        "^active$",
    )));
    engine.add_policy(Policy::new("policy2").add_rule(Rule::new(
        RuleType::Allow,
        "role",
        "^user$",
    )));
    engine.add_policy(Policy::new("policy3").add_rule(Rule::new(
        RuleType::Require,
        "email",
        ".+@.+",
    )));

    let mut ctx = EvaluationContext::new();
    ctx.set_string("status", "active");
    ctx.set_string("role", "user");
    ctx.set_string("email", "test@example.com");

    c.bench_function("policy_engine_evaluate_all_3_policies", |b| {
        b.iter(|| engine.evaluate_all(black_box(&ctx)).unwrap());
    });
}

fn bench_engine_evaluate_all_many_policies(c: &mut Criterion) {
    let engine = PolicyEngine::new();
    for i in 0..20 {
        let rule = Rule::new(RuleType::Allow, format!("field{}", i), format!("^value{}$", i));
        engine.add_policy(Policy::new(format!("policy{}", i)).add_rule(rule));
    }

    let mut ctx = EvaluationContext::new();
    for i in 0..20 {
        ctx.set_string(format!("field{}", i), format!("value{}", i));
    }

    c.bench_function("policy_engine_evaluate_all_20_policies", |b| {
        b.iter(|| engine.evaluate_all(black_box(&ctx)).unwrap());
    });
}

fn bench_context_new(c: &mut Criterion) {
    c.bench_function("context_new", |b| {
        b.iter(|| EvaluationContext::new());
    });
}

fn bench_context_with_map(c: &mut Criterion) {
    let mut facts = std::collections::HashMap::new();
    for i in 0..10 {
        facts.insert(format!("key{}", i), serde_json::json!(format!("value{}", i)));
    }
    c.bench_function("context_from_map_10_facts", |b| {
        b.iter(|| EvaluationContext::from_map(black_box(facts.clone())));
    });
}

fn bench_context_set_string(c: &mut Criterion) {
    let mut ctx = EvaluationContext::new();
    c.bench_function("context_set_string", |b| {
        b.iter(|| ctx.set_string(black_box("key"), black_box("value")));
    });
}

fn bench_context_get_string(c: &mut Criterion) {
    let mut ctx = EvaluationContext::new();
    ctx.set_string("key", "value");
    c.bench_function("context_get_string", |b| {
        b.iter(|| ctx.get_string(black_box("key")));
    });
}

fn bench_rule_evaluate_allow(c: &mut Criterion) {
    let rule = Rule::new(RuleType::Allow, "status", "^active$");
    let mut ctx = EvaluationContext::new();
    ctx.set_string("status", "active");
    c.bench_function("rule_evaluate_allow_match", |b| {
        b.iter(|| rule.evaluate(black_box(&ctx)));
    });
}

fn bench_rule_evaluate_deny(c: &mut Criterion) {
    let rule = Rule::new(RuleType::Deny, "status", "^banned$");
    let mut ctx = EvaluationContext::new();
    ctx.set_string("status", "active");
    c.bench_function("rule_evaluate_deny_no_match", |b| {
        b.iter(|| rule.evaluate(black_box(&ctx)));
    });
}

fn bench_rule_evaluate_require(c: &mut Criterion) {
    let rule = Rule::new(RuleType::Require, "email", "^[a-z]+@example\\.com$");
    let mut ctx = EvaluationContext::new();
    ctx.set_string("email", "user@example.com");
    c.bench_function("rule_evaluate_require_match", |b| {
        b.iter(|| rule.evaluate(black_box(&ctx)));
    });
}

criterion_group!(
    benches,
    bench_engine_new,
    bench_engine_with_policies,
    bench_engine_add_policy,
    bench_engine_evaluate_single_policy,
    bench_engine_evaluate_all,
    bench_engine_evaluate_all_many_policies,
    bench_context_new,
    bench_context_with_map,
    bench_context_set_string,
    bench_context_get_string,
    bench_rule_evaluate_allow,
    bench_rule_evaluate_deny,
    bench_rule_evaluate_require
);
criterion_main!(benches);
