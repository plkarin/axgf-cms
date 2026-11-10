The AFG does more than answer: it can also keep the tree. Ask it in plain words what to add or correct. It proposes the changes on a card, and nothing changes before you press **Apply**. Each chapter below covers one kind of work, with requests to use as they are or to adapt.

## 1. Before you start

- **The archive must go with the project.** In the project's settings, tick “Give the AFG the family's archive in this project”. Without it the AFG knows neither the records nor their ids, and it will tell you so.
- **Your role decides what is possible.** A *viewer* account may ask the AFG, not change anything. A *contributor* changes the records of their branch. An *administrator* may also delete a person.
- **The “Proposed changes” card** lists in plain words what will be done. **Apply** does it; **Set aside** leaves it. Everything on a card is applied together, or nothing is.
- **Everything is recorded.** Each change goes into the history under your name, followed by “AFG”.

> Can you change the tree from this project? What does my account let me do?

## 2. Find and check before changing

Have the AFG check what the archive holds first: a correction is better made when you know what is already there.

> Who are the children of Pierre Dupont, born in 1890? Give me the ids of their records.

> Could Marie Durand have a duplicate? Compare the dates and places.

> Which records of the Dupont branch have neither a date of birth nor a place?

## 3. Add people

Say who the person is and who they belong to. A parent, brother or sister, partner or child named by a name nobody bears yet is created along with them.

> Add Jeanne Dupont, born on 3 March 1923 in Lyon, daughter of Pierre Dupont and Marie Durand.

> Add the three children of Jean Martin and Rose Petit: Paul (1950), Louise (1952) and André (about 1955), all born in Nantes.

> Add my great-grandfather Joseph Ngoma, born about 1880 in the Congo, died in 1952 in Kinshasa; his son is Albert Ngoma, who is already in the tree.

## 4. Correct a record

Given names, family name, nicknames, gender, biography, visibility: name the record and the correction.

> Jean Dupont's first name (born 1921) is really Jean-Baptiste. Correct it.

> Add the nickname “Granny Lou” to Louise Martin.

> Write a short biography of Paul Martin from what the archive says of him, and propose it.

> Rose Petit's record should be seen by members only, not by the public.

## 5. Births and deaths

A date is written 1890, 1890-06 or 1890-06-14; say “about” when it is approximate. A place the archive does not know is created.

> Pierre Dupont was born about 1890 in Saint-Étienne.

> Marie Durand died on 12 January 1975 in Lyon.

> Louise Martin's date of birth is wrong: it is 2 February 1952, not 1950.

> Remove André Martin's place of death: it is not certain.

## 6. Unions and children

A union joins one or two partners, with its kind (marriage, civil union, cohabitation, religious union, polygamous, unknown), and the date and place of the marriage when there is one.

> Pierre Dupont and Marie Durand married on 1 May 1920 in Lyon.

> Paul Martin is also a child of Jean Martin and Rose Petit.

> Louise is not this couple's daughter: take her out of the family of Jean Martin and Rose Petit.

> Joseph Ngoma had two wives, Marie Kanza and Thérèse Mbala: record each union on its own, as polygamous.

## 7. Life events

Baptism, burial, emigration, military service, census, graduation, will… with the other people there and their role (witness, godparent, officiant).

> Add Jeanne Dupont's baptism on 10 March 1923 in Lyon, with Paul Martin as godfather.

> Jean Martin emigrated to Canada in 1956.

> Record Pierre Dupont's military service from 1914 to 1918.

> The 1936 census shows the Dupont family in Lyon: add it for Pierre and Marie.

## 8. Work from a document

Attach a photograph of a record, a family book as a PDF, or a letter. The AFG reads the document, says what it finds there, then proposes the changes. Read the card again before applying: an old document is hard to read.

> Here is my great-grandparents' marriage record. Tell me what you read, then propose what to add to the tree.

> From this family book, add the children who are missing and their dates.

> Compare this letter with Joseph Ngoma's record: what does not agree?

## 9. Large work, step by step

A card may hold up to sixty operations. For a large piece of work, go in batches: one branch, one generation or one document at a time.

> Complete the places of birth of all of Pierre Dupont's descendants, one generation at a time, starting with his children.

> First propose a plan of work to put the Martin branch in order, without changing anything.

## 10. Delete and repair

A contributor may delete an event, take a child out of a family or undo a union. Deleting a person is for administrators: they leave their families and events, then their record is deleted. To go back, ask for the opposite operation; the history keeps track of everything.

> Delete the duplicate 1936 census event on Pierre Dupont's record.

> The record “Jean Dupond”, with no date and no family, was made by mistake: delete it.

> I applied the marriage of Pierre Dupont and Marie Durand by mistake: undo this union and delete the marriage event.

## 11. From ChatGPT, Claude or another AI (MCP)

The same operations are open to AI assistants elsewhere through the MCP, when an administrator has opened it (Administration → AFG → MCP).

1. On the **My account** page, section “Access tokens for an AI assistant”, make a token; tick “It may change the archive” if the assistant is to write. Copy it: it is shown once.
2. Connect your assistant:
   - **Claude Desktop**: install the community connector `ax-genealogy.mcpb` (folder `mcp/ax-genealogy` of the `axgf-cms` repository) and give it the site's address and the token.
   - **Claude Code**: `claude mcp add --transport http ax-genealogy https://your-site/mcp --header "Authorization: Bearer <token>"`
   - **ChatGPT** (connectors, developer mode) or **`claude.ai`** (custom connector): give the address with the token in it, `https://your-site/mcp/t/<token>`, with no authentication.
   - **Cursor** and the others: the address `https://your-site/mcp` and the header `Authorization: Bearer <token>`.
3. Revoke a token as soon as it is no longer used, or if it may have leaked.

> With the ax-genealogy tool, search for people named Dupont and tell me who has no date of death.

> With ax-genealogy, add Jeanne Dupont, born in 1923 in Lyon, daughter of Pierre Dupont and Marie Durand. Check it first with `check_operations`.

## 12. Good practice and limits

- **A source for every fact.** Say where the information comes from (a record, a memory, a document); the AFG can add it to the biography.
- **Say when in doubt.** Prefer “about 1890” to an invented date, and ask the AFG to point out what is uncertain.
- **The living are protected.** A living person is not public by default. Think before opening a record to the public.
- **What the AFG does not do yet**: attach photographs or documents to records, merge two records that describe the same person, write notes. These stay on the administration's pages, where merging keeps everything both records hold.

> Before you propose anything, list what seems uncertain to you in what I just told you.
