use std::time::Duration;

use eldenring::{
    cs::{CSTaskGroupIndex, CSTaskImp, ChrInsExt, WorldChrMan},
    fd4::FD4TaskData,
};
use fromsoftware_shared::{FromStatic, SharedTaskImpExt};
use windows::Win32::{
    System::Threading::GetCurrentProcessId,
    UI::{
        Input::{
            KeyboardAndMouse::GetAsyncKeyState,
            XboxController::{
                XINPUT_GAMEPAD_A, XINPUT_GAMEPAD_B, XINPUT_GAMEPAD_BACK, XINPUT_GAMEPAD_DPAD_DOWN,
                XINPUT_GAMEPAD_DPAD_LEFT, XINPUT_GAMEPAD_DPAD_RIGHT, XINPUT_GAMEPAD_DPAD_UP,
                XINPUT_GAMEPAD_LEFT_SHOULDER, XINPUT_GAMEPAD_LEFT_THUMB,
                XINPUT_GAMEPAD_RIGHT_SHOULDER, XINPUT_GAMEPAD_RIGHT_THUMB, XINPUT_GAMEPAD_START,
                XINPUT_GAMEPAD_X, XINPUT_GAMEPAD_Y, XINPUT_STATE, XInputGetState,
            },
        },
        WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId},
    },
};

const XINPUT_SUCCESS: u32 = 0;
const TRIGGER_THRESHOLD: u8 = 30;

#[derive(Clone, Copy)]
enum Input {
    Key(i32),
    GamepadButton(u16),
    GamepadLeftTrigger,
    GamepadRightTrigger,
}

#[derive(Clone, Copy)]
struct InputEffect {
    sp_effect: i32,
    input: Input,
}

