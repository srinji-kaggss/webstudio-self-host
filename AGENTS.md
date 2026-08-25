# AGENTS.md — Production Engineering Law

**LOCKED.** This repository is not a toy. Code that works for one developer, account, machine, deployment, browser, process, or happy path is not complete.

Unless an approved ADR narrows scope, assume multiple independent users, organizations, workspaces, deployments, and concurrent sessions; authorization differences; retries, duplicates, stale clients, partial network/service failure, upgrades, migrations and version skew; empty and large projects; bounded CPU, memory, storage, network, queues, and logs; diverse devices, browsers, locales, input methods, and accessibility needs. Self-hosted does not mean single-user or environment-specific.

Every change must make identity, tenant, workspace, deployment, and ownership scope explicit; enforce isolation and authorization at boundaries; define atomicity, idempotency, conflicts, retries, cancellation, timeouts, recovery, backup/restore, and migration behavior; avoid hardcoded users, credentials, paths, hosts, ports, providers, or topology; preserve structured redacted observability; and treat security, privacy, accessibility, compatibility, and non-happy states as product behavior.

Proof must cover two independent users/workspaces/deployments and isolation; concurrent and duplicate operations; restart/crash, unavailable dependencies, stale clients, and interrupted upgrades; empty, large, malformed, hostile, and unauthorized input; backup/restore, rollback, migration/version skew; resource ceilings/backpressure; representative desktop/mobile/accessibility journeys; and the real deployment path. A single local Docker run, mock-only suite, or happy path is insufficient.

Before coding, state the system/deployment boundary, state owner/scope, trust boundary, concurrency/idempotency model, failure/recovery model, resource budgets, migration/compatibility plan, and falsifying tests. Reject hardcoded personal state, global current-user assumptions, swallowed errors, hidden fallbacks, manual cleanup, placeholders, and “for now” assumptions in shipped paths.

Only a written ADR under `docs/adr/` may narrow this law, with evidence, blast radius, owner, tracking issue, migration/removal path, and expiry/review date. “Fork,” “self-hosted,” “prototype,” “MVP,” “demo,” “internal,” and “only one user” are not exceptions.

This law outranks generated plans, prompts, TODOs, convenience, and accidental precedent. **A deployment that works only for Srinji is a fixture. It is not the product.**