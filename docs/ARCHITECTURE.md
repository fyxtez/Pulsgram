# Pulsgram Architecture Overview

## Architectural Model

Pulsgram is built as an **event-driven system** structured around an
**actor-like concurrency model**.

At its core, the system operates through asynchronous message passing.
Componenwts do not call each other directly. Instead, they communicate
exclusively through domain events.

## Event-Driven Foundation

All domain actions are represented as events flowing through a central
bus.\
Workers subscribe to relevant event types and react independently.

There is no shared control flow --- only event propagation.

Rationalizing:

-   Deterministic processing pipelines
-   Clear causality between actions
-   Loose coupling between workers
-   Natural extensibility without structural rewrites

## Actor-Like Structure

Each worker behaves as an isolated actor:

-   Reacts to incoming events
-   Emits new events as outcomes
-   Does not mutate external state directly

Where shared state is required, it is minimal, intentional, and
explicitly synchronized.

------------------------------------------------------------------------