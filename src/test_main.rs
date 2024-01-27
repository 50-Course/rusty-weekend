// We are going TDD now
// or maybe not, the fact is, we are testing for behaviours - this way makes me feel more comfortable


// test the behaviour of our greet function from main.rs
#[test]
fn test_greet() {
    assert_ne!("Hello, world!", greet());
}

#[cfg(test)]
fn test_greetings_should_fail_if_no_args_is_passed() {

    let expected = "Hello, world!";
    let actual = greet();

    assert_eq!(expected, actual, "Greeted message is not as expected");
}

#[test]
fn test_greetings_should_fail_if_incorrect_args_is_passed() {

    let arg1 = "Somebody";
    let arg2 = "Somebody else";

    let expected = "Hello, Somebody!";
    let actual = greet(arg1, arg2);
    assert_ne!(expected, actual);
}

#[test]
fn test_greetings_should_pass_if_correct_args_is_passed() {

    let arg = "Somebody";
    let expected = "Hello, Somebody!";
    let actual = greet(arg);
    assert_eq!(expected, actual);
}

