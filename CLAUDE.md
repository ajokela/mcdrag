# Claude Development Instructions

## Git Commit Guidelines

When creating git commits for this project, follow these guidelines:

### Commit Message Format
- Write clear, descriptive commit messages that explain what was changed and why
- Use the imperative mood in the subject line (e.g., "Add feature" not "Added feature")
- Keep the subject line under 72 characters
- Separate subject from body with a blank line when a detailed explanation is needed
- Include bullet points in the body for multiple changes

### Important Rules
- **DO NOT** acknowledge or mention that Claude was used in any commit messages
- **DO NOT** include any AI-related attributions or credits in commits
- Write commits as if they were authored directly by the developer

### Example Commit Message
```
Add BASIC interpreter page using wwwbasic

- Interactive BASIC interpreter running in the browser
- Loads and executes the original MCDRAG.txt BASIC program
- Retro styling mimicking classic BASIC terminals
- Controls for RUN, STOP, CLEAR, and LIST commands
- Link to switch between BASIC and Rust versions
```

## Project-Specific Information

### Testing Commands
Before committing code changes, run:
- `cargo build` - Build the Rust project
- `cargo test` - Run tests (if available)
- `./build_wasm.sh` - Build the WASM module for web version

### License Information
- Original MCDRAG BASIC code: Public Domain
- Rust implementation: BSD 3-Clause License

### Repository
- GitHub: https://github.com/ajokela/mcdrag