const INPUT_EFFECTS: &[InputEffect] = &[
    InputEffect {
        sp_effect: 899000,
        input: Input::Key(0x41),
    },
    InputEffect {
        sp_effect: 899001,
        input: Input::Key(0x42),
    },
    InputEffect {
        sp_effect: 899002,
        input: Input::Key(0x43),
    },
    InputEffect {
        sp_effect: 899003,
        input: Input::Key(0x44),
    },
    InputEffect {
        sp_effect: 899004,
        input: Input::Key(0x45),
    },
    InputEffect {
        sp_effect: 899005,
        input: Input::Key(0x46),
    },
    InputEffect {
        sp_effect: 899006,
        input: Input::Key(0x47),
    },
    InputEffect {
        sp_effect: 899007,
        input: Input::Key(0x48),
    },
    InputEffect {
        sp_effect: 899008,
        input: Input::Key(0x49),
    },
    InputEffect {
        sp_effect: 899009,
        input: Input::Key(0x4A),
    },
    InputEffect {
        sp_effect: 899010,
        input: Input::Key(0x4B),
    },
    InputEffect {
        sp_effect: 899011,
        input: Input::Key(0x4C),
    },
    InputEffect {
        sp_effect: 899012,
        input: Input::Key(0x4D),
    },
    InputEffect {
        sp_effect: 899013,
        input: Input::Key(0x4E),
    },
    InputEffect {
        sp_effect: 899014,
        input: Input::Key(0x4F),
    },
    InputEffect {
        sp_effect: 899015,
        input: Input::Key(0x50),
    },
    InputEffect {
        sp_effect: 899016,
        input: Input::Key(0x51),
    },
    InputEffect {
        sp_effect: 899017,
        input: Input::Key(0x52),
    },
    InputEffect {
        sp_effect: 899018,
        input: Input::Key(0x53),
    },
    InputEffect {
        sp_effect: 899019,
        input: Input::Key(0x54),
    },
    InputEffect {
        sp_effect: 899020,
        input: Input::Key(0x55),
    },
    InputEffect {
        sp_effect: 899021,
        input: Input::Key(0x56),
    },
    InputEffect {
        sp_effect: 899022,
        input: Input::Key(0x57),
    },
    InputEffect {
        sp_effect: 899023,
        input: Input::Key(0x58),
    },
    InputEffect {
        sp_effect: 899024,
        input: Input::Key(0x59),
    },
    InputEffect {
        sp_effect: 899025,
        input: Input::Key(0x5A),
    },
    InputEffect {
        sp_effect: 899100,
        input: Input::Key(0x30),
    },
    InputEffect {
        sp_effect: 899101,
        input: Input::Key(0x31),
    },
    InputEffect {
        sp_effect: 899102,
        input: Input::Key(0x32),
    },
    InputEffect {
        sp_effect: 899103,
        input: Input::Key(0x33),
    },
    InputEffect {
        sp_effect: 899104,
        input: Input::Key(0x34),
    },
    InputEffect {
        sp_effect: 899105,
        input: Input::Key(0x35),
    },
    InputEffect {
        sp_effect: 899106,
        input: Input::Key(0x36),
    },
    InputEffect {
        sp_effect: 899107,
        input: Input::Key(0x37),
    },
    InputEffect {
        sp_effect: 899108,
        input: Input::Key(0x38),
    },
    InputEffect {
        sp_effect: 899109,
        input: Input::Key(0x39),
    },
    InputEffect {
        sp_effect: 899200,
        input: Input::Key(0x25),
    },
    InputEffect {
        sp_effect: 899201,
        input: Input::Key(0x26),
    },
    InputEffect {
        sp_effect: 899202,
        input: Input::Key(0x27),
    },
    InputEffect {
        sp_effect: 899203,
        input: Input::Key(0x28),
    },
    InputEffect {
        sp_effect: 899400,
        input: Input::Key(0x70),
    },
    InputEffect {
        sp_effect: 899401,
        input: Input::Key(0x71),
    },
    InputEffect {
        sp_effect: 899402,
        input: Input::Key(0x72),
    },
    InputEffect {
        sp_effect: 899403,
        input: Input::Key(0x73),
    },
    InputEffect {
        sp_effect: 899404,
        input: Input::Key(0x74),
    },
    InputEffect {
        sp_effect: 899405,
        input: Input::Key(0x75),
    },
    InputEffect {
        sp_effect: 899406,
        input: Input::Key(0x76),
    },
    InputEffect {
        sp_effect: 899407,
        input: Input::Key(0x77),
    },
    InputEffect {
        sp_effect: 899408,
        input: Input::Key(0x78),
    },
    InputEffect {
        sp_effect: 899409,
        input: Input::Key(0x79),
    },
    InputEffect {
        sp_effect: 899410,
        input: Input::Key(0x7A),
    },
    InputEffect {
        sp_effect: 899411,
        input: Input::Key(0x7B),
    },
    InputEffect {
        sp_effect: 899500,
        input: Input::Key(0x20),
    },
    InputEffect {
        sp_effect: 899501,
        input: Input::Key(0x0D),
    },
    InputEffect {
        sp_effect: 899502,
        input: Input::Key(0x1B),
    },
    InputEffect {
        sp_effect: 899503,
        input: Input::Key(0x08),
    },
    InputEffect {
        sp_effect: 899504,
        input: Input::Key(0x09),
    },
    InputEffect {
        sp_effect: 899505,
        input: Input::Key(0x10),
    },
    InputEffect {
        sp_effect: 899506,
        input: Input::Key(0x11),
    },
    InputEffect {
        sp_effect: 899507,
        input: Input::Key(0x12),
    },
    InputEffect {
        sp_effect: 899600,
        input: Input::Key(0xBA),
    },
    InputEffect {
        sp_effect: 899601,
        input: Input::Key(0xBB),
    },
    InputEffect {
        sp_effect: 899602,
        input: Input::Key(0xBC),
    },
    InputEffect {
        sp_effect: 899603,
        input: Input::Key(0xBD),
    },
    InputEffect {
        sp_effect: 899604,
        input: Input::Key(0xBE),
    },
    InputEffect {
        sp_effect: 899605,
        input: Input::Key(0xBF),
    },
    InputEffect {
        sp_effect: 899606,
        input: Input::Key(0xC0),
    },
    InputEffect {
        sp_effect: 899607,
        input: Input::Key(0xDB),
    },
    InputEffect {
        sp_effect: 899608,
        input: Input::Key(0xDC),
    },
    InputEffect {
        sp_effect: 899609,
        input: Input::Key(0xDD),
    },
    InputEffect {
        sp_effect: 899610,
        input: Input::Key(0xDE),
    },
    InputEffect {
        sp_effect: 899700,
        input: Input::GamepadButton(XINPUT_GAMEPAD_A.0),
    },
    InputEffect {
        sp_effect: 899701,
        input: Input::GamepadButton(XINPUT_GAMEPAD_B.0),
    },
    InputEffect {
        sp_effect: 899702,
        input: Input::GamepadButton(XINPUT_GAMEPAD_X.0),
    },
    InputEffect {
        sp_effect: 899703,
        input: Input::GamepadButton(XINPUT_GAMEPAD_Y.0),
    },
    InputEffect {
        sp_effect: 899704,
        input: Input::GamepadButton(XINPUT_GAMEPAD_LEFT_SHOULDER.0),
    },
    InputEffect {
        sp_effect: 899705,
        input: Input::GamepadButton(XINPUT_GAMEPAD_RIGHT_SHOULDER.0),
    },
    InputEffect {
        sp_effect: 899706,
        input: Input::GamepadLeftTrigger,
    },
    InputEffect {
        sp_effect: 899707,
        input: Input::GamepadRightTrigger,
    },
    InputEffect {
        sp_effect: 899708,
        input: Input::GamepadButton(XINPUT_GAMEPAD_BACK.0),
    },
    InputEffect {
        sp_effect: 899709,
        input: Input::GamepadButton(XINPUT_GAMEPAD_START.0),
    },
    InputEffect {
        sp_effect: 899710,
        input: Input::GamepadButton(XINPUT_GAMEPAD_LEFT_THUMB.0),
    },
    InputEffect {
        sp_effect: 899711,
        input: Input::GamepadButton(XINPUT_GAMEPAD_RIGHT_THUMB.0),
    },
    InputEffect {
        sp_effect: 899712,
        input: Input::GamepadButton(XINPUT_GAMEPAD_DPAD_UP.0),
    },
    InputEffect {
        sp_effect: 899713,
        input: Input::GamepadButton(XINPUT_GAMEPAD_DPAD_DOWN.0),
    },
    InputEffect {
        sp_effect: 899714,
        input: Input::GamepadButton(XINPUT_GAMEPAD_DPAD_LEFT.0),
    },
    InputEffect {
        sp_effect: 899715,
        input: Input::GamepadButton(XINPUT_GAMEPAD_DPAD_RIGHT.0),
    },
];

