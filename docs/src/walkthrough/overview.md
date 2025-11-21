# Overview & Concepts

## What is redisctl?

redisctl is the first command-line tool for Redis Cloud and Redis Enterprise management.

## The Problem

Before redisctl, operators had to:
- Use web UIs for everything
- Write fragile bash scripts with curl and polling loops
- Manually track async operations

## The Three-Tier Model

redisctl provides three levels of interaction:

### 1. API Layer
Direct REST access for scripting and automation. Think of it as a smart curl replacement with authentication handling.

### 2. Human Commands
Type-safe, ergonomic commands for day-to-day operations. Covers databases, subscriptions, users, and more.

### 3. Workflows
Multi-step operations that would normally require multiple commands, polling, and error handling.

## Getting Help

```bash
# General help
redisctl --help

# Command-specific help
redisctl cloud database --help
redisctl enterprise cluster --help
```
