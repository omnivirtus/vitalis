//! Contest Resolution - Universal mathematical system for Thread interactions
//!
//! All Thread interactions are resolved through mathematical contests using
//! the universal formula: 50% + (stat_difference × 5%) + d20 + luck_modifier

use super::properties::{Stat, ThreadProperties};
use super::states::ThreadStates;
use rand::Rng;

/// Result of a contest between two Threads
#[derive(Debug, PartialEq)]
pub enum ContestResult {
    /// The initiating Thread succeeded
    Success,
    /// The initiating Thread failed
    Failure,
}

/// Resolve a contest between two Threads using specified stats
///
/// Formula: 50% + (stat_difference × 5%) + d20 + luck_modifier
pub fn resolve_contest(
    initiator_props: &ThreadProperties,
    initiator_states: &ThreadStates,
    initiator_stat: Stat,
    defender_props: &ThreadProperties,
    defender_states: &ThreadStates,
    defender_stat: Stat,
) -> ContestResult {
    let mut rng = rand::thread_rng();

    // Get base stats
    let initiator_value = initiator_props.get_stat(initiator_stat) as i32;
    let defender_value = defender_props.get_stat(defender_stat) as i32;

    // Calculate stat difference modifier (5% per point difference)
    let stat_diff = initiator_value - defender_value;
    let stat_modifier = stat_diff * 5;

    // Roll d20
    let roll: i32 = rng.gen_range(1..=20);

    // Luck modifiers (luck affects d20 roll, ±2 per 10 points)
    let initiator_luck = initiator_props.luck as i32;
    let defender_luck = defender_props.luck as i32;
    let luck_modifier = (initiator_luck - defender_luck) / 5;

    // State modifiers (future: implement state effects on contests)
    let _state_modifier = calculate_state_modifier(initiator_states, defender_states);

    // Total probability: 50 (base) + stat_modifier + roll + luck_modifier
    let total = 50 + stat_modifier + roll + luck_modifier;

    // Success if total >= 50
    if total >= 50 {
        ContestResult::Success
    } else {
        ContestResult::Failure
    }
}

/// Calculate modifier from Thread states (currently stubbed)
fn calculate_state_modifier(_initiator_states: &ThreadStates, _defender_states: &ThreadStates) -> i32 {
    // Future: Implement state effects on contests
    // - Damaged/Corrupted/Stressed reduce effectiveness
    // - Enhanced/Blessed/Experienced increase effectiveness
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_threads_have_roughly_50_percent_success() {
        let props = ThreadProperties::default();
        let states = ThreadStates::default();

        let mut successes = 0;
        let iterations = 1000;

        for _ in 0..iterations {
            if resolve_contest(&props, &states, Stat::Strength, &props, &states, Stat::Strength)
                == ContestResult::Success
            {
                successes += 1;
            }
        }

        // Should be roughly 50% (allow 40-60% for randomness)
        let success_rate = (successes as f32 / iterations as f32) * 100.0;
        assert!(success_rate > 40.0 && success_rate < 60.0);
    }

    #[test]
    fn higher_stat_increases_success_rate() {
        let strong_props = ThreadProperties {
            strength: 18,
            ..Default::default()
        };
        let weak_props = ThreadProperties {
            strength: 8,
            ..Default::default()
        };
        let states = ThreadStates::default();

        let mut successes = 0;
        let iterations = 1000;

        for _ in 0..iterations {
            if resolve_contest(
                &strong_props,
                &states,
                Stat::Strength,
                &weak_props,
                &states,
                Stat::Strength,
            ) == ContestResult::Success
            {
                successes += 1;
            }
        }

        // Strong vs weak should win significantly more than 50%
        let success_rate = (successes as f32 / iterations as f32) * 100.0;
        assert!(success_rate > 60.0);
    }
}
