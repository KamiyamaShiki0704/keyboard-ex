//! Consumer-contract fixtures from this repository's owned 87-entry baseline, not SDK internals.
use shikifsrs_elden_ring::{
    Ww171File,
    character_sets::CharacterIdentity,
    effects::{EffectApplication, EffectEngine, EffectId, InputEffectController},
    keyboard_effects::keyboard_ex_bindings,
};
use shikifsrs_runtime::{
    Error, Result,
    input::{
        ForegroundWindow, GamepadButton, GamepadState, Input, InputPoller, InputSource, VirtualKey,
    },
};
use std::cell::{Cell, RefCell};

fn expected() -> Vec<(i32, Input)> {
    vec![
        (899000, Input::Key(VirtualKey::new(0x41).unwrap())),
        (899001, Input::Key(VirtualKey::new(0x42).unwrap())),
        (899002, Input::Key(VirtualKey::new(0x43).unwrap())),
        (899003, Input::Key(VirtualKey::new(0x44).unwrap())),
        (899004, Input::Key(VirtualKey::new(0x45).unwrap())),
        (899005, Input::Key(VirtualKey::new(0x46).unwrap())),
        (899006, Input::Key(VirtualKey::new(0x47).unwrap())),
        (899007, Input::Key(VirtualKey::new(0x48).unwrap())),
        (899008, Input::Key(VirtualKey::new(0x49).unwrap())),
        (899009, Input::Key(VirtualKey::new(0x4A).unwrap())),
        (899010, Input::Key(VirtualKey::new(0x4B).unwrap())),
        (899011, Input::Key(VirtualKey::new(0x4C).unwrap())),
        (899012, Input::Key(VirtualKey::new(0x4D).unwrap())),
        (899013, Input::Key(VirtualKey::new(0x4E).unwrap())),
        (899014, Input::Key(VirtualKey::new(0x4F).unwrap())),
        (899015, Input::Key(VirtualKey::new(0x50).unwrap())),
        (899016, Input::Key(VirtualKey::new(0x51).unwrap())),
        (899017, Input::Key(VirtualKey::new(0x52).unwrap())),
        (899018, Input::Key(VirtualKey::new(0x53).unwrap())),
        (899019, Input::Key(VirtualKey::new(0x54).unwrap())),
        (899020, Input::Key(VirtualKey::new(0x55).unwrap())),
        (899021, Input::Key(VirtualKey::new(0x56).unwrap())),
        (899022, Input::Key(VirtualKey::new(0x57).unwrap())),
        (899023, Input::Key(VirtualKey::new(0x58).unwrap())),
        (899024, Input::Key(VirtualKey::new(0x59).unwrap())),
        (899025, Input::Key(VirtualKey::new(0x5A).unwrap())),
        (899100, Input::Key(VirtualKey::new(0x30).unwrap())),
        (899101, Input::Key(VirtualKey::new(0x31).unwrap())),
        (899102, Input::Key(VirtualKey::new(0x32).unwrap())),
        (899103, Input::Key(VirtualKey::new(0x33).unwrap())),
        (899104, Input::Key(VirtualKey::new(0x34).unwrap())),
        (899105, Input::Key(VirtualKey::new(0x35).unwrap())),
        (899106, Input::Key(VirtualKey::new(0x36).unwrap())),
        (899107, Input::Key(VirtualKey::new(0x37).unwrap())),
        (899108, Input::Key(VirtualKey::new(0x38).unwrap())),
        (899109, Input::Key(VirtualKey::new(0x39).unwrap())),
        (899200, Input::Key(VirtualKey::new(0x25).unwrap())),
        (899201, Input::Key(VirtualKey::new(0x26).unwrap())),
        (899202, Input::Key(VirtualKey::new(0x27).unwrap())),
        (899203, Input::Key(VirtualKey::new(0x28).unwrap())),
        (899400, Input::Key(VirtualKey::new(0x70).unwrap())),
        (899401, Input::Key(VirtualKey::new(0x71).unwrap())),
        (899402, Input::Key(VirtualKey::new(0x72).unwrap())),
        (899403, Input::Key(VirtualKey::new(0x73).unwrap())),
        (899404, Input::Key(VirtualKey::new(0x74).unwrap())),
        (899405, Input::Key(VirtualKey::new(0x75).unwrap())),
        (899406, Input::Key(VirtualKey::new(0x76).unwrap())),
        (899407, Input::Key(VirtualKey::new(0x77).unwrap())),
        (899408, Input::Key(VirtualKey::new(0x78).unwrap())),
        (899409, Input::Key(VirtualKey::new(0x79).unwrap())),
        (899410, Input::Key(VirtualKey::new(0x7A).unwrap())),
        (899411, Input::Key(VirtualKey::new(0x7B).unwrap())),
        (899500, Input::Key(VirtualKey::new(0x20).unwrap())),
        (899501, Input::Key(VirtualKey::new(0x0D).unwrap())),
        (899502, Input::Key(VirtualKey::new(0x1B).unwrap())),
        (899503, Input::Key(VirtualKey::new(0x08).unwrap())),
        (899504, Input::Key(VirtualKey::new(0x09).unwrap())),
        (899505, Input::Key(VirtualKey::new(0x10).unwrap())),
        (899506, Input::Key(VirtualKey::new(0x11).unwrap())),
        (899507, Input::Key(VirtualKey::new(0x12).unwrap())),
        (899600, Input::Key(VirtualKey::new(0xBA).unwrap())),
        (899601, Input::Key(VirtualKey::new(0xBB).unwrap())),
        (899602, Input::Key(VirtualKey::new(0xBC).unwrap())),
        (899603, Input::Key(VirtualKey::new(0xBD).unwrap())),
        (899604, Input::Key(VirtualKey::new(0xBE).unwrap())),
        (899605, Input::Key(VirtualKey::new(0xBF).unwrap())),
        (899606, Input::Key(VirtualKey::new(0xC0).unwrap())),
        (899607, Input::Key(VirtualKey::new(0xDB).unwrap())),
        (899608, Input::Key(VirtualKey::new(0xDC).unwrap())),
        (899609, Input::Key(VirtualKey::new(0xDD).unwrap())),
        (899610, Input::Key(VirtualKey::new(0xDE).unwrap())),
        (899700, Input::Button(GamepadButton::new(0x1000).unwrap())),
        (899701, Input::Button(GamepadButton::new(0x2000).unwrap())),
        (899702, Input::Button(GamepadButton::new(0x4000).unwrap())),
        (899703, Input::Button(GamepadButton::new(0x8000).unwrap())),
        (899704, Input::Button(GamepadButton::new(0x100).unwrap())),
        (899705, Input::Button(GamepadButton::new(0x200).unwrap())),
        (899706, Input::LeftTrigger),
        (899707, Input::RightTrigger),
        (899708, Input::Button(GamepadButton::new(0x20).unwrap())),
        (899709, Input::Button(GamepadButton::new(0x10).unwrap())),
        (899710, Input::Button(GamepadButton::new(0x40).unwrap())),
        (899711, Input::Button(GamepadButton::new(0x80).unwrap())),
        (899712, Input::Button(GamepadButton::new(0x1).unwrap())),
        (899713, Input::Button(GamepadButton::new(0x2).unwrap())),
        (899714, Input::Button(GamepadButton::new(0x4).unwrap())),
        (899715, Input::Button(GamepadButton::new(0x8).unwrap())),
    ]
}

