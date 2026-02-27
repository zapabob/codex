# MILSPEC-Style Software Engineering Compliance Checklist

- Document ID: `MILSPEC-BUILD-CHECKLIST-2026-02-27`
- Date: `2026-02-27`
- Scope: Build request compliance review (reliability, traceability, verification)
- Owner: MILSPEC worker

## 1. Reliability Compliance

| ID | Requirement | Required Evidence | Verification Method | Status |
|---|---|---|---|---|
| REL-01 | Build inputs (toolchain, dependencies, config) are version-pinned and reproducible. | Lockfiles, tool/version manifest, build config snapshot. | Independent rebuild reproduces same artifact hash/version metadata. | [ ] |
| REL-02 | Build execution is deterministic across supported environments. | Build logs from at least two clean environments. | Compare artifact checksums and build metadata for match. | [ ] |
| REL-03 | Failure handling is defined and observable (non-zero exits, clear error logging). | Failing test/build run log with explicit exit codes. | Negative test run confirms detectable failure behavior. | [ ] |
| REL-04 | Critical reliability tests pass before release candidate designation. | Test summary (unit/integration/smoke) with pass/fail counts. | Verify required test gates are all green. | [ ] |

## 2. Traceability Compliance

| ID | Requirement | Required Evidence | Verification Method | Status |
|---|---|---|---|---|
| TRC-01 | Build request is linked to a unique request/ticket identifier. | Request ID recorded in build report and logs. | Cross-check report ID against ticket/work item. | [ ] |
| TRC-02 | Source baseline is uniquely identified (commit SHA/branch/tag). | Source revision metadata in report. | Validate SHA exists and matches build input. | [ ] |
| TRC-03 | Artifact provenance is recorded from source to output. | Artifact manifest with SHA256 + source/build references. | Trace one artifact end-to-end from manifest entries. | [ ] |
| TRC-04 | Verification results are linked to exact artifact version. | Test/report IDs mapped to artifact digest/version. | Confirm no ambiguous or missing version linkage. | [ ] |

## 3. Verification Compliance

| ID | Requirement | Required Evidence | Verification Method | Status |
|---|---|---|---|---|
| VER-01 | Verification plan defines acceptance criteria before final build sign-off. | Approved verification checklist or test plan. | Review plan completeness against REL/TRC/VER items. | [ ] |
| VER-02 | Required automated checks are executed and archived. | CI logs, local verification logs, timestamps. | Re-run selected checks to confirm repeatability. | [ ] |
| VER-03 | Manual verification steps are documented with reviewer identity/date. | Reviewer sign-off entries and execution notes. | Audit sign-off fields for completeness and accountability. | [ ] |
| VER-04 | Deviations/non-conformances are logged with disposition. | NCR/deviation log with owner and due date. | Confirm each deviation has closure or approved waiver. | [ ] |

## 4. Approval Gate

- Technical reviewer sign-off: `[ ]`
- Quality/compliance sign-off: `[ ]`
- Release authorization: `[ ]`

## 5. Notes

- This checklist is intentionally concise and MILSPEC-style (ID-based, evidence-driven, verifiable).
- Mark each status checkbox only after objective evidence is attached or referenced.
