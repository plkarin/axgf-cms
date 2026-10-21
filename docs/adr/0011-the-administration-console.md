# ADR 0011 — The administration console

**Status:** accepted, v0.5.0 (2026-10-21) · **Applies to:**
`src/routes/console.rs`, `src/conformity.rs`, `src/findings.rs`,
`src/selfcheck.rs`, `src/host.rs`, `src/releases.rs`, `src/seal.rs`,
`src/events.rs`, `src/routes/mod.rs` (`same_site_writes`, `cache_policy`),
`templates/_admin_nav.html`, `templates/_checks.html`,
`templates/admin_bundle.html`, `templates/admin_infrastructure.html`,
`templates/admin_dashboard.html`, `deploy/bootstrap.sh`, `src/config.rs`
(`axgf-cms releases`), `tests/console.rs`, `tests/bootstrap.rs`

## Context

On 21 October 2026 the operator asked for the administration section to be
rebuilt "cleanly", with two references (a glossy funnel chart and an IT
administration dashboard: KPI tiles, cards, a roles distribution, a security
and compliance panel) and four rooms:

- **AXGF bundle**: the anomalies and the conformity of the bundle that is
  loaded (or that none is), and the export of the current bundle raw
  (`.axgf`) or encrypted (`.axgf.gpg`, "gpg RSA 256, FIPS compliant");
- **Infrastructure**: the bugs and their mitigation plans described
  automatically, by modules inside the CMS, "like the QA tests you run";
  the infrastructure's health as green, warning and critical; the CMS
  version, whether a newer one exists, and a button to upgrade to it or to
  any published release; and the machine's storage (the CMS, the bundle,
  what is left of the partition) and computing;
- **Accounts** and **Connectivity**, recorded in
  [ADR 0012](0012-the-ways-in.md).

They wrote "five pages" and listed four. The fifth is the overview the
administration already opened on, which now leads to the other four.

## Decision

### 1. Five rooms behind one strip

`/admin` (overview), `/admin/bundle`, `/admin/infrastructure`,
`/admin/users` (Accounts, at its old address so that every link to it still
works) and `/admin/connectivity`, joined by a strip of five links that only
an administrator receives (`_admin_nav.html`): a contributor's dashboard
does not show doors that answer 403. The strip is counted apart from the
chrome budget (`tests/interactive_walk.rs`, `ADMIN_NAV`).

The overview opens on one tile per room — the bundle's verdict, the
infrastructure's, the version, the accounts, the ways in — for an
administrator only, since computing them reads the machine.

### 2. Findings, one shape for both rooms

A finding (`src/findings.rs`) has an area, a level (green, warning,
critical), a title, what was found and, when it is not green, the plan to
put it right, each a catalogue key with its arguments: every word of it is
translated, and the JSON twin speaks English to a monitor. The bundle's
conformity and the infrastructure's checks are both lists of findings,
grouped by area and worst first, drawn by the same include
(`_checks.html`). A critical one is open on arrival.

The bundle's (`src/conformity.rs`, AXGF 1.1): format version, manifest
counts, validation, payloads, the accounts' binding to the bundle, the
living in public, special categories, data quality, size; and the open
anomalies per quality rule, with a link to each group. An empty
installation says there is nothing to check and shows the way to set up.

### 3. The infrastructure's checks run on every visit

`src/selfcheck.rs` asks, every time the page opens, what the October 2026
QA pass asked by hand:

- **security**: how the page arrived (TLS, or plain on loopback, or plain
  and exposed), whether forwarded addresses can be trusted, posts from other
  sites refused in the last day, sign-in services against the public
  address;
- **access**: administrators (none is critical, one a warning), the
  emergency token, registration, failed sign-ins in the last day;