struct Source {
    input: Option<Input>,
    pid: u32,
    trigger: u8,
    connected: bool,
    samples: Cell<usize>,
}
impl Source {
    fn selected(input: Input) -> Self {
        Self {
            input: Some(input),
            pid: 42,
            trigger: 31,
            connected: true,
            samples: Cell::new(0),
        }
    }
}
impl InputSource for Source {
    fn foreground(&self) -> Result<Option<ForegroundWindow>> {
        Ok(Some(ForegroundWindow {
            handle: 1,
            process_id: self.pid,
        }))
    }
    fn key_state(&self, key: VirtualKey) -> Result<i16> {
        self.samples.set(self.samples.get() + 1);
        // Low history bit alone must not apply another key's effect.
        Ok(if self.input == Some(Input::Key(key)) {
            i16::MIN
        } else {
            1
        })
    }
    fn gamepad(&self, index: u8) -> Result<Option<GamepadState>> {
        assert_eq!(index, 0, "consumer always uses pad 0");
        self.samples.set(self.samples.get() + 1);
        Ok(self.connected.then(|| GamepadState {
            buttons: match self.input {
                Some(Input::Button(button)) => button.mask(),
                _ => 0,
            },
            left_trigger: if self.input == Some(Input::LeftTrigger) {
                self.trigger
            } else {
                0
            },
            right_trigger: if self.input == Some(Input::RightTrigger) {
                self.trigger
            } else {
                0
            },
            ..GamepadState::default()
        }))
    }
}
#[derive(Debug, PartialEq, Eq)]
enum Call {
    Apply(CharacterIdentity, i32, bool),
    Remove(CharacterIdentity, i32),
}
#[derive(Default)]
struct Engine {
    calls: RefCell<Vec<Call>>,
    fail_remove: Cell<bool>,
}
// SAFETY: Original recording fixture; it never reads target memory or invokes a game function.
unsafe impl EffectEngine for Engine {
    unsafe fn apply(
        &self,
        target: CharacterIdentity,
        effect: EffectId,
        force: bool,
    ) -> Result<EffectApplication> {
        self.calls
            .borrow_mut()
            .push(Call::Apply(target, effect.value(), force));
        Ok(EffectApplication::Submitted)
    }
    unsafe fn remove(&self, target: CharacterIdentity, effect: EffectId) -> Result<bool> {
        self.calls
            .borrow_mut()
            .push(Call::Remove(target, effect.value()));
        if self.fail_remove.replace(false) {
            Err(Error::NativeOperationRejected(
                "original fixture remove failure",
            ))
        } else {
            Ok(true)
        }
    }
}
fn identity() -> CharacterIdentity {
    CharacterIdentity {
        world_address: 0x1000,
        address: 0x2000,
        native_identifier: 0x3456,
        epoch: 1,
    }
}
fn poller() -> InputPoller {
    InputPoller::new(
        42,
        keyboard_ex_bindings()
            .unwrap()
            .iter()
            .map(|binding| binding.input),
        0,
        30,
    )
    .unwrap()
}
fn update(
    controller: &mut InputEffectController,
    engine: &Engine,
    source: &Source,
    target: Option<CharacterIdentity>,
) -> shikifsrs_elden_ring::effects::EffectFrameReport {
    // SAFETY: This records pure consumer-contract operations on copied identities only.
    unsafe { controller.update(engine, target, poller().poll(source)) }
}

