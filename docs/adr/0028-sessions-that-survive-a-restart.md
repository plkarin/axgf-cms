# ADR 0028 — Sessions that survive a restart

**Status:** accepted, v1.3.1 (2026-11-03) · **Reverses:** the sessions held in memory
only (`src/session.rs`, "In memory, on purpose") · **Applies to:**
`src/session.rs`, `src/state.rs`, `src/main.rs`, `tests/accounts.rs`

## Context

On 3 November 2026 the operator reported being signed out often, on a
simple reload, and asked for sessions to last longer. Three causes added up:

1. **A restart signed everyone out.**
   - The sessions lived in memory, under a secret drawn at startup.
   - That was chosen as "a mild inconvenience at this scale".
   - That evening, upgrades and settings changes restarted the service about
     ten times, and each restart signed the family out.
2. **A session ended 12 hours after signing in**, however much it was used.
3. **Two installations on one machine signed each other out.**
   - A browser keeps one cookie per name and per host, whatever the port.
   - The production (port 8088) and the preview (port 8089), both on
     192.168.1.105, therefore wrote over each other's `axgf_session`.

## Decision

### 1. Sessions kept beside the bundle

Every account's session is written to `family.sessions` beside the bundle:

- **Mode:** the file is mode 600, written whole and renamed into place.
- **Contents:** the secret that signs the cookies, and for each session its
  account and its expiry.
- **At startup:** the next run reads it back and drops what has expired.

It is not the bundle, and it is not backed up. It is transient state: a file
that is missing, unreadable or corrupt only means that everyone signs in
again, with a fresh secret. To sign everyone out, an operator deletes it and
restarts the service.

What changes a session is written at once:

- signing in;
- signing out;
- a session closed because a password changed, an account was disabled or
  its role was lowered.

A revocation therefore holds after a restart.

**The emergency token's session is never written.** It ends with the
process, as it did.

### 2. Thirty days without use, moved on by use

- **Server side:** a session lasts 30 days without being used. Each use
  moves that on, in steps of at least an hour, so a session in use is
  written to disk at most once an hour, not on every page.
- **Browser side:** the browser keeps the cookie as long as browsers allow
  (400 days). The expiry on the server is what ends the session.

### 3. A cookie name of the installation's own

The cookie is named `axgf_session_<mark>`. The mark is the first eight
hexadecimal digits of the SHA-256 of the bundle's path, which no other
installation can share, since two servers over one bundle are refused. A
library caller, such as the test suite, keeps the plain name.

## Consequences

- **A family member stays signed in** across upgrades, restarts and weeks of
  absence shorter than a month.
- **One more file in the data directory.** It is small, mode 600 and
  disposable. It is not in any backup, on purpose.
- **A stolen cookie is good for longer.** A cookie taken from a browser stays
  usable up to 30 days after its last use, instead of 12 hours after sign-in.
  - Signing out, changing the password or disabling the account still ends it
    at once.
  - Over plain HTTP on a LAN, the cookie travels in clear text. A TLS reverse
    proxy (docs/DEPLOY.md) is the answer, as it always was.
- **Everyone signs in once more** when this version is installed: the cookie's
  name changes.
