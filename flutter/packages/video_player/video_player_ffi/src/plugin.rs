use std::collections::BTreeMap;
use std::sync::Arc;

use flutter_plugin::codec::EncodableValue;
use flutter_plugin::messenger::{FlutterDesktopMessenger, FlutterDesktopMessengerReply};
use flutter_plugin::registrar::FlutterDesktopPluginRegistrar;
use flutter_plugin::texture_registrar::FlutterDesktopTextureRegistrar;
use flutter_plugin::{flutter_plugin, FlutterDesktopPlugin};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetAncestor, GA_ROOT};

use crate::cstr;
use crate::media_player::{MediaPlayer, MediaPlayerEvent};
use crate::video_surface::VideoSurface;

struct VideoPlayerFfiPlugin {
    messenger: Arc<FlutterDesktopMessenger>,
    texture_registrar: Arc<FlutterDesktopTextureRegistrar>,
    window: HWND,
}

impl FlutterDesktopPlugin for VideoPlayerFfiPlugin {
    fn register_with_registrar(registrar: &FlutterDesktopPluginRegistrar) {
        let flutter_window = registrar.view().hwnd();
        let root_window = unsafe { GetAncestor(flutter_window, GA_ROOT) };

        let plugin = VideoPlayerFfiPlugin {
            messenger: registrar.messenger().clone(),
            texture_registrar: registrar.texture_registrar().clone(),
            window: root_window,
        };

        registrar
            .messenger()
            .set_callback("video_player_ffi", move |name, args, reply| {
                plugin.handle_method_call(name, args, reply)
            });
    }
}

impl VideoPlayerFfiPlugin {
    fn handle_method_call(
        &self,
        name: &str,
        args: EncodableValue,
        reply: FlutterDesktopMessengerReply,
    ) {
        match name {
            "createPlayer" => {
                let player = Box::new(MediaPlayer::new(self.window));
                let player_ref = player.as_ref() as *const MediaPlayer;

                std::thread::spawn({
                    let messenger = self.messenger.clone();
                    move || {
                        player.run_event_loop(|position, event| {
                            let mut res = BTreeMap::new();

                            res.insert(
                                EncodableValue::Str("position"),
                                EncodableValue::F64(position.into()),
                            );

                            match event {
                                MediaPlayerEvent::DurationChanged(v) => {
                                    res.insert(
                                        EncodableValue::Str("duration"),
                                        EncodableValue::F64(v.into()),
                                    );
                                }
                                MediaPlayerEvent::PauseChanged(v) => {
                                    res.insert(
                                        EncodableValue::Str("paused"),
                                        EncodableValue::Bool(v),
                                    );
                                }
                                MediaPlayerEvent::IdleChanged(v) => {
                                    res.insert(
                                        EncodableValue::Str("idle"),
                                        EncodableValue::Bool(v),
                                    );
                                }
                                MediaPlayerEvent::VideoEnded => {
                                    res.insert(
                                        EncodableValue::Str("state"),
                                        EncodableValue::Str("ended"),
                                    );
                                }
                                MediaPlayerEvent::SpeedChanged(v) => {
                                    res.insert(
                                        EncodableValue::Str("speed"),
                                        EncodableValue::F64(v.into()),
                                    );
                                }
                                MediaPlayerEvent::PlaylistPosChanged(v) => {
                                    res.insert(
                                        EncodableValue::Str("playlist-pos"),
                                        EncodableValue::I64(v.into()),
                                    );
                                }
                            }

                            messenger.call("video_player_ffi", "event", &EncodableValue::Map(res));
                        });
                    }
                });

                reply.success(&EncodableValue::I64(player_ref as i64));
            }
            "destroyPlayer" => {
                let args = args.as_map().unwrap();

                let player = *args
                    .get(&EncodableValue::Str("player"))
                    .unwrap()
                    .as_i64()
                    .unwrap();

                let player = unsafe { (player as *const MediaPlayer).as_ref().unwrap() };

                player.quit();

                reply.success(&EncodableValue::Null);
            }
            "createVideoSurface" => {
                let args = args.as_map().unwrap();

                let player = *args
                    .get(&EncodableValue::Str("player"))
                    .unwrap()
                    .as_i64()
                    .unwrap();

                let player = unsafe { (player as *const MediaPlayer).as_ref().unwrap() };
                let mpv = player.mpv();

                mpv.set_property("hwdec", cstr!("d3d11va-copy"));
                mpv.set_property("video-sync", cstr!("audio"));
                mpv.set_property("video-timing-offset", cstr!("0"));

                // Leaked memory will be freed by destroyVideoSurface below
                let surface = Box::leak(Box::new(
                    VideoSurface::new(mpv, self.texture_registrar.clone()).unwrap(),
                ));

                reply.success(&EncodableValue::I64(surface as *const VideoSurface as i64));
            }
            "destroyVideoSurface" => {
                let args = args.as_map().unwrap();
                let surface = *args
                    .get(&EncodableValue::Str("surface"))
                    .unwrap()
                    .as_i64()
                    .unwrap();

                let surface = unsafe { Box::from_raw(surface as *mut VideoSurface) };

                surface.destroy();

                reply.success(&EncodableValue::Null);
            }
            _ => reply.not_implemented(),
        }
    }
}

flutter_plugin!(VideoPlayerFfiPlugin);
