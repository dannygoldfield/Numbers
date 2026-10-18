# Invited-session operating examples

These files are implementation artifacts for review and local rehearsal. They do not authorize installation, DNS changes, credentials for real people, invitations, spending or deployment. The separate deployment plan must name the DigitalOcean account/Droplet, hostname/TLS method, package versions, operator, effective firewall rules, budget cap, private credential delivery/revocation, backup destination and retention/shutdown plan.

## Boundary

One NGINX HTTPS gateway authenticates bidder-a, bidder-b and facilitator. It overwrites `X-Numbers-Principal`, strips Authorization and all unlisted forwarded headers, buffers complete bounded requests and makes one upstream attempt. The application binds loopback and enforces role, bidder binding and transport/origin rules before evaluation. Every resource is authenticated. Application state/history GETs can evaluate time and must not be used as anonymous health probes; `/access` is authenticated and non-evaluating.

The host, administrators and local processes remain trusted. Loopback assertions are not a defense against malicious local software. Keep the backend unreachable externally. Inspect the combined effective firewall rules; permit HTTPS to invited clients and SSH only from approved operator sources. Do not weaken TLS validation to make a client connect.

`nginx.conf.example` is a standalone template. Replace its marked placeholders in an external configuration, including the writable runtime directory, listening address, hostname, certificate/key, credential file and loopback backend. Use a valid hostname/certificate for a real test. The local acceptance script supplies a certificate trusted only by that script and binds loopback. Body size is 4,096 bytes; headers are bounded to ten seconds and body/upstream inactivity to ten seconds. These are transport limits, not economic deadlines. There is no API cache, upstream retry or POST redirect.

## Credentials and session association

Generate a distinct secret with at least 128 random bits for each of the three principals. Store only supported salted password hashes in the protected gateway credential file, outside Git; deliver the plaintext privately through the separately approved method. Do not put credentials in URLs, commands, canonical records or application configuration. Do not log Authorization or full bodies. The example logs only a fixed known principal/route, method, status and time; arbitrary supplied usernames, paths and queries are excluded. Avoid debug logging of gateway requests.

Assign bidder-a permanently to one invited tester's demo-1 account and bidder-b to the other tester's demo-2 account for this history. Keep assignment evidence privately outside the journal; use simulated labels in session records. No reassignment or role switching within a history. Rotation of a compromised secret requires stopped ingress and preserves that same assignment. Removing its hash revokes access. A new test group needs a distinct label/history.

Prepare a new external session directory and explicit configuration. Build the binary once and preserve its checksum; the process receipt also captures the executable and configuration hashes. Initialize only with `--initialize` against an unused history path. Restart the same history with the same binary/configuration without that flag. The fixed principal mapping and label are in `config.session.json`; credentials remain external. The service example runs as an unprivileged dedicated user with `Restart=no` and no automatic boot target. Initialize deliberately before using the normal service command.

Do not repoint an existing configuration to a new database, remove its receipt, or silently substitute a new store. A changed association fails closed. Diagnostic code and guided engine tests do not provide a remote authentication bypass. The old live terminal is retained only in the preserved 0.2 build.

## Facilitation, shutdown and preservation

Before and after the trial:

1. Stop gateway ingress and allow the current authoritative boundary to finish where possible, then stop the application writer. Disable automatic restart. Any interrupted transaction follows PR03–PR06; stopping never pauses auction time.
2. Copy the stopped journal, matching executable/source reference, exact config, receipt and relevant nonsecret diagnostics into a new preservation directory. Keep credentials/keys in the separately protected credential backup if the approved plan requires it, never in the journal bundle.
3. Record checksums, run SQLite integrity checking and matching read-only reconstruction on a copy. For the inspector, use a directory containing `history.sqlite3` and its matching `config.json`. Confirm the original and copied journal hashes stay unchanged by inspection.
4. Store a verified off-host copy at the approved destination. Record the committed prefix and verification result. Test copies are disposable; originals and preserved bundles are not.

For an unclear command outcome, stop ingress/process and inspect durable history with the matching reader. Resume only that valid same history and association after identifying the failure. No replay, balance correction, deadline shift, winner substitution, or inferred failed settlement is permitted.

A backup is a prefix, not permission to roll back canonical facts. If later commits might be missing, do not serve an older snapshot as continuation. Halt and preserve the available evidence. Resource exhaustion, disk failure, clock failure and connection errors cannot select an economic outcome.

## Local rehearsal and limits

Run `NUMBERS_TEST_NGINX=/path/to/nginx python3 tests/gateway.py --directory /new/disposable/directory` from `prototype-01/`. It uses the actual template and application, removes temporary keys/passwords and stops its servers. This proves local controls; it does not verify a future Droplet's firewall, DNS, certificate renewal, operator access, storage, clock or off-host backup. Those checks and the two-device observation session follow separate deployment approval.
