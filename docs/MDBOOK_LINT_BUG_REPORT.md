# mdbook-lint Bug Report

Found multiple bugs in mdbook-lint v0.11.2 during redisctl documentation linting.

## Bug 1: Configuration Rules Not Applied

**Issue**: Rules defined in `.mdbook-lint.toml` configuration file are not being applied.

**Expected**: Rules configured in the config file should override defaults.

**Actual**: Default rule settings are used regardless of configuration.

**Reproduction**:
1. Create `.mdbook-lint.toml` with:
   ```toml
   [rules.MD013]
   line_length = 120
   ```
2. Run `mdbook-lint lint file.md` 
3. Observe that MD013 still enforces 80 character limit instead of 120

**Evidence**: Lines over 80 chars trigger MD013 warnings despite config setting line_length = 120.

## Bug 2: --fix Option Not Working

**Issue**: The `--fix` option doesn't actually fix any issues, even for rules that should be auto-fixable.

**Expected**: `mdbook-lint lint --fix file.md` should automatically fix issues like MD022, MD032, MD031, MD047.

**Actual**: Command shows the same warnings but doesn't modify the file.

**Reproduction**:
1. Create test file with MD022 violations (missing blank lines after headings)
2. Run `mdbook-lint lint --fix test.md`
3. File remains unchanged despite warnings being shown

**Test case**:
```markdown
# Heading
Text directly after heading without blank line
```

Should be auto-fixed to:
```markdown
# Heading

Text directly after heading without blank line
```

But file remains unmodified.

## Environment
- mdbook-lint version: 0.11.2
- OS: macOS (Darwin 24.6.0)
- Configuration file present: `.mdbook-lint.toml`

## Workaround
Currently fixing issues manually by editing markdown files directly.

## Impact
These bugs prevent automated fixing and proper configuration of linting rules, requiring manual intervention for all markdown formatting issues.