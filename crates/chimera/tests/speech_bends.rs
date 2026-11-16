//! Speech draws a vowel's pitch bends (`Segment.pitch`, how a Mandarin tone is written) around the
//! intonation baseline: the bends are that vowel's pitch, so it takes no English stress accent and
//! no final fall. A vowel without bends is planned as before.
use chimera::control::score::{ControlKey, Gesture};
use chimera::phonetics::planner::{PlanOptions, plan_utterance};
use chimera::phonetics::{Bend, Edge, Intonation, Segment};

const F0: f64 = 100.0;

fn bend(at: Edge, st: f64) -> Bend {
    Bend { at, len: 0.2, max: 1.0, st }
}
fn vowel(ipa: &str, pitch: Vec<Bend>) -> Segment {
    Segment { dur: Some(0.3), stress: Some(1), pitch, ..Segment::new(ipa) }
}
fn pitch_gestures(segs: &[Segment], intonation: Intonation) -> Vec<Gesture> {
    let o = PlanOptions { body: "male".into(), f0: Some(F0), intonation: Some(intonation), ..Default::default() };
    plan_utterance(segs, &o).gestures.into_iter().filter(|g| g.label.as_ref().is_some_and(|l| l.as_str() == "pitch")).collect()
}
fn f0(g: &Gesture) -> f64 {
    g.targets.get(ControlKey::F0).unwrap()
}

#[test]
fn each_bend_is_a_target_around_the_baseline_in_its_part_of_the_vowel() {
    // a falling tone: high at the start, low at the end
    let g = pitch_gestures(&[Segment::new("m"), vowel("a", vec![bend(Edge::Start, 5.0), bend(Edge::End, -5.0)])], Intonation::Statement);
    assert_eq!(g.len(), 2);
    assert!(f0(&g[0]) > F0 * 2f64.powf(4.0 / 12.0) && f0(&g[1]) < F0 * 2f64.powf(-4.0 / 12.0), "{} {}", f0(&g[0]), f0(&g[1]));
    assert!(g[0].t1 < g[1].t0, "the start bend comes first, over its fifth of the vowel");
}

#[test]
fn a_toned_vowel_takes_no_stress_accent_and_no_final_fall() {
    let flat = || vec![Bend { at: Edge::Middle, len: 1.0, max: 1.0, st: 0.0 }];
    let toned = pitch_gestures(&[Segment::new("m"), vowel("a", flat())], Intonation::Statement);
    assert_eq!(toned.len(), 1);
    // the baseline declines ~1.7 semitones over the phrase, so a short phrase's end is a little under F0
    assert!(f0(&toned[0]) > F0 * 0.95 && f0(&toned[0]) <= F0, "the tone sits on the baseline: {}", f0(&toned[0]));
    // the same vowel without bends is English's: the statement falls to 0.82 of the baseline
    let english = pitch_gestures(&[Segment::new("m"), vowel("a", vec![])], Intonation::Statement);
    assert!(f0(english.last().unwrap()) < F0 * 0.9);
}

#[test]
fn a_question_lifts_the_last_toned_vowel() {
    let flat = || vec![Bend { at: Edge::Middle, len: 1.0, max: 1.0, st: 0.0 }];
    let g = pitch_gestures(&[Segment::new("m"), vowel("a", flat())], Intonation::Question);
    assert!(f0(&g[0]) > F0 * 2f64.powf(2.0 / 12.0), "a question rises ~3 semitones: {}", f0(&g[0]));
}

#[test]
fn a_diphthong_is_one_syllable_with_one_set_of_bends() {
    let g = pitch_gestures(&[Segment::new("m"), vowel("aɪ", vec![bend(Edge::Start, 5.0), bend(Edge::End, -5.0)])], Intonation::Statement);
    assert_eq!(g.len(), 2, "its second target adds none of its own");
}
