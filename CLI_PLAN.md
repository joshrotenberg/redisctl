  CLI Architecture (3-Layer Design)

  Layer 1: Raw API Tool (redisctl api)

  - Direct REST endpoint access: redisctl api cloud GET /subscriptions
  - JSON in/out, minimal processing
  - Perfect curl replacement for scripts/automation
  - Handles auth, but otherwise just passes through

  Layer 2: Human-Friendly Interface (redisctl cloud/enterprise)

  - Current commands: redisctl cloud database list
  - Nice table output by default, JSON optional (-o json)
  - Type-safe operations using our handlers
  - User-friendly error messages

  Layer 3: Workflow Orchestration (redisctl workflow)

  - Multi-step operations: redisctl workflow enterprise-cluster-init
  - redisctl workflow create-database-with-backup-schedule
  - Combines multiple API calls into cohesive operations
  - Progress indicators, rollback on failure

  Implementation Plan

  I suggest we start completely fresh and build incrementally with separate PRs:

  1. PR 1: Core Foundation
    - Configuration system (files, env vars, CLI args)
    - Connection management and profiles
    - Basic CLI structure and shared utilities
    - Error handling patterns
  2. PR 2: Raw API Layer
    - redisctl api command implementation
    - JSON passthrough functionality
    - Authentication integration
  3. PR 3: Human-Friendly Layer
    - redisctl cloud/enterprise commands
    - Table formatting, human-readable output
    - Integration with both libraries
  4. PR 4: Workflow Layer
    - Multi-step operation framework
    - Initial workflow implementations
    - Progress tracking

  Should we start by moving the current CLI code out of the way and beginning with PR 1 (Core Foundation)? This gives us a
  clean slate while preserving the existing implementation for reference.

  What workflows do you think would be most valuable for the initial implementation?
