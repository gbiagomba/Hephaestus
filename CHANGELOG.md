# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.1.0] - 2025-12-11

### Added
- Cross-platform install scripts for automated installation
  - `install.sh` for Linux/Unix/macOS with support for multiple package managers
  - `install.ps1` for Windows with winget/chocolatey support
- `.version-tracking.md` for semantic versioning documentation
- Enhanced CI/CD pipeline with proper binary naming scheme
  - Binaries now named as: `hephaestus_OS-ARCH` (e.g., `hephaestus_linux-x64`, `hephaestus_windows-x64.exe`)
  - Added SHA256 checksums for all release binaries
  - Support for Linux (x64, ARM64), macOS (Intel, ARM), Windows (x64, ARM64)
- Cargo caching in CI workflow for faster builds
- Automatic release notes generation in GitHub releases

### Changed
- Updated .gitignore to exclude test files (`test_*.html`, `test_*.txt`, etc.)
- Applied Clippy lint fixes for improved code quality
- Enhanced GitHub Actions workflow with better artifact handling
- Updated README with installation instructions for all platforms
- Improved Makefile with cross-platform support
- Updated Dockerfile with latest Rust version and security improvements

### Fixed
- Code style improvements based on Clippy recommendations
- Proper binary naming in CI/CD releases

## [3.0.0] - 2025-10-01

### Added
- Complete Rust implementation replacing legacy bash scripts
- Subcommands:
  - `update`: Safe fast-forward git pull for single or multiple repos
  - `clone-links`: Clone repositories from newline-separated links file
  - `parse-html`: Extract repository links from HTML files
  - `init`: Initialize new git repository with README
  - `status`: Show git status for current repository
  - `rollback`: Confirmed destructive reset to previous ref
  - `commit-push`: Stage, commit, and push changes
- Docker support with multi-stage builds
- Comprehensive CI/CD for Linux/macOS/Windows on x64/arm64
- Unit tests for HTML link extraction
- Makefile with common development tasks
- Security improvements:
  - Fast-forward-only updates by default
  - Confirmation prompts for destructive operations
  - No global git config modifications
  - Input validation for URLs

### Changed
- Breaking: All CLI interfaces now use subcommands instead of flags
- Breaking: Different argument structure compared to legacy scripts

### Security
- No command injection vulnerabilities (proper argument arrays)
- Proper path handling (no path traversal)
- Input validation for URLs
- No privileged path writes

## [2.0.0]

### Changed
- Initial conversion from Bash scripts to Rust

## [1.x.x] - Legacy

### Features (Legacy Bash Scripts)
- Git_Mngr.sh: Update and clone multiple git repositories
- GitHTMLParser.sh: Parse git links from HTML files
- Git_Init.sh: Initialize git projects
- Git_Roleback.sh: Hard reset git repositories
- Git_Updater.sh: Automated git commit and push

### Known Issues (Legacy)
- Security vulnerabilities: Unquoted variables, command injection risks
- Portability issues: Relies on specific bash features and external commands
- No input validation
- Destructive operations without confirmation
- Hardcoded paths and branch names

---

[3.1.0]: https://github.com/yourusername/hephaestus/compare/v3.0.0...v3.1.0
[3.0.0]: https://github.com/yourusername/hephaestus/compare/v2.0.0...v3.0.0
[2.0.0]: https://github.com/yourusername/hephaestus/releases/tag/v2.0.0
