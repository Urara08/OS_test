#![no_std]
#![no_main]

use core::ptr;

use uefi::prelude::*;
use uefi::proto::console::gop::{GraphicsOutput, PixelFormat};

#[entry]
fn efi_main(handle: Handle, st: SystemTable<Boot>) -> Status {
    let bt = st.boot_services();

    // Graphics Output Protocol (GOP) を取得
    let gop = unsafe {
        &mut *bt
            .locate_protocol::<GraphicsOutput>()
            .expect("GOP を取得できなかった")
            .get()
    };

    // フレームバッファを u32 ポインタにキャスト
    let fb = gop.frame_buffer().as_mut_ptr() as *mut u32;
    let info = gop.current_mode_info();
    let stride = info.stride() as usize;

    let width = info.resolution().0 as usize;
    let height = info.resolution().1 as usize;

    // 単色 (青) 塗りつぶし
    unsafe {
        for y in 0..height {
            for x in 0..width {
                let offset = y * stride + x;
                let pixel_ptr = fb.add(offset);
                match info.pixel_format() {
                    PixelFormat::Rgb => ptr::write_volatile(pixel_ptr, 0x000000FF), // 青
                    PixelFormat::Bgr => ptr::write_volatile(pixel_ptr, 0x000000FF), // QEMUでは大体これでOK
    _ => {}
}
            }
        }
    }

    Status::SUCCESS
}

// panic_handler 必須
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
