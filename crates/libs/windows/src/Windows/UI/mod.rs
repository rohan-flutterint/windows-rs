#[cfg(feature = "UI_Accessibility")]
#[doc = "Required features: `\"UI_Accessibility\"`"]
pub mod Accessibility;
#[cfg(feature = "UI_ApplicationSettings")]
#[doc = "Required features: `\"UI_ApplicationSettings\"`"]
pub mod ApplicationSettings;
#[cfg(feature = "UI_Composition")]
#[doc = "Required features: `\"UI_Composition\"`"]
pub mod Composition;
#[cfg(feature = "UI_Core")]
#[doc = "Required features: `\"UI_Core\"`"]
pub mod Core;
#[cfg(feature = "UI_Input")]
#[doc = "Required features: `\"UI_Input\"`"]
pub mod Input;
#[cfg(feature = "UI_Notifications")]
#[doc = "Required features: `\"UI_Notifications\"`"]
pub mod Notifications;
#[cfg(feature = "UI_Popups")]
#[doc = "Required features: `\"UI_Popups\"`"]
pub mod Popups;
#[cfg(feature = "UI_Shell")]
#[doc = "Required features: `\"UI_Shell\"`"]
pub mod Shell;
#[cfg(feature = "UI_StartScreen")]
#[doc = "Required features: `\"UI_StartScreen\"`"]
pub mod StartScreen;
#[cfg(feature = "UI_Text")]
#[doc = "Required features: `\"UI_Text\"`"]
pub mod Text;
#[cfg(feature = "UI_UIAutomation")]
#[doc = "Required features: `\"UI_UIAutomation\"`"]
pub mod UIAutomation;
#[cfg(feature = "UI_ViewManagement")]
#[doc = "Required features: `\"UI_ViewManagement\"`"]
pub mod ViewManagement;
#[cfg(feature = "UI_WebUI")]
#[doc = "Required features: `\"UI_WebUI\"`"]
pub mod WebUI;
#[cfg(feature = "UI_WindowManagement")]
#[doc = "Required features: `\"UI_WindowManagement\"`"]
pub mod WindowManagement;
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IColorHelper(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorHelper {
    type Vtable = IColorHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IColorHelper {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x193cfbe7_65c7_4540_ad08_6283ba76879a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelper_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IColorHelperStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorHelperStatics {
    type Vtable = IColorHelperStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IColorHelperStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8504dbea_fb6a_4144_a6c2_33499c9284f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelperStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FromArgb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, a: u8, r: u8, g: u8, b: u8, result__: *mut Color) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IColorHelperStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorHelperStatics2 {
    type Vtable = IColorHelperStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IColorHelperStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24d9af02_6eb0_4b94_855c_fcf0818d9a16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorHelperStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ToDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: Color, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IColors(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColors {
    type Vtable = IColors_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IColors {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b8c9326_4ca6_4ce5_8994_9eff65cabdcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColors_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IColorsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IColorsStatics {
    type Vtable = IColorsStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IColorsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcff52e04_cca6_4614_a17e_754910c84a99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColorsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AliceBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub AntiqueWhite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Aqua: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Aquamarine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Azure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Beige: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Bisque: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Black: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub BlanchedAlmond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Blue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub BlueViolet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Brown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub BurlyWood: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub CadetBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Chartreuse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Chocolate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Coral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub CornflowerBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Cornsilk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Crimson: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Cyan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkCyan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkGoldenrod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkKhaki: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkMagenta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkOliveGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkOrange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkOrchid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkSalmon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkSeaGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkSlateBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkSlateGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkTurquoise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DarkViolet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DeepPink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DeepSkyBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DimGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub DodgerBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Firebrick: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub FloralWhite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub ForestGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Fuchsia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Gainsboro: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub GhostWhite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Gold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Goldenrod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Gray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Green: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub GreenYellow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Honeydew: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub HotPink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub IndianRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Indigo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Ivory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Khaki: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Lavender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LavenderBlush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LawnGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LemonChiffon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightCoral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightCyan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightGoldenrodYellow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightPink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightSalmon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightSeaGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightSkyBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightSlateGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightSteelBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LightYellow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Lime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub LimeGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Linen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Magenta: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Maroon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumAquamarine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumOrchid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumPurple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumSeaGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumSlateBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumSpringGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumTurquoise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MediumVioletRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MidnightBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MintCream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub MistyRose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Moccasin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub NavajoWhite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Navy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub OldLace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Olive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub OliveDrab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Orange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub OrangeRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Orchid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub PaleGoldenrod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub PaleGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub PaleTurquoise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub PaleVioletRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub PapayaWhip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub PeachPuff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Peru: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Pink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Plum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub PowderBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Purple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Red: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub RosyBrown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub RoyalBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SaddleBrown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Salmon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SandyBrown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SeaGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SeaShell: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Sienna: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Silver: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SkyBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SlateBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SlateGray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Snow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SpringGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub SteelBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Tan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Teal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Thistle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Tomato: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Transparent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Turquoise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Violet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Wheat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub White: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub WhiteSmoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub Yellow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
    pub YellowGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Color) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIContentRoot(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIContentRoot {
    type Vtable = IUIContentRoot_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIContentRoot {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1dfcbac6_b36b_5cb9_9bc5_2b7a0eddc378);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIContentRoot_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub UIContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IUIContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IUIContext {
    type Vtable = IUIContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IUIContext {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb5cfacd_5bd8_59d0_a59e_1c17a4d6d243);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIContext_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ColorHelper(::windows_core::IUnknown);
impl ColorHelper {
    pub fn FromArgb(a: u8, r: u8, g: u8, b: u8) -> ::windows_core::Result<Color> {
        Self::IColorHelperStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FromArgb(a, r, g, b, &mut result__)).from_abi(result__)
        })
    }
    pub fn ToDisplayName(color: Color) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IColorHelperStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ToDisplayName(color, &mut result__)).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IColorHelperStatics<R, F: FnOnce(&IColorHelperStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ColorHelper, IColorHelperStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IColorHelperStatics2<R, F: FnOnce(&IColorHelperStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ColorHelper, IColorHelperStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ColorHelper {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.ColorHelper;{193cfbe7-65c7-4540-ad08-6283ba76879a})");
}
unsafe impl ::windows_core::Interface for ColorHelper {
    type Vtable = IColorHelper_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ColorHelper {
    const IID: ::windows_core::GUID = <IColorHelper as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ColorHelper {
    const NAME: &'static str = "Windows.UI.ColorHelper";
}
::windows_core::imp::interface_hierarchy!(ColorHelper, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ColorHelper {}
unsafe impl ::core::marker::Sync for ColorHelper {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Colors(::windows_core::IUnknown);
impl Colors {
    pub fn AliceBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.AliceBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn AntiqueWhite() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.AntiqueWhite(&mut result__)).from_abi(result__)
        })
    }
    pub fn Aqua() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Aqua(&mut result__)).from_abi(result__)
        })
    }
    pub fn Aquamarine() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Aquamarine(&mut result__)).from_abi(result__)
        })
    }
    pub fn Azure() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Azure(&mut result__)).from_abi(result__)
        })
    }
    pub fn Beige() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Beige(&mut result__)).from_abi(result__)
        })
    }
    pub fn Bisque() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Bisque(&mut result__)).from_abi(result__)
        })
    }
    pub fn Black() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Black(&mut result__)).from_abi(result__)
        })
    }
    pub fn BlanchedAlmond() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BlanchedAlmond(&mut result__)).from_abi(result__)
        })
    }
    pub fn Blue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Blue(&mut result__)).from_abi(result__)
        })
    }
    pub fn BlueViolet() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BlueViolet(&mut result__)).from_abi(result__)
        })
    }
    pub fn Brown() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Brown(&mut result__)).from_abi(result__)
        })
    }
    pub fn BurlyWood() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BurlyWood(&mut result__)).from_abi(result__)
        })
    }
    pub fn CadetBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CadetBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn Chartreuse() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Chartreuse(&mut result__)).from_abi(result__)
        })
    }
    pub fn Chocolate() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Chocolate(&mut result__)).from_abi(result__)
        })
    }
    pub fn Coral() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Coral(&mut result__)).from_abi(result__)
        })
    }
    pub fn CornflowerBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CornflowerBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn Cornsilk() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Cornsilk(&mut result__)).from_abi(result__)
        })
    }
    pub fn Crimson() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Crimson(&mut result__)).from_abi(result__)
        })
    }
    pub fn Cyan() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Cyan(&mut result__)).from_abi(result__)
        })
    }
    pub fn DarkBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DarkBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn DarkCyan() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DarkCyan(&mut result__)).from_abi(result__)
        })
    }
    pub fn DarkGoldenrod() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DarkGoldenrod(&mut result__)).from_abi(result__)
        })
    }
    pub fn DarkGray() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DarkGray(&mut result__)).from_abi(result__)
        })
    }
    pub fn DarkGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DarkGreen(&mut result__)).from_abi(result__)
        })
    }
    pub fn DarkKhaki() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DarkKhaki(&mut result__)).from_abi(result__)
        })
    }
    pub fn DarkMagenta() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DarkMagenta(&mut result__)).from_abi(result__)
        })
    }
    pub fn DarkOliveGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DarkOliveGreen(&mut result__)).from_abi(result__)
        })
    }
    pub fn DarkOrange() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DarkOrange(&mut result__)).from_abi(result__)
        })
    }
    pub fn DarkOrchid() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DarkOrchid(&mut result__)).from_abi(result__)
        })
    }
    pub fn DarkRed() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DarkRed(&mut result__)).from_abi(result__)
        })
    }
    pub fn DarkSalmon() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DarkSalmon(&mut result__)).from_abi(result__)
        })
    }
    pub fn DarkSeaGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DarkSeaGreen(&mut result__)).from_abi(result__)
        })
    }
    pub fn DarkSlateBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DarkSlateBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn DarkSlateGray() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DarkSlateGray(&mut result__)).from_abi(result__)
        })
    }
    pub fn DarkTurquoise() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DarkTurquoise(&mut result__)).from_abi(result__)
        })
    }
    pub fn DarkViolet() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DarkViolet(&mut result__)).from_abi(result__)
        })
    }
    pub fn DeepPink() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DeepPink(&mut result__)).from_abi(result__)
        })
    }
    pub fn DeepSkyBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DeepSkyBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn DimGray() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DimGray(&mut result__)).from_abi(result__)
        })
    }
    pub fn DodgerBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DodgerBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn Firebrick() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Firebrick(&mut result__)).from_abi(result__)
        })
    }
    pub fn FloralWhite() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FloralWhite(&mut result__)).from_abi(result__)
        })
    }
    pub fn ForestGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ForestGreen(&mut result__)).from_abi(result__)
        })
    }
    pub fn Fuchsia() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Fuchsia(&mut result__)).from_abi(result__)
        })
    }
    pub fn Gainsboro() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Gainsboro(&mut result__)).from_abi(result__)
        })
    }
    pub fn GhostWhite() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GhostWhite(&mut result__)).from_abi(result__)
        })
    }
    pub fn Gold() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Gold(&mut result__)).from_abi(result__)
        })
    }
    pub fn Goldenrod() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Goldenrod(&mut result__)).from_abi(result__)
        })
    }
    pub fn Gray() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Gray(&mut result__)).from_abi(result__)
        })
    }
    pub fn Green() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Green(&mut result__)).from_abi(result__)
        })
    }
    pub fn GreenYellow() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GreenYellow(&mut result__)).from_abi(result__)
        })
    }
    pub fn Honeydew() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Honeydew(&mut result__)).from_abi(result__)
        })
    }
    pub fn HotPink() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.HotPink(&mut result__)).from_abi(result__)
        })
    }
    pub fn IndianRed() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IndianRed(&mut result__)).from_abi(result__)
        })
    }
    pub fn Indigo() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Indigo(&mut result__)).from_abi(result__)
        })
    }
    pub fn Ivory() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Ivory(&mut result__)).from_abi(result__)
        })
    }
    pub fn Khaki() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Khaki(&mut result__)).from_abi(result__)
        })
    }
    pub fn Lavender() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Lavender(&mut result__)).from_abi(result__)
        })
    }
    pub fn LavenderBlush() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LavenderBlush(&mut result__)).from_abi(result__)
        })
    }
    pub fn LawnGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LawnGreen(&mut result__)).from_abi(result__)
        })
    }
    pub fn LemonChiffon() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LemonChiffon(&mut result__)).from_abi(result__)
        })
    }
    pub fn LightBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LightBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn LightCoral() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LightCoral(&mut result__)).from_abi(result__)
        })
    }
    pub fn LightCyan() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LightCyan(&mut result__)).from_abi(result__)
        })
    }
    pub fn LightGoldenrodYellow() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LightGoldenrodYellow(&mut result__)).from_abi(result__)
        })
    }
    pub fn LightGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LightGreen(&mut result__)).from_abi(result__)
        })
    }
    pub fn LightGray() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LightGray(&mut result__)).from_abi(result__)
        })
    }
    pub fn LightPink() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LightPink(&mut result__)).from_abi(result__)
        })
    }
    pub fn LightSalmon() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LightSalmon(&mut result__)).from_abi(result__)
        })
    }
    pub fn LightSeaGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LightSeaGreen(&mut result__)).from_abi(result__)
        })
    }
    pub fn LightSkyBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LightSkyBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn LightSlateGray() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LightSlateGray(&mut result__)).from_abi(result__)
        })
    }
    pub fn LightSteelBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LightSteelBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn LightYellow() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LightYellow(&mut result__)).from_abi(result__)
        })
    }
    pub fn Lime() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Lime(&mut result__)).from_abi(result__)
        })
    }
    pub fn LimeGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LimeGreen(&mut result__)).from_abi(result__)
        })
    }
    pub fn Linen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Linen(&mut result__)).from_abi(result__)
        })
    }
    pub fn Magenta() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Magenta(&mut result__)).from_abi(result__)
        })
    }
    pub fn Maroon() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Maroon(&mut result__)).from_abi(result__)
        })
    }
    pub fn MediumAquamarine() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MediumAquamarine(&mut result__)).from_abi(result__)
        })
    }
    pub fn MediumBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MediumBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn MediumOrchid() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MediumOrchid(&mut result__)).from_abi(result__)
        })
    }
    pub fn MediumPurple() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MediumPurple(&mut result__)).from_abi(result__)
        })
    }
    pub fn MediumSeaGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MediumSeaGreen(&mut result__)).from_abi(result__)
        })
    }
    pub fn MediumSlateBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MediumSlateBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn MediumSpringGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MediumSpringGreen(&mut result__)).from_abi(result__)
        })
    }
    pub fn MediumTurquoise() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MediumTurquoise(&mut result__)).from_abi(result__)
        })
    }
    pub fn MediumVioletRed() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MediumVioletRed(&mut result__)).from_abi(result__)
        })
    }
    pub fn MidnightBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MidnightBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn MintCream() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MintCream(&mut result__)).from_abi(result__)
        })
    }
    pub fn MistyRose() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MistyRose(&mut result__)).from_abi(result__)
        })
    }
    pub fn Moccasin() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Moccasin(&mut result__)).from_abi(result__)
        })
    }
    pub fn NavajoWhite() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.NavajoWhite(&mut result__)).from_abi(result__)
        })
    }
    pub fn Navy() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Navy(&mut result__)).from_abi(result__)
        })
    }
    pub fn OldLace() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OldLace(&mut result__)).from_abi(result__)
        })
    }
    pub fn Olive() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Olive(&mut result__)).from_abi(result__)
        })
    }
    pub fn OliveDrab() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OliveDrab(&mut result__)).from_abi(result__)
        })
    }
    pub fn Orange() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Orange(&mut result__)).from_abi(result__)
        })
    }
    pub fn OrangeRed() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OrangeRed(&mut result__)).from_abi(result__)
        })
    }
    pub fn Orchid() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Orchid(&mut result__)).from_abi(result__)
        })
    }
    pub fn PaleGoldenrod() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.PaleGoldenrod(&mut result__)).from_abi(result__)
        })
    }
    pub fn PaleGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.PaleGreen(&mut result__)).from_abi(result__)
        })
    }
    pub fn PaleTurquoise() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.PaleTurquoise(&mut result__)).from_abi(result__)
        })
    }
    pub fn PaleVioletRed() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.PaleVioletRed(&mut result__)).from_abi(result__)
        })
    }
    pub fn PapayaWhip() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.PapayaWhip(&mut result__)).from_abi(result__)
        })
    }
    pub fn PeachPuff() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.PeachPuff(&mut result__)).from_abi(result__)
        })
    }
    pub fn Peru() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Peru(&mut result__)).from_abi(result__)
        })
    }
    pub fn Pink() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Pink(&mut result__)).from_abi(result__)
        })
    }
    pub fn Plum() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Plum(&mut result__)).from_abi(result__)
        })
    }
    pub fn PowderBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.PowderBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn Purple() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Purple(&mut result__)).from_abi(result__)
        })
    }
    pub fn Red() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Red(&mut result__)).from_abi(result__)
        })
    }
    pub fn RosyBrown() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.RosyBrown(&mut result__)).from_abi(result__)
        })
    }
    pub fn RoyalBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.RoyalBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn SaddleBrown() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SaddleBrown(&mut result__)).from_abi(result__)
        })
    }
    pub fn Salmon() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Salmon(&mut result__)).from_abi(result__)
        })
    }
    pub fn SandyBrown() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SandyBrown(&mut result__)).from_abi(result__)
        })
    }
    pub fn SeaGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SeaGreen(&mut result__)).from_abi(result__)
        })
    }
    pub fn SeaShell() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SeaShell(&mut result__)).from_abi(result__)
        })
    }
    pub fn Sienna() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Sienna(&mut result__)).from_abi(result__)
        })
    }
    pub fn Silver() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Silver(&mut result__)).from_abi(result__)
        })
    }
    pub fn SkyBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SkyBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn SlateBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SlateBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn SlateGray() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SlateGray(&mut result__)).from_abi(result__)
        })
    }
    pub fn Snow() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Snow(&mut result__)).from_abi(result__)
        })
    }
    pub fn SpringGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SpringGreen(&mut result__)).from_abi(result__)
        })
    }
    pub fn SteelBlue() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SteelBlue(&mut result__)).from_abi(result__)
        })
    }
    pub fn Tan() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Tan(&mut result__)).from_abi(result__)
        })
    }
    pub fn Teal() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Teal(&mut result__)).from_abi(result__)
        })
    }
    pub fn Thistle() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Thistle(&mut result__)).from_abi(result__)
        })
    }
    pub fn Tomato() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Tomato(&mut result__)).from_abi(result__)
        })
    }
    pub fn Transparent() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Transparent(&mut result__)).from_abi(result__)
        })
    }
    pub fn Turquoise() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Turquoise(&mut result__)).from_abi(result__)
        })
    }
    pub fn Violet() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Violet(&mut result__)).from_abi(result__)
        })
    }
    pub fn Wheat() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Wheat(&mut result__)).from_abi(result__)
        })
    }
    pub fn White() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.White(&mut result__)).from_abi(result__)
        })
    }
    pub fn WhiteSmoke() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.WhiteSmoke(&mut result__)).from_abi(result__)
        })
    }
    pub fn Yellow() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Yellow(&mut result__)).from_abi(result__)
        })
    }
    pub fn YellowGreen() -> ::windows_core::Result<Color> {
        Self::IColorsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.YellowGreen(&mut result__)).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IColorsStatics<R, F: FnOnce(&IColorsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Colors, IColorsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for Colors {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Colors;{9b8c9326-4ca6-4ce5-8994-9eff65cabdcc})");
}
unsafe impl ::windows_core::Interface for Colors {
    type Vtable = IColors_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Colors {
    const IID: ::windows_core::GUID = <IColors as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Colors {
    const NAME: &'static str = "Windows.UI.Colors";
}
::windows_core::imp::interface_hierarchy!(Colors, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for Colors {}
unsafe impl ::core::marker::Sync for Colors {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UIContentRoot(::windows_core::IUnknown);
impl UIContentRoot {
    pub fn UIContext(&self) -> ::windows_core::Result<UIContext> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.UIContext(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for UIContentRoot {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.UIContentRoot;{1dfcbac6-b36b-5cb9-9bc5-2b7a0eddc378})");
}
unsafe impl ::windows_core::Interface for UIContentRoot {
    type Vtable = IUIContentRoot_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UIContentRoot {
    const IID: ::windows_core::GUID = <IUIContentRoot as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UIContentRoot {
    const NAME: &'static str = "Windows.UI.UIContentRoot";
}
::windows_core::imp::interface_hierarchy!(UIContentRoot, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for UIContentRoot {}
unsafe impl ::core::marker::Sync for UIContentRoot {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UIContext(::windows_core::IUnknown);
impl UIContext {}
impl ::windows_core::RuntimeType for UIContext {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.UIContext;{bb5cfacd-5bd8-59d0-a59e-1c17a4d6d243})");
}
unsafe impl ::windows_core::Interface for UIContext {
    type Vtable = IUIContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for UIContext {
    const IID: ::windows_core::GUID = <IUIContext as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for UIContext {
    const NAME: &'static str = "Windows.UI.UIContext";
}
::windows_core::imp::interface_hierarchy!(UIContext, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for UIContext {}
unsafe impl ::core::marker::Sync for UIContext {}
#[repr(C)]
pub struct Color {
    pub A: u8,
    pub R: u8,
    pub G: u8,
    pub B: u8,
}
impl ::core::marker::Copy for Color {}
impl ::core::clone::Clone for Color {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Color {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Color").field("A", &self.A).field("R", &self.R).field("G", &self.G).field("B", &self.B).finish()
    }
}
impl ::windows_core::TypeKind for Color {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for Color {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Color;u1;u1;u1;u1)");
}
impl ::core::cmp::PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.A == other.A && self.R == other.R && self.G == other.G && self.B == other.B
    }
}
impl ::core::cmp::Eq for Color {}
impl ::core::default::Default for Color {
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
        f.debug_struct("WindowId").field("Value", &self.Value).finish()
    }
}
impl ::windows_core::TypeKind for WindowId {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for WindowId {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.WindowId;u8)");
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
