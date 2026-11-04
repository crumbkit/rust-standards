# personal-data

A project adopts this profile when it handles personal data. A personal
value is data that identifies or describes a person, such as a name, a phone
number, an email address, an IP address, or what a person said, typed or was
sent. Credentials are secrets, which BASE-24 governs.

A projection is any copy of data that leaves the store and the screens that
own it: a log line, a trace span, a metric label, an error report, an event,
an export or a request to another service.

## test

| ID | Rule | Check |
| --- | --- | --- |
| PII-1 | Hold every personal value in a wrapper type, such as the project's `Pii<T>`, or in a domain type such as `PhoneNumber` that keeps its value in a private field. The type never derives `Debug`. Its hand-written `Debug` and `Display` print a redacted form, such as `[phone-number]`, that the type documents. | a test that `format!("{:?}")` and `format!("{}")` of each personal type print the redacted form |
| PII-2 | A wrapper type does not implement `Serialize`. A wire type serializes a personal value with `serialize_with`, through a function that calls the expose method at that boundary. | a test that serializes each wire type that holds a personal value and checks the output |
| PII-3 | A URL, a path or a query string never holds a personal value. This includes callback URLs given to other services and links in messages sent to people. A URL carries an opaque reference, such as a record's ID, instead. | a test of each route and each URL the program builds, and review |
| PII-4 | Logs, traces, metrics and error reports never hold a personal value. A metric label never holds a personal value or a reference to one. SVC-5's redaction test also logs each personal type the program handles and checks that the raw value is absent from the output. | the SVC-5 redaction test |
| PII-5 | Build each projection from an allowlist of fields. Never copy a record and remove the keys known to be personal. | a test per projection that adds a field to the source record and checks that the projection does not carry it |
| PII-6 | A response that holds personal data sends `Cache-Control: no-store`. No cache key contains a personal value. | a test of each such response's headers |

## drift

| ID | Rule | Check |
| --- | --- | --- |
| PII-7 | Code reads a personal value's raw form only through the type's explicit expose method, such as `Pii::expose`, and only in the files that the map's PII-7 binding lists. | drift, built like BASE-24's check |

## review

| ID | Rule |
| --- | --- |
| PII-8 | An error or a refusal never holds a personal value in its message. A refusal of personal input names the field and the kind of value. An error that keeps a response body or refused input keeps it in a wrapper type. |
| PII-9 | The repository holds no real personal data. Fixtures, seeds, mocks and recorded requests use synthetic values. A test phone number comes from a range reserved for fiction, such as Ofcom's +44 7700 900000 to +44 7700 900999. |

Parsing a personal value into its type where it enters the program, and
taking that type everywhere inside, follows from BASE-32 and BASE-42.
SVC-9's `skip_all` keeps arguments out of spans.

## Reasons

- PII-1: `secrecy` prints a fixed placeholder, which hides which value a log
  line refers to. A documented redacted form lets a reader tell the kind of
  value apart without seeing it.
- PII-3: A URL is copied into access logs, proxy logs, browser history and
  referrer headers, which no redaction reaches.
- PII-5: A projection that removes known keys carries every personal field
  that is added to the record later.
- PII-6: A shared cache or a browser cache can keep the response after the
  person's session ends.
- PII-7: A short list of files that may read the raw value makes every new
  reader visible in review.
