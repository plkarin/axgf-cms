# ax-genealogy — the community connector

Connect ChatGPT, Claude or any assistant that speaks the
[Model Context Protocol](https://modelcontextprotocol.io) to a family's
genealogy on an ax·genealogy site (axgf-cms 1.14 or later). The assistant
then searches the family, reads records and — if its token may write — adds
people, corrects names and dates, records events and unions: every change
checked by the site with your account's rights, applied whole or not at all,
and journalled under your name followed by `MCP` and the token's name.

The site does all the work. This connector only carries the messages, for
the assistants that start a program rather than call an address.

## 1. On the site

1. An administrator opens the MCP: **Administration → AFG → MCP for AI
   assistants elsewhere**. It is closed until then. Whatever an account may
   read then leaves for the company whose assistant reads it, the living
   included: open it only if the family agrees.
2. On **My account → Access tokens for an AI assistant**, make a token. Tick
   *It may change the archive* if the assistant is to write. Copy it: it is
   shown once, and only its fingerprint is kept. It does what your account
   may, and nothing more; revoke it there as soon as it is no longer used.

The site's MCP is at `https://your-site/mcp`, with the token in
`Authorization: Bearer axgf_…`; or at `https://your-site/mcp/t/axgf_…` for an
assistant that takes an address and nothing else. Keep that address as
secret as the token.

## 2. In your assistant

**Claude Desktop** — download `ax-genealogy.mcpb` from your site
(**My account → Access tokens → Download the connector**) or from a release
of axgf-cms, open it, and give it the site's address and the token. Or, by
hand, in `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "ax-genealogy": {
      "command": "node",
      "args": ["/path/to/mcp/ax-genealogy/server.mjs"],
      "env": { "AXGF_URL": "https://your-site", "AXGF_TOKEN": "axgf_…" }
    }
  }
}
```

**Claude Code**:

```sh
claude mcp add --transport http ax-genealogy https://your-site/mcp \
  --header "Authorization: Bearer axgf_…"
```

**claude.ai** (Settings → Connectors → Add custom connector) and **ChatGPT**
(Settings → Connectors, developer mode → Create): the address with the token
in it, `https://your-site/mcp/t/axgf_…`, and no authentication. The site must
be reachable from the Internet over HTTPS.

**Cursor, Windsurf, VS Code and the others** — an HTTP server at
`https://your-site/mcp` with the header `Authorization: Bearer axgf_…`, or
this connector as a stdio server:

```json
{
  "mcpServers": {
    "ax-genealogy": {
      "url": "https://your-site/mcp",
      "headers": { "Authorization": "Bearer axgf_…" }
    }
  }
}
```

## 3. What it offers

| Tool | What it does |
| --- | --- |
| `search_people` | People whose name holds every word of the query |
| `get_person` | A record: names, birth, death, biography, parents, siblings, unions, events |
| `get_family` | A family: its kind, partners and children |
| `create_person` | A person, with parents, siblings and unions; a relative named by a name nobody bears is created too |
| `update_person` | Names, nicknames, gender, living, visibility, biography |
| `set_birth`, `set_death` | Date, approximation, place |
| `add_event`, `update_event`, `delete_event` | Baptism, burial, migration, census, military service… with others in it |
| `add_union`, `delete_union` | Unite one or two people, with a marriage's date and place; undo a union |
| `add_child`, `remove_child` | A child of parents or of a family |
| `delete_person` | Administrators only |
| `check_operations` | A batch checked without writing |
| `apply_operations` | A batch applied, all or nothing; `"ref": "new:1"` names what one creates for the next |
| `guide` | Every operation and its rules |

Ask, for example:

> With ax-genealogy, find the people named Dupont and tell me who has no date of death.

> With ax-genealogy, add Jeanne Dupont, born in 1923 in Lyon, daughter of Pierre Dupont and Marie Durand. Check it with check_operations first.

## 4. This connector

`server.mjs` reads the client's JSON-RPC messages on its standard input, one
a line, posts each to `AXGF_URL/mcp` with `AXGF_TOKEN`, and writes the site's
answers on its standard output. It needs Node 18 or later and nothing else.
`manifest.json` describes it as an MCP bundle for Claude Desktop.

Apache-2.0, as axgf-cms.
