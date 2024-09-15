use std::ffi::CString;

use libmpv2_sys::*;

const VIDEO_URL: &str = "test-data/test-video.mp4";

fn main() {
    // Initialize MPV client
    unsafe {
        let mpv_handle = mpv_create();
        if mpv_handle.is_null() {
            eprintln!("Failed to create MPV instance");
            return;
        }

        // Start MPV
        if mpv_initialize(mpv_handle) < 0 {
            eprintln!("Failed to initialize MPV");
            mpv_destroy(mpv_handle);
            return;
        }

        let raw = CString::new(format!("{} {} {}", "loadfile", VIDEO_URL, "replace")).unwrap();

        // Send the command to MPV to load the file
        if mpv_command_string(mpv_handle, raw.as_ptr()) < 0 {
            eprintln!("Failed to load file");
            mpv_destroy(mpv_handle);
            return;
        }

        // Now MPV will start playing the file, we can enter a basic event loop
        loop {
            let event = mpv_wait_event(mpv_handle, 1000.0);
            if !event.is_null() {
                let event_id = (*event).event_id;

                match event_id {
                    mpv_event_id_MPV_EVENT_END_FILE => {
                        println!("Playback finished");
                        break;
                    }
                    mpv_event_id_MPV_EVENT_SHUTDOWN => {
                        println!("MPV is shutting down");
                        break;
                    }
                    _ => {
                        // Handle other events if necessary
                    }
                }
            }
        }

        // Clean up and close MPV
        mpv_destroy(mpv_handle);
    }
}
