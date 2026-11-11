// Regenerates crates/hydra-shared/src/char_map.rs from src/shared/char-map.ts.
// Usage: node rust/scripts/gen-char-map.mjs
import fs from "node:fs";
import path from "node:path";

const root = path.resolve(import.meta.dirname, "../..");
const source = fs.readFileSync(path.join(root, "src/shared/char-map.ts"), "utf8");
const entries = [...source.matchAll(/^\s{2}(.+?):\s*"(.+?)",$/gm)].map((match) => [
  match[1],
  match[2],
]);

const escape = (value) =>
  [...value]
    .map((character) => {
      const codePoint = character.codePointAt(0);
      if (codePoint >= 0x20 && codePoint < 0x7f && character !== "\\" && character !== '"') {
        return character;
      }
      return `\\u{${codePoint.toString(16)}}`;
    })
    .join("");

const single = [];
const multi = [];
const seen = new Set();

for (const [key, value] of entries) {
  if (seen.has(key)) continue;
  seen.add(key);

  const characters = [...key];
  if (characters.length === 1) single.push([characters[0].codePointAt(0), value]);
  else multi.push([key, value]);
}

multi.sort((first, second) => [...second[0]].length - [...first[0]].length);

let output = "// @generated from src/shared/char-map.ts - do not edit by hand.\n";
output += "// Regenerate with: node rust/scripts/gen-char-map.mjs\n\n";
output += "pub static SINGLE_CHAR_MAP: &[(char, &str)] = &[\n";
for (const [codePoint, value] of single) {
  output += `    ('\\u{${codePoint.toString(16)}}', "${escape(value)}"),\n`;
}
output += "];\n\n";
output += "/// Multi-codepoint keys, longest first so composed sequences win over parts.\n";
output += "pub static MULTI_CHAR_MAP: &[(&str, &str)] = &[\n";
for (const [key, value] of multi) {
  output += `    ("${escape(key)}", "${escape(value)}"),\n`;
}
output += "];\n";

fs.writeFileSync(path.join(root, "rust/crates/hydra-shared/src/char_map.rs"), output);
console.log(
  `char-map: ${entries.length} entries (${single.length} single, ${multi.length} multi)`,
);
