# deployed

A project adopts this profile when it runs as a long-lived container on
hosted infrastructure. A program that serves traffic only on a developer's
machine adopts `service` and not this profile. A deployed project that serves
network traffic also adopts `service`. DEP-8 and DEP-12 apply only to a
deployed project that serves traffic.

## test

| ID | Rule | Check |
| --- | --- | --- |
| DEP-1 | Write each log line as one OpenTelemetry log record, flattened to one JSON object, on standard output or standard error. Each line has every field that [Log fields](#log-fields) requires. Write no log files. | a test that parses each line of a request's log output and checks its fields |
| DEP-2 | Read the log level from a variable in the BASE-28 catalog, so changing it needs no rebuild. | a test that starts the program with the variable set and checks the level |
| DEP-3 | Install a panic hook that writes the panic as one structured log event at error level before the process exits. | a test that panics a child process and parses its last log line |
| DEP-4 | Read W3C `traceparent` from every inbound request and message, and send it on every outbound call and message. Start a new trace when an inbound request carries none. Export spans over OTLP when the deployment names an endpoint. | a test that sends a request with `traceparent` and checks the outbound call's header, and a test without one that checks a new trace starts |
| DEP-5 | SIGTERM starts a graceful shutdown. A service stops accepting work and drains it as SVC-2 says, and a worker stops taking new messages and finishes or releases the ones in progress. The drain deadline is shorter than the platform's stop timeout, after which the platform kills the process. The map's `shutdown` decision records the drain deadline and the stop timeout ([adoption.md](../adoption.md#decisions)). | a test that sends SIGTERM with work in progress |
| DEP-6 | Validate the whole configuration at startup. On an invalid value, exit with a non-zero code that names the variable, before accepting any traffic. Run database migrations as their own command, never as a side effect of starting the service. | a test that starts the program with each variable invalid |
| DEP-7 | A message handler gives the same result when the same message is delivered twice. | a test that delivers each message kind twice |
| DEP-8 | A service caps the requests in flight per instance with a named, configurable limit. Past the cap it sheds load at once with the answer that the caller's protocol treats as busy, which for plain HTTP is 503 with `Retry-After`. Where work arrives in sessions, the cap counts the sessions in progress and applies when a session starts, and every request that belongs to an admitted session is served. The cap is the load at which a load test shows the instance's latency, memory or CPU degrading. The map's `request-cap` decision records the cap and the measurement it came from, or, until a measurement exists, that the cap is owed. Autoscaling adds instances before instances reach the cap. | a test that sends one request more than the cap and checks the busy answer; for sessions, a test that fills the cap and checks that a new session is refused while a request for an admitted session is served; a reviewer compares the cap with the measurement and the scaling policy |
| DEP-11 | Each inbound request, each message handled and each scheduled job run writes one wide event when it ends, whether it succeeds, fails or is cancelled. The wide event is the record of the unit's span, and it holds the fields that [Log fields](#log-fields) requires of a wide event. | a test that runs a request and each kind of message and job to success, to error and to cancellation, and finds exactly one wide event with the required fields for each |
| DEP-12 | In a deployed project that serves traffic, the load balancer's health check calls `GET /up`, which checks only that the process can serve requests. Dependency status goes to metrics and logs, never to `/up`. The load balancer never calls SVC-6's `GET /ready`. DEP-8's cap does not count `/up`. | a test with a store unavailable that checks `GET /up` still answers 200 |
| DEP-14 | The binary reports its version and commit through `--version`. | a test of the `--version` output |
| DEP-16 | The program reads its environment name from one variable in the BASE-28 catalog, whose name the project chooses. Its values are a closed set that the catalog lists, such as `production`, `staging` and `development`, and they match `deployment.environment.name` in the logs. When the variable is unset, empty or any other value, the program refuses to start and names the variable (DEP-6). | a test that the program refuses to start with the variable unset, empty and set to an unknown value |

## review

| ID | Rule | Where it applies |
| --- | --- | --- |
| DEP-9 | Build the image in several stages. It runs as a non-root user, starts one process, and carries the OCI labels `org.opencontainers.image.revision` and `org.opencontainers.image.version`. Every image is published with an SBOM. The production image should use a distroless base, such as `gcr.io/distroless/cc-debian12:nonroot`, or `gcr.io/distroless/static-debian12:nonroot` for a fully static binary. A project whose production image has no shell also builds a debug image from the same binary on the matching `:debug-nonroot` base, and tags it beside the production image, such as `git-<sha>-debug` beside `git-<sha>`. | `**/Dockerfile*`, `**/*.Dockerfile`, `**/Containerfile*`, `**/docker-bake.hcl`, `**/Makefile`, `**/*.mk`, `**/justfile`, `**/Justfile`, `**/.justfile`, `**/*.sh`, `bin/**`, `scripts/**`, `.github/workflows/**` |
| DEP-10 | An instance keeps no state that another instance needs. Any instance can serve any request, and a lost cache loses no data. | `**/*.rs` |
| DEP-13 | Where the deployment collects metrics, their names and units follow the OpenTelemetry semantic conventions. Every label value comes from a bounded set, such as a route template, and is never a raw path, an ID or a URL. A build-info metric carries the version and commit that DEP-14 reports. | `**/*.rs`, `**/*.toml`, `**/*.yaml`, `**/*.yml`, `.github/workflows/**` |
| DEP-15 | Size worker threads from `std::thread::available_parallelism`, which respects the container's CPU limit. Every cache has a bound on its entries or its bytes. | `**/*.rs`, `**/*.toml`, `**/*.yaml`, `**/*.yml`, `.github/workflows/**` |

## Log fields

Field names follow the OpenTelemetry semantic conventions. A field that has
no convention goes under the project's prefix, such as `notekeeper.`. An ID
field holds the opaque ID, never a name or a phone number.

Every line has these fields:

| Field | Status | Holds |
| --- | --- | --- |
| `timestamp` | required | RFC 3339, UTC |
| `level` | required | the severity text `TRACE`, `DEBUG`, `INFO`, `WARN` or `ERROR` |
| `message` | required | the log body |
| `service.name` | required | the program's name |
| `service.version` | required | the full commit SHA of the build |
| `trace_id`, `span_id` | required inside a trace | the current span's IDs |
| `deployment.environment.name` | recommended | the environment name that DEP-16 reads |
| `target` | required | the module that wrote the line, which is the instrumentation scope |

A wide event (DEP-11) also has these fields. `<prefix>` is the project's
prefix.

| Field | Status | Holds |
| --- | --- | --- |
| `event.name` | required | `<prefix>.request`, `<prefix>.message` or `<prefix>.job` |
| `<prefix>.outcome` | required | `success`, `error` or `cancelled` |
| `<prefix>.duration_ms` | required | whole milliseconds |
| `error.type` | required on error | the error's stable code (SVC-4) |
| `http.request.method` | required on a request | the method |
| `http.route` | required on a request | the route template, never the raw path |
| `http.response.status_code` | required on a request that sent a response | the status; absent when the outcome is `cancelled` |
| `<prefix>.tenant.id` | required in a multi-tenant service once the unit has resolved its tenant | the tenant (SVC-11) |

## Reasons

- DEP-1: Log backends link logs to traces by field name. A backend that
  expects other names renames the fields in its own ingestion pipeline, so
  the code keeps one set of names. `service.name` and `service.version` let
  a query find every line of one build without a join.
- DEP-4: W3C trace context and OTLP are supported by every major tracing
  backend, so moving to another backend changes only the collector's
  configuration.
- DEP-8: Answering at once past the cap keeps an overloaded instance
  serving the requests it already holds, and `Retry-After` tells the caller
  when to try again. A cap chosen without a measurement is either too low,
  which refuses load the instance could carry, or too high, which lets the
  instance degrade before it sheds anything. Autoscaling that starts only
  after instances reach the cap sheds load that a new instance would have
  carried.
- DEP-9: A distroless image has no shell or package manager for an attacker
  to use, and fewer packages for advisories to name. An incident that needs
  a shell redeploys the debug tag of the same build, so nothing is compiled
  during the incident. The redeploy replaces the instance being inspected,
  so that instance's memory and open connections are lost.
- DEP-11: A wide event answers a question about one unit of work with one
  query. Many small log lines answer it only after they are joined, and a
  line that a sampler or a level drops leaves a gap.
- DEP-12: A store outage fails a health check that checks the store on every
  instance at once. The load balancer then removes every instance and the
  service answers nothing, even requests that do not need the store.
- DEP-13: Each distinct label value creates a separate time series. One
  unbounded label, such as a customer ID, multiplies a metric's series by
  the number of customers.
- DEP-15: A count of the host's CPUs ignores the container's limit, so a
  runtime sized from it starts more threads than the container may use, and
  the platform throttles them. An unbounded cache grows until the
  platform kills the instance for its memory.
- DEP-16: A default environment name such as `development` turns off the
  checks that production needs, and a container that lost its variable
  would start without them and report nothing.
