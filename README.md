# catfact
A dummy Rust client to display random cat facts querying https://catfact.ninja/fact

# Running via Cargo

```bash
$  cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/catfact`
Researchers are unsure exactly how a cat purrs. Most veterinarians believe that a cat purrs by vibrating vocal folds deep in the throat. To do this, a muscle in the larynx opens and closes the air passage about 25 times per second.
```

# Running via Docker

A working sample Dockerfile is shipped

```bash
$ docker build -t rust-catfact
$ docker run --rm --name cat rust-catfact
```