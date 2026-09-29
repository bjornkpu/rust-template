# Invariants

Rules that must always hold. Each has an id, one sentence, and the test that pins it. Never
break one. An invariant without a test is a bug: write the test or delete the invariant.
Ids are never reused.

| Id | Invariant | Pinned by |
| --- | --- | --- |
| INV-1 | Logs never go to stdout; stdout carries only command output. | `tests/cli.rs::stdout_has_no_logs` |
