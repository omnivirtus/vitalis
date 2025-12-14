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
/// Uses opposed rolls: Both sides roll d20 + stat + luck modifier
/// Higher roll wins, ties go to defender
pub fn resolve_contest(
    initiator_props: &ThreadProperties,
    initiator_states: &ThreadStates,
    initiator_stat: Stat,
    defender_props: &ThreadProperties,
    defender_states: &ThreadStates,
    defender_stat: Stat,
) -> ContestResult {
    let mut rng = rand::thread_rng();

    // Both sides roll d20
    let initiator_d20: i32 = rng.gen_range(1..=20);
    let defender_d20: i32 = rng.gen_range(1..=20);

    // Get stats
    let initiator_stat_value = initiator_props.get_stat(initiator_stat) as i32;
    let defender_stat_value = defender_props.get_stat(defender_stat) as i32;

    // Calculate luck modifiers (±1 per 5 points of luck)
    let initiator_luck_mod = initiator_props.luck as i32 / 5;
    let defender_luck_mod = defender_props.luck as i32 / 5;

    // State modifiers (future: implement state effects on contests)
    let (initiator_state_mod, defender_state_mod) =
        calculate_state_modifiers(initiator_states, defender_states);

    // Calculate total rolls
    let initiator_total = initiator_d20 + initiator_stat_value + initiator_luck_mod + initiator_state_mod;
    let defender_total = defender_d20 + defender_stat_value + defender_luck_mod + defender_state_mod;

    // Higher roll wins, ties go to defender
    if initiator_total > defender_total {
        ContestResult::Success
    } else {
        ContestResult::Failure
    }
}

/// Calculate modifiers from Thread states (currently stubbed)
/// Returns (initiator_modifier, defender_modifier)
fn calculate_state_modifiers(_initiator_states: &ThreadStates, _defender_states: &ThreadStates) -> (i32, i32) {
    // Future: Implement state effects on contests
    // - Damaged/Corrupted/Stressed reduce effectiveness
    // - Enhanced/Blessed/Experienced increase effectiveness
    (0, 0)
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
