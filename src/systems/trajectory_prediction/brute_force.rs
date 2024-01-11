/// Brute force incremental prediction does the following:
/// - steps the time
/// - updates each conic to end at that time
/// - checks for SOI changes at that time
/// - if SOI change found, creates a new conic at that time
/// This method is extremely reliable but very slow, so we can use it to test the reliability of faster methods
/// Also, it's not very accurate and doesn't make any attempt to refine past what if finds
/// But that doesn't matter much since we only really use it to verify other methods
mod incremental;