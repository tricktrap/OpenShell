# Agent Instructions

See [CONTRIBUTING.md](CONTRIBUTING.md) for instructions on how to perform common operations (building, testing, linting, running components).

## Sandbox Infra Changes

- If you change sandbox infrastructure, ensure `mise run sandbox` succeeds.

## Commits

- Always use [Conventional Commits](https://www.conventionalcommits.org/) format for commit messages
- Format: `<type>(<scope>): <description>` (scope is optional)
- Common types: `feat`, `fix`, `docs`, `chore`, `refactor`, `test`, `ci`, `perf`
- Never mention Claude or any AI agent in commits (no author attribution, no Co-Authored-By, no references in commit messages)

## Python

- Always use `uv` for Python commands (e.g., `uv pip install`, `uv run`, `uv venv`)

## Docker

- Always prefer `mise` commands over direct docker builds (e.g., `mise run docker:build` instead of `docker build`)
