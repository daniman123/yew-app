use super::meditation_database::MeditationData;
use chrono::prelude::*;
use std::collections::HashMap;

/// Represents aggregated statistics from a collection of meditation sessions.
#[derive(Debug, Clone)]
pub struct Stats {
    pub total_hours_meditated: f32,
    pub average_duration_per_meditation: f32,
    pub days_meditated_in_row: u32,
    pub total_meditation_sessions: u32,
    pub favorite_category: String,
    pub favorite_speaker: String,
}

/// A builder for compiling meditation statistics from a set of `MeditationData`.
pub struct StatsBuilder {
    meditations: Vec<MeditationData>,
}

impl StatsBuilder {
    /// Constructs a new `StatsBuilder` with a given list of meditation sessions.
    ///
    /// # Arguments
    ///
    /// * `meditations` - A vector of `MeditationData` representing individual meditation sessions.
    pub fn new(meditations: Vec<MeditationData>) -> Self {
        StatsBuilder { meditations }
    }

    /// Calculates the longest consecutive streak of daily meditation sessions.
    ///
    /// # Returns
    ///
    /// The number of days in the longest consecutive daily meditation streak.
    fn calculate_current_streak(&self) -> u32 {
        let now = Utc::now().date_naive();
        self.meditations
            .iter()
            .rev() // Start from the most recent meditation session.
            .filter_map(|m| NaiveDateTime::from_timestamp_opt(m.datetime, 0).map(|dt| dt.date()))
            .take_while(|&date| date == now || (now - date).num_days() == 1)
            .count() as u32
    }

    /// Finds the most frequently occurring item in a given list.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the items in the list, must implement `std::hash::Hash`, `Eq`, and `Clone`.
    ///
    /// # Arguments
    ///
    /// * `items` - A vector of items for which to find the most frequent occurrence.
    ///
    /// # Returns
    ///
    /// An option containing the most frequent item, or `None` if the list is empty.
    fn find_most_frequent<T: std::hash::Hash + Eq + Clone>(items: Vec<T>) -> Option<T> {
        let mut counts = HashMap::new();
        for item in items.iter() {
            *counts.entry(item.clone()).or_insert(0) += 1;
        }

        counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(item, _)| item)
    }

    /// Builds the `Stats` object from the provided meditation sessions.
    ///
    /// # Returns
    ///
    /// A `Stats` object containing aggregated statistics from the meditation sessions.
    pub fn build(self) -> Stats {
        let total_hours_meditated = self
            .meditations
            .iter()
            .map(|m| m.duration as f32 / 3600.0)
            .sum();

        let average_duration_per_meditation = if self.meditations.is_empty() {
            0.0 // Avoid division by zero if there are no meditations.
        } else {
            let sum: f32 = self.meditations.iter().map(|m| m.duration as f32).sum();
            sum / self.meditations.len() as f32
        };

        let total_meditation_sessions = self.meditations.len() as u32;
        let days_meditated_in_row = self.calculate_current_streak();
        let favorite_category = Self::find_most_frequent(
            self.meditations
                .iter()
                .map(|m| m.category.clone())
                .collect(),
        )
        .unwrap_or_default();
        let favorite_speaker =
            Self::find_most_frequent(self.meditations.iter().map(|m| m.speaker.clone()).collect())
                .unwrap_or_default();

        Stats {
            total_hours_meditated,
            average_duration_per_meditation,
            total_meditation_sessions,
            days_meditated_in_row,
            favorite_category,
            favorite_speaker,
        }
    }
}

pub fn create_mock_meditations() -> Vec<MeditationData> {
    vec![
        MeditationData {
            datetime: 1617638400, // Example: Day 1
            duration: 1800,       // 30 minutes
            category: "Mindfulness".to_string(),
            speaker: "Alice".to_string(),
        },
        MeditationData {
            datetime: 1617724800, // Example: Day 2
            duration: 3600,       // 60 minutes
            category: "Relaxation".to_string(),
            speaker: "Bob".to_string(),
        },
        MeditationData {
            datetime: 1617811200, // Example: Day 3
            duration: 900,        // 15 minutes
            category: "Mindfulness".to_string(),
            speaker: "Alice".to_string(),
        },
        MeditationData {
            datetime: 1617897600, // Consecutive Day 4
            duration: 1200,       // 20 minutes
            category: "Relaxation".to_string(),
            speaker: "Charlie".to_string(),
        },
        MeditationData {
            datetime: 1707421416, // Skipping Day 5, then Day 6
            duration: 3000,       // 50 minutes
            category: "Healing".to_string(),
            speaker: "Alice".to_string(),
        },
        // Simulate a break in streak, then another session
        MeditationData {
            datetime: 1707575913, // Should be todays date
            duration: 2100,       // 35 minutes
            category: "Mindfulness".to_string(),
            speaker: "Bob".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_builder_with_meditations() {
        let meditations = create_mock_meditations();
        let stats = StatsBuilder::new(meditations).build();

        // println!("{:?}", stats);

        assert_eq!(stats.total_hours_meditated, 3.4999998);
        assert_eq!(stats.average_duration_per_meditation, 2100.0);
        assert_eq!(stats.total_meditation_sessions, 6);
        assert_eq!(stats.days_meditated_in_row, 1);
        assert_eq!(stats.favorite_category, "Mindfulness");
        assert_eq!(stats.favorite_speaker, "Alice");
    }

    #[test]
    fn test_stats_builder_defaults_with_empty_meditations() {
        let meditations = Vec::new(); // No meditation data
        let stats = StatsBuilder::new(meditations).build();

        // println!("stats: {:?}", stats);

        assert_eq!(stats.total_hours_meditated, 0.0);
        assert_eq!(stats.average_duration_per_meditation, 0.0);
        assert_eq!(stats.days_meditated_in_row, 0);
        assert_eq!(stats.total_meditation_sessions, 0);
        assert_eq!(stats.favorite_category, "");
        assert_eq!(stats.favorite_speaker, "");
    }
}
