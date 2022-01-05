// This is a module file that doesn't get included in test unless we use it in our tests
// These tests also don't get compiled into our crate because they are in a sub folder of 'tests'

pub fn setup() {
    // setup code specific to your library's tests would go here
    // to see this print we need to run our test as cargo test -- --show-output
    println!("Setting up our tests")
}
