# ADR 0049 — What search engines see

**Status:** proposed (1.17.1) · **Builds on:** docs/adr/0046 (the site closed
by default), docs/adr/0048 §7 (links that survive the sign-in) ·
**Applies to:** `src/routes/mod.rs` (`search_engines`, `is_reading_page`),
`src/routes/public.rs` (`river`, `document_view`), `src/site.rs`
(`normalise_base`), `tests/search_engines.rs`, README, docs/DEPLOY.md,
docs/REFERENCE.md

## Context

On 16 November 2026 Google Search Console told the operator that pages of
ax-genealogy.com were left out of its index as "Duplicate without
user-selected canonical". The site had been open, then closed to visitors
(1.15.0); since 1.17.0 a closed page sends a visitor to the sign-in with the
page it asked for, `/admin/login?next=/person/…`. A crawler revisiting the
site's old addresses — every record, the tree, Impact — found behind each
one the same sign-in under an address of its own: as many copies of one
page as the site had pages, none of them naming an original. Until then the
product said nothing to search engines at all: no canonical address, no
`noindex`, no `robots.txt`.

Google's own words: a page so marked "is a duplicate of another page,
although it doesn't indicate a preferred canonical page"; it is not an
error, and the remedy is to name the canonical page or to keep the page out
of the index. A sign-in has no place in search results.

## Decision

1. **A page a reader reads names its canonical address.** The river (`/`),
   the family's page (`/about`), the tree (`/tree`), Impact (`/impact`), one
   record (`/person/<id>`), one person's Impact (`/impact/<id>`) and one note
   (`/notes/<id>`), served whole (2xx) as HTML to a GET or HEAD, carry
   `Link: <public address + path>; rel="canonical"`. The query that brought
   them there — which person the river is centred on, which tab of a record,
   the depth of the tree — is left out: it is one page seen another way.
2. **Every other page says `noindex`**, in `X-Robots-Tag`: the sign-in in
   every form, the registration, the pages of mail, the search, the tools,
   the settings, a fragment (the tree's panel, a note's frame), the
   administration, and every error page. A redirect and what is not HTML —
   a picture, a file, JSON, a stylesheet, an error answered in plain text —
   carry neither.
3. **One page, one address.** With nobody in the archive yet, `/` shows the
   family's page and names `/about` as its canonical address, so that the
   page served under both is kept once; a file opened through its viewer
   (`/document/<id>/view`) that serves its stored bytes names
   `/document/<id>/raw`, where the same bytes are, as its canonical
   address.
4. **The canonical address is built on the public address only**, never on
   the request's `Host`, which anyone can forge. Without a public address
   none is named, and the reading pages say nothing. A public address in
   letters beyond ASCII is written as links and headers take it: its name
   in punycode, its path percent-encoded.
5. **No `robots.txt` rule.** A crawler must be able to read a page to see
   its `noindex`; blocking it in `robots.txt` would hide exactly that. A
   missing `robots.txt` lets everything be crawled, which is what is meant.
6. **One place.** A layer over every route decides, from the path and the
   response; no handler or template has to remember it, and a page added
   later is kept out of the index until it is listed as a reading page.

Both headers are HTTP headers rather than elements of the page: Google
reads a canonical address "for a document supported by Search" from the
header, and "any rule that can be used in a robots meta tag can also be
specified as an X-Robots-Tag".

## Consequences

- The sign-in's copies leave the "duplicate" list and become "excluded by
  noindex", which is what they are, once Google crawls them again; Search
  Console's "Validate fix" asks it to.
- On a site opened to visitors, each record is one page in the index,
  whatever tab or link led to it; a tab's own content is not indexed apart.
- Whether a family's open pages should be in search engines at all is still
  the doors' question (docs/adr/0046): what is closed to visitors is never
  seen by a crawler. A setting to keep even the open pages out is left for
  later, if a family asks.
