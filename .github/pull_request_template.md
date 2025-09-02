## Summary

- What does this PR do? Why is it needed?

## Scope

- Affected areas (crates/paths):
- API surface changes (typed models, handlers, versions):
- Docs/tests updates:

## Checklist

- [ ] Draft PR opened early; incremental commits pushed
- [ ] Scope and plan documented in this description
- [ ] Tests added/updated; both success and error cases covered
- [ ] Docs updated (lib/module overviews, examples)
- [ ] cargo fmt, clippy (-D warnings), tests pass locally
- [ ] CI status is green
- [ ] Ready for review (remove Draft)
- [ ] Squash and merge when approved

## Links

- Related issues (Fixes #...):
- Follow‑ups (if any):

## Plan & Progress (optional)

Use this section to track multi‑step work. Check items off as they complete.

- [ ] Phase 1: Update tests to typed APIs (logs, sso, billing, users, acl)
- [ ] Phase 2: Region model parity (id/name/available/zones/networking/pricing/compliance/diskTypes)
- [ ] Phase 3: Metrics subscription returns typed `CloudMetrics`
- [ ] Phase 4: Transit Gateway + Private Service Connect typed request structs and handlers
- [ ] Phase 5: Sweep tests for JSON indexing on typed structs; standardize assertions
