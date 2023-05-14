#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
// pub mod Composition;
// pub mod Dispatching;
// pub mod Input;
// pub mod Text;
pub mod Windowing;
// pub mod Xaml;
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorHelper(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorHelper {
    type Vtable = IColorHelper_Vtbl;
}
impl ::core::clone::Clone for IColorHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IColorHelper {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x3adddccd_3949_585b_a566_ccb8350dd221);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelper_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorHelperStatics {
    type Vtable = IColorHelperStatics_Vtbl;
}
impl ::core::clone::Clone for IColorHelperStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IColorHelperStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x1d1d85a1_eb63_538a_84f0_019210bc406b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelperStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FromArgb: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        a: u8,
        r: u8,
        g: u8,
        b: u8,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColors(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColors {
    type Vtable = IColors_Vtbl;
}
impl ::core::clone::Clone for IColors {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IColors {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8cf15863_8411_5afd_946c_328e04da2f2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColors_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IColorsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IColorsStatics {
    type Vtable = IColorsStatics_Vtbl;
}
impl ::core::clone::Clone for IColorsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IColorsStatics {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x8620a5b0_015a_57ac_a3f3_895d0b1269ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorsStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AliceBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub AntiqueWhite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Aqua: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Aquamarine: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Azure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Beige: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Bisque: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Black: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub BlanchedAlmond: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Blue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub BlueViolet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Brown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub BurlyWood: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub CadetBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Chartreuse: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Chocolate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Coral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub CornflowerBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Cornsilk: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Crimson: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Cyan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkCyan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkGoldenrod: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkKhaki: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkMagenta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkOliveGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkOrange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkOrchid: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkRed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkSalmon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkSeaGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkSlateBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkSlateGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkTurquoise: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DarkViolet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DeepPink: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DeepSkyBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DimGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub DodgerBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Firebrick: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub FloralWhite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub ForestGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Fuchsia: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Gainsboro: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub GhostWhite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Gold: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Goldenrod: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Gray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Green: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub GreenYellow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Honeydew: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub HotPink: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub IndianRed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Indigo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Ivory: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Khaki: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Lavender: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LavenderBlush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LawnGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LemonChiffon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightCoral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightCyan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightGoldenrodYellow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightPink: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightSalmon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightSeaGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightSkyBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightSlateGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightSteelBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LightYellow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Lime: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub LimeGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Linen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Magenta: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Maroon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumAquamarine: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumOrchid: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumPurple: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumSeaGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumSlateBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumSpringGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumTurquoise: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MediumVioletRed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MidnightBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MintCream: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub MistyRose: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Moccasin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub NavajoWhite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Navy: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub OldLace: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Olive: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub OliveDrab: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Orange: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub OrangeRed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Orchid: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub PaleGoldenrod: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub PaleGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub PaleTurquoise: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub PaleVioletRed: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub PapayaWhip: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub PeachPuff: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Peru: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Pink: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Plum: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub PowderBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Purple: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Red: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub RosyBrown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub RoyalBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SaddleBrown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Salmon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SandyBrown: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SeaGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SeaShell: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Sienna: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Silver: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SkyBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SlateBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SlateGray: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Snow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SpringGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub SteelBlue: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Tan: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Teal: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Thistle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Tomato: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Transparent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Turquoise: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Violet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Wheat: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub White: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub WhiteSmoke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub Yellow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    pub YellowGreen: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct ColorHelper(::windows::core::IUnknown);
impl ColorHelper {
    pub fn FromArgb(a: u8, r: u8, g: u8, b: u8) -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorHelperStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).FromArgb)(
                ::windows::core::Interface::as_raw(this),
                a,
                r,
                g,
                b,
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IColorHelperStatics<R, F: FnOnce(&IColorHelperStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<ColorHelper, IColorHelperStatics> =
            ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for ColorHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ColorHelper {}
impl ::core::fmt::Debug for ColorHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorHelper").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ColorHelper {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.ColorHelper;{3adddccd-3949-585b-a566-ccb8350dd221})",
    );
}
impl ::core::clone::Clone for ColorHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ColorHelper {
    type Vtable = IColorHelper_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ColorHelper {
    const IID: ::windows::core::GUID = <IColorHelper as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ColorHelper {
    const NAME: &'static str = "Microsoft.UI.ColorHelper";
}
::windows::imp::interface_hierarchy!(
    ColorHelper,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for ColorHelper {}
unsafe impl ::core::marker::Sync for ColorHelper {}
#[repr(transparent)]
pub struct Colors(::windows::core::IUnknown);
impl Colors {
    pub fn AliceBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).AliceBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn AntiqueWhite() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).AntiqueWhite)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Aqua() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Aqua)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Aquamarine() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Aquamarine)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Azure() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Azure)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Beige() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Beige)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Bisque() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Bisque)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Black() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Black)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn BlanchedAlmond() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).BlanchedAlmond)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Blue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Blue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn BlueViolet() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).BlueViolet)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Brown() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Brown)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn BurlyWood() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).BurlyWood)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CadetBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).CadetBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Chartreuse() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Chartreuse)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Chocolate() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Chocolate)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Coral() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Coral)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn CornflowerBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).CornflowerBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Cornsilk() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Cornsilk)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Crimson() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Crimson)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Cyan() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Cyan)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DarkBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DarkBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DarkCyan() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DarkCyan)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DarkGoldenrod() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DarkGoldenrod)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DarkGray() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DarkGray)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DarkGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DarkGreen)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DarkKhaki() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DarkKhaki)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DarkMagenta() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DarkMagenta)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DarkOliveGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DarkOliveGreen)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DarkOrange() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DarkOrange)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DarkOrchid() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DarkOrchid)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DarkRed() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DarkRed)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DarkSalmon() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DarkSalmon)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DarkSeaGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DarkSeaGreen)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DarkSlateBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DarkSlateBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DarkSlateGray() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DarkSlateGray)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DarkTurquoise() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DarkTurquoise)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DarkViolet() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DarkViolet)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DeepPink() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DeepPink)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DeepSkyBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DeepSkyBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DimGray() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DimGray)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn DodgerBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).DodgerBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Firebrick() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Firebrick)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn FloralWhite() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).FloralWhite)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn ForestGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).ForestGreen)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Fuchsia() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Fuchsia)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Gainsboro() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Gainsboro)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GhostWhite() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).GhostWhite)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Gold() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Gold)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Goldenrod() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Goldenrod)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Gray() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Gray)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Green() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Green)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn GreenYellow() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).GreenYellow)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Honeydew() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Honeydew)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn HotPink() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).HotPink)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IndianRed() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).IndianRed)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Indigo() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Indigo)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Ivory() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Ivory)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Khaki() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Khaki)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Lavender() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Lavender)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LavenderBlush() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).LavenderBlush)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LawnGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).LawnGreen)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LemonChiffon() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).LemonChiffon)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LightBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).LightBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LightCoral() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).LightCoral)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LightCyan() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).LightCyan)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LightGoldenrodYellow() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).LightGoldenrodYellow)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LightGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).LightGreen)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LightGray() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).LightGray)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LightPink() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).LightPink)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LightSalmon() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).LightSalmon)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LightSeaGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).LightSeaGreen)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LightSkyBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).LightSkyBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LightSlateGray() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).LightSlateGray)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LightSteelBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).LightSteelBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LightYellow() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).LightYellow)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Lime() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Lime)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn LimeGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).LimeGreen)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Linen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Linen)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Magenta() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Magenta)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Maroon() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Maroon)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MediumAquamarine() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).MediumAquamarine)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MediumBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).MediumBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MediumOrchid() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).MediumOrchid)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MediumPurple() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).MediumPurple)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MediumSeaGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).MediumSeaGreen)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MediumSlateBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).MediumSlateBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MediumSpringGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).MediumSpringGreen)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MediumTurquoise() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).MediumTurquoise)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MediumVioletRed() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).MediumVioletRed)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MidnightBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).MidnightBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MintCream() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).MintCream)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn MistyRose() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).MistyRose)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Moccasin() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Moccasin)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn NavajoWhite() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).NavajoWhite)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Navy() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Navy)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn OldLace() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).OldLace)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Olive() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Olive)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn OliveDrab() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).OliveDrab)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Orange() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Orange)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn OrangeRed() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).OrangeRed)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Orchid() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Orchid)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn PaleGoldenrod() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).PaleGoldenrod)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn PaleGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).PaleGreen)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn PaleTurquoise() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).PaleTurquoise)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn PaleVioletRed() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).PaleVioletRed)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn PapayaWhip() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).PapayaWhip)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn PeachPuff() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).PeachPuff)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Peru() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Peru)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Pink() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Pink)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Plum() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Plum)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn PowderBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).PowderBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Purple() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Purple)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Red() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Red)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RosyBrown() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).RosyBrown)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn RoyalBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).RoyalBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SaddleBrown() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).SaddleBrown)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Salmon() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Salmon)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SandyBrown() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).SandyBrown)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SeaGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).SeaGreen)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SeaShell() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).SeaShell)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Sienna() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Sienna)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Silver() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Silver)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SkyBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).SkyBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SlateBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).SlateBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SlateGray() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).SlateGray)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Snow() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Snow)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SpringGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).SpringGreen)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn SteelBlue() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).SteelBlue)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Tan() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Tan)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Teal() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Teal)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Thistle() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Thistle)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Tomato() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Tomato)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Transparent() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Transparent)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Turquoise() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Turquoise)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Violet() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Violet)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Wheat() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Wheat)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn White() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).White)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn WhiteSmoke() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).WhiteSmoke)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn Yellow() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).Yellow)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn YellowGreen() -> ::windows::core::Result<::windows::UI::Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::UI::Color>();
            (::windows::core::Interface::vtable(this).YellowGreen)(
                ::windows::core::Interface::as_raw(this),
                &mut result__,
            )
            .from_abi(result__)
        })
    }
    pub fn IColorsStatics<R, F: FnOnce(&IColorsStatics) -> ::windows::core::Result<R>>(
        callback: F,
    ) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Colors, IColorsStatics> =
            ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Colors {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Colors {}
impl ::core::fmt::Debug for Colors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Colors").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Colors {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(
        b"rc(Microsoft.UI.Colors;{8cf15863-8411-5afd-946c-328e04da2f2f})",
    );
}
impl ::core::clone::Clone for Colors {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Colors {
    type Vtable = IColors_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Colors {
    const IID: ::windows::core::GUID = <IColors as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Colors {
    const NAME: &'static str = "Microsoft.UI.Colors";
}
::windows::imp::interface_hierarchy!(
    Colors,
    ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for Colors {}
unsafe impl ::core::marker::Sync for Colors {}
#[repr(C)]
pub struct DisplayId {
    pub Value: u64,
}
impl ::core::marker::Copy for DisplayId {}
impl ::core::clone::Clone for DisplayId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DisplayId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DisplayId")
            .field("Value", &self.Value)
            .finish()
    }
}
impl ::windows::core::TypeKind for DisplayId {
    type TypeKind = ::windows::core::CopyType;
}
impl ::windows::core::RuntimeType for DisplayId {
    const SIGNATURE: ::windows::imp::ConstBuffer =
        ::windows::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.DisplayId;u8)");
}
impl ::core::cmp::PartialEq for DisplayId {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::core::cmp::Eq for DisplayId {}
impl ::core::default::Default for DisplayId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct IconId {
    pub Value: u64,
}
impl ::core::marker::Copy for IconId {}
impl ::core::clone::Clone for IconId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IconId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IconId")
            .field("Value", &self.Value)
            .finish()
    }
}
impl ::windows::core::TypeKind for IconId {
    type TypeKind = ::windows::core::CopyType;
}
impl ::windows::core::RuntimeType for IconId {
    const SIGNATURE: ::windows::imp::ConstBuffer =
        ::windows::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.IconId;u8)");
}
impl ::core::cmp::PartialEq for IconId {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::core::cmp::Eq for IconId {}
impl ::core::default::Default for IconId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WindowId {
    pub Value: u64,
}
impl ::core::marker::Copy for WindowId {}
impl ::core::clone::Clone for WindowId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WindowId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WindowId")
            .field("Value", &self.Value)
            .finish()
    }
}
impl ::windows::core::TypeKind for WindowId {
    type TypeKind = ::windows::core::CopyType;
}
impl ::windows::core::RuntimeType for WindowId {
    const SIGNATURE: ::windows::imp::ConstBuffer =
        ::windows::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.WindowId;u8)");
}
impl ::core::cmp::PartialEq for WindowId {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::core::cmp::Eq for WindowId {}
impl ::core::default::Default for WindowId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub trait IColorHelper_Impl: Sized {}
impl ::windows::core::RuntimeName for IColorHelper {
    const NAME: &'static str = "Microsoft.UI.IColorHelper";
}
impl IColorHelper_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IColorHelper_Impl,
        const OFFSET: isize,
    >() -> IColorHelper_Vtbl {
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IColorHelper, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorHelper as ::windows::core::ComInterface>::IID
    }
}
pub trait IColorHelperStatics_Impl: Sized {
    fn FromArgb(&self, a: u8, r: u8, g: u8, b: u8)
        -> ::windows::core::Result<::windows::UI::Color>;
}
impl ::windows::core::RuntimeName for IColorHelperStatics {
    const NAME: &'static str = "Microsoft.UI.IColorHelperStatics";
}
impl IColorHelperStatics_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IColorHelperStatics_Impl,
        const OFFSET: isize,
    >() -> IColorHelperStatics_Vtbl {
        unsafe extern "system" fn FromArgb<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorHelperStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            a: u8,
            r: u8,
            g: u8,
            b: u8,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FromArgb(a, r, g, b) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IColorHelperStatics, OFFSET>(
            ),
            FromArgb: FromArgb::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorHelperStatics as ::windows::core::ComInterface>::IID
    }
}
pub trait IColors_Impl: Sized {}
impl ::windows::core::RuntimeName for IColors {
    const NAME: &'static str = "Microsoft.UI.IColors";
}
impl IColors_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IColors_Impl,
        const OFFSET: isize,
    >() -> IColors_Vtbl {
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IColors, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColors as ::windows::core::ComInterface>::IID
    }
}
pub trait IColorsStatics_Impl: Sized {
    fn AliceBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn AntiqueWhite(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Aqua(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Aquamarine(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Azure(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Beige(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Bisque(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Black(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn BlanchedAlmond(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Blue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn BlueViolet(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Brown(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn BurlyWood(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn CadetBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Chartreuse(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Chocolate(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Coral(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn CornflowerBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Cornsilk(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Crimson(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Cyan(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DarkBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DarkCyan(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DarkGoldenrod(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DarkGray(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DarkGreen(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DarkKhaki(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DarkMagenta(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DarkOliveGreen(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DarkOrange(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DarkOrchid(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DarkRed(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DarkSalmon(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DarkSeaGreen(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DarkSlateBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DarkSlateGray(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DarkTurquoise(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DarkViolet(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DeepPink(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DeepSkyBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DimGray(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn DodgerBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Firebrick(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn FloralWhite(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn ForestGreen(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Fuchsia(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Gainsboro(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn GhostWhite(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Gold(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Goldenrod(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Gray(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Green(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn GreenYellow(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Honeydew(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn HotPink(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn IndianRed(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Indigo(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Ivory(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Khaki(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Lavender(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn LavenderBlush(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn LawnGreen(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn LemonChiffon(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn LightBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn LightCoral(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn LightCyan(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn LightGoldenrodYellow(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn LightGreen(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn LightGray(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn LightPink(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn LightSalmon(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn LightSeaGreen(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn LightSkyBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn LightSlateGray(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn LightSteelBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn LightYellow(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Lime(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn LimeGreen(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Linen(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Magenta(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Maroon(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn MediumAquamarine(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn MediumBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn MediumOrchid(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn MediumPurple(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn MediumSeaGreen(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn MediumSlateBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn MediumSpringGreen(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn MediumTurquoise(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn MediumVioletRed(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn MidnightBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn MintCream(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn MistyRose(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Moccasin(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn NavajoWhite(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Navy(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn OldLace(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Olive(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn OliveDrab(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Orange(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn OrangeRed(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Orchid(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn PaleGoldenrod(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn PaleGreen(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn PaleTurquoise(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn PaleVioletRed(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn PapayaWhip(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn PeachPuff(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Peru(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Pink(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Plum(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn PowderBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Purple(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Red(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn RosyBrown(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn RoyalBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn SaddleBrown(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Salmon(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn SandyBrown(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn SeaGreen(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn SeaShell(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Sienna(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Silver(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn SkyBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn SlateBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn SlateGray(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Snow(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn SpringGreen(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn SteelBlue(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Tan(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Teal(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Thistle(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Tomato(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Transparent(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Turquoise(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Violet(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Wheat(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn White(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn WhiteSmoke(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn Yellow(&self) -> ::windows::core::Result<::windows::UI::Color>;
    fn YellowGreen(&self) -> ::windows::core::Result<::windows::UI::Color>;
}
impl ::windows::core::RuntimeName for IColorsStatics {
    const NAME: &'static str = "Microsoft.UI.IColorsStatics";
}
impl IColorsStatics_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IColorsStatics_Impl,
        const OFFSET: isize,
    >() -> IColorsStatics_Vtbl {
        unsafe extern "system" fn AliceBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AliceBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AntiqueWhite<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AntiqueWhite() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Aqua<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Aqua() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Aquamarine<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Aquamarine() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Azure<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Azure() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Beige<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Beige() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bisque<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Bisque() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Black<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Black() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlanchedAlmond<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BlanchedAlmond() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Blue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Blue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlueViolet<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BlueViolet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Brown<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Brown() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BurlyWood<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BurlyWood() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CadetBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CadetBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Chartreuse<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Chartreuse() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Chocolate<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Chocolate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Coral<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Coral() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CornflowerBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CornflowerBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cornsilk<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Cornsilk() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Crimson<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Crimson() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cyan<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Cyan() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DarkBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkCyan<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DarkCyan() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkGoldenrod<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DarkGoldenrod() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkGray<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DarkGray() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkGreen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DarkGreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkKhaki<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DarkKhaki() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkMagenta<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DarkMagenta() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkOliveGreen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DarkOliveGreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkOrange<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DarkOrange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkOrchid<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DarkOrchid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkRed<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DarkRed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkSalmon<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DarkSalmon() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkSeaGreen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DarkSeaGreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkSlateBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DarkSlateBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkSlateGray<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DarkSlateGray() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkTurquoise<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DarkTurquoise() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DarkViolet<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DarkViolet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeepPink<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeepPink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeepSkyBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeepSkyBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DimGray<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DimGray() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DodgerBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DodgerBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Firebrick<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Firebrick() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FloralWhite<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FloralWhite() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForestGreen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ForestGreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Fuchsia<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Fuchsia() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gainsboro<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Gainsboro() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GhostWhite<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GhostWhite() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gold<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Gold() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Goldenrod<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Goldenrod() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Gray<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Gray() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Green<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Green() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GreenYellow<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GreenYellow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Honeydew<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Honeydew() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HotPink<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HotPink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndianRed<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IndianRed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Indigo<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Indigo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ivory<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Ivory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Khaki<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Khaki() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lavender<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Lavender() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LavenderBlush<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LavenderBlush() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LawnGreen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LawnGreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LemonChiffon<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LemonChiffon() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LightBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightCoral<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LightCoral() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightCyan<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LightCyan() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightGoldenrodYellow<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LightGoldenrodYellow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightGreen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LightGreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightGray<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LightGray() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightPink<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LightPink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightSalmon<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LightSalmon() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightSeaGreen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LightSeaGreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightSkyBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LightSkyBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightSlateGray<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LightSlateGray() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightSteelBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LightSteelBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightYellow<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LightYellow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lime<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Lime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LimeGreen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LimeGreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Linen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Linen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Magenta<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Magenta() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Maroon<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Maroon() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumAquamarine<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediumAquamarine() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediumBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumOrchid<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediumOrchid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumPurple<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediumPurple() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumSeaGreen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediumSeaGreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumSlateBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediumSlateBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumSpringGreen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediumSpringGreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumTurquoise<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediumTurquoise() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediumVioletRed<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediumVioletRed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MidnightBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MidnightBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MintCream<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MintCream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MistyRose<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MistyRose() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Moccasin<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Moccasin() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NavajoWhite<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NavajoWhite() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Navy<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Navy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldLace<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OldLace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Olive<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Olive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OliveDrab<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OliveDrab() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Orange<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Orange() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OrangeRed<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OrangeRed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Orchid<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Orchid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PaleGoldenrod<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PaleGoldenrod() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PaleGreen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PaleGreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PaleTurquoise<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PaleTurquoise() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PaleVioletRed<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PaleVioletRed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PapayaWhip<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PapayaWhip() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeachPuff<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PeachPuff() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peru<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Peru() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Pink<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Pink() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Plum<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Plum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PowderBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PowderBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Purple<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Purple() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Red<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Red() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RosyBrown<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RosyBrown() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoyalBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoyalBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaddleBrown<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SaddleBrown() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Salmon<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Salmon() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SandyBrown<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SandyBrown() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeaGreen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SeaGreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SeaShell<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SeaShell() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sienna<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Sienna() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Silver<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Silver() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SkyBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SkyBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SlateBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SlateBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SlateGray<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SlateGray() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Snow<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Snow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpringGreen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SpringGreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SteelBlue<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SteelBlue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tan<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Tan() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Teal<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Teal() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Thistle<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Thistle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tomato<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Tomato() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transparent<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Transparent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Turquoise<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Turquoise() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Violet<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Violet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Wheat<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Wheat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn White<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.White() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WhiteSmoke<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WhiteSmoke() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Yellow<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Yellow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn YellowGreen<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IColorsStatics_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::UI::Color,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.YellowGreen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IColorsStatics, OFFSET>(),
            AliceBlue: AliceBlue::<Identity, Impl, OFFSET>,
            AntiqueWhite: AntiqueWhite::<Identity, Impl, OFFSET>,
            Aqua: Aqua::<Identity, Impl, OFFSET>,
            Aquamarine: Aquamarine::<Identity, Impl, OFFSET>,
            Azure: Azure::<Identity, Impl, OFFSET>,
            Beige: Beige::<Identity, Impl, OFFSET>,
            Bisque: Bisque::<Identity, Impl, OFFSET>,
            Black: Black::<Identity, Impl, OFFSET>,
            BlanchedAlmond: BlanchedAlmond::<Identity, Impl, OFFSET>,
            Blue: Blue::<Identity, Impl, OFFSET>,
            BlueViolet: BlueViolet::<Identity, Impl, OFFSET>,
            Brown: Brown::<Identity, Impl, OFFSET>,
            BurlyWood: BurlyWood::<Identity, Impl, OFFSET>,
            CadetBlue: CadetBlue::<Identity, Impl, OFFSET>,
            Chartreuse: Chartreuse::<Identity, Impl, OFFSET>,
            Chocolate: Chocolate::<Identity, Impl, OFFSET>,
            Coral: Coral::<Identity, Impl, OFFSET>,
            CornflowerBlue: CornflowerBlue::<Identity, Impl, OFFSET>,
            Cornsilk: Cornsilk::<Identity, Impl, OFFSET>,
            Crimson: Crimson::<Identity, Impl, OFFSET>,
            Cyan: Cyan::<Identity, Impl, OFFSET>,
            DarkBlue: DarkBlue::<Identity, Impl, OFFSET>,
            DarkCyan: DarkCyan::<Identity, Impl, OFFSET>,
            DarkGoldenrod: DarkGoldenrod::<Identity, Impl, OFFSET>,
            DarkGray: DarkGray::<Identity, Impl, OFFSET>,
            DarkGreen: DarkGreen::<Identity, Impl, OFFSET>,
            DarkKhaki: DarkKhaki::<Identity, Impl, OFFSET>,
            DarkMagenta: DarkMagenta::<Identity, Impl, OFFSET>,
            DarkOliveGreen: DarkOliveGreen::<Identity, Impl, OFFSET>,
            DarkOrange: DarkOrange::<Identity, Impl, OFFSET>,
            DarkOrchid: DarkOrchid::<Identity, Impl, OFFSET>,
            DarkRed: DarkRed::<Identity, Impl, OFFSET>,
            DarkSalmon: DarkSalmon::<Identity, Impl, OFFSET>,
            DarkSeaGreen: DarkSeaGreen::<Identity, Impl, OFFSET>,
            DarkSlateBlue: DarkSlateBlue::<Identity, Impl, OFFSET>,
            DarkSlateGray: DarkSlateGray::<Identity, Impl, OFFSET>,
            DarkTurquoise: DarkTurquoise::<Identity, Impl, OFFSET>,
            DarkViolet: DarkViolet::<Identity, Impl, OFFSET>,
            DeepPink: DeepPink::<Identity, Impl, OFFSET>,
            DeepSkyBlue: DeepSkyBlue::<Identity, Impl, OFFSET>,
            DimGray: DimGray::<Identity, Impl, OFFSET>,
            DodgerBlue: DodgerBlue::<Identity, Impl, OFFSET>,
            Firebrick: Firebrick::<Identity, Impl, OFFSET>,
            FloralWhite: FloralWhite::<Identity, Impl, OFFSET>,
            ForestGreen: ForestGreen::<Identity, Impl, OFFSET>,
            Fuchsia: Fuchsia::<Identity, Impl, OFFSET>,
            Gainsboro: Gainsboro::<Identity, Impl, OFFSET>,
            GhostWhite: GhostWhite::<Identity, Impl, OFFSET>,
            Gold: Gold::<Identity, Impl, OFFSET>,
            Goldenrod: Goldenrod::<Identity, Impl, OFFSET>,
            Gray: Gray::<Identity, Impl, OFFSET>,
            Green: Green::<Identity, Impl, OFFSET>,
            GreenYellow: GreenYellow::<Identity, Impl, OFFSET>,
            Honeydew: Honeydew::<Identity, Impl, OFFSET>,
            HotPink: HotPink::<Identity, Impl, OFFSET>,
            IndianRed: IndianRed::<Identity, Impl, OFFSET>,
            Indigo: Indigo::<Identity, Impl, OFFSET>,
            Ivory: Ivory::<Identity, Impl, OFFSET>,
            Khaki: Khaki::<Identity, Impl, OFFSET>,
            Lavender: Lavender::<Identity, Impl, OFFSET>,
            LavenderBlush: LavenderBlush::<Identity, Impl, OFFSET>,
            LawnGreen: LawnGreen::<Identity, Impl, OFFSET>,
            LemonChiffon: LemonChiffon::<Identity, Impl, OFFSET>,
            LightBlue: LightBlue::<Identity, Impl, OFFSET>,
            LightCoral: LightCoral::<Identity, Impl, OFFSET>,
            LightCyan: LightCyan::<Identity, Impl, OFFSET>,
            LightGoldenrodYellow: LightGoldenrodYellow::<Identity, Impl, OFFSET>,
            LightGreen: LightGreen::<Identity, Impl, OFFSET>,
            LightGray: LightGray::<Identity, Impl, OFFSET>,
            LightPink: LightPink::<Identity, Impl, OFFSET>,
            LightSalmon: LightSalmon::<Identity, Impl, OFFSET>,
            LightSeaGreen: LightSeaGreen::<Identity, Impl, OFFSET>,
            LightSkyBlue: LightSkyBlue::<Identity, Impl, OFFSET>,
            LightSlateGray: LightSlateGray::<Identity, Impl, OFFSET>,
            LightSteelBlue: LightSteelBlue::<Identity, Impl, OFFSET>,
            LightYellow: LightYellow::<Identity, Impl, OFFSET>,
            Lime: Lime::<Identity, Impl, OFFSET>,
            LimeGreen: LimeGreen::<Identity, Impl, OFFSET>,
            Linen: Linen::<Identity, Impl, OFFSET>,
            Magenta: Magenta::<Identity, Impl, OFFSET>,
            Maroon: Maroon::<Identity, Impl, OFFSET>,
            MediumAquamarine: MediumAquamarine::<Identity, Impl, OFFSET>,
            MediumBlue: MediumBlue::<Identity, Impl, OFFSET>,
            MediumOrchid: MediumOrchid::<Identity, Impl, OFFSET>,
            MediumPurple: MediumPurple::<Identity, Impl, OFFSET>,
            MediumSeaGreen: MediumSeaGreen::<Identity, Impl, OFFSET>,
            MediumSlateBlue: MediumSlateBlue::<Identity, Impl, OFFSET>,
            MediumSpringGreen: MediumSpringGreen::<Identity, Impl, OFFSET>,
            MediumTurquoise: MediumTurquoise::<Identity, Impl, OFFSET>,
            MediumVioletRed: MediumVioletRed::<Identity, Impl, OFFSET>,
            MidnightBlue: MidnightBlue::<Identity, Impl, OFFSET>,
            MintCream: MintCream::<Identity, Impl, OFFSET>,
            MistyRose: MistyRose::<Identity, Impl, OFFSET>,
            Moccasin: Moccasin::<Identity, Impl, OFFSET>,
            NavajoWhite: NavajoWhite::<Identity, Impl, OFFSET>,
            Navy: Navy::<Identity, Impl, OFFSET>,
            OldLace: OldLace::<Identity, Impl, OFFSET>,
            Olive: Olive::<Identity, Impl, OFFSET>,
            OliveDrab: OliveDrab::<Identity, Impl, OFFSET>,
            Orange: Orange::<Identity, Impl, OFFSET>,
            OrangeRed: OrangeRed::<Identity, Impl, OFFSET>,
            Orchid: Orchid::<Identity, Impl, OFFSET>,
            PaleGoldenrod: PaleGoldenrod::<Identity, Impl, OFFSET>,
            PaleGreen: PaleGreen::<Identity, Impl, OFFSET>,
            PaleTurquoise: PaleTurquoise::<Identity, Impl, OFFSET>,
            PaleVioletRed: PaleVioletRed::<Identity, Impl, OFFSET>,
            PapayaWhip: PapayaWhip::<Identity, Impl, OFFSET>,
            PeachPuff: PeachPuff::<Identity, Impl, OFFSET>,
            Peru: Peru::<Identity, Impl, OFFSET>,
            Pink: Pink::<Identity, Impl, OFFSET>,
            Plum: Plum::<Identity, Impl, OFFSET>,
            PowderBlue: PowderBlue::<Identity, Impl, OFFSET>,
            Purple: Purple::<Identity, Impl, OFFSET>,
            Red: Red::<Identity, Impl, OFFSET>,
            RosyBrown: RosyBrown::<Identity, Impl, OFFSET>,
            RoyalBlue: RoyalBlue::<Identity, Impl, OFFSET>,
            SaddleBrown: SaddleBrown::<Identity, Impl, OFFSET>,
            Salmon: Salmon::<Identity, Impl, OFFSET>,
            SandyBrown: SandyBrown::<Identity, Impl, OFFSET>,
            SeaGreen: SeaGreen::<Identity, Impl, OFFSET>,
            SeaShell: SeaShell::<Identity, Impl, OFFSET>,
            Sienna: Sienna::<Identity, Impl, OFFSET>,
            Silver: Silver::<Identity, Impl, OFFSET>,
            SkyBlue: SkyBlue::<Identity, Impl, OFFSET>,
            SlateBlue: SlateBlue::<Identity, Impl, OFFSET>,
            SlateGray: SlateGray::<Identity, Impl, OFFSET>,
            Snow: Snow::<Identity, Impl, OFFSET>,
            SpringGreen: SpringGreen::<Identity, Impl, OFFSET>,
            SteelBlue: SteelBlue::<Identity, Impl, OFFSET>,
            Tan: Tan::<Identity, Impl, OFFSET>,
            Teal: Teal::<Identity, Impl, OFFSET>,
            Thistle: Thistle::<Identity, Impl, OFFSET>,
            Tomato: Tomato::<Identity, Impl, OFFSET>,
            Transparent: Transparent::<Identity, Impl, OFFSET>,
            Turquoise: Turquoise::<Identity, Impl, OFFSET>,
            Violet: Violet::<Identity, Impl, OFFSET>,
            Wheat: Wheat::<Identity, Impl, OFFSET>,
            White: White::<Identity, Impl, OFFSET>,
            WhiteSmoke: WhiteSmoke::<Identity, Impl, OFFSET>,
            Yellow: Yellow::<Identity, Impl, OFFSET>,
            YellowGreen: YellowGreen::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorsStatics as ::windows::core::ComInterface>::IID
    }
}