#[test]
fn all_baseline_mappings_preserve_held_reapplication_and_release_once() {
    let bindings = keyboard_ex_bindings().unwrap();
    assert_eq!(bindings.len(), 87);
    assert_eq!(
        bindings
            .iter()
            .map(|binding| (binding.effect.value(), binding.input))
            .collect::<Vec<_>>(),
        expected()
    );
    for (effect, input) in expected() {
        let engine = Engine::default();
        let mut controller = InputEffectController::new(bindings.iter().copied()).unwrap();
        let mut source = Source::selected(input);
        for _ in 0..2 {
            let report = update(&mut controller, &engine, &source, Some(identity()));
            assert_eq!(report.submitted, vec![EffectId::new(effect).unwrap()]);
            assert!(report.failures.is_empty());
        }
        source.input = None;
        assert_eq!(
            update(&mut controller, &engine, &source, Some(identity())).cleared,
            vec![EffectId::new(effect).unwrap()]
        );
        assert!(
            update(&mut controller, &engine, &source, Some(identity()))
                .cleared
                .is_empty()
        );
        assert!(controller.is_idle());
        assert_eq!(
            *engine.calls.borrow(),
            vec![
                Call::Apply(identity(), effect, true),
                Call::Apply(identity(), effect, true),
                Call::Remove(identity(), effect)
            ]
        );
    }
}

