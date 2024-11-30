use host::add;
use methods::ADD_ID;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Pick two numbers
    let (receipt, _) = add(10, 100);

    // Here is where one would send 'receipt' over the network...

    // Verify receipt, panic if it's wrong
    receipt.verify(ADD_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
}
