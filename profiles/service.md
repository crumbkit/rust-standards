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
| SVC-5 | Redact headers, tokens and credentials before they reach a log, a trace or an error response. The one exception is a validated idempotency key, which SVC-10 records in a log field. | a test that logs a request carrying a secret and searches the output for it |
| SVC-6 | A service whose stores can be unreachable serves a readiness endpoint at `GET /ready`, without authentication and separate from the health check at `GET /health_check`. `GET /ready` answers with a 2xx status only when the stores that the service needs are reachable, and with 503 otherwise. A deployed service's load balancer never calls it (DEP-12). | a test with a store unavailable that checks `GET /ready` answers 503 |
| SVC-10 | A service that sends or accepts an idempotency key validates the key before it uses or logs it. The key's format is a documented, bounded, opaque operation ID that holds no secret and no personal data. The service refuses an invalid key without logging or echoing its raw text. A valid key stays the same across all retries of one operation, and the service records it in one log field under the project's prefix, such as `notekeeper.idempotency.key`, on each attempt, each outcome and each duplicate replay. | a test that captures the keys on the wire and in the structured logs across retries, a duplicate replay and a failure, and checks that one operation's keys are equal and separate operations' keys differ; and a test that sends a malformed key, an oversized key and a key holding a secret, and checks that each is refused and that its raw text appears in no log, trace or error response |
| SVC-11 | A multi-tenant service scopes every store query to one tenant. The query takes a tenant type that only the authentication layer can construct, so a query without an authenticated tenant does not compile. A query that spans tenants by design, such as a migration, takes a separate type whose name states that purpose. | a test per table that writes rows for two tenants and checks that each tenant's queries return none of the other tenant's rows |

## review

| ID | Rule |
| --- | --- |
| SVC-7 | Each outbound call takes its timeout from what remains of the inbound request's deadline. A retry has a count and backs off. |
| SVC-8 | No async task makes a blocking call. Blocking work runs through `spawn_blocking` in the task owner (SVC-1) or through the owner of its effect (BASE-5). Every channel and queue is bounded. A future used in `select!` is cancel-safe, or a comment says why it need not be. |
| SVC-9 | Log through `tracing`, with each value as a structured field. Each request has a span that holds its ID. Every `#[instrument]` attribute uses `skip_all` and names the fields it records. A service whose deployment collects metrics exports request rate, error count and latency. |

## Reasons

- SVC-6: A readiness endpoint lets a tool or a startup check wait until the
  service can do its work. The load balancer uses the health check instead,
  for the reason DEP-12 gives.
- SVC-8: A blocking call stalls every task on its runtime thread. An
  unbounded channel grows until the process runs out of memory when a
  consumer falls behind. `select!` drops the futures that lose the race, so a
  future that is not cancel-safe loses data it had already read.
- SVC-9: A structured field can be searched and aggregated. A value
  formatted into the message text cannot. Without `skip_all`,
  `#[instrument]` records every argument with its `Debug` form, so a new
  argument that holds a secret or a personal value reaches the trace without
  any change to the attribute.
- SVC-10: A key that changes between retries makes the server run the
  operation twice. A key that holds a secret or personal data leaks it into
  every log line that records the key. Echoing a refused key copies
  untrusted text into logs and responses.
- SVC-11: A filter that each query adds by hand is missed in one query
  sooner or later, and that query returns every tenant's rows. A type that
  only the authentication layer builds makes the compiler find the missing
  filter, and the per-table test finds a query that takes the type and
  ignores it.
