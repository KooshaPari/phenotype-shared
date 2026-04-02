//! Benchmarks for phenotype-domain value objects

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use phenotype_domain::{AgentId, TaskId};

fn bench_task_id_new(c: &mut Criterion) {
    c.bench_function("task_id_new", |b| {
        b.iter(|| TaskId::new());
    });
}

fn bench_task_id_parse_valid(c: &mut Criterion) {
    c.bench_function("task_id_parse_valid", |b| {
        b.iter(|| TaskId::parse(black_box("task12345678901234567890123456")).unwrap());
    });
}

fn bench_task_id_parse_invalid_empty(c: &mut Criterion) {
    c.bench_function("task_id_parse_invalid_empty", |b| {
        b.iter(|| TaskId::parse(black_box("")));
    });
}

fn bench_task_id_is_valid(c: &mut Criterion) {
    let id = TaskId::new();
    c.bench_function("task_id_is_valid", |b| {
        b.iter(|| id.is_valid());
    });
}

fn bench_task_id_as_str(c: &mut Criterion) {
    let id = TaskId::new();
    c.bench_function("task_id_as_str", |b| {
        b.iter(|| id.as_str());
    });
}

fn bench_agent_id_new(c: &mut Criterion) {
    c.bench_function("agent_id_new", |b| {
        b.iter(|| AgentId::new());
    });
}

fn bench_agent_id_parse_valid(c: &mut Criterion) {
    c.bench_function("agent_id_parse_valid", |b| {
        b.iter(|| AgentId::parse(black_box("0123456789ABCDEF0123456789")).unwrap());
    });
}

fn bench_agent_id_parse_invalid_hex(c: &mut Criterion) {
    c.bench_function("agent_id_parse_invalid_hex", |b| {
        b.iter(|| AgentId::parse(black_box("not-hex!")));
    });
}

fn bench_agent_id_is_ulid_format(c: &mut Criterion) {
    let id = AgentId::new();
    c.bench_function("agent_id_is_ulid_format", |b| {
        b.iter(|| id.is_ulid_format());
    });
}

fn bench_agent_id_as_str(c: &mut Criterion) {
    let id = AgentId::new();
    c.bench_function("agent_id_as_str", |b| {
        b.iter(|| id.as_str());
    });
}

criterion_group!(
    benches,
    bench_task_id_new,
    bench_task_id_parse_valid,
    bench_task_id_parse_invalid_empty,
    bench_task_id_is_valid,
    bench_task_id_as_str,
    bench_agent_id_new,
    bench_agent_id_parse_valid,
    bench_agent_id_parse_invalid_hex,
    bench_agent_id_is_ulid_format,
    bench_agent_id_as_str
);
criterion_main!(benches);
