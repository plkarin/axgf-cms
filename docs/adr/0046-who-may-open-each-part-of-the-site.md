# ADR 0046 — Who may open each part of the site

**Status:** accepted, v1.15.0 (2026-11-10) · **Builds on:** docs/adr/0012 (the rules of
entry), docs/adr/0001 (the lens), docs/adr/0002 §9 (the masthead's
controls) · **Applies to:** `src/site.rs` (`Doors`, `Level`, `Part`),
`src/doors.rs`, `src/routes/mod.rs` (`doors`), `src/render.rs`
(`Chrome::doors`), `src/routes/connectivity.rs`, `templates/base.html`,
`templates/home.html`, `templates/admin_connectivity.html`

## Context

On 10 November 2026, putting ax-genealogy.com online from a fresh
installation, the operator found every part of the site open to whoever
came: the river, the tree, the family's page, Impact, the import, with the
masthead's search, language, theme and density controls. Each record already
kept its own visibility (docs/adr/0001) — a visitor reads only what is
public — but the site itself said too much to a stranger: that there is an
archive, what it looks like, what it holds. The operator asked:

- everything closed by default, the family's page alone left open, and of
  the masthead's controls only "Sign in";
- then the administrator who set the site up opens its parts, according to
  the rights.

## Decision

### 1. A door for each part

The site's settings (`family.acl`, beside the rules of entry) gain `doors`:
for each part of the site, the narrowest role that may open it —
`visitors` (anyone), `members` (any account signed in), `contributors` (the
accounts that may write) or `admins`. The parts:

| Part | Addresses | By default |
| --- | --- | --- |
| family | `/about` | visitors |
| river | `/`, `/river/…` | members |
| tree | `/tree`, `/tree/…` | members |
| records | `/person/…`, `/search`, `/search.json`, `/notes/…`, `/document/…`, `/quality…` | members |
| impact | `/impact`, `/impact/…` | members |
| import | `/convert`, `/convert/…` | members |

An accounts file written before 1.15.0 holds no doors, and reads as the
defaults: an archive already online is closed the day it is upgraded, as
the operator asked. The sign-in, registration and the identity providers,
an account's own pages, the administration (which has its own roles), the
preferences, the static files and `/health` keep their own rules. The MCP
(docs/adr/0045) reads and writes the records, and follows their door: a
token whose account it does not admit is offered the guide alone.

### 2. The rule is the server's

One middleware, before any page is drawn, decides: a part the reader may
not open is not served to them. A visitor asking for the river is shown the
family's page, when that is open to them; for any other closed page, the
sign-in. A signed-in reader whose role is short of a part is told so, on a
page of its own (403). What a script or a file asks for — `/search.json`,
`/river/data`, a tree's panel, a document — is answered `403` and nothing
else. The archive's emblem (docs/adr/0022), drawn at the head of the
family's page, is served with that page.

### 3. The masthead offers what is open

The masthead shows a reader the sections they may open, and the row of
sections only when it would hold two at least; the person search with the
records; the reading preferences to whoever may read more than the family's
page. A visitor to a site left as installed sees the wordmark — then a link
back to the family's page — and "Sign in", nothing else. The family's page
offers the river and the tree, and a record's example, only to a reader who
may open them.

### 4. The administrator sets the doors

In the Connectivity room, beside the public address and the sign-in rules:
who may open each part, read at a glance, and changed in one form
(`POST /admin/connectivity/pages`); the change is logged with who made it.

## Consequences

- A new installation shows strangers its family's page and nothing more;
  opening the river or the records to the public is a choice, made once.
- An installation upgraded from an earlier release closes too: its
  administrator opens again what should be public.
- The tests that read pages as a visitor open every door first, as every
  page was open when they were written; the doors' own tests keep the
  defaults.
