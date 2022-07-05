# Contributing

Contributions in the form of pull requests and issues are appreciated, but I
cannot promise they will be addressed in a timely manner.

## Issues

If filing a bug report, please include a minimal reproducible example and
details about your environment. If the issue was found by running the tests
then the contents of `proptest-regressions/` would be great (if available).

Feature requests should include a motivating example i.e. _why_ do you want it
to do the thing you're asking. Maybe there's a better way to achieve that goal
or perhaps you can already do it but not in the way you suggested.

## Pull Requests

Before submitting a PR you should run `cargo clippy` and `cargo fmt`.
Accompanying tests with your PR are also appreciated, even more so if they are
property-based. Regardless, all tests should pass when running `cargo test`
before you submit the PR.

It is also very strongly encouraged to avoid anything that breaks `no_std`
compatibility. Also, while there is (currently) no MSRV target, changes that
require nightly-only features will be rejected or worse: ignored until the
feature gets stabilized.

All uses of `unwrap` and `unsafe` should have safety comments about why they
are safe.
