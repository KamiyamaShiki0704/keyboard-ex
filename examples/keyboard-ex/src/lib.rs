//! Fixed WW 1.17.1 candidate. Game-runtime equivalence remains unaccepted.
//! The host retains this DLL from LoadLibrary through installation, then the SDK pins it.
//! Registration/native allocator phase and every future FrameBegin's effect/PARAM/target
//! lifetime remain experimental host obligations. No hot unloading is supported.
#![deny(unsafe_op_in_unsafe_fn)]
use shikifsrs_elden_ring::{TaskPhase, Ww171File, input_effect_resident::ResidentInputEffectSlot};
use shikifsrs_runtime::{Result, memory::CurrentProcess};
use std::{
    ffi::c_void,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    time::Duration,
};
use windows::{
    Win32::System::{
        Diagnostics::Debug::OutputDebugStringW, LibraryLoader::GetModuleHandleW,
        Threading::GetCurrentProcessId,
    },
    core::PCWSTR,
};

const EFFECT_NODE_LIMIT: usize = 4096;
const GROUP_NODE_LIMIT: usize = 4096;
static INPUT_EFFECTS: ResidentInputEffectSlot = ResidentInputEffectSlot::new();
// 0=not started, 1=preparing, 2=waiting for FrameBegin, 3=registered, 4=rejected.
static STARTUP_STATUS: AtomicU32 = AtomicU32::new(0);

/// Copies startup status only. It neither polls input nor calls the game.
#[unsafe(no_mangle)]
pub extern "system" fn ShikiKeyboardStatus() -> u32 {
    STARTUP_STATUS.load(Ordering::Acquire)
}

/// Explicit notification for a host that has independently established a later lifetime epoch.
/// 0=accepted/equal, 1=not installed, 2=rejected. Never calls the game or retries installation.
/// # Safety
/// The caller serializes a genuine native generation transition against all effect callbacks,
/// keeps old objects live until those callbacks return, and supplies new graph/PARAM/allocator
/// guarantees. Numeric increments do not detect scene changes or reconcile abandoned effects.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn ShikiKeyboardNotifyEpoch(epoch: u64) -> u32 {
    let Some(control) = INPUT_EFFECTS
        .control()
        .and_then(|control| control.epoch_control())
    else {
        return 1;
    };
    // SAFETY: This export requires the same actual lifetime-transition contract as the SDK control.
    match unsafe { control.request_epoch(epoch) } {
        Ok(_) => 0,
        Err(_) => 2,
    }
}

fn debug(message: &str) {
    let message: Vec<u16> = format!("[keyboard-ex/ShikiFSRS] {message}\n")
        .encode_utf16()
        .chain([0])
        .collect();
    // SAFETY: The owned NUL-terminated UTF-16 buffer lives throughout this synchronous API call.
    unsafe { OutputDebugStringW(PCWSTR(message.as_ptr())) };
}

fn frame_begin_ready(file: &Ww171File<'_>, base: u64) -> Result<bool> {
    let memory = CurrentProcess;
    let catalog = file.bind(&memory, base)?;
    let Some(group) = catalog.observe_phase(TaskPhase::FRAME_BEGIN)? else {
        return Ok(false);
    };
    let Some(id) = group.native_group_id() else {
        return Ok(false);
    };
    // Presence is initialization evidence only, never a mutation-phase/teardown barrier.
    Ok(catalog.registered_group(id, GROUP_NODE_LIMIT)?.is_some())
}

/// # Safety
/// The loader host retains this DLL while waiting and provides the supported registration/heap
/// phase. After install, all resident task obligations in the module header hold to process exit.
unsafe fn install() -> std::result::Result<(), String> {
    let path = std::env::current_exe().map_err(|error| format!("executable path: {error}"))?;
    let bytes: Arc<[u8]> = std::fs::read(path)
        .map_err(|error| format!("executable read: {error}"))?
        .into();
    // Verify file identity before touching any loaded game memory.
    let file = Ww171File::verify(&bytes).map_err(|error| error.to_string())?;
    // SAFETY: None requests the current process executable; no borrowed module handle is released.
    let base = unsafe { GetModuleHandleW(None) }
        .map_err(|error| error.to_string())?
        .0 as u64;
    STARTUP_STATUS.store(2, Ordering::Release);
    loop {
        if frame_begin_ready(&file, base).map_err(|error| error.to_string())? {
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    // SAFETY: Standard Win32 query returns the actual current process PID, not game memory.
    let process_id = unsafe { GetCurrentProcessId() };
    // SAFETY: Experimental loader/registration contract above plus supported exclusive FrameBegin
    // graph/effect lifetime. Epoch 1 is the initial generation; known later transitions use the
    // explicit export. This candidate does not claim to detect identical-identity ABA.
    let installed = unsafe {
        INPUT_EFFECTS.install_current_keyboard(bytes, base, 1, EFFECT_NODE_LIMIT, process_id, None)
    }
    .map_err(|error| error.to_string())?;
    installed.registration.map_err(|error| {
        format!(
            "module pinned at {:#x}; registration failed: {error}",
            installed.module_handle
        )
    })?;
    STARTUP_STATUS.store(3, Ordering::Release);
    debug("FrameBegin registered; all 87 mappings enabled; process-resident experimental runtime");
    Ok(())
}

/// Windows DLL entry point. Work is deferred to a worker; it never waits under the loader lock.
/// # Safety
/// Windows invokes this entry under its DLL-loading ABI. The host retains the DLL until worker
/// installation pins it, provides native phase/lifetime obligations, and does not hot-unload it.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllMain(_module: isize, reason: u32, _reserved: *mut c_void) -> i32 {
    if reason == 1
        && STARTUP_STATUS
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    {
        let result = std::thread::Builder::new()
            .name("keyboard-ex-init".into())
            .spawn(|| {
                // SAFETY: Windows loader host contract is propagated from DllMain to this worker.
                if let Err(error) = unsafe { install() } {
                    STARTUP_STATUS.store(4, Ordering::Release);
                    debug(&error);
                }
            });
        if result.is_err() {
            STARTUP_STATUS.store(4, Ordering::Release);
        }
    }
    1
}

#[cfg(test)]
mod tests;
