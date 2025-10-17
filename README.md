Coding Challenges Solutions
===

A repository of my solutions to the challenges put forth in [codingchallenges.fyi].

So far I'm planning to use Rust to implement my solutions but in future challenges I may opt for other languages if I feel they are more suitable.
For example, I may use Elixir instead for solving a challenge where a web application is called for.
I also aim to use as few dependencies as I reasonably can.
Some exceptions to this rule (off the top of my head):
- [`rand`](https://lib.rs/crates/rand) for generating random numbers
- [`winit`](https://lib.rs/crates/winit) for making windows
- [`bevy`](https://bevy.org/) for challenges that amount to "make a game"
- [`chrono`](https://lib.rs/crates/chrono) for datetimes
- A crate for non-blocking IO like [`tokio`](https://tokio.rs/) or [`mio`](https://lib.rs/crates/mio)
- "Utility" crates like [`anyhow`](https://lib.rs/crates/anyhow) and [`env_logger`](https://lib.rs/crates/env_logger)

I will obviously _avoid_ libraries that solve the challenge all on their own, such as using `serde_json` for the [JSON parser challenge](https://codingchallenges.fyi/challenges/challenge-json-parser)
or `hyper` for the [web server challenge](https://codingchallenges.fyi/challenges/challenge-webserver).
As an aside, I have implemented both of these things in the past so I already know I'm perfectly capable of completing them honestly :3
