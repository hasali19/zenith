use std::ffi::{c_char, c_void, CStr};
use std::ptr;

use mpv_sys::{
    mpv_opengl_fbo, mpv_opengl_init_params, mpv_render_context, mpv_render_context_create,
    mpv_render_context_free, mpv_render_context_render, mpv_render_context_set_update_callback,
    mpv_render_context_update, mpv_render_param,
    mpv_render_param_type_MPV_RENDER_PARAM_ADVANCED_CONTROL,
    mpv_render_param_type_MPV_RENDER_PARAM_API_TYPE,
    mpv_render_param_type_MPV_RENDER_PARAM_OPENGL_FBO,
    mpv_render_param_type_MPV_RENDER_PARAM_OPENGL_INIT_PARAMS,
    mpv_render_update_flag_MPV_RENDER_UPDATE_FRAME, MPV_RENDER_API_TYPE_OPENGL,
};

use crate::mpv_player::MpvPlayer;

type GetProcAddress = Box<dyn Fn(&CStr) -> *mut c_void>;

pub struct MpvRenderContext {
    ctx: *mut mpv_render_context,
    _get_proc_address: Box<GetProcAddress>,
    _update_callback: Box<Box<dyn Fn()>>,
}

impl MpvRenderContext {
    pub fn new(
        mpv: &MpvPlayer,
        get_proc_address: GetProcAddress,
        update_callback: Box<dyn Fn()>,
    ) -> MpvRenderContext {
        unsafe extern "C" fn get_proc_address_(
            fn_ctx: *mut c_void,
            name: *const c_char,
        ) -> *mut c_void {
            let get_proc_address = fn_ctx
                .cast_const()
                .cast::<GetProcAddress>()
                .as_ref()
                .unwrap();
            get_proc_address(CStr::from_ptr(name))
        }

        let get_proc_address = Box::new(get_proc_address);

        let mut opengl_init_params = mpv_opengl_init_params {
            get_proc_address: Some(get_proc_address_),
            get_proc_address_ctx: get_proc_address.as_ref() as *const _ as _,
        };

        let mut render_params = [
            mpv_render_param {
                type_: mpv_render_param_type_MPV_RENDER_PARAM_API_TYPE,
                data: MPV_RENDER_API_TYPE_OPENGL.as_ptr() as *mut c_void,
            },
            mpv_render_param {
                type_: mpv_render_param_type_MPV_RENDER_PARAM_OPENGL_INIT_PARAMS,
                data: &mut opengl_init_params as *mut _ as *mut _,
            },
            mpv_render_param {
                type_: mpv_render_param_type_MPV_RENDER_PARAM_ADVANCED_CONTROL,
                data: &mut 1 as *mut _ as _,
            },
            mpv_render_param::default(),
        ];

        let mut render_ctx = ptr::null_mut();
        unsafe {
            mpv_render_context_create(&mut render_ctx, mpv.handle(), render_params.as_mut_ptr());
        }

        unsafe extern "C" fn update_callback_(user_data: *mut c_void) {
            let callback = user_data
                .cast_const()
                .cast::<Box<dyn Fn()>>()
                .as_ref()
                .unwrap();
            callback();
        }

        let update_callback = Box::new(update_callback);
        unsafe {
            mpv_render_context_set_update_callback(
                render_ctx,
                Some(update_callback_),
                update_callback.as_ref() as *const _ as _,
            );
        }

        MpvRenderContext {
            ctx: render_ctx,
            _get_proc_address: get_proc_address,
            _update_callback: update_callback,
        }
    }

    pub fn update(&self) -> bool {
        let flags = unsafe { mpv_render_context_update(self.ctx) };
        (flags & mpv_render_update_flag_MPV_RENDER_UPDATE_FRAME as u64) != 0
    }

    pub fn render(&self, width: i32, height: i32) {
        let mut fbo = mpv_opengl_fbo {
            fbo: 0,
            w: width,
            h: height,
            internal_format: gl::RGBA8 as i32,
        };

        let mut params = [
            mpv_render_param {
                type_: mpv_render_param_type_MPV_RENDER_PARAM_OPENGL_FBO,
                data: &mut fbo as *mut mpv_opengl_fbo as *mut c_void,
            },
            mpv_render_param::default(),
        ];

        unsafe {
            mpv_render_context_render(self.ctx, params.as_mut_ptr());
        }
    }
}

unsafe impl Send for MpvRenderContext {}
unsafe impl Sync for MpvRenderContext {}

impl Drop for MpvRenderContext {
    fn drop(&mut self) {
        unsafe {
            mpv_render_context_free(self.ctx);
        }
    }
}
