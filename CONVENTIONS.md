# Instructions for Rust & Repository Development for Aider

As an software engineer, you are tasked with writing mission-critical Rust code and robust repository automation (Python/GitHub Actions) inside a high-performance scientific physics engine. Adhere strictly to these guidelines:

## Core Principles

1. **Safety & Idiomatic Rust:** Prioritize Rust's safety. Avoid `unsafe`. Prefer compile-time checks over runtime checks.
2. **Surgical Changes:** Only modify what is necessary. If the codebase is currently non-compiling, focus on the logical correctness of your specific task.
3. **Automation First:** For repetitive tasks (benchmarking, data migration, deployments), prefer writing a Python script or a GitHub Workflow over manual instructions.

## Workflow & Verification Loop

Before finalizing any change, you must follow this verification sequence:

### 1. Targeted Verification

- **Run `cargo check`:** Use this as your primary health check.
- **Handle Broken Code:** If `cargo check` fails due to pre-existing errors in files you **did not** touch, **DO NOT** attempt to fix them. Report the error to the user and proceed only if your changes are logically sound within their own context.
- **Linting:** Run `cargo clippy --all-targets --all-features -- -D warnings` when asked. If your new code triggers a lint, you must fix it.

### 2. Formatting & Sync

- **Consistency:** Always run `cargo fmt` after modifying Rust files to ensure the repository remains in sync with official style guidelines.
- **Non-Rust Files:** Ensure Python scripts follow PEP8 (use `black` if available) and YAML files for GitHub Actions are linted for valid syntax.

## Cross-Language Development

### Python Automation

- **Purpose:** Use Python for "glue" code, complex build scripts, or local automation.
- **Environment:** Use standard libraries where possible to minimize dependency overhead. If external packages are needed, update the relevant `requirements.txt` or `pyproject.toml`.
- **Style:** Write type-hinted Python 3.10+ code. Include docstrings for non-obvious utility functions.

### GitHub Workflows (CI/CD)

- **Security:** Use least-privilege `permissions` for GITHUB_TOKEN.
- **Efficiency:** Use caching for Rust dependencies (`actions/cache` or `swatinem/rust-cache`) to keep pipeline times low.
- **Modularity:** Prefer small, focused jobs over one massive script.

## Documentation

- **Rust:** Use `///` for public APIs. Use LaTeX only for formal mathematical theory within docs (e.g., O(n log n) complexity or relativistic formulas).
- **DevOps:** Add comments to GitHub Action `.yml` files explaining _why_ a specific trigger or environment variable is used.
