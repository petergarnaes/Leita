# Leita - The search engine _almost_ as bad as Bing!

This project is re-implementation of classic algorithms and data structures for search and information retrieval.

This project is implemented in Rust, simply build and run it through cargo: `cargo build; cargo run`. You might need to run the web crawler script first, in order to pull the test data that is indexed.

Project can also be run through docker, simply build `docker build -t leita .` and run `docker run -ti --rm --name leita leita`.

## `Dockerfile-tine` release build

The tiny docker release build runs on the `alpine` linux image, because of its minimal runtime.

Because `alpine` uses musl libc we target that in the rust build.

We also compile the binary so it display less helpful panic messages and is compiled single threaded for better optimizations.

Lastly we strip the binary, which saves a lot of disk space, but makes it harder to debug.

## Final remarks

This project is based on the teachings in "Information Retrieval: Implementing and Evaluating Search Engines" ISBN: 0262528878.

Made in collaboration with DBC A/S.