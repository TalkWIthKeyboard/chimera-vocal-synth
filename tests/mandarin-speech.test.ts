// Speak reads Mandarin through the wasm engine's own front-end list (speechPhrases), as Sing does:
// a path missing there is silently English in the browser while the cargo tests still pass.
import { describe, expect, test } from "bun:test";
import { initEngine } from "./harness.ts";

await initEngine();
const { speechPhrases, planPhrases } = await import("../src/wasm/engine.ts");

describe("Mandarin speech in the web app", () => {
  test("pinyin with tones comes back as phrases whose vowels carry their tone as bends", () => {
    const phrases = speechPhrases("nǐhǎo, wǒ ài nǐ?");
    expect(phrases?.map((p) => p.intonation)).toEqual(["statement", "question"]);
    const toned = phrases![0].segments.filter((s) => s.pitch?.length);
    expect(toned.length).toBe(2);
    expect(toned[0].pitch!.map((b) => b.at)).toEqual(["start", "end"]); // ní: 35
  });

  test("text with no Mandarin is null, so the caller speaks it as English", () => {
    for (const t of ["Hello world.", "My résumé is on the table."]) expect(speechPhrases(t)).toBeNull();
  });

  test("the planner draws the bends: a toned phrase plans pitch gestures, and nothing is added for English", () => {
    const phrases = speechPhrases("nǐhǎo.")!;
    const plan = planPhrases(phrases, { body: "male", t0: 0, f0: 110 });
    const pitch = plan.gestures.filter((g) => g.label === "pitch");
    expect(pitch.length).toBe(5); // ní: start and end; hǎo: start, middle, end (214)
  });
});
