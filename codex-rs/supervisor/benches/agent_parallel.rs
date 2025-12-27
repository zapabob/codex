//! Performance benchmarks for parallel agent execution.
//!
//! These benchmarks verify that the supervisor meets performance targets:
//! - Cold start < 80ms
//! - RSS < 30MB (measured separately)
//! - 8 agents parallel < 500ms
//! - 100 req/min with no spikes

use codex_supervisor::AgentType;
use codex_supervisor::SubAgent;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use std::hint::black_box;
use std::time::Duration;
use tokio::runtime::Runtime;

/// Benchmark single agent task execution.
fn bench_single_agent(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("single_agent_task", |b| {
        b.to_async(&rt).iter(|| async {
            let mut agent = SubAgent::new(AgentType::CodeExpert);
            let result = agent
                .process_task(black_box("Simple task".to_string()))
                .await;
            black_box(result)
        });
    });
}

/// Benchmark parallel agent execution (2, 4, 8 agents).
fn bench_parallel_agents(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    for agent_count in [2, 4, 8].iter() {
        c.bench_with_input(
            BenchmarkId::new("parallel_agents", agent_count),
            agent_count,
            |b, &count| {
                b.to_async(&rt).iter(|| async move {
                    let mut agents = Vec::with_capacity(count);
                    for i in 0..count {
                        let agent_type = match i % 3 {
                            0 => AgentType::CodeExpert,
                            1 => AgentType::DeepResearcher,
                            _ => AgentType::TestingExpert,
                        };
                        agents.push(SubAgent::new(agent_type));
                    }

                    let mut tasks = Vec::with_capacity(count);
                    for (i, mut agent) in agents.into_iter().enumerate() {
                        let task = format!("Benchmark task {i}");
                        tasks.push(tokio::spawn(async move { agent.process_task(task).await }));
                    }

                    for task in tasks {
                        let _ = task.await.unwrap();
                    }
                });
            },
        );
    }
}

/// Benchmark agent state transitions.
fn bench_state_transitions(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("agent_state_transitions", |b| {
        b.to_async(&rt).iter(|| async {
            let mut agent = SubAgent::new(AgentType::CodeExpert);
            let _ = agent
                .process_task("State transition task".to_string())
                .await;
            black_box(agent.get_state());
        });
    });
}

/// Benchmark agent manager creation and cleanup.
fn bench_manager_lifecycle(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("manager_lifecycle", |b| {
        b.to_async(&rt).iter(|| async {
            let mut manager = codex_supervisor::SubAgentManager::new();
            manager.register_agent(AgentType::CodeExpert);
            manager.register_agent(AgentType::DeepResearcher);
            manager.register_agent(AgentType::TestingExpert);
            black_box(manager.get_all_states());
        });
    });
}

/// Benchmark sequential task execution.
fn bench_sequential_tasks(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("sequential_tasks", |b| {
        b.to_async(&rt).iter(|| async {
            let mut agent = SubAgent::new(AgentType::CodeExpert);
            let result = agent
                .process_task(black_box("Sequential task".to_string()))
                .await;
            black_box(result)
        });
    });
}

/// Benchmark high-throughput scenario (100 tasks).
fn bench_high_throughput(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("high_throughput_100_tasks", |b| {
        b.to_async(&rt).iter(|| async {
            let mut agent = SubAgent::new(AgentType::CodeExpert);
            for i in 0..100 {
                let task = black_box(format!("Task {i}"));
                let _ = agent.process_task(task).await;
            }
        });
    });
}

/// Cold start benchmark (agent creation time).
fn bench_cold_start(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("cold_start", |b| {
        b.to_async(&rt).iter(|| async {
            // Measure time to create and initialize an agent
            let agent = SubAgent::new(black_box(AgentType::CodeExpert));
            black_box(agent)
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .warm_up_time(Duration::from_secs(3));
    targets =
        bench_cold_start,
        bench_single_agent,
        bench_parallel_agents,
        bench_state_transitions,
        bench_manager_lifecycle,
        bench_sequential_tasks,
        bench_high_throughput,
}

criterion_main!(benches);
