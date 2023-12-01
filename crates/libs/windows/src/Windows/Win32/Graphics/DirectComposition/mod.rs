#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionAttachMouseDragToHwnd<P0, P1, P2>(visual: P0, hwnd: P1, enable: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<IDCompositionVisual>,
    P1: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("dcomp.dll" "system" fn DCompositionAttachMouseDragToHwnd(visual : * mut::core::ffi::c_void, hwnd : super::super::Foundation:: HWND, enable : super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    DCompositionAttachMouseDragToHwnd(visual.into_param().abi(), hwnd.into_param().abi(), enable.into_param().abi()).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionAttachMouseWheelToHwnd<P0, P1, P2>(visual: P0, hwnd: P1, enable: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<IDCompositionVisual>,
    P1: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("dcomp.dll" "system" fn DCompositionAttachMouseWheelToHwnd(visual : * mut::core::ffi::c_void, hwnd : super::super::Foundation:: HWND, enable : super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    DCompositionAttachMouseWheelToHwnd(visual.into_param().abi(), hwnd.into_param().abi(), enable.into_param().abi()).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionBoostCompositorClock<P0>(enable: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("dcomp.dll" "system" fn DCompositionBoostCompositorClock(enable : super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    DCompositionBoostCompositorClock(enable.into_param().abi()).ok()
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn DCompositionCreateDevice<P0, T>(dxgidevice: P0) -> ::windows_core::Result<T>
where
    P0: ::windows_core::IntoParam<super::Dxgi::IDXGIDevice>,
    T: ::windows_core::ComInterface,
{
    ::windows_targets::link!("dcomp.dll" "system" fn DCompositionCreateDevice(dxgidevice : * mut::core::ffi::c_void, iid : *const ::windows_core::GUID, dcompositiondevice : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::ptr::null_mut();
    DCompositionCreateDevice(dxgidevice.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn DCompositionCreateDevice2<P0, T>(renderingdevice: P0) -> ::windows_core::Result<T>
where
    P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    T: ::windows_core::ComInterface,
{
    ::windows_targets::link!("dcomp.dll" "system" fn DCompositionCreateDevice2(renderingdevice : * mut::core::ffi::c_void, iid : *const ::windows_core::GUID, dcompositiondevice : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::ptr::null_mut();
    DCompositionCreateDevice2(renderingdevice.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn DCompositionCreateDevice3<P0, T>(renderingdevice: P0) -> ::windows_core::Result<T>
where
    P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    T: ::windows_core::ComInterface,
{
    ::windows_targets::link!("dcomp.dll" "system" fn DCompositionCreateDevice3(renderingdevice : * mut::core::ffi::c_void, iid : *const ::windows_core::GUID, dcompositiondevice : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::ptr::null_mut();
    DCompositionCreateDevice3(renderingdevice.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn DCompositionCreateSurfaceHandle(desiredaccess: u32, securityattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> ::windows_core::Result<super::super::Foundation::HANDLE> {
    ::windows_targets::link!("dcomp.dll" "system" fn DCompositionCreateSurfaceHandle(desiredaccess : u32, securityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, surfacehandle : *mut super::super::Foundation:: HANDLE) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    DCompositionCreateSurfaceHandle(desiredaccess, ::core::mem::transmute(securityattributes.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn DCompositionGetFrameId(frameidtype: COMPOSITION_FRAME_ID_TYPE) -> ::windows_core::Result<u64> {
    ::windows_targets::link!("dcomp.dll" "system" fn DCompositionGetFrameId(frameidtype : COMPOSITION_FRAME_ID_TYPE, frameid : *mut u64) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    DCompositionGetFrameId(frameidtype, &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionGetStatistics(frameid: u64, framestats: *mut COMPOSITION_FRAME_STATS, targetidcount: u32, targetids: ::core::option::Option<*mut COMPOSITION_TARGET_ID>, actualtargetidcount: ::core::option::Option<*mut u32>) -> ::windows_core::Result<()> {
    ::windows_targets::link!("dcomp.dll" "system" fn DCompositionGetStatistics(frameid : u64, framestats : *mut COMPOSITION_FRAME_STATS, targetidcount : u32, targetids : *mut COMPOSITION_TARGET_ID, actualtargetidcount : *mut u32) -> ::windows_core::HRESULT);
    DCompositionGetStatistics(frameid, framestats, targetidcount, ::core::mem::transmute(targetids.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(actualtargetidcount.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionGetTargetStatistics(frameid: u64, targetid: *const COMPOSITION_TARGET_ID, targetstats: *mut COMPOSITION_TARGET_STATS) -> ::windows_core::Result<()> {
    ::windows_targets::link!("dcomp.dll" "system" fn DCompositionGetTargetStatistics(frameid : u64, targetid : *const COMPOSITION_TARGET_ID, targetstats : *mut COMPOSITION_TARGET_STATS) -> ::windows_core::HRESULT);
    DCompositionGetTargetStatistics(frameid, targetid, targetstats).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionWaitForCompositorClock(handles: ::core::option::Option<&[super::super::Foundation::HANDLE]>, timeoutinms: u32) -> u32 {
    ::windows_targets::link!("dcomp.dll" "system" fn DCompositionWaitForCompositorClock(count : u32, handles : *const super::super::Foundation:: HANDLE, timeoutinms : u32) -> u32);
    DCompositionWaitForCompositorClock(handles.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(handles.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), timeoutinms)
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionAffineTransform2DEffect(::windows_core::IUnknown);
impl IDCompositionAffineTransform2DEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.SetInput(index, input.into_param().abi(), flags)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetInterpolationMode(&self, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetInterpolationMode(interpolationmode)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBorderMode(&self, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBorderMode(bordermode)).ok()
    }
    #[doc = "Required features: `\"Foundation_Numerics\"`"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransformMatrix(&self, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetTransformMatrix(transformmatrix)).ok()
    }
    pub unsafe fn SetTransformMatrixElement<P0>(&self, row: i32, column: i32, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetTransformMatrixElement(row, column, animation.into_param().abi())).ok()
    }
    pub unsafe fn SetTransformMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetTransformMatrixElement2(row, column, value)).ok()
    }
    pub unsafe fn SetSharpness<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetSharpness(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetSharpness2(&self, sharpness: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetSharpness2(sharpness)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionAffineTransform2DEffect, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
unsafe impl ::windows_core::Interface for IDCompositionAffineTransform2DEffect {
    type Vtable = IDCompositionAffineTransform2DEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionAffineTransform2DEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b74b9e8_cdd6_492f_bbbc_5ed32157026d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionAffineTransform2DEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetInterpolationMode: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBorderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBorderMode: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransformMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransformMatrix: usize,
    pub SetTransformMatrixElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTransformMatrixElement2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows_core::HRESULT,
    pub SetSharpness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSharpness2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharpness: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionAnimation(::windows_core::IUnknown);
impl IDCompositionAnimation {
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Reset()).ok()
    }
    pub unsafe fn SetAbsoluteBeginTime(&self, begintime: i64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAbsoluteBeginTime(begintime)).ok()
    }
    pub unsafe fn AddCubic(&self, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.AddCubic(beginoffset, constantcoefficient, linearcoefficient, quadraticcoefficient, cubiccoefficient)).ok()
    }
    pub unsafe fn AddSinusoidal(&self, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.AddSinusoidal(beginoffset, bias, amplitude, frequency, phase)).ok()
    }
    pub unsafe fn AddRepeat(&self, beginoffset: f64, durationtorepeat: f64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.AddRepeat(beginoffset, durationtorepeat)).ok()
    }
    pub unsafe fn End(&self, endoffset: f64, endvalue: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.End(endoffset, endvalue)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionAnimation, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDCompositionAnimation {
    type Vtable = IDCompositionAnimation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionAnimation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcbfd91d9_51b2_45e4_b3de_d19ccfb863c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionAnimation_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAbsoluteBeginTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, begintime: i64) -> ::windows_core::HRESULT,
    pub AddCubic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows_core::HRESULT,
    pub AddSinusoidal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows_core::HRESULT,
    pub AddRepeat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, beginoffset: f64, durationtorepeat: f64) -> ::windows_core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endoffset: f64, endvalue: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionArithmeticCompositeEffect(::windows_core::IUnknown);
impl IDCompositionArithmeticCompositeEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.SetInput(index, input.into_param().abi(), flags)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetCoefficients(&self, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCoefficients(coefficients)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClampOutput<P0>(&self, clampoutput: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetClampOutput(clampoutput.into_param().abi())).ok()
    }
    pub unsafe fn SetCoefficient1<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetCoefficient1(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetCoefficient12(&self, coeffcient1: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCoefficient12(coeffcient1)).ok()
    }
    pub unsafe fn SetCoefficient2<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetCoefficient2(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetCoefficient22(&self, coefficient2: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCoefficient22(coefficient2)).ok()
    }
    pub unsafe fn SetCoefficient3<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetCoefficient3(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetCoefficient32(&self, coefficient3: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCoefficient32(coefficient3)).ok()
    }
    pub unsafe fn SetCoefficient4<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetCoefficient4(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetCoefficient42(&self, coefficient4: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCoefficient42(coefficient4)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionArithmeticCompositeEffect, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
unsafe impl ::windows_core::Interface for IDCompositionArithmeticCompositeEffect {
    type Vtable = IDCompositionArithmeticCompositeEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionArithmeticCompositeEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b67dfa8_e3dd_4e61_b640_46c2f3d739dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionArithmeticCompositeEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetCoefficients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetCoefficients: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClampOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClampOutput: usize,
    pub SetCoefficient1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCoefficient12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coeffcient1: f32) -> ::windows_core::HRESULT,
    pub SetCoefficient2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCoefficient22: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coefficient2: f32) -> ::windows_core::HRESULT,
    pub SetCoefficient3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCoefficient32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coefficient3: f32) -> ::windows_core::HRESULT,
    pub SetCoefficient4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCoefficient42: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coefficient4: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionBlendEffect(::windows_core::IUnknown);
impl IDCompositionBlendEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.SetInput(index, input.into_param().abi(), flags)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetMode(&self, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetMode(mode)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionBlendEffect, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
unsafe impl ::windows_core::Interface for IDCompositionBlendEffect {
    type Vtable = IDCompositionBlendEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionBlendEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33ecdc0a_578a_4a11_9c14_0cb90517f9c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionBlendEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMode: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionBrightnessEffect(::windows_core::IUnknown);
impl IDCompositionBrightnessEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.SetInput(index, input.into_param().abi(), flags)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetWhitePoint(&self, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetWhitePoint(whitepoint)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBlackPoint(&self, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBlackPoint(blackpoint)).ok()
    }
    pub unsafe fn SetWhitePointX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetWhitePointX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetWhitePointX2(&self, whitepointx: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetWhitePointX2(whitepointx)).ok()
    }
    pub unsafe fn SetWhitePointY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetWhitePointY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetWhitePointY2(&self, whitepointy: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetWhitePointY2(whitepointy)).ok()
    }
    pub unsafe fn SetBlackPointX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetBlackPointX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetBlackPointX2(&self, blackpointx: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBlackPointX2(blackpointx)).ok()
    }
    pub unsafe fn SetBlackPointY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetBlackPointY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetBlackPointY2(&self, blackpointy: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBlackPointY2(blackpointy)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionBrightnessEffect, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
unsafe impl ::windows_core::Interface for IDCompositionBrightnessEffect {
    type Vtable = IDCompositionBrightnessEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionBrightnessEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6027496e_cb3a_49ab_934f_d798da4f7da6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionBrightnessEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetWhitePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetWhitePoint: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBlackPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBlackPoint: usize,
    pub SetWhitePointX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetWhitePointX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, whitepointx: f32) -> ::windows_core::HRESULT,
    pub SetWhitePointY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetWhitePointY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, whitepointy: f32) -> ::windows_core::HRESULT,
    pub SetBlackPointX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetBlackPointX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blackpointx: f32) -> ::windows_core::HRESULT,
    pub SetBlackPointY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetBlackPointY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blackpointy: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionClip(::windows_core::IUnknown);
impl IDCompositionClip {}
::windows_core::imp::interface_hierarchy!(IDCompositionClip, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDCompositionClip {
    type Vtable = IDCompositionClip_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionClip {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64ac3703_9d3f_45ec_a109_7cac0e7a13a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionClip_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionColorMatrixEffect(::windows_core::IUnknown);
impl IDCompositionColorMatrixEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.SetInput(index, input.into_param().abi(), flags)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetMatrix(&self, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetMatrix(matrix)).ok()
    }
    pub unsafe fn SetMatrixElement<P0>(&self, row: i32, column: i32, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetMatrixElement(row, column, animation.into_param().abi())).ok()
    }
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetMatrixElement2(row, column, value)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetAlphaMode(&self, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAlphaMode(mode)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClampOutput<P0>(&self, clamp: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetClampOutput(clamp.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionColorMatrixEffect, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
unsafe impl ::windows_core::Interface for IDCompositionColorMatrixEffect {
    type Vtable = IDCompositionColorMatrixEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionColorMatrixEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1170a22_3ce2_4966_90d4_55408bfc84c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionColorMatrixEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetAlphaMode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClampOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clamp: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClampOutput: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionCompositeEffect(::windows_core::IUnknown);
impl IDCompositionCompositeEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.SetInput(index, input.into_param().abi(), flags)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetMode(&self, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetMode(mode)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionCompositeEffect, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
unsafe impl ::windows_core::Interface for IDCompositionCompositeEffect {
    type Vtable = IDCompositionCompositeEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionCompositeEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x576616c0_a231_494d_a38d_00fd5ec4db46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionCompositeEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMode: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionDelegatedInkTrail(::windows_core::IUnknown);
impl IDCompositionDelegatedInkTrail {
    pub unsafe fn AddTrailPoints(&self, inkpoints: &[DCompositionInkTrailPoint]) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.AddTrailPoints(::core::mem::transmute(inkpoints.as_ptr()), inkpoints.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn AddTrailPointsWithPrediction(&self, inkpoints: &[DCompositionInkTrailPoint], predictedinkpoints: &[DCompositionInkTrailPoint]) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.AddTrailPointsWithPrediction(::core::mem::transmute(inkpoints.as_ptr()), inkpoints.len().try_into().unwrap(), ::core::mem::transmute(predictedinkpoints.as_ptr()), predictedinkpoints.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn RemoveTrailPoints(&self, generationid: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.RemoveTrailPoints(generationid)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn StartNewTrail(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.StartNewTrail(color)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionDelegatedInkTrail, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDCompositionDelegatedInkTrail {
    type Vtable = IDCompositionDelegatedInkTrail_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionDelegatedInkTrail {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2448e9b_547d_4057_8cf5_8144ede1c2da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDelegatedInkTrail_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddTrailPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, generationid: *mut u32) -> ::windows_core::HRESULT,
    pub AddTrailPointsWithPrediction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32, generationid: *mut u32) -> ::windows_core::HRESULT,
    pub RemoveTrailPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, generationid: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub StartNewTrail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    StartNewTrail: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionDesktopDevice(::windows_core::IUnknown);
impl IDCompositionDesktopDevice {
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.Commit()).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.WaitForCommitCompletion()).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetFrameStatistics(statistics)).ok()
    }
    pub unsafe fn CreateVisual(&self) -> ::windows_core::Result<IDCompositionVisual2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateVisual(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateSurfaceFactory<P0>(&self, renderingdevice: P0) -> ::windows_core::Result<IDCompositionSurfaceFactory>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateSurfaceFactory(renderingdevice.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionSurface> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateSurface(width, height, pixelformat, alphamode, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionVirtualSurface> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateVirtualSurface(initialwidth, initialheight, pixelformat, alphamode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows_core::Result<IDCompositionTranslateTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateTranslateTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateScaleTransform(&self) -> ::windows_core::Result<IDCompositionScaleTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateScaleTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRotateTransform(&self) -> ::windows_core::Result<IDCompositionRotateTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateRotateTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateSkewTransform(&self) -> ::windows_core::Result<IDCompositionSkewTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateSkewTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows_core::Result<IDCompositionMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateMatrixTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTransformGroup(&self, transforms: &[::core::option::Option<IDCompositionTransform>]) -> ::windows_core::Result<IDCompositionTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateTransformGroup(::core::mem::transmute(transforms.as_ptr()), transforms.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows_core::Result<IDCompositionTranslateTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateTranslateTransform3D(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows_core::Result<IDCompositionScaleTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateScaleTransform3D(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows_core::Result<IDCompositionRotateTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateRotateTransform3D(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows_core::Result<IDCompositionMatrixTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateMatrixTransform3D(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[::core::option::Option<IDCompositionTransform3D>]) -> ::windows_core::Result<IDCompositionTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateTransform3DGroup(::core::mem::transmute(transforms3d.as_ptr()), transforms3d.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateEffectGroup(&self) -> ::windows_core::Result<IDCompositionEffectGroup> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateEffectGroup(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRectangleClip(&self) -> ::windows_core::Result<IDCompositionRectangleClip> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateRectangleClip(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateAnimation(&self) -> ::windows_core::Result<IDCompositionAnimation> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateAnimation(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTargetForHwnd<P0, P1>(&self, hwnd: P0, topmost: P1) -> ::windows_core::Result<IDCompositionTarget>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTargetForHwnd(hwnd.into_param().abi(), topmost.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurfaceFromHandle<P0>(&self, handle: P0) -> ::windows_core::Result<::windows_core::IUnknown>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateSurfaceFromHandle(handle.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurfaceFromHwnd<P0>(&self, hwnd: P0) -> ::windows_core::Result<::windows_core::IUnknown>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateSurfaceFromHwnd(hwnd.into_param().abi(), &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionDesktopDevice, ::windows_core::IUnknown, IDCompositionDevice2);
unsafe impl ::windows_core::Interface for IDCompositionDesktopDevice {
    type Vtable = IDCompositionDesktopDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionDesktopDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f4633fe_1e08_4cb8_8c75_ce24333f5602);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDesktopDevice_Vtbl {
    pub base__: IDCompositionDevice2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTargetForHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTargetForHwnd: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSurfaceFromHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSurfaceFromHandle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSurfaceFromHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSurfaceFromHwnd: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionDevice(::windows_core::IUnknown);
impl IDCompositionDevice {
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Commit()).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.WaitForCommitCompletion()).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFrameStatistics(statistics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTargetForHwnd<P0, P1>(&self, hwnd: P0, topmost: P1) -> ::windows_core::Result<IDCompositionTarget>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTargetForHwnd(hwnd.into_param().abi(), topmost.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateVisual(&self) -> ::windows_core::Result<IDCompositionVisual> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateVisual(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionSurface> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateSurface(width, height, pixelformat, alphamode, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionVirtualSurface> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateVirtualSurface(initialwidth, initialheight, pixelformat, alphamode, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurfaceFromHandle<P0>(&self, handle: P0) -> ::windows_core::Result<::windows_core::IUnknown>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateSurfaceFromHandle(handle.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurfaceFromHwnd<P0>(&self, hwnd: P0) -> ::windows_core::Result<::windows_core::IUnknown>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateSurfaceFromHwnd(hwnd.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows_core::Result<IDCompositionTranslateTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTranslateTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateScaleTransform(&self) -> ::windows_core::Result<IDCompositionScaleTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateScaleTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRotateTransform(&self) -> ::windows_core::Result<IDCompositionRotateTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateRotateTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateSkewTransform(&self) -> ::windows_core::Result<IDCompositionSkewTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateSkewTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows_core::Result<IDCompositionMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateMatrixTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTransformGroup(&self, transforms: &[::core::option::Option<IDCompositionTransform>]) -> ::windows_core::Result<IDCompositionTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTransformGroup(::core::mem::transmute(transforms.as_ptr()), transforms.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows_core::Result<IDCompositionTranslateTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTranslateTransform3D(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows_core::Result<IDCompositionScaleTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateScaleTransform3D(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows_core::Result<IDCompositionRotateTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateRotateTransform3D(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows_core::Result<IDCompositionMatrixTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateMatrixTransform3D(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[::core::option::Option<IDCompositionTransform3D>]) -> ::windows_core::Result<IDCompositionTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTransform3DGroup(::core::mem::transmute(transforms3d.as_ptr()), transforms3d.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateEffectGroup(&self) -> ::windows_core::Result<IDCompositionEffectGroup> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateEffectGroup(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRectangleClip(&self) -> ::windows_core::Result<IDCompositionRectangleClip> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateRectangleClip(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateAnimation(&self) -> ::windows_core::Result<IDCompositionAnimation> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateAnimation(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckDeviceState(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CheckDeviceState(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionDevice, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDCompositionDevice {
    type Vtable = IDCompositionDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc37ea93a_e7aa_450d_b16f_9746cb0407f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WaitForCommitCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetFrameStatistics: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTargetForHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTargetForHwnd: usize,
    pub CreateVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSurfaceFromHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSurfaceFromHandle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSurfaceFromHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSurfaceFromHwnd: usize,
    pub CreateTranslateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translatetransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateScaleTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaletransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateRotateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotatetransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSkewTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, skewtransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateTransformGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transforms: *const *mut ::core::ffi::c_void, elements: u32, transformgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateTranslateTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translatetransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateScaleTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaletransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateRotateTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateMatrixTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateTransform3DGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transforms3d: *const *mut ::core::ffi::c_void, elements: u32, transform3dgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateEffectGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateRectangleClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clip: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CheckDeviceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CheckDeviceState: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionDevice2(::windows_core::IUnknown);
impl IDCompositionDevice2 {
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Commit()).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.WaitForCommitCompletion()).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFrameStatistics(statistics)).ok()
    }
    pub unsafe fn CreateVisual(&self) -> ::windows_core::Result<IDCompositionVisual2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateVisual(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateSurfaceFactory<P0>(&self, renderingdevice: P0) -> ::windows_core::Result<IDCompositionSurfaceFactory>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateSurfaceFactory(renderingdevice.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionSurface> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateSurface(width, height, pixelformat, alphamode, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionVirtualSurface> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateVirtualSurface(initialwidth, initialheight, pixelformat, alphamode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows_core::Result<IDCompositionTranslateTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTranslateTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateScaleTransform(&self) -> ::windows_core::Result<IDCompositionScaleTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateScaleTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRotateTransform(&self) -> ::windows_core::Result<IDCompositionRotateTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateRotateTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateSkewTransform(&self) -> ::windows_core::Result<IDCompositionSkewTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateSkewTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows_core::Result<IDCompositionMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateMatrixTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTransformGroup(&self, transforms: &[::core::option::Option<IDCompositionTransform>]) -> ::windows_core::Result<IDCompositionTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTransformGroup(::core::mem::transmute(transforms.as_ptr()), transforms.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows_core::Result<IDCompositionTranslateTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTranslateTransform3D(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows_core::Result<IDCompositionScaleTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateScaleTransform3D(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows_core::Result<IDCompositionRotateTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateRotateTransform3D(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows_core::Result<IDCompositionMatrixTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateMatrixTransform3D(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[::core::option::Option<IDCompositionTransform3D>]) -> ::windows_core::Result<IDCompositionTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTransform3DGroup(::core::mem::transmute(transforms3d.as_ptr()), transforms3d.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateEffectGroup(&self) -> ::windows_core::Result<IDCompositionEffectGroup> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateEffectGroup(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRectangleClip(&self) -> ::windows_core::Result<IDCompositionRectangleClip> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateRectangleClip(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateAnimation(&self) -> ::windows_core::Result<IDCompositionAnimation> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateAnimation(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionDevice2, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDCompositionDevice2 {
    type Vtable = IDCompositionDevice2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionDevice2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75f6468d_1b8e_447c_9bc6_75fea80b5b25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WaitForCommitCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetFrameStatistics: usize,
    pub CreateVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSurfaceFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, surfacefactory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
    pub CreateTranslateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translatetransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateScaleTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaletransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateRotateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotatetransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSkewTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, skewtransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateTransformGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transforms: *const *mut ::core::ffi::c_void, elements: u32, transformgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateTranslateTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translatetransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateScaleTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaletransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateRotateTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateMatrixTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateTransform3DGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transforms3d: *const *mut ::core::ffi::c_void, elements: u32, transform3dgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateEffectGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateRectangleClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clip: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionDevice3(::windows_core::IUnknown);
impl IDCompositionDevice3 {
    pub unsafe fn Commit(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.Commit()).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.WaitForCommitCompletion()).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetFrameStatistics(statistics)).ok()
    }
    pub unsafe fn CreateVisual(&self) -> ::windows_core::Result<IDCompositionVisual2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateVisual(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateSurfaceFactory<P0>(&self, renderingdevice: P0) -> ::windows_core::Result<IDCompositionSurfaceFactory>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateSurfaceFactory(renderingdevice.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionSurface> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateSurface(width, height, pixelformat, alphamode, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionVirtualSurface> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateVirtualSurface(initialwidth, initialheight, pixelformat, alphamode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows_core::Result<IDCompositionTranslateTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateTranslateTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateScaleTransform(&self) -> ::windows_core::Result<IDCompositionScaleTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateScaleTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRotateTransform(&self) -> ::windows_core::Result<IDCompositionRotateTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateRotateTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateSkewTransform(&self) -> ::windows_core::Result<IDCompositionSkewTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateSkewTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows_core::Result<IDCompositionMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateMatrixTransform(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTransformGroup(&self, transforms: &[::core::option::Option<IDCompositionTransform>]) -> ::windows_core::Result<IDCompositionTransform> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateTransformGroup(::core::mem::transmute(transforms.as_ptr()), transforms.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows_core::Result<IDCompositionTranslateTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateTranslateTransform3D(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows_core::Result<IDCompositionScaleTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateScaleTransform3D(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows_core::Result<IDCompositionRotateTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateRotateTransform3D(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows_core::Result<IDCompositionMatrixTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateMatrixTransform3D(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[::core::option::Option<IDCompositionTransform3D>]) -> ::windows_core::Result<IDCompositionTransform3D> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateTransform3DGroup(::core::mem::transmute(transforms3d.as_ptr()), transforms3d.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateEffectGroup(&self) -> ::windows_core::Result<IDCompositionEffectGroup> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateEffectGroup(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRectangleClip(&self) -> ::windows_core::Result<IDCompositionRectangleClip> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateRectangleClip(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateAnimation(&self) -> ::windows_core::Result<IDCompositionAnimation> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateAnimation(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateGaussianBlurEffect(&self) -> ::windows_core::Result<IDCompositionGaussianBlurEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateGaussianBlurEffect(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateBrightnessEffect(&self) -> ::windows_core::Result<IDCompositionBrightnessEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateBrightnessEffect(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateColorMatrixEffect(&self) -> ::windows_core::Result<IDCompositionColorMatrixEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateColorMatrixEffect(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateShadowEffect(&self) -> ::windows_core::Result<IDCompositionShadowEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateShadowEffect(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateHueRotationEffect(&self) -> ::windows_core::Result<IDCompositionHueRotationEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateHueRotationEffect(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateSaturationEffect(&self) -> ::windows_core::Result<IDCompositionSaturationEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateSaturationEffect(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTurbulenceEffect(&self) -> ::windows_core::Result<IDCompositionTurbulenceEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTurbulenceEffect(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateLinearTransferEffect(&self) -> ::windows_core::Result<IDCompositionLinearTransferEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateLinearTransferEffect(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTableTransferEffect(&self) -> ::windows_core::Result<IDCompositionTableTransferEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTableTransferEffect(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCompositeEffect(&self) -> ::windows_core::Result<IDCompositionCompositeEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateCompositeEffect(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateBlendEffect(&self) -> ::windows_core::Result<IDCompositionBlendEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateBlendEffect(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateArithmeticCompositeEffect(&self) -> ::windows_core::Result<IDCompositionArithmeticCompositeEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateArithmeticCompositeEffect(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateAffineTransform2DEffect(&self) -> ::windows_core::Result<IDCompositionAffineTransform2DEffect> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateAffineTransform2DEffect(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionDevice3, ::windows_core::IUnknown, IDCompositionDevice2);
unsafe impl ::windows_core::Interface for IDCompositionDevice3 {
    type Vtable = IDCompositionDevice3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionDevice3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0987cb06_f916_48bf_8d35_ce7641781bd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice3_Vtbl {
    pub base__: IDCompositionDevice2_Vtbl,
    pub CreateGaussianBlurEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gaussianblureffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateBrightnessEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brightnesseffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateColorMatrixEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colormatrixeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateShadowEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shadoweffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateHueRotationEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, huerotationeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSaturationEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, saturationeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateTurbulenceEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, turbulenceeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateLinearTransferEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineartransfereffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateTableTransferEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tabletransfereffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateCompositeEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositeeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateBlendEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blendeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateArithmeticCompositeEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arithmeticcompositeeffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateAffineTransform2DEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, affinetransform2deffect: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionDeviceDebug(::windows_core::IUnknown);
impl IDCompositionDeviceDebug {
    pub unsafe fn EnableDebugCounters(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.EnableDebugCounters()).ok()
    }
    pub unsafe fn DisableDebugCounters(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.DisableDebugCounters()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionDeviceDebug, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDCompositionDeviceDebug {
    type Vtable = IDCompositionDeviceDebug_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionDeviceDebug {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1a3c64a_224f_4a81_9773_4f03a89d3c6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDeviceDebug_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub EnableDebugCounters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DisableDebugCounters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionEffect(::windows_core::IUnknown);
impl IDCompositionEffect {}
::windows_core::imp::interface_hierarchy!(IDCompositionEffect, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDCompositionEffect {
    type Vtable = IDCompositionEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec81b08f_bfcb_4e8d_b193_a915587999e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionEffect_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionEffectGroup(::windows_core::IUnknown);
impl IDCompositionEffectGroup {
    pub unsafe fn SetOpacity<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetOpacity(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetOpacity2(&self, opacity: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOpacity2(opacity)).ok()
    }
    pub unsafe fn SetTransform3D<P0>(&self, transform3d: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionTransform3D>,
    {
        ::windows_core::vcall!(self.SetTransform3D(transform3d.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionEffectGroup, ::windows_core::IUnknown, IDCompositionEffect);
unsafe impl ::windows_core::Interface for IDCompositionEffectGroup {
    type Vtable = IDCompositionEffectGroup_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionEffectGroup {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7929a74_e6b2_4bd6_8b95_4040119ca34d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionEffectGroup_Vtbl {
    pub base__: IDCompositionEffect_Vtbl,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetOpacity2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows_core::HRESULT,
    pub SetTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform3d: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionFilterEffect(::windows_core::IUnknown);
impl IDCompositionFilterEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.SetInput(index, input.into_param().abi(), flags)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionFilterEffect, ::windows_core::IUnknown, IDCompositionEffect);
unsafe impl ::windows_core::Interface for IDCompositionFilterEffect {
    type Vtable = IDCompositionFilterEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionFilterEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30c421d5_8cb2_4e9f_b133_37be270d4ac2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionFilterEffect_Vtbl {
    pub base__: IDCompositionEffect_Vtbl,
    pub SetInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, input: *mut ::core::ffi::c_void, flags: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionGaussianBlurEffect(::windows_core::IUnknown);
impl IDCompositionGaussianBlurEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.SetInput(index, input.into_param().abi(), flags)).ok()
    }
    pub unsafe fn SetStandardDeviation<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetStandardDeviation(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetStandardDeviation2(&self, amount: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetStandardDeviation2(amount)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBorderMode(&self, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBorderMode(mode)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionGaussianBlurEffect, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
unsafe impl ::windows_core::Interface for IDCompositionGaussianBlurEffect {
    type Vtable = IDCompositionGaussianBlurEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionGaussianBlurEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45d4d0b7_1bd4_454e_8894_2bfa68443033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionGaussianBlurEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetStandardDeviation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetStandardDeviation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBorderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBorderMode: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionHueRotationEffect(::windows_core::IUnknown);
impl IDCompositionHueRotationEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.SetInput(index, input.into_param().abi(), flags)).ok()
    }
    pub unsafe fn SetAngle<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetAngle(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetAngle2(&self, amountdegrees: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAngle2(amountdegrees)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionHueRotationEffect, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
unsafe impl ::windows_core::Interface for IDCompositionHueRotationEffect {
    type Vtable = IDCompositionHueRotationEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionHueRotationEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6db9f920_0770_4781_b0c6_381912f9d167);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionHueRotationEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amountdegrees: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionInkTrailDevice(::windows_core::IUnknown);
impl IDCompositionInkTrailDevice {
    pub unsafe fn CreateDelegatedInkTrail(&self) -> ::windows_core::Result<IDCompositionDelegatedInkTrail> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateDelegatedInkTrail(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateDelegatedInkTrailForSwapChain<P0>(&self, swapchain: P0) -> ::windows_core::Result<IDCompositionDelegatedInkTrail>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateDelegatedInkTrailForSwapChain(swapchain.into_param().abi(), &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionInkTrailDevice, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDCompositionInkTrailDevice {
    type Vtable = IDCompositionInkTrailDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionInkTrailDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf0c7cec_cdeb_4d4a_b91c_721bf22f4e6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionInkTrailDevice_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateDelegatedInkTrail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inktrail: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateDelegatedInkTrailForSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, inktrail: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionLinearTransferEffect(::windows_core::IUnknown);
impl IDCompositionLinearTransferEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.SetInput(index, input.into_param().abi(), flags)).ok()
    }
    pub unsafe fn SetRedYIntercept<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetRedYIntercept(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetRedYIntercept2(&self, redyintercept: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetRedYIntercept2(redyintercept)).ok()
    }
    pub unsafe fn SetRedSlope<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetRedSlope(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetRedSlope2(&self, redslope: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetRedSlope2(redslope)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRedDisable<P0>(&self, reddisable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetRedDisable(reddisable.into_param().abi())).ok()
    }
    pub unsafe fn SetGreenYIntercept<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetGreenYIntercept(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetGreenYIntercept2(&self, greenyintercept: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetGreenYIntercept2(greenyintercept)).ok()
    }
    pub unsafe fn SetGreenSlope<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetGreenSlope(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetGreenSlope2(&self, greenslope: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetGreenSlope2(greenslope)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGreenDisable<P0>(&self, greendisable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetGreenDisable(greendisable.into_param().abi())).ok()
    }
    pub unsafe fn SetBlueYIntercept<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetBlueYIntercept(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetBlueYIntercept2(&self, blueyintercept: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBlueYIntercept2(blueyintercept)).ok()
    }
    pub unsafe fn SetBlueSlope<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetBlueSlope(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetBlueSlope2(&self, blueslope: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBlueSlope2(blueslope)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBlueDisable<P0>(&self, bluedisable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetBlueDisable(bluedisable.into_param().abi())).ok()
    }
    pub unsafe fn SetAlphaYIntercept<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetAlphaYIntercept(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetAlphaYIntercept2(&self, alphayintercept: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAlphaYIntercept2(alphayintercept)).ok()
    }
    pub unsafe fn SetAlphaSlope<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetAlphaSlope(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetAlphaSlope2(&self, alphaslope: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAlphaSlope2(alphaslope)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlphaDisable<P0>(&self, alphadisable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetAlphaDisable(alphadisable.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClampOutput<P0>(&self, clampoutput: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetClampOutput(clampoutput.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionLinearTransferEffect, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
unsafe impl ::windows_core::Interface for IDCompositionLinearTransferEffect {
    type Vtable = IDCompositionLinearTransferEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionLinearTransferEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4305ee5b_c4a0_4c88_9385_67124e017683);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionLinearTransferEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetRedYIntercept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRedYIntercept2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, redyintercept: f32) -> ::windows_core::HRESULT,
    pub SetRedSlope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRedSlope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, redslope: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRedDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRedDisable: usize,
    pub SetGreenYIntercept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetGreenYIntercept2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, greenyintercept: f32) -> ::windows_core::HRESULT,
    pub SetGreenSlope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetGreenSlope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, greenslope: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGreenDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGreenDisable: usize,
    pub SetBlueYIntercept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetBlueYIntercept2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blueyintercept: f32) -> ::windows_core::HRESULT,
    pub SetBlueSlope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetBlueSlope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blueslope: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBlueDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBlueDisable: usize,
    pub SetAlphaYIntercept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAlphaYIntercept2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphayintercept: f32) -> ::windows_core::HRESULT,
    pub SetAlphaSlope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAlphaSlope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphaslope: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAlphaDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAlphaDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClampOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClampOutput: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionMatrixTransform(::windows_core::IUnknown);
impl IDCompositionMatrixTransform {
    #[doc = "Required features: `\"Foundation_Numerics\"`"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetMatrix(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetMatrix(matrix)).ok()
    }
    pub unsafe fn SetMatrixElement<P0>(&self, row: i32, column: i32, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetMatrixElement(row, column, animation.into_param().abi())).ok()
    }
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetMatrixElement2(row, column, value)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionMatrixTransform, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D, IDCompositionTransform);
unsafe impl ::windows_core::Interface for IDCompositionMatrixTransform {
    type Vtable = IDCompositionMatrixTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionMatrixTransform {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x16cdff07_c503_419c_83f2_0965c7af1fa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionMatrixTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionMatrixTransform3D(::windows_core::IUnknown);
impl IDCompositionMatrixTransform3D {
    #[doc = "Required features: `\"Foundation_Numerics\"`"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetMatrix(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetMatrix(matrix)).ok()
    }
    pub unsafe fn SetMatrixElement<P0>(&self, row: i32, column: i32, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetMatrixElement(row, column, animation.into_param().abi())).ok()
    }
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetMatrixElement2(row, column, value)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionMatrixTransform3D, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D);
unsafe impl ::windows_core::Interface for IDCompositionMatrixTransform3D {
    type Vtable = IDCompositionMatrixTransform3D_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionMatrixTransform3D {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b3363f0_643b_41b7_b6e0_ccf22d34467c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionMatrixTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix4x4) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionRectangleClip(::windows_core::IUnknown);
impl IDCompositionRectangleClip {
    pub unsafe fn SetLeft<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetLeft(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetLeft2(&self, left: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetLeft2(left)).ok()
    }
    pub unsafe fn SetTop<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetTop(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetTop2(&self, top: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetTop2(top)).ok()
    }
    pub unsafe fn SetRight<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetRight(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetRight2(&self, right: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetRight2(right)).ok()
    }
    pub unsafe fn SetBottom<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetBottom(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetBottom2(&self, bottom: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBottom2(bottom)).ok()
    }
    pub unsafe fn SetTopLeftRadiusX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetTopLeftRadiusX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetTopLeftRadiusX2(&self, radius: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetTopLeftRadiusX2(radius)).ok()
    }
    pub unsafe fn SetTopLeftRadiusY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetTopLeftRadiusY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetTopLeftRadiusY2(&self, radius: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetTopLeftRadiusY2(radius)).ok()
    }
    pub unsafe fn SetTopRightRadiusX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetTopRightRadiusX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetTopRightRadiusX2(&self, radius: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetTopRightRadiusX2(radius)).ok()
    }
    pub unsafe fn SetTopRightRadiusY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetTopRightRadiusY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetTopRightRadiusY2(&self, radius: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetTopRightRadiusY2(radius)).ok()
    }
    pub unsafe fn SetBottomLeftRadiusX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetBottomLeftRadiusX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetBottomLeftRadiusX2(&self, radius: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBottomLeftRadiusX2(radius)).ok()
    }
    pub unsafe fn SetBottomLeftRadiusY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetBottomLeftRadiusY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetBottomLeftRadiusY2(&self, radius: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBottomLeftRadiusY2(radius)).ok()
    }
    pub unsafe fn SetBottomRightRadiusX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetBottomRightRadiusX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetBottomRightRadiusX2(&self, radius: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBottomRightRadiusX2(radius)).ok()
    }
    pub unsafe fn SetBottomRightRadiusY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetBottomRightRadiusY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetBottomRightRadiusY2(&self, radius: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBottomRightRadiusY2(radius)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionRectangleClip, ::windows_core::IUnknown, IDCompositionClip);
unsafe impl ::windows_core::Interface for IDCompositionRectangleClip {
    type Vtable = IDCompositionRectangleClip_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionRectangleClip {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9842ad7d_d9cf_4908_aed7_48b51da5e7c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRectangleClip_Vtbl {
    pub base__: IDCompositionClip_Vtbl,
    pub SetLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetLeft2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: f32) -> ::windows_core::HRESULT,
    pub SetTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTop2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: f32) -> ::windows_core::HRESULT,
    pub SetRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRight2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: f32) -> ::windows_core::HRESULT,
    pub SetBottom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetBottom2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows_core::HRESULT,
    pub SetTopLeftRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTopLeftRadiusX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT,
    pub SetTopLeftRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTopLeftRadiusY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT,
    pub SetTopRightRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTopRightRadiusX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT,
    pub SetTopRightRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTopRightRadiusY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT,
    pub SetBottomLeftRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetBottomLeftRadiusX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT,
    pub SetBottomLeftRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetBottomLeftRadiusY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT,
    pub SetBottomRightRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetBottomRightRadiusX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT,
    pub SetBottomRightRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetBottomRightRadiusY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionRotateTransform(::windows_core::IUnknown);
impl IDCompositionRotateTransform {
    pub unsafe fn SetAngle<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetAngle(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetAngle2(&self, angle: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAngle2(angle)).ok()
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetCenterX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCenterX2(centerx)).ok()
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetCenterY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCenterY2(centery)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionRotateTransform, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D, IDCompositionTransform);
unsafe impl ::windows_core::Interface for IDCompositionRotateTransform {
    type Vtable = IDCompositionRotateTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionRotateTransform {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x641ed83c_ae96_46c5_90dc_32774cc5c6d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRotateTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionRotateTransform3D(::windows_core::IUnknown);
impl IDCompositionRotateTransform3D {
    pub unsafe fn SetAngle<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetAngle(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetAngle2(&self, angle: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAngle2(angle)).ok()
    }
    pub unsafe fn SetAxisX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetAxisX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetAxisX2(&self, axisx: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAxisX2(axisx)).ok()
    }
    pub unsafe fn SetAxisY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetAxisY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetAxisY2(&self, axisy: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAxisY2(axisy)).ok()
    }
    pub unsafe fn SetAxisZ<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetAxisZ(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetAxisZ2(&self, axisz: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAxisZ2(axisz)).ok()
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetCenterX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCenterX2(centerx)).ok()
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetCenterY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCenterY2(centery)).ok()
    }
    pub unsafe fn SetCenterZ<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetCenterZ(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetCenterZ2(&self, centerz: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCenterZ2(centerz)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionRotateTransform3D, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D);
unsafe impl ::windows_core::Interface for IDCompositionRotateTransform3D {
    type Vtable = IDCompositionRotateTransform3D_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionRotateTransform3D {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8f5b23f_d429_4a91_b55a_d2f45fd75b18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRotateTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    pub SetAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows_core::HRESULT,
    pub SetAxisX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAxisX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, axisx: f32) -> ::windows_core::HRESULT,
    pub SetAxisY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAxisY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, axisy: f32) -> ::windows_core::HRESULT,
    pub SetAxisZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAxisZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, axisz: f32) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows_core::HRESULT,
    pub SetCenterZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCenterZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionSaturationEffect(::windows_core::IUnknown);
impl IDCompositionSaturationEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.SetInput(index, input.into_param().abi(), flags)).ok()
    }
    pub unsafe fn SetSaturation<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetSaturation(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetSaturation2(&self, ratio: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetSaturation2(ratio)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionSaturationEffect, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
unsafe impl ::windows_core::Interface for IDCompositionSaturationEffect {
    type Vtable = IDCompositionSaturationEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionSaturationEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa08debda_3258_4fa4_9f16_9174d3fe93b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSaturationEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetSaturation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSaturation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ratio: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionScaleTransform(::windows_core::IUnknown);
impl IDCompositionScaleTransform {
    pub unsafe fn SetScaleX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetScaleX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetScaleX2(&self, scalex: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetScaleX2(scalex)).ok()
    }
    pub unsafe fn SetScaleY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetScaleY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetScaleY2(&self, scaley: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetScaleY2(scaley)).ok()
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetCenterX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCenterX2(centerx)).ok()
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetCenterY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCenterY2(centery)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionScaleTransform, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D, IDCompositionTransform);
unsafe impl ::windows_core::Interface for IDCompositionScaleTransform {
    type Vtable = IDCompositionScaleTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionScaleTransform {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71fde914_40ef_45ef_bd51_68b037c339f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionScaleTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetScaleX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows_core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetScaleY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionScaleTransform3D(::windows_core::IUnknown);
impl IDCompositionScaleTransform3D {
    pub unsafe fn SetScaleX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetScaleX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetScaleX2(&self, scalex: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetScaleX2(scalex)).ok()
    }
    pub unsafe fn SetScaleY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetScaleY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetScaleY2(&self, scaley: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetScaleY2(scaley)).ok()
    }
    pub unsafe fn SetScaleZ<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetScaleZ(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetScaleZ2(&self, scalez: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetScaleZ2(scalez)).ok()
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetCenterX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCenterX2(centerx)).ok()
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetCenterY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCenterY2(centery)).ok()
    }
    pub unsafe fn SetCenterZ<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetCenterZ(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetCenterZ2(&self, centerz: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCenterZ2(centerz)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionScaleTransform3D, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D);
unsafe impl ::windows_core::Interface for IDCompositionScaleTransform3D {
    type Vtable = IDCompositionScaleTransform3D_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionScaleTransform3D {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a9e9ead_364b_4b15_a7c4_a1997f78b389);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionScaleTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    pub SetScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetScaleX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows_core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetScaleY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows_core::HRESULT,
    pub SetScaleZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetScaleZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scalez: f32) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows_core::HRESULT,
    pub SetCenterZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCenterZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionShadowEffect(::windows_core::IUnknown);
impl IDCompositionShadowEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.SetInput(index, input.into_param().abi(), flags)).ok()
    }
    pub unsafe fn SetStandardDeviation<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetStandardDeviation(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetStandardDeviation2(&self, amount: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetStandardDeviation2(amount)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetColor(&self, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetColor(color)).ok()
    }
    pub unsafe fn SetRed<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetRed(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetRed2(&self, amount: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetRed2(amount)).ok()
    }
    pub unsafe fn SetGreen<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetGreen(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetGreen2(&self, amount: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetGreen2(amount)).ok()
    }
    pub unsafe fn SetBlue<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetBlue(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetBlue2(&self, amount: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBlue2(amount)).ok()
    }
    pub unsafe fn SetAlpha<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetAlpha(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetAlpha2(&self, amount: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAlpha2(amount)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionShadowEffect, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
unsafe impl ::windows_core::Interface for IDCompositionShadowEffect {
    type Vtable = IDCompositionShadowEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionShadowEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ad18ac0_cfd2_4c2f_bb62_96e54fdb6879);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionShadowEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetStandardDeviation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetStandardDeviation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetColor: usize,
    pub SetRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRed2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT,
    pub SetGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetGreen2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT,
    pub SetBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetBlue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT,
    pub SetAlpha: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAlpha2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionSkewTransform(::windows_core::IUnknown);
impl IDCompositionSkewTransform {
    pub unsafe fn SetAngleX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetAngleX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetAngleX2(&self, anglex: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAngleX2(anglex)).ok()
    }
    pub unsafe fn SetAngleY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetAngleY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetAngleY2(&self, angley: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAngleY2(angley)).ok()
    }
    pub unsafe fn SetCenterX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetCenterX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCenterX2(centerx)).ok()
    }
    pub unsafe fn SetCenterY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetCenterY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCenterY2(centery)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionSkewTransform, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D, IDCompositionTransform);
unsafe impl ::windows_core::Interface for IDCompositionSkewTransform {
    type Vtable = IDCompositionSkewTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionSkewTransform {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe57aa735_dcdb_4c72_9c61_0591f58889ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSkewTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetAngleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAngleX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anglex: f32) -> ::windows_core::HRESULT,
    pub SetAngleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAngleY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, angley: f32) -> ::windows_core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows_core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionSurface(::windows_core::IUnknown);
impl IDCompositionSurface {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginDraw<T>(&self, updaterect: ::core::option::Option<*const super::super::Foundation::RECT>, updateoffset: *mut super::super::Foundation::POINT) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        ::windows_core::vcall!(self.BeginDraw(::core::mem::transmute(updaterect.unwrap_or(::std::ptr::null())), &<T as ::windows_core::ComInterface>::IID, &mut result__, updateoffset)).from_abi(result__)
    }
    pub unsafe fn EndDraw(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.EndDraw()).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SuspendDraw()).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.ResumeDraw()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scroll(&self, scrollrect: ::core::option::Option<*const super::super::Foundation::RECT>, cliprect: ::core::option::Option<*const super::super::Foundation::RECT>, offsetx: i32, offsety: i32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Scroll(::core::mem::transmute(scrollrect.unwrap_or(::std::ptr::null())), ::core::mem::transmute(cliprect.unwrap_or(::std::ptr::null())), offsetx, offsety)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionSurface, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDCompositionSurface {
    type Vtable = IDCompositionSurface_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionSurface {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb8a4953_2c99_4f5a_96f5_4819027fa3ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSurface_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginDraw: usize,
    pub EndDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ResumeDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Scroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Scroll: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionSurfaceFactory(::windows_core::IUnknown);
impl IDCompositionSurfaceFactory {
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionSurface> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateSurface(width, height, pixelformat, alphamode, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<IDCompositionVirtualSurface> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateVirtualSurface(initialwidth, initialheight, pixelformat, alphamode, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionSurfaceFactory, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDCompositionSurfaceFactory {
    type Vtable = IDCompositionSurfaceFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionSurfaceFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe334bc12_3937_4e02_85eb_fcf4eb30d2c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSurfaceFactory_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionTableTransferEffect(::windows_core::IUnknown);
impl IDCompositionTableTransferEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.SetInput(index, input.into_param().abi(), flags)).ok()
    }
    pub unsafe fn SetRedTable(&self, tablevalues: &[f32]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetRedTable(::core::mem::transmute(tablevalues.as_ptr()), tablevalues.len().try_into().unwrap())).ok()
    }
    pub unsafe fn SetGreenTable(&self, tablevalues: &[f32]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetGreenTable(::core::mem::transmute(tablevalues.as_ptr()), tablevalues.len().try_into().unwrap())).ok()
    }
    pub unsafe fn SetBlueTable(&self, tablevalues: &[f32]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBlueTable(::core::mem::transmute(tablevalues.as_ptr()), tablevalues.len().try_into().unwrap())).ok()
    }
    pub unsafe fn SetAlphaTable(&self, tablevalues: &[f32]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAlphaTable(::core::mem::transmute(tablevalues.as_ptr()), tablevalues.len().try_into().unwrap())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRedDisable<P0>(&self, reddisable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetRedDisable(reddisable.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGreenDisable<P0>(&self, greendisable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetGreenDisable(greendisable.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBlueDisable<P0>(&self, bluedisable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetBlueDisable(bluedisable.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlphaDisable<P0>(&self, alphadisable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetAlphaDisable(alphadisable.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClampOutput<P0>(&self, clampoutput: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetClampOutput(clampoutput.into_param().abi())).ok()
    }
    pub unsafe fn SetRedTableValue<P0>(&self, index: u32, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetRedTableValue(index, animation.into_param().abi())).ok()
    }
    pub unsafe fn SetRedTableValue2(&self, index: u32, value: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetRedTableValue2(index, value)).ok()
    }
    pub unsafe fn SetGreenTableValue<P0>(&self, index: u32, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetGreenTableValue(index, animation.into_param().abi())).ok()
    }
    pub unsafe fn SetGreenTableValue2(&self, index: u32, value: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetGreenTableValue2(index, value)).ok()
    }
    pub unsafe fn SetBlueTableValue<P0>(&self, index: u32, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetBlueTableValue(index, animation.into_param().abi())).ok()
    }
    pub unsafe fn SetBlueTableValue2(&self, index: u32, value: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBlueTableValue2(index, value)).ok()
    }
    pub unsafe fn SetAlphaTableValue<P0>(&self, index: u32, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetAlphaTableValue(index, animation.into_param().abi())).ok()
    }
    pub unsafe fn SetAlphaTableValue2(&self, index: u32, value: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAlphaTableValue2(index, value)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionTableTransferEffect, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
unsafe impl ::windows_core::Interface for IDCompositionTableTransferEffect {
    type Vtable = IDCompositionTableTransferEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionTableTransferEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b7e82e2_69c5_4eb4_a5f5_a7033f5132cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTableTransferEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    pub SetRedTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows_core::HRESULT,
    pub SetGreenTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows_core::HRESULT,
    pub SetBlueTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows_core::HRESULT,
    pub SetAlphaTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRedDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRedDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGreenDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGreenDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBlueDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBlueDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAlphaDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAlphaDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClampOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClampOutput: usize,
    pub SetRedTableValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRedTableValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows_core::HRESULT,
    pub SetGreenTableValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetGreenTableValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows_core::HRESULT,
    pub SetBlueTableValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetBlueTableValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows_core::HRESULT,
    pub SetAlphaTableValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAlphaTableValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionTarget(::windows_core::IUnknown);
impl IDCompositionTarget {
    pub unsafe fn SetRoot<P0>(&self, visual: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionVisual>,
    {
        ::windows_core::vcall!(self.SetRoot(visual.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionTarget, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDCompositionTarget {
    type Vtable = IDCompositionTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionTarget {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeacdd04c_117e_4e17_88f4_d1b12b0e3d89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTarget_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionTransform(::windows_core::IUnknown);
impl IDCompositionTransform {}
::windows_core::imp::interface_hierarchy!(IDCompositionTransform, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D);
unsafe impl ::windows_core::Interface for IDCompositionTransform {
    type Vtable = IDCompositionTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionTransform {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd55faa7_37e0_4c20_95d2_9be45bc33f55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTransform_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionTransform3D(::windows_core::IUnknown);
impl IDCompositionTransform3D {}
::windows_core::imp::interface_hierarchy!(IDCompositionTransform3D, ::windows_core::IUnknown, IDCompositionEffect);
unsafe impl ::windows_core::Interface for IDCompositionTransform3D {
    type Vtable = IDCompositionTransform3D_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionTransform3D {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71185722_246b_41f2_aad1_0443f7f4bfc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTransform3D_Vtbl {
    pub base__: IDCompositionEffect_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionTranslateTransform(::windows_core::IUnknown);
impl IDCompositionTranslateTransform {
    pub unsafe fn SetOffsetX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetOffsetX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOffsetX2(offsetx)).ok()
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetOffsetY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOffsetY2(offsety)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionTranslateTransform, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D, IDCompositionTransform);
unsafe impl ::windows_core::Interface for IDCompositionTranslateTransform {
    type Vtable = IDCompositionTranslateTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionTranslateTransform {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x06791122_c6f0_417d_8323_269e987f5954);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTranslateTransform_Vtbl {
    pub base__: IDCompositionTransform_Vtbl,
    pub SetOffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows_core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionTranslateTransform3D(::windows_core::IUnknown);
impl IDCompositionTranslateTransform3D {
    pub unsafe fn SetOffsetX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetOffsetX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOffsetX2(offsetx)).ok()
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetOffsetY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOffsetY2(offsety)).ok()
    }
    pub unsafe fn SetOffsetZ<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetOffsetZ(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetOffsetZ2(&self, offsetz: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOffsetZ2(offsetz)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionTranslateTransform3D, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionTransform3D);
unsafe impl ::windows_core::Interface for IDCompositionTranslateTransform3D {
    type Vtable = IDCompositionTranslateTransform3D_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionTranslateTransform3D {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x91636d4b_9ba1_4532_aaf7_e3344994d788);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTranslateTransform3D_Vtbl {
    pub base__: IDCompositionTransform3D_Vtbl,
    pub SetOffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows_core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows_core::HRESULT,
    pub SetOffsetZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetOffsetZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionTurbulenceEffect(::windows_core::IUnknown);
impl IDCompositionTurbulenceEffect {
    pub unsafe fn SetInput<P0>(&self, index: u32, input: P0, flags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.SetInput(index, input.into_param().abi(), flags)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetOffset(&self, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOffset(offset)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBaseFrequency(&self, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBaseFrequency(frequency)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetSize(&self, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetSize(size)).ok()
    }
    pub unsafe fn SetNumOctaves(&self, numoctaves: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetNumOctaves(numoctaves)).ok()
    }
    pub unsafe fn SetSeed(&self, seed: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetSeed(seed)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetNoise(&self, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetNoise(noise)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStitchable<P0>(&self, stitchable: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetStitchable(stitchable.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionTurbulenceEffect, ::windows_core::IUnknown, IDCompositionEffect, IDCompositionFilterEffect);
unsafe impl ::windows_core::Interface for IDCompositionTurbulenceEffect {
    type Vtable = IDCompositionTurbulenceEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionTurbulenceEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa6a55bda_c09c_49f3_9193_a41922c89715);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTurbulenceEffect_Vtbl {
    pub base__: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetOffset: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBaseFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBaseFrequency: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetSize: usize,
    pub SetNumOctaves: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numoctaves: u32) -> ::windows_core::HRESULT,
    pub SetSeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seed: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetNoise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetNoise: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStitchable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stitchable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStitchable: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionVirtualSurface(::windows_core::IUnknown);
impl IDCompositionVirtualSurface {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginDraw<T>(&self, updaterect: ::core::option::Option<*const super::super::Foundation::RECT>, updateoffset: *mut super::super::Foundation::POINT) -> ::windows_core::Result<T>
    where
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        ::windows_core::vcall!(self.base__.BeginDraw(::core::mem::transmute(updaterect.unwrap_or(::std::ptr::null())), &<T as ::windows_core::ComInterface>::IID, &mut result__, updateoffset)).from_abi(result__)
    }
    pub unsafe fn EndDraw(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.EndDraw()).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SuspendDraw()).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.ResumeDraw()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scroll(&self, scrollrect: ::core::option::Option<*const super::super::Foundation::RECT>, cliprect: ::core::option::Option<*const super::super::Foundation::RECT>, offsetx: i32, offsety: i32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.Scroll(::core::mem::transmute(scrollrect.unwrap_or(::std::ptr::null())), ::core::mem::transmute(cliprect.unwrap_or(::std::ptr::null())), offsetx, offsety)).ok()
    }
    pub unsafe fn Resize(&self, width: u32, height: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Resize(width, height)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Trim(&self, rectangles: ::core::option::Option<&[super::super::Foundation::RECT]>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Trim(::core::mem::transmute(rectangles.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), rectangles.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionVirtualSurface, ::windows_core::IUnknown, IDCompositionSurface);
unsafe impl ::windows_core::Interface for IDCompositionVirtualSurface {
    type Vtable = IDCompositionVirtualSurface_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionVirtualSurface {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae471c51_5f53_4a24_8d3e_d0c39c30b3f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVirtualSurface_Vtbl {
    pub base__: IDCompositionSurface_Vtbl,
    pub Resize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Trim: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangles: *const super::super::Foundation::RECT, count: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Trim: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionVisual(::windows_core::IUnknown);
impl IDCompositionVisual {
    pub unsafe fn SetOffsetX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetOffsetX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOffsetX2(offsetx)).ok()
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetOffsetY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOffsetY2(offsety)).ok()
    }
    pub unsafe fn SetTransform<P0>(&self, transform: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionTransform>,
    {
        ::windows_core::vcall!(self.SetTransform(transform.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Foundation_Numerics\"`"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetTransform2(matrix)).ok()
    }
    pub unsafe fn SetTransformParent<P0>(&self, visual: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionVisual>,
    {
        ::windows_core::vcall!(self.SetTransformParent(visual.into_param().abi())).ok()
    }
    pub unsafe fn SetEffect<P0>(&self, effect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionEffect>,
    {
        ::windows_core::vcall!(self.SetEffect(effect.into_param().abi())).ok()
    }
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBitmapInterpolationMode(interpolationmode)).ok()
    }
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBorderMode(bordermode)).ok()
    }
    pub unsafe fn SetClip<P0>(&self, clip: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionClip>,
    {
        ::windows_core::vcall!(self.SetClip(clip.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetClip2(rect)).ok()
    }
    pub unsafe fn SetContent<P0>(&self, content: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.SetContent(content.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddVisual<P0, P1, P2>(&self, visual: P0, insertabove: P1, referencevisual: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionVisual>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<IDCompositionVisual>,
    {
        ::windows_core::vcall!(self.AddVisual(visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi())).ok()
    }
    pub unsafe fn RemoveVisual<P0>(&self, visual: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionVisual>,
    {
        ::windows_core::vcall!(self.RemoveVisual(visual.into_param().abi())).ok()
    }
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.RemoveAllVisuals()).ok()
    }
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCompositeMode(compositemode)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionVisual, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDCompositionVisual {
    type Vtable = IDCompositionVisual_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionVisual {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d93059d_097b_4651_9a60_f0f25116e2f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetOffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows_core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows_core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform2: usize,
    pub SetTransformParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetBitmapInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows_core::HRESULT,
    pub SetBorderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows_core::HRESULT,
    pub SetClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clip: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetClip2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetClip2: usize,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void, insertabove: super::super::Foundation::BOOL, referencevisual: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddVisual: usize,
    pub RemoveVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAllVisuals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCompositeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionVisual2(::windows_core::IUnknown);
impl IDCompositionVisual2 {
    pub unsafe fn SetOffsetX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.base__.SetOffsetX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetOffsetX2(offsetx)).ok()
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.base__.SetOffsetY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetOffsetY2(offsety)).ok()
    }
    pub unsafe fn SetTransform<P0>(&self, transform: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionTransform>,
    {
        ::windows_core::vcall!(self.base__.SetTransform(transform.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Foundation_Numerics\"`"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetTransform2(matrix)).ok()
    }
    pub unsafe fn SetTransformParent<P0>(&self, visual: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionVisual>,
    {
        ::windows_core::vcall!(self.base__.SetTransformParent(visual.into_param().abi())).ok()
    }
    pub unsafe fn SetEffect<P0>(&self, effect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionEffect>,
    {
        ::windows_core::vcall!(self.base__.SetEffect(effect.into_param().abi())).ok()
    }
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetBitmapInterpolationMode(interpolationmode)).ok()
    }
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetBorderMode(bordermode)).ok()
    }
    pub unsafe fn SetClip<P0>(&self, clip: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionClip>,
    {
        ::windows_core::vcall!(self.base__.SetClip(clip.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetClip2(rect)).ok()
    }
    pub unsafe fn SetContent<P0>(&self, content: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.SetContent(content.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddVisual<P0, P1, P2>(&self, visual: P0, insertabove: P1, referencevisual: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionVisual>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<IDCompositionVisual>,
    {
        ::windows_core::vcall!(self.base__.AddVisual(visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi())).ok()
    }
    pub unsafe fn RemoveVisual<P0>(&self, visual: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionVisual>,
    {
        ::windows_core::vcall!(self.base__.RemoveVisual(visual.into_param().abi())).ok()
    }
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.RemoveAllVisuals()).ok()
    }
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetCompositeMode(compositemode)).ok()
    }
    pub unsafe fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOpacityMode(mode)).ok()
    }
    pub unsafe fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBackFaceVisibility(visibility)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionVisual2, ::windows_core::IUnknown, IDCompositionVisual);
unsafe impl ::windows_core::Interface for IDCompositionVisual2 {
    type Vtable = IDCompositionVisual2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionVisual2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8de1639_4331_4b26_bc5f_6a321d347a85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual2_Vtbl {
    pub base__: IDCompositionVisual_Vtbl,
    pub SetOpacityMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows_core::HRESULT,
    pub SetBackFaceVisibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionVisual3(::windows_core::IUnknown);
impl IDCompositionVisual3 {
    pub unsafe fn SetOffsetX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetOffsetX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetOffsetX2(offsetx)).ok()
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetOffsetY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetOffsetY2(offsety)).ok()
    }
    pub unsafe fn SetTransform<P0>(&self, transform: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionTransform>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetTransform(transform.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Foundation_Numerics\"`"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetTransform2(matrix)).ok()
    }
    pub unsafe fn SetTransformParent<P0>(&self, visual: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionVisual>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetTransformParent(visual.into_param().abi())).ok()
    }
    pub unsafe fn SetEffect<P0>(&self, effect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionEffect>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetEffect(effect.into_param().abi())).ok()
    }
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetBitmapInterpolationMode(interpolationmode)).ok()
    }
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetBorderMode(bordermode)).ok()
    }
    pub unsafe fn SetClip<P0>(&self, clip: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionClip>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetClip(clip.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetClip2(rect)).ok()
    }
    pub unsafe fn SetContent<P0>(&self, content: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetContent(content.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddVisual<P0, P1, P2>(&self, visual: P0, insertabove: P1, referencevisual: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionVisual>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<IDCompositionVisual>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.AddVisual(visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi())).ok()
    }
    pub unsafe fn RemoveVisual<P0>(&self, visual: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionVisual>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.RemoveVisual(visual.into_param().abi())).ok()
    }
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.RemoveAllVisuals()).ok()
    }
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetCompositeMode(compositemode)).ok()
    }
    pub unsafe fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetOpacityMode(mode)).ok()
    }
    pub unsafe fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetBackFaceVisibility(visibility)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn EnableHeatMap(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.EnableHeatMap(color)).ok()
    }
    pub unsafe fn DisableHeatMap(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.DisableHeatMap()).ok()
    }
    pub unsafe fn EnableRedrawRegions(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.EnableRedrawRegions()).ok()
    }
    pub unsafe fn DisableRedrawRegions(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.DisableRedrawRegions()).ok()
    }
    pub unsafe fn SetDepthMode(&self, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetDepthMode(mode)).ok()
    }
    pub unsafe fn SetOffsetZ<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetOffsetZ(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetOffsetZ2(&self, offsetz: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOffsetZ2(offsetz)).ok()
    }
    pub unsafe fn SetOpacity<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.SetOpacity(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetOpacity2(&self, opacity: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOpacity2(opacity)).ok()
    }
    pub unsafe fn SetTransform3<P0>(&self, transform: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionTransform3D>,
    {
        ::windows_core::vcall!(self.SetTransform3(transform.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetTransform4(&self, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetTransform4(matrix)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVisible<P0>(&self, visible: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetVisible(visible.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionVisual3, ::windows_core::IUnknown, IDCompositionVisual, IDCompositionVisual2, IDCompositionVisualDebug);
unsafe impl ::windows_core::Interface for IDCompositionVisual3 {
    type Vtable = IDCompositionVisual3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionVisual3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2775f462_b6c1_4015_b0be_b3e7d6a4976d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual3_Vtbl {
    pub base__: IDCompositionVisualDebug_Vtbl,
    pub SetDepthMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows_core::HRESULT,
    pub SetOffsetZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetOffsetZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetOpacity2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows_core::HRESULT,
    pub SetTransform3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetTransform4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetTransform4: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVisible: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDCompositionVisualDebug(::windows_core::IUnknown);
impl IDCompositionVisualDebug {
    pub unsafe fn SetOffsetX<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.base__.base__.SetOffsetX(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetOffsetX2(offsetx)).ok()
    }
    pub unsafe fn SetOffsetY<P0>(&self, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionAnimation>,
    {
        ::windows_core::vcall!(self.base__.base__.SetOffsetY(animation.into_param().abi())).ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetOffsetY2(offsety)).ok()
    }
    pub unsafe fn SetTransform<P0>(&self, transform: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionTransform>,
    {
        ::windows_core::vcall!(self.base__.base__.SetTransform(transform.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Foundation_Numerics\"`"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetTransform2(matrix)).ok()
    }
    pub unsafe fn SetTransformParent<P0>(&self, visual: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionVisual>,
    {
        ::windows_core::vcall!(self.base__.base__.SetTransformParent(visual.into_param().abi())).ok()
    }
    pub unsafe fn SetEffect<P0>(&self, effect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionEffect>,
    {
        ::windows_core::vcall!(self.base__.base__.SetEffect(effect.into_param().abi())).ok()
    }
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetBitmapInterpolationMode(interpolationmode)).ok()
    }
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetBorderMode(bordermode)).ok()
    }
    pub unsafe fn SetClip<P0>(&self, clip: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionClip>,
    {
        ::windows_core::vcall!(self.base__.base__.SetClip(clip.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetClip2(rect)).ok()
    }
    pub unsafe fn SetContent<P0>(&self, content: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.base__.SetContent(content.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddVisual<P0, P1, P2>(&self, visual: P0, insertabove: P1, referencevisual: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionVisual>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<IDCompositionVisual>,
    {
        ::windows_core::vcall!(self.base__.base__.AddVisual(visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi())).ok()
    }
    pub unsafe fn RemoveVisual<P0>(&self, visual: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDCompositionVisual>,
    {
        ::windows_core::vcall!(self.base__.base__.RemoveVisual(visual.into_param().abi())).ok()
    }
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.RemoveAllVisuals()).ok()
    }
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetCompositeMode(compositemode)).ok()
    }
    pub unsafe fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetOpacityMode(mode)).ok()
    }
    pub unsafe fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetBackFaceVisibility(visibility)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn EnableHeatMap(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.EnableHeatMap(color)).ok()
    }
    pub unsafe fn DisableHeatMap(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.DisableHeatMap()).ok()
    }
    pub unsafe fn EnableRedrawRegions(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.EnableRedrawRegions()).ok()
    }
    pub unsafe fn DisableRedrawRegions(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.DisableRedrawRegions()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDCompositionVisualDebug, ::windows_core::IUnknown, IDCompositionVisual, IDCompositionVisual2);
unsafe impl ::windows_core::Interface for IDCompositionVisualDebug {
    type Vtable = IDCompositionVisualDebug_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDCompositionVisualDebug {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfed2b808_5eb4_43a0_aea3_35f65280f91b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisualDebug_Vtbl {
    pub base__: IDCompositionVisual2_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub EnableHeatMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    EnableHeatMap: usize,
    pub DisableHeatMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnableRedrawRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DisableRedrawRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
pub const COMPOSITIONOBJECT_READ: i32 = 1i32;
pub const COMPOSITIONOBJECT_WRITE: i32 = 2i32;
pub const COMPOSITION_FRAME_ID_COMPLETED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(2i32);
pub const COMPOSITION_FRAME_ID_CONFIRMED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(1i32);
pub const COMPOSITION_FRAME_ID_CREATED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(0i32);
pub const COMPOSITION_STATS_MAX_TARGETS: u32 = 256u32;
pub const DCOMPOSITION_BACKFACE_VISIBILITY_HIDDEN: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(1i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_INHERIT: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(-1i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_VISIBLE: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(0i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_INHERIT: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(-1i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_LINEAR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(1i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(0i32);
pub const DCOMPOSITION_BORDER_MODE_HARD: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(1i32);
pub const DCOMPOSITION_BORDER_MODE_INHERIT: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(-1i32);
pub const DCOMPOSITION_BORDER_MODE_SOFT: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(0i32);
pub const DCOMPOSITION_COMPOSITE_MODE_DESTINATION_INVERT: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(1i32);
pub const DCOMPOSITION_COMPOSITE_MODE_INHERIT: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(-1i32);
pub const DCOMPOSITION_COMPOSITE_MODE_MIN_BLEND: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(2i32);
pub const DCOMPOSITION_COMPOSITE_MODE_SOURCE_OVER: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(0i32);
pub const DCOMPOSITION_DEPTH_MODE_INHERIT: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(-1i32);
pub const DCOMPOSITION_DEPTH_MODE_SORTED: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(3i32);
pub const DCOMPOSITION_DEPTH_MODE_SPATIAL: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(1i32);
pub const DCOMPOSITION_DEPTH_MODE_TREE: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(0i32);
pub const DCOMPOSITION_MAX_WAITFORCOMPOSITORCLOCK_OBJECTS: u32 = 32u32;
pub const DCOMPOSITION_OPACITY_MODE_INHERIT: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(-1i32);
pub const DCOMPOSITION_OPACITY_MODE_LAYER: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(0i32);
pub const DCOMPOSITION_OPACITY_MODE_MULTIPLY: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(1i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMPOSITION_FRAME_ID_TYPE(pub i32);
impl ::core::marker::Copy for COMPOSITION_FRAME_ID_TYPE {}
impl ::core::clone::Clone for COMPOSITION_FRAME_ID_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMPOSITION_FRAME_ID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for COMPOSITION_FRAME_ID_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for COMPOSITION_FRAME_ID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPOSITION_FRAME_ID_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DCOMPOSITION_BACKFACE_VISIBILITY(pub i32);
impl ::core::marker::Copy for DCOMPOSITION_BACKFACE_VISIBILITY {}
impl ::core::clone::Clone for DCOMPOSITION_BACKFACE_VISIBILITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_BACKFACE_VISIBILITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DCOMPOSITION_BACKFACE_VISIBILITY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DCOMPOSITION_BACKFACE_VISIBILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_BACKFACE_VISIBILITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DCOMPOSITION_BITMAP_INTERPOLATION_MODE(pub i32);
impl ::core::marker::Copy for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_BITMAP_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DCOMPOSITION_BORDER_MODE(pub i32);
impl ::core::marker::Copy for DCOMPOSITION_BORDER_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_BORDER_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_BORDER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DCOMPOSITION_BORDER_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DCOMPOSITION_BORDER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_BORDER_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DCOMPOSITION_COMPOSITE_MODE(pub i32);
impl ::core::marker::Copy for DCOMPOSITION_COMPOSITE_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_COMPOSITE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_COMPOSITE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DCOMPOSITION_COMPOSITE_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DCOMPOSITION_COMPOSITE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_COMPOSITE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DCOMPOSITION_DEPTH_MODE(pub i32);
impl ::core::marker::Copy for DCOMPOSITION_DEPTH_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_DEPTH_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_DEPTH_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DCOMPOSITION_DEPTH_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DCOMPOSITION_DEPTH_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_DEPTH_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DCOMPOSITION_OPACITY_MODE(pub i32);
impl ::core::marker::Copy for DCOMPOSITION_OPACITY_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_OPACITY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_OPACITY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DCOMPOSITION_OPACITY_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DCOMPOSITION_OPACITY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_OPACITY_MODE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct COMPOSITION_FRAME_STATS {
    pub startTime: u64,
    pub targetTime: u64,
    pub framePeriod: u64,
}
impl ::core::marker::Copy for COMPOSITION_FRAME_STATS {}
impl ::core::clone::Clone for COMPOSITION_FRAME_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPOSITION_FRAME_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITION_FRAME_STATS").field("startTime", &self.startTime).field("targetTime", &self.targetTime).field("framePeriod", &self.framePeriod).finish()
    }
}
impl ::windows_core::TypeKind for COMPOSITION_FRAME_STATS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for COMPOSITION_FRAME_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.startTime == other.startTime && self.targetTime == other.targetTime && self.framePeriod == other.framePeriod
    }
}
impl ::core::cmp::Eq for COMPOSITION_FRAME_STATS {}
impl ::core::default::Default for COMPOSITION_FRAME_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct COMPOSITION_STATS {
    pub presentCount: u32,
    pub refreshCount: u32,
    pub virtualRefreshCount: u32,
    pub time: u64,
}
impl ::core::marker::Copy for COMPOSITION_STATS {}
impl ::core::clone::Clone for COMPOSITION_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPOSITION_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITION_STATS").field("presentCount", &self.presentCount).field("refreshCount", &self.refreshCount).field("virtualRefreshCount", &self.virtualRefreshCount).field("time", &self.time).finish()
    }
}
impl ::windows_core::TypeKind for COMPOSITION_STATS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for COMPOSITION_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.presentCount == other.presentCount && self.refreshCount == other.refreshCount && self.virtualRefreshCount == other.virtualRefreshCount && self.time == other.time
    }
}
impl ::core::cmp::Eq for COMPOSITION_STATS {}
impl ::core::default::Default for COMPOSITION_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct COMPOSITION_TARGET_ID {
    pub displayAdapterLuid: super::super::Foundation::LUID,
    pub renderAdapterLuid: super::super::Foundation::LUID,
    pub vidPnSourceId: u32,
    pub vidPnTargetId: u32,
    pub uniqueId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMPOSITION_TARGET_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMPOSITION_TARGET_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMPOSITION_TARGET_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITION_TARGET_ID").field("displayAdapterLuid", &self.displayAdapterLuid).field("renderAdapterLuid", &self.renderAdapterLuid).field("vidPnSourceId", &self.vidPnSourceId).field("vidPnTargetId", &self.vidPnTargetId).field("uniqueId", &self.uniqueId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for COMPOSITION_TARGET_ID {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMPOSITION_TARGET_ID {
    fn eq(&self, other: &Self) -> bool {
        self.displayAdapterLuid == other.displayAdapterLuid && self.renderAdapterLuid == other.renderAdapterLuid && self.vidPnSourceId == other.vidPnSourceId && self.vidPnTargetId == other.vidPnTargetId && self.uniqueId == other.uniqueId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMPOSITION_TARGET_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMPOSITION_TARGET_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct COMPOSITION_TARGET_STATS {
    pub outstandingPresents: u32,
    pub presentTime: u64,
    pub vblankDuration: u64,
    pub presentedStats: COMPOSITION_STATS,
    pub completedStats: COMPOSITION_STATS,
}
impl ::core::marker::Copy for COMPOSITION_TARGET_STATS {}
impl ::core::clone::Clone for COMPOSITION_TARGET_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPOSITION_TARGET_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITION_TARGET_STATS").field("outstandingPresents", &self.outstandingPresents).field("presentTime", &self.presentTime).field("vblankDuration", &self.vblankDuration).field("presentedStats", &self.presentedStats).field("completedStats", &self.completedStats).finish()
    }
}
impl ::windows_core::TypeKind for COMPOSITION_TARGET_STATS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for COMPOSITION_TARGET_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.outstandingPresents == other.outstandingPresents && self.presentTime == other.presentTime && self.vblankDuration == other.vblankDuration && self.presentedStats == other.presentedStats && self.completedStats == other.completedStats
    }
}
impl ::core::cmp::Eq for COMPOSITION_TARGET_STATS {}
impl ::core::default::Default for COMPOSITION_TARGET_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct DCOMPOSITION_FRAME_STATISTICS {
    pub lastFrameTime: i64,
    pub currentCompositionRate: super::Dxgi::Common::DXGI_RATIONAL,
    pub currentTime: i64,
    pub timeFrequency: i64,
    pub nextEstimatedFrameTime: i64,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for DCOMPOSITION_FRAME_STATISTICS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for DCOMPOSITION_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for DCOMPOSITION_FRAME_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCOMPOSITION_FRAME_STATISTICS").field("lastFrameTime", &self.lastFrameTime).field("currentCompositionRate", &self.currentCompositionRate).field("currentTime", &self.currentTime).field("timeFrequency", &self.timeFrequency).field("nextEstimatedFrameTime", &self.nextEstimatedFrameTime).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::TypeKind for DCOMPOSITION_FRAME_STATISTICS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for DCOMPOSITION_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.lastFrameTime == other.lastFrameTime && self.currentCompositionRate == other.currentCompositionRate && self.currentTime == other.currentTime && self.timeFrequency == other.timeFrequency && self.nextEstimatedFrameTime == other.nextEstimatedFrameTime
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for DCOMPOSITION_FRAME_STATISTICS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for DCOMPOSITION_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DCompositionInkTrailPoint {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
impl ::core::marker::Copy for DCompositionInkTrailPoint {}
impl ::core::clone::Clone for DCompositionInkTrailPoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCompositionInkTrailPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCompositionInkTrailPoint").field("x", &self.x).field("y", &self.y).field("radius", &self.radius).finish()
    }
}
impl ::windows_core::TypeKind for DCompositionInkTrailPoint {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DCompositionInkTrailPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.radius == other.radius
    }
}
impl ::core::cmp::Eq for DCompositionInkTrailPoint {}
impl ::core::default::Default for DCompositionInkTrailPoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
