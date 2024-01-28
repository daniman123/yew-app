use super::meditation_database::MeditationDataBuilder;
use crate::utils::database::database_manager::DatabaseManager;

/// Logs meditation data into the database.
///
/// This function takes in meditation data parameters, constructs a MeditationData object using
/// the MeditationDataBuilder, and then attempts to write this data to the database using the
/// DatabaseManager. If the construction of the MeditationData object fails, it does nothing.
///
/// # Arguments
/// * `datetime` - An i32 representing the date and time of the meditation session.
/// * `duration` - An i32 representing the duration of the meditation session in minutes.
/// * `category` - A String specifying the category of the meditation session (e.g., "Mindfulness").
/// * `speaker` - A String specifying the name of the speaker or guide of the meditation session.
///

pub fn log_meditation_data(datetime: i32, duration: i32, category: String, speaker: String) {
    // Construct a new MeditationData object using the builder pattern.
    let meditation_data_builder = MeditationDataBuilder::new()
        .datetime(datetime)
        .duration(duration)
        .category(category)
        .speaker(speaker)
        .build();

    // Attempt to write the meditation data to the database.
    match meditation_data_builder {
        Ok(meditation_data) => DatabaseManager::write_data(meditation_data, ""), // On success, write the data.
        Err(_) => (), // On failure, do nothing.
    }
}
