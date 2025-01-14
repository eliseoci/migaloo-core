<a href="https://whitewhale.money/">
  <h1 align="center">
    <picture>
      <img alt="Flutter" src="https://miro.medium.com/max/1400/1*29OYRJqqddosWtWo-c3TYQ.png">
    </picture>
  </h1>
</a>

[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](https://makeapullrequest.com)
[![first-timers-only](https://img.shields.io/badge/first--timers--only-friendly-blue.svg?style=flat-square)](https://www.firsttimersonly.com/)
[![Discord badge][]][Discord invite]
[![Twitter handle][]][Twitter badge]
[![CII Best Practices](https://bestpractices.coreinfrastructure.org/projects/6401/badge)](https://bestpractices.coreinfrastructure.org/projects/6401)


[Discord invite]: https://discord.com/invite/tSxyyCWgYX
[Discord badge]: https://img.shields.io/discord/908044702794801233
[Twitter handle]: https://img.shields.io/twitter/follow/WhiteWhaleDefi.svg?style=social&label=Follow
[Twitter badge]: https://twitter.com/intent/follow?screen_name=WhiteWhaleDefi

## Getting started

To get started with `migaloo-core`, we encourage you to go through our [contributing guide](./CONTRIBUTING.md) to see the 
different ways to contribute to the project.

## Resources

1. [Website](https://whitewhale.money/)
2. [LitePaper](https://whitewhale.money/LitepaperV2.pdf)
3. [Docs](https://white-whale-defi-platform.github.io/docs/)
4. [Discord](https://discord.com/invite/tSxyyCWgYX)
5. [Twitter](https://twitter.com/WhiteWhaleDefi)
6. [Telegram](https://t.me/whitewhaleofficial)

## Building Migaloo

To build Migaloo´s smart contracts, you can run the rust workspace optimizer on the root folder of the project. Alternatively, you can run `scripts/build_release.sh` to build the project artifacts.

## Testing

To run the tests, run `cargo test`. You can also run `cargo tarpaulin -v` to get test code coverage.

## Disclaimer

Migaloo core contracts are not audited yet in their current form, though most of the code has been audited and heavily used in the past, namely White Whale's v1 and TerraSwap. We plan the audit the code once the IBC components are ready, as they can potentialy introduce changes on the existing contracts.

**Use the contracts and the White Whale app at your own risk!**

## Contributing

[Contributing Guide](./CONTRIBUTING.md)

[Code of Conduct](./CODE_OF_CONDUCT.md)

[Security Policies and Procedures](./SECURITY.md)

[License](./LICENSE)
