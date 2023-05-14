#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindow(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindow {
    type Vtable = IAppWindow_Vtbl;
}
impl ::core::clone::Clone for IAppWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindow {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xcfa788b3_643b_5c5e_ad4e_321d48a82acd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindow_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowId,
    ) -> ::windows::core::HRESULT,
    pub IsShownInSwitchers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsShownInSwitchers: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub OwnerWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowId,
    ) -> ::windows::core::HRESULT,
    pub Position: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::PointInt32,
    ) -> ::windows::core::HRESULT,
    pub Presenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::SizeInt32,
    ) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::std::mem::MaybeUninit<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub TitleBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Destroy:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Move: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        position: ::windows::Graphics::PointInt32,
    ) -> ::windows::core::HRESULT,
    pub MoveAndResize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Graphics::RectInt32,
    ) -> ::windows::core::HRESULT,
    pub MoveAndResizeRelativeToDisplayArea: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Graphics::RectInt32,
        displayarea: *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
    pub Resize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        size: ::windows::Graphics::SizeInt32,
    ) -> ::windows::core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        iconpath: ::std::mem::MaybeUninit<::windows::core::HSTRING>,
    ) -> ::windows::core::HRESULT,
    pub SetIconWithIconId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        iconid: super::IconId,
    ) -> ::windows::core::HRESULT,
    pub SetPresenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appwindowpresenter: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetPresenterByKind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appwindowpresenterkind: AppWindowPresenterKind,
    ) -> ::windows::core::HRESULT,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ShowWithActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        activatewindow: bool,
    ) -> ::windows::core::HRESULT,
    pub Changed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Closing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveClosing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Destroying: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveDestroying: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindow2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindow2 {
    type Vtable = IAppWindow2_Vtbl;
}
impl ::core::clone::Clone for IAppWindow2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindow2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x6cd41292_794c_5cac_8961_210d012c6ebc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindow2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ClientSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::SizeInt32,
    ) -> ::windows::core::HRESULT,
    pub MoveInZOrderAtBottom:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveInZOrderAtTop:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveInZOrderBelow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::WindowId,
    ) -> ::windows::core::HRESULT,
    pub ResizeClient: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        size: ::windows::Graphics::SizeInt32,
    ) -> ::windows::core::HRESULT,
    pub ShowOnceWithRequestedStartupState:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowChangedEventArgs {
    type Vtable = IAppWindowChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppWindowChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowChangedEventArgs {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2182bc5d_fdac_5c3e_bf37_7d8d684e9d1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DidPositionChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub DidPresenterChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub DidSizeChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub DidVisibilityChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowChangedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowChangedEventArgs2 {
    type Vtable = IAppWindowChangedEventArgs2_Vtbl;
}
impl ::core::clone::Clone for IAppWindowChangedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowChangedEventArgs2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa773ab4c_a5ec_50e8_98ac_247fe6cd4227);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowChangedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DidZOrderChange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsZOrderAtBottom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsZOrderAtTop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub ZOrderBelowWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::WindowId,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowClosingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowClosingEventArgs {
    type Vtable = IAppWindowClosingEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAppWindowClosingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowClosingEventArgs {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x0e09d90b_2261_590b_9ad1_8504991d8754);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowClosingEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Cancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowPresenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowPresenter {
    type Vtable = IAppWindowPresenter_Vtbl;
}
impl ::core::clone::Clone for IAppWindowPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowPresenter {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xbc3042c2_c6c6_5632_8989_ff0ec6d3b40d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresenter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut AppWindowPresenterKind,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowPresenterFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowPresenterFactory {
    type Vtable = IAppWindowPresenterFactory_Vtbl;
}
impl ::core::clone::Clone for IAppWindowPresenterFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowPresenterFactory {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x62082e3c_1368_5238_90d1_e932dc718a82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowPresenterFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowStatics {
    type Vtable = IAppWindowStatics_Vtbl;
}
impl ::core::clone::Clone for IAppWindowStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3c315c24_d540_5d72_b518_b226b83627cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateWithPresenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appwindowpresenter: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateWithPresenterAndOwner: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        appwindowpresenter: *mut ::core::ffi::c_void,
        ownerwindowid: super::WindowId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetFromWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::WindowId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowTitleBar(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowTitleBar {
    type Vtable = IAppWindowTitleBar_Vtbl;
}
impl ::core::clone::Clone for IAppWindowTitleBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowTitleBar {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5574efa2_c91c_5700_a363_539c71a7aaf4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBar_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ButtonBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetButtonBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ButtonForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetButtonForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ButtonHoverBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetButtonHoverBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ButtonHoverForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetButtonHoverForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ButtonInactiveBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetButtonInactiveBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
    pub ButtonInactiveForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetButtonInactiveForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    )
        -> ::windows::core::HRESULT,
    pub ButtonPressedBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetButtonPressedBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ButtonPressedForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetButtonPressedForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ExtendsContentIntoTitleBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetExtendsContentIntoTitleBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub ForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub IconShowOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut IconShowOptions,
    ) -> ::windows::core::HRESULT,
    pub SetIconShowOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: IconShowOptions,
    ) -> ::windows::core::HRESULT,
    pub InactiveBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetInactiveBackgroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub InactiveForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetInactiveForegroundColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub LeftInset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub RightInset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub ResetToDefault:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDragRectangles: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value_array_size: u32,
        value: *const ::windows::Graphics::RectInt32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowTitleBar2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowTitleBar2 {
    type Vtable = IAppWindowTitleBar2_Vtbl;
}
impl ::core::clone::Clone for IAppWindowTitleBar2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowTitleBar2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x86faed38_748a_5b4b_9ccf_3ba0496c9041);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBar2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PreferredHeightOption: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut TitleBarHeightOption,
    ) -> ::windows::core::HRESULT,
    pub SetPreferredHeightOption: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: TitleBarHeightOption,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAppWindowTitleBarStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAppWindowTitleBarStatics {
    type Vtable = IAppWindowTitleBarStatics_Vtbl;
}
impl ::core::clone::Clone for IAppWindowTitleBarStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IAppWindowTitleBarStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x9e1da52e_8b15_54d6_a886_f7b9f9d930b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppWindowTitleBarStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsCustomizationSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompactOverlayPresenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompactOverlayPresenter {
    type Vtable = ICompactOverlayPresenter_Vtbl;
}
impl ::core::clone::Clone for ICompactOverlayPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompactOverlayPresenter {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xefeb0812_6fc7_5b7d_bd92_cc8f9a6454c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompactOverlayPresenter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub InitialSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CompactOverlaySize,
    ) -> ::windows::core::HRESULT,
    pub SetInitialSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CompactOverlaySize,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICompactOverlayPresenterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICompactOverlayPresenterStatics {
    type Vtable = ICompactOverlayPresenterStatics_Vtbl;
}
impl ::core::clone::Clone for ICompactOverlayPresenterStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ICompactOverlayPresenterStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xeab93186_4f6a_52f9_8c03_da57a1522f6e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompactOverlayPresenterStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayArea(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayArea {
    type Vtable = IDisplayArea_Vtbl;
}
impl ::core::clone::Clone for IDisplayArea {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDisplayArea {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5c7e0537_b621_5579_bcae_a84aa8746167);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayArea_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DisplayId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::DisplayId,
    ) -> ::windows::core::HRESULT,
    pub IsPrimary: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub OuterBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::RectInt32,
    ) -> ::windows::core::HRESULT,
    pub WorkArea: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::RectInt32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayAreaStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayAreaStatics {
    type Vtable = IDisplayAreaStatics_Vtbl;
}
impl ::core::clone::Clone for IDisplayAreaStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDisplayAreaStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x02ab4926_211e_5d49_8e4b_2af193daed09);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayAreaStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Primary: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateWatcher: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub FindAll: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetFromWindowId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        windowid: super::WindowId,
        displayareafallback: DisplayAreaFallback,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetFromPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Graphics::PointInt32,
        displayareafallback: DisplayAreaFallback,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetFromRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Graphics::RectInt32,
        displayareafallback: DisplayAreaFallback,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayAreaStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayAreaStatics2 {
    type Vtable = IDisplayAreaStatics2_Vtbl;
}
impl ::core::clone::Clone for IDisplayAreaStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDisplayAreaStatics2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x7207ad4b_890d_5dd7_bc18_78ffd9544d8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayAreaStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetFromDisplayId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        displayid: super::DisplayId,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDisplayAreaWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDisplayAreaWatcher {
    type Vtable = IDisplayAreaWatcher_Vtbl;
}
impl ::core::clone::Clone for IDisplayAreaWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDisplayAreaWatcher {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x83f6562f_d3a0_548b_8e4f_a99be3d95c9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayAreaWatcher_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut DisplayAreaWatcherStatus,
    ) -> ::windows::core::HRESULT,
    pub Start:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Added: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveAdded: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub EnumerationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Removed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveRemoved: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Stopped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveStopped: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub Updated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        handler: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    pub RemoveUpdated: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullScreenPresenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFullScreenPresenter {
    type Vtable = IFullScreenPresenter_Vtbl;
}
impl ::core::clone::Clone for IFullScreenPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFullScreenPresenter {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xfa9141fd_b8dd_5da1_8b2b_7cdadb76f593);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullScreenPresenter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFullScreenPresenterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFullScreenPresenterStatics {
    type Vtable = IFullScreenPresenterStatics_Vtbl;
}
impl ::core::clone::Clone for IFullScreenPresenterStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IFullScreenPresenterStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2ec0d2c1_e086_55bb_a3b2_44942e231c67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullScreenPresenterStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOverlappedPresenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOverlappedPresenter {
    type Vtable = IOverlappedPresenter_Vtbl;
}
impl ::core::clone::Clone for IOverlappedPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOverlappedPresenter {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x21693970_4f4c_5172_9e9d_682a2d174884);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOverlappedPresenter_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HasBorder: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub HasTitleBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsAlwaysOnTop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsAlwaysOnTop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsMaximizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsMaximizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsMinimizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsMinimizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsModal: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsModal: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub IsResizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetIsResizable: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut OverlappedPresenterState,
    ) -> ::windows::core::HRESULT,
    pub Maximize:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Minimize:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Restore:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBorderAndTitleBar: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        hasborder: bool,
        hastitlebar: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOverlappedPresenter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOverlappedPresenter2 {
    type Vtable = IOverlappedPresenter2_Vtbl;
}
impl ::core::clone::Clone for IOverlappedPresenter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOverlappedPresenter2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x5c6ccd93_4244_5cd2_b355_ed5ea34df730);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOverlappedPresenter2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MinimizeWithActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        activatewindow: bool,
    ) -> ::windows::core::HRESULT,
    pub RestoreWithActivation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        activatewindow: bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOverlappedPresenterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOverlappedPresenterStatics {
    type Vtable = IOverlappedPresenterStatics_Vtbl;
}
impl ::core::clone::Clone for IOverlappedPresenterStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOverlappedPresenterStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x997225e4_7b00_5aee_a4be_d4068d1999e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOverlappedPresenterStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateForContextMenu: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateForDialog: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateForToolWindow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOverlappedPresenterStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOverlappedPresenterStatics2 {
    type Vtable = IOverlappedPresenterStatics2_Vtbl;
}
impl ::core::clone::Clone for IOverlappedPresenterStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOverlappedPresenterStatics2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xed5c4f92_32f4_5d15_80d0_b2a5efa04d39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOverlappedPresenterStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RequestedStartupState: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut OverlappedPresenterState,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct AppWindow(::windows::core::IUnknown);
impl AppWindow {
    pub fn Id(&self) -> ::windows::core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::WindowId>();
            (::windows::core::Interface::vtable(this).Id)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsShownInSwitchers(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsShownInSwitchers)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsShownInSwitchers(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsShownInSwitchers)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsVisible)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OwnerWindowId(&self) -> ::windows::core::Result<super::WindowId> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::WindowId>();
            (::windows::core::Interface::vtable(this).OwnerWindowId)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Position(&self) -> ::windows::core::Result<::windows::Graphics::PointInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::Graphics::PointInt32>();
            (::windows::core::Interface::vtable(this).Position)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Presenter(&self) -> ::windows::core::Result<AppWindowPresenter> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowPresenter>();
            (::windows::core::Interface::vtable(this).Presenter)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Size(&self) -> ::windows::core::Result<::windows::Graphics::SizeInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::Graphics::SizeInt32>();
            (::windows::core::Interface::vtable(this).Size)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Title)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetTitle)(
                ::windows::core::Interface::as_raw(this),
                ::core::mem::transmute_copy(value),
            )
            .ok()
        }
    }
    pub fn TitleBar(&self) -> ::windows::core::Result<AppWindowTitleBar> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowTitleBar>();
            (::windows::core::Interface::vtable(this).TitleBar)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Destroy(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Destroy)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Hide(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Hide)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Move(&self, position: ::windows::Graphics::PointInt32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Move)(
                ::windows::core::Interface::as_raw(this),
                position,
            )
            .ok()
        }
    }
    pub fn MoveAndResize(
        &self,
        rect: ::windows::Graphics::RectInt32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).MoveAndResize)(
                ::windows::core::Interface::as_raw(this),
                rect,
            )
            .ok()
        }
    }
    pub fn MoveAndResizeRelativeToDisplayArea(
        &self,
        rect: ::windows::Graphics::RectInt32,
        displayarea: &DisplayArea,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).MoveAndResizeRelativeToDisplayArea)(
                ::windows::core::Interface::as_raw(this),
                rect,
                ::core::mem::transmute_copy(displayarea),
            )
            .ok()
        }
    }
    pub fn Resize(&self, size: ::windows::Graphics::SizeInt32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Resize)(
                ::windows::core::Interface::as_raw(this),
                size,
            )
            .ok()
        }
    }
    pub fn SetIcon(&self, iconpath: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIcon)(
                ::windows::core::Interface::as_raw(this),
                ::core::mem::transmute_copy(iconpath),
            )
            .ok()
        }
    }
    pub fn SetIconWithIconId(&self, iconid: super::IconId) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIconWithIconId)(
                ::windows::core::Interface::as_raw(this),
                iconid,
            )
            .ok()
        }
    }
    pub fn SetPresenter<P0>(&self, appwindowpresenter: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<AppWindowPresenter>,
    {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPresenter)(
                ::windows::core::Interface::as_raw(this),
                appwindowpresenter.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn SetPresenterByKind(
        &self,
        appwindowpresenterkind: AppWindowPresenterKind,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPresenterByKind)(
                ::windows::core::Interface::as_raw(this),
                appwindowpresenterkind,
            )
            .ok()
        }
    }
    pub fn Show(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Show)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn ShowWithActivation(&self, activatewindow: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).ShowWithActivation)(
                ::windows::core::Interface::as_raw(this),
                activatewindow,
            )
            .ok()
        }
    }
    pub fn Changed(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<AppWindow, AppWindowChangedEventArgs>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::windows::core::zeroed::<::windows::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Changed)(
                ::windows::core::Interface::as_raw(this),
                ::core::mem::transmute_copy(handler),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RemoveChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveChanged)(
                ::windows::core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Closing(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<AppWindow, AppWindowClosingEventArgs>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::windows::core::zeroed::<::windows::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Closing)(
                ::windows::core::Interface::as_raw(this),
                ::core::mem::transmute_copy(handler),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RemoveClosing(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveClosing)(
                ::windows::core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Destroying(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            AppWindow,
            ::windows::core::IInspectable,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::windows::core::zeroed::<::windows::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Destroying)(
                ::windows::core::Interface::as_raw(this),
                ::core::mem::transmute_copy(handler),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RemoveDestroying(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveDestroying)(
                ::windows::core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn ClientSize(&self) -> ::windows::core::Result<::windows::Graphics::SizeInt32> {
        let this = &::windows::core::ComInterface::cast::<IAppWindow2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::Graphics::SizeInt32>();
            (::windows::core::Interface::vtable(this).ClientSize)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn MoveInZOrderAtBottom(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAppWindow2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).MoveInZOrderAtBottom)(
                ::windows::core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn MoveInZOrderAtTop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAppWindow2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).MoveInZOrderAtTop)(
                ::windows::core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn MoveInZOrderBelow(&self, windowid: super::WindowId) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAppWindow2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).MoveInZOrderBelow)(
                ::windows::core::Interface::as_raw(this),
                windowid,
            )
            .ok()
        }
    }
    pub fn ResizeClient(
        &self,
        size: ::windows::Graphics::SizeInt32,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAppWindow2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ResizeClient)(
                ::windows::core::Interface::as_raw(this),
                size,
            )
            .ok()
        }
    }
    pub fn ShowOnceWithRequestedStartupState(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAppWindow2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).ShowOnceWithRequestedStartupState)(
                ::windows::core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn Create() -> ::windows::core::Result<AppWindow> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindow>();
            (::windows::core::Interface::vtable(this).Create)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateWithPresenter<P0>(appwindowpresenter: P0) -> ::windows::core::Result<AppWindow>
    where
        P0: ::windows::core::TryIntoParam<AppWindowPresenter>,
    {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindow>();
            (::windows::core::Interface::vtable(this).CreateWithPresenter)(
                ::windows::core::Interface::as_raw(this),
                appwindowpresenter.try_into_param()?.abi(),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateWithPresenterAndOwner<P0>(
        appwindowpresenter: P0,
        ownerwindowid: super::WindowId,
    ) -> ::windows::core::Result<AppWindow>
    where
        P0: ::windows::core::TryIntoParam<AppWindowPresenter>,
    {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindow>();
            (::windows::core::Interface::vtable(this).CreateWithPresenterAndOwner)(
                ::windows::core::Interface::as_raw(this),
                appwindowpresenter.try_into_param()?.abi(),
                ownerwindowid,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetFromWindowId(windowid: super::WindowId) -> ::windows::core::Result<AppWindow> {
        Self::IAppWindowStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindow>();
            (::windows::core::Interface::vtable(this).GetFromWindowId)(
                ::windows::core::Interface::as_raw(this),
                windowid,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IAppWindowStatics<R, F: FnOnce(&IAppWindowStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AppWindow, IAppWindowStatics> =
            ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AppWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindow {}
impl ::core::fmt::Debug for AppWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindow").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppWindow {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.AppWindow;{cfa788b3-643b-5c5e-ad4e-321d48a82acd})",
    );
}
impl ::core::clone::Clone for AppWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppWindow {
    type Vtable = IAppWindow_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppWindow {
    const IID: ::windows::core::GUID = <IAppWindow as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppWindow {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindow";
}
::windows::imp::interface_hierarchy!(
    AppWindow,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AppWindow {}
unsafe impl ::core::marker::Sync for AppWindow {}
#[repr(transparent)]
pub struct AppWindowChangedEventArgs(::windows::core::IUnknown);
impl AppWindowChangedEventArgs {
    pub fn DidPositionChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DidPositionChange)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DidPresenterChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DidPresenterChange)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DidSizeChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DidSizeChange)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DidVisibilityChange(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DidVisibilityChange)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn DidZOrderChange(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IAppWindowChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).DidZOrderChange)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsZOrderAtBottom(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IAppWindowChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsZOrderAtBottom)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsZOrderAtTop(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::ComInterface::cast::<IAppWindowChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsZOrderAtTop)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ZOrderBelowWindowId(&self) -> ::windows::core::Result<super::WindowId> {
        let this = &::windows::core::ComInterface::cast::<IAppWindowChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::WindowId>();
            (::windows::core::Interface::vtable(this).ZOrderBelowWindowId)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppWindowChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowChangedEventArgs {}
impl ::core::fmt::Debug for AppWindowChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowChangedEventArgs")
            .field(&self.0)
            .finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowChangedEventArgs {
    const SIGNATURE : ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Windowing.AppWindowChangedEventArgs;{2182bc5d-fdac-5c3e-bf37-7d8d684e9d1d})" ) ;
}
impl ::core::clone::Clone for AppWindowChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppWindowChangedEventArgs {
    type Vtable = IAppWindowChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppWindowChangedEventArgs {
    const IID: ::windows::core::GUID =
        <IAppWindowChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppWindowChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowChangedEventArgs";
}
::windows::imp::interface_hierarchy!(
    AppWindowChangedEventArgs,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AppWindowChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppWindowChangedEventArgs {}
#[repr(transparent)]
pub struct AppWindowClosingEventArgs(::windows::core::IUnknown);
impl AppWindowClosingEventArgs {
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Cancel)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetCancel)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
}
impl ::core::cmp::PartialEq for AppWindowClosingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowClosingEventArgs {}
impl ::core::fmt::Debug for AppWindowClosingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowClosingEventArgs")
            .field(&self.0)
            .finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowClosingEventArgs {
    const SIGNATURE : ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Windowing.AppWindowClosingEventArgs;{0e09d90b-2261-590b-9ad1-8504991d8754})" ) ;
}
impl ::core::clone::Clone for AppWindowClosingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppWindowClosingEventArgs {
    type Vtable = IAppWindowClosingEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppWindowClosingEventArgs {
    const IID: ::windows::core::GUID =
        <IAppWindowClosingEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppWindowClosingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowClosingEventArgs";
}
::windows::imp::interface_hierarchy!(
    AppWindowClosingEventArgs,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AppWindowClosingEventArgs {}
unsafe impl ::core::marker::Sync for AppWindowClosingEventArgs {}
#[repr(transparent)]
pub struct AppWindowPresenter(::windows::core::IUnknown);
impl AppWindowPresenter {
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresenterKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowPresenterKind>();
            (::windows::core::Interface::vtable(this).Kind)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AppWindowPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowPresenter {}
impl ::core::fmt::Debug for AppWindowPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowPresenter").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowPresenter {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.AppWindowPresenter;{bc3042c2-c6c6-5632-8989-ff0ec6d3b40d})",
    );
}
impl ::core::clone::Clone for AppWindowPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppWindowPresenter {
    type Vtable = IAppWindowPresenter_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppWindowPresenter {
    const IID: ::windows::core::GUID = <IAppWindowPresenter as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppWindowPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowPresenter";
}
::windows::imp::interface_hierarchy!(
    AppWindowPresenter,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AppWindowPresenter {}
unsafe impl ::core::marker::Sync for AppWindowPresenter {}
#[repr(transparent)]
pub struct AppWindowTitleBar(::windows::core::IUnknown);
impl AppWindowTitleBar {
    pub fn BackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<
                ::windows::Foundation::IReference<::windows::UI::Color>,
            >();
            (::windows::core::Interface::vtable(this).BackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetBackgroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn ButtonBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<
                ::windows::Foundation::IReference<::windows::UI::Color>,
            >();
            (::windows::core::Interface::vtable(this).ButtonBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetButtonBackgroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetButtonBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn ButtonForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<
                ::windows::Foundation::IReference<::windows::UI::Color>,
            >();
            (::windows::core::Interface::vtable(this).ButtonForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetButtonForegroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetButtonForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn ButtonHoverBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<
                ::windows::Foundation::IReference<::windows::UI::Color>,
            >();
            (::windows::core::Interface::vtable(this).ButtonHoverBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetButtonHoverBackgroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetButtonHoverBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn ButtonHoverForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<
                ::windows::Foundation::IReference<::windows::UI::Color>,
            >();
            (::windows::core::Interface::vtable(this).ButtonHoverForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetButtonHoverForegroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetButtonHoverForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn ButtonInactiveBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<
                ::windows::Foundation::IReference<::windows::UI::Color>,
            >();
            (::windows::core::Interface::vtable(this).ButtonInactiveBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetButtonInactiveBackgroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetButtonInactiveBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn ButtonInactiveForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<
                ::windows::Foundation::IReference<::windows::UI::Color>,
            >();
            (::windows::core::Interface::vtable(this).ButtonInactiveForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetButtonInactiveForegroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetButtonInactiveForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn ButtonPressedBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<
                ::windows::Foundation::IReference<::windows::UI::Color>,
            >();
            (::windows::core::Interface::vtable(this).ButtonPressedBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetButtonPressedBackgroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetButtonPressedBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn ButtonPressedForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<
                ::windows::Foundation::IReference<::windows::UI::Color>,
            >();
            (::windows::core::Interface::vtable(this).ButtonPressedForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetButtonPressedForegroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetButtonPressedForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn ExtendsContentIntoTitleBar(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ExtendsContentIntoTitleBar)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetExtendsContentIntoTitleBar(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetExtendsContentIntoTitleBar)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn ForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<
                ::windows::Foundation::IReference<::windows::UI::Color>,
            >();
            (::windows::core::Interface::vtable(this).ForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetForegroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn Height(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).Height)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IconShowOptions(&self) -> ::windows::core::Result<IconShowOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IconShowOptions>();
            (::windows::core::Interface::vtable(this).IconShowOptions)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIconShowOptions(&self, value: IconShowOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIconShowOptions)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn InactiveBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<
                ::windows::Foundation::IReference<::windows::UI::Color>,
            >();
            (::windows::core::Interface::vtable(this).InactiveBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetInactiveBackgroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInactiveBackgroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn InactiveForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<
                ::windows::Foundation::IReference<::windows::UI::Color>,
            >();
            (::windows::core::Interface::vtable(this).InactiveForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetInactiveForegroundColor<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<::windows::Foundation::IReference<::windows::UI::Color>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInactiveForegroundColor)(
                ::windows::core::Interface::as_raw(this),
                value.try_into_param()?.abi(),
            )
            .ok()
        }
    }
    pub fn LeftInset(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).LeftInset)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RightInset(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).RightInset)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn ResetToDefault(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).ResetToDefault)(
                ::windows::core::Interface::as_raw(this),
            )
            .ok()
        }
    }
    pub fn SetDragRectangles(
        &self,
        value: &[::windows::Graphics::RectInt32],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetDragRectangles)(
                ::windows::core::Interface::as_raw(this),
                value.len() as u32,
                value.as_ptr(),
            )
            .ok()
        }
    }
    pub fn PreferredHeightOption(&self) -> ::windows::core::Result<TitleBarHeightOption> {
        let this = &::windows::core::ComInterface::cast::<IAppWindowTitleBar2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<TitleBarHeightOption>();
            (::windows::core::Interface::vtable(this).PreferredHeightOption)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetPreferredHeightOption(
        &self,
        value: TitleBarHeightOption,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IAppWindowTitleBar2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).SetPreferredHeightOption)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsCustomizationSupported() -> ::windows::core::Result<bool> {
        Self::IAppWindowTitleBarStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsCustomizationSupported)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IAppWindowTitleBarStatics<
        R,
        F: FnOnce(&IAppWindowTitleBarStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<AppWindowTitleBar, IAppWindowTitleBarStatics> =
            ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AppWindowTitleBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppWindowTitleBar {}
impl ::core::fmt::Debug for AppWindowTitleBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowTitleBar").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowTitleBar {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.AppWindowTitleBar;{5574efa2-c91c-5700-a363-539c71a7aaf4})",
    );
}
impl ::core::clone::Clone for AppWindowTitleBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for AppWindowTitleBar {
    type Vtable = IAppWindowTitleBar_Vtbl;
}
unsafe impl ::windows::core::ComInterface for AppWindowTitleBar {
    const IID: ::windows::core::GUID = <IAppWindowTitleBar as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for AppWindowTitleBar {
    const NAME: &'static str = "Microsoft.UI.Windowing.AppWindowTitleBar";
}
::windows::imp::interface_hierarchy!(
    AppWindowTitleBar,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for AppWindowTitleBar {}
unsafe impl ::core::marker::Sync for AppWindowTitleBar {}
#[repr(transparent)]
pub struct CompactOverlayPresenter(::windows::core::IUnknown);
impl CompactOverlayPresenter {
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresenterKind> {
        let this = &::windows::core::ComInterface::cast::<IAppWindowPresenter>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowPresenterKind>();
            (::windows::core::Interface::vtable(this).Kind)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn InitialSize(&self) -> ::windows::core::Result<CompactOverlaySize> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<CompactOverlaySize>();
            (::windows::core::Interface::vtable(this).InitialSize)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetInitialSize(&self, value: CompactOverlaySize) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetInitialSize)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn Create() -> ::windows::core::Result<CompactOverlayPresenter> {
        Self::ICompactOverlayPresenterStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<CompactOverlayPresenter>();
            (::windows::core::Interface::vtable(this).Create)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ICompactOverlayPresenterStatics<
        R,
        F: FnOnce(&ICompactOverlayPresenterStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<
            CompactOverlayPresenter,
            ICompactOverlayPresenterStatics,
        > = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for CompactOverlayPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CompactOverlayPresenter {}
impl ::core::fmt::Debug for CompactOverlayPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompactOverlayPresenter")
            .field(&self.0)
            .finish()
    }
}
impl ::windows::core::RuntimeType for CompactOverlayPresenter {
    const SIGNATURE : ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice ( b"rc(Microsoft.UI.Windowing.CompactOverlayPresenter;{efeb0812-6fc7-5b7d-bd92-cc8f9a6454c9})" ) ;
}
impl ::core::clone::Clone for CompactOverlayPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for CompactOverlayPresenter {
    type Vtable = ICompactOverlayPresenter_Vtbl;
}
unsafe impl ::windows::core::ComInterface for CompactOverlayPresenter {
    const IID: ::windows::core::GUID =
        <ICompactOverlayPresenter as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for CompactOverlayPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.CompactOverlayPresenter";
}
::windows::imp::interface_hierarchy!(
    CompactOverlayPresenter,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::windows::core::CanTryInto<AppWindowPresenter> for CompactOverlayPresenter {}
unsafe impl ::core::marker::Send for CompactOverlayPresenter {}
unsafe impl ::core::marker::Sync for CompactOverlayPresenter {}
#[repr(transparent)]
pub struct DisplayArea(::windows::core::IUnknown);
impl DisplayArea {
    pub fn DisplayId(&self) -> ::windows::core::Result<super::DisplayId> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::DisplayId>();
            (::windows::core::Interface::vtable(this).DisplayId)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsPrimary(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsPrimary)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn OuterBounds(&self) -> ::windows::core::Result<::windows::Graphics::RectInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::Graphics::RectInt32>();
            (::windows::core::Interface::vtable(this).OuterBounds)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn WorkArea(&self) -> ::windows::core::Result<::windows::Graphics::RectInt32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::Graphics::RectInt32>();
            (::windows::core::Interface::vtable(this).WorkArea)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Primary() -> ::windows::core::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<DisplayArea>();
            (::windows::core::Interface::vtable(this).Primary)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateWatcher() -> ::windows::core::Result<DisplayAreaWatcher> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<DisplayAreaWatcher>();
            (::windows::core::Interface::vtable(this).CreateWatcher)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FindAll(
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVectorView<DisplayArea>> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<
                ::windows::Foundation::Collections::IVectorView<DisplayArea>,
            >();
            (::windows::core::Interface::vtable(this).FindAll)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetFromWindowId(
        windowid: super::WindowId,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows::core::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<DisplayArea>();
            (::windows::core::Interface::vtable(this).GetFromWindowId)(
                ::windows::core::Interface::as_raw(this),
                windowid,
                displayareafallback,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetFromPoint(
        point: ::windows::Graphics::PointInt32,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows::core::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<DisplayArea>();
            (::windows::core::Interface::vtable(this).GetFromPoint)(
                ::windows::core::Interface::as_raw(this),
                point,
                displayareafallback,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetFromRect(
        rect: ::windows::Graphics::RectInt32,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows::core::Result<DisplayArea> {
        Self::IDisplayAreaStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<DisplayArea>();
            (::windows::core::Interface::vtable(this).GetFromRect)(
                ::windows::core::Interface::as_raw(this),
                rect,
                displayareafallback,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GetFromDisplayId(displayid: super::DisplayId) -> ::windows::core::Result<DisplayArea> {
        Self::IDisplayAreaStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<DisplayArea>();
            (::windows::core::Interface::vtable(this).GetFromDisplayId)(
                ::windows::core::Interface::as_raw(this),
                displayid,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IDisplayAreaStatics<R, F: FnOnce(&IDisplayAreaStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<DisplayArea, IDisplayAreaStatics> =
            ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IDisplayAreaStatics2<
        R,
        F: FnOnce(&IDisplayAreaStatics2) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<DisplayArea, IDisplayAreaStatics2> =
            ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for DisplayArea {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayArea {}
impl ::core::fmt::Debug for DisplayArea {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayArea").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DisplayArea {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.DisplayArea;{5c7e0537-b621-5579-bcae-a84aa8746167})",
    );
}
impl ::core::clone::Clone for DisplayArea {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DisplayArea {
    type Vtable = IDisplayArea_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DisplayArea {
    const IID: ::windows::core::GUID = <IDisplayArea as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DisplayArea {
    const NAME: &'static str = "Microsoft.UI.Windowing.DisplayArea";
}
::windows::imp::interface_hierarchy!(
    DisplayArea,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for DisplayArea {}
unsafe impl ::core::marker::Sync for DisplayArea {}
#[repr(transparent)]
pub struct DisplayAreaWatcher(::windows::core::IUnknown);
impl DisplayAreaWatcher {
    pub fn Status(&self) -> ::windows::core::Result<DisplayAreaWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<DisplayAreaWatcherStatus>();
            (::windows::core::Interface::vtable(this).Status)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Added(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<DisplayAreaWatcher, DisplayArea>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::windows::core::zeroed::<::windows::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Added)(
                ::windows::core::Interface::as_raw(this),
                ::core::mem::transmute_copy(handler),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RemoveAdded(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveAdded)(
                ::windows::core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn EnumerationCompleted(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            DisplayAreaWatcher,
            ::windows::core::IInspectable,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::windows::core::zeroed::<::windows::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).EnumerationCompleted)(
                ::windows::core::Interface::as_raw(this),
                ::core::mem::transmute_copy(handler),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RemoveEnumerationCompleted(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveEnumerationCompleted)(
                ::windows::core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Removed(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<DisplayAreaWatcher, DisplayArea>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::windows::core::zeroed::<::windows::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Removed)(
                ::windows::core::Interface::as_raw(this),
                ::core::mem::transmute_copy(handler),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RemoveRemoved(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveRemoved)(
                ::windows::core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Stopped(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<
            DisplayAreaWatcher,
            ::windows::core::IInspectable,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::windows::core::zeroed::<::windows::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Stopped)(
                ::windows::core::Interface::as_raw(this),
                ::core::mem::transmute_copy(handler),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RemoveStopped(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveStopped)(
                ::windows::core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
    pub fn Updated(
        &self,
        handler: &::windows::Foundation::TypedEventHandler<DisplayAreaWatcher, DisplayArea>,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ =
                ::windows::core::zeroed::<::windows::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Updated)(
                ::windows::core::Interface::as_raw(this),
                ::core::mem::transmute_copy(handler),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn RemoveUpdated(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).RemoveUpdated)(
                ::windows::core::Interface::as_raw(this),
                token,
            )
            .ok()
        }
    }
}
impl ::core::cmp::PartialEq for DisplayAreaWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DisplayAreaWatcher {}
impl ::core::fmt::Debug for DisplayAreaWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayAreaWatcher").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DisplayAreaWatcher {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.DisplayAreaWatcher;{83f6562f-d3a0-548b-8e4f-a99be3d95c9c})",
    );
}
impl ::core::clone::Clone for DisplayAreaWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for DisplayAreaWatcher {
    type Vtable = IDisplayAreaWatcher_Vtbl;
}
unsafe impl ::windows::core::ComInterface for DisplayAreaWatcher {
    const IID: ::windows::core::GUID = <IDisplayAreaWatcher as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for DisplayAreaWatcher {
    const NAME: &'static str = "Microsoft.UI.Windowing.DisplayAreaWatcher";
}
::windows::imp::interface_hierarchy!(
    DisplayAreaWatcher,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for DisplayAreaWatcher {}
unsafe impl ::core::marker::Sync for DisplayAreaWatcher {}
#[repr(transparent)]
pub struct FullScreenPresenter(::windows::core::IUnknown);
impl FullScreenPresenter {
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresenterKind> {
        let this = &::windows::core::ComInterface::cast::<IAppWindowPresenter>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowPresenterKind>();
            (::windows::core::Interface::vtable(this).Kind)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Create() -> ::windows::core::Result<FullScreenPresenter> {
        Self::IFullScreenPresenterStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<FullScreenPresenter>();
            (::windows::core::Interface::vtable(this).Create)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IFullScreenPresenterStatics<
        R,
        F: FnOnce(&IFullScreenPresenterStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<
            FullScreenPresenter,
            IFullScreenPresenterStatics,
        > = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for FullScreenPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FullScreenPresenter {}
impl ::core::fmt::Debug for FullScreenPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FullScreenPresenter").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for FullScreenPresenter {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.FullScreenPresenter;{fa9141fd-b8dd-5da1-8b2b-7cdadb76f593})",
    );
}
impl ::core::clone::Clone for FullScreenPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for FullScreenPresenter {
    type Vtable = IFullScreenPresenter_Vtbl;
}
unsafe impl ::windows::core::ComInterface for FullScreenPresenter {
    const IID: ::windows::core::GUID = <IFullScreenPresenter as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for FullScreenPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.FullScreenPresenter";
}
::windows::imp::interface_hierarchy!(
    FullScreenPresenter,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::windows::core::CanTryInto<AppWindowPresenter> for FullScreenPresenter {}
unsafe impl ::core::marker::Send for FullScreenPresenter {}
unsafe impl ::core::marker::Sync for FullScreenPresenter {}
#[repr(transparent)]
pub struct OverlappedPresenter(::windows::core::IUnknown);
impl OverlappedPresenter {
    pub fn Kind(&self) -> ::windows::core::Result<AppWindowPresenterKind> {
        let this = &::windows::core::ComInterface::cast::<IAppWindowPresenter>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AppWindowPresenterKind>();
            (::windows::core::Interface::vtable(this).Kind)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn HasBorder(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasBorder)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn HasTitleBar(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).HasTitleBar)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn IsAlwaysOnTop(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsAlwaysOnTop)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsAlwaysOnTop(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsAlwaysOnTop)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsMaximizable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsMaximizable)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsMaximizable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsMaximizable)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsMinimizable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsMinimizable)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsMinimizable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsMinimizable)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsModal(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsModal)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsModal(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsModal)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn IsResizable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsResizable)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn SetIsResizable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetIsResizable)(
                ::windows::core::Interface::as_raw(this),
                value,
            )
            .ok()
        }
    }
    pub fn State(&self) -> ::windows::core::Result<OverlappedPresenterState> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<OverlappedPresenterState>();
            (::windows::core::Interface::vtable(this).State)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        }
    }
    pub fn Maximize(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Maximize)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Minimize(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Minimize)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn Restore(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).Restore)(::windows::core::Interface::as_raw(
                this,
            ))
            .ok()
        }
    }
    pub fn SetBorderAndTitleBar(
        &self,
        hasborder: bool,
        hastitlebar: bool,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Interface::vtable(this).SetBorderAndTitleBar)(
                ::windows::core::Interface::as_raw(this),
                hasborder,
                hastitlebar,
            )
            .ok()
        }
    }
    pub fn MinimizeWithActivation(&self, activatewindow: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IOverlappedPresenter2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).MinimizeWithActivation)(
                ::windows::core::Interface::as_raw(this),
                activatewindow,
            )
            .ok()
        }
    }
    pub fn RestoreWithActivation(&self, activatewindow: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IOverlappedPresenter2>(self)?;
        unsafe {
            (::windows::core::Interface::vtable(this).RestoreWithActivation)(
                ::windows::core::Interface::as_raw(this),
                activatewindow,
            )
            .ok()
        }
    }
    pub fn Create() -> ::windows::core::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<OverlappedPresenter>();
            (::windows::core::Interface::vtable(this).Create)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateForContextMenu() -> ::windows::core::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<OverlappedPresenter>();
            (::windows::core::Interface::vtable(this).CreateForContextMenu)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateForDialog() -> ::windows::core::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<OverlappedPresenter>();
            (::windows::core::Interface::vtable(this).CreateForDialog)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CreateForToolWindow() -> ::windows::core::Result<OverlappedPresenter> {
        Self::IOverlappedPresenterStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<OverlappedPresenter>();
            (::windows::core::Interface::vtable(this).CreateForToolWindow)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RequestedStartupState() -> ::windows::core::Result<OverlappedPresenterState> {
        Self::IOverlappedPresenterStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<OverlappedPresenterState>();
            (::windows::core::Interface::vtable(this).RequestedStartupState)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IOverlappedPresenterStatics<
        R,
        F: FnOnce(&IOverlappedPresenterStatics) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<
            OverlappedPresenter,
            IOverlappedPresenterStatics,
        > = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IOverlappedPresenterStatics2<
        R,
        F: FnOnce(&IOverlappedPresenterStatics2) -> ::windows::core::Result<R>,
    >(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<
            OverlappedPresenter,
            IOverlappedPresenterStatics2,
        > = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for OverlappedPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OverlappedPresenter {}
impl ::core::fmt::Debug for OverlappedPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OverlappedPresenter").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for OverlappedPresenter {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Windowing.OverlappedPresenter;{21693970-4f4c-5172-9e9d-682a2d174884})",
    );
}
impl ::core::clone::Clone for OverlappedPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for OverlappedPresenter {
    type Vtable = IOverlappedPresenter_Vtbl;
}
unsafe impl ::windows::core::ComInterface for OverlappedPresenter {
    const IID: ::windows::core::GUID = <IOverlappedPresenter as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for OverlappedPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.OverlappedPresenter";
}
::windows::imp::interface_hierarchy!(
    OverlappedPresenter,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::windows::core::CanTryInto<AppWindowPresenter> for OverlappedPresenter {}
unsafe impl ::core::marker::Send for OverlappedPresenter {}
unsafe impl ::core::marker::Sync for OverlappedPresenter {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppWindowPresenterKind(pub i32);
impl AppWindowPresenterKind {
    pub const Default: Self = Self(0i32);
    pub const CompactOverlay: Self = Self(1i32);
    pub const FullScreen: Self = Self(2i32);
    pub const Overlapped: Self = Self(3i32);
}
impl ::core::marker::Copy for AppWindowPresenterKind {}
impl ::core::clone::Clone for AppWindowPresenterKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppWindowPresenterKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AppWindowPresenterKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AppWindowPresenterKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppWindowPresenterKind")
            .field(&self.0)
            .finish()
    }
}
impl ::windows::core::RuntimeType for AppWindowPresenterKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.AppWindowPresenterKind;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CompactOverlaySize(pub i32);
impl CompactOverlaySize {
    pub const Small: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Large: Self = Self(2i32);
}
impl ::core::marker::Copy for CompactOverlaySize {}
impl ::core::clone::Clone for CompactOverlaySize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CompactOverlaySize {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CompactOverlaySize {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CompactOverlaySize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompactOverlaySize").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CompactOverlaySize {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.CompactOverlaySize;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DisplayAreaFallback(pub i32);
impl DisplayAreaFallback {
    pub const None: Self = Self(0i32);
    pub const Primary: Self = Self(1i32);
    pub const Nearest: Self = Self(2i32);
}
impl ::core::marker::Copy for DisplayAreaFallback {}
impl ::core::clone::Clone for DisplayAreaFallback {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayAreaFallback {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DisplayAreaFallback {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DisplayAreaFallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayAreaFallback").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for DisplayAreaFallback {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.DisplayAreaFallback;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DisplayAreaWatcherStatus(pub i32);
impl DisplayAreaWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for DisplayAreaWatcherStatus {}
impl ::core::clone::Clone for DisplayAreaWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DisplayAreaWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DisplayAreaWatcherStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DisplayAreaWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DisplayAreaWatcherStatus")
            .field(&self.0)
            .finish()
    }
}
impl ::windows::core::RuntimeType for DisplayAreaWatcherStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.DisplayAreaWatcherStatus;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IconShowOptions(pub i32);
impl IconShowOptions {
    pub const ShowIconAndSystemMenu: Self = Self(0i32);
    pub const HideIconAndSystemMenu: Self = Self(1i32);
}
impl ::core::marker::Copy for IconShowOptions {}
impl ::core::clone::Clone for IconShowOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IconShowOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IconShowOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IconShowOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IconShowOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IconShowOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer =
        ::windows::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Windowing.IconShowOptions;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OverlappedPresenterState(pub i32);
impl OverlappedPresenterState {
    pub const Maximized: Self = Self(0i32);
    pub const Minimized: Self = Self(1i32);
    pub const Restored: Self = Self(2i32);
}
impl ::core::marker::Copy for OverlappedPresenterState {}
impl ::core::clone::Clone for OverlappedPresenterState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OverlappedPresenterState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OverlappedPresenterState {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OverlappedPresenterState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OverlappedPresenterState")
            .field(&self.0)
            .finish()
    }
}
impl ::windows::core::RuntimeType for OverlappedPresenterState {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.OverlappedPresenterState;i4)",
    );
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TitleBarHeightOption(pub i32);
impl TitleBarHeightOption {
    pub const Standard: Self = Self(0i32);
    pub const Tall: Self = Self(1i32);
}
impl ::core::marker::Copy for TitleBarHeightOption {}
impl ::core::clone::Clone for TitleBarHeightOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TitleBarHeightOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TitleBarHeightOption {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TitleBarHeightOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TitleBarHeightOption")
            .field(&self.0)
            .finish()
    }
}
impl ::windows::core::RuntimeType for TitleBarHeightOption {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(
        b"enum(Microsoft.UI.Windowing.TitleBarHeightOption;i4)",
    );
}
pub trait IAppWindow_Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<super::WindowId>;
    fn IsShownInSwitchers(&self) -> ::windows::core::Result<bool>;
    fn SetIsShownInSwitchers(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsVisible(&self) -> ::windows::core::Result<bool>;
    fn OwnerWindowId(&self) -> ::windows::core::Result<super::WindowId>;
    fn Position(&self) -> ::windows::core::Result<::windows::Graphics::PointInt32>;
    fn Presenter(&self) -> ::windows::core::Result<AppWindowPresenter>;
    fn Size(&self) -> ::windows::core::Result<::windows::Graphics::SizeInt32>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TitleBar(&self) -> ::windows::core::Result<AppWindowTitleBar>;
    fn Destroy(&self) -> ::windows::core::Result<()>;
    fn Hide(&self) -> ::windows::core::Result<()>;
    fn Move(&self, position: &::windows::Graphics::PointInt32) -> ::windows::core::Result<()>;
    fn MoveAndResize(&self, rect: &::windows::Graphics::RectInt32) -> ::windows::core::Result<()>;
    fn MoveAndResizeRelativeToDisplayArea(
        &self,
        rect: &::windows::Graphics::RectInt32,
        displayarea: ::core::option::Option<&DisplayArea>,
    ) -> ::windows::core::Result<()>;
    fn Resize(&self, size: &::windows::Graphics::SizeInt32) -> ::windows::core::Result<()>;
    fn SetIcon(&self, iconpath: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetIconWithIconId(&self, iconid: &super::IconId) -> ::windows::core::Result<()>;
    fn SetPresenter(
        &self,
        appwindowpresenter: ::core::option::Option<&AppWindowPresenter>,
    ) -> ::windows::core::Result<()>;
    fn SetPresenterByKind(
        &self,
        appwindowpresenterkind: AppWindowPresenterKind,
    ) -> ::windows::core::Result<()>;
    fn Show(&self) -> ::windows::core::Result<()>;
    fn ShowWithActivation(&self, activatewindow: bool) -> ::windows::core::Result<()>;
    fn Changed(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<AppWindow, AppWindowChangedEventArgs>,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveChanged(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()>;
    fn Closing(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<AppWindow, AppWindowClosingEventArgs>,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveClosing(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()>;
    fn Destroying(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<AppWindow, ::windows::core::IInspectable>,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveDestroying(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAppWindow {
    const NAME: &'static str = "Microsoft.UI.Windowing.IAppWindow";
}
impl IAppWindow_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IAppWindow_Impl,
        const OFFSET: isize,
    >() -> IAppWindow_Vtbl {
        unsafe extern "system" fn Id<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut super::WindowId,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsShownInSwitchers<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsShownInSwitchers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsShownInSwitchers<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsShownInSwitchers(value).into()
        }
        unsafe extern "system" fn IsVisible<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsVisible() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OwnerWindowId<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut super::WindowId,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OwnerWindowId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Graphics::PointInt32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Position() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Presenter<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Presenter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Graphics::SizeInt32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Title() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: ::std::mem::MaybeUninit<::windows::core::HSTRING>,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTitle(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn TitleBar<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TitleBar() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Destroy<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Destroy().into()
        }
        unsafe extern "system" fn Hide<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Hide().into()
        }
        unsafe extern "system" fn Move<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            position: ::windows::Graphics::PointInt32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Move(::core::mem::transmute(&position)).into()
        }
        unsafe extern "system" fn MoveAndResize<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            rect: ::windows::Graphics::RectInt32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveAndResize(::core::mem::transmute(&rect)).into()
        }
        unsafe extern "system" fn MoveAndResizeRelativeToDisplayArea<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            rect: ::windows::Graphics::RectInt32,
            displayarea: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveAndResizeRelativeToDisplayArea(
                ::core::mem::transmute(&rect),
                ::windows::core::from_raw_borrowed(&displayarea),
            )
            .into()
        }
        unsafe extern "system" fn Resize<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            size: ::windows::Graphics::SizeInt32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resize(::core::mem::transmute(&size)).into()
        }
        unsafe extern "system" fn SetIcon<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            iconpath: ::std::mem::MaybeUninit<::windows::core::HSTRING>,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIcon(::core::mem::transmute(&iconpath)).into()
        }
        unsafe extern "system" fn SetIconWithIconId<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            iconid: super::IconId,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIconWithIconId(::core::mem::transmute(&iconid))
                .into()
        }
        unsafe extern "system" fn SetPresenter<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            appwindowpresenter: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPresenter(::windows::core::from_raw_borrowed(&appwindowpresenter))
                .into()
        }
        unsafe extern "system" fn SetPresenterByKind<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            appwindowpresenterkind: AppWindowPresenterKind,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPresenterByKind(appwindowpresenterkind).into()
        }
        unsafe extern "system" fn Show<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Show().into()
        }
        unsafe extern "system" fn ShowWithActivation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            activatewindow: bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowWithActivation(activatewindow).into()
        }
        unsafe extern "system" fn Changed<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Changed(::windows::core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveChanged(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn Closing<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Closing(::windows::core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosing<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveClosing(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn Destroying<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Destroying(::windows::core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDestroying<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveDestroying(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IAppWindow, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            IsShownInSwitchers: IsShownInSwitchers::<Identity, Impl, OFFSET>,
            SetIsShownInSwitchers: SetIsShownInSwitchers::<Identity, Impl, OFFSET>,
            IsVisible: IsVisible::<Identity, Impl, OFFSET>,
            OwnerWindowId: OwnerWindowId::<Identity, Impl, OFFSET>,
            Position: Position::<Identity, Impl, OFFSET>,
            Presenter: Presenter::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            TitleBar: TitleBar::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            Hide: Hide::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            MoveAndResize: MoveAndResize::<Identity, Impl, OFFSET>,
            MoveAndResizeRelativeToDisplayArea: MoveAndResizeRelativeToDisplayArea::<
                Identity,
                Impl,
                OFFSET,
            >,
            Resize: Resize::<Identity, Impl, OFFSET>,
            SetIcon: SetIcon::<Identity, Impl, OFFSET>,
            SetIconWithIconId: SetIconWithIconId::<Identity, Impl, OFFSET>,
            SetPresenter: SetPresenter::<Identity, Impl, OFFSET>,
            SetPresenterByKind: SetPresenterByKind::<Identity, Impl, OFFSET>,
            Show: Show::<Identity, Impl, OFFSET>,
            ShowWithActivation: ShowWithActivation::<Identity, Impl, OFFSET>,
            Changed: Changed::<Identity, Impl, OFFSET>,
            RemoveChanged: RemoveChanged::<Identity, Impl, OFFSET>,
            Closing: Closing::<Identity, Impl, OFFSET>,
            RemoveClosing: RemoveClosing::<Identity, Impl, OFFSET>,
            Destroying: Destroying::<Identity, Impl, OFFSET>,
            RemoveDestroying: RemoveDestroying::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindow as ::windows::core::ComInterface>::IID
    }
}
pub trait IAppWindow2_Impl: Sized {
    fn ClientSize(&self) -> ::windows::core::Result<::windows::Graphics::SizeInt32>;
    fn MoveInZOrderAtBottom(&self) -> ::windows::core::Result<()>;
    fn MoveInZOrderAtTop(&self) -> ::windows::core::Result<()>;
    fn MoveInZOrderBelow(&self, windowid: &super::WindowId) -> ::windows::core::Result<()>;
    fn ResizeClient(&self, size: &::windows::Graphics::SizeInt32) -> ::windows::core::Result<()>;
    fn ShowOnceWithRequestedStartupState(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAppWindow2 {
    const NAME: &'static str = "Microsoft.UI.Windowing.IAppWindow2";
}
impl IAppWindow2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IAppWindow2_Impl,
        const OFFSET: isize,
    >() -> IAppWindow2_Vtbl {
        unsafe extern "system" fn ClientSize<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Graphics::SizeInt32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClientSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveInZOrderAtBottom<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveInZOrderAtBottom().into()
        }
        unsafe extern "system" fn MoveInZOrderAtTop<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveInZOrderAtTop().into()
        }
        unsafe extern "system" fn MoveInZOrderBelow<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            windowid: super::WindowId,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveInZOrderBelow(::core::mem::transmute(&windowid))
                .into()
        }
        unsafe extern "system" fn ResizeClient<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            size: ::windows::Graphics::SizeInt32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResizeClient(::core::mem::transmute(&size)).into()
        }
        unsafe extern "system" fn ShowOnceWithRequestedStartupState<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindow2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowOnceWithRequestedStartupState().into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IAppWindow2, OFFSET>(),
            ClientSize: ClientSize::<Identity, Impl, OFFSET>,
            MoveInZOrderAtBottom: MoveInZOrderAtBottom::<Identity, Impl, OFFSET>,
            MoveInZOrderAtTop: MoveInZOrderAtTop::<Identity, Impl, OFFSET>,
            MoveInZOrderBelow: MoveInZOrderBelow::<Identity, Impl, OFFSET>,
            ResizeClient: ResizeClient::<Identity, Impl, OFFSET>,
            ShowOnceWithRequestedStartupState: ShowOnceWithRequestedStartupState::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindow2 as ::windows::core::ComInterface>::IID
    }
}
pub trait IAppWindowChangedEventArgs_Impl: Sized {
    fn DidPositionChange(&self) -> ::windows::core::Result<bool>;
    fn DidPresenterChange(&self) -> ::windows::core::Result<bool>;
    fn DidSizeChange(&self) -> ::windows::core::Result<bool>;
    fn DidVisibilityChange(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IAppWindowChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Windowing.IAppWindowChangedEventArgs";
}
impl IAppWindowChangedEventArgs_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IAppWindowChangedEventArgs_Impl,
        const OFFSET: isize,
    >() -> IAppWindowChangedEventArgs_Vtbl {
        unsafe extern "system" fn DidPositionChange<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowChangedEventArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DidPositionChange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DidPresenterChange<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowChangedEventArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DidPresenterChange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DidSizeChange<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowChangedEventArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DidSizeChange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DidVisibilityChange<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowChangedEventArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DidVisibilityChange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<
                Identity,
                IAppWindowChangedEventArgs,
                OFFSET,
            >(),
            DidPositionChange: DidPositionChange::<Identity, Impl, OFFSET>,
            DidPresenterChange: DidPresenterChange::<Identity, Impl, OFFSET>,
            DidSizeChange: DidSizeChange::<Identity, Impl, OFFSET>,
            DidVisibilityChange: DidVisibilityChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowChangedEventArgs as ::windows::core::ComInterface>::IID
    }
}
pub trait IAppWindowChangedEventArgs2_Impl: Sized {
    fn DidZOrderChange(&self) -> ::windows::core::Result<bool>;
    fn IsZOrderAtBottom(&self) -> ::windows::core::Result<bool>;
    fn IsZOrderAtTop(&self) -> ::windows::core::Result<bool>;
    fn ZOrderBelowWindowId(&self) -> ::windows::core::Result<super::WindowId>;
}
impl ::windows::core::RuntimeName for IAppWindowChangedEventArgs2 {
    const NAME: &'static str = "Microsoft.UI.Windowing.IAppWindowChangedEventArgs2";
}
impl IAppWindowChangedEventArgs2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IAppWindowChangedEventArgs2_Impl,
        const OFFSET: isize,
    >() -> IAppWindowChangedEventArgs2_Vtbl {
        unsafe extern "system" fn DidZOrderChange<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowChangedEventArgs2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DidZOrderChange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsZOrderAtBottom<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowChangedEventArgs2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsZOrderAtBottom() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsZOrderAtTop<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowChangedEventArgs2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsZOrderAtTop() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZOrderBelowWindowId<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowChangedEventArgs2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut super::WindowId,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ZOrderBelowWindowId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<
                Identity,
                IAppWindowChangedEventArgs2,
                OFFSET,
            >(),
            DidZOrderChange: DidZOrderChange::<Identity, Impl, OFFSET>,
            IsZOrderAtBottom: IsZOrderAtBottom::<Identity, Impl, OFFSET>,
            IsZOrderAtTop: IsZOrderAtTop::<Identity, Impl, OFFSET>,
            ZOrderBelowWindowId: ZOrderBelowWindowId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowChangedEventArgs2 as ::windows::core::ComInterface>::IID
    }
}
pub trait IAppWindowClosingEventArgs_Impl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAppWindowClosingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Windowing.IAppWindowClosingEventArgs";
}
impl IAppWindowClosingEventArgs_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IAppWindowClosingEventArgs_Impl,
        const OFFSET: isize,
    >() -> IAppWindowClosingEventArgs_Vtbl {
        unsafe extern "system" fn Cancel<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowClosingEventArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCancel<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowClosingEventArgs_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCancel(value).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<
                Identity,
                IAppWindowClosingEventArgs,
                OFFSET,
            >(),
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            SetCancel: SetCancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowClosingEventArgs as ::windows::core::ComInterface>::IID
    }
}
pub trait IAppWindowPresenter_Impl: Sized {
    fn Kind(&self) -> ::windows::core::Result<AppWindowPresenterKind>;
}
impl ::windows::core::RuntimeName for IAppWindowPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.IAppWindowPresenter";
}
impl IAppWindowPresenter_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IAppWindowPresenter_Impl,
        const OFFSET: isize,
    >() -> IAppWindowPresenter_Vtbl {
        unsafe extern "system" fn Kind<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut AppWindowPresenterKind,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Kind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IAppWindowPresenter, OFFSET>(
            ),
            Kind: Kind::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowPresenter as ::windows::core::ComInterface>::IID
    }
}
pub trait IAppWindowPresenterFactory_Impl: Sized {}
impl ::windows::core::RuntimeName for IAppWindowPresenterFactory {
    const NAME: &'static str = "Microsoft.UI.Windowing.IAppWindowPresenterFactory";
}
impl IAppWindowPresenterFactory_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IAppWindowPresenterFactory_Impl,
        const OFFSET: isize,
    >() -> IAppWindowPresenterFactory_Vtbl {
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<
                Identity,
                IAppWindowPresenterFactory,
                OFFSET,
            >(),
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowPresenterFactory as ::windows::core::ComInterface>::IID
    }
}
pub trait IAppWindowStatics_Impl: Sized {
    fn Create(&self) -> ::windows::core::Result<AppWindow>;
    fn CreateWithPresenter(
        &self,
        appwindowpresenter: ::core::option::Option<&AppWindowPresenter>,
    ) -> ::windows::core::Result<AppWindow>;
    fn CreateWithPresenterAndOwner(
        &self,
        appwindowpresenter: ::core::option::Option<&AppWindowPresenter>,
        ownerwindowid: &super::WindowId,
    ) -> ::windows::core::Result<AppWindow>;
    fn GetFromWindowId(&self, windowid: &super::WindowId) -> ::windows::core::Result<AppWindow>;
}
impl ::windows::core::RuntimeName for IAppWindowStatics {
    const NAME: &'static str = "Microsoft.UI.Windowing.IAppWindowStatics";
}
impl IAppWindowStatics_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IAppWindowStatics_Impl,
        const OFFSET: isize,
    >() -> IAppWindowStatics_Vtbl {
        unsafe extern "system" fn Create<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithPresenter<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            appwindowpresenter: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateWithPresenter(::windows::core::from_raw_borrowed(&appwindowpresenter))
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithPresenterAndOwner<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            appwindowpresenter: *mut ::core::ffi::c_void,
            ownerwindowid: super::WindowId,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateWithPresenterAndOwner(
                ::windows::core::from_raw_borrowed(&appwindowpresenter),
                ::core::mem::transmute(&ownerwindowid),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFromWindowId<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            windowid: super::WindowId,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFromWindowId(::core::mem::transmute(&windowid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IAppWindowStatics, OFFSET>(
            ),
            Create: Create::<Identity, Impl, OFFSET>,
            CreateWithPresenter: CreateWithPresenter::<Identity, Impl, OFFSET>,
            CreateWithPresenterAndOwner: CreateWithPresenterAndOwner::<Identity, Impl, OFFSET>,
            GetFromWindowId: GetFromWindowId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowStatics as ::windows::core::ComInterface>::IID
    }
}
pub trait IAppWindowTitleBar_Impl: Sized {
    fn BackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>>;
    fn SetBackgroundColor(
        &self,
        value: ::core::option::Option<&::windows::Foundation::IReference<::windows::UI::Color>>,
    ) -> ::windows::core::Result<()>;
    fn ButtonBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>>;
    fn SetButtonBackgroundColor(
        &self,
        value: ::core::option::Option<&::windows::Foundation::IReference<::windows::UI::Color>>,
    ) -> ::windows::core::Result<()>;
    fn ButtonForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>>;
    fn SetButtonForegroundColor(
        &self,
        value: ::core::option::Option<&::windows::Foundation::IReference<::windows::UI::Color>>,
    ) -> ::windows::core::Result<()>;
    fn ButtonHoverBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>>;
    fn SetButtonHoverBackgroundColor(
        &self,
        value: ::core::option::Option<&::windows::Foundation::IReference<::windows::UI::Color>>,
    ) -> ::windows::core::Result<()>;
    fn ButtonHoverForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>>;
    fn SetButtonHoverForegroundColor(
        &self,
        value: ::core::option::Option<&::windows::Foundation::IReference<::windows::UI::Color>>,
    ) -> ::windows::core::Result<()>;
    fn ButtonInactiveBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>>;
    fn SetButtonInactiveBackgroundColor(
        &self,
        value: ::core::option::Option<&::windows::Foundation::IReference<::windows::UI::Color>>,
    ) -> ::windows::core::Result<()>;
    fn ButtonInactiveForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>>;
    fn SetButtonInactiveForegroundColor(
        &self,
        value: ::core::option::Option<&::windows::Foundation::IReference<::windows::UI::Color>>,
    ) -> ::windows::core::Result<()>;
    fn ButtonPressedBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>>;
    fn SetButtonPressedBackgroundColor(
        &self,
        value: ::core::option::Option<&::windows::Foundation::IReference<::windows::UI::Color>>,
    ) -> ::windows::core::Result<()>;
    fn ButtonPressedForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>>;
    fn SetButtonPressedForegroundColor(
        &self,
        value: ::core::option::Option<&::windows::Foundation::IReference<::windows::UI::Color>>,
    ) -> ::windows::core::Result<()>;
    fn ExtendsContentIntoTitleBar(&self) -> ::windows::core::Result<bool>;
    fn SetExtendsContentIntoTitleBar(&self, value: bool) -> ::windows::core::Result<()>;
    fn ForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>>;
    fn SetForegroundColor(
        &self,
        value: ::core::option::Option<&::windows::Foundation::IReference<::windows::UI::Color>>,
    ) -> ::windows::core::Result<()>;
    fn Height(&self) -> ::windows::core::Result<i32>;
    fn IconShowOptions(&self) -> ::windows::core::Result<IconShowOptions>;
    fn SetIconShowOptions(&self, value: IconShowOptions) -> ::windows::core::Result<()>;
    fn InactiveBackgroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>>;
    fn SetInactiveBackgroundColor(
        &self,
        value: ::core::option::Option<&::windows::Foundation::IReference<::windows::UI::Color>>,
    ) -> ::windows::core::Result<()>;
    fn InactiveForegroundColor(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IReference<::windows::UI::Color>>;
    fn SetInactiveForegroundColor(
        &self,
        value: ::core::option::Option<&::windows::Foundation::IReference<::windows::UI::Color>>,
    ) -> ::windows::core::Result<()>;
    fn LeftInset(&self) -> ::windows::core::Result<i32>;
    fn RightInset(&self) -> ::windows::core::Result<i32>;
    fn ResetToDefault(&self) -> ::windows::core::Result<()>;
    fn SetDragRectangles(
        &self,
        value: &[::windows::Graphics::RectInt32],
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAppWindowTitleBar {
    const NAME: &'static str = "Microsoft.UI.Windowing.IAppWindowTitleBar";
}
impl IAppWindowTitleBar_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IAppWindowTitleBar_Impl,
        const OFFSET: isize,
    >() -> IAppWindowTitleBar_Vtbl {
        unsafe extern "system" fn BackgroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBackgroundColor(::windows::core::from_raw_borrowed(&value))
                .into()
        }
        unsafe extern "system" fn ButtonBackgroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ButtonBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonBackgroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetButtonBackgroundColor(::windows::core::from_raw_borrowed(&value))
                .into()
        }
        unsafe extern "system" fn ButtonForegroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ButtonForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonForegroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetButtonForegroundColor(::windows::core::from_raw_borrowed(&value))
                .into()
        }
        unsafe extern "system" fn ButtonHoverBackgroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ButtonHoverBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonHoverBackgroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetButtonHoverBackgroundColor(::windows::core::from_raw_borrowed(&value))
                .into()
        }
        unsafe extern "system" fn ButtonHoverForegroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ButtonHoverForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonHoverForegroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetButtonHoverForegroundColor(::windows::core::from_raw_borrowed(&value))
                .into()
        }
        unsafe extern "system" fn ButtonInactiveBackgroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ButtonInactiveBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonInactiveBackgroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetButtonInactiveBackgroundColor(::windows::core::from_raw_borrowed(&value))
                .into()
        }
        unsafe extern "system" fn ButtonInactiveForegroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ButtonInactiveForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonInactiveForegroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetButtonInactiveForegroundColor(::windows::core::from_raw_borrowed(&value))
                .into()
        }
        unsafe extern "system" fn ButtonPressedBackgroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ButtonPressedBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonPressedBackgroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetButtonPressedBackgroundColor(::windows::core::from_raw_borrowed(&value))
                .into()
        }
        unsafe extern "system" fn ButtonPressedForegroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ButtonPressedForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetButtonPressedForegroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetButtonPressedForegroundColor(::windows::core::from_raw_borrowed(&value))
                .into()
        }
        unsafe extern "system" fn ExtendsContentIntoTitleBar<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExtendsContentIntoTitleBar() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtendsContentIntoTitleBar<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExtendsContentIntoTitleBar(value).into()
        }
        unsafe extern "system" fn ForegroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForegroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetForegroundColor(::windows::core::from_raw_borrowed(&value))
                .into()
        }
        unsafe extern "system" fn Height<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut i32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Height() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IconShowOptions<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut IconShowOptions,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IconShowOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIconShowOptions<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: IconShowOptions,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIconShowOptions(value).into()
        }
        unsafe extern "system" fn InactiveBackgroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InactiveBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInactiveBackgroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInactiveBackgroundColor(::windows::core::from_raw_borrowed(&value))
                .into()
        }
        unsafe extern "system" fn InactiveForegroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InactiveForegroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInactiveForegroundColor<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInactiveForegroundColor(::windows::core::from_raw_borrowed(&value))
                .into()
        }
        unsafe extern "system" fn LeftInset<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut i32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LeftInset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RightInset<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut i32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RightInset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetToDefault<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetToDefault().into()
        }
        unsafe extern "system" fn SetDragRectangles<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value_array_size: u32,
            value: *const ::windows::Graphics::RectInt32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDragRectangles(::core::slice::from_raw_parts(
                ::core::mem::transmute_copy(&value),
                value_array_size as _,
            ))
            .into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IAppWindowTitleBar, OFFSET>(
            ),
            BackgroundColor: BackgroundColor::<Identity, Impl, OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Identity, Impl, OFFSET>,
            ButtonBackgroundColor: ButtonBackgroundColor::<Identity, Impl, OFFSET>,
            SetButtonBackgroundColor: SetButtonBackgroundColor::<Identity, Impl, OFFSET>,
            ButtonForegroundColor: ButtonForegroundColor::<Identity, Impl, OFFSET>,
            SetButtonForegroundColor: SetButtonForegroundColor::<Identity, Impl, OFFSET>,
            ButtonHoverBackgroundColor: ButtonHoverBackgroundColor::<Identity, Impl, OFFSET>,
            SetButtonHoverBackgroundColor: SetButtonHoverBackgroundColor::<Identity, Impl, OFFSET>,
            ButtonHoverForegroundColor: ButtonHoverForegroundColor::<Identity, Impl, OFFSET>,
            SetButtonHoverForegroundColor: SetButtonHoverForegroundColor::<Identity, Impl, OFFSET>,
            ButtonInactiveBackgroundColor: ButtonInactiveBackgroundColor::<Identity, Impl, OFFSET>,
            SetButtonInactiveBackgroundColor: SetButtonInactiveBackgroundColor::<
                Identity,
                Impl,
                OFFSET,
            >,
            ButtonInactiveForegroundColor: ButtonInactiveForegroundColor::<Identity, Impl, OFFSET>,
            SetButtonInactiveForegroundColor: SetButtonInactiveForegroundColor::<
                Identity,
                Impl,
                OFFSET,
            >,
            ButtonPressedBackgroundColor: ButtonPressedBackgroundColor::<Identity, Impl, OFFSET>,
            SetButtonPressedBackgroundColor: SetButtonPressedBackgroundColor::<
                Identity,
                Impl,
                OFFSET,
            >,
            ButtonPressedForegroundColor: ButtonPressedForegroundColor::<Identity, Impl, OFFSET>,
            SetButtonPressedForegroundColor: SetButtonPressedForegroundColor::<
                Identity,
                Impl,
                OFFSET,
            >,
            ExtendsContentIntoTitleBar: ExtendsContentIntoTitleBar::<Identity, Impl, OFFSET>,
            SetExtendsContentIntoTitleBar: SetExtendsContentIntoTitleBar::<Identity, Impl, OFFSET>,
            ForegroundColor: ForegroundColor::<Identity, Impl, OFFSET>,
            SetForegroundColor: SetForegroundColor::<Identity, Impl, OFFSET>,
            Height: Height::<Identity, Impl, OFFSET>,
            IconShowOptions: IconShowOptions::<Identity, Impl, OFFSET>,
            SetIconShowOptions: SetIconShowOptions::<Identity, Impl, OFFSET>,
            InactiveBackgroundColor: InactiveBackgroundColor::<Identity, Impl, OFFSET>,
            SetInactiveBackgroundColor: SetInactiveBackgroundColor::<Identity, Impl, OFFSET>,
            InactiveForegroundColor: InactiveForegroundColor::<Identity, Impl, OFFSET>,
            SetInactiveForegroundColor: SetInactiveForegroundColor::<Identity, Impl, OFFSET>,
            LeftInset: LeftInset::<Identity, Impl, OFFSET>,
            RightInset: RightInset::<Identity, Impl, OFFSET>,
            ResetToDefault: ResetToDefault::<Identity, Impl, OFFSET>,
            SetDragRectangles: SetDragRectangles::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowTitleBar as ::windows::core::ComInterface>::IID
    }
}
pub trait IAppWindowTitleBar2_Impl: Sized {
    fn PreferredHeightOption(&self) -> ::windows::core::Result<TitleBarHeightOption>;
    fn SetPreferredHeightOption(&self, value: TitleBarHeightOption) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAppWindowTitleBar2 {
    const NAME: &'static str = "Microsoft.UI.Windowing.IAppWindowTitleBar2";
}
impl IAppWindowTitleBar2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IAppWindowTitleBar2_Impl,
        const OFFSET: isize,
    >() -> IAppWindowTitleBar2_Vtbl {
        unsafe extern "system" fn PreferredHeightOption<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut TitleBarHeightOption,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PreferredHeightOption() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredHeightOption<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBar2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: TitleBarHeightOption,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPreferredHeightOption(value).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IAppWindowTitleBar2, OFFSET>(
            ),
            PreferredHeightOption: PreferredHeightOption::<Identity, Impl, OFFSET>,
            SetPreferredHeightOption: SetPreferredHeightOption::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowTitleBar2 as ::windows::core::ComInterface>::IID
    }
}
pub trait IAppWindowTitleBarStatics_Impl: Sized {
    fn IsCustomizationSupported(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IAppWindowTitleBarStatics {
    const NAME: &'static str = "Microsoft.UI.Windowing.IAppWindowTitleBarStatics";
}
impl IAppWindowTitleBarStatics_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IAppWindowTitleBarStatics_Impl,
        const OFFSET: isize,
    >() -> IAppWindowTitleBarStatics_Vtbl {
        unsafe extern "system" fn IsCustomizationSupported<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IAppWindowTitleBarStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCustomizationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<
                Identity,
                IAppWindowTitleBarStatics,
                OFFSET,
            >(),
            IsCustomizationSupported: IsCustomizationSupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppWindowTitleBarStatics as ::windows::core::ComInterface>::IID
    }
}
pub trait ICompactOverlayPresenter_Impl: Sized {
    fn InitialSize(&self) -> ::windows::core::Result<CompactOverlaySize>;
    fn SetInitialSize(&self, value: CompactOverlaySize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICompactOverlayPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.ICompactOverlayPresenter";
}
impl ICompactOverlayPresenter_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ICompactOverlayPresenter_Impl,
        const OFFSET: isize,
    >() -> ICompactOverlayPresenter_Vtbl {
        unsafe extern "system" fn InitialSize<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICompactOverlayPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut CompactOverlaySize,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitialSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialSize<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICompactOverlayPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: CompactOverlaySize,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInitialSize(value).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<
                Identity,
                ICompactOverlayPresenter,
                OFFSET,
            >(),
            InitialSize: InitialSize::<Identity, Impl, OFFSET>,
            SetInitialSize: SetInitialSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompactOverlayPresenter as ::windows::core::ComInterface>::IID
    }
}
pub trait ICompactOverlayPresenterStatics_Impl: Sized {
    fn Create(&self) -> ::windows::core::Result<CompactOverlayPresenter>;
}
impl ::windows::core::RuntimeName for ICompactOverlayPresenterStatics {
    const NAME: &'static str = "Microsoft.UI.Windowing.ICompactOverlayPresenterStatics";
}
impl ICompactOverlayPresenterStatics_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ICompactOverlayPresenterStatics_Impl,
        const OFFSET: isize,
    >() -> ICompactOverlayPresenterStatics_Vtbl {
        unsafe extern "system" fn Create<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICompactOverlayPresenterStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<
                Identity,
                ICompactOverlayPresenterStatics,
                OFFSET,
            >(),
            Create: Create::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompactOverlayPresenterStatics as ::windows::core::ComInterface>::IID
    }
}
pub trait IDisplayArea_Impl: Sized {
    fn DisplayId(&self) -> ::windows::core::Result<super::DisplayId>;
    fn IsPrimary(&self) -> ::windows::core::Result<bool>;
    fn OuterBounds(&self) -> ::windows::core::Result<::windows::Graphics::RectInt32>;
    fn WorkArea(&self) -> ::windows::core::Result<::windows::Graphics::RectInt32>;
}
impl ::windows::core::RuntimeName for IDisplayArea {
    const NAME: &'static str = "Microsoft.UI.Windowing.IDisplayArea";
}
impl IDisplayArea_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IDisplayArea_Impl,
        const OFFSET: isize,
    >() -> IDisplayArea_Vtbl {
        unsafe extern "system" fn DisplayId<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayArea_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut super::DisplayId,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPrimary<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayArea_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPrimary() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OuterBounds<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayArea_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Graphics::RectInt32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OuterBounds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WorkArea<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayArea_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Graphics::RectInt32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WorkArea() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IDisplayArea, OFFSET>(),
            DisplayId: DisplayId::<Identity, Impl, OFFSET>,
            IsPrimary: IsPrimary::<Identity, Impl, OFFSET>,
            OuterBounds: OuterBounds::<Identity, Impl, OFFSET>,
            WorkArea: WorkArea::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayArea as ::windows::core::ComInterface>::IID
    }
}
pub trait IDisplayAreaStatics_Impl: Sized {
    fn Primary(&self) -> ::windows::core::Result<DisplayArea>;
    fn CreateWatcher(&self) -> ::windows::core::Result<DisplayAreaWatcher>;
    fn FindAll(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Collections::IVectorView<DisplayArea>>;
    fn GetFromWindowId(
        &self,
        windowid: &super::WindowId,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows::core::Result<DisplayArea>;
    fn GetFromPoint(
        &self,
        point: &::windows::Graphics::PointInt32,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows::core::Result<DisplayArea>;
    fn GetFromRect(
        &self,
        rect: &::windows::Graphics::RectInt32,
        displayareafallback: DisplayAreaFallback,
    ) -> ::windows::core::Result<DisplayArea>;
}
impl ::windows::core::RuntimeName for IDisplayAreaStatics {
    const NAME: &'static str = "Microsoft.UI.Windowing.IDisplayAreaStatics";
}
impl IDisplayAreaStatics_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IDisplayAreaStatics_Impl,
        const OFFSET: isize,
    >() -> IDisplayAreaStatics_Vtbl {
        unsafe extern "system" fn Primary<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Primary() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWatcher<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAll<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindAll() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFromWindowId<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            windowid: super::WindowId,
            displayareafallback: DisplayAreaFallback,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFromWindowId(::core::mem::transmute(&windowid), displayareafallback) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFromPoint<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            point: ::windows::Graphics::PointInt32,
            displayareafallback: DisplayAreaFallback,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFromPoint(::core::mem::transmute(&point), displayareafallback) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFromRect<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            rect: ::windows::Graphics::RectInt32,
            displayareafallback: DisplayAreaFallback,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFromRect(::core::mem::transmute(&rect), displayareafallback) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IDisplayAreaStatics, OFFSET>(
            ),
            Primary: Primary::<Identity, Impl, OFFSET>,
            CreateWatcher: CreateWatcher::<Identity, Impl, OFFSET>,
            FindAll: FindAll::<Identity, Impl, OFFSET>,
            GetFromWindowId: GetFromWindowId::<Identity, Impl, OFFSET>,
            GetFromPoint: GetFromPoint::<Identity, Impl, OFFSET>,
            GetFromRect: GetFromRect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayAreaStatics as ::windows::core::ComInterface>::IID
    }
}
pub trait IDisplayAreaStatics2_Impl: Sized {
    fn GetFromDisplayId(
        &self,
        displayid: &super::DisplayId,
    ) -> ::windows::core::Result<DisplayArea>;
}
impl ::windows::core::RuntimeName for IDisplayAreaStatics2 {
    const NAME: &'static str = "Microsoft.UI.Windowing.IDisplayAreaStatics2";
}
impl IDisplayAreaStatics2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IDisplayAreaStatics2_Impl,
        const OFFSET: isize,
    >() -> IDisplayAreaStatics2_Vtbl {
        unsafe extern "system" fn GetFromDisplayId<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaStatics2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            displayid: super::DisplayId,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFromDisplayId(::core::mem::transmute(&displayid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IDisplayAreaStatics2, OFFSET>(
            ),
            GetFromDisplayId: GetFromDisplayId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayAreaStatics2 as ::windows::core::ComInterface>::IID
    }
}
pub trait IDisplayAreaWatcher_Impl: Sized {
    fn Status(&self) -> ::windows::core::Result<DisplayAreaWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Added(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<DisplayAreaWatcher, DisplayArea>,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveAdded(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<
                DisplayAreaWatcher,
                ::windows::core::IInspectable,
            >,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()>;
    fn Removed(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<DisplayAreaWatcher, DisplayArea>,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()>;
    fn Stopped(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<
                DisplayAreaWatcher,
                ::windows::core::IInspectable,
            >,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveStopped(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()>;
    fn Updated(
        &self,
        handler: ::core::option::Option<
            &::windows::Foundation::TypedEventHandler<DisplayAreaWatcher, DisplayArea>,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(
        &self,
        token: &::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDisplayAreaWatcher {
    const NAME: &'static str = "Microsoft.UI.Windowing.IDisplayAreaWatcher";
}
impl IDisplayAreaWatcher_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IDisplayAreaWatcher_Impl,
        const OFFSET: isize,
    >() -> IDisplayAreaWatcher_Vtbl {
        unsafe extern "system" fn Status<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaWatcher_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut DisplayAreaWatcherStatus,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Status() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaWatcher_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start().into()
        }
        unsafe extern "system" fn Stop<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaWatcher_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn Added<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaWatcher_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Added(::windows::core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdded<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaWatcher_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAdded(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaWatcher_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerationCompleted(::windows::core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaWatcher_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveEnumerationCompleted(::core::mem::transmute(&token))
                .into()
        }
        unsafe extern "system" fn Removed<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaWatcher_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Removed(::windows::core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoved<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaWatcher_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveRemoved(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn Stopped<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaWatcher_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Stopped(::windows::core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaWatcher_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStopped(::core::mem::transmute(&token)).into()
        }
        unsafe extern "system" fn Updated<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaWatcher_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            handler: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Updated(::windows::core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdated<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IDisplayAreaWatcher_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            token: ::windows::Foundation::EventRegistrationToken,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveUpdated(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IDisplayAreaWatcher, OFFSET>(
            ),
            Status: Status::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Added: Added::<Identity, Impl, OFFSET>,
            RemoveAdded: RemoveAdded::<Identity, Impl, OFFSET>,
            EnumerationCompleted: EnumerationCompleted::<Identity, Impl, OFFSET>,
            RemoveEnumerationCompleted: RemoveEnumerationCompleted::<Identity, Impl, OFFSET>,
            Removed: Removed::<Identity, Impl, OFFSET>,
            RemoveRemoved: RemoveRemoved::<Identity, Impl, OFFSET>,
            Stopped: Stopped::<Identity, Impl, OFFSET>,
            RemoveStopped: RemoveStopped::<Identity, Impl, OFFSET>,
            Updated: Updated::<Identity, Impl, OFFSET>,
            RemoveUpdated: RemoveUpdated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayAreaWatcher as ::windows::core::ComInterface>::IID
    }
}
pub trait IFullScreenPresenter_Impl: Sized {}
impl ::windows::core::RuntimeName for IFullScreenPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.IFullScreenPresenter";
}
impl IFullScreenPresenter_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFullScreenPresenter_Impl,
        const OFFSET: isize,
    >() -> IFullScreenPresenter_Vtbl {
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IFullScreenPresenter, OFFSET>(
            ),
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFullScreenPresenter as ::windows::core::ComInterface>::IID
    }
}
pub trait IFullScreenPresenterStatics_Impl: Sized {
    fn Create(&self) -> ::windows::core::Result<FullScreenPresenter>;
}
impl ::windows::core::RuntimeName for IFullScreenPresenterStatics {
    const NAME: &'static str = "Microsoft.UI.Windowing.IFullScreenPresenterStatics";
}
impl IFullScreenPresenterStatics_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IFullScreenPresenterStatics_Impl,
        const OFFSET: isize,
    >() -> IFullScreenPresenterStatics_Vtbl {
        unsafe extern "system" fn Create<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IFullScreenPresenterStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<
                Identity,
                IFullScreenPresenterStatics,
                OFFSET,
            >(),
            Create: Create::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFullScreenPresenterStatics as ::windows::core::ComInterface>::IID
    }
}
pub trait IOverlappedPresenter_Impl: Sized {
    fn HasBorder(&self) -> ::windows::core::Result<bool>;
    fn HasTitleBar(&self) -> ::windows::core::Result<bool>;
    fn IsAlwaysOnTop(&self) -> ::windows::core::Result<bool>;
    fn SetIsAlwaysOnTop(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsMaximizable(&self) -> ::windows::core::Result<bool>;
    fn SetIsMaximizable(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsMinimizable(&self) -> ::windows::core::Result<bool>;
    fn SetIsMinimizable(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsModal(&self) -> ::windows::core::Result<bool>;
    fn SetIsModal(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsResizable(&self) -> ::windows::core::Result<bool>;
    fn SetIsResizable(&self, value: bool) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<OverlappedPresenterState>;
    fn Maximize(&self) -> ::windows::core::Result<()>;
    fn Minimize(&self) -> ::windows::core::Result<()>;
    fn Restore(&self) -> ::windows::core::Result<()>;
    fn SetBorderAndTitleBar(
        &self,
        hasborder: bool,
        hastitlebar: bool,
    ) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IOverlappedPresenter {
    const NAME: &'static str = "Microsoft.UI.Windowing.IOverlappedPresenter";
}
impl IOverlappedPresenter_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IOverlappedPresenter_Impl,
        const OFFSET: isize,
    >() -> IOverlappedPresenter_Vtbl {
        unsafe extern "system" fn HasBorder<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasBorder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasTitleBar<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasTitleBar() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAlwaysOnTop<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsAlwaysOnTop() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAlwaysOnTop<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsAlwaysOnTop(value).into()
        }
        unsafe extern "system" fn IsMaximizable<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsMaximizable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsMaximizable<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsMaximizable(value).into()
        }
        unsafe extern "system" fn IsMinimizable<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsMinimizable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsMinimizable<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsMinimizable(value).into()
        }
        unsafe extern "system" fn IsModal<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsModal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsModal<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsModal(value).into()
        }
        unsafe extern "system" fn IsResizable<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsResizable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsResizable<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsResizable(value).into()
        }
        unsafe extern "system" fn State<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut OverlappedPresenterState,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.State() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Maximize<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Maximize().into()
        }
        unsafe extern "system" fn Minimize<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Minimize().into()
        }
        unsafe extern "system" fn Restore<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Restore().into()
        }
        unsafe extern "system" fn SetBorderAndTitleBar<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            hasborder: bool,
            hastitlebar: bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBorderAndTitleBar(hasborder, hastitlebar).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IOverlappedPresenter, OFFSET>(
            ),
            HasBorder: HasBorder::<Identity, Impl, OFFSET>,
            HasTitleBar: HasTitleBar::<Identity, Impl, OFFSET>,
            IsAlwaysOnTop: IsAlwaysOnTop::<Identity, Impl, OFFSET>,
            SetIsAlwaysOnTop: SetIsAlwaysOnTop::<Identity, Impl, OFFSET>,
            IsMaximizable: IsMaximizable::<Identity, Impl, OFFSET>,
            SetIsMaximizable: SetIsMaximizable::<Identity, Impl, OFFSET>,
            IsMinimizable: IsMinimizable::<Identity, Impl, OFFSET>,
            SetIsMinimizable: SetIsMinimizable::<Identity, Impl, OFFSET>,
            IsModal: IsModal::<Identity, Impl, OFFSET>,
            SetIsModal: SetIsModal::<Identity, Impl, OFFSET>,
            IsResizable: IsResizable::<Identity, Impl, OFFSET>,
            SetIsResizable: SetIsResizable::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Maximize: Maximize::<Identity, Impl, OFFSET>,
            Minimize: Minimize::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            SetBorderAndTitleBar: SetBorderAndTitleBar::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOverlappedPresenter as ::windows::core::ComInterface>::IID
    }
}
pub trait IOverlappedPresenter2_Impl: Sized {
    fn MinimizeWithActivation(&self, activatewindow: bool) -> ::windows::core::Result<()>;
    fn RestoreWithActivation(&self, activatewindow: bool) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IOverlappedPresenter2 {
    const NAME: &'static str = "Microsoft.UI.Windowing.IOverlappedPresenter2";
}
impl IOverlappedPresenter2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IOverlappedPresenter2_Impl,
        const OFFSET: isize,
    >() -> IOverlappedPresenter2_Vtbl {
        unsafe extern "system" fn MinimizeWithActivation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            activatewindow: bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MinimizeWithActivation(activatewindow).into()
        }
        unsafe extern "system" fn RestoreWithActivation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenter2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            activatewindow: bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreWithActivation(activatewindow).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<
                Identity,
                IOverlappedPresenter2,
                OFFSET,
            >(),
            MinimizeWithActivation: MinimizeWithActivation::<Identity, Impl, OFFSET>,
            RestoreWithActivation: RestoreWithActivation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOverlappedPresenter2 as ::windows::core::ComInterface>::IID
    }
}
pub trait IOverlappedPresenterStatics_Impl: Sized {
    fn Create(&self) -> ::windows::core::Result<OverlappedPresenter>;
    fn CreateForContextMenu(&self) -> ::windows::core::Result<OverlappedPresenter>;
    fn CreateForDialog(&self) -> ::windows::core::Result<OverlappedPresenter>;
    fn CreateForToolWindow(&self) -> ::windows::core::Result<OverlappedPresenter>;
}
impl ::windows::core::RuntimeName for IOverlappedPresenterStatics {
    const NAME: &'static str = "Microsoft.UI.Windowing.IOverlappedPresenterStatics";
}
impl IOverlappedPresenterStatics_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IOverlappedPresenterStatics_Impl,
        const OFFSET: isize,
    >() -> IOverlappedPresenterStatics_Vtbl {
        unsafe extern "system" fn Create<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenterStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForContextMenu<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenterStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateForContextMenu() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForDialog<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenterStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateForDialog() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForToolWindow<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenterStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateForToolWindow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<
                Identity,
                IOverlappedPresenterStatics,
                OFFSET,
            >(),
            Create: Create::<Identity, Impl, OFFSET>,
            CreateForContextMenu: CreateForContextMenu::<Identity, Impl, OFFSET>,
            CreateForDialog: CreateForDialog::<Identity, Impl, OFFSET>,
            CreateForToolWindow: CreateForToolWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOverlappedPresenterStatics as ::windows::core::ComInterface>::IID
    }
}
pub trait IOverlappedPresenterStatics2_Impl: Sized {
    fn RequestedStartupState(&self) -> ::windows::core::Result<OverlappedPresenterState>;
}
impl ::windows::core::RuntimeName for IOverlappedPresenterStatics2 {
    const NAME: &'static str = "Microsoft.UI.Windowing.IOverlappedPresenterStatics2";
}
impl IOverlappedPresenterStatics2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IOverlappedPresenterStatics2_Impl,
        const OFFSET: isize,
    >() -> IOverlappedPresenterStatics2_Vtbl {
        unsafe extern "system" fn RequestedStartupState<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IOverlappedPresenterStatics2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut OverlappedPresenterState,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestedStartupState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<
                Identity,
                IOverlappedPresenterStatics2,
                OFFSET,
            >(),
            RequestedStartupState: RequestedStartupState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOverlappedPresenterStatics2 as ::windows::core::ComInterface>::IID
    }
}
