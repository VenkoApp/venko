# venko ✌️

[![Crates.io](https://img.shields.io/crates/v/venko)](https://crates.io/crates/venko)
[![Docs.rs](https://img.shields.io/docsrs/venko)](https://docs.rs/venko)
[![License](https://img.shields.io/crates/l/venko)](https://github.com/VenkoApp/venko/blob/master/LICENSE)
[![Build Status](https://img.shields.io/github/workflow/status/VenkoApp/venko/E2E/master)](https://github.com/VenkoApp/venko/actions/workflows/programs-e2e.yml?query=branch%3Amaster)
[![Contributors](https://img.shields.io/github/contributors/VenkoApp/venko)](https://github.com/VenkoApp/venko/graphs/contributors)
[![NPM](https://img.shields.io/npm/v/@venkoapp/venko)](https://www.npmjs.com/package/@venkoapp/venko)

<p align="center">
    <img src="https://raw.githubusercontent.com/VenkoApp/venko/master/images/banner.png" />
</p>

Venko: Rails for realtime finance on Solana.

## About

Venko is a protocol for issuing tokenized payments streams. It is designed for a variety
of common payments usecases, including:

- **Token lockups.** Issue team tokens.
- **Grants.** Issue revocable grants with a cliff and release schedule.
- **Escrow.** Send tokens to someone with a cancellation period, allowing you to cancel the payment if it was sent to the wrong address.

We're in active development. For the latest updates, please join our community:

- Twitter: https://twitter.com/VenkoApp

## Note

- **Venko is in active development, so all APIs are subject to change.**
- **This code is unaudited. Use at your own risk.**

## Addresses

Program addresses are the same on devnet, testnet, and mainnet-beta.

- Venko: [`AnatoLyYrd5iaAe36Lvq2oS4nuVDnRAb3KBVCARt4XiZ`](https://explorer.solana.com/address/AnatoLyYrd5iaAe36Lvq2oS4nuVDnRAb3KBVCARt4XiZ)

## Contribution

Thank you for your interest in contributing to Venko Protocol! All contributions are welcome no matter how big or small. This includes
(but is not limited to) filing issues, adding documentation, fixing bugs, creating examples, and implementing features.

When contributing, please make sure your code adheres to some basic coding guidlines:

- Code must be formatted with the configured formatters (e.g. `rustfmt` and `prettier`).
- Comment lines should be no longer than 80 characters and written with proper grammar and punctuation.
- Commit messages should be prefixed with the package(s) they modify. Changes affecting multiple packages should list all packages. In rare cases, changes may omit the package name prefix.

## License

Venko Protocol is licensed under the GNU Affero General Public License v3.0.
