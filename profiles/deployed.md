# deployed

A project adopts this profile when it runs as a long-lived container on
hosted infrastructure. A program that serves traffic only on a developer's
machine adopts `service` and not this profile. A deployed project that serves
network traffic also adopts `service`.

## test

| ID | Rule | Check |
| --- | --- | --- |
| DEP-1 | Write logs as one JSON object per line on standard output, with the fields `timestamp` (RFC 3339, UTC), `level`, `message`, `target`, `trace_id` and `span_id`. Write no log files. | a test that parses each line of a request's log output |
| DEP-2 | Read the log level from a variable in the BASE-28 catalog, so changing it needs no rebuild. | a test that starts the program with the variable set and checks the level |
| DEP-3 | Install a panic hook that writes the panic as one structured log event at error level before the process exits. | a test that panics a child process and parses its last log line |
| DEP-4 | Read W3C `traceparent` from every inbound request and message, and send it on every outbound call and message. Start a new trace when an inbound request carries none. Export spans over OTLP when the deployment names an endpoint. | a test that sends a request with `traceparent` and checks the outbound call's header, and a test without one that checks a new trace starts |
| DEP-5 | SIGTERM starts a graceful shutdown. A service stops accepting work and drains it as SVC-2 says, and a worker stops taking new messages and finishes or releases the ones in progress. The drain deadline is shorter than the platform's stop timeout, after which the platform kills the process. | a test that sends SIGTERM with work in progress |
| DEP-6 | Validate the whole configuration at startup. On an invalid value, exit with a non-zero code that names the variable, before accepting any traffic. Run database migrations as their own command, never as a side effect of starting the service. | a test that starts the program with each variable invalid |
| DEP-7 | A message handler gives the same result when the same message is delivered twice. | a test that delivers each message kind twice |
| DEP-8 | A service caps the requests in flight per instance with a named, configurable limit. Past the cap it answers at once with 503 and `Retry-After`. | a test that sends one request more than the cap and checks the answer |

## review

| ID | Rule |
| --- | --- |
| DEP-9 | Build the image in several stages. It runs as a non-root user, starts one process, and carries the OCI labels `org.opencontainers.image.revision` and `org.opencontainers.image.version`. |
| DEP-10 | An instance keeps no state that another instance needs. Any instance can serve any request, and a lost cache loses no data. |

## Reasons

- DEP-1: Log backends link logs to traces by field name. A backend that
  expects other names renames the fields in its own ingestion pipeline, so
  the code keeps one set of names.
- DEP-4: W3C trace context and OTLP are supported by every major tracing
  backend, so moving to another backend changes only the collector's
  configuration.
- DEP-8: Answering at once past the cap keeps an overloaded instance
  serving the requests it already holds, and `Retry-After` tells the caller
  when to try again.
