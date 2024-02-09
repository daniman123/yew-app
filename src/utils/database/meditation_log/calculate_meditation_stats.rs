/* TODO -
    data fields:
    - `datetime`: The UNIX timestamp representing the date and time of the meditation session.
    - `duration`: The duration of the meditation session in seconds.
    - `category`: A string categorizing the type of meditation.
    - `speaker`: The name of the speaker or guide leading the meditation session.

    calculate:
    - total hours meditated.
    - average duration per meditation.
    - days meditated in row.
    - total meditation sessions.
    - favorite category.| get max total count of unique observations.
    - favourite speaker.| get max total count of unique observations.
    - average time of day meditation session start.
*/

use std::collections::HashMap;

use super::meditation_database::MeditationData;
use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct Stats {
    pub total_hours_meditated: f32,
    pub average_duration_per_meditation: f32,
    pub days_meditated_in_row: u32,
    pub total_meditation_sessions: u32,
    pub favorite_category: String,
    pub favorite_speaker: String,
}

pub struct StatsBuilder {
    meditations: Vec<MeditationData>,
    total_hours_meditated: Option<f32>,
    average_duration_per_meditation: Option<f32>,
    days_meditated_in_row: Option<u32>,
    total_meditation_sessions: Option<u32>,
    favorite_category: Option<String>,
    favorite_speaker: Option<String>,
}

impl StatsBuilder {
    // Constructor
    pub fn new(meditations: Vec<MeditationData>) -> Self {
        StatsBuilder {
            meditations,
            total_hours_meditated: None,
            average_duration_per_meditation: None,
            days_meditated_in_row: None,
            total_meditation_sessions: None,
            favorite_category: None,
            favorite_speaker: None,
        }
    }

    // Setters
    pub fn total_hours_meditated(mut self, value: f32) -> Self {
        self.total_hours_meditated = Some(value);
        self
    }

    pub fn average_duration_per_meditation(mut self, value: f32) -> Self {
        self.average_duration_per_meditation = Some(value);
        self
    }

    pub fn days_meditated_in_row(mut self, value: u32) -> Self {
        self.days_meditated_in_row = Some(value);
        self
    }

    pub fn total_meditation_sessions(mut self, value: u32) -> Self {
        self.total_meditation_sessions = Some(value);
        self
    }

    pub fn favorite_category(mut self, value: String) -> Self {
        self.favorite_category = Some(value);
        self
    }

    pub fn favorite_speaker(mut self, value: String) -> Self {
        self.favorite_speaker = Some(value);
        self
    }

    fn calculate_current_streak(&self) -> u32 {
        if self.meditations.is_empty() {
            return 0;
        }

        // Ensure meditations are sorted by datetime
        let mut sorted_meditations = self.meditations.clone();
        sorted_meditations.sort_by_key(|m| m.datetime);

        match sorted_meditations.last() {
            Some(last_date) => {
                let current_datetime = chrono::offset::Utc::now().date();

                let diff = Utc
                    .timestamp(last_date.datetime as i64, 0)
                    .date()
                    .signed_duration_since(current_datetime)
                    .num_days();
                if diff < 0 {
                    return 0;
                }
            }
            None => return 0,
        };

        let mut streak = 1;
        let mut last_date = Utc
            .timestamp(sorted_meditations[0].datetime as i64, 0)
            .date(); // Start from the first session

        for meditation in sorted_meditations.iter().skip(1) {
            // Skip the first, already accounted for
            let current_date = Utc.timestamp(meditation.datetime as i64, 0).date();
            let difference = current_date.signed_duration_since(last_date).num_days();

            if difference == 1 {
                streak += 1; // Consecutive day
            } else if difference > 1 {
                streak = 1; // Reset streak if there's a gap of more than one day
            }
            // If difference is 0, it's the same day, and the streak doesn't change

            last_date = current_date; // Update last_date to current session's date for next iteration
        }

        streak
    }

    fn find_most_occuring_category(&self) -> Option<String> {
        let mut counts = HashMap::new();
        let sorted_records = self.meditations.clone();

        // Count occurrences of each category
        for record in sorted_records {
            *counts.entry(record.category).or_insert(0) += 1;
        }

        // Find the category with the most occurrences
        counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(category, _)| category)
    }

    fn find_most_occuring_speaker(&self) -> Option<String> {
        let mut counts = HashMap::new();
        let sorted_records = self.meditations.clone();

        // Count occurrences of each category
        for record in sorted_records {
            *counts.entry(record.speaker).or_insert(0) += 1;
        }

        // Find the category with the most occurrences
        counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(category, _)| category)
    }

    // Build method to consume the builder and return the constructed Stats
    pub fn build(self) -> Stats {
        let total_hours_meditated = self
            .meditations
            .iter()
            .map(|m| m.duration as f32 / 3600.0)
            .sum();

        let average_duration_per_meditation = if self.meditations.is_empty() {
            0.0
        } else {
            self.meditations
                .iter()
                .map(|m| m.duration as f32)
                .sum::<f32>()
                / self.meditations.len() as f32
        };

        let total_meditation_sessions = self.meditations.len() as u32;
        let days_meditated_in_row = self.calculate_current_streak();

        let favorite_category = self.find_most_occuring_category().unwrap_or_default();
        let favorite_speaker = self.find_most_occuring_speaker().unwrap_or_default();

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
            datetime: 1707511333, // Skipping to Day 9
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
        assert_eq!(stats.days_meditated_in_row, 2);
        assert_eq!(stats.favorite_category, "Mindfulness");
        assert_eq!(stats.favorite_speaker, "Alice");
    }

    #[test]
    fn test_stats_builder_defaults_with_empty_meditations() {
        let meditations = Vec::new(); // No meditation data
        let stats = StatsBuilder::new(meditations).build();

        assert_eq!(stats.total_hours_meditated, 0.0);
        assert_eq!(stats.average_duration_per_meditation, 0.0);
        assert_eq!(stats.days_meditated_in_row, 0);
        assert_eq!(stats.total_meditation_sessions, 0);
        assert_eq!(stats.favorite_category, "");
        assert_eq!(stats.favorite_speaker, "");
    }
}