- **data**: the bundle, the cache and the backups, as `/health` sees them;
- **capacity**: free space, growth (the least-squares slope of the backup
  archives' sizes, and the days of room it leaves), memory against the
  service's cgroup ceiling;
- **release**: the version against the published list, the upgrade helper;
- **platform**: the sandbox (no-new-privileges, the seccomp filter), the
  clock.

Nothing here acts: a check that could fix what it found would change the
installation whenever somebody looks at it. The same checks are served as
JSON at `/admin/infrastructure.json` for a monitor, behind an
administrator's session.

Storage and computing (`src/host.rs`) are read from the process's own
view: the partition holding the data directory, what the site keeps there
(bundle, cache, backups, program, journal, accounts), the operating system,
CPU, load, memory, the cgroup's usage and ceiling. The production sandbox
(`ProcSubset=pid`) hides the machine's own figures, and the page says so
rather than guessing them.

### 4. The encrypted export: OpenPGP, honestly described

The operator asked for "gpg RSA 256 (FIPS compliant)". There is no such
thing as RSA-256: it would be broken in seconds, and FIPS 186-5 requires at
least 2048 bits. What is built is what GnuPG means by it:

- the archive is encrypted to one OpenPGP public key (`src/seal.rs`, rpgp),
  with AES-256 (SEIPD v1), and streamed, so a large bundle never sits in
  memory;
- RSA keys under 2048 bits are refused; RSA from 3072 bits and ECDH on the
  NIST curves are named as in the FIPS-approved set, Curve25519 as outside
  it — the page says which, without refusing what is not;
- a revoked, expired or sign-only key is refused before anything is
  exported, with a message saying why;
- the key may be kept (it is public) so the next export needs no paste;
- the plain copy made for the export is deleted once it is sealed.

Verified against GnuPG 2.4.4 for RSA-3072, P-384 and X25519. "FIPS
compliant" describes the algorithms; a FIPS-validated module is a claim
about a certified build this project does not ship, and the page does not
make it.

### 5. Upgrades: the page asks, a root helper acts

The web process runs unprivileged, offline, in a sandbox that cannot write
`/usr/local/bin`. That is the point of it, and a button that worked around
it would undo it. So:

- the page writes `upgrade.request` (a release tag, who asked, when) into
  the data directory, and only for a tag in the published list; an older
  version needs its tag typed a second time; one request at a time;
- `bootstrap.sh` installs a root helper (`/usr/local/lib/axgf-cms/
  upgrade-helper`, a path unit and a service for each of its two jobs, and
  a daily timer) that runs `bootstrap.sh --upgrade --version TAG` — the
  pre-upgrade backup, the checksum, the health check and the automatic
  rollback — and writes `upgrade.status` back, with the last lines of its
  output; the helper accepts a release tag and nothing else, so the most a
  compromised web process can ask for is another official release;
- the list of releases is fetched by `axgf-cms releases`, run by the
  helper as the service user, daily and when the page asks;
- the repository is private, so the helper reads a token from
  `/etc/axgf-cms/github-token` (root only); `bootstrap.sh` downloads
  through the API with it, and never prints it.

Without the helper the page shows the command to run as root instead.

### 6. Posts come from this site, and admin pages are not cached

Every POST, PUT, PATCH and DELETE whose `Origin` (else `Referer`) is not
this site is refused with 403 and counted (the cross-site check above).
An HTML page served to a signed-in reader is `no-store`.

## Consequences

- An installation upgraded to 0.5.0 by hand gets the helper with it; until
  then the button explains what to run.
- Production as installed by 0.4.3 has no outbound network
  (`IPAddressDeny=any`): the helper's fetch runs outside that sandbox, the
  web process's direct fetch only on a machine that allows it.
- Adding a check is a function in `selfcheck.rs`, its keys in eleven
  languages, and `tests/i18n.rs` (`every_key_the_console_names_in_rust_is_
  defined`) refuses one whose sentence is missing.

## Alternatives considered

- **Upgrading from the web process with sudo.** Rejected: it hands root to
  the most exposed process on the machine.
- **RSA-only exports, 256-bit as asked.** Rejected: it would be a broken
  key presented as compliant.
- **Running the checks in a timer and showing the last result.** Rejected
  for now: the page is opened rarely, the checks are cheap, and a result
  from the night before is the wrong answer to "what is the state now".
