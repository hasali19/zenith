use windows::core::{h, HSTRING};
use windows::Foundation::{EventRegistrationToken, TypedEventHandler};
use windows::Media::{
    MediaPlaybackStatus, MediaPlaybackType, SystemMediaTransportControls,
    SystemMediaTransportControlsButtonPressedEventArgs,
};
use windows::Win32::Foundation::HWND;
use windows::Win32::System::WinRT::{ISystemMediaTransportControlsInterop, RoGetActivationFactory};
use windows::Win32::UI::WindowsAndMessaging::{GetAncestor, GA_ROOT};

pub struct SystemMediaControls {
    controls: SystemMediaTransportControls,
    button_handler: Option<EventRegistrationToken>,
}

impl SystemMediaControls {
    pub fn set_button_handler(
        &mut self,
        mut f: impl FnMut(&SystemMediaTransportControlsButtonPressedEventArgs) -> windows::core::Result<()>
            + Send
            + 'static,
    ) {
        let handler =
            TypedEventHandler::new(move |_, event: &Option<_>| f(event.as_ref().unwrap()));

        let token = self.controls.ButtonPressed(&handler).unwrap();

        self.button_handler = Some(token);
    }
}

impl Drop for SystemMediaControls {
    fn drop(&mut self) {
        self.controls.SetIsEnabled(false).unwrap();

        if let Some(handler) = self.button_handler.take() {
            self.controls.RemoveButtonPressed(handler).unwrap();
        }
    }
}

impl SystemMediaControls {
    pub fn new(hwnd: HWND) -> SystemMediaControls {
        const WINDOWS_MEDIA_SYSTEMMEDIATRANSPORTCONTROLS: &windows::core::HSTRING =
            h!("Windows.Media.SystemMediaTransportControls");

        let controls: SystemMediaTransportControls = unsafe {
            let interop: ISystemMediaTransportControlsInterop =
                RoGetActivationFactory(WINDOWS_MEDIA_SYSTEMMEDIATRANSPORTCONTROLS).unwrap();
            let root_window = GetAncestor(hwnd, GA_ROOT);
            interop.GetForWindow(root_window).unwrap()
        };

        controls.SetIsEnabled(true).unwrap();
        controls.SetIsPauseEnabled(true).unwrap();
        controls.SetIsPlayEnabled(true).unwrap();
        controls.SetIsNextEnabled(true).unwrap();
        controls.SetIsPreviousEnabled(true).unwrap();

        let media_display_updater = controls.DisplayUpdater().unwrap();

        media_display_updater
            .SetType(MediaPlaybackType::Video)
            .unwrap();

        media_display_updater.Update().unwrap();

        SystemMediaControls {
            controls,
            button_handler: None,
        }
    }

    pub fn set_stopped(&self) {
        self.controls
            .SetPlaybackStatus(MediaPlaybackStatus::Stopped)
            .unwrap();
    }

    pub fn set_playing(&self) {
        self.controls
            .SetPlaybackStatus(MediaPlaybackStatus::Playing)
            .unwrap();
    }

    pub fn set_paused(&self, paused: bool) {
        self.controls
            .SetPlaybackStatus(if paused {
                MediaPlaybackStatus::Paused
            } else {
                MediaPlaybackStatus::Playing
            })
            .unwrap();
    }

    pub fn update_media_display(&self, title: Option<&str>, subtitle: Option<&str>) {
        let media_display_updater = self.controls.DisplayUpdater().unwrap();

        if let Some(title) = title {
            media_display_updater
                .VideoProperties()
                .unwrap()
                .SetTitle(&HSTRING::from(title))
                .unwrap();
        }

        if let Some(subtitle) = subtitle {
            media_display_updater
                .VideoProperties()
                .unwrap()
                .SetSubtitle(&HSTRING::from(subtitle))
                .unwrap();
        }

        media_display_updater.Update().unwrap();
    }
}
