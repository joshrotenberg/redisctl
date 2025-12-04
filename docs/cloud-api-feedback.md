# Redis Cloud API Feedback

Feedback from building redisctl - a type-safe Rust CLI for Redis Cloud API.

## Executive Summary

The Redis Cloud API is generally well-designed and functional. However, several inconsistencies create friction for SDK/CLI developers and increase maintenance burden. The issues below are ordered by impact.

---

## 1. Inconsistent Response Wrappers

**Impact: High** - Requires custom unwrapping logic per endpoint

Different endpoints wrap their responses differently:

| Endpoint | Response Format |
|----------|----------------|
| `/subscriptions` | `{ "subscriptions": [...] }` |
| `/tasks` | `{ "task": {...} }` (single) |
| `/users` | `{ "users": [...] }` |
| `/acl/redisRules` | Direct array `[...]` |
| `/crdb` | `{ "crdb": {...} }` |
| `/sso/saml/metadata` | `{ "metadata": {...} }` |

**Commits**: d8ace5c, 3af10c5

**Recommendation**: Standardize on one pattern. Suggest: always wrap with resource name for lists, no wrapper for single resource GETs.

---

## 2. Database ID Format (subscription_id:database_id)

**Impact: High** - Confusing UX, error-prone

Cloud uses composite IDs like `123456:789` while Enterprise uses simple numeric IDs.

- Users must know which subscription a database belongs to
- Can't query a database by just its ID
- Different from every other Redis product

**Recommendation**: Allow querying databases by database_id alone (at least for account-scoped operations), or provide a lookup endpoint.

---

## 3. Separate Active-Active (CRDB) Endpoints

**Impact: Medium** - Code duplication, maintenance burden

Connectivity features (VPC Peering, PSC, Transit Gateway, PrivateLink) have completely separate endpoints for standard vs Active-Active subscriptions:

```
# Standard
GET /subscriptions/{id}/peerings

# Active-Active  
GET /subscriptions/{id}/peerings/active-active
```

This requires:
- Duplicate handler methods (20+ in our codebase)
- Users must know subscription type before calling
- Duplicate documentation

**Files affected**: connectivity/psc.rs, connectivity/private_link.rs, connectivity/transit_gateway.rs, connectivity/peering.rs

**Recommendation**: Single endpoint that works for both, or add subscription type to response so clients can auto-detect.

---

## 4. Field Naming Inconsistencies

**Impact: Medium** - Deserialization failures, confusion

### camelCase vs snake_case
The API uses camelCase (good), but inconsistently:
- `memoryLimitInGb` vs `memory_limit_in_gb` in different contexts
- `crdbId` vs `taskId` (consistent) but `database_id` in some places

### Field name variations
Same concept, different names across endpoints:
- `createdAt` vs `created_time` vs `creation_time`
- `lastLogin` vs `last_login_time`
- `autoProvisioning` vs `auto_provision`
- `ssoUrl` vs `sso_url` vs `saml_url`

**Commit**: d8ace5c (massive alignment fix)

**Recommendation**: Audit all endpoints for naming consistency. Document the naming convention.

---

## 5. Async Operation Handling

**Impact: Medium** - Polling complexity

Create/Update/Delete operations return task IDs, but:

- No webhook/callback option - must poll
- Task status endpoint sometimes nested differently
- `taskId` vs `task_id` inconsistency
- No estimated completion time

Current flow requires:
```rust
let task_id = create_resource().await?;
loop {
    let status = get_task(task_id).await?;
    if status.is_complete() { break; }
    sleep(5_seconds).await;
}
```

**Recommendation**: 
- Add webhook callback option
- Standardize task response format
- Include estimated completion time

---

## 6. Pagination Inconsistencies

**Impact: Medium** - Different handling per endpoint

Some endpoints use:
- `offset` + `limit`
- `page` + `pageSize`  
- No pagination at all (returns everything)

Logs endpoint has nested pagination (sessionLogs within paginated response).

**Commit**: d8ace5c (logs pagination fix)

**Recommendation**: Standardize on one pagination approach across all list endpoints.

---

## 7. Type Mismatches in Documentation vs Reality

**Impact: Medium** - Runtime failures

Fields documented as one type but returned as another:

- `master_persistence`: documented as string, returned as boolean
- Various numeric fields returned as strings
- Optional fields sometimes null, sometimes missing entirely

**Commit**: 66aac2c (added serde_path_to_error to debug these)

**Recommendation**: Validate API responses against OpenAPI spec in CI.

---

## 8. Outdated Terminology

**Impact: Low** - Confusion, maintenance

API still uses deprecated terms:
- `whitelist` instead of `allowlist`
- `master` in some contexts

**Commit**: 6f0d45e

**Recommendation**: Migrate to modern terminology with deprecation period.

---

## 9. Missing Endpoints

**Impact: Low-Medium** - Feature gaps

Functionality available in UI but not API:
- Some billing details
- Certain subscription modification options
- Advanced monitoring/alerting configuration

**Recommendation**: Audit UI vs API feature parity.

---

## 10. DELETE with Body

**Impact: Low** - Requires workaround

One endpoint requires DELETE with request body, which many HTTP clients don't support well:

```rust
// crates/redis-cloud/src/flexible/subscriptions.rs:1037
// TODO: DELETE with body not yet supported by client
```

**Recommendation**: Accept parameters as query string for DELETE operations.

---

## Summary of Top Recommendations

1. **Standardize response wrappers** - biggest time sink
2. **Simplify database ID handling** - biggest UX issue
3. **Unify Active-Active endpoints** - reduces duplication by 50%
4. **Audit field naming consistency** - prevents deserialization errors
5. **Add webhook callbacks for async ops** - modern pattern

---

## Positive Notes

Things the API does well:
- Clear REST conventions (verbs, resource paths)
- Good error messages with actionable details
- Comprehensive endpoint coverage
- Consistent authentication headers
- JSON throughout (no XML)

---

## Appendix: Key Commits

| Commit | Description |
|--------|-------------|
| d8ace5c | Massive alignment fix for wrappers and field names |
| 66aac2c | Added serde_path_to_error for type mismatch debugging |
| 6f0d45e | Fixed naming inconsistencies (whitelist, _1 suffixes) |
| 3065b08 | Fixed peering struct mismatch |
| 3af10c5 | Began wrapper response alignment |

