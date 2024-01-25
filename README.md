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

local_storage_state.rs:
/// LocalStorageState represents the state and behavior associated with local storage operations.