//! The ends of an utterance, for a language that wants them held: `final_hold` keeps the last vowel
//! or nasal past its plan (a final stop or fricative lets go on time, and so does anything when it
//! is unset or the note is open-ended), and `breath_at` starts the breath later.
use chimera::control::score::Gesture;
use chimera::phonetics::planner::{PlanOptions, plan_utterance};
use chimera::phonetics::Segment;

fn plan(segs: &[Segment], o: PlanOptions) -> Vec<Gesture> {
    plan_utterance(segs, &PlanOptions { body: "male".into(), f0: Some(110.0), ..o }).gestures
}
fn ends_of(gs: &[Gesture], label: &str) -> Vec<f64> {
    gs.iter().filter(|g| g.label.as_ref().is_some_and(|l| l.as_str() == label)).map(|g| g.t1).collect()
}
/// every gesture's label and times (a gesture's targets hold NaN for what it leaves alone, so two
/// plans don't compare equal as a whole)
fn timing(gs: &[Gesture]) -> Vec<(String, f64, f64)> {
    gs.iter().map(|g| (g.label.as_ref().map(|l| l.as_str().to_string()).unwrap_or_default(), g.t0, g.t1)).collect()
}
fn segs(ipa: &[&str]) -> Vec<Segment> {
    ipa.iter().map(|s| Segment::new(s)).collect()
}
const HOLD: f64 = 0.12;

#[test]
fn the_last_vowel_is_held_and_one_before_it_is_not() {
    let (a, b) = (plan(&segs(&["m", "a", "t", "a"]), PlanOptions::default()), plan(&segs(&["m", "a", "t", "a"]), PlanOptions { final_hold: Some(HOLD), ..Default::default() }));
    let (va, vb) = (ends_of(&a, "a"), ends_of(&b, "a"));
    assert_eq!(va.len(), 2);
    assert!((vb[0] - va[0]).abs() < 1e-9, "the first a lets go on time");
    assert!((vb[1] - va[1] - HOLD).abs() < 1e-9, "the last a is held");
}

#[test]
fn a_final_nasal_keeps_its_closure_and_its_velum() {
    let s = segs(&["t", "a", "n"]);
    let (a, b) = (plan(&s, PlanOptions::default()), plan(&s, PlanOptions { final_hold: Some(HOLD), ..Default::default() }));
    for label in ["n", "velum"] {
        assert!((ends_of(&b, label)[0] - ends_of(&a, label)[0] - HOLD).abs() < 1e-9, "{label}");
    }
    assert!((ends_of(&b, "a")[0] - ends_of(&a, "a")[0]).abs() < 1e-9, "the vowel before it is not held");
}

#[test]
fn a_final_stop_lets_go_on_time() {
    let s = segs(&["a", "t"]);
    let held = plan(&s, PlanOptions { final_hold: Some(HOLD), ..Default::default() });
    assert_eq!(timing(&held), timing(&plan(&s, PlanOptions::default())));
}

#[test]
fn the_breath_starts_when_asked_and_at_40_ms_otherwise() {
    let s = segs(&["m", "a"]);
    let start = |o: PlanOptions| plan(&s, o).iter().find(|g| g.label.as_ref().is_some_and(|l| l.as_str() == "breath")).unwrap().t0;
    assert!((start(PlanOptions::default()) - 0.04).abs() < 1e-9);
    assert!((start(PlanOptions { breath_at: Some(0.12), ..Default::default() }) - 0.12).abs() < 1e-9);
    assert!((start(PlanOptions { t0: 0.5, breath_at: Some(0.12), ..Default::default() }) - 0.62).abs() < 1e-9, "from the utterance's start");
}
