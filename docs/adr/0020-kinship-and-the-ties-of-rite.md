# ADR 0020 — Kinship: how two people are related, every relative, and the ties of rite

**Status:** accepted (2026-11-01) · **Builds on:** ADR 0019 (the tree page),
AXGF 1.1 §5.13 (`lineage`, Link `relation`) · **Applies to:**
`src/kinship.rs`, `src/kinship_text.rs`, `src/rites.rs`, `src/person.rs`,
`src/chart.rs`, `src/tree.rs`, `src/routes/public.rs`,
`src/routes/editors.rs`, `templates/_kinship.html`,
`templates/_person_detail.html`, `templates/_panel.html`,
`templates/_tree_canvas.html`, `templates/tree.html`,
`templates/_macros.html`, `static/app.css`, `static/css/tokens.css`,
`static/tree.js`, `locales/*.ftl`

## Context

On 1 November 2026 the operator asked for two tools, on a person's record and
wherever a person is shown. The first chooses another person of the family
and says how the two are related: a third cousin, a nephew, a child. Every
relationship must be named. The second lists every relative.

They also asked for new relationships. A godmother or a godfather ties a
person to another family, and that tie must be drawn dotted, apart from the
nuclear family. The other affiliations each culture makes by its rites and
customs belong with it.

The records hold only near facts: a child of a union, a partner in one, a
godmother. A third cousin is a path through those facts, and so is a
sister-in-law or the godson of an uncle.

## Decision

1. **The relationship is computed, never stored** (`src/kinship.rs`). The
   family's structure is read once per request, and every way two people are
   related is found:
   - **By descent.** A is `up` generations below the ancestors the two share
     and B `down`. (1, 1) is a brother or sister, (2, 1) an uncle, (3, 3) a
     second cousin, and (3, 2) a first cousin a generation older.
     - Two paths meeting only at their top give the closest shared
       ancestors: a couple when both paths come down through one of its
       unions, which is full kinship; one ancestor when they come down
       through two unions, which is half.
     - Birth and adoption are descent, and a path through an adoption says
       so.
     - A family that married into itself gives two paths, and both are
       kept.
   - **By marriage.** A partner; a relative of A's partner; the partner of
     A's relative. Both ends by marriage are named only when no nearer way
     exists.
   - **By care.** A step-parent, a foster parent, a guardian the records
     name as such (`lineage`).
   - **By rite or custom** (§3).
   - **Otherwise**, if the records connect the two at all, the shortest
     chain of facts between them.
2. **Words that agree** (`src/kinship_text.rs`).
   - **Every word is a Fluent message** in the fourteen languages. Each
     receives every fact a language may make it agree with:
     - B's sex;
     - the side of the family (Arabic عم and خال, Swahili *baba mdogo*
       and *mjomba*, Chinese 外);
     - an elder or a younger (Japanese 兄 and 弟, 伯父 and 叔父);
     - one male line (Chinese 堂 and 表);
     - the parent a half-sibling shares (Russian единокровный and
       единоутробный);
     - the sibling a nephew comes from (Polish *bratanek* and
       *siostrzeniec*).
   - **A language uses what it needs**, and no word is assembled in Rust
     out of a language's grammar. Where one word cannot say it all, a
     language puts two together in its own order, through one message that
     receives both: "first cousin of the father", "son of a second cousin",
     "wife of a first cousin".
   - **The degree** is counted as civil law counts it: the generations up
     to the shared ancestor and down again.
3. **Ties of rite and custom, stored in AXGF 1.1** (`src/rites.rs`). The
   catalogue holds 27 rites in ten traditions:
   - **Christian:** godparenthood, baptism, confirmation, wedding sponsors.
   - **Orthodox:** *kum*, *koumbaros*.
   - **Hispanic and Filipino:** *padrino*, *compadre*, *ninong*.
   - **Jewish:** *sandek*, *kvatter*.
   - **Islamic:** milk parent and milk sibling, *kafala*.
   - **South Asian:** *rakhi*, *guru*.
   - **East Asian:** *gān qīn*, sworn siblings, the name-giver, *nakōdo*.
   - **African:** namesake (*ndoyi*, *somo*), initiation, blood pact.
   - **Indigenous and Pacific:** *hānai*, *hunka*.
   - **Civil:** civil godparenthood, guardianship.

   Each is stored as a 1.1 Link: the closed vocabulary's nearest
   `relation` (`godparent`, `guardian`, `mentor`, `witness`, `other`) and a
   `category`. The rite itself goes in this application's extension,
   `extensions["axgf-cms:kinship/v1"] = {"rite": …, "sponsor": "from"|"to"}`
   (1.0 §10.4). A bundle stays valid 1.1, and any other reader sees a
   Link with its words. A plain `godparent` or `guardian` Link with no
   extension is read as godparenthood or guardianship. The tradition's own
   word is shown untranslated beside the reader's: «кум · кума»,
   «أخ من الرضاعة», «ndoyi · somo».
4. **Drawn apart from the family.**
   - **On the tree**, a tie of rite between two people both drawn is a
     curve, in pairs of dots, in a colour of its own (`--kin-rite`, in
     every theme, never the accent). It never runs along a family's
     straight lines. The legend shows it.
   - **On the record**, the Family tab gives the ties of rite a section of
     their own, "Rite and custom". Each tie is a card in a dotted frame of
     that colour, with the rite, the tradition and what it is.
   - **The open card** names a person's ties in the same dotted frame.
5. **The two tools, wherever a person is.**
   - **The Kinship tab** is on the record page and in the record beside the
     river.
     - *"How are they related?"* chooses any relative from a list grouped
       as the relatives are. It answers with every way they are related:
       the word in large type, whose relative they are, the shared
       ancestors, and the path step by step, from this person to the other,
       each person named (or said to be hidden) and each step worded
       ("father", "son", "wife").
     - *"Every relative"* lists them by group: partners, ancestors,
       descendants, brothers and sisters, uncles and aunts, nephews and
       nieces, cousins, by marriage, the stepfamily, fostering and
       guardianship, rite and custom, and anyone else the records connect.
       Each line is a name, the years, the word and the way to the link.
   - **On the tree**, each open card says what the person is to the one in
     the middle ("first cousin of Lucien"). The side panel says it too,
     with the way to the link step by step.
6. **The editor.** The link form offers the rites by tradition, as what this
   person is to the other ("Baptism: godparent", "Baptism: godchild").
   Choosing one sets the relation, the category and the extension, and
   writes the words in the editor's language when the form leaves them
   empty.

## Consequences

- **Visibility.** The paths run through the family's structure whatever a
  reader may see, as the tree does. A person this reader may not see keeps
  their place in a path and is named by nobody ("a person you may not see").
  Only people the reader may see are offered or listed. A Link has its own
  visibility, and a hidden one is not followed.
- **Computed only when asked.** The Kinship tab walks the whole family when
  it is opened. Its tab carries no count. The tree computes one relationship
  per card it draws.
- **A tie of rite outside the chart** is not drawn as a line. The open card
  and the record say it.
- **A format proposal.** The format has no word for most of these rites.
  axgf-spec's proposals/0002 asks for them in AXGF 1.2. Until then the
  extension carries them, and a 1.2 reader can map it back.
- **Read mode.** Every word of kinship names something the records hold,
  and read mode allows them (`tests/read_mode.rs`). The tab never says
  "nothing here": a person with no relative to show has no list.
- **The translations** of the twelve languages beyond English and French are
  first drafts and need a native reader, the kinship words most of all.
