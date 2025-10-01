# v3.0.0 - 2025-10-01
- Replace legacy bash scripts with secure Rust subcommands
- Add `update`, `clone-links`, `parse-html`, `init`, `status`, `rollback`, `commit-push`
- Safer defaults: ff-only updates, confirmation for destructive ops, no global git config writes
- Docker: multi-stage, non-root runtime image
- Makefile: fmt, lint, install, docker helpers
- CI: cross-OS/arch build matrix (Linux/macOS/Windows; x64 and arm64 best effort)
- Tests: add unit test for HTML link extraction

# v2.0.0
- Converted to Rust
