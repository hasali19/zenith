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
use crate::media_player::{MediaPlayer, MediaPlayerEvent, MediaTrack, MediaTrackType};
use crate::video_surface::VideoSurface;

struct VideoPlayerPlugin {
    messenger: Arc<FlutterDesktopMessenger>,
    texture_registrar: Arc<FlutterDesktopTextureRegistrar>,
    window: HWND,
}

impl FlutterDesktopPlugin for VideoPlayerPlugin {
    fn register_with_registrar(registrar: &FlutterDesktopPluginRegistrar) {
        let flutter_window = registrar.view().hwnd();
        let root_window = unsafe { GetAncestor(flutter_window, GA_ROOT) };

        let plugin = VideoPlayerPlugin {
            messenger: registrar.messenger().clone(),
            texture_registrar: registrar.texture_registrar().clone(),
            window: root_window,
        };

        registrar
            .messenger()
            .set_callback("video_player", move |name, args, reply| {
                plugin.handle_method_call(name, args, reply)
            });
    }
}

impl VideoPlayerPlugin {
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

                            match &event {
                                &MediaPlayerEvent::DurationChanged(v) => {
                                    res.insert(
                                        EncodableValue::Str("duration"),
                                        EncodableValue::F64(v.into()),
                                    );
                                }
                                &MediaPlayerEvent::PauseChanged(v) => {
                                    res.insert(
                                        EncodableValue::Str("paused"),
                                        EncodableValue::Bool(v),
                                    );
                                }
                                &MediaPlayerEvent::IdleChanged(v) => {
                                    res.insert(
                                        EncodableValue::Str("idle"),
                                        EncodableValue::Bool(v),
                                    );
                                }
                                &MediaPlayerEvent::VideoEnded => {
                                    res.insert(
                                        EncodableValue::Str("state"),
                                        EncodableValue::Str("ended"),
                                    );
                                }
                                &MediaPlayerEvent::SpeedChanged(v) => {
                                    res.insert(
                                        EncodableValue::Str("speed"),
                                        EncodableValue::F64(v.into()),
                                    );
                                }
                                &MediaPlayerEvent::PlaylistPosChanged(v) => {
                                    res.insert(
                                        EncodableValue::Str("playlist-pos"),
                                        EncodableValue::I64(v.into()),
                                    );
                                }
                                MediaPlayerEvent::TracksChanged(tracks) => {
                                    res.insert(
                                        EncodableValue::Str("tracks"),
                                        build_tracks_list(tracks),
                                    );
                                }
                                &MediaPlayerEvent::SubTrackChanged(track_id) => {
                                    res.insert(
                                        EncodableValue::Str("selected-sub-track"),
                                        track_id
                                            .map(EncodableValue::I64)
                                            .unwrap_or(EncodableValue::Null),
                                    );
                                }
                                MediaPlayerEvent::SubStyleChanged(style) => {
                                    res.insert(
                                        EncodableValue::Str("subtitle-style"),
                                        EncodableValue::Map({
                                            let mut map = BTreeMap::new();
                                            map.insert(
                                                EncodableValue::Str("size"),
                                                EncodableValue::I64(style.size.into()),
                                            );
                                            map
                                        }),
                                    );
                                }
                            }

                            messenger.call("video_player", "event", &EncodableValue::Map(res));
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

                mpv.set_property("vo", cstr!("libmpv"));
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

                drop(surface);

                reply.success(&EncodableValue::Null);
            }
            _ => reply.not_implemented(),
        }
    }
}

fn build_tracks_list(tracks: &[MediaTrack]) -> EncodableValue<'_> {
    EncodableValue::List(
        tracks
            .iter()
            .map(|track| {
                EncodableValue::Map({
                    let mut map = BTreeMap::new();
                    map.insert(EncodableValue::Str("id"), EncodableValue::I64(track.id));
                    map.insert(
                        EncodableValue::Str("type"),
                        EncodableValue::I64(match track.track_type {
                            MediaTrackType::Video => 1,
                            MediaTrackType::Audio => 2,
                            MediaTrackType::Subtitle => 3,
                        }),
                    );
                    map.insert(
                        EncodableValue::Str("title"),
                        track
                            .title
                            .as_deref()
                            .map(EncodableValue::Str)
                            .unwrap_or(EncodableValue::Null),
                    );
                    map.insert(
                        EncodableValue::Str("lang"),
                        track
                            .language
                            .as_deref()
                            .map(EncodableValue::Str)
                            .unwrap_or(EncodableValue::Null),
                    );
                    map.insert(
                        EncodableValue::Str("selected"),
                        EncodableValue::Bool(track.selected),
                    );
                    map.insert(
                        EncodableValue::Str("codec"),
                        track
                            .codec
                            .as_deref()
                            .map(EncodableValue::Str)
                            .unwrap_or(EncodableValue::Null),
                    );
                    map
                })
            })
            .collect(),
    )
}

flutter_plugin!(VideoPlayerPlugin);
