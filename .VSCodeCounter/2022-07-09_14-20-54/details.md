# Details

Date : 2022-07-09 14:20:54

Directory c:\\Users\\berys\\rust\\viaulence

Total : 77 files,  1325 codes, 81 comments, 104 blanks, all 1510 lines

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [src/game.rs](/src/game.rs) | Rust | 1 | 0 | 1 | 2 |
| [src/game/gamestate.rs](/src/game/gamestate.rs) | Rust | 84 | 8 | 10 | 102 |
| [src/grid.rs](/src/grid.rs) | Rust | 1 | 0 | 1 | 2 |
| [src/grid/isometric_grid.rs](/src/grid/isometric_grid.rs) | Rust | 144 | 0 | 12 | 156 |
| [src/main.rs](/src/main.rs) | Rust | 12 | 28 | 7 | 47 |
| [src/pieces.rs](/src/pieces.rs) | Rust | 4 | 0 | 1 | 5 |
| [src/pieces/attack.rs](/src/pieces/attack.rs) | Rust | 19 | 2 | 4 | 25 |
| [src/pieces/movement.rs](/src/pieces/movement.rs) | Rust | 518 | 9 | 20 | 547 |
| [src/pieces/tokens.rs](/src/pieces/tokens.rs) | Rust | 342 | 6 | 27 | 375 |
| [src/pieces/traits.rs](/src/pieces/traits.rs) | Rust | 33 | 10 | 11 | 54 |
| [src/player.rs](/src/player.rs) | Rust | 1 | 0 | 1 | 2 |
| [src/player/player.rs](/src/player/player.rs) | Rust | 54 | 14 | 4 | 72 |
| [src/tests.rs](/src/tests.rs) | Rust | 40 | 4 | 5 | 49 |
| [target/.rustc_info.json](/target/.rustc_info.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/cfg-if-146fde0898878878/lib-cfg-if.json](/target/debug/.fingerprint/cfg-if-146fde0898878878/lib-cfg-if.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/cfg-if-1bb00f2c2acb55d3/lib-cfg-if.json](/target/debug/.fingerprint/cfg-if-1bb00f2c2acb55d3/lib-cfg-if.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/cfg-if-a2f7edacd6252f09/lib-cfg-if.json](/target/debug/.fingerprint/cfg-if-a2f7edacd6252f09/lib-cfg-if.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/cfg-if-d5d3c734ee521f19/lib-cfg-if.json](/target/debug/.fingerprint/cfg-if-d5d3c734ee521f19/lib-cfg-if.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/getrandom-08d27a16f670269d/lib-getrandom.json](/target/debug/.fingerprint/getrandom-08d27a16f670269d/lib-getrandom.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/getrandom-3af68d3f870e741d/lib-getrandom.json](/target/debug/.fingerprint/getrandom-3af68d3f870e741d/lib-getrandom.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/getrandom-6cd5bf57d46005b5/lib-getrandom.json](/target/debug/.fingerprint/getrandom-6cd5bf57d46005b5/lib-getrandom.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/getrandom-767ea6e8f65e64bb/lib-getrandom.json](/target/debug/.fingerprint/getrandom-767ea6e8f65e64bb/lib-getrandom.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/ppv-lite86-723d7d867fbf8014/lib-ppv-lite86.json](/target/debug/.fingerprint/ppv-lite86-723d7d867fbf8014/lib-ppv-lite86.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/ppv-lite86-88d06862c3855439/lib-ppv-lite86.json](/target/debug/.fingerprint/ppv-lite86-88d06862c3855439/lib-ppv-lite86.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/ppv-lite86-9cf9422a9083a461/lib-ppv-lite86.json](/target/debug/.fingerprint/ppv-lite86-9cf9422a9083a461/lib-ppv-lite86.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/ppv-lite86-a14fb3007e95d1db/lib-ppv-lite86.json](/target/debug/.fingerprint/ppv-lite86-a14fb3007e95d1db/lib-ppv-lite86.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/proc-macro2-078019beeeaf1849/build-script-build-script-build.json](/target/debug/.fingerprint/proc-macro2-078019beeeaf1849/build-script-build-script-build.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/proc-macro2-17cacf12d1aea2f6/lib-proc-macro2.json](/target/debug/.fingerprint/proc-macro2-17cacf12d1aea2f6/lib-proc-macro2.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/proc-macro2-377d5acae16abea1/lib-proc-macro2.json](/target/debug/.fingerprint/proc-macro2-377d5acae16abea1/lib-proc-macro2.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/proc-macro2-819138a3245e8c6d/run-build-script-build-script-build.json](/target/debug/.fingerprint/proc-macro2-819138a3245e8c6d/run-build-script-build-script-build.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/proc-macro2-c95e6ee3ae09345a/run-build-script-build-script-build.json](/target/debug/.fingerprint/proc-macro2-c95e6ee3ae09345a/run-build-script-build-script-build.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/proc-macro2-e85f1ce21f03058c/build-script-build-script-build.json](/target/debug/.fingerprint/proc-macro2-e85f1ce21f03058c/build-script-build-script-build.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/quote-151168f1c8ffa31e/run-build-script-build-script-build.json](/target/debug/.fingerprint/quote-151168f1c8ffa31e/run-build-script-build-script-build.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/quote-7b1c5979f27ebf31/run-build-script-build-script-build.json](/target/debug/.fingerprint/quote-7b1c5979f27ebf31/run-build-script-build-script-build.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/quote-92fbc1c72d84b93f/build-script-build-script-build.json](/target/debug/.fingerprint/quote-92fbc1c72d84b93f/build-script-build-script-build.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/quote-a8b1327612f8c5a9/lib-quote.json](/target/debug/.fingerprint/quote-a8b1327612f8c5a9/lib-quote.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/quote-cff0c17c4e86d538/build-script-build-script-build.json](/target/debug/.fingerprint/quote-cff0c17c4e86d538/build-script-build-script-build.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/quote-efeb0ee113951780/lib-quote.json](/target/debug/.fingerprint/quote-efeb0ee113951780/lib-quote.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand-012c0cd3a4871f09/lib-rand.json](/target/debug/.fingerprint/rand-012c0cd3a4871f09/lib-rand.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand-7a20481b14930149/lib-rand.json](/target/debug/.fingerprint/rand-7a20481b14930149/lib-rand.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand-ab422469756e526f/lib-rand.json](/target/debug/.fingerprint/rand-ab422469756e526f/lib-rand.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand-f65d428c754b6702/lib-rand.json](/target/debug/.fingerprint/rand-f65d428c754b6702/lib-rand.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand_chacha-7ac7ae1b74401287/lib-rand_chacha.json](/target/debug/.fingerprint/rand_chacha-7ac7ae1b74401287/lib-rand_chacha.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand_chacha-8b168c42868d0925/lib-rand_chacha.json](/target/debug/.fingerprint/rand_chacha-8b168c42868d0925/lib-rand_chacha.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand_chacha-ad85786b6f7ffd49/lib-rand_chacha.json](/target/debug/.fingerprint/rand_chacha-ad85786b6f7ffd49/lib-rand_chacha.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand_chacha-b03c8d46fe245079/lib-rand_chacha.json](/target/debug/.fingerprint/rand_chacha-b03c8d46fe245079/lib-rand_chacha.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand_core-25cf07a154d74aa5/lib-rand_core.json](/target/debug/.fingerprint/rand_core-25cf07a154d74aa5/lib-rand_core.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand_core-2b4019b5b93957c9/lib-rand_core.json](/target/debug/.fingerprint/rand_core-2b4019b5b93957c9/lib-rand_core.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand_core-65ec677ad4b4c489/lib-rand_core.json](/target/debug/.fingerprint/rand_core-65ec677ad4b4c489/lib-rand_core.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/rand_core-d7e8f2d9a28aa458/lib-rand_core.json](/target/debug/.fingerprint/rand_core-d7e8f2d9a28aa458/lib-rand_core.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/syn-3d75e5a36521ba4e/run-build-script-build-script-build.json](/target/debug/.fingerprint/syn-3d75e5a36521ba4e/run-build-script-build-script-build.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/syn-557380ed06c4678f/run-build-script-build-script-build.json](/target/debug/.fingerprint/syn-557380ed06c4678f/run-build-script-build-script-build.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/syn-94e4e136b7597694/lib-syn.json](/target/debug/.fingerprint/syn-94e4e136b7597694/lib-syn.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/syn-b678dd28ff9522e5/build-script-build-script-build.json](/target/debug/.fingerprint/syn-b678dd28ff9522e5/build-script-build-script-build.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/syn-e9218ede8644b035/lib-syn.json](/target/debug/.fingerprint/syn-e9218ede8644b035/lib-syn.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/syn-ed19948ec8d1ac59/build-script-build-script-build.json](/target/debug/.fingerprint/syn-ed19948ec8d1ac59/build-script-build-script-build.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/unicode-ident-0b1ba40d209b295b/lib-unicode-ident.json](/target/debug/.fingerprint/unicode-ident-0b1ba40d209b295b/lib-unicode-ident.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/unicode-ident-299dbb72ba97d2ab/lib-unicode-ident.json](/target/debug/.fingerprint/unicode-ident-299dbb72ba97d2ab/lib-unicode-ident.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/uuid-30c17d8eab3b681b/lib-uuid.json](/target/debug/.fingerprint/uuid-30c17d8eab3b681b/lib-uuid.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/uuid-4088405bd90d84b1/lib-uuid.json](/target/debug/.fingerprint/uuid-4088405bd90d84b1/lib-uuid.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/uuid-9d9b71dcda18ffc4/lib-uuid.json](/target/debug/.fingerprint/uuid-9d9b71dcda18ffc4/lib-uuid.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/uuid-a0ce9330078d818e/lib-uuid.json](/target/debug/.fingerprint/uuid-a0ce9330078d818e/lib-uuid.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/uuid-ac34aa5c5dd49f60/lib-uuid.json](/target/debug/.fingerprint/uuid-ac34aa5c5dd49f60/lib-uuid.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/uuid-b0960f170c10a2dd/lib-uuid.json](/target/debug/.fingerprint/uuid-b0960f170c10a2dd/lib-uuid.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/uuid-macro-internal-1c0b4beab59b5e25/lib-uuid-macro-internal.json](/target/debug/.fingerprint/uuid-macro-internal-1c0b4beab59b5e25/lib-uuid-macro-internal.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/uuid-macro-internal-5c628c26ba80cba2/lib-uuid-macro-internal.json](/target/debug/.fingerprint/uuid-macro-internal-5c628c26ba80cba2/lib-uuid-macro-internal.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/viaulence-07dbc36edad8c039/test-bin-viaulence.json](/target/debug/.fingerprint/viaulence-07dbc36edad8c039/test-bin-viaulence.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/viaulence-17e205c50f476328/bin-viaulence.json](/target/debug/.fingerprint/viaulence-17e205c50f476328/bin-viaulence.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/viaulence-2d557bf703aec4bd/test-bin-viaulence.json](/target/debug/.fingerprint/viaulence-2d557bf703aec4bd/test-bin-viaulence.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/viaulence-42f5cf38944c32f2/test-bin-viaulence.json](/target/debug/.fingerprint/viaulence-42f5cf38944c32f2/test-bin-viaulence.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/viaulence-4c15daa97c12d54f/bin-viaulence.json](/target/debug/.fingerprint/viaulence-4c15daa97c12d54f/bin-viaulence.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/viaulence-5197bdaddbb963ca/test-bin-viaulence.json](/target/debug/.fingerprint/viaulence-5197bdaddbb963ca/test-bin-viaulence.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/viaulence-5ba9e813b2e286a0/bin-viaulence.json](/target/debug/.fingerprint/viaulence-5ba9e813b2e286a0/bin-viaulence.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/viaulence-78169f20d9df8b89/bin-viaulence.json](/target/debug/.fingerprint/viaulence-78169f20d9df8b89/bin-viaulence.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/viaulence-7ff7d8a33a311175/bin-viaulence.json](/target/debug/.fingerprint/viaulence-7ff7d8a33a311175/bin-viaulence.json) | JSON | 1 | 0 | 0 | 1 |
| [target/debug/.fingerprint/viaulence-d0f6ffacb22179f6/test-bin-viaulence.json](/target/debug/.fingerprint/viaulence-d0f6ffacb22179f6/test-bin-viaulence.json) | JSON | 1 | 0 | 0 | 1 |
| [viaulence.code-workspace](/viaulence.code-workspace) | JSON with Comments | 9 | 0 | 0 | 9 |

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)