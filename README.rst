Revision of the Rust programming language over the sunny weekend.


Roadmap
=======

- Weekend 1: Read the book, do the exercises, focus on fundamentals.

- Weekend 2: Read the book, do the exercises, focus on ownership, concurrency in Rust.

- Weekend 3: Build a small project in Rust, for this, i am picking out a simple project from the back of my head that takes your favorite Jira Issues and migrate it to GitHub Issues


Weekend 1 - Notes
=================
So I'm gonna say this, I took a look at Rust and I impressed with its close performance to C++ in spite of its absence of GC or Runtime.
Today, I took a whole new approach to learning Rust, I decided to try out the TDD approach, more specifially Behaviour-Driven Development.

Here are a few things I learned today:

- Writing tests in Rust is pretty easy, you just need to add the `#[test]` attribute to your test function and you are good to go.
- You can also add the `#[ignore]` attribute to ignore a test.
- You can also add the `#[should_panic]` attribute to check if a test panics (that is should fail delibratey).
- You can also add the `#cfg(test)` attribute to your test module to tell the compiler to only compile the module when running tests.
- There is a `cargo test` command that runs all the tests in your project.
- You can run a single test by passing the name of the test to the `cargo test` command.

Having worked with a few languages, with deep knowledge of Java, Python and JavaScript, 

Here are a few knowlege I immidiately picked up - let's talk about Python and Go for now:

- Property-based testing is a thing in Rust as in Go, which in Python is called Hypothesis.
- Testing functions are similar to pytest - figuratively speaking, except Rust uses macros and you have to append a `!` to the end of the macro name.
  e.g. `assert_eq!` is the equivalent of `assertEqual` in Python or `assert.Equal` in Go.
       `assert_ne!` is the equivalent of `assertNotEqual` in Python or `assert.NotEqual` in Go.
       `assert!` is the equivalent of `assertTrue` in Python or `assert.True` in Go.
       `assert!` is also the equivalent of `assert` in Python or `assert` in Go.


