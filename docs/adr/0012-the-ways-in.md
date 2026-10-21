# ADR 0012 — The ways in: registration, sign-in services, Connectivity

**Status:** accepted, v0.5.0 (2026-10-21) · **Builds on:**
[ADR 0011](0011-the-administration-console.md) · **Applies to:**
`src/site.rs`, `src/sso.rs`, `src/events.rs`, `src/acl.rs` (identities),
`src/routes/entry.rs`, `src/routes/connectivity.rs`, `src/routes/admin.rs`
(sign-in, accounts), `templates/register.html`,
`templates/register_finish.html`, `templates/admin_login.html`,
`templates/admin_users.html`, `templates/admin_connectivity.html`,
`tests/entry.rs`

## Context

Until 0.5.0 an administrator created every account, and there was no other
way in. The operator asked for:

- a page administering the accounts, and the choice of letting a visitor
  open an account under three rules: freely, as a member; as a member, only
  with a secret; not at all, accounts being created by an administrator;
- quick account creation "via Google Authenticator" and other quick
  services, the worst case being a small classic form without them;
- every global policy of those services, the administrators' included, on a
  page of its own: **Connectivity**.

"Google Authenticator" is the one-time-code application; what creates an
account in one step is *Sign in with Google*. That is what is built, with
Microsoft, GitHub and any OpenID Connect provider beside it.

## Decision

### 1. The rules live with the accounts

The registration mode, the sign-in rules and the providers' settings are
kept under `site` in the `.acl` — mode 600, beside the bundle, never inside
it, in every backup. A client secret is written on the Connectivity page and
never shown again: the page says whether one is set, and a blank field keeps
it.

### 2. Registration: three modes, a member's account

- **Administrators only** (the default, and every existing installation).
- **With the family's secret**: a generated secret (five groups of four from
  an unambiguous alphabet), shown to administrators, drawn again on demand,
  compared in constant time.
- **Open to visitors.**

An account opened by a visitor is a member's (`viewer`): it reads what
members read and changes nothing. Its role is an administrator's choice
afterwards.

`/register` offers the services first, then a small form (username,
password, and the secret when it is asked). The form takes no email
address: an address typed into a form proves nothing, and one stored there
could later be matched (§4) to someone else's Google account. Every attempt
counts against the visitor's address, eight per quarter of an hour.

### 3. Sign-in services: OpenID Connect, done fully

- authorization code with PKCE (S256), `state` and `nonce`, the flow bound
  to the browser by a short-lived cookie (`SameSite=Lax`, so it survives
  the provider's redirect) and spent once;
- the ID token's issuer (Microsoft's per-tenant template included),
  audience, `azp`, expiry, issue time, nonce and subject checked; its
  signature is not, because it arrives directly from the token endpoint over
  TLS (OpenID Connect Core §3.1.3.7);
- an address counts as verified only when the provider says so explicitly
  (Microsoft does not);
- GitHub, which is OAuth 2.0 without an ID token, through its user and
  addresses APIs;
- the session is opened by a page that moves on by itself rather than by a
  redirect, so that the `SameSite=Strict` session cookie travels with the
  next, same-site, request.

### 4. How a service's answer finds its account

1. The identity linked before (provider, issuer, subject).
2. When the rule allows it, an active account holding the same address,
   **only if the provider verified it**, and only an address an
   administrator typed or a provider verified (§2). This is how a relative
   gets a one-click sign-in without any registration: an administrator
   creates the account with their address, and their first sign-in links
   it.
3. Somebody new: refused on a closed site; refused outside the allowed
   domains; under the secret, held until it is given
   (`/register/finish`); otherwise a member's account is created, its
   address kept only if verified.

### 5. The administrators' rules

- **Passwords**: for everyone; for everyone but administrators (whose
  stolen password then opens nothing); or for nobody. A rule, or a change
  to a provider, that would leave no active administrator a way in is
  refused: the emergency token remains, but it is the way back from a
  mistake, not a reason to allow one.
- **Link by verified address**, on by default.
- **Allowed domains** for new accounts; when set, the registration form is
  not offered.

### 6. What is counted

Failed sign-ins, sign-ins, accounts opened and cross-site posts are kept for
a day in memory (`src/events.rs`) for the Infrastructure page's checks.

## Consequences

- A service needs the public address set, and that address in HTTPS: the
  Infrastructure page marks the opposite critical.
- The production unit refuses outgoing connections; a service needs them
  (`IPAddressAllow` in a drop-in). The Connectivity page says so and has a
  button that tests the connection.
- An account created by a service has no password (`!no-password`, which
  nothing verifies against); an administrator can give it one.

## Alternatives considered

- **Time-based one-time codes (Google Authenticator) as the quick way.**
  Not what was meant: a second factor makes an account safer, not quicker
  to create. It remains possible later, on top of this.
- **Verifying ID token signatures.** Not needed for a token received
  directly from the token endpoint over TLS; it would add a JWKS cache and
  key rotation for no protection.
- **Taking an address on the registration form.** Rejected, §2.
