# ADR 0047 — The sign-in jail

**Status:** accepted, v1.16.0 (2026-11-11) · **Builds on:** docs/adr/0012 (the ways in),
docs/adr/0046 (the site closed by default) · **Applies to:** `src/jail.rs`,
`src/session.rs` (`client_addr`, `client_key`, `MAX_ACCOUNT_FAILURES`),
`src/acl.rs` (`check_password`), `src/routes/admin.rs` (`login`),
`src/routes/entry.rs`, `deploy/proxy/nginx-axgf-cms.conf`, README ("Rate
limiting and jail")

## Context

On 10 November 2026, with ax-genealogy.com closed to visitors but for the
family's page and the sign-in (docs/adr/0046), the operator pointed at the
sign-in page — its password form and the emergency token beneath — and asked
for jail rules and rate limiting, so that one address could not hammer the
service; what the product cannot do itself, the README should say how to do.

What there was: a throttle counting failed sign-ins per client address and
per username, eight in a quarter of an hour, refusing the ninth until the
quarter was out. It had four holes.

- **An address came back every fifteen minutes, for ever**, and the journal
  never named it, so nothing outside the product could ban it.
- **Attempts sent at once passed together.** The count was checked before the
  password and raised after it; a hundred attempts in flight all saw a count
  of zero, and cost a hundred Argon2id hashes.
- **The hashes ran on the async workers**, so a burst of sign-ins held every
  page of the site for as long as it lasted.
- **The username throttle shut the account against its owner** as soon as
  one address had failed eight times: the address and the account ran out
  together.

## Decision

### 1. A jail for addresses, in memory

`src/jail.rs`. An address that fails [`MAX_FAILURES`] (8) sign-ins in
[`WINDOW`] (15 minutes), or offers [`MAX_TOKEN_FAILURES`] (3) wrong emergency
tokens, serves a term: 15 minutes, then 1 hour, 6 hours, 24 hours, and 24
hours again after that. A day ([`MEMORY`]) after its last term ends, the next
is the first again. A success gives back its own charge and nothing more: the
failures before it stay counted until their window ends, or signing in to one
account would wash away the guesses at another.

A term closes every way in that is a guess: the password, the emergency token
and the registration secret. Sign-in through a service (docs/adr/0012) is not
a guess and stays open. The refusal is a **429** with `Retry-After`, and the
page says how long in the reader's language, in minutes under the hour and in
hours past it, rounded up.

The jail lives in memory, as the throttle did. A restart empties it — which
is also the documented way for an operator who shut themselves out to get
back in.

### 2. Counted before it is compared

An attempt is charged when it begins and given back when it succeeds
(`Charge::succeeded`), or when it is refused before anything is compared
because the account it names is shut (`Charge::refund`): that may be the
account's owner, shut out by others' guesses, who must not jail their own
address by trying again — and hammering a shut account compares nothing.
Dropped without a word — the reader gave up mid-hash — an attempt stays
counted. Attempts in flight therefore count: the ninth of a burst is refused
before any hash runs. A wrong registration secret, compared before anything
costly, is counted after the comparison (`Jail::record_failure`).

### 3. Hashes off the async workers, a few at a time

`acl::check_password` runs Argon2id on the blocking pool behind a semaphore of
half the cores, between one and four. The slot is held by the hash itself, so
a reader who disconnects does not free it early. Sign-ins queue; pages go on.

### 4. The emergency token, counted from everywhere too

It opens an administrator's session and has no account to lock, so wrong
tokens are also counted from every address together:
[`MAX_TOKEN_FAILURES_ANYWHERE`] (20) in an hour, and the token is refused to
everyone for the next hour — a full hour from the twentieth, however the
guesses were spread — with a page that says so and that a restart opens it
again. A strong token makes the count moot; a token chosen by hand
is what it is for.

### 5. The account outlasts one address

The per-account limit ([`MAX_ACCOUNT_FAILURES`]) is three addresses' worth,
24 in the quarter hour: one address is jailed long before it can shut the
account against its owner, and guesses spread over many addresses still run
out. The account is charged before the hash as well
(`SessionStore::charge_account`), so a burst from many addresses at once is
compared 24 times, not once per attempt; the owner's success clears it.

### 6. An address is an address

`session::client_addr` takes `X-Forwarded-For`'s first entry only if it reads
as an address (with or without a port), and only from a peer that connects over
loopback (`127.0.0.1`, `::1`) — as the documented proxy does — else the peer's
address. It used to take any
text from anyone, so whoever could set the header chose their bucket, named
somebody else's address to have it jailed, and wrote what they liked into the
journal. Bound wider with no proxy, the application now counts each client at
its own address. An IPv4 address mapped into IPv6 is the IPv4 address.
An IPv6 client is counted with its whole /64, which it can roam at will.

This machine's own address (loopback) never serves more than the first term:
behind the documented proxy every visitor arrives from it, and an attempt that
carries no other address is most likely every visitor at once, through a
proxy that does not pass `X-Forwarded-For`. The journal says so when it
happens.

### 7. Written in the journal, once a term, with the address in the sentence

```
sign-in jail: 203.0.113.7 shut out for 900 s, after 8 failed sign-ins (term 1)
a sign-in failed from 203.0.113.7
a wrong emergency admin token was offered from 203.0.113.7
sign-in jail: the emergency token is refused to everyone for 3600 s, …
```

Under systemd the fields go to the journal as fields, but a
fail2ban filter reads the message, so the address is in the sentence. What
happens during a term is not written at `info` or above: an address that
keeps knocking would otherwise fill the journal for it.

### 8. At the door: the proxy's share

The application cannot stop a flood from reaching it. The shipped nginx file
counts sign-in posts per address — ten a minute with a burst of ten, a 429
past them, the page itself never refused — and sets the headers the
application reads once at the server, which every location inherits. The
README gives a fail2ban filter that bans at the firewall what the application
jailed, and what to do when the proxy is on another machine (ban where the
connection arrives). Caddy has no rate limiting without a plugin: there, the
jail and fail2ban do the work.

## Consequences

- Eight wrong passwords from one address shut that address out for a quarter
  of an hour, and longer each time it comes back the same day; the owner of
  the account signs in from elsewhere as before.
- Every term is one line in the journal that names the address, which fail2ban
  can turn into a ban at the firewall or at the proxy.
- The jail is only as right as the address the proxy reports. Behind a chain
  of proxies, each must pass the visitor's address on: the README says how to
  check that the journal names your own address when you fail on purpose.
- A restart empties the jail. That is the way out for an operator, and also
  for an attacker able to restart the service — who has no need of the jail
  to get in.
