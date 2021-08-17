# midicsv

In the spirit of RIIR or [Rewrite It In Rust](https://transitiontech.ca/random/RIIR) I decided
to try my hand at implementing John Walker's [`midicsv`](https://www.fourmilab.ch/webtools/midicsv/) and `csvmidi` utilities in Rust.

The results of this work-in-progress rewrite show that Rust crates are an awesome way of
reusing functionality. For example, the processing of CSV and Standard MIDI files (SMF)
is entirely delegated to crates made by other users. CSV files are handled using the
[csv](https://crates.io/crates/csv) crate, while MIDI file parsing is handled by the [midly](https://docs.rs/midly/0.5.2/midly/) crate. In the original `midicsv` and `csvmidi`
utilities these were handcrafted in C.

Of course, this crate-focused approach has both
good and bad sides. Starting with the obvious dependency on someone else's code, there is
also something of a learning curve as you get adjusted to someone else's idea of how to
model a domain. Nevertheless, `csv` in particular has an excellent
[tutorial](https://docs.rs/csv/1.1.6/csv/tutorial/index.html).

This is very much a work in progress; I would still rely on the original C language
utilities if I were doing something serious.

## To do

* Command line argument processing.
