#[cfg(test)]
mod token_tracker_integration_tests {
    use codex_core::orchestration::{TokenBudget, TokenTracker};
    use std::sync::Arc;

    #[test]
    fn test_token_budget_enforcement() {
        let budget = TokenBudget {
            total_budget: 1000,
            warning_threshold: 800,
            per_agent_limit: Some(500),
        };
        let tracker = TokenTracker::new(budget);

        // Record usage within budget
        tracker.record_usage("agent1", 300, 100).unwrap();
        assert_eq!(tracker.get_total_usage(), 400);

        // Record more usage
        tracker.record_usage("agent2", 200, 100).unwrap();
        assert_eq!(tracker.get_total_usage(), 700);

        // Exceed budget should fail
        let result = tracker.record_usage("agent3", 500, 100);
        assert!(result.is_err(), "Should fail when exceeding budget");
    }

    #[test]
    fn test_per_agent_token_limit() {
        let budget = TokenBudget {
            total_budget: 10000,
            warning_threshold: 8000,
            per_agent_limit: Some(500),
        };
        let tracker = TokenTracker::new(budget);

        // Record usage below per-agent limit
        tracker.record_usage("agent1", 200, 100).unwrap();
        let usage = tracker.get_agent_usage("agent1").unwrap();
        assert_eq!(usage.total_tokens, 300);

        // Record usage exceeding per-agent limit (should warn but not fail)
        tracker.record_usage("agent1", 300, 100).unwrap();
        let usage = tracker.get_agent_usage("agent1").unwrap();
        assert_eq!(usage.total_tokens, 700);
    }

    #[test]
    fn test_multiple_agents_usage() {
        let budget = TokenBudget::default();
        let tracker = TokenTracker::new(budget);

        // Record usage for multiple agents
        tracker.record_usage("agent1", 100, 50).unwrap();
        tracker.record_usage("agent2", 200, 100).unwrap();
        tracker.record_usage("agent3", 150, 75).unwrap();

        let total = tracker.get_total_usage();
        assert_eq!(total, 150 + 300 + 225);

        let usages = tracker.get_all_usages();
        assert_eq!(usages.len(), 3);
    }

    #[test]
    fn test_token_usage_reset() {
        let budget = TokenBudget::default();
        let tracker = TokenTracker::new(budget);

        // Record some usage
        tracker.record_usage("agent1", 100, 50).unwrap();
        tracker.record_usage("agent2", 200, 100).unwrap();

        assert_eq!(tracker.get_total_usage(), 450);

        // Reset
        tracker.reset();
        assert_eq!(tracker.get_total_usage(), 0);
    }

    #[test]
    fn test_pair_session_lifecycle() {
        let budget = TokenBudget::default();
        let tracker = TokenTracker::new(budget);

        // Start session
        tracker.start_session(
            "session1".to_string(),
            vec!["alice".to_string(), "bob".to_string()],
            vec!["driver".to_string(), "navigator".to_string()],
        );

        let sessions = tracker.get_sessions();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].session_id, "session1");
        assert!(sessions[0].ended_at.is_none());

        // Add notes
        tracker.add_session_notes("session1", "Fixed bug X").unwrap();

        // End session
        tracker.end_session("session1").unwrap();

        let sessions = tracker.get_sessions();
        assert!(sessions[0].ended_at.is_some());
    }

    #[test]
    fn test_concurrent_token_tracking() {
        use std::thread;

        let budget = TokenBudget {
            total_budget: 100000,
            warning_threshold: 80000,
            per_agent_limit: None,
        };
        let tracker = Arc::new(TokenTracker::new(budget));

        let mut handles = vec![];

        // Spawn multiple threads recording usage
        for i in 0..10 {
            let tracker = Arc::clone(&tracker);
            let handle = thread::spawn(move || {
                let agent_id = format!("agent{}", i);
                for _ in 0..100 {
                    tracker.record_usage(&agent_id, 10, 5).unwrap();
                }
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Each agent recorded 100 times with 15 tokens each
        // 10 agents * 100 * 15 = 15000 total tokens
        assert_eq!(tracker.get_total_usage(), 15000);
    }

    #[test]
    fn test_token_usage_by_agent() {
        let budget = TokenBudget::default();
        let tracker = TokenTracker::new(budget);

        tracker.record_usage("agent1", 100, 50).unwrap();
        tracker.record_usage("agent1", 200, 100).unwrap();
        tracker.record_usage("agent2", 150, 75).unwrap();

        let agent1_usage = tracker.get_agent_usage("agent1").unwrap();
        assert_eq!(agent1_usage.prompt_tokens, 300);
        assert_eq!(agent1_usage.completion_tokens, 150);
        assert_eq!(agent1_usage.total_tokens, 450);

        let agent2_usage = tracker.get_agent_usage("agent2").unwrap();
        assert_eq!(agent2_usage.total_tokens, 225);
    }

    #[test]
    fn test_session_not_found() {
        let budget = TokenBudget::default();
        let tracker = TokenTracker::new(budget);

        let result = tracker.end_session("nonexistent");
        assert!(result.is_err());

        let result = tracker.add_session_notes("nonexistent", "test");
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_concurrent_sessions() {
        let budget = TokenBudget::default();
        let tracker = TokenTracker::new(budget);

        // Start multiple sessions
        tracker.start_session(
            "session1".to_string(),
            vec!["alice".to_string(), "bob".to_string()],
            vec!["driver".to_string(), "navigator".to_string()],
        );

        tracker.start_session(
            "session2".to_string(),
            vec!["charlie".to_string(), "diana".to_string()],
            vec!["driver".to_string(), "navigator".to_string()],
        );

        let sessions = tracker.get_sessions();
        assert_eq!(sessions.len(), 2);

        // End one session
        tracker.end_session("session1").unwrap();

        let sessions = tracker.get_sessions();
        let active_sessions: Vec<_> = sessions.iter().filter(|s| s.ended_at.is_none()).collect();
        assert_eq!(active_sessions.len(), 1);
    }
}
