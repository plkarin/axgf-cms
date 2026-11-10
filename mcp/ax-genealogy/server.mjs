#!/usr/bin/env node
// ax-genealogy: the community connector of an ax·genealogy site's MCP
// (docs/adr/0045 §5 of axgf-cms).
//
// An MCP server over stdio for the clients that start one: it reads the
// client's JSON-RPC messages, one a line, posts each to the site's /mcp with
// the account's token, and writes the site's answers back, one a line. The
// site does the work, with the account's rights and nothing more; this file
// only carries the messages. No dependency: Node 18 or later.
//
//   AXGF_URL    the site's address, https://family.example
//   AXGF_TOKEN  a token made on the account's page of the site (axgf_…)

import { createInterface } from "node:readline";

const site = (process.env.AXGF_URL || "").trim().replace(/\/+$/, "");
const token = (process.env.AXGF_TOKEN || "").trim();
const endpoint = site ? `${site}/mcp` : "";

function write(message) {
  process.stdout.write(JSON.stringify(message) + "\n");
}

function failure(id, message) {
  return { jsonrpc: "2.0", id, error: { code: -32000, message } };
}

async function forward(line) {
  let message;
  try {
    message = JSON.parse(line);
  } catch {
    write({ jsonrpc: "2.0", id: null, error: { code: -32700, message: "Parse error" } });
    return;
  }
  const id = Array.isArray(message) ? null : (message.id ?? null);
  if (!endpoint || !token) {
    if (id !== null) {
      write(failure(id, "Set AXGF_URL to the site's address and AXGF_TOKEN to a token made on the account's page of the site."));
    }
    return;
  }
  let response;
  try {
    response = await fetch(endpoint, {
      method: "POST",
      headers: {
        "content-type": "application/json",
        accept: "application/json, text/event-stream",
        authorization: `Bearer ${token}`,
      },
      body: line,
    });
  } catch (e) {
    if (id !== null) write(failure(id, `${endpoint} cannot be reached: ${e.message}`));
    return;
  }
  const text = await response.text();
  if (!text.trim()) return; // a notification, accepted
  let answer;
  try {
    answer = JSON.parse(text);
  } catch {
    if (id !== null) write(failure(id, `${endpoint} answered HTTP ${response.status}, not JSON-RPC.`));
    return;
  }
  // A refusal said before the message was read (a token unknown, the MCP
  // closed) names no id: it is the answer to this one.
  if (!Array.isArray(answer) && answer.id === null && id !== null) answer.id = id;
  write(answer);
}

// One message after another, so that answers come back in order.
let queue = Promise.resolve();
createInterface({ input: process.stdin, crlfDelay: Infinity }).on("line", (line) => {
  if (line.trim()) queue = queue.then(() => forward(line));
});