#[test]
fn trigger_threshold_disconnect_and_foreground_pid_preserve_cleanup() {
    for input in [Input::LeftTrigger, Input::RightTrigger] {
        let engine = Engine::default();
        let mut controller = InputEffectController::new(keyboard_ex_bindings().unwrap()).unwrap();
        let mut source = Source::selected(input);
        source.trigger = 30;
        assert!(
            update(&mut controller, &engine, &source, Some(identity()))
                .submitted
                .is_empty()
        );
        source.trigger = 31;
        assert_eq!(
            update(&mut controller, &engine, &source, Some(identity()))
                .submitted
                .len(),
            1
        );
        source.connected = false;
        assert_eq!(
            update(&mut controller, &engine, &source, Some(identity()))
                .cleared
                .len(),
            1
        );
        source.connected = true;
        assert_eq!(
            update(&mut controller, &engine, &source, Some(identity()))
                .submitted
                .len(),
            1
        );
        source.pid = 99;
        source.samples.set(0);
        assert_eq!(
            update(&mut controller, &engine, &source, Some(identity()))
                .cleared
                .len(),
            1
        );
        assert_eq!(
            source.samples.get(),
            0,
            "focus loss must clean without input sampling"
        );
        assert!(controller.is_idle());
    }
}

#[test]
fn failed_cleanup_retries_and_replaced_player_never_calls_old_target() {
    let bindings = keyboard_ex_bindings().unwrap();
    let mut controller = InputEffectController::new(bindings).unwrap();
    let engine = Engine::default();
    let mut source = Source::selected(Input::Key(VirtualKey::new(0x41).unwrap()));
    update(&mut controller, &engine, &source, Some(identity()));
    source.pid = 99;
    engine.fail_remove.set(true);
    let failed = update(&mut controller, &engine, &source, Some(identity()));
    assert_eq!(failed.cleanup_pending, vec![EffectId::new(899000).unwrap()]);
    assert_eq!(failed.failures.len(), 1);
    assert_eq!(
        update(&mut controller, &engine, &source, Some(identity()))
            .cleared
            .len(),
        1
    );
    source.pid = 42;
    update(&mut controller, &engine, &source, Some(identity()));
    let before = engine.calls.borrow().len();
    let missing = update(&mut controller, &engine, &source, None);
    assert_eq!(missing.abandoned, vec![EffectId::new(899000).unwrap()]);
    assert_eq!(engine.calls.borrow().len(), before);
    let replacement = CharacterIdentity {
        native_identifier: 0x9999,
        epoch: 2,
        ..identity()
    };
    assert_eq!(
        update(&mut controller, &engine, &source, Some(replacement))
            .submitted
            .len(),
        1
    );
    assert_eq!(
        engine.calls.borrow().last(),
        Some(&Call::Apply(replacement, 899000, true))
    );
}

#[test]
fn unsupported_file_and_uninstalled_exports_make_no_game_or_input_calls() {
    assert!(Ww171File::verify(b"original unknown executable fixture").is_err());
    assert_eq!(super::ShikiKeyboardStatus(), 0);
    // SAFETY: Uninstalled slot rejects without a native call or publication.
    assert_eq!(unsafe { super::ShikiKeyboardNotifyEpoch(1) }, 1);
}
