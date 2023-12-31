# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.0.48 (2023-09-07)

<csr-id-5ae5b4e60e3f764615d89dca9e45db10be732f37/>

The initial release which really is just a renamed fork of `therror`.
Thanks to `dtolnay` for providing such a good base for further development.

### Other

 - <csr-id-5ae5b4e60e3f764615d89dca9e45db10be732f37/> avoid using `formatter`
   This can collide with enum variant field names. Namespace with a
   `_therror_` prefix to avoid such collisions.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 249 commits contributed to the release over the course of 1428 calendar days.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release therror-impl v0.0.48 ([`3a6cbc4`](https://github.com/Byron/theerror/commit/3a6cbc444c5c912c0e48a8d739beee4b3fc00281))
    - Add changelogs prior to release ([`dc9b3d6`](https://github.com/Byron/theerror/commit/dc9b3d669f4b5acfecffc2654afa0c0da689317c))
    - Rename `therror` to `theerror` ([`b19ab24`](https://github.com/Byron/theerror/commit/b19ab24e0260684a13582e08978c94afeebb8917))
    - Release 1.0.48 ([`5c5f342`](https://github.com/Byron/theerror/commit/5c5f342a5e601e4bfb49c8c095cb57c9f14eb0c8))
    - Ignore manual_let_else pedantic clippy lint ([`1f5cbd7`](https://github.com/Byron/theerror/commit/1f5cbd701ed0977b0649f055549becd8181970c4))
    - AsDisplay is no longer an unused import ([`7566a29`](https://github.com/Byron/theerror/commit/7566a29b2aa5bad90ef190445bc094c2979b041a))
    - Touch up PR 251 ([`887c9fb`](https://github.com/Byron/theerror/commit/887c9fb3fcd6add62c847292a13db728825f5837))
    - Merge pull request #251 from mina86/b ([`f0c79cb`](https://github.com/Byron/theerror/commit/f0c79cbd29f826d6227ca560a806c8b036ac039c))
    - Replace DisplayAsDisplay and PathAsDisplay with AsDisplay trait ([`97eeb45`](https://github.com/Byron/theerror/commit/97eeb45b8b8a8b2373b74c2c6be0d4fa14d6596d))
    - Release 1.0.47 ([`0495eaa`](https://github.com/Byron/theerror/commit/0495eaa802c73454ed6969fa1a3520db635bb901))
    - Release 1.0.46 ([`5ada5d5`](https://github.com/Byron/theerror/commit/5ada5d5198d1f616d296c8dcbbbeef88f1118ab1))
    - Release 1.0.45 ([`06f1895`](https://github.com/Byron/theerror/commit/06f189583213cf2bce239af2f7e24c921c58bc8e))
    - Merge pull request #246 from dtolnay/errorprovide ([`a11330f`](https://github.com/Byron/theerror/commit/a11330f7fb9b502fa46672d0c548e838134eb555))
    - Update to nightly's new Error::provide API ([`8a95c25`](https://github.com/Byron/theerror/commit/8a95c2583e291bdb18f1cd0fb8600a65cc1866b6))
    - Release 1.0.44 ([`54b70cf`](https://github.com/Byron/theerror/commit/54b70cfe109981e6349aebae4393c62c93cccafc))
    - Opt in to generate-link-to-definition when building on docs.rs ([`f86e8e5`](https://github.com/Byron/theerror/commit/f86e8e5881f157f115c271b9ff578867cc91ba92))
    - Release 1.0.43 ([`225adab`](https://github.com/Byron/theerror/commit/225adab854715459bc81dd3e788805debcf7f310))
    - Merge pull request #242 from dtolnay/stdwrite ([`f6dc5e5`](https://github.com/Byron/theerror/commit/f6dc5e54e2c0604388ac43da99ebea5e9c75decc))
    - Avoid calling a nonstandard write! macro that might be in scope ([`cab9fec`](https://github.com/Byron/theerror/commit/cab9fec03e0f734ab7d42aa5de34024d0df761cd))
    - Revert "Avoid calling a nonstandard write! macro that might be in scope" ([`900f018`](https://github.com/Byron/theerror/commit/900f0189889133f1d76365dc961023f85e4b3a19))
    - Release 1.0.42 ([`305be4a`](https://github.com/Byron/theerror/commit/305be4a9798449ef757b8c9ddd2a6c3f6a10a101))
    - Merge pull request #240 from dtolnay/stdwrite ([`6165f58`](https://github.com/Byron/theerror/commit/6165f5859661500fe47a796bca73bac5504f90e3))
    - Avoid calling a nonstandard write! macro that might be in scope ([`264b7d1`](https://github.com/Byron/theerror/commit/264b7d19360332bd2cf8289a54619c3360a2f8fe))
    - Update to 2021 edition ([`43f3a2a`](https://github.com/Byron/theerror/commit/43f3a2a661f1fa75a23933a0d039677307a4e1e0))
    - Release 1.0.41 ([`281997e`](https://github.com/Byron/theerror/commit/281997e7606bd90c70b39e114a4d26b733e63b30))
    - Eliminate syn 1 from minimal-versions ([`c28f8fa`](https://github.com/Byron/theerror/commit/c28f8fa8f1a725b600e41563a8a51b38962d0459))
    - Use error reporting provided by Meta ([`39aaeb0`](https://github.com/Byron/theerror/commit/39aaeb00ff270a49e3c254d7b38b10e934d3c7a5))
    - Release 1.0.40 ([`3cec8c4`](https://github.com/Byron/theerror/commit/3cec8c487953298acd00c61ef9a81d0461517974))
    - Merge pull request #227 from dtolnay/syn ([`2c65cea`](https://github.com/Byron/theerror/commit/2c65ceadfa53f9a12dfd4c3b79b0b5e596e46d6e))
    - Update to syn 2 ([`fb8b81f`](https://github.com/Byron/theerror/commit/fb8b81f20b352b9adf47639a9af1dbcbdcc13d81))
    - Merge pull request #226 from dtolnay/tokenspan ([`0e45dde`](https://github.com/Byron/theerror/commit/0e45dde2065aecdaf64a8a4970bc75888de71b48))
    - Eliminate unneeded use of Spanned trait on single tokens ([`490dc01`](https://github.com/Byron/theerror/commit/490dc0102b2bd11f23755ca977b4610974cc8af4))
    - Release 1.0.39 ([`f729af9`](https://github.com/Byron/theerror/commit/f729af95d16fbec0ee167ed79231c6ff1a4d88c7))
    - Release 1.0.38 ([`74bfe75`](https://github.com/Byron/theerror/commit/74bfe75eb25ba9d39b0ae5b570d611855cbc5086))
    - Release 1.0.37 ([`8a996a5`](https://github.com/Byron/theerror/commit/8a996a5bfd5303c2fab64827cbfde02caa6cef66))
    - Release 1.0.36 ([`7b226e3`](https://github.com/Byron/theerror/commit/7b226e33c7e83999363ce6a0eeb341aeb38ca8b2))
    - Release 1.0.35 ([`10ffe03`](https://github.com/Byron/theerror/commit/10ffe038172ef47a742b438f5c8dad059628c417))
    - Merge pull request #190 from dtolnay/provider ([`2ca76ed`](https://github.com/Byron/theerror/commit/2ca76edd6eada9522e1198355a83bc16049f52b0))
    - Use ThiserrorProvide to disambiguate 'provide' method calls ([`aaf8449`](https://github.com/Byron/theerror/commit/aaf8449dcb25b31a24c39056c420afc99416e7b0))
    - Revert "Directly call source.provide instead of going through dyn error" ([`3bcad59`](https://github.com/Byron/theerror/commit/3bcad5957d22c0331dbb65580eb42b66daf782f3))
    - Release 1.0.34 ([`48f697a`](https://github.com/Byron/theerror/commit/48f697af3d66e69be1923eaf474f175dc2c825b7))
    - Merge pull request #184 from dtolnay/provide ([`76c5568`](https://github.com/Byron/theerror/commit/76c5568135a62bf48c44a62648b89f35306be487))
    - Directly call source.provide instead of going through dyn error ([`f924c25`](https://github.com/Byron/theerror/commit/f924c251ecae1b4cce08422ed5f7a7eb69776ff9))
    - Release 1.0.33 ([`fdb266a`](https://github.com/Byron/theerror/commit/fdb266af297fa8531dad0859615b0a8e0b22da36))
    - Merge pull request #182 from dtolnay/provider ([`905680e`](https://github.com/Byron/theerror/commit/905680eee0e78b092d4adee5b3a7631ea87879e2))
    - Expose backtrace via generic member access ([`985da4c`](https://github.com/Byron/theerror/commit/985da4c7d9c460629e268fc069a4125042d24647))
    - Make private module more clearly private ([`c81dc67`](https://github.com/Byron/theerror/commit/c81dc6731b9578326dcc945b067c9d78ffa3f847))
    - Release 1.0.32 ([`8cb98af`](https://github.com/Byron/theerror/commit/8cb98afb749ec24cf703b8b89af2e159d529c4a0))
    - Sort package entries in Cargo.toml ([`c79f5c9`](https://github.com/Byron/theerror/commit/c79f5c9be5d0c60e1eda78efdd10cbd0e0c410ba))
    - Ignore manual_find clippy lint ([`f09771e`](https://github.com/Byron/theerror/commit/f09771ebba5b3f064fb2b3e84c6601ee5065dc0d))
    - Ignore type_repetition_in_bounds/trait_duplication_in_bounds clippy false positive ([`d5fa929`](https://github.com/Byron/theerror/commit/d5fa92940b0a4af9d7fa53a41dffc557e1d281b2))
    - Release 1.0.31 ([`cbe8212`](https://github.com/Byron/theerror/commit/cbe821249595aa5e389d5cb96e84da80adca8531))
    - Ignore wrong_self_convention clippy lint ([`21c2690`](https://github.com/Byron/theerror/commit/21c26903e29cb92ba1a7ff11e82ae2001646b60d))
    - Merge pull request #164 from dtolnay/frombacktrace ([`320d70f`](https://github.com/Byron/theerror/commit/320d70f691dc93cff8fc4cb17b384d4913be5884))
    - Fix miscounting fields when from and backtrace are same field ([`c4d7c2b`](https://github.com/Byron/theerror/commit/c4d7c2b1358d4c1da0c728da7d78a6e4787a1750))
    - Ignore cast_lossless Clippy pedantic lint ([`d7664d5`](https://github.com/Byron/theerror/commit/d7664d54757fc214e2978911b40a6335fe909140))
    - Release 1.0.30 ([`672e952`](https://github.com/Byron/theerror/commit/672e9525bbc2e5682c380d36974f34716b963591))
    - Declare minimum Rust version in Cargo metadata ([`ed234d4`](https://github.com/Byron/theerror/commit/ed234d41b5fc56810f680f925ff38515847a6b25))
    - Release 1.0.29 ([`c7dd271`](https://github.com/Byron/theerror/commit/c7dd271dcd92af17168746a809503ee392d6f6ad))
    - Skip a redundant Member clone ([`4b581e3`](https://github.com/Byron/theerror/commit/4b581e3fb95fb16f0938ebe8b4ac64eb9b5bbe43))
    - Merge pull request #152 from dtolnay/bounds ([`19a15cb`](https://github.com/Byron/theerror/commit/19a15cb0c7b80e16ca81a02fc50a1b97c4f88812))
    - Handle multiple bounds from the same format string on the same field ([`f688fd7`](https://github.com/Byron/theerror/commit/f688fd70092daafa1b44fb10a324968d4b2fc9b4))
    - Merge pull request #151 from dtolnay/bounds ([`113fcaa`](https://github.com/Byron/theerror/commit/113fcaa22501847988f71a04602c9fb78172bd34))
    - Deduplicate inferred bounds ([`72abba6`](https://github.com/Byron/theerror/commit/72abba6f046ddfd1dc590b3ba8f4d9de8856a4bc))
    - Merge pull request #150 from dtolnay/bounds ([`34f5931`](https://github.com/Byron/theerror/commit/34f5931adaf09a9cd729944826e6b6c2ea301047))
    - Implied bounds for the remaining std::fmt traits ([`cc65053`](https://github.com/Byron/theerror/commit/cc65053651b82c8317d71fdfa907efe6f6efbc19))
    - Merge pull request #149 from dtolnay/bound ([`0a1c5bd`](https://github.com/Byron/theerror/commit/0a1c5bd7bb7f77550c8f1614af5598d13d1f3363))
    - Implied bounds for transparent attribute ([`3e699aa`](https://github.com/Byron/theerror/commit/3e699aa43b661a6faa7aac4de25d12edf91ed38c))
    - Merge pull request #148 from dtolnay/bounds ([`e95b4ad`](https://github.com/Byron/theerror/commit/e95b4adaab3bb72db858391541c8baf2da4ea423))
    - Implied bounds for Display and Error impl ([`1e6e267`](https://github.com/Byron/theerror/commit/1e6e2679140b995428c5cf552b463443d7fdf261))
    - Revert "Suppress nonstandard_macro_braces in generated code" ([`ec9ac76`](https://github.com/Byron/theerror/commit/ec9ac76c52a1ffcb1e442565d4924385b6611546))
    - Release 1.0.28 ([`b47c75d`](https://github.com/Byron/theerror/commit/b47c75d3f5fc3f9d4101feb1b1ab30dbcd543141))
    - Merge pull request #147 from dtolnay/optionfrom ([`9af5265`](https://github.com/Byron/theerror/commit/9af5265aa950bfe4542fdad111655091ab141f10))
    - Support #[from] on an Option field ([`2a2d172`](https://github.com/Byron/theerror/commit/2a2d1725e7ae18aa4dec976dfc629aa41ee23e0f))
    - Release 1.0.27 ([`b087faf`](https://github.com/Byron/theerror/commit/b087faf217affd8248a1019cc20db218f1059e36))
    - Merge pull request #146 from dtolnay/backtrace ([`2e2c126`](https://github.com/Byron/theerror/commit/2e2c1263422f9b1d13806e4aa543cae2f6170f3e))
    - Extract combined backtrace-source case to separate match arm ([`799bb53`](https://github.com/Byron/theerror/commit/799bb53b10324f72cf6042f126b467735693f1f1))
    - Handle backtrace coming from Option source field ([`d49c5af`](https://github.com/Byron/theerror/commit/d49c5af10f8fb0448258ebfd400f32b6ed8c1372))
    - Handle enum containing #[source] #[backtrace] field ([`2b37b9e`](https://github.com/Byron/theerror/commit/2b37b9ed6226b5ea46116d4d0775275f086bc3a6))
    - Factor out distinct backtrace logic ([`ed396c2`](https://github.com/Byron/theerror/commit/ed396c2074be980ffde77d63bd78427b0d2b4cf2))
    - Adjust how distinct backtrace field is accessed ([`9b542ce`](https://github.com/Byron/theerror/commit/9b542cef8fbb31e07c5e0ec5cdca682d020c0888))
    - Touch up PR 137 ([`ca33ed9`](https://github.com/Byron/theerror/commit/ca33ed9bc7417d35f45f7de4b6d1ea9aa2801382))
    - Merge pull request 137 from astraw/combined-from-and-backtrace-field ([`c45d7e4`](https://github.com/Byron/theerror/commit/c45d7e4de8cc4a3d337931633cb964b02f7dd8fb))
    - Release 1.0.26 ([`031fea6`](https://github.com/Byron/theerror/commit/031fea6f3b82c72be11477e7550c6ae3579e6139))
    - Suppress nonstandard_macro_braces in generated code ([`245e7cf`](https://github.com/Byron/theerror/commit/245e7cfd149140806ecef89d44b14e9557b297b1))
    - Allow #[from] and #[backtrace] on same field ([`86993c2`](https://github.com/Byron/theerror/commit/86993c25ab39d24a983eb76a208f51c502e17b90))
    - Resolve needless_borrow clippy lints ([`a37b5ab`](https://github.com/Byron/theerror/commit/a37b5ab11f7c133d1bcb24bc67dfe6c037b69a34))
    - Release 1.0.25 ([`19cb5ce`](https://github.com/Byron/theerror/commit/19cb5cee4b51203f6801daa3ff0185761d0d3d4c))
    - Ignore manual_map clippy lint ([`c10adbc`](https://github.com/Byron/theerror/commit/c10adbc25e730ff1a94315c9e11927274c303997))
    - Release 1.0.24 ([`1b0a849`](https://github.com/Byron/theerror/commit/1b0a84996b9492c0dc5779127a91c930f23a259e))
    - Merge pull request 121 from Aaron1011/fix/source-span ([`d81b746`](https://github.com/Byron/theerror/commit/d81b7466bc70fdfe4183e61d65c17d42904f542e))
    - Consistently use `quote!` when emitting 'source' ([`0fa679b`](https://github.com/Byron/theerror/commit/0fa679b1b87e5c13457ee654318e2dff3d6b7d1c))
    - Resolve clippy filter_map_next pedantic lint ([`dc3c5c6`](https://github.com/Byron/theerror/commit/dc3c5c6a87570feb61f6ac21da4bc5ad58bd1e21))
    - Opt in to pedantic clippy lints ([`c0a56fe`](https://github.com/Byron/theerror/commit/c0a56fefbe2c78f710122fa5a6614fd64e72464e))
    - Release 1.0.23 ([`d263b4b`](https://github.com/Byron/theerror/commit/d263b4b7e4f8e3ad9558256d8f6b2ac916985f7b))
    - Merge pull request #116 from dtolnay/lifetime ([`da6ee81`](https://github.com/Byron/theerror/commit/da6ee819094ed13ccb7c98c98dd45556b77197b7))
    - Detect non-static lifetime behind reference type parameter ([`0859205`](https://github.com/Byron/theerror/commit/08592050184b5b525d04de5bec131baae3e3b1b8))
    - Merge pull request #115 from dtolnay/lifetime ([`cb3077c`](https://github.com/Byron/theerror/commit/cb3077c3156f25d456579d48f0e20541261b99b1))
    - Add justification to non-static lifetime diagnostic ([`d31d96b`](https://github.com/Byron/theerror/commit/d31d96bba2479e24f6a5414837c83c849c15e483))
    - Tweak placement of diagnostic on non-static lifetimes in source ([`464a409`](https://github.com/Byron/theerror/commit/464a40960886e04eb1a3a75339cc16bf7b9d6833))
    - Release 1.0.22 ([`09f247a`](https://github.com/Byron/theerror/commit/09f247addaf6c5f57353f9558ba131e6619390c7))
    - Merge pull request #109 from dtolnay/keyword ([`53bb2fb`](https://github.com/Byron/theerror/commit/53bb2fbf981b4bba914fc07af9c69589bfcb302e))
    - Fix second place that keyword fmt argument is failing ([`aafcf0a`](https://github.com/Byron/theerror/commit/aafcf0a1b6946b407740fce08c783db5a96cdb20))
    - Fix first place that keyword fmt argument is failing ([`2722f8e`](https://github.com/Byron/theerror/commit/2722f8e2c3b98c76902da3d10aa7938b9cc6168d))
    - Briefer naming convention for raw identifier format vars ([`334fed3`](https://github.com/Byron/theerror/commit/334fed3a7443b9a03c9fc97cfce9d0a161bb3615))
    - Consolidate ident lex and conversion to Ident ([`dbbd0f8`](https://github.com/Byron/theerror/commit/dbbd0f816c7c39ff2286f179067816bf19bc1390))
    - Replace use of str::strip_prefix ([`227ef7d`](https://github.com/Byron/theerror/commit/227ef7df7617e7034c4d9cf81b21fbe4744c3210))
    - Merge pull request #108 from ninevra/raw-idents ([`7014b69`](https://github.com/Byron/theerror/commit/7014b69e1a49e3e95db8bdd722c0c5484c46ab99))
    - Prefix raw field named params with "raw_field_" ([`5fc018d`](https://github.com/Byron/theerror/commit/5fc018d195a3fec14a15eda97a882993d349ca4f))
    - Support raw idents in error() format strings ([`9d274d4`](https://github.com/Byron/theerror/commit/9d274d433e4cd588a20154e9ef62104a5599fccf))
    - Pick up more specific string literal error message ([`04d2e6c`](https://github.com/Byron/theerror/commit/04d2e6c998fe6790c9a32c64ce29682fc0159a01))
    - Release 1.0.21 ([`f757a04`](https://github.com/Byron/theerror/commit/f757a0489b2cddfea15ab870b49f159ce1aa71cd))
    - Merge pull request #102 from dtolnay/arc-backtrace-from ([`56a4fa6`](https://github.com/Byron/theerror/commit/56a4fa620b754c86d9b9a166df5ba2597fe68bae))
    - Generalize backtrace capture to From<Backtrace> ([`c6dcc69`](https://github.com/Byron/theerror/commit/c6dcc694baa25b997c5864bc0f6ee618beccb381))
    - Release 1.0.20 ([`42b537a`](https://github.com/Byron/theerror/commit/42b537acf08de385dcf6138f24e3274ff8a18148))
    - Merge pull request #92 from dtolnay/qual ([`bcb60bd`](https://github.com/Byron/theerror/commit/bcb60bd1b5649027f9120535a7a7a5d59e50b9c5))
    - Suppress unused_qualifications lint ([`d0ece37`](https://github.com/Byron/theerror/commit/d0ece37338998b8799674a0cad8187229704550b))
    - Release 1.0.19 ([`8305a8c`](https://github.com/Byron/theerror/commit/8305a8cc8a1024aa657ae17ab65c040865bedd2a))
    - Merge pull request #89 from dtolnay/clippy ([`8ade6a4`](https://github.com/Byron/theerror/commit/8ade6a44207d7fbb2761358883704948b587f58a))
    - Suppress clippy used_underscore_binding pedantic lint ([`ecb595b`](https://github.com/Byron/theerror/commit/ecb595b3892cb5510c8c718ca083221c92c59b5b))
    - Update name of renamed block_in_if_condition_stmt lint ([`54adff8`](https://github.com/Byron/theerror/commit/54adff8fc5cbab7a628ac5222e558812c7cb5c5b))
    - Release 1.0.18 ([`25632e8`](https://github.com/Byron/theerror/commit/25632e8afb175cd3bed58d5723bd8e927c9c6c90))
    - Merge pull request #87 from dtolnay/span ([`72aaea5`](https://github.com/Byron/theerror/commit/72aaea5e1fab7d8f16ce9d94177546cc45e728cc))
    - Inherit span from enum to variants ([`3a49609`](https://github.com/Byron/theerror/commit/3a49609f78e49df944e74776bac1f3010eb2f244))
    - Take a useful span for constructed idents ([`3891d98`](https://github.com/Byron/theerror/commit/3891d985fba6457911ef753dff633151f0c208ae))
    - Release 1.0.17 ([`f79a85f`](https://github.com/Byron/theerror/commit/f79a85f72be0a04644b392acf1ba7f97380c1e4f))
    - Remove CI badge from Cargo.toml ([`186509d`](https://github.com/Byron/theerror/commit/186509da2a7be4346e27a4077e93d6a0d46ec8d5))
    - Release 1.0.16 ([`94e62a8`](https://github.com/Byron/theerror/commit/94e62a81bc6da67efb06a45a8f45caf49768d5e4))
    - Merge pull request #82 from dtolnay/transparent ([`fc3ad54`](https://github.com/Byron/theerror/commit/fc3ad544114dbd4c6d571981088936b2c899c090))
    - Reject transparent attr in field attribute position ([`85b0944`](https://github.com/Byron/theerror/commit/85b0944eac901c7bf9922469439579964a066842))
    - Release 1.0.15 ([`d8d55e6`](https://github.com/Byron/theerror/commit/d8d55e6655fa31a80d8852d7c4146ff1c839c014))
    - Merge pull request #76 from dtolnay/span ([`382445c`](https://github.com/Byron/theerror/commit/382445c05c1afdf254e4ac771e1f7783004ead0b))
    - Improve span of missing Display impl error ([`4885372`](https://github.com/Byron/theerror/commit/4885372cede117b02ba90c908f24530b8628215e))
    - Release 1.0.14 ([`6fd4059`](https://github.com/Byron/theerror/commit/6fd405929807e73ac4b1bd026192a91f64a08636))
    - Merge pull request #73 from dtolnay/deprecated ([`4ede816`](https://github.com/Byron/theerror/commit/4ede8165f19314dae0b131a02cfd970c35f1b130))
    - Allow matches to refer to deprecated variants ([`33166f7`](https://github.com/Byron/theerror/commit/33166f7af43a76021f15c5e5a451364407a0ae9b))
    - Release 1.0.13 ([`14b54d2`](https://github.com/Byron/theerror/commit/14b54d22564bbb6d75ddcc6fc732e1f6cf882311))
    - Fix missing comma between named format elements ([`1b40434`](https://github.com/Byron/theerror/commit/1b404340348b7639a43fbe2d951f0864c5b5c187))
    - Release 1.0.12 ([`e160f5d`](https://github.com/Byron/theerror/commit/e160f5d90878bb81005ba24876d025f88aec066b))
    - Merge pull request #69 from dtolnay/source ([`3612a72`](https://github.com/Byron/theerror/commit/3612a7207e5636f494b253150b94554885680987))
    - Add error message for non-static source ([`a1ae05c`](https://github.com/Byron/theerror/commit/a1ae05c95438cada493dfe74de28125e1c04c850))
    - Select a single docs.rs build target ([`db357fa`](https://github.com/Byron/theerror/commit/db357fac8380b212ac6805741930ad75bcb0d761))
    - Release 1.0.11 ([`55d6fbb`](https://github.com/Byron/theerror/commit/55d6fbb46032887f21e9c387671e01bca5392818))
    - Link license files into impl subcrate ([`0856edd`](https://github.com/Byron/theerror/commit/0856edd777b3ca07df30f5d75d4b57c862e5d350))
    - Release 1.0.10 ([`ccbb2ab`](https://github.com/Byron/theerror/commit/ccbb2ab862cdff9912a438aca9c54d5945e20351))
    - Merge pull request #60 from dtolnay/fmt ([`51f3fb4`](https://github.com/Byron/theerror/commit/51f3fb40f51a520850fad0196a15153800018030))
    - Parse .0 fmt arguments in all valid places ([`5e6ebaf`](https://github.com/Byron/theerror/commit/5e6ebafd8b9e74110ce30778018cfb1d6180b5ec))
    - Display a Travis badge on crates.io ([`5f36dac`](https://github.com/Byron/theerror/commit/5f36dac9b2a47fcbb3f75564f39148225b5550a2))
    - Release 1.0.9 ([`1d0b399`](https://github.com/Byron/theerror/commit/1d0b3999af726e816010d108431492865e6df018))
    - Remove incorrect detection of simple fmt cases ([`20202db`](https://github.com/Byron/theerror/commit/20202db2990a5ab1aa3c6bb0ffa911b6270fd0c0))
    - Release 1.0.8 ([`d53be52`](https://github.com/Byron/theerror/commit/d53be52a3b0236ce34e0cd3b7af8ba8d490b0e7c))
    - Release 1.0.7 ([`79b740e`](https://github.com/Byron/theerror/commit/79b740e3d77d6303798397e94166a48d1ea10edc))
    - Merge pull request #50 from dtolnay/transparent ([`62e8e66`](https://github.com/Byron/theerror/commit/62e8e66bf6891bf5caa5c04d2f6a82aaf7c896ad))
    - Expand transparent attribute ([`c3da163`](https://github.com/Byron/theerror/commit/c3da1630d34a483f522ed0f9eb113a8bc48489b0))
    - Validate transparent attribute ([`7672b1e`](https://github.com/Byron/theerror/commit/7672b1eccdd59a8a8ef856f0ef4c05ff6f8c7f0b))
    - Parse transparent attribute ([`e38d6bd`](https://github.com/Byron/theerror/commit/e38d6bd1a5f0d2932e7f700e6353a42c4a0fe395))
    - Make function for error attribute parsing ([`c3043a1`](https://github.com/Byron/theerror/commit/c3043a18908bb9ba8f1fd7654468df5c591b18a1))
    - Defer handling of trailing comma in display attribute ([`038b8d5`](https://github.com/Byron/theerror/commit/038b8d55348a29b337501b213023bad31f37240c))
    - Accommodate trailing comma in shorthand expansion ([`a960689`](https://github.com/Byron/theerror/commit/a96068939b08586ee8f432a2dfe448967b5ab68a))
    - Only apply int shorthand for tuple fields that exist ([`26fe392`](https://github.com/Byron/theerror/commit/26fe392262267ef918bf54cbc750f2291ec75a2c))
    - Merge pull request #49 from dtolnay/shorthand ([`52b5fb3`](https://github.com/Byron/theerror/commit/52b5fb398ffac294a2d1c963eabb37ea1750325f))
    - Support mixing shorthand and non-shorthand format args ([`c05e9ed`](https://github.com/Byron/theerror/commit/c05e9ed4ecbfd4806541b8fad2eba5bce81bb5e0))
    - Merge pull request #48 from dtolnay/static ([`77c43ea`](https://github.com/Byron/theerror/commit/77c43ea23a9c660d37086c291974d06b4f65450d))
    - Simplify with Index's new IdentFragment impl ([`6dddf44`](https://github.com/Byron/theerror/commit/6dddf444931cb39c668c20fb275e2c8b65a28307))
    - Support referring to statics and consts from shorthand ([`7f8b578`](https://github.com/Byron/theerror/commit/7f8b5784cbb1cf6124aa06978d08986a938e27f5))
    - Pass field list into expand_shorthand ([`6b6857b`](https://github.com/Byron/theerror/commit/6b6857ba893abe5638c96bc6f0f2fcb226831935))
    - Defer expansion of fmt shorthand ([`886baec`](https://github.com/Byron/theerror/commit/886baece011ea297154089f5237504064f938893))
    - Remove shorthand test superseded by integration tests ([`bfc7f8a`](https://github.com/Byron/theerror/commit/bfc7f8a4b10b9470321b641fcc135665bed92fd4))
    - Suppress range_plus_one lint ([`f866fa9`](https://github.com/Byron/theerror/commit/f866fa9a710f1090c53b907b4704e137371120d7))
    - Release 1.0.6 ([`938bcec`](https://github.com/Byron/theerror/commit/938bcec8f963c3e50895d3527a3ecea9d6a83150))
    - Ignore a new clippy lint ([`eb05272`](https://github.com/Byron/theerror/commit/eb052728d4b85a28fae9409fd8beb245d573a050))
    - Release 1.0.5 ([`10b8e58`](https://github.com/Byron/theerror/commit/10b8e5817acdcd052f335c4ea9b6c8909eab76e7))
    - Merge pull request #39 from dtolnay/path ([`2e19391`](https://github.com/Byron/theerror/commit/2e1939158adac24a647fb9057327b5fc32b2964b))
    - Support interpolating paths as if they had a Display impl ([`72cb53e`](https://github.com/Byron/theerror/commit/72cb53e1c78ec544db9fd273f05093919ea0f111))
    - Fill in missing fields of therror-impl unit test ([`e6762d9`](https://github.com/Byron/theerror/commit/e6762d9440aab9e1c387c02334f416b15042284d))
    - Release 1.0.4 ([`5079141`](https://github.com/Byron/theerror/commit/507914148ad4c69837ae42eb7930cb9e4f2fdc5b))
    - Merge pull request 36 from mathstuf:formatter-name-collision ([`12404e4`](https://github.com/Byron/theerror/commit/12404e42a5349aa027410e38d7e348f44c0ea91d))
    - Avoid using `formatter` ([`5ae5b4e`](https://github.com/Byron/theerror/commit/5ae5b4e60e3f764615d89dca9e45db10be732f37))
    - Release 1.0.3 ([`b2b3bae`](https://github.com/Byron/theerror/commit/b2b3bae1db325cd52b79c438d30987e0d19891b4))
    - Allow ident containing number ([`7460283`](https://github.com/Byron/theerror/commit/74602839279d69719f2d6731cc36209d8655cf18))
    - Release 1.0.2 ([`ee864e1`](https://github.com/Byron/theerror/commit/ee864e16419ae8a6e8ded983b53fc4514262d449))
    - Merge pull request #31 from dtolnay/from ([`df94656`](https://github.com/Byron/theerror/commit/df94656ef3fb48665959770bf700b0b647c547a8))
    - Support backtrace in From impl ([`46de723`](https://github.com/Byron/theerror/commit/46de723f206e5e6e93d09fe03d4031405b0540e6))
    - Merge pull request #30 from dtolnay/from ([`f409102`](https://github.com/Byron/theerror/commit/f40910293b1e2595880c2f924939e09b69746f67))
    - Detect extraneous fields before generating From ([`0e8d1ba`](https://github.com/Byron/theerror/commit/0e8d1baa69cd5b727ca4c4266c5777fde3c11275))
    - Detect multiple variants having same From type ([`2492735`](https://github.com/Byron/theerror/commit/2492735b67dfdd2137a8308b35bdbd47ac88c348))
    - Implement From for enums ([`f3b38c3`](https://github.com/Byron/theerror/commit/f3b38c3a8a8f725c7fab687d9fa68d0f4cf50ddf))
    - Implement From for structs ([`946be60`](https://github.com/Byron/theerror/commit/946be606a9ca71f062be9065951a63c14fed3435))
    - Treat the #[from] field implicitly as source ([`03e77b3`](https://github.com/Byron/theerror/commit/03e77b3d2b2d108c044d28601617a087b16c79b5))
    - Validate #[from] attribute ([`2305f75`](https://github.com/Byron/theerror/commit/2305f75976a3076310e340ad18839772ac00e1ad))
    - Parse #[from] attribute ([`8cbc50e`](https://github.com/Byron/theerror/commit/8cbc50e05aea8e0e3c1ea229cf8397f2e6dbfd5b))
    - Rename attr validation methods ([`0c5e34f`](https://github.com/Byron/theerror/commit/0c5e34fb12a093915bd3edeeebe4ce56581e41f8))
    - Simplify storage of source and backtrace attrs ([`f169522`](https://github.com/Byron/theerror/commit/f16952290d38cb066cb01591f2b4a715cb081ee6))
    - Merge pull request #29 from dtolnay/opt ([`cb73bf9`](https://github.com/Byron/theerror/commit/cb73bf9ccc18093b35ff34431b917f8dd6c0ac97))
    - Support Option<Backtrace> ([`fac0a7a`](https://github.com/Byron/theerror/commit/fac0a7aa0d750049b82a0ede61d33e766eb6ac80))
    - Support Option<$source> ([`f18a2a6`](https://github.com/Byron/theerror/commit/f18a2a605d3487939271bd9113a96e9a798249e2))
    - Merge pull request #28 from dtolnay/backtrace ([`c286564`](https://github.com/Byron/theerror/commit/c2865642b039b1a0215d02c0ee26406256ecafb0))
    - Elevate precedence of explicitly marked backtrace ([`2c9bcc5`](https://github.com/Byron/theerror/commit/2c9bcc5347c5dca2f8be2fcdd636160d920c706b))
    - Prefer the source's backtrace if it has one ([`18f2337`](https://github.com/Byron/theerror/commit/18f2337ffcb51a92da8a8bbc0493d40ba5fcd603))
    - #[backtrace] attribute for explicitly selecting the backtrace field ([`72cf49c`](https://github.com/Byron/theerror/commit/72cf49ccd4d36150ec390029ffdf6bc5220a23d6))
    - Improve span for the enum case ([`de6719e`](https://github.com/Byron/theerror/commit/de6719e9231c47ede386f3aaf9906dfcbedeeb14))
    - Improve span when source is not an Error ([`098dcf2`](https://github.com/Byron/theerror/commit/098dcf26c3a557160374492459afbf2482a8f6d8))
    - Merge pull request #27 from dtolnay/source ([`9757677`](https://github.com/Byron/theerror/commit/97576776e75deb96e68877b46f5b75e47c5ffa56))
    - Assume #[source] attribute for fields named `source` ([`6b35d7f`](https://github.com/Byron/theerror/commit/6b35d7f73cfb6cb8e137575184833e1ce3f63930))
    - Merge pull request #26 from dtolnay/void ([`a34af0d`](https://github.com/Byron/theerror/commit/a34af0d6a6ae6023785c93f24c87a4fe0956d612))
    - Support Display impl for void enums ([`5ea03dd`](https://github.com/Byron/theerror/commit/5ea03dd7da0a8eddf0709713fc8e6d9b918385bc))
    - Merge pull request #25 from dtolnay/valid ([`acaa504`](https://github.com/Byron/theerror/commit/acaa504718b1558e42a2a93b8a10feef15879023))
    - Reject display attribute on a field ([`aa123cf`](https://github.com/Byron/theerror/commit/aa123cfe0cafca605eac78ba35ece9e9e656a9ec))
    - Reject source attribute not on a field ([`10d1f64`](https://github.com/Byron/theerror/commit/10d1f640da2ea687c6675f1f79aaf8aae08e26ee))
    - Merge pull request #24 from dtolnay/inherit ([`7c9dcae`](https://github.com/Byron/theerror/commit/7c9dcaebbcb162b04d7872bb8e1148614f47b93d))
    - Variants inherit fmt attr from the enum ([`41068e5`](https://github.com/Byron/theerror/commit/41068e543530a23bf329ed905c821387a185a462))
    - Merge pull request #23 from dtolnay/dup ([`f7b08d4`](https://github.com/Byron/theerror/commit/f7b08d401cf43510e731a1a15f964bdb80943985))
    - Detect duplicate source attributes ([`d42a950`](https://github.com/Byron/theerror/commit/d42a95076c500d959007a6f872726b1907852f0c))
    - Merge pull request #22 from dtolnay/prop ([`ae085b0`](https://github.com/Byron/theerror/commit/ae085b08639681e709c97509de971cec25252881))
    - Move more tree analysis logic to prop.rs ([`68c18d6`](https://github.com/Byron/theerror/commit/68c18d6ecf7d9acfa7788cf50678289062d03ae1))
    - Merge pull request #21 from dtolnay/valid ([`3b27947`](https://github.com/Byron/theerror/commit/3b279475b2b75ced882714e531bba0b06534fd0a))
    - Perform all validation up front ([`1764dde`](https://github.com/Byron/theerror/commit/1764ddeae494c1056c9c2c07d966d14a4011d30e))
    - Merge pull request #20 from dtolnay/ast ([`90020d4`](https://github.com/Byron/theerror/commit/90020d4804241bf051a6514dd85101723275d85e))
    - Record original syntax tree references for error reporting ([`e3f1529`](https://github.com/Byron/theerror/commit/e3f152983c4a45c47eea82cda286934ce0495b3e))
    - Switch expansion to use new ast ([`69052f8`](https://github.com/Byron/theerror/commit/69052f88d81cdace2d3a0f8b7908777866b1d16c))
    - Introduce syntax tree that makes validations easier ([`60987ff`](https://github.com/Byron/theerror/commit/60987ffa76ae3543eddfc9457392cf32e030ef60))
    - Merge pull request #19 from dtolnay/ast ([`0f4f767`](https://github.com/Byron/theerror/commit/0f4f7673dc5beb94dec84c7c46a3166dfd41901d))
    - Restore support for rustc <1.36 ([`5b0d29a`](https://github.com/Byron/theerror/commit/5b0d29a6e5adb63045c66e7c45ed85c6c59f62ec))
    - Detect duplicate attributes on the same syntax tree node ([`b78fe18`](https://github.com/Byron/theerror/commit/b78fe1891b1a68ac76d1044821aea0378595910f))
    - Uniformly parse all attributes in all positions ([`3d43d39`](https://github.com/Byron/theerror/commit/3d43d39ed1abd5dc51db8e56ee2c4ac2a4f932ea))
    - Resolve iter_skip_next lint ([`ab48dd1`](https://github.com/Byron/theerror/commit/ab48dd19d397dbaaf8839bbfc82fdc2e2f948883))
    - Generate more concise code for pass-through format ([`5994705`](https://github.com/Byron/theerror/commit/5994705b387c58620ca8967ce6c6452f28f300ad))
    - Keep track of whether fmt string uses shorthand ([`a6b1d28`](https://github.com/Byron/theerror/commit/a6b1d28e406554bae1bf581832136ced137cd272))
    - Merge pull request #16 from dtolnay/nested ([`02f56e0`](https://github.com/Byron/theerror/commit/02f56e0c49a29e299b7a2c2569b13054c31a4888))
    - Format with rustfmt 2019-09-08 ([`8b31ec6`](https://github.com/Byron/theerror/commit/8b31ec6fd04122432f108c1104ffe3c030c4abf9))
    - Accept `.var` shorthand inside of parens ([`7fa5b81`](https://github.com/Byron/theerror/commit/7fa5b8114def81afb6e6263aa559623a77789d4c))
    - Extract the fmt args parser to a function ([`e50cf82`](https://github.com/Byron/theerror/commit/e50cf82f33e2d24fe7bee91857bdb76fe9896574))
    - Release 1.0.0 ([`86f6b3b`](https://github.com/Byron/theerror/commit/86f6b3bc4a4083434dcafe467f60abda0c2fbfc4))
    - Fill in crates.io metadata ([`7d26b19`](https://github.com/Byron/theerror/commit/7d26b19d07635c62b893ea25d0ef6dd8a5567421))
    - Resolve range_plus_one lint ([`c0d3d56`](https://github.com/Byron/theerror/commit/c0d3d564cfb3cee155b26d36d8b86267d3e6f792))
    - Avoid enum variants through Self ([`490c962`](https://github.com/Byron/theerror/commit/490c962d6864b99a246d0cb42a12b1c68f7255a9))
    - Accept shorthand for the format string ([`95cd986`](https://github.com/Byron/theerror/commit/95cd9864d6d17bd95aa7189bc74a03d0192b3f2d))
    - Implement Display for enums ([`69df195`](https://github.com/Byron/theerror/commit/69df19571ad96a598269e72ddfd2e7a23ad74b80))
    - Implement Display for structs ([`63ba03b`](https://github.com/Byron/theerror/commit/63ba03bacbd24286c19057a409738717f9337339))
    - Error implementation for enums ([`8e866cd`](https://github.com/Byron/theerror/commit/8e866cde576823a73107000168688fc3ce6446ea))
    - Support any source() that derefs to std::error::Error ([`761ff1c`](https://github.com/Byron/theerror/commit/761ff1c7084124a0c5ec94409a7a6dde0ed30857))
    - Improve error position when cause is wrong type ([`bb13f4b`](https://github.com/Byron/theerror/commit/bb13f4b07aa0d4363aa5832ff5ad75da6c2c398a))
    - Unify braced struct and tuple struct field iteration ([`5038953`](https://github.com/Byron/theerror/commit/5038953fd1942bde6591a4bb207e125f2869568f))
    - Add backtrace() method for structs ([`c86452c`](https://github.com/Byron/theerror/commit/c86452cc68c95c28ddba296b270d60c66a91c287))
    - Locate field that has the #[source] attribute ([`4cdeec1`](https://github.com/Byron/theerror/commit/4cdeec15e5aedebd737ba4247f4bfac18ecc45b3))
    - Add source() method for structs ([`f1dcfe0`](https://github.com/Byron/theerror/commit/f1dcfe0f0a557f49c59cb2c152f0836ee50ee6d7))
    - Implement derive for unit struct errors ([`1f02d8d`](https://github.com/Byron/theerror/commit/1f02d8d9fd74f807b0668927cc89dc614edb7555))
    - Begin derive macro implementation ([`c30d847`](https://github.com/Byron/theerror/commit/c30d847847273febb7894bc5cf134049ff4c0b06))
    - Add therror-impl crate to be the proc macro ([`1987e62`](https://github.com/Byron/theerror/commit/1987e628fa9213ab9251cb928f170f0b0a4d431e))
</details>

