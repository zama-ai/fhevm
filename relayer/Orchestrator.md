# Orchestrator Abstraction

## Overview

The Orchestrator is a framework for structuring software components using an **event-driven architecture**.

- Each functionality of the relayer is modeled as a flow with a series of steps.
- Each step in a flow is represented as an **event type**, and every event type has an associated **handler** that processes events of that type.
- Once an event is processed successfully or fails, the handler emits a **result event** (e.g., success or error).
- The result event is then processed by following handler and this continues until the flow is complete.

This decouples **functionality** (in the handlers) from **execution** (via an event dispatcher).
This allows the application to have multiple versions for same functionality with minor (or even major) differences in the flow.
It also simplifies tracing and error handling.

## Core Concepts

### 1. Events and Handlers

- Each step of a request that needs **intense computation** or **external system interaction** is modeled as an event.
- **Handlers** are assigned to these event types. They take the event as input, processes it, and emit a result event.

### 2. Identifiers

- **Event Type**: Distinguishes the kind of processing required. Events of the same type share a handler.
- **Request ID**: A unique identifier (often a UUID) linking all events that belong to the same end-to-end process or request.

### 3. Event Dispatcher Interface

Event dispatcher interface is an abstraction to **plug in various dispatcher implementations**.

- A **trait** (in Rust) or interface (in other languages) will define the core functions needed by any dispatcher.
- Possible implementations:
  1. **Tokio Event Dispatcher**
     - Runs handlers on a single machine using Tokio’s async runtime.
     - Suitable when handlers make I/O-bound calls (disk, network).
  2. **Rayon Event Dispatcher**
     - Runs handlers on a single machine using Rayon’s parallelism.
     - Useful for CPU-bound computations or parallel tasks.
  3. **Notifications-based Dispatchers** (e.g., SNS, SQL notifications)
     - Offloads some events to external systems, letting different machines process them.
  4. **Queue-based Dispatchers** (e.g., SQS, Kafka)
     - Fully distributed approach. Events can be processed on different machines/containers for maximum scalability.

It is possible to use one or a combination of dispatchers in a single orchestrator.

### 4. Hook Registration

- **Hooks** let you implement meta-handlers that attach to the event flow without modifying the core logic.
- Examples:
  - Persistence and Crash Recovery
  - Status Tracking (e.g., marking requests as pending, succeeded, or failed)
  - Event-level Monitoring (collecting metrics/tracing)

## Implementation Details

1. **Event trait**

   ```rust
   pub trait Event: Clone + Send + Sync + 'static {
        fn event_type(&self) -> u8;
        fn request_id(&self) -> Uuid;
    }
   ```

   Orchestrator just needs the event type and request id for handling the events.

2. **Event Dispatcher trait**

   ```rust
    #[async_trait]
    pub trait EventDispatcher<E: Event>: Send + Sync {
        async fn dispatch_event(&self, event: E) -> Result<(), Error>;
    }
   ```

   Defined generically over the `Event`, orchestrator uses the dispatcher to execute a handler for the event.

3. ** Event Handler and Handler Registry traits**

   ```rust
   #[async_trait]
    pub trait EventHandler<E: Event>: Send + Sync {
        async fn handle_event(&self, event: E);
    }

    pub trait HandlerRegistry<E: Event> {
        fn register_handler(&self, event_id: u8, handler: Arc<dyn EventHandler<E>>);
    }

   ```

   Consumers of the event implement the Handler trait and Orchestrator implements a Handler Registry. The Handlers are registered in the orchestrator in the main program.

4. **Request ID**
   - For request ID, we use UUID V1.
   - It includes a timestamp. Sorting the events orders them chronologically.
   - It includes a node ID. Enabling unique ID generation across different
     horizontal scaled instances without co-ordination.

## Example: Input Proof in Relayer SDK

Below is a high-level diagram showing how event-driven flow is structured for **Input Proof** in the Relayer SDK.
[Input Proof in Relayer SDK](./out/design-docs/input-flow-inside-relayer/Input%20flow%20inside%20relayer.svg)

## Benefits

1. **Decouples Functionality from Execution**

   - **Handlers** implement the business logic.
   - **Event Dispatchers** decide how/where to run that logic (locally, distributed, CPU parallel, etc.).

2. **Flexible Flow Definition**

   - Adding or removing a step is as simple as adding or removing an event type + handler.
   - Multiple versions or variations of the pipeline can coexist, sharing core handlers and diverging where needed.

3. **Meta-Features as Shared Components**
   - **Hooks** allow implementing cross-cutting concerns (persistence, status tracking, monitoring) _once_ and reusing them across flows.
   - Teams focus on the domain-specific handlers, while Orchestrator + hooks handle the rest.
