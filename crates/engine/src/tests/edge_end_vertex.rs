//! Booleans whose intersection curves end on an edge that already has a vertex there.

use super::*;
use solidcraft_kernel::BooleanStrategy;

/// Seeds 34 and 39 of the robustness fuzzer end an intersection curve on the boundary edge of a face
/// where an earlier curve had put a vertex. The polygon store, made from coarse meshes, puts the new
/// vertex a hair inside the edge while the geometry puts it at the edge's end, and an edge cannot be
/// split at its own end: the plain run gives up. The booleans were solved only by retries, seed 34 at
/// a refined tolerance and seed 39 by nudging the tool (the last resort, which moves the result). The
/// run that puts the vertex on the edge's end solves seed 34 at once and seed 39 at the refined
/// tolerance.
#[test]
fn a_vertex_a_hair_from_an_edge_end_is_put_on_it_not_dodged_by_retries() {
    let r = crate::fuzz::run(34, 8);
    assert_eq!(r.outcome.kind, "ok", "seed 34: {:?}", r.outcome);
    assert!(r.boolean_fallbacks.is_empty(), "seed 34 needed a retry: {:?}", r.boolean_fallbacks);
    let r = crate::fuzz::run(39, 8);
    assert_eq!(r.outcome.kind, "ok", "seed 39: {:?}", r.outcome);
    assert!(!r.boolean_fallbacks.contains_key(&BooleanStrategy::ToolNudge), "seed 39 needed the tool nudged: {:?}", r.boolean_fallbacks);
}
