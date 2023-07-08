# deck-farfalle ![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue) [![deck-farfalle on crates.io](https://img.shields.io/crates/v/deck-farfalle)](https://crates.io/crates/deck-farfalle) [![deck-farfalle on docs.rs](https://docs.rs/deck-farfalle/badge.svg)](https://docs.rs/deck-farfalle) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/niluxv/permutation_based_crypto) ![Rust Version: ^1.65](https://img.shields.io/badge/rustc-%5E1.65-orange.svg)

Generic Farfalle construction which is generic over the cryptographic permutation and Kravatte and Xoofff instantiations.

`Farfalle: Permutation -> DeckFunction`

This crate contains an implementation of the [Farfalle construction][__link0], [`Farfalle`][__link1]. It is generic over the permutations and rolling functions used, through the [`FarfalleConfig`][__link2] trait. The [`Farfalle`][__link3] struct is intended to be used through the [`crypto_permutation::DeckFunction`][__link4] trait that it implements.

**Note**: No security audits of this crate have ever been performed. Use at your own risk!

The `kravatte` and `xoofff` crate-features enable the Kravatte and Xoofff instantiations of Farfalle, in the [`kravatte`][__link5] and [`xoofff`][__link6] modules respectively. These also contain the rolling functions that are used by these instantiations, so it is easy create your own custom instantiation of Farfalle that differs from Kravatte or Xoofff in the round count for the permutation (in case you think the advised parameters are not conservative enough).


## Features

 - `kravatte`: Enables the [`kravatte`][__link7] module.
 - `xoofff`: Enables the [`xoofff`][__link8] module.
 - `debug`: Used for tests. Don’t use!


## Testing

The Kravatte instantiation has been tested against the [`kravatte` python package][__link9]. The Xoofff instantiation has been tested against the [`xoofff` crate][__link10].



 [__cargo_doc2readme_dependencies_info]: ggGkYW0BYXSEG8lq_dIqgTVtG1jCMXLwLpFYGykuOgo4U562G4crLsejH8cFYXKEG0Es_eRDRk6zG_yCjXNZxYZmGzPpa6Mnh3FKG4v-kgZOZ4QgYWSCgnJjcnlwdG9fcGVybXV0YXRpb25lMC4xLjCDbWRlY2stZmFyZmFsbGVlMC4xLjBtZGVja19mYXJmYWxsZQ
 [__link0]: https://keccak.team/farfalle.html
 [__link1]: https://docs.rs/deck-farfalle/0.1.0/deck_farfalle/?search=Farfalle
 [__link10]: https://crates.io/crates/xoofff
 [__link2]: https://docs.rs/deck-farfalle/0.1.0/deck_farfalle/trait.FarfalleConfig.html
 [__link3]: https://docs.rs/deck-farfalle/0.1.0/deck_farfalle/?search=Farfalle
 [__link4]: https://docs.rs/crypto_permutation/0.1.0/crypto_permutation/?search=DeckFunction
 [__link5]: https://docs.rs/deck-farfalle/0.1.0/deck_farfalle/kravatte/index.html
 [__link6]: https://docs.rs/deck-farfalle/0.1.0/deck_farfalle/xoofff/index.html
 [__link7]: https://docs.rs/deck-farfalle/0.1.0/deck_farfalle/kravatte/index.html
 [__link8]: https://docs.rs/deck-farfalle/0.1.0/deck_farfalle/xoofff/index.html
 [__link9]: https://pypi.org/project/kravatte
