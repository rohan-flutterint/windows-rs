#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffect(::windows_core::IUnknown);
impl IMILBitmapEffect {
    #[doc = "Required features: `\"Win32_Graphics_Imaging\"`"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutput<P0>(&self, uiindex: u32, pcontext: P0) -> ::windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectRenderContext>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetOutput(uiindex, pcontext.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetParentEffect(&self) -> ::windows_core::Result<IMILBitmapEffectGroup> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetParentEffect(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Imaging\"`"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn SetInputSource<P0>(&self, uiindex: u32, pbitmapsource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Graphics::Imaging::IWICBitmapSource>,
    {
        ::windows_core::vcall!(self.SetInputSource(uiindex, pbitmapsource.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffect, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMILBitmapEffect {
    type Vtable = IMILBitmapEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a6ff321_c944_4a1b_9944_9954af301258);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffect_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: *mut ::core::ffi::c_void, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetOutput: usize,
    pub GetParentEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppparenteffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub SetInputSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pbitmapsource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    SetInputSource: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectConnections(::windows_core::IUnknown);
impl IMILBitmapEffectConnections {
    pub unsafe fn GetInputConnector(&self, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectInputConnector> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetInputConnector(uiindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetOutputConnector(&self, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetOutputConnector(uiindex, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectConnections, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMILBitmapEffectConnections {
    type Vtable = IMILBitmapEffectConnections_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectConnections {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2b5d861_9b1a_4374_89b0_dec4874d6a81);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnections_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetInputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetOutputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectConnectionsInfo(::windows_core::IUnknown);
impl IMILBitmapEffectConnectionsInfo {
    pub unsafe fn GetNumberInputs(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetNumberInputs(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetNumberOutputs(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetNumberOutputs(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetInputConnectorInfo(&self, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectConnectorInfo> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetInputConnectorInfo(uiindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetOutputConnectorInfo(&self, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectConnectorInfo> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetOutputConnectorInfo(uiindex, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectConnectionsInfo, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMILBitmapEffectConnectionsInfo {
    type Vtable = IMILBitmapEffectConnectionsInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectConnectionsInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x476b538a_c765_4237_ba4a_d6a880ff0cfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnectionsInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetNumberInputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puinuminputs: *mut u32) -> ::windows_core::HRESULT,
    pub GetNumberOutputs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puinumoutputs: *mut u32) -> ::windows_core::HRESULT,
    pub GetInputConnectorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetOutputConnectorInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnectorinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectConnector(::windows_core::IUnknown);
impl IMILBitmapEffectConnector {
    pub unsafe fn GetIndex(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetIndex(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetOptimalFormat(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetNumberFormats(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFormat(ulindex, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnected(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.IsConnected(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetBitmapEffect(&self) -> ::windows_core::Result<IMILBitmapEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetBitmapEffect(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectConnector, ::windows_core::IUnknown, IMILBitmapEffectConnectorInfo);
unsafe impl ::windows_core::Interface for IMILBitmapEffectConnector {
    type Vtable = IMILBitmapEffectConnector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectConnector {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf59567b3_76c1_4d47_ba1e_79f955e350ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnector_Vtbl {
    pub base__: IMILBitmapEffectConnectorInfo_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsConnected: usize,
    pub GetBitmapEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectConnectorInfo(::windows_core::IUnknown);
impl IMILBitmapEffectConnectorInfo {
    pub unsafe fn GetIndex(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetIndex(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetOptimalFormat(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetNumberFormats(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFormat(ulindex, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectConnectorInfo, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMILBitmapEffectConnectorInfo {
    type Vtable = IMILBitmapEffectConnectorInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectConnectorInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf66d2e4b_b46b_42fc_859e_3da0ecdb3c43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnectorInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiindex: *mut u32) -> ::windows_core::HRESULT,
    pub GetOptimalFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetNumberFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulnumberformats: *mut u32) -> ::windows_core::HRESULT,
    pub GetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulindex: u32, pformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectEvents(::windows_core::IUnknown);
impl IMILBitmapEffectEvents {
    pub unsafe fn PropertyChange<P0, P1>(&self, peffect: P0, bstrpropertyname: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffect>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        ::windows_core::vcall!(self.PropertyChange(peffect.into_param().abi(), bstrpropertyname.into_param().abi())).ok()
    }
    pub unsafe fn DirtyRegion<P0>(&self, peffect: P0, prect: *const MilRectD) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffect>,
    {
        ::windows_core::vcall!(self.DirtyRegion(peffect.into_param().abi(), prect)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectEvents, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMILBitmapEffectEvents {
    type Vtable = IMILBitmapEffectEvents_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectEvents {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e880dd8_f8ce_457b_8199_d60bb3d7ef98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectEvents_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub PropertyChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DirtyRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectFactory(::windows_core::IUnknown);
impl IMILBitmapEffectFactory {
    pub unsafe fn CreateEffect(&self, pguideffect: *const ::windows_core::GUID) -> ::windows_core::Result<IMILBitmapEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateEffect(pguideffect, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateContext(&self) -> ::windows_core::Result<IMILBitmapEffectRenderContext> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateContext(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateEffectOuter(&self) -> ::windows_core::Result<IMILBitmapEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateEffectOuter(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectFactory, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMILBitmapEffectFactory {
    type Vtable = IMILBitmapEffectFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33a9df34_a403_4ec7_b07e_bc0682370845);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectFactory_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguideffect: *const ::windows_core::GUID, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateEffectOuter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectGroup(::windows_core::IUnknown);
impl IMILBitmapEffectGroup {
    pub unsafe fn GetInteriorInputConnector(&self, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetInteriorInputConnector(uiindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetInteriorOutputConnector(&self, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectInputConnector> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetInteriorOutputConnector(uiindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn Add<P0>(&self, peffect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffect>,
    {
        ::windows_core::vcall!(self.Add(peffect.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectGroup, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMILBitmapEffectGroup {
    type Vtable = IMILBitmapEffectGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectGroup {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f952360_698a_4ac6_81a1_bcfdf08eb8e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectGroup_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetInteriorInputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetInteriorOutputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectGroupImpl(::windows_core::IUnknown);
impl IMILBitmapEffectGroupImpl {
    pub unsafe fn Preprocess<P0>(&self, pcontext: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectRenderContext>,
    {
        ::windows_core::vcall!(self.Preprocess(pcontext.into_param().abi())).ok()
    }
    pub unsafe fn GetNumberChildren(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetNumberChildren(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetChildren(&self) -> ::windows_core::Result<IMILBitmapEffects> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetChildren(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectGroupImpl, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMILBitmapEffectGroupImpl {
    type Vtable = IMILBitmapEffectGroupImpl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectGroupImpl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x78fed518_1cfc_4807_8b85_6b6e51398f62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectGroupImpl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Preprocess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetNumberChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puinumberchildren: *mut u32) -> ::windows_core::HRESULT,
    pub GetChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchildren: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectImpl(::windows_core::IUnknown);
impl IMILBitmapEffectImpl {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsInPlaceModificationAllowed<P0>(&self, poutputconnector: P0) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectOutputConnector>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.IsInPlaceModificationAllowed(poutputconnector.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn SetParentEffect<P0>(&self, pparenteffect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectGroup>,
    {
        ::windows_core::vcall!(self.SetParentEffect(pparenteffect.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Imaging\"`"]
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetInputSource(&self, uiindex: u32) -> ::windows_core::Result<super::super::Graphics::Imaging::IWICBitmapSource> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetInputSource(uiindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetInputSourceBounds(&self, uiindex: u32, prect: *mut MilRectD) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetInputSourceBounds(uiindex, prect)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Imaging\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn GetInputBitmapSource<P0>(&self, uiindex: u32, prendercontext: P0, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectRenderContext>,
    {
        ::windows_core::vcall!(self.GetInputBitmapSource(uiindex, prendercontext.into_param().abi(), pfmodifyinplace, ::core::mem::transmute(ppbitmapsource))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Imaging\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn GetOutputBitmapSource<P0>(&self, uiindex: u32, prendercontext: P0, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectRenderContext>,
    {
        ::windows_core::vcall!(self.GetOutputBitmapSource(uiindex, prendercontext.into_param().abi(), pfmodifyinplace, ::core::mem::transmute(ppbitmapsource))).ok()
    }
    pub unsafe fn Initialize<P0>(&self, pinner: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.Initialize(pinner.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectImpl, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMILBitmapEffectImpl {
    type Vtable = IMILBitmapEffectImpl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectImpl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcc2468f2_9936_47be_b4af_06b5df5dbcbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectImpl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsInPlaceModificationAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutputconnector: *mut ::core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsInPlaceModificationAllowed: usize,
    pub SetParentEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparenteffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub GetInputSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))]
    GetInputSource: usize,
    pub GetInputSourceBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, prect: *mut MilRectD) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub GetInputBitmapSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging")))]
    GetInputBitmapSource: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub GetOutputBitmapSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, prendercontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging")))]
    GetOutputBitmapSource: usize,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinner: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectInputConnector(::windows_core::IUnknown);
impl IMILBitmapEffectInputConnector {
    pub unsafe fn GetIndex(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetIndex(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetOptimalFormat(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetNumberFormats(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFormat(ulindex, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnected(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.IsConnected(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetBitmapEffect(&self) -> ::windows_core::Result<IMILBitmapEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetBitmapEffect(&mut result__)).from_abi(result__)
    }
    pub unsafe fn ConnectTo<P0>(&self, pconnector: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectOutputConnector>,
    {
        ::windows_core::vcall!(self.ConnectTo(pconnector.into_param().abi())).ok()
    }
    pub unsafe fn GetConnection(&self) -> ::windows_core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetConnection(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectInputConnector, ::windows_core::IUnknown, IMILBitmapEffectConnectorInfo, IMILBitmapEffectConnector);
unsafe impl ::windows_core::Interface for IMILBitmapEffectInputConnector {
    type Vtable = IMILBitmapEffectInputConnector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectInputConnector {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa9b4ecaa_7a3c_45e7_8573_f4b81b60dd6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInputConnector_Vtbl {
    pub base__: IMILBitmapEffectConnector_Vtbl,
    pub ConnectTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnector: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectInteriorInputConnector(::windows_core::IUnknown);
impl IMILBitmapEffectInteriorInputConnector {
    pub unsafe fn GetInputConnector(&self) -> ::windows_core::Result<IMILBitmapEffectInputConnector> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetInputConnector(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectInteriorInputConnector, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMILBitmapEffectInteriorInputConnector {
    type Vtable = IMILBitmapEffectInteriorInputConnector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectInteriorInputConnector {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20287e9e_86a2_4e15_953d_eb1438a5b842);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInteriorInputConnector_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetInputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinputconnector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectInteriorOutputConnector(::windows_core::IUnknown);
impl IMILBitmapEffectInteriorOutputConnector {
    pub unsafe fn GetOutputConnector(&self) -> ::windows_core::Result<IMILBitmapEffectOutputConnector> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetOutputConnector(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectInteriorOutputConnector, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMILBitmapEffectInteriorOutputConnector {
    type Vtable = IMILBitmapEffectInteriorOutputConnector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectInteriorOutputConnector {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00bbb6dc_acc9_4bfc_b344_8bee383dfefa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInteriorOutputConnector_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetOutputConnector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutputconnector: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectOutputConnector(::windows_core::IUnknown);
impl IMILBitmapEffectOutputConnector {
    pub unsafe fn GetIndex(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetIndex(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetOptimalFormat(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetNumberFormats(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFormat(ulindex, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsConnected(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.IsConnected(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetBitmapEffect(&self) -> ::windows_core::Result<IMILBitmapEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetBitmapEffect(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetNumberConnections(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetNumberConnections(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetConnection(&self, uiindex: u32) -> ::windows_core::Result<IMILBitmapEffectInputConnector> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetConnection(uiindex, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectOutputConnector, ::windows_core::IUnknown, IMILBitmapEffectConnectorInfo, IMILBitmapEffectConnector);
unsafe impl ::windows_core::Interface for IMILBitmapEffectOutputConnector {
    type Vtable = IMILBitmapEffectOutputConnector_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectOutputConnector {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92957aad_841b_4866_82ec_8752468b07fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectOutputConnector_Vtbl {
    pub base__: IMILBitmapEffectConnector_Vtbl,
    pub GetNumberConnections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puinumberconnections: *mut u32) -> ::windows_core::HRESULT,
    pub GetConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, ppconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectOutputConnectorImpl(::windows_core::IUnknown);
impl IMILBitmapEffectOutputConnectorImpl {
    pub unsafe fn AddBackLink<P0>(&self, pconnection: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectInputConnector>,
    {
        ::windows_core::vcall!(self.AddBackLink(pconnection.into_param().abi())).ok()
    }
    pub unsafe fn RemoveBackLink<P0>(&self, pconnection: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectInputConnector>,
    {
        ::windows_core::vcall!(self.RemoveBackLink(pconnection.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectOutputConnectorImpl, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMILBitmapEffectOutputConnectorImpl {
    type Vtable = IMILBitmapEffectOutputConnectorImpl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectOutputConnectorImpl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x21fae777_8b39_4bfa_9f2d_f3941ed36913);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectOutputConnectorImpl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddBackLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveBackLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconnection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectPrimitive(::windows_core::IUnknown);
impl IMILBitmapEffectPrimitive {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Imaging\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub unsafe fn GetOutput<P0>(&self, uiindex: u32, pcontext: P0, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut ::core::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IMILBitmapEffectRenderContext>,
    {
        ::windows_core::vcall!(self.GetOutput(uiindex, pcontext.into_param().abi(), pfmodifyinplace, ::core::mem::transmute(ppbitmapsource))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransformPoint<P0, P1>(&self, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: P0, pcontext: P1, pfpointtransformed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P1: ::windows_core::IntoParam<IMILBitmapEffectRenderContext>,
    {
        ::windows_core::vcall!(self.TransformPoint(uiindex, p, fforwardtransform.into_param().abi(), pcontext.into_param().abi(), pfpointtransformed)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TransformRect<P0, P1>(&self, uiindex: u32, p: *mut MilRectD, fforwardtransform: P0, pcontext: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P1: ::windows_core::IntoParam<IMILBitmapEffectRenderContext>,
    {
        ::windows_core::vcall!(self.TransformRect(uiindex, p, fforwardtransform.into_param().abi(), pcontext.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasAffineTransform(&self, uiindex: u32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.HasAffineTransform(uiindex, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasInverseTransform(&self, uiindex: u32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.HasInverseTransform(uiindex, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dwm\"`"]
    #[cfg(feature = "Win32_Graphics_Dwm")]
    pub unsafe fn GetAffineMatrix(&self, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetAffineMatrix(uiindex, pmatrix)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectPrimitive, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMILBitmapEffectPrimitive {
    type Vtable = IMILBitmapEffectPrimitive_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectPrimitive {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x67e31025_3091_4dfc_98d6_dd494551461d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectPrimitive_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    pub GetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pcontext: *mut ::core::ffi::c_void, pfmodifyinplace: *mut super::super::Foundation::VARIANT_BOOL, ppbitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging")))]
    GetOutput: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TransformPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: *mut ::core::ffi::c_void, pfpointtransformed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TransformPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TransformRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, p: *mut MilRectD, fforwardtransform: super::super::Foundation::VARIANT_BOOL, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TransformRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HasAffineTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pfaffine: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasAffineTransform: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HasInverseTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pfhasinverse: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasInverseTransform: usize,
    #[cfg(feature = "Win32_Graphics_Dwm")]
    pub GetAffineMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dwm"))]
    GetAffineMatrix: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectPrimitiveImpl(::windows_core::IUnknown);
impl IMILBitmapEffectPrimitiveImpl {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDirty(&self, uioutputindex: u32, pfdirty: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
        ::windows_core::vcall!(self.IsDirty(uioutputindex, pfdirty))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsVolatile(&self, uioutputindex: u32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.IsVolatile(uioutputindex, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectPrimitiveImpl, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMILBitmapEffectPrimitiveImpl {
    type Vtable = IMILBitmapEffectPrimitiveImpl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectPrimitiveImpl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce41e00b_efa6_44e7_b007_dd042e3ae126);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectPrimitiveImpl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDirty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfdirty: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDirty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsVolatile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uioutputindex: u32, pfvolatile: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsVolatile: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectRenderContext(::windows_core::IUnknown);
impl IMILBitmapEffectRenderContext {
    pub unsafe fn SetOutputPixelFormat(&self, format: *const ::windows_core::GUID) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOutputPixelFormat(format)).ok()
    }
    pub unsafe fn GetOutputPixelFormat(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetOutputPixelFormat(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseSoftwareRenderer<P0>(&self, fsoftware: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        ::windows_core::vcall!(self.SetUseSoftwareRenderer(fsoftware.into_param().abi())).ok()
    }
    pub unsafe fn SetInitialTransform(&self, pmatrix: *const MILMatrixF) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetInitialTransform(pmatrix)).ok()
    }
    pub unsafe fn GetFinalTransform(&self, pmatrix: *mut MILMatrixF) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFinalTransform(pmatrix)).ok()
    }
    pub unsafe fn SetOutputDPI(&self, dbldpix: f64, dbldpiy: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOutputDPI(dbldpix, dbldpiy)).ok()
    }
    pub unsafe fn GetOutputDPI(&self, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetOutputDPI(pdbldpix, pdbldpiy)).ok()
    }
    pub unsafe fn SetRegionOfInterest(&self, prect: *const MilRectD) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetRegionOfInterest(prect)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectRenderContext, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMILBitmapEffectRenderContext {
    type Vtable = IMILBitmapEffectRenderContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectRenderContext {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12a2ec7e_2d33_44b2_b334_1abb7846e390);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectRenderContext_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetOutputPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: *const ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetOutputPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseSoftwareRenderer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fsoftware: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseSoftwareRenderer: usize,
    pub SetInitialTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows_core::HRESULT,
    pub GetFinalTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows_core::HRESULT,
    pub SetOutputDPI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dbldpix: f64, dbldpiy: f64) -> ::windows_core::HRESULT,
    pub GetOutputDPI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows_core::HRESULT,
    pub SetRegionOfInterest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffectRenderContextImpl(::windows_core::IUnknown);
impl IMILBitmapEffectRenderContextImpl {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUseSoftwareRenderer(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetUseSoftwareRenderer(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetTransform(&self, pmatrix: *mut MILMatrixF) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetTransform(pmatrix)).ok()
    }
    pub unsafe fn UpdateTransform(&self, pmatrix: *const MILMatrixF) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.UpdateTransform(pmatrix)).ok()
    }
    pub unsafe fn GetOutputBounds(&self, prect: *mut MilRectD) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetOutputBounds(prect)).ok()
    }
    pub unsafe fn UpdateOutputBounds(&self, prect: *const MilRectD) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.UpdateOutputBounds(prect)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffectRenderContextImpl, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMILBitmapEffectRenderContextImpl {
    type Vtable = IMILBitmapEffectRenderContextImpl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffectRenderContextImpl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d25accb_797d_4fd2_b128_dffeff84fcc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectRenderContextImpl_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUseSoftwareRenderer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsoftware: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUseSoftwareRenderer: usize,
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *mut MILMatrixF) -> ::windows_core::HRESULT,
    pub UpdateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmatrix: *const MILMatrixF) -> ::windows_core::HRESULT,
    pub GetOutputBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *mut MilRectD) -> ::windows_core::HRESULT,
    pub UpdateOutputBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prect: *const MilRectD) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IMILBitmapEffects(::windows_core::IUnknown);
impl IMILBitmapEffects {
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self._NewEnum(&mut result__)).from_abi(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows_core::Result<IMILBitmapEffectGroup> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.Parent(&mut result__)).from_abi(result__)
    }
    pub unsafe fn Item(&self, uindex: u32) -> ::windows_core::Result<IMILBitmapEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.Item(uindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.Count(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IMILBitmapEffects, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMILBitmapEffects {
    type Vtable = IMILBitmapEffects_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IMILBitmapEffects {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51ac3dce_67c5_448b_9180_ad3eabddd5dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffects_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiureturn: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uindex: u32, ppeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puicount: *mut u32) -> ::windows_core::HRESULT,
}
pub const CLSID_MILBitmapEffectBevel: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd361dbe_6c9b_4de0_8290_f6400c2737ed);
pub const CLSID_MILBitmapEffectBlur: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa924df87_225d_4373_8f5b_b90ec85ae3de);
pub const CLSID_MILBitmapEffectDropShadow: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x459a3fbe_d8ac_4692_874b_7a265715aa16);
pub const CLSID_MILBitmapEffectEmboss: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd299846_824f_47ec_a007_12aa767f2816);
pub const CLSID_MILBitmapEffectGroup: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac9c1a9a_7e18_4f64_ac7e_47cf7f051e95);
pub const CLSID_MILBitmapEffectOuterGlow: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2161bdd_7eb6_4725_9c0b_8a2a1b4f0667);
pub const MILBITMAPEFFECT_SDK_VERSION: u32 = 16777216u32;
#[repr(C)]
pub struct MILMatrixF {
    pub _11: f64,
    pub _12: f64,
    pub _13: f64,
    pub _14: f64,
    pub _21: f64,
    pub _22: f64,
    pub _23: f64,
    pub _24: f64,
    pub _31: f64,
    pub _32: f64,
    pub _33: f64,
    pub _34: f64,
    pub _41: f64,
    pub _42: f64,
    pub _43: f64,
    pub _44: f64,
}
impl ::core::marker::Copy for MILMatrixF {}
impl ::core::clone::Clone for MILMatrixF {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MILMatrixF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MILMatrixF").field("_11", &self._11).field("_12", &self._12).field("_13", &self._13).field("_14", &self._14).field("_21", &self._21).field("_22", &self._22).field("_23", &self._23).field("_24", &self._24).field("_31", &self._31).field("_32", &self._32).field("_33", &self._33).field("_34", &self._34).field("_41", &self._41).field("_42", &self._42).field("_43", &self._43).field("_44", &self._44).finish()
    }
}
impl ::windows_core::TypeKind for MILMatrixF {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for MILMatrixF {
    fn eq(&self, other: &Self) -> bool {
        self._11 == other._11 && self._12 == other._12 && self._13 == other._13 && self._14 == other._14 && self._21 == other._21 && self._22 == other._22 && self._23 == other._23 && self._24 == other._24 && self._31 == other._31 && self._32 == other._32 && self._33 == other._33 && self._34 == other._34 && self._41 == other._41 && self._42 == other._42 && self._43 == other._43 && self._44 == other._44
    }
}
impl ::core::cmp::Eq for MILMatrixF {}
impl ::core::default::Default for MILMatrixF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MilPoint2D {
    pub X: f64,
    pub Y: f64,
}
impl ::core::marker::Copy for MilPoint2D {}
impl ::core::clone::Clone for MilPoint2D {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MilPoint2D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MilPoint2D").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::windows_core::TypeKind for MilPoint2D {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for MilPoint2D {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for MilPoint2D {}
impl ::core::default::Default for MilPoint2D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MilRectD {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
impl ::core::marker::Copy for MilRectD {}
impl ::core::clone::Clone for MilRectD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MilRectD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MilRectD").field("left", &self.left).field("top", &self.top).field("right", &self.right).field("bottom", &self.bottom).finish()
    }
}
impl ::windows_core::TypeKind for MilRectD {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for MilRectD {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.right == other.right && self.bottom == other.bottom
    }
}
impl ::core::cmp::Eq for MilRectD {}
impl ::core::default::Default for MilRectD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
