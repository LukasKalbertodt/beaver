Busy Beaver
===========

An application that analyses/simulates all possible N-state Turing machines with tape alphabet = {0, 1}, where `N` is given as command line argument.
All TMs are generated, then categorized based on static analysis or their runtime behavior.
Finally, a summary of all results is printed.
This is related to the [Busy Beaver game](https://en.wikipedia.org/wiki/Busy_Beaver_game).

**Features**:
- Nice output, including histogram
- Fairly fast (on a Ryzen 3600, N=4 with its 11 billion TMs, 5.5 billion deduplicated, runs in roughly half a minute)
- Static analysis: state graph reachability check
- Can detect and categorize ≈97% of non-halting TMs (for N=4)

<p align="center">
    <img src=".github/readme-image.png" width="90%"></img>
</p>

**Potential goals for future development**
- [ ] Optimize
- [ ] More smart analysis techniques
- [ ] Print states of certain TMs (e.g. the winning one)
- [ ] Show trace of the winning TM

Note that this is just a hobby project which I don't expect to be useful to anyone.
If you are still interested, PRs are always welcome.

---

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
