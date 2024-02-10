# https://www.conventionalcommits.org/en/v1.0.0/

# REVERT LAST COMMIT - KEEP STAGED CHANGES:  git reset --soft HEAD~1

# // build: Changes that affect the build system or external dependencies (example scopes: gulp, broccoli, npm)
# // ci: Changes to our CI configuration files and scripts (example scopes: Travis, Circle, BrowserStack, SauceLabs)
# // docs: Documentation only changes
# // feat: A new feature
# // fix: A bug fix
# // perf: A code change that improves performance
# // refactor: A code change that neither fixes a bug nor adds a feature
# // style: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
# // test: Adding missing tests or correcting existing tests


local_storage.rs:
A service for interacting with the browser's local storage.
The `LocalStorageService` provides methods to create, read, update, and delete key-value pairs in the browser's local storage.

database_manager.rs:
`DatabaseManager` is a struct representing the data related to meditation.
This struct can be serialized and deserialized for storage purposes
Manages database operations for `MeditationData`.

meditation_data_builder.rs:
`MeditationData` holds information about a meditation session.
It includes the date and time of the session, its duration, the category of meditation,
and the name of the speaker or guide.
`MeditationDataBuilder` is a builder for `MeditationData`.
It provides a way to construct a `MeditationData` instance with optional fields.

log_meditation.rs:
`log_meditation_data` logs meditation data into the database.
This function takes in meditation data parameters, constructs a MeditationData object using
the MeditationDataBuilder, and then attempts to write this data to the database using the
DatabaseManager. If the construction of the MeditationData object fails, it does nothing.

read_write_meditation_data.rs:
The function `read_meditation_data` reads meditation data from a database.
Returns - A vector of `MeditationData` objects is being returned.
The function `log_meditation_data` logs meditation data into the database.
This function takes in meditation data parameters, constructs a MeditationData object using
the MeditationDataBuilder, and then attempts to write this data to the database using the
DatabaseManager. If the construction of the MeditationData object fails, it does nothing.

calculate_meditation_stats.rs:
`StatsBuilder` is a builder for compiling meditation statistics from a set of `MeditationData`.