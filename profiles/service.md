# service

A project adopts this profile when it serves network traffic.

## lint

| ID | Rule | Check |
| --- | --- | --- |
| SVC-1 | One module spawns tasks and threads, and every task or thread has an owner that joins or aborts it. | [baseline/clippy/service.toml](../baseline/clippy/service.toml) |

## test

| ID | Rule | Check |
| --- | --- | --- |
| SVC-2 | Shutdown stops accepting work, lets work in progress finish within a deadline, then cancels what remains. | a test that shuts down with work in progress |
| SVC-3 | Keep every wire contract change compatible. A new field is optional, and a field is removed or retyped only once no client reads it, or under a new endpoint version. | fixtures of recorded requests and responses |
| SVC-4 | Every error response has a stable code. A service that owns its error format uses RFC 9457 problem details. A service that replaces another keeps its predecessor's format. | a test that lists every code an endpoint returns |
| SVC-5 | Redact headers, tokens and credentials before they reach a log, a trace or an error response. | a test that logs a request carrying a secret and searches the output for it |
| SVC-6 | The readiness endpoint reports ready only when the stores the service needs are reachable. | a test with a store unavailable |

## review

| ID | Rule |
| --- | --- |
| SVC-7 | Each outbound call takes its timeout from what remains of the inbound request's deadline. A retry has a count and backs off. |
| SVC-8 | No async task makes a blocking call. Blocking work runs through `spawn_blocking` in the task owner (SVC-1) or through the owner of its effect (BASE-5). Every channel and queue is bounded. A future used in `select!` is cancel-safe, or a comment says why it need not be. |
| SVC-9 | Log through `tracing`, with each value as a structured field. Each request has a span that holds its ID. A service whose deployment collects metrics exports request rate, error count and latency. |

## Reasons

- SVC-8: A blocking call stalls every task on its runtime thread. An
  unbounded channel grows until the process runs out of memory when a
  consumer falls behind. `select!` drops the futures that lose the race, so a
  future that is not cancel-safe loses data it had already read.
- SVC-9: A structured field can be searched and aggregated. A value
  formatted into the message text cannot.
