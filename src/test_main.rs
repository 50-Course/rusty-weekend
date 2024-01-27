// We are going TDD now
// or maybe not, the fact is, we are testing for behaviours - this way makes me feel more comfortable


// test the behaviour of our greet function from main.rs
#[test]
fn test_greet() {
    assert_ne!("Hello, world!", greet());
}
