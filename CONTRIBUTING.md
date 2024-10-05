# Contribution Guide for Whisper

## Introduction
Whisper is an all-in-one CLI toolbox designed to automate repetitive tasks in mobile app development. Contributions are essential to its growth, and we welcome the community's involvement in expanding its capabilities. This guide outlines how to contribute code, report issues, and suggest improvements to Whisper.

## General Guidelines
1. **Be kind, respectful, and honest** when communicating with the community. Constructive feedback and positive engagement are essential.
2. **Have fun!** Contributing to Whisper should be an enjoyable experience, so let your creativity flow.

## Dependencies
Whisper relies on the following core dependencies:
- `serde`
- `toml`
- `clap`
- `ratatui`
- `reqwest`
- `thiserror`
- `color-eyre`

Before adding any new dependencies, please discuss them in an open issue to gather community feedback. Whisper is focused on performance and simplicity, so any additional dependency should align with these goals.

## Project Structure
Whisper is designed with a clear separation between logic and user interface:
- **whisper_core**: Contains all the pure logic, core algorithms, and data processing.
- **whisper_ui**: Houses all `clap` and `ratatui` code to provide a clean, intuitive interface for users.

This structure allows for easier testing and modification of core functionality without impacting the UI.

## Testing
Whisper appreciates unit testing and follows a **Test-Driven Development (TDD)** approach. While not mandatory, contributions with tests are highly encouraged. Unit tests help ensure the stability and reliability of the codebase, and we value this added effort.

## Error Handling
Whisper uses custom exceptions to improve error handling. These exceptions are located in:
`src/whisper_core/exceptions/whisper_error.rs`

When contributing new features or modifying existing ones, please ensure that you utilize these custom exceptions where appropriate to maintain consistent error handling throughout the tool.

## Code Review
All contributions, whether new features, bug fixes, or updates to existing code, require a **code review**. This ensures that the quality, security, and performance of Whisper are maintained. Please be responsive to reviewer feedback and ready to make any necessary revisions.

## Code Style
Whisper follows **Rust's official code style guide**. There is no specific Whisper code style guide; just stick to Rust conventions, and you'll be fine. Use tools like `rustfmt` to ensure consistency across the codebase.

## Architecture Improvements
We welcome suggestions for improving the architecture of Whisper. If you have an idea for improving code organization, performance, or scalability, feel free to open an issue or discussion. We are always eager to hear new ideas and incorporate them if they benefit the project.

## Asynchronous Execution
At the moment, asynchronous execution is not required since Whisper does not have any features that rely on non-blocking operations. CLI tools like Whisper typically run once and finish, so there is no need for sequential execution.

If you think asynchronous execution is necessary for a specific feature, please raise the issue for discussion before implementing it.

## Submitting New Features
When submitting new features, improvements, or refactors, please follow the format below for feature issues:

**Example Feature Issue:**

**Description:**
Implement the TestFairy build upload feature, allowing Whisper users to deliver iOS IPAs and Android APKs to TestFairy using the `whisper testfairy upload` command. This requires users to specify their TestFairy API token in the `whisper.toml` configuration file. The command validates the file input, ensuring that only IPA and APK files are accepted. The path to the file is specified as the argument for the `whisper testfairy upload` command.

**Changes Required:**
- Add `whisper testfairy upload` command.
- Validate file input to accept only IPA and APK files.
- Read TestFairy API token from `whisper.toml` configuration file.

**Scope of Changes:**
- New `whisper testfairy upload` command.
- Updated file input validation.
- Integration with TestFairy API.

**Acceptance Criteria:**
- Users can upload IPA and APK files to TestFairy using the `whisper testfairy upload` command.
- File input validation prevents upload of non-IPA and non-APK files.
- TestFairy API token is successfully read from the `whisper.toml` configuration file.

**Test Plans:**
- Unit tests for the `whisper testfairy upload` command.
- Functional tests to verify file input validation.
- Integration tests to verify TestFairy API integration.

## Submitting a Pull Request (PR)
When submitting a PR, follow this format:

**Example PR Description:**

**Description:**
This PR implements a `-c` flag in the `WhisperCommandApp`, allowing users to load custom Whisper configuration files. This enables users to load different `whisper.toml` files from the start.

**Ticket:**
This PR closes eruehl/whisper_cli#3

**Changes I've Made:**
- Modified `main.rs` to load the configuration path from the current directory if the `-c` flag is not specified, or from the custom directory if the `-c` flag is used.

**Acceptance Criteria:**
- The `-c` flag should allow users to load custom Whisper configuration files.
- Whisper should load the configuration from the correct directory based on the presence or absence of the `-c` flag.

**Test Plans:**
- Run `cargo test` to verify the functionality of the `-c` flag.
- Manually test the `-c` flag by loading different Whisper configuration files.

## Reporting Issues
If you encounter a bug or want to suggest an improvement, please follow this format:

- **Issue Description (user story)**
- **Screenshots** (if applicable)
- **Steps to Replicate**
- **Logs, Files, or Attachments** (that can help replay the issue)

This detailed information helps us reproduce and resolve issues quickly.

---

Thank you for contributing to Whisper! We look forward to your ideas, features, and improvements to make this tool even better for mobile app developers.
