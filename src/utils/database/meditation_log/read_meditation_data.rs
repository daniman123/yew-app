use super::meditation_database::MeditationData;
use crate::utils::database::database_manager::DatabaseManager;

// TODO - store key in constant file.
const MEDITATION_LOG_KEY: &str = "meditationLog";

pub fn read_meditation_data() -> Vec<MeditationData> {
    DatabaseManager::read_data(MEDITATION_LOG_KEY)
}
