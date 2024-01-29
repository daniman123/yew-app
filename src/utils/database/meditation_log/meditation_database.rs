use serde::{Deserialize, Serialize};

/// Represents data for a meditation session.
///
/// # Fields
/// - `datetime`: The UNIX timestamp representing the date and time of the meditation session.
/// - `duration`: The duration of the meditation session in minutes.
/// - `category`: A string categorizing the type of meditation.
/// - `speaker`: The name of the speaker or guide leading the meditation session.
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct MeditationData {
    pub datetime: i32,
    pub duration: i32,
    pub category: String,
    pub speaker: String,
}

/// A builder for creating instances of `MeditationData`.
///
/// This builder uses the builder pattern to create `MeditationData` instances, allowing
/// for optional setting of each field. If a field is not set, it defaults to `None`.
///
/// # Fields
/// - `datetime`: Optional. The UNIX timestamp for the session's date and time.
/// - `duration`: Optional. The session duration in minutes.
/// - `category`: Optional. A string categorizing the meditation type.
/// - `speaker`: Optional. The name of the meditation session's speaker or guide.
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct MeditationDataBuilder {
    datetime: Option<i32>,
    duration: Option<i32>,
    category: Option<String>,
    speaker: Option<String>,
}

impl MeditationDataBuilder {
    /// Constructs a new `MeditationDataBuilder` with default values (all fields set to `None`).
    ///
    /// # Returns
    /// A new instance of `MeditationDataBuilder`.
    pub fn new() -> Self {
        MeditationDataBuilder::default()
    }

    /// Sets the `datetime` field for the meditation session.
    ///
    /// # Arguments
    /// - `datetime`: A value that can be converted into an `i32`, representing the UNIX timestamp.
    ///
    /// # Returns
    /// A mutable reference to the builder itself (for chaining methods).
    pub fn datetime(&mut self, datetime: impl Into<i32>) -> &mut Self {
        self.datetime = Some(datetime.into());
        self
    }

    /// Sets the `duration` of the meditation session.
    ///
    /// # Arguments
    /// - `duration`: A value that can be converted into an `i32`, representing the duration in minutes.
    ///
    /// # Returns
    /// A mutable reference to the builder itself (for chaining methods).
    pub fn duration(&mut self, duration: impl Into<i32>) -> &mut Self {
        self.duration = Some(duration.into());
        self
    }

    /// Sets the `category` of the meditation session.
    ///
    /// # Arguments
    /// - `category`: A value that can be converted into a `String`, representing the session's category.
    ///
    /// # Returns
    /// A mutable reference to the builder itself (for chaining methods).
    pub fn category(&mut self, category: impl Into<String>) -> &mut Self {
        self.category = Some(category.into());
        self
    }

    /// Sets the `speaker` or guide of the meditation session.
    ///
    /// # Arguments
    /// - `speaker`: A value that can be converted into a `String`, representing the session's speaker or guide.
    ///
    /// # Returns
    /// A mutable reference to the builder itself (for chaining methods).
    pub fn speaker(&mut self, speaker: impl Into<String>) -> &mut Self {
        self.speaker = Some(speaker.into());
        self
    }

    /// Finalizes the builder and returns a `MeditationData` instance.
    ///
    /// This method checks for the presence and validity of required fields. If any required field is missing or invalid,
    /// it returns an `Err` with an appropriate error message.
    ///
    /// # Returns
    /// `Result<MeditationData, String>` - `Ok(MeditationData)` if all fields are valid, or `Err(String)` with an error message.
    pub fn build(&self) -> Result<MeditationData, String> {
        let datetime = match self.datetime {
            Some(dt) if dt > 0 => dt,
            _ => return Err("datetime must be non-zero".into()),
        };

        let duration = match self.duration {
            Some(dur) if dur > 0 => dur,
            _ => return Err("duration must be non-zero".into()),
        };

        let category = self.category.clone().ok_or("category is required")?;
        if category.trim().is_empty() {
            return Err("category cannot be empty or whitespace".into());
        }

        let speaker = self.speaker.clone().ok_or("speaker is required")?;
        if speaker.trim().is_empty() {
            return Err("speaker cannot be empty or whitespace".into());
        }

        Ok(MeditationData {
            datetime,
            duration,
            category,
            speaker,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meditation_data_builder() {
        let meditation_data_builder = MeditationDataBuilder::new()
            .datetime(10)
            .duration(123)
            .category("category")
            .speaker("speaker")
            .build();

        // println!("{:?}", meditation_data_builder.unwrap());
        assert!(meditation_data_builder.is_ok())
    }

    #[test]
    fn test_meditation_data_builder_non_zero() {
        let meditation_data_builder = MeditationDataBuilder::new()
            .datetime(10)
            .duration(1)
            .category(" ")
            .speaker("speaker")
            .build();

        // println!("{:?}", meditation_data_builder.unwrap());
        assert!(meditation_data_builder.is_err())
    }
}
