# control-plane

A project adopts this profile when it drives other resources toward a
desired state.

## test

| ID | Rule | Check |
| --- | --- | --- |
| CP-1 | Reconciliation is level-triggered: each pass reads the desired and the observed state and acts on the difference. An event only starts a pass, and a periodic resync also runs, so a missed event is corrected on the next pass. | a test that drops an event and checks that the next resync acts on the difference |
| CP-2 | A pass that finds no difference performs no effect. | a test that runs a pass twice and checks that the second performs no effect |
| CP-3 | Each managed resource records the generation it was asked for, the generation last acted on, and its current condition, including the last error. | a test that fails a pass and reads the resource's record |
| CP-4 | One writer acts on a resource at a time. The writer holds a lease that expires, and every write carries a fencing token, so a writer that lost its lease cannot write. Where the remote resource cannot check a fencing token, the writer uses the resource's conditional write instead: an ETag with `If-Match`, a version, or an expected state that the API compares before it writes. A resource with no conditional write is listed as an exception in the map, with the module that writes it in `paths`. | a test in which a writer whose lease expired tries to write, and, for a resource written conditionally, a test in which a stale write is refused |
| CP-5 | Delete a remote resource only when its recorded owner and identity match. This extends DUR-4 from files to remote resources. | a test of deletion with a competing owner |
| CP-6 | The data plane keeps serving with its last good configuration while the control plane is unreachable. Configuration is versioned and can be rolled back. | a test with the control plane unavailable, and a test of a rollback |
| CP-7 | Retries back off per resource, so one failing resource does not delay the others. | a test with one resource failing that checks the others are reconciled |

## Reasons

- CP-1: An edge-triggered controller acts only when an event arrives. After
  one lost event it never acts on that change, and the resource stays out of
  its desired state until something else changes it.
- CP-6: The data plane serves the users. If it stops when the control plane
  is unreachable, an outage of the control plane becomes an outage for the
  users.
