# Keyboard Ex

Keyboard Ex is a small Elden Ring DLL that mirrors keyboard and XInput gamepad
button state to player Speffects. It is intended as a simple bridge for
parameter-side logic, TAE/EzState conditions, and other DLLs that want to react
to extra inputs without adding a full input system of their own.

The DLL only applies Speffects while the game window is focused. When a mapped
input is released, the corresponding Speffect is removed from the player.

## Build

```powershell
cargo build --package keyboard-ex --release --target x86_64-pc-windows-msvc
```

Output:

```text
target\x86_64-pc-windows-msvc\release\keyboard_ex.dll
```

## Runtime Behavior

- Starts from `DllMain`.
- Registers a recurring task at `CSTaskGroupIndex::FrameBegin`.
- Checks foreground window ownership before accepting input.
- Reads keyboard state with `GetAsyncKeyState`.
- Reads controller 1 with `XInputGetState(0, ...)`.
- Applies mapped Speffects to the main player while input is held.
- Removes mapped Speffects when input is released or the player/world is not
  available.

## Compatibility

The vendored Elden Ring runtime currently recognizes:

- WW `2.6.2.0`
- WW `2.7.0.0`
- JP `2.6.2.1`

The WW `2.7.0.0` profile updates the RVAs used by Keyboard Ex for task
registration and player Speffect application/removal.

## Keyboard Speffect Map

### Letters

| Key | Speffect |
| --- | ---: |
| A | `899000` |
| B | `899001` |
| C | `899002` |
| D | `899003` |
| E | `899004` |
| F | `899005` |
| G | `899006` |
| H | `899007` |
| I | `899008` |
| J | `899009` |
| K | `899010` |
| L | `899011` |
| M | `899012` |
| N | `899013` |
| O | `899014` |
| P | `899015` |
| Q | `899016` |
| R | `899017` |
| S | `899018` |
| T | `899019` |
| U | `899020` |
| V | `899021` |
| W | `899022` |
| X | `899023` |
| Y | `899024` |
| Z | `899025` |

### Number Row

| Key | Speffect |
| --- | ---: |
| 0 | `899100` |
| 1 | `899101` |
| 2 | `899102` |
| 3 | `899103` |
| 4 | `899104` |
| 5 | `899105` |
| 6 | `899106` |
| 7 | `899107` |
| 8 | `899108` |
| 9 | `899109` |

### Arrow Keys

| Key | Speffect |
| --- | ---: |
| Left | `899200` |
| Up | `899201` |
| Right | `899202` |
| Down | `899203` |

### Function Keys

| Key | Speffect |
| --- | ---: |
| F1 | `899400` |
| F2 | `899401` |
| F3 | `899402` |
| F4 | `899403` |
| F5 | `899404` |
| F6 | `899405` |
| F7 | `899406` |
| F8 | `899407` |
| F9 | `899408` |
| F10 | `899409` |
| F11 | `899410` |
| F12 | `899411` |

### Common Keys

| Key | Speffect |
| --- | ---: |
| Space | `899500` |
| Enter | `899501` |
| Escape | `899502` |
| Backspace | `899503` |
| Tab | `899504` |
| Shift | `899505` |
| Ctrl | `899506` |
| Alt | `899507` |

### Punctuation Keys

These use Windows virtual-key codes and therefore follow the active keyboard
layout.

| Virtual key | Common US key | Speffect |
| ---: | --- | ---: |
| `0xBA` | `;` | `899600` |
| `0xBB` | `=` | `899601` |
| `0xBC` | `,` | `899602` |
| `0xBD` | `-` | `899603` |
| `0xBE` | `.` | `899604` |
| `0xBF` | `/` | `899605` |
| `0xC0` | `` ` `` | `899606` |
| `0xDB` | `[` | `899607` |
| `0xDC` | `\` | `899608` |
| `0xDD` | `]` | `899609` |
| `0xDE` | `'` | `899610` |

## Gamepad Speffect Map

| XInput input | Speffect |
| --- | ---: |
| A | `899700` |
| B | `899701` |
| X | `899702` |
| Y | `899703` |
| Left shoulder | `899704` |
| Right shoulder | `899705` |
| Left trigger | `899706` |
| Right trigger | `899707` |
| Back | `899708` |
| Start | `899709` |
| Left thumb | `899710` |
| Right thumb | `899711` |
| D-pad up | `899712` |
| D-pad down | `899713` |
| D-pad left | `899714` |
| D-pad right | `899715` |

Trigger threshold is `30`.

## Repository Layout

This repository is a trimmed workspace containing only the crates required by
`examples/keyboard-ex`:

- `crates/shared`
- `crates/eldenring`
- `examples/keyboard-ex`

## Credits

Special thanks to:

- `Vizzo` - The inspiration and creation of the functions come from him, the lua file is from him.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-ASL2](LICENSE-ASL2))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Building with private ShikiFSRS

Cargo pins `KamiyamaShiki0704/ShikiFSRS` to `930f818431e72209b49d2e37265a1f8150fa5390`. This SDK repository is private: the build account needs repository read access and Git credentials. `.cargo/config.toml` enables Git CLI fetching; no token is stored in this repository.

The migration was checked offline against ER WW 1.17.1 (EXE 2.7.1.0). Native engine ownership, phase/lifetime requirements and unexecuted gameplay behavior remain experimental and require manual acceptance. Existing ignored game/asset fixtures were not executed.