/// # Safety
/// Exposed for Windows LoadLibrary. Do not call directly.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    if reason != 1 {
        return true;
    }

    std::thread::spawn(|| {
        let Ok(cs_task) = CSTaskImp::wait_for_instance(Duration::MAX) else {
            return;
        };

        let mut active = vec![false; INPUT_EFFECTS.len()];

        cs_task.run_recurring(
            move |_: &FD4TaskData| {
                let focused = is_game_focused();
                let gamepad = read_gamepad_state();

                let Ok(world_chr_man) = (unsafe { WorldChrMan::instance_mut() }) else {
                    active.fill(false);
                    return;
                };

                let Some(ref mut main_player) = world_chr_man.main_player else {
                    active.fill(false);
                    return;
                };

                for (index, entry) in INPUT_EFFECTS.iter().enumerate() {
                    let pressed = focused && is_input_down(entry.input, gamepad.as_ref());
                    if pressed {
                        main_player.apply_speffect(entry.sp_effect, true);
                        active[index] = true;
                    } else if active[index] {
                        main_player.chr_ins.remove_speffect(entry.sp_effect);
                        active[index] = false;
                    }
                }
            },
            CSTaskGroupIndex::FrameBegin,
        );
    });

    true
}

fn is_game_focused() -> bool {
    unsafe {
        let foreground = GetForegroundWindow();
        if foreground.0.is_null() {
            return false;
        }

        let mut process_id = 0;
        GetWindowThreadProcessId(foreground, Some(&mut process_id));
        process_id == GetCurrentProcessId()
    }
}

fn read_gamepad_state() -> Option<XINPUT_STATE> {
    let mut state = XINPUT_STATE::default();
    let result = unsafe { XInputGetState(0, &mut state) };
    (result == XINPUT_SUCCESS).then_some(state)
}

fn is_input_down(input: Input, gamepad: Option<&XINPUT_STATE>) -> bool {
    match input {
        Input::Key(vk) => unsafe { GetAsyncKeyState(vk) & i16::MIN != 0 },
        Input::GamepadButton(button) => gamepad
            .map(|state| state.Gamepad.wButtons.0 & button != 0)
            .unwrap_or(false),
        Input::GamepadLeftTrigger => gamepad
            .map(|state| state.Gamepad.bLeftTrigger > TRIGGER_THRESHOLD)
            .unwrap_or(false),
        Input::GamepadRightTrigger => gamepad
            .map(|state| state.Gamepad.bRightTrigger > TRIGGER_THRESHOLD)
            .unwrap_or(false),
    }
}
