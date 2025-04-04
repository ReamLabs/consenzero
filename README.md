# consenzero

`consenzero` is a Proof of Concept of integrating zkVM with the current Ethereum Consensus layer. It will leverage [lighthouse](https://github.com/sigp/lighthouse) and [RISC Zero](https://github.com/risc0/risc0).

## Responsibilities

1. **The "host" program 👷** - Retrieves the beacon state and beacon block from wild, feeds them to the guest to prove the state transition and broadcasts the receipt to the attester program.
2. **The "guest" program 📜** - Contains the actual beacon state transition function (STF) to be compiled into an ELF for 1) the host to prove and 2) the attester to verify the state transition.
3. **The "attester" program 🕵️** - Listens for a broadcasted receipt, verifies that the receipt is valid (follows the expected state transition function defined in the ELF), progresses its internal beacon state, and announces the receipt's validity.

## Behavior Flow

1. The **host 👷** retrieves a beacon state at a specific slot number (simulates a sync'ed client).
2. The **host 👷** initializes the **guest 📜** with the retrieved beacon state.
3. The **host 👷** listens for a beacon block broadcast (simulates block building).
4. The **host 👷** submits the newly broadcasted block to the **guest 📜** to prove (simulates block sealing).
5. The **guest 📜** returns a receipt to the **host 👷**.
6. The **host 👷** broadcasts the receipt to the **attester 🕵️** (simulates a block proposal)..
7. The **attester 🕵️** verifies the receipt, progresses its beacon state, and announces the receipt's validity (simulates block attestation).

## Directory Structure

```text
consenzero
├── Cargo.toml
├── host
│   ├── Cargo.toml
│   └── src
│       └── main.rs                    <-- [Host code goes here]
└── methods
    ├── Cargo.toml
    ├── build.rs
    ├── guest
    │   ├── Cargo.toml
    │   └── src
    │       └── method_name.rs         <-- [Guest code goes here]
    └── src
        └── lib.rs
```

## Quick Start

First, make sure [rustup] is installed. The
[`rust-toolchain.toml`][rust-toolchain] file will be used by `cargo` to
automatically install the correct version.

To build all methods and execute the method within the zkVM, run the following
command:

```bash
cargo run -- --local
```

This is an empty template, and so there is no expected output (until you modify
the code).

### Executing the Project Locally in Development Mode

During development, faster iteration upon code changes can be achieved by leveraging [dev-mode], we strongly suggest activating it during your early development phase. Furthermore, you might want to get insights into the execution statistics of your project, and this can be achieved by specifying the environment variable `RUST_LOG="[executor]=info"` before running your project.

Put together, the command to run your project in development mode while getting execution statistics is:

```bash
RUST_LOG="[executor]=info" RISC0_DEV_MODE=1 cargo run -- --local
```

### Real Proof Generation

Once you've reached a point where you're ready to generate real proofs, you can do so by setting RISC0_DEV_MODE=0. Generating proofs locally would be achieved by running the following:

```bash
RISC0_DEV_MODE=0 cargo run --release -- --local
```

### Executor Statistics

To gain insights into your application's performance, you can obtain executor statistics by setting the RUST_LOG environment variable to info.

Setting this filter will print statistics about the execution before proof generation, so you can understand how computationally expensive your application is. Since the statistics concern only the executor phase, it is recommended to run your application in dev-mode to avoid the overhead of proof generation:

```bash
RISC0_DEV_MODE=1 RUST_LOG=info cargo run --release -- --local
```

The statistics include:

- Total Cycles
- Session Cycle
- Segments Count
- Execution time
