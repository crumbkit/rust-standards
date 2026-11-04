# dynamodb

A project adopts this profile when it stores data in Amazon DynamoDB. The
module or crate that uses the DynamoDB client is the store's owner under
SVC-12, or under BASE-5 in a project that does not adopt `service`.

A request path is code that runs while a caller waits for its result, such
as an inbound request or a message handler. A batch job is code that runs on
a schedule or by hand, with no caller waiting for it.

## lint

| ID | Rule | Check |
| --- | --- | --- |
| DDB-3 | No request path runs a `Scan` or a PartiQL statement. A batch job may scan, with a `Limit` on each page (DDB-4) and a deadline (BASE-34). Each batch job that scans suppresses the lint with `#[expect(clippy::disallowed_methods, reason = "...")]`, and the map's DDB-3 binding lists the batch jobs' files. | `disallowed_methods` on `Client::scan` and the PartiQL methods ([baseline/clippy/dynamodb.toml](../baseline/clippy/dynamodb.toml)); drift for the binding |

## test

| ID | Rule | Check |
| --- | --- | --- |
| DDB-1 | Keep the table's key layout in one module: the format of every partition key and sort key, every index name and every attribute name. The module's documentation holds an access-pattern table with one row per access pattern, which names the operation, the index, the key condition and the owner's function that runs it. Each key format is a type whose fields are private and whose constructor and parser are in the module (BASE-42). No other code formats a key or names an attribute or an index with a string. The map's DDB-1 binding names the module. | the compiler, because only the module can build a key type; a test that builds each key format and parses it back to its parts; review that the access-pattern table matches the owner's functions |
| DDB-2 | Enforce each invariant that spans writes, such as a unique email address or a seat that is taken once, with a condition expression on the write or with `TransactWriteItems`. Never read an item and then decide on the result whether to write. A write whose new value depends on the old value conditions on a version attribute or on the old value. A write that fails its condition returns a variant of the operation's outcome (BASE-44), such as `AlreadyTaken`. | a test per invariant, against DynamoDB Local or a test table, that runs two conflicting writes at once and checks that exactly one succeeds and the other returns the outcome variant |
| DDB-4 | Every `Query`, and every `Scan` that a batch job runs, sets `Limit` and stops at a bound. The bound is a named constant or a validated configuration value, and it counts the items or the pages that the call reads. A function that serves a caller returns one page and an opaque cursor that the DDB-1 module encodes from `LastEvaluatedKey`, or it reads up to the bound and its outcome says that more items remain. No code collects every page of a paginator without a bound. | a test that writes more items than the bound and checks that the function returns no more than the bound with a cursor, and that following the cursors returns every item exactly once |
| DDB-5 | Every DynamoDB call runs under the operation's deadline (BASE-34). The owner sets the call's operation timeout, which covers the SDK's retries, from what remains of the deadline (BASE-51), and fails without sending when nothing remains. The SDK retries only calls that are safe to repeat. A read, an unconditional `PutItem` and a `DeleteItem` are safe. A conditional write is safe when it sets `ReturnValuesOnConditionCheckFailure` to `ALL_OLD` and treats a returned item that holds its own write as success. A `TransactWriteItems` call is safe when it sets `ClientRequestToken`. Any other write, such as an `UpdateItem` that uses `ADD` or arithmetic without a version condition, runs with the SDK's retries disabled for that call. | a test that calls with no time left and checks that nothing is sent; a test with the SDK's replay HTTP client that times out the first attempt and checks that each unsafe write is sent once and each safe call is retried |
| DDB-6 | Each item type documents its size bound, which counts attribute names and values as DynamoDB measures them, and the bound is below DynamoDB's limit of 400 KB per item. Every variable-length attribute has a length limit that its type enforces (BASE-34, BASE-42). An item type that expires names its TTL attribute, which holds the expiry time as a Number of Unix epoch seconds. A read of such a type ignores an item whose expiry time has passed. | a test that builds the largest item each type allows and checks its size against the stated bound; a test that writes an expired item and checks that reads do not return it |

## Reasons

- DDB-1: A key that several modules format drifts when one of them changes
  the format, and items written under the old format stop matching any
  query. An access-pattern table shows a reviewer each query the table must
  serve, and which index serves it, before an index is added or removed.
- DDB-2: Between a read and the write that depends on it, another writer can
  change the item, so both writers pass the check. A condition expression or
  a transaction makes DynamoDB check the invariant at the moment of the
  write.
- DDB-3: A `Scan` reads every item in the table or index, so its latency and
  its read cost grow with the table, and a request path that scans becomes
  slow and expensive without any change to its code. A PartiQL `SELECT`
  whose condition does not fix the partition key also runs as a scan, and a
  PartiQL statement is a string that DDB-1's key types cannot check.
- DDB-4: DynamoDB returns at most 1 MB per page, and code that follows
  `LastEvaluatedKey` without a bound reads the whole partition, which grows
  with the data. A cursor that the key layout module encodes keeps the key
  attributes from becoming part of a wire contract (SVC-3).
- DDB-5: The SDK retries throttling and transient errors by default. When an
  attempt times out after DynamoDB applied the write, a retry of an `ADD`
  applies it twice, and a retry of a conditional write fails its own
  condition and reports a conflict that did not happen. DynamoDB treats a
  repeated `TransactWriteItems` call with the same `ClientRequestToken` as
  one call for ten minutes. The Rust SDK disables retries for one call
  through `customize().config_override(...)`.
- DDB-6: A write of an item over 400 KB fails, so an attribute with no
  length limit turns large input into a failed request. DynamoDB deletes an
  expired item up to a few days after its expiry time, so a read that trusts
  the TTL alone returns expired items.
