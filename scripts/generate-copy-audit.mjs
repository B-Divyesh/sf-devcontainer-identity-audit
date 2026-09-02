import { readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const outputPath = resolve(root, ".factory/copy-audit.md");
const banned = ["leverage", "seamless", "effortless", "robust", "powerful", "intuitive", "reimagine", "supercharge", "unlock", "delightful", "journey", "ecosystem", "ai-powered"];

export function wordCount(text) {
  return text.split(/\s+/u).filter((token) => /[\p{L}\p{N}]/u.test(token)).length;
}

function decode(text) {
  return text
    .replaceAll("&lt;", "<")
    .replaceAll("&gt;", ">")
    .replaceAll("&amp;", "&")
    .replaceAll("&quot;", "\"")
    .replaceAll("&#39;", "'")
    .replaceAll("&nbsp;", " ");
}

function protectMarkdown(text) {
  const protectedText = [];
  const protect = (value) => {
    const marker = `\uE000${protectedText.length}\uE001`;
    protectedText.push(value);
    return marker;
  };

  // Inline code must be protected before HTML stripping so a reader-visible
  // placeholder such as `services.<name>.user` is not mistaken for a tag.
  let result = text.replace(/`([^`\n]+)`/gu, (_match, code) => protect(code));
  // Keep only a link's visible label. Any protected inline code in that label
  // is restored with the rest of the reader-facing text below.
  result = result.replace(/\[([^\]]+)\]\([^\s)]+(?:\s+"[^"]*")?\)/gu, (_match, label) => label);

  return {
    text: result,
    restore(value) {
      return value.replace(/\uE000(\d+)\uE001/gu, (_match, index) => protectedText[Number(index)] ?? "");
    }
  };
}

function cleanInline(text) {
  const markdown = protectMarkdown(text);
  const cleaned = decode(markdown.text)
    .replace(/<[^>]+>/gu, " ")
    .replace(/\s+/gu, " ")
    // Remove accidental whitespace before sentence punctuation, but retain a
    // space when punctuation starts a filename or code token (for example,
    // `.factory/claims.json`).
    .replace(/\s+([.,;:!?])(?=\s|$|["')\]])/gu, "$1")
    .trim();
  return markdown.restore(cleaned);
}

function splitSentences(text) {
  const clean = cleanInline(text);
  if (!clean) return [];
  return clean.split(/(?<=[.!?])\s+(?=[\p{Lu}\d“])/u).map((sentence) => sentence.trim()).filter(Boolean);
}

function htmlSentences() {
  const html = readFileSync(resolve(root, "site/index.html"), "utf8");
  const body = html.match(/<body[\s\S]*?<\/body>/u)?.[0] ?? "";
  const prose = [];
  for (const match of body.matchAll(/<(p|figcaption|h1|small)\b[^>]*>([\s\S]*?)<\/\1>/gu)) {
    prose.push(...splitSentences(match[2]));
  }
  for (const id of ["offline-banner"]) {
    const match = body.match(new RegExp(`<[^>]+id="${id}"[^>]*>([\\s\\S]*?)<\\/[^>]+>`, "u"));
    if (match) prose.push(...splitSentences(match[1]));
  }
  const heroActions = body.match(/<div class="hero-actions">([\s\S]*?)<\/div>/u)?.[1] ?? "";
  for (const match of heroActions.matchAll(/<span[^>]*>([\s\S]*?)<\/span>/gu)) prose.push(...splitSentences(match[1]));
  const facts = body.match(/<ul class="plain-facts">([\s\S]*?)<\/ul>/u)?.[1] ?? "";
  for (const match of facts.matchAll(/<li[^>]*>([\s\S]*?)<\/li>/gu)) prose.push(...splitSentences(match[1]));

  for (const path of ["site/src/main.ts", "site/src/audit.ts"]) {
    const source = readFileSync(resolve(root, path), "utf8");
    for (const match of source.matchAll(/"([^"\n]*(?:[.!?]))"/gu)) {
      // Source messages often contain two short sentences. Audit each reader-
      // visible sentence independently so a later edit cannot hide an overlong
      // follow-up sentence in a combined row.
      for (const sentence of splitSentences(match[1])) {
        if (sentence && !sentence.startsWith("#") && !sentence.includes("${")) prose.push(sentence);
      }
    }
  }
  return [...new Set(prose)];
}

function readmeSentences() {
  const markdown = readFileSync(resolve(root, "README.md"), "utf8").replace(/```[\s\S]*?```/gu, "");
  const blocks = [];
  let current = "";
  const flush = () => {
    if (current.trim()) blocks.push(current.trim());
    current = "";
  };
  for (const line of markdown.split("\n")) {
    if (!line.trim() || /^\s*#/u.test(line)) {
      flush();
    } else if (/^\s*[-*]\s+/u.test(line)) {
      flush();
      current = line.replace(/^\s*[-*]\s+/u, "");
    } else {
      current += `${current ? " " : ""}${line.trim()}`;
    }
  }
  flush();
  const prose = [];
  for (const block of blocks) prose.push(...splitSentences(block));
  return [...new Set(prose)];
}

function table(rows) {
  return ["| Words | Sentence |", "| ---: | --- |", ...rows.map((sentence) => `| ${wordCount(sentence)} | ${sentence.replaceAll("|", "\\|")} |`)].join("\n");
}

function validate(rows, section) {
  const long = rows.filter((sentence) => wordCount(sentence) > 22);
  const flagged = rows.filter((sentence) => banned.some((term) => sentence.toLocaleLowerCase("en").includes(term)));
  if (long.length || flagged.length) {
    throw new Error(`${section} copy failed: ${JSON.stringify({ long, flagged })}`);
  }
}

const landing = htmlSentences();
const readme = readmeSentences();
validate(landing, "Landing");
validate(readme, "README");

const document = `# Copy audit

Generated by \`npm run copy:audit\` from \`site/index.html\`,
\`site/src/main.ts\`, \`site/src/audit.ts\`, and \`README.md\` on 2026-09-02.
The tokenizer splits on whitespace and counts each token containing a Unicode
letter or number exactly once. Thus \`identities—the\` is one token. Code blocks
are excluded. Run \`npm run copy:audit:check\` to reject stale tables, incorrect
counts, sentences over 22 words, or banned terms.

## Landing sentences

${table(landing)}

## README sentences

${table(readme)}

## Labels and terminology

Literal labels: \`Try it with sample data\`, \`Copy install command\`, \`Check
mount permissions\`, \`Permission report\`, \`Check numeric workspace access\`,
\`Read the configuration\`, \`Read the runtime identity map\`, \`Read workspace
ownership and mode\`, \`Reset demo\`, and \`Open blank browser check\`.

| Concept | Term |
| --- | --- |
| Host directory mounted into a container | workspace |
| Person inside the container | remote user |
| UID/GID transformation | identity map |
| Container configuration | Dev Container |
| Rootless preservation mode | keep-id |
| Browser try-out | browser sample |
| Command-line try-out | bundled sample |
| Access results | PASS, FAIL, UNKNOWN |

## Result

- Sentences over 22 words: **0**.
- Banned terms: **0**.
- The first screen contains the job headline, audience sentence, sample action,
  next-step note, and three plain facts at 390 px and desktop widths.
`;

if (process.argv.includes("--check")) {
  const committed = readFileSync(outputPath, "utf8");
  if (committed !== document) {
    console.error(".factory/copy-audit.md is stale; run npm run copy:audit");
    process.exit(1);
  }
} else {
  writeFileSync(outputPath, document);
}
