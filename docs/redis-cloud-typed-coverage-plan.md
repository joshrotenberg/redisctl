# Redis Cloud Typed Coverage Plan

Use this as a living checklist for the PR body. Copy the list below into the PR description and check items off as you complete them.

## Phases

- [ ] Phase 1: Update tests to typed APIs
  - Replace `*_raw` calls in tests with typed handler methods (logs, SSO, billing, users, ACL).
  - Stop indexing typed results as JSON; assert via struct fields or `serde_json::to_value` when JSON shape matters.
  - Fix arg types (borrow `&Value` where needed; construct typed requests when available).

- [ ] Phase 2: Region model parity
  - Expand `models::region::RegionInfo` to include: `id`, `name`, `provider`, `available`, `zones`, `networking.{vpc, privateServiceConnect, transitGateway, supportedFeatures}`, `pricing.currency`, `pricing.dataTransfer.{inbound, outbound}`, `compliance`, `maxInstances`, `diskTypes`, `reason`.

- [ ] Phase 3: Metrics (typed)
  - Return `CloudMetrics` for subscription‑level metrics; align tests accordingly.

- [ ] Phase 4: Networking requests (typed)
  - Transit Gateway: use typed request structs for create/update where applicable.
  - Private Service Connect: add typed request structs and update handler signatures.

- [ ] Phase 5: Final sweep
  - Remove remaining JSON indexing on typed structs in tests.
  - Ensure handlers consistently return typed models or documented `Value` for delete/enable/disable messages.

## Validation

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test --workspace --all-features`

