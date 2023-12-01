#[inline]
pub unsafe fn DWriteCreateFactory<T>(factorytype: DWRITE_FACTORY_TYPE) -> ::windows_core::Result<T>
where
    T: ::windows_core::ComInterface,
{
    ::windows_targets::link!("dwrite.dll" "system" fn DWriteCreateFactory(factorytype : DWRITE_FACTORY_TYPE, iid : *const ::windows_core::GUID, factory : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::ptr::null_mut();
    DWriteCreateFactory(factorytype, &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteAsyncResult(::windows_core::IUnknown);
impl IDWriteAsyncResult {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWaitHandle(&self) -> super::super::Foundation::HANDLE {
        ::windows_core::vcall!(self.GetWaitHandle())
    }
    pub unsafe fn GetResult(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetResult()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteAsyncResult, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteAsyncResult {}
unsafe impl ::core::marker::Sync for IDWriteAsyncResult {}
unsafe impl ::windows_core::Interface for IDWriteAsyncResult {
    type Vtable = IDWriteAsyncResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteAsyncResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xce25f8fd_863b_4d13_9651_c1f88dc73fe2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteAsyncResult_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetWaitHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetWaitHandle: usize,
    pub GetResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteBitmapRenderTarget(::windows_core::IUnknown);
impl IDWriteBitmapRenderTarget {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DrawGlyphRun<P0, P1>(&self, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: P0, textcolor: P1, blackboxrect: ::core::option::Option<*mut super::super::Foundation::RECT>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteRenderingParams>,
        P1: ::windows_core::IntoParam<super::super::Foundation::COLORREF>,
    {
        ::windows_core::vcall!(self.DrawGlyphRun(baselineoriginx, baselineoriginy, measuringmode, glyphrun, renderingparams.into_param().abi(), textcolor.into_param().abi(), ::core::mem::transmute(blackboxrect.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetMemoryDC(&self) -> super::Gdi::HDC {
        ::windows_core::vcall!(self.GetMemoryDC())
    }
    pub unsafe fn GetPixelsPerDip(&self) -> f32 {
        ::windows_core::vcall!(self.GetPixelsPerDip())
    }
    pub unsafe fn SetPixelsPerDip(&self, pixelsperdip: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetPixelsPerDip(pixelsperdip)).ok()
    }
    pub unsafe fn GetCurrentTransform(&self, transform: *mut DWRITE_MATRIX) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetCurrentTransform(transform)).ok()
    }
    pub unsafe fn SetCurrentTransform(&self, transform: ::core::option::Option<*const DWRITE_MATRIX>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCurrentTransform(::core::mem::transmute(transform.unwrap_or(::std::ptr::null())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSize(&self) -> ::windows_core::Result<super::super::Foundation::SIZE> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetSize(&mut result__)).from_abi(result__)
    }
    pub unsafe fn Resize(&self, width: u32, height: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Resize(width, height)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteBitmapRenderTarget, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteBitmapRenderTarget {}
unsafe impl ::core::marker::Sync for IDWriteBitmapRenderTarget {}
unsafe impl ::windows_core::Interface for IDWriteBitmapRenderTarget {
    type Vtable = IDWriteBitmapRenderTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteBitmapRenderTarget {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e5a32a3_8dff_4773_9ff6_0696eab77267);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteBitmapRenderTarget_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DrawGlyphRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: *mut ::core::ffi::c_void, textcolor: super::super::Foundation::COLORREF, blackboxrect: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DrawGlyphRun: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetMemoryDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::Gdi::HDC,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetMemoryDC: usize,
    pub GetPixelsPerDip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> f32,
    pub SetPixelsPerDip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelsperdip: f32) -> ::windows_core::HRESULT,
    pub GetCurrentTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> ::windows_core::HRESULT,
    pub SetCurrentTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *const DWRITE_MATRIX) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSize: usize,
    pub Resize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteBitmapRenderTarget1(::windows_core::IUnknown);
impl IDWriteBitmapRenderTarget1 {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DrawGlyphRun<P0, P1>(&self, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: P0, textcolor: P1, blackboxrect: ::core::option::Option<*mut super::super::Foundation::RECT>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteRenderingParams>,
        P1: ::windows_core::IntoParam<super::super::Foundation::COLORREF>,
    {
        ::windows_core::vcall!(self.base__.DrawGlyphRun(baselineoriginx, baselineoriginy, measuringmode, glyphrun, renderingparams.into_param().abi(), textcolor.into_param().abi(), ::core::mem::transmute(blackboxrect.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetMemoryDC(&self) -> super::Gdi::HDC {
        ::windows_core::vcall!(self.base__.GetMemoryDC())
    }
    pub unsafe fn GetPixelsPerDip(&self) -> f32 {
        ::windows_core::vcall!(self.base__.GetPixelsPerDip())
    }
    pub unsafe fn SetPixelsPerDip(&self, pixelsperdip: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetPixelsPerDip(pixelsperdip)).ok()
    }
    pub unsafe fn GetCurrentTransform(&self, transform: *mut DWRITE_MATRIX) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetCurrentTransform(transform)).ok()
    }
    pub unsafe fn SetCurrentTransform(&self, transform: ::core::option::Option<*const DWRITE_MATRIX>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetCurrentTransform(::core::mem::transmute(transform.unwrap_or(::std::ptr::null())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSize(&self) -> ::windows_core::Result<super::super::Foundation::SIZE> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetSize(&mut result__)).from_abi(result__)
    }
    pub unsafe fn Resize(&self, width: u32, height: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.Resize(width, height)).ok()
    }
    pub unsafe fn GetTextAntialiasMode(&self) -> DWRITE_TEXT_ANTIALIAS_MODE {
        ::windows_core::vcall!(self.GetTextAntialiasMode())
    }
    pub unsafe fn SetTextAntialiasMode(&self, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetTextAntialiasMode(antialiasmode)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteBitmapRenderTarget1, ::windows_core::IUnknown, IDWriteBitmapRenderTarget);
unsafe impl ::core::marker::Send for IDWriteBitmapRenderTarget1 {}
unsafe impl ::core::marker::Sync for IDWriteBitmapRenderTarget1 {}
unsafe impl ::windows_core::Interface for IDWriteBitmapRenderTarget1 {
    type Vtable = IDWriteBitmapRenderTarget1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteBitmapRenderTarget1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x791e8298_3ef3_4230_9880_c9bdecc42064);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteBitmapRenderTarget1_Vtbl {
    pub base__: IDWriteBitmapRenderTarget_Vtbl,
    pub GetTextAntialiasMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_TEXT_ANTIALIAS_MODE,
    pub SetTextAntialiasMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteColorGlyphRunEnumerator(::windows_core::IUnknown);
impl IDWriteColorGlyphRunEnumerator {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.MoveNext(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCurrentRun(&self) -> ::windows_core::Result<*mut DWRITE_COLOR_GLYPH_RUN> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetCurrentRun(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteColorGlyphRunEnumerator, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteColorGlyphRunEnumerator {}
unsafe impl ::core::marker::Sync for IDWriteColorGlyphRunEnumerator {}
unsafe impl ::windows_core::Interface for IDWriteColorGlyphRunEnumerator {
    type Vtable = IDWriteColorGlyphRunEnumerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteColorGlyphRunEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd31fbe17_f157_41a2_8d24_cb779e0560e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteColorGlyphRunEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasrun: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCurrentRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorglyphrun: *mut *mut DWRITE_COLOR_GLYPH_RUN) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCurrentRun: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteColorGlyphRunEnumerator1(::windows_core::IUnknown);
impl IDWriteColorGlyphRunEnumerator1 {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.MoveNext(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCurrentRun(&self) -> ::windows_core::Result<*mut DWRITE_COLOR_GLYPH_RUN> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetCurrentRun(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCurrentRun2(&self) -> ::windows_core::Result<*mut DWRITE_COLOR_GLYPH_RUN1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetCurrentRun2(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteColorGlyphRunEnumerator1, ::windows_core::IUnknown, IDWriteColorGlyphRunEnumerator);
unsafe impl ::core::marker::Send for IDWriteColorGlyphRunEnumerator1 {}
unsafe impl ::core::marker::Sync for IDWriteColorGlyphRunEnumerator1 {}
unsafe impl ::windows_core::Interface for IDWriteColorGlyphRunEnumerator1 {
    type Vtable = IDWriteColorGlyphRunEnumerator1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteColorGlyphRunEnumerator1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c5f86da_c7a1_4f05_b8e1_55a179fe5a35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteColorGlyphRunEnumerator1_Vtbl {
    pub base__: IDWriteColorGlyphRunEnumerator_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCurrentRun2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorglyphrun: *mut *mut DWRITE_COLOR_GLYPH_RUN1) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCurrentRun2: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFactory(::windows_core::IUnknown);
impl IDWriteFactory {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.GetSystemFontCollection(::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn CreateCustomFontCollection<P0>(&self, collectionloader: P0, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows_core::Result<IDWriteFontCollection>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateCustomFontCollection(collectionloader.into_param().abi(), collectionkey, collectionkeysize, &mut result__)).from_abi(result__)
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        ::windows_core::vcall!(self.RegisterFontCollectionLoader(fontcollectionloader.into_param().abi())).ok()
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        ::windows_core::vcall!(self.UnregisterFontCollectionLoader(fontcollectionloader.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFileReference<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFileReference(filepath.into_param().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomFontFileReference<P0>(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: P0) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateCustomFontFileReference(fontfilereferencekey, fontfilereferencekeysize, fontfileloader.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, fontfiles: &[::core::option::Option<IDWriteFontFile>], faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFace> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFace(fontfacetype, fontfiles.len().try_into().unwrap(), ::core::mem::transmute(fontfiles.as_ptr()), faceindex, fontfacesimulationflags, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRenderingParams(&self) -> ::windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateRenderingParams(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateMonitorRenderingParams<P0>(&self, monitor: P0) -> ::windows_core::Result<IDWriteRenderingParams>
    where
        P0: ::windows_core::IntoParam<super::Gdi::HMONITOR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateMonitorRenderingParams(monitor.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateCustomRenderingParams(gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        ::windows_core::vcall!(self.RegisterFontFileLoader(fontfileloader.into_param().abi())).ok()
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        ::windows_core::vcall!(self.UnregisterFontFileLoader(fontfileloader.into_param().abi())).ok()
    }
    pub unsafe fn CreateTextFormat<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P2) -> ::windows_core::Result<IDWriteTextFormat>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontCollection>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTextFormat(fontfamilyname.into_param().abi(), fontcollection.into_param().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTypography(&self) -> ::windows_core::Result<IDWriteTypography> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTypography(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGdiInterop(&self) -> ::windows_core::Result<IDWriteGdiInterop> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetGdiInterop(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextLayout<P0>(&self, string: &[u16], textformat: P0, maxwidth: f32, maxheight: f32) -> ::windows_core::Result<IDWriteTextLayout>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTextLayout(::core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.into_param().abi(), maxwidth, maxheight, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGdiCompatibleTextLayout<P0, P1>(&self, string: &[u16], textformat: P0, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P1) -> ::windows_core::Result<IDWriteTextLayout>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateGdiCompatibleTextLayout(::core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.into_param().abi(), layoutwidth, layoutheight, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> ::windows_core::Result<IDWriteInlineObject>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateEllipsisTrimmingSign(textformat.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> ::windows_core::Result<IDWriteTextAnalyzer> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTextAnalyzer(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNumberSubstitution<P0, P1>(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: P0, ignoreuseroverride: P1) -> ::windows_core::Result<IDWriteNumberSubstitution>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateNumberSubstitution(substitutionmethod, localename.into_param().abi(), ignoreuseroverride.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateGlyphRunAnalysis(glyphrun, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFactory, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFactory {}
unsafe impl ::core::marker::Sync for IDWriteFactory {}
unsafe impl ::windows_core::Interface for IDWriteFactory {
    type Vtable = IDWriteFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb859ee5a_d838_4b5b_a2e8_1adc7d93db48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSystemFontCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontcollection: *mut *mut ::core::ffi::c_void, checkforupdates: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSystemFontCollection: usize,
    pub CreateCustomFontCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collectionloader: *mut ::core::ffi::c_void, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RegisterFontCollectionLoader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontcollectionloader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UnregisterFontCollectionLoader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontcollectionloader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateFontFileReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::windows_core::PCWSTR, lastwritetime: *const super::super::Foundation::FILETIME, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateFontFileReference: usize,
    pub CreateCustomFontFileReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: *mut ::core::ffi::c_void, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFontFace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfacetype: DWRITE_FONT_FACE_TYPE, numberoffiles: u32, fontfiles: *const *mut ::core::ffi::c_void, faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateRenderingParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateMonitorRenderingParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, monitor: super::Gdi::HMONITOR, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateMonitorRenderingParams: usize,
    pub CreateCustomRenderingParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RegisterFontFileLoader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfileloader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub UnregisterFontFileLoader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfileloader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateTextFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfamilyname: ::windows_core::PCWSTR, fontcollection: *mut ::core::ffi::c_void, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: ::windows_core::PCWSTR, textformat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateTypography: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typography: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetGdiInterop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gdiinterop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateTextLayout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, string: ::windows_core::PCWSTR, stringlength: u32, textformat: *mut ::core::ffi::c_void, maxwidth: f32, maxheight: f32, textlayout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateGdiCompatibleTextLayout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, string: ::windows_core::PCWSTR, stringlength: u32, textformat: *mut ::core::ffi::c_void, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, textlayout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateGdiCompatibleTextLayout: usize,
    pub CreateEllipsisTrimmingSign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textformat: *mut ::core::ffi::c_void, trimmingsign: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateTextAnalyzer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textanalyzer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateNumberSubstitution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: ::windows_core::PCWSTR, ignoreuseroverride: super::super::Foundation::BOOL, numbersubstitution: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateNumberSubstitution: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateGlyphRunAnalysis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateGlyphRunAnalysis: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFactory1(::windows_core::IUnknown);
impl IDWriteFactory1 {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.GetSystemFontCollection(::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn CreateCustomFontCollection<P0>(&self, collectionloader: P0, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows_core::Result<IDWriteFontCollection>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateCustomFontCollection(collectionloader.into_param().abi(), collectionkey, collectionkeysize, &mut result__)).from_abi(result__)
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        ::windows_core::vcall!(self.base__.RegisterFontCollectionLoader(fontcollectionloader.into_param().abi())).ok()
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        ::windows_core::vcall!(self.base__.UnregisterFontCollectionLoader(fontcollectionloader.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFileReference<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontFileReference(filepath.into_param().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomFontFileReference<P0>(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: P0) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateCustomFontFileReference(fontfilereferencekey, fontfilereferencekeysize, fontfileloader.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, fontfiles: &[::core::option::Option<IDWriteFontFile>], faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFace> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontFace(fontfacetype, fontfiles.len().try_into().unwrap(), ::core::mem::transmute(fontfiles.as_ptr()), faceindex, fontfacesimulationflags, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRenderingParams(&self) -> ::windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateRenderingParams(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateMonitorRenderingParams<P0>(&self, monitor: P0) -> ::windows_core::Result<IDWriteRenderingParams>
    where
        P0: ::windows_core::IntoParam<super::Gdi::HMONITOR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateMonitorRenderingParams(monitor.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateCustomRenderingParams(gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        ::windows_core::vcall!(self.base__.RegisterFontFileLoader(fontfileloader.into_param().abi())).ok()
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        ::windows_core::vcall!(self.base__.UnregisterFontFileLoader(fontfileloader.into_param().abi())).ok()
    }
    pub unsafe fn CreateTextFormat<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P2) -> ::windows_core::Result<IDWriteTextFormat>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontCollection>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateTextFormat(fontfamilyname.into_param().abi(), fontcollection.into_param().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTypography(&self) -> ::windows_core::Result<IDWriteTypography> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateTypography(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGdiInterop(&self) -> ::windows_core::Result<IDWriteGdiInterop> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetGdiInterop(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextLayout<P0>(&self, string: &[u16], textformat: P0, maxwidth: f32, maxheight: f32) -> ::windows_core::Result<IDWriteTextLayout>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateTextLayout(::core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.into_param().abi(), maxwidth, maxheight, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGdiCompatibleTextLayout<P0, P1>(&self, string: &[u16], textformat: P0, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P1) -> ::windows_core::Result<IDWriteTextLayout>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateGdiCompatibleTextLayout(::core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.into_param().abi(), layoutwidth, layoutheight, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> ::windows_core::Result<IDWriteInlineObject>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateEllipsisTrimmingSign(textformat.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> ::windows_core::Result<IDWriteTextAnalyzer> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateTextAnalyzer(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNumberSubstitution<P0, P1>(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: P0, ignoreuseroverride: P1) -> ::windows_core::Result<IDWriteNumberSubstitution>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateNumberSubstitution(substitutionmethod, localename.into_param().abi(), ignoreuseroverride.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateGlyphRunAnalysis(glyphrun, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEudcFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.GetEudcFontCollection(::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn CreateCustomRenderingParams2(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows_core::Result<IDWriteRenderingParams1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateCustomRenderingParams2(gamma, enhancedcontrast, enhancedcontrastgrayscale, cleartypelevel, pixelgeometry, renderingmode, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFactory1, ::windows_core::IUnknown, IDWriteFactory);
unsafe impl ::core::marker::Send for IDWriteFactory1 {}
unsafe impl ::core::marker::Sync for IDWriteFactory1 {}
unsafe impl ::windows_core::Interface for IDWriteFactory1 {
    type Vtable = IDWriteFactory1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFactory1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x30572f99_dac6_41db_a16e_0486307e606a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory1_Vtbl {
    pub base__: IDWriteFactory_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEudcFontCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontcollection: *mut *mut ::core::ffi::c_void, checkforupdates: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEudcFontCollection: usize,
    pub CreateCustomRenderingParams2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFactory2(::windows_core::IUnknown);
impl IDWriteFactory2 {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.GetSystemFontCollection(::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn CreateCustomFontCollection<P0>(&self, collectionloader: P0, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows_core::Result<IDWriteFontCollection>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateCustomFontCollection(collectionloader.into_param().abi(), collectionkey, collectionkeysize, &mut result__)).from_abi(result__)
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.RegisterFontCollectionLoader(fontcollectionloader.into_param().abi())).ok()
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.UnregisterFontCollectionLoader(fontcollectionloader.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFileReference<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateFontFileReference(filepath.into_param().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomFontFileReference<P0>(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: P0) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateCustomFontFileReference(fontfilereferencekey, fontfilereferencekeysize, fontfileloader.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, fontfiles: &[::core::option::Option<IDWriteFontFile>], faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFace> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateFontFace(fontfacetype, fontfiles.len().try_into().unwrap(), ::core::mem::transmute(fontfiles.as_ptr()), faceindex, fontfacesimulationflags, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRenderingParams(&self) -> ::windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateRenderingParams(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateMonitorRenderingParams<P0>(&self, monitor: P0) -> ::windows_core::Result<IDWriteRenderingParams>
    where
        P0: ::windows_core::IntoParam<super::Gdi::HMONITOR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateMonitorRenderingParams(monitor.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateCustomRenderingParams(gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.RegisterFontFileLoader(fontfileloader.into_param().abi())).ok()
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.UnregisterFontFileLoader(fontfileloader.into_param().abi())).ok()
    }
    pub unsafe fn CreateTextFormat<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P2) -> ::windows_core::Result<IDWriteTextFormat>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontCollection>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateTextFormat(fontfamilyname.into_param().abi(), fontcollection.into_param().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTypography(&self) -> ::windows_core::Result<IDWriteTypography> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateTypography(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGdiInterop(&self) -> ::windows_core::Result<IDWriteGdiInterop> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetGdiInterop(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextLayout<P0>(&self, string: &[u16], textformat: P0, maxwidth: f32, maxheight: f32) -> ::windows_core::Result<IDWriteTextLayout>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateTextLayout(::core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.into_param().abi(), maxwidth, maxheight, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGdiCompatibleTextLayout<P0, P1>(&self, string: &[u16], textformat: P0, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P1) -> ::windows_core::Result<IDWriteTextLayout>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateGdiCompatibleTextLayout(::core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.into_param().abi(), layoutwidth, layoutheight, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> ::windows_core::Result<IDWriteInlineObject>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateEllipsisTrimmingSign(textformat.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> ::windows_core::Result<IDWriteTextAnalyzer> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateTextAnalyzer(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNumberSubstitution<P0, P1>(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: P0, ignoreuseroverride: P1) -> ::windows_core::Result<IDWriteNumberSubstitution>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateNumberSubstitution(substitutionmethod, localename.into_param().abi(), ignoreuseroverride.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateGlyphRunAnalysis(glyphrun, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEudcFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.GetEudcFontCollection(::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn CreateCustomRenderingParams2(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows_core::Result<IDWriteRenderingParams1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateCustomRenderingParams2(gamma, enhancedcontrast, enhancedcontrastgrayscale, cleartypelevel, pixelgeometry, renderingmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetSystemFontFallback(&self) -> ::windows_core::Result<IDWriteFontFallback> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetSystemFontFallback(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFallbackBuilder(&self) -> ::windows_core::Result<IDWriteFontFallbackBuilder> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFallbackBuilder(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateColorGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows_core::Result<IDWriteColorGlyphRunEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.TranslateColorGlyphRun(baselineoriginx, baselineoriginy, glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), measuringmode, ::core::mem::transmute(worldtodevicetransform.unwrap_or(::std::ptr::null())), colorpaletteindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams3(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<IDWriteRenderingParams2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateCustomRenderingParams3(gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis2(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateGlyphRunAnalysis2(glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFactory2, ::windows_core::IUnknown, IDWriteFactory, IDWriteFactory1);
unsafe impl ::core::marker::Send for IDWriteFactory2 {}
unsafe impl ::core::marker::Sync for IDWriteFactory2 {}
unsafe impl ::windows_core::Interface for IDWriteFactory2 {
    type Vtable = IDWriteFactory2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFactory2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0439fc60_ca44_4994_8dee_3a9af7b732ec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory2_Vtbl {
    pub base__: IDWriteFactory1_Vtbl,
    pub GetSystemFontFallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfallback: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFontFallbackBuilder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfallbackbuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TranslateColorGlyphRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: *const DWRITE_MATRIX, colorpaletteindex: u32, colorlayers: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TranslateColorGlyphRun: usize,
    pub CreateCustomRenderingParams3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateGlyphRunAnalysis2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateGlyphRunAnalysis2: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFactory3(::windows_core::IUnknown);
impl IDWriteFactory3 {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.GetSystemFontCollection(::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn CreateCustomFontCollection<P0>(&self, collectionloader: P0, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows_core::Result<IDWriteFontCollection>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateCustomFontCollection(collectionloader.into_param().abi(), collectionkey, collectionkeysize, &mut result__)).from_abi(result__)
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.RegisterFontCollectionLoader(fontcollectionloader.into_param().abi())).ok()
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.UnregisterFontCollectionLoader(fontcollectionloader.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFileReference<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateFontFileReference(filepath.into_param().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomFontFileReference<P0>(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: P0) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateCustomFontFileReference(fontfilereferencekey, fontfilereferencekeysize, fontfileloader.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, fontfiles: &[::core::option::Option<IDWriteFontFile>], faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFace> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateFontFace(fontfacetype, fontfiles.len().try_into().unwrap(), ::core::mem::transmute(fontfiles.as_ptr()), faceindex, fontfacesimulationflags, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRenderingParams(&self) -> ::windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateRenderingParams(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateMonitorRenderingParams<P0>(&self, monitor: P0) -> ::windows_core::Result<IDWriteRenderingParams>
    where
        P0: ::windows_core::IntoParam<super::Gdi::HMONITOR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateMonitorRenderingParams(monitor.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateCustomRenderingParams(gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.RegisterFontFileLoader(fontfileloader.into_param().abi())).ok()
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.UnregisterFontFileLoader(fontfileloader.into_param().abi())).ok()
    }
    pub unsafe fn CreateTextFormat<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P2) -> ::windows_core::Result<IDWriteTextFormat>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontCollection>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateTextFormat(fontfamilyname.into_param().abi(), fontcollection.into_param().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTypography(&self) -> ::windows_core::Result<IDWriteTypography> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateTypography(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGdiInterop(&self) -> ::windows_core::Result<IDWriteGdiInterop> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetGdiInterop(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextLayout<P0>(&self, string: &[u16], textformat: P0, maxwidth: f32, maxheight: f32) -> ::windows_core::Result<IDWriteTextLayout>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateTextLayout(::core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.into_param().abi(), maxwidth, maxheight, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGdiCompatibleTextLayout<P0, P1>(&self, string: &[u16], textformat: P0, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P1) -> ::windows_core::Result<IDWriteTextLayout>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateGdiCompatibleTextLayout(::core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.into_param().abi(), layoutwidth, layoutheight, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> ::windows_core::Result<IDWriteInlineObject>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateEllipsisTrimmingSign(textformat.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> ::windows_core::Result<IDWriteTextAnalyzer> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateTextAnalyzer(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNumberSubstitution<P0, P1>(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: P0, ignoreuseroverride: P1) -> ::windows_core::Result<IDWriteNumberSubstitution>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateNumberSubstitution(substitutionmethod, localename.into_param().abi(), ignoreuseroverride.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateGlyphRunAnalysis(glyphrun, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEudcFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.GetEudcFontCollection(::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn CreateCustomRenderingParams2(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows_core::Result<IDWriteRenderingParams1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateCustomRenderingParams2(gamma, enhancedcontrast, enhancedcontrastgrayscale, cleartypelevel, pixelgeometry, renderingmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetSystemFontFallback(&self) -> ::windows_core::Result<IDWriteFontFallback> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetSystemFontFallback(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFallbackBuilder(&self) -> ::windows_core::Result<IDWriteFontFallbackBuilder> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontFallbackBuilder(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateColorGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows_core::Result<IDWriteColorGlyphRunEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.TranslateColorGlyphRun(baselineoriginx, baselineoriginy, glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), measuringmode, ::core::mem::transmute(worldtodevicetransform.unwrap_or(::std::ptr::null())), colorpaletteindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams3(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<IDWriteRenderingParams2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateCustomRenderingParams3(gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis2(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateGlyphRunAnalysis2(glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis3(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateGlyphRunAnalysis3(glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams4(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<IDWriteRenderingParams3> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateCustomRenderingParams4(gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFaceReference<P0>(&self, fontfile: P0, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFaceReference>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFile>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFaceReference(fontfile.into_param().abi(), faceindex, fontsimulations, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFaceReference2<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFaceReference>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFaceReference2(filepath.into_param().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), faceindex, fontsimulations, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetSystemFontSet(&self) -> ::windows_core::Result<IDWriteFontSet> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetSystemFontSet(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontSetBuilder(&self) -> ::windows_core::Result<IDWriteFontSetBuilder> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontSetBuilder(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontCollectionFromFontSet<P0>(&self, fontset: P0) -> ::windows_core::Result<IDWriteFontCollection1>
    where
        P0: ::windows_core::IntoParam<IDWriteFontSet>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontCollectionFromFontSet(fontset.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection2<P0, P1>(&self, includedownloadablefonts: P0, fontcollection: *mut ::core::option::Option<IDWriteFontCollection1>, checkforupdates: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.GetSystemFontCollection2(includedownloadablefonts.into_param().abi(), ::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn GetFontDownloadQueue(&self) -> ::windows_core::Result<IDWriteFontDownloadQueue> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontDownloadQueue(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFactory3, ::windows_core::IUnknown, IDWriteFactory, IDWriteFactory1, IDWriteFactory2);
unsafe impl ::core::marker::Send for IDWriteFactory3 {}
unsafe impl ::core::marker::Sync for IDWriteFactory3 {}
unsafe impl ::windows_core::Interface for IDWriteFactory3 {
    type Vtable = IDWriteFactory3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFactory3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a1b41c3_d3bb_466a_87fc_fe67556a3b65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory3_Vtbl {
    pub base__: IDWriteFactory2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateGlyphRunAnalysis3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateGlyphRunAnalysis3: usize,
    pub CreateCustomRenderingParams4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFontFaceReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfile: *mut ::core::ffi::c_void, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateFontFaceReference2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::windows_core::PCWSTR, lastwritetime: *const super::super::Foundation::FILETIME, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateFontFaceReference2: usize,
    pub GetSystemFontSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFontSetBuilder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontsetbuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFontCollectionFromFontSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontset: *mut ::core::ffi::c_void, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSystemFontCollection2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontcollection: *mut *mut ::core::ffi::c_void, checkforupdates: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSystemFontCollection2: usize,
    pub GetFontDownloadQueue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontdownloadqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFactory4(::windows_core::IUnknown);
impl IDWriteFactory4 {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetSystemFontCollection(::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn CreateCustomFontCollection<P0>(&self, collectionloader: P0, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows_core::Result<IDWriteFontCollection>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateCustomFontCollection(collectionloader.into_param().abi(), collectionkey, collectionkeysize, &mut result__)).from_abi(result__)
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.RegisterFontCollectionLoader(fontcollectionloader.into_param().abi())).ok()
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.UnregisterFontCollectionLoader(fontcollectionloader.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFileReference<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateFontFileReference(filepath.into_param().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomFontFileReference<P0>(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: P0) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateCustomFontFileReference(fontfilereferencekey, fontfilereferencekeysize, fontfileloader.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, fontfiles: &[::core::option::Option<IDWriteFontFile>], faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFace> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateFontFace(fontfacetype, fontfiles.len().try_into().unwrap(), ::core::mem::transmute(fontfiles.as_ptr()), faceindex, fontfacesimulationflags, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRenderingParams(&self) -> ::windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateRenderingParams(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateMonitorRenderingParams<P0>(&self, monitor: P0) -> ::windows_core::Result<IDWriteRenderingParams>
    where
        P0: ::windows_core::IntoParam<super::Gdi::HMONITOR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateMonitorRenderingParams(monitor.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateCustomRenderingParams(gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.RegisterFontFileLoader(fontfileloader.into_param().abi())).ok()
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.UnregisterFontFileLoader(fontfileloader.into_param().abi())).ok()
    }
    pub unsafe fn CreateTextFormat<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P2) -> ::windows_core::Result<IDWriteTextFormat>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontCollection>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateTextFormat(fontfamilyname.into_param().abi(), fontcollection.into_param().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTypography(&self) -> ::windows_core::Result<IDWriteTypography> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateTypography(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGdiInterop(&self) -> ::windows_core::Result<IDWriteGdiInterop> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetGdiInterop(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextLayout<P0>(&self, string: &[u16], textformat: P0, maxwidth: f32, maxheight: f32) -> ::windows_core::Result<IDWriteTextLayout>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateTextLayout(::core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.into_param().abi(), maxwidth, maxheight, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGdiCompatibleTextLayout<P0, P1>(&self, string: &[u16], textformat: P0, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P1) -> ::windows_core::Result<IDWriteTextLayout>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateGdiCompatibleTextLayout(::core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.into_param().abi(), layoutwidth, layoutheight, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> ::windows_core::Result<IDWriteInlineObject>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateEllipsisTrimmingSign(textformat.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> ::windows_core::Result<IDWriteTextAnalyzer> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateTextAnalyzer(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNumberSubstitution<P0, P1>(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: P0, ignoreuseroverride: P1) -> ::windows_core::Result<IDWriteNumberSubstitution>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateNumberSubstitution(substitutionmethod, localename.into_param().abi(), ignoreuseroverride.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateGlyphRunAnalysis(glyphrun, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEudcFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.GetEudcFontCollection(::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn CreateCustomRenderingParams2(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows_core::Result<IDWriteRenderingParams1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateCustomRenderingParams2(gamma, enhancedcontrast, enhancedcontrastgrayscale, cleartypelevel, pixelgeometry, renderingmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetSystemFontFallback(&self) -> ::windows_core::Result<IDWriteFontFallback> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetSystemFontFallback(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFallbackBuilder(&self) -> ::windows_core::Result<IDWriteFontFallbackBuilder> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateFontFallbackBuilder(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateColorGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows_core::Result<IDWriteColorGlyphRunEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.TranslateColorGlyphRun(baselineoriginx, baselineoriginy, glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), measuringmode, ::core::mem::transmute(worldtodevicetransform.unwrap_or(::std::ptr::null())), colorpaletteindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams3(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<IDWriteRenderingParams2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateCustomRenderingParams3(gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis2(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateGlyphRunAnalysis2(glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis3(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateGlyphRunAnalysis3(glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams4(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<IDWriteRenderingParams3> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateCustomRenderingParams4(gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFaceReference<P0>(&self, fontfile: P0, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFaceReference>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFile>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontFaceReference(fontfile.into_param().abi(), faceindex, fontsimulations, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFaceReference2<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFaceReference>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontFaceReference2(filepath.into_param().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), faceindex, fontsimulations, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetSystemFontSet(&self) -> ::windows_core::Result<IDWriteFontSet> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetSystemFontSet(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontSetBuilder(&self) -> ::windows_core::Result<IDWriteFontSetBuilder> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontSetBuilder(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontCollectionFromFontSet<P0>(&self, fontset: P0) -> ::windows_core::Result<IDWriteFontCollection1>
    where
        P0: ::windows_core::IntoParam<IDWriteFontSet>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontCollectionFromFontSet(fontset.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection2<P0, P1>(&self, includedownloadablefonts: P0, fontcollection: *mut ::core::option::Option<IDWriteFontCollection1>, checkforupdates: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.GetSystemFontCollection2(includedownloadablefonts.into_param().abi(), ::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn GetFontDownloadQueue(&self) -> ::windows_core::Result<IDWriteFontDownloadQueue> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontDownloadQueue(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn TranslateColorGlyphRun2(&self, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows_core::Result<IDWriteColorGlyphRunEnumerator1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.TranslateColorGlyphRun2(::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), desiredglyphimageformats, measuringmode, ::core::mem::transmute(worldanddpitransform.unwrap_or(::std::ptr::null())), colorpaletteindex, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputeGlyphOrigins(&self, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: super::Direct2D::Common::D2D_POINT_2F) -> ::windows_core::Result<super::Direct2D::Common::D2D_POINT_2F> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.ComputeGlyphOrigins(glyphrun, ::core::mem::transmute(baselineorigin), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputeGlyphOrigins2(&self, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: ::core::option::Option<*const DWRITE_MATRIX>) -> ::windows_core::Result<super::Direct2D::Common::D2D_POINT_2F> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.ComputeGlyphOrigins2(glyphrun, measuringmode, ::core::mem::transmute(baselineorigin), ::core::mem::transmute(worldanddpitransform.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFactory4, ::windows_core::IUnknown, IDWriteFactory, IDWriteFactory1, IDWriteFactory2, IDWriteFactory3);
unsafe impl ::core::marker::Send for IDWriteFactory4 {}
unsafe impl ::core::marker::Sync for IDWriteFactory4 {}
unsafe impl ::windows_core::Interface for IDWriteFactory4 {
    type Vtable = IDWriteFactory4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFactory4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b0b5bd3_0797_4549_8ac5_fe915cc53856);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory4_Vtbl {
    pub base__: IDWriteFactory3_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub TranslateColorGlyphRun2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: *const DWRITE_MATRIX, colorpaletteindex: u32, colorlayers: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    TranslateColorGlyphRun2: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub ComputeGlyphOrigins: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphorigins: *mut super::Direct2D::Common::D2D_POINT_2F) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    ComputeGlyphOrigins: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub ComputeGlyphOrigins2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: *const DWRITE_MATRIX, glyphorigins: *mut super::Direct2D::Common::D2D_POINT_2F) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    ComputeGlyphOrigins2: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFactory5(::windows_core::IUnknown);
impl IDWriteFactory5 {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetSystemFontCollection(::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn CreateCustomFontCollection<P0>(&self, collectionloader: P0, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows_core::Result<IDWriteFontCollection>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateCustomFontCollection(collectionloader.into_param().abi(), collectionkey, collectionkeysize, &mut result__)).from_abi(result__)
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.RegisterFontCollectionLoader(fontcollectionloader.into_param().abi())).ok()
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.UnregisterFontCollectionLoader(fontcollectionloader.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFileReference<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateFontFileReference(filepath.into_param().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomFontFileReference<P0>(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: P0) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateCustomFontFileReference(fontfilereferencekey, fontfilereferencekeysize, fontfileloader.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, fontfiles: &[::core::option::Option<IDWriteFontFile>], faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFace> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateFontFace(fontfacetype, fontfiles.len().try_into().unwrap(), ::core::mem::transmute(fontfiles.as_ptr()), faceindex, fontfacesimulationflags, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRenderingParams(&self) -> ::windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateRenderingParams(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateMonitorRenderingParams<P0>(&self, monitor: P0) -> ::windows_core::Result<IDWriteRenderingParams>
    where
        P0: ::windows_core::IntoParam<super::Gdi::HMONITOR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateMonitorRenderingParams(monitor.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateCustomRenderingParams(gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.RegisterFontFileLoader(fontfileloader.into_param().abi())).ok()
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.UnregisterFontFileLoader(fontfileloader.into_param().abi())).ok()
    }
    pub unsafe fn CreateTextFormat<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P2) -> ::windows_core::Result<IDWriteTextFormat>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontCollection>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateTextFormat(fontfamilyname.into_param().abi(), fontcollection.into_param().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTypography(&self) -> ::windows_core::Result<IDWriteTypography> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateTypography(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGdiInterop(&self) -> ::windows_core::Result<IDWriteGdiInterop> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetGdiInterop(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextLayout<P0>(&self, string: &[u16], textformat: P0, maxwidth: f32, maxheight: f32) -> ::windows_core::Result<IDWriteTextLayout>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateTextLayout(::core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.into_param().abi(), maxwidth, maxheight, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGdiCompatibleTextLayout<P0, P1>(&self, string: &[u16], textformat: P0, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P1) -> ::windows_core::Result<IDWriteTextLayout>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateGdiCompatibleTextLayout(::core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.into_param().abi(), layoutwidth, layoutheight, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> ::windows_core::Result<IDWriteInlineObject>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateEllipsisTrimmingSign(textformat.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> ::windows_core::Result<IDWriteTextAnalyzer> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateTextAnalyzer(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNumberSubstitution<P0, P1>(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: P0, ignoreuseroverride: P1) -> ::windows_core::Result<IDWriteNumberSubstitution>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateNumberSubstitution(substitutionmethod, localename.into_param().abi(), ignoreuseroverride.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateGlyphRunAnalysis(glyphrun, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEudcFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetEudcFontCollection(::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn CreateCustomRenderingParams2(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows_core::Result<IDWriteRenderingParams1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateCustomRenderingParams2(gamma, enhancedcontrast, enhancedcontrastgrayscale, cleartypelevel, pixelgeometry, renderingmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetSystemFontFallback(&self) -> ::windows_core::Result<IDWriteFontFallback> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetSystemFontFallback(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFallbackBuilder(&self) -> ::windows_core::Result<IDWriteFontFallbackBuilder> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateFontFallbackBuilder(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateColorGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows_core::Result<IDWriteColorGlyphRunEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.TranslateColorGlyphRun(baselineoriginx, baselineoriginy, glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), measuringmode, ::core::mem::transmute(worldtodevicetransform.unwrap_or(::std::ptr::null())), colorpaletteindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams3(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<IDWriteRenderingParams2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateCustomRenderingParams3(gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis2(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateGlyphRunAnalysis2(glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis3(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateGlyphRunAnalysis3(glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams4(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<IDWriteRenderingParams3> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateCustomRenderingParams4(gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFaceReference<P0>(&self, fontfile: P0, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFaceReference>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFile>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateFontFaceReference(fontfile.into_param().abi(), faceindex, fontsimulations, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFaceReference2<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFaceReference>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateFontFaceReference2(filepath.into_param().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), faceindex, fontsimulations, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetSystemFontSet(&self) -> ::windows_core::Result<IDWriteFontSet> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetSystemFontSet(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontSetBuilder(&self) -> ::windows_core::Result<IDWriteFontSetBuilder> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateFontSetBuilder(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontCollectionFromFontSet<P0>(&self, fontset: P0) -> ::windows_core::Result<IDWriteFontCollection1>
    where
        P0: ::windows_core::IntoParam<IDWriteFontSet>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateFontCollectionFromFontSet(fontset.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection2<P0, P1>(&self, includedownloadablefonts: P0, fontcollection: *mut ::core::option::Option<IDWriteFontCollection1>, checkforupdates: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.GetSystemFontCollection2(includedownloadablefonts.into_param().abi(), ::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn GetFontDownloadQueue(&self) -> ::windows_core::Result<IDWriteFontDownloadQueue> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFontDownloadQueue(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn TranslateColorGlyphRun2(&self, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows_core::Result<IDWriteColorGlyphRunEnumerator1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.TranslateColorGlyphRun2(::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), desiredglyphimageformats, measuringmode, ::core::mem::transmute(worldanddpitransform.unwrap_or(::std::ptr::null())), colorpaletteindex, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputeGlyphOrigins(&self, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: super::Direct2D::Common::D2D_POINT_2F) -> ::windows_core::Result<super::Direct2D::Common::D2D_POINT_2F> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.ComputeGlyphOrigins(glyphrun, ::core::mem::transmute(baselineorigin), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputeGlyphOrigins2(&self, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: ::core::option::Option<*const DWRITE_MATRIX>) -> ::windows_core::Result<super::Direct2D::Common::D2D_POINT_2F> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.ComputeGlyphOrigins2(glyphrun, measuringmode, ::core::mem::transmute(baselineorigin), ::core::mem::transmute(worldanddpitransform.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontSetBuilder2(&self) -> ::windows_core::Result<IDWriteFontSetBuilder1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontSetBuilder2(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateInMemoryFontFileLoader(&self) -> ::windows_core::Result<IDWriteInMemoryFontFileLoader> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateInMemoryFontFileLoader(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateHttpFontFileLoader<P0, P1>(&self, referrerurl: P0, extraheaders: P1) -> ::windows_core::Result<IDWriteRemoteFontFileLoader>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateHttpFontFileLoader(referrerurl.into_param().abi(), extraheaders.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn AnalyzeContainerType(&self, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE {
        ::windows_core::vcall!(self.AnalyzeContainerType(filedata, filedatasize))
    }
    pub unsafe fn UnpackFontFile(&self, containertype: DWRITE_CONTAINER_TYPE, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> ::windows_core::Result<IDWriteFontFileStream> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.UnpackFontFile(containertype, filedata, filedatasize, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFactory5, ::windows_core::IUnknown, IDWriteFactory, IDWriteFactory1, IDWriteFactory2, IDWriteFactory3, IDWriteFactory4);
unsafe impl ::core::marker::Send for IDWriteFactory5 {}
unsafe impl ::core::marker::Sync for IDWriteFactory5 {}
unsafe impl ::windows_core::Interface for IDWriteFactory5 {
    type Vtable = IDWriteFactory5_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFactory5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x958db99a_be2a_4f09_af7d_65189803d1d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory5_Vtbl {
    pub base__: IDWriteFactory4_Vtbl,
    pub CreateFontSetBuilder2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontsetbuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateInMemoryFontFileLoader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newloader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateHttpFontFileLoader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, referrerurl: ::windows_core::PCWSTR, extraheaders: ::windows_core::PCWSTR, newloader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AnalyzeContainerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE,
    pub UnpackFontFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, containertype: DWRITE_CONTAINER_TYPE, filedata: *const ::core::ffi::c_void, filedatasize: u32, unpackedfontstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFactory6(::windows_core::IUnknown);
impl IDWriteFactory6 {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.GetSystemFontCollection(::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn CreateCustomFontCollection<P0>(&self, collectionloader: P0, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows_core::Result<IDWriteFontCollection>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.CreateCustomFontCollection(collectionloader.into_param().abi(), collectionkey, collectionkeysize, &mut result__)).from_abi(result__)
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.RegisterFontCollectionLoader(fontcollectionloader.into_param().abi())).ok()
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.UnregisterFontCollectionLoader(fontcollectionloader.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFileReference<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.CreateFontFileReference(filepath.into_param().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomFontFileReference<P0>(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: P0) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.CreateCustomFontFileReference(fontfilereferencekey, fontfilereferencekeysize, fontfileloader.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, fontfiles: &[::core::option::Option<IDWriteFontFile>], faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFace> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.CreateFontFace(fontfacetype, fontfiles.len().try_into().unwrap(), ::core::mem::transmute(fontfiles.as_ptr()), faceindex, fontfacesimulationflags, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRenderingParams(&self) -> ::windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.CreateRenderingParams(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateMonitorRenderingParams<P0>(&self, monitor: P0) -> ::windows_core::Result<IDWriteRenderingParams>
    where
        P0: ::windows_core::IntoParam<super::Gdi::HMONITOR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.CreateMonitorRenderingParams(monitor.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.CreateCustomRenderingParams(gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.RegisterFontFileLoader(fontfileloader.into_param().abi())).ok()
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.UnregisterFontFileLoader(fontfileloader.into_param().abi())).ok()
    }
    pub unsafe fn CreateTextFormat<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P2) -> ::windows_core::Result<IDWriteTextFormat>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontCollection>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.CreateTextFormat(fontfamilyname.into_param().abi(), fontcollection.into_param().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTypography(&self) -> ::windows_core::Result<IDWriteTypography> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.CreateTypography(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGdiInterop(&self) -> ::windows_core::Result<IDWriteGdiInterop> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.GetGdiInterop(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextLayout<P0>(&self, string: &[u16], textformat: P0, maxwidth: f32, maxheight: f32) -> ::windows_core::Result<IDWriteTextLayout>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.CreateTextLayout(::core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.into_param().abi(), maxwidth, maxheight, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGdiCompatibleTextLayout<P0, P1>(&self, string: &[u16], textformat: P0, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P1) -> ::windows_core::Result<IDWriteTextLayout>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.CreateGdiCompatibleTextLayout(::core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.into_param().abi(), layoutwidth, layoutheight, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> ::windows_core::Result<IDWriteInlineObject>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.CreateEllipsisTrimmingSign(textformat.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> ::windows_core::Result<IDWriteTextAnalyzer> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.CreateTextAnalyzer(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNumberSubstitution<P0, P1>(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: P0, ignoreuseroverride: P1) -> ::windows_core::Result<IDWriteNumberSubstitution>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.CreateNumberSubstitution(substitutionmethod, localename.into_param().abi(), ignoreuseroverride.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.CreateGlyphRunAnalysis(glyphrun, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEudcFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetEudcFontCollection(::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn CreateCustomRenderingParams2(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows_core::Result<IDWriteRenderingParams1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateCustomRenderingParams2(gamma, enhancedcontrast, enhancedcontrastgrayscale, cleartypelevel, pixelgeometry, renderingmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetSystemFontFallback(&self) -> ::windows_core::Result<IDWriteFontFallback> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetSystemFontFallback(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFallbackBuilder(&self) -> ::windows_core::Result<IDWriteFontFallbackBuilder> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateFontFallbackBuilder(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateColorGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows_core::Result<IDWriteColorGlyphRunEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.TranslateColorGlyphRun(baselineoriginx, baselineoriginy, glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), measuringmode, ::core::mem::transmute(worldtodevicetransform.unwrap_or(::std::ptr::null())), colorpaletteindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams3(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<IDWriteRenderingParams2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateCustomRenderingParams3(gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis2(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateGlyphRunAnalysis2(glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis3(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateGlyphRunAnalysis3(glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams4(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<IDWriteRenderingParams3> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateCustomRenderingParams4(gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFaceReference<P0>(&self, fontfile: P0, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFaceReference>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFile>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateFontFaceReference(fontfile.into_param().abi(), faceindex, fontsimulations, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFaceReference2<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFaceReference>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateFontFaceReference2(filepath.into_param().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), faceindex, fontsimulations, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetSystemFontSet(&self) -> ::windows_core::Result<IDWriteFontSet> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetSystemFontSet(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontSetBuilder(&self) -> ::windows_core::Result<IDWriteFontSetBuilder> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateFontSetBuilder(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontCollectionFromFontSet<P0>(&self, fontset: P0) -> ::windows_core::Result<IDWriteFontCollection1>
    where
        P0: ::windows_core::IntoParam<IDWriteFontSet>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateFontCollectionFromFontSet(fontset.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection2<P0, P1>(&self, includedownloadablefonts: P0, fontcollection: *mut ::core::option::Option<IDWriteFontCollection1>, checkforupdates: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.GetSystemFontCollection2(includedownloadablefonts.into_param().abi(), ::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn GetFontDownloadQueue(&self) -> ::windows_core::Result<IDWriteFontDownloadQueue> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFontDownloadQueue(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn TranslateColorGlyphRun2(&self, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows_core::Result<IDWriteColorGlyphRunEnumerator1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.TranslateColorGlyphRun2(::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), desiredglyphimageformats, measuringmode, ::core::mem::transmute(worldanddpitransform.unwrap_or(::std::ptr::null())), colorpaletteindex, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputeGlyphOrigins(&self, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: super::Direct2D::Common::D2D_POINT_2F) -> ::windows_core::Result<super::Direct2D::Common::D2D_POINT_2F> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.ComputeGlyphOrigins(glyphrun, ::core::mem::transmute(baselineorigin), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputeGlyphOrigins2(&self, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: ::core::option::Option<*const DWRITE_MATRIX>) -> ::windows_core::Result<super::Direct2D::Common::D2D_POINT_2F> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.ComputeGlyphOrigins2(glyphrun, measuringmode, ::core::mem::transmute(baselineorigin), ::core::mem::transmute(worldanddpitransform.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontSetBuilder2(&self) -> ::windows_core::Result<IDWriteFontSetBuilder1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontSetBuilder2(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateInMemoryFontFileLoader(&self) -> ::windows_core::Result<IDWriteInMemoryFontFileLoader> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateInMemoryFontFileLoader(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateHttpFontFileLoader<P0, P1>(&self, referrerurl: P0, extraheaders: P1) -> ::windows_core::Result<IDWriteRemoteFontFileLoader>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateHttpFontFileLoader(referrerurl.into_param().abi(), extraheaders.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn AnalyzeContainerType(&self, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE {
        ::windows_core::vcall!(self.base__.AnalyzeContainerType(filedata, filedatasize))
    }
    pub unsafe fn UnpackFontFile(&self, containertype: DWRITE_CONTAINER_TYPE, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> ::windows_core::Result<IDWriteFontFileStream> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.UnpackFontFile(containertype, filedata, filedatasize, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFaceReference3<P0>(&self, fontfile: P0, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> ::windows_core::Result<IDWriteFontFaceReference1>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFile>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFaceReference3(fontfile.into_param().abi(), faceindex, fontsimulations, ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontResource<P0>(&self, fontfile: P0, faceindex: u32) -> ::windows_core::Result<IDWriteFontResource>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFile>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontResource(fontfile.into_param().abi(), faceindex, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontSet2<P0>(&self, includedownloadablefonts: P0) -> ::windows_core::Result<IDWriteFontSet1>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetSystemFontSet2(includedownloadablefonts.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection3<P0>(&self, includedownloadablefonts: P0, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows_core::Result<IDWriteFontCollection2>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetSystemFontCollection3(includedownloadablefonts.into_param().abi(), fontfamilymodel, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontCollectionFromFontSet2<P0>(&self, fontset: P0, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows_core::Result<IDWriteFontCollection2>
    where
        P0: ::windows_core::IntoParam<IDWriteFontSet>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontCollectionFromFontSet2(fontset.into_param().abi(), fontfamilymodel, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontSetBuilder3(&self) -> ::windows_core::Result<IDWriteFontSetBuilder2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontSetBuilder3(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextFormat2<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE], fontsize: f32, localename: P2) -> ::windows_core::Result<IDWriteTextFormat3>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontCollection>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateTextFormat2(fontfamilyname.into_param().abi(), fontcollection.into_param().abi(), ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), fontsize, localename.into_param().abi(), &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFactory6, ::windows_core::IUnknown, IDWriteFactory, IDWriteFactory1, IDWriteFactory2, IDWriteFactory3, IDWriteFactory4, IDWriteFactory5);
unsafe impl ::core::marker::Send for IDWriteFactory6 {}
unsafe impl ::core::marker::Sync for IDWriteFactory6 {}
unsafe impl ::windows_core::Interface for IDWriteFactory6 {
    type Vtable = IDWriteFactory6_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFactory6 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf3744d80_21f7_42eb_b35d_995bc72fc223);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory6_Vtbl {
    pub base__: IDWriteFactory5_Vtbl,
    pub CreateFontFaceReference3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfile: *mut ::core::ffi::c_void, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFontResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfile: *mut ::core::ffi::c_void, faceindex: u32, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSystemFontSet2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSystemFontSet2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSystemFontCollection3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSystemFontCollection3: usize,
    pub CreateFontCollectionFromFontSet2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontset: *mut ::core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFontSetBuilder3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontsetbuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateTextFormat2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfamilyname: ::windows_core::PCWSTR, fontcollection: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontsize: f32, localename: ::windows_core::PCWSTR, textformat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFactory7(::windows_core::IUnknown);
impl IDWriteFactory7 {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.GetSystemFontCollection(::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn CreateCustomFontCollection<P0>(&self, collectionloader: P0, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows_core::Result<IDWriteFontCollection>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.CreateCustomFontCollection(collectionloader.into_param().abi(), collectionkey, collectionkeysize, &mut result__)).from_abi(result__)
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.RegisterFontCollectionLoader(fontcollectionloader.into_param().abi())).ok()
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(&self, fontcollectionloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollectionLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.UnregisterFontCollectionLoader(fontcollectionloader.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFileReference<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.CreateFontFileReference(filepath.into_param().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomFontFileReference<P0>(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: P0) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.CreateCustomFontFileReference(fontfilereferencekey, fontfilereferencekeysize, fontfileloader.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, fontfacetype: DWRITE_FONT_FACE_TYPE, fontfiles: &[::core::option::Option<IDWriteFontFile>], faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFace> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.CreateFontFace(fontfacetype, fontfiles.len().try_into().unwrap(), ::core::mem::transmute(fontfiles.as_ptr()), faceindex, fontfacesimulationflags, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRenderingParams(&self) -> ::windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.CreateRenderingParams(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateMonitorRenderingParams<P0>(&self, monitor: P0) -> ::windows_core::Result<IDWriteRenderingParams>
    where
        P0: ::windows_core::IntoParam<super::Gdi::HMONITOR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.CreateMonitorRenderingParams(monitor.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams(&self, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows_core::Result<IDWriteRenderingParams> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.CreateCustomRenderingParams(gamma, enhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.RegisterFontFileLoader(fontfileloader.into_param().abi())).ok()
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFileLoader>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.UnregisterFontFileLoader(fontfileloader.into_param().abi())).ok()
    }
    pub unsafe fn CreateTextFormat<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P2) -> ::windows_core::Result<IDWriteTextFormat>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontCollection>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.CreateTextFormat(fontfamilyname.into_param().abi(), fontcollection.into_param().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTypography(&self) -> ::windows_core::Result<IDWriteTypography> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.CreateTypography(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGdiInterop(&self) -> ::windows_core::Result<IDWriteGdiInterop> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.GetGdiInterop(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextLayout<P0>(&self, string: &[u16], textformat: P0, maxwidth: f32, maxheight: f32) -> ::windows_core::Result<IDWriteTextLayout>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.CreateTextLayout(::core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.into_param().abi(), maxwidth, maxheight, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGdiCompatibleTextLayout<P0, P1>(&self, string: &[u16], textformat: P0, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P1) -> ::windows_core::Result<IDWriteTextLayout>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.CreateGdiCompatibleTextLayout(::core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.into_param().abi(), layoutwidth, layoutheight, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> ::windows_core::Result<IDWriteInlineObject>
    where
        P0: ::windows_core::IntoParam<IDWriteTextFormat>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.CreateEllipsisTrimmingSign(textformat.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> ::windows_core::Result<IDWriteTextAnalyzer> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.CreateTextAnalyzer(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNumberSubstitution<P0, P1>(&self, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: P0, ignoreuseroverride: P1) -> ::windows_core::Result<IDWriteNumberSubstitution>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.CreateNumberSubstitution(substitutionmethod, localename.into_param().abi(), ignoreuseroverride.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis(&self, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.base__.CreateGlyphRunAnalysis(glyphrun, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEudcFontCollection<P0>(&self, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.GetEudcFontCollection(::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn CreateCustomRenderingParams2(&self, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows_core::Result<IDWriteRenderingParams1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.CreateCustomRenderingParams2(gamma, enhancedcontrast, enhancedcontrastgrayscale, cleartypelevel, pixelgeometry, renderingmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetSystemFontFallback(&self) -> ::windows_core::Result<IDWriteFontFallback> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetSystemFontFallback(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFallbackBuilder(&self) -> ::windows_core::Result<IDWriteFontFallbackBuilder> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateFontFallbackBuilder(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TranslateColorGlyphRun(&self, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows_core::Result<IDWriteColorGlyphRunEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.TranslateColorGlyphRun(baselineoriginx, baselineoriginy, glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), measuringmode, ::core::mem::transmute(worldtodevicetransform.unwrap_or(::std::ptr::null())), colorpaletteindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams3(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<IDWriteRenderingParams2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateCustomRenderingParams3(gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis2(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.CreateGlyphRunAnalysis2(glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateGlyphRunAnalysis3(&self, glyphrun: *const DWRITE_GLYPH_RUN, transform: ::core::option::Option<*const DWRITE_MATRIX>, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateGlyphRunAnalysis3(glyphrun, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), renderingmode, measuringmode, gridfitmode, antialiasmode, baselineoriginx, baselineoriginy, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateCustomRenderingParams4(&self, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<IDWriteRenderingParams3> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateCustomRenderingParams4(gamma, enhancedcontrast, grayscaleenhancedcontrast, cleartypelevel, pixelgeometry, renderingmode, gridfitmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFaceReference<P0>(&self, fontfile: P0, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFaceReference>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFile>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateFontFaceReference(fontfile.into_param().abi(), faceindex, fontsimulations, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFontFaceReference2<P0>(&self, filepath: P0, lastwritetime: ::core::option::Option<*const super::super::Foundation::FILETIME>, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFaceReference>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateFontFaceReference2(filepath.into_param().abi(), ::core::mem::transmute(lastwritetime.unwrap_or(::std::ptr::null())), faceindex, fontsimulations, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetSystemFontSet(&self) -> ::windows_core::Result<IDWriteFontSet> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetSystemFontSet(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontSetBuilder(&self) -> ::windows_core::Result<IDWriteFontSetBuilder> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateFontSetBuilder(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontCollectionFromFontSet<P0>(&self, fontset: P0) -> ::windows_core::Result<IDWriteFontCollection1>
    where
        P0: ::windows_core::IntoParam<IDWriteFontSet>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.CreateFontCollectionFromFontSet(fontset.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection2<P0, P1>(&self, includedownloadablefonts: P0, fontcollection: *mut ::core::option::Option<IDWriteFontCollection1>, checkforupdates: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetSystemFontCollection2(includedownloadablefonts.into_param().abi(), ::core::mem::transmute(fontcollection), checkforupdates.into_param().abi())).ok()
    }
    pub unsafe fn GetFontDownloadQueue(&self) -> ::windows_core::Result<IDWriteFontDownloadQueue> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFontDownloadQueue(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn TranslateColorGlyphRun2(&self, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: ::core::option::Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: ::core::option::Option<*const DWRITE_MATRIX>, colorpaletteindex: u32) -> ::windows_core::Result<IDWriteColorGlyphRunEnumerator1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.TranslateColorGlyphRun2(::core::mem::transmute(baselineorigin), glyphrun, ::core::mem::transmute(glyphrundescription.unwrap_or(::std::ptr::null())), desiredglyphimageformats, measuringmode, ::core::mem::transmute(worldanddpitransform.unwrap_or(::std::ptr::null())), colorpaletteindex, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputeGlyphOrigins(&self, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: super::Direct2D::Common::D2D_POINT_2F) -> ::windows_core::Result<super::Direct2D::Common::D2D_POINT_2F> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.ComputeGlyphOrigins(glyphrun, ::core::mem::transmute(baselineorigin), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn ComputeGlyphOrigins2(&self, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: ::core::option::Option<*const DWRITE_MATRIX>) -> ::windows_core::Result<super::Direct2D::Common::D2D_POINT_2F> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.ComputeGlyphOrigins2(glyphrun, measuringmode, ::core::mem::transmute(baselineorigin), ::core::mem::transmute(worldanddpitransform.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontSetBuilder2(&self) -> ::windows_core::Result<IDWriteFontSetBuilder1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateFontSetBuilder2(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateInMemoryFontFileLoader(&self) -> ::windows_core::Result<IDWriteInMemoryFontFileLoader> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateInMemoryFontFileLoader(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateHttpFontFileLoader<P0, P1>(&self, referrerurl: P0, extraheaders: P1) -> ::windows_core::Result<IDWriteRemoteFontFileLoader>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateHttpFontFileLoader(referrerurl.into_param().abi(), extraheaders.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn AnalyzeContainerType(&self, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE {
        ::windows_core::vcall!(self.base__.base__.AnalyzeContainerType(filedata, filedatasize))
    }
    pub unsafe fn UnpackFontFile(&self, containertype: DWRITE_CONTAINER_TYPE, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> ::windows_core::Result<IDWriteFontFileStream> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.UnpackFontFile(containertype, filedata, filedatasize, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFaceReference3<P0>(&self, fontfile: P0, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> ::windows_core::Result<IDWriteFontFaceReference1>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFile>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontFaceReference3(fontfile.into_param().abi(), faceindex, fontsimulations, ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontResource<P0>(&self, fontfile: P0, faceindex: u32) -> ::windows_core::Result<IDWriteFontResource>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFile>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontResource(fontfile.into_param().abi(), faceindex, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontSet2<P0>(&self, includedownloadablefonts: P0) -> ::windows_core::Result<IDWriteFontSet1>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetSystemFontSet2(includedownloadablefonts.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection3<P0>(&self, includedownloadablefonts: P0, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows_core::Result<IDWriteFontCollection2>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetSystemFontCollection3(includedownloadablefonts.into_param().abi(), fontfamilymodel, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontCollectionFromFontSet2<P0>(&self, fontset: P0, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows_core::Result<IDWriteFontCollection2>
    where
        P0: ::windows_core::IntoParam<IDWriteFontSet>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontCollectionFromFontSet2(fontset.into_param().abi(), fontfamilymodel, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontSetBuilder3(&self) -> ::windows_core::Result<IDWriteFontSetBuilder2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontSetBuilder3(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateTextFormat2<P0, P1, P2>(&self, fontfamilyname: P0, fontcollection: P1, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE], fontsize: f32, localename: P2) -> ::windows_core::Result<IDWriteTextFormat3>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontCollection>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateTextFormat2(fontfamilyname.into_param().abi(), fontcollection.into_param().abi(), ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), fontsize, localename.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontSet3<P0>(&self, includedownloadablefonts: P0) -> ::windows_core::Result<IDWriteFontSet2>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetSystemFontSet3(includedownloadablefonts.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSystemFontCollection4<P0>(&self, includedownloadablefonts: P0, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows_core::Result<IDWriteFontCollection3>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetSystemFontCollection4(includedownloadablefonts.into_param().abi(), fontfamilymodel, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFactory7, ::windows_core::IUnknown, IDWriteFactory, IDWriteFactory1, IDWriteFactory2, IDWriteFactory3, IDWriteFactory4, IDWriteFactory5, IDWriteFactory6);
unsafe impl ::core::marker::Send for IDWriteFactory7 {}
unsafe impl ::core::marker::Sync for IDWriteFactory7 {}
unsafe impl ::windows_core::Interface for IDWriteFactory7 {
    type Vtable = IDWriteFactory7_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFactory7 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35d0e0b3_9076_4d2e_a016_a91b568a06b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory7_Vtbl {
    pub base__: IDWriteFactory6_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSystemFontSet3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSystemFontSet3: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSystemFontCollection4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSystemFontCollection4: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFont(::windows_core::IUnknown);
impl IDWriteFont {
    pub unsafe fn GetFontFamily(&self) -> ::windows_core::Result<IDWriteFontFamily> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontFamily(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        ::windows_core::vcall!(self.GetWeight())
    }
    pub unsafe fn GetStretch(&self) -> DWRITE_FONT_STRETCH {
        ::windows_core::vcall!(self.GetStretch())
    }
    pub unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        ::windows_core::vcall!(self.GetStyle())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.IsSymbolFont())
    }
    pub unsafe fn GetFaceNames(&self) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFaceNames(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetInformationalStrings(informationalstringid, ::core::mem::transmute(informationalstrings), exists)).ok()
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        ::windows_core::vcall!(self.GetSimulations())
    }
    pub unsafe fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS) {
        ::windows_core::vcall!(self.GetMetrics(fontmetrics))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.HasCharacter(unicodevalue, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self) -> ::windows_core::Result<IDWriteFontFace> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFace(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFont, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFont {}
unsafe impl ::core::marker::Sync for IDWriteFont {}
unsafe impl ::windows_core::Interface for IDWriteFont {
    type Vtable = IDWriteFont_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFont {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xacd16696_8c14_4f5d_877e_fe3fc1d32737);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFont_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetFontFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfamily: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_WEIGHT,
    pub GetStretch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STRETCH,
    pub GetStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STYLE,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSymbolFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSymbolFont: usize,
    pub GetFaceNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, names: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInformationalStrings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInformationalStrings: usize,
    pub GetSimulations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS,
    pub GetMetrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS),
    #[cfg(feature = "Win32_Foundation")]
    pub HasCharacter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicodevalue: u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasCharacter: usize,
    pub CreateFontFace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFont1(::windows_core::IUnknown);
impl IDWriteFont1 {
    pub unsafe fn GetFontFamily(&self) -> ::windows_core::Result<IDWriteFontFamily> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontFamily(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        ::windows_core::vcall!(self.base__.GetWeight())
    }
    pub unsafe fn GetStretch(&self) -> DWRITE_FONT_STRETCH {
        ::windows_core::vcall!(self.base__.GetStretch())
    }
    pub unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        ::windows_core::vcall!(self.base__.GetStyle())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.IsSymbolFont())
    }
    pub unsafe fn GetFaceNames(&self) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFaceNames(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetInformationalStrings(informationalstringid, ::core::mem::transmute(informationalstrings), exists)).ok()
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        ::windows_core::vcall!(self.base__.GetSimulations())
    }
    pub unsafe fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS) {
        ::windows_core::vcall!(self.base__.GetMetrics(fontmetrics))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.HasCharacter(unicodevalue, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self) -> ::windows_core::Result<IDWriteFontFace> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontFace(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        ::windows_core::vcall!(self.GetMetrics2(fontmetrics))
    }
    pub unsafe fn GetPanose(&self) -> DWRITE_PANOSE {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetPanose(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: ::core::option::Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetUnicodeRanges(unicoderanges.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(unicoderanges.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), actualrangecount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.IsMonospacedFont())
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFont1, ::windows_core::IUnknown, IDWriteFont);
unsafe impl ::core::marker::Send for IDWriteFont1 {}
unsafe impl ::core::marker::Sync for IDWriteFont1 {}
unsafe impl ::windows_core::Interface for IDWriteFont1 {
    type Vtable = IDWriteFont1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFont1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xacd16696_8c14_4f5d_877e_fe3fc1d32738);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFont1_Vtbl {
    pub base__: IDWriteFont_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMetrics2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS1),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMetrics2: usize,
    pub GetPanose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, panose: *mut DWRITE_PANOSE),
    pub GetUnicodeRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMonospacedFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMonospacedFont: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFont2(::windows_core::IUnknown);
impl IDWriteFont2 {
    pub unsafe fn GetFontFamily(&self) -> ::windows_core::Result<IDWriteFontFamily> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFontFamily(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        ::windows_core::vcall!(self.base__.base__.GetWeight())
    }
    pub unsafe fn GetStretch(&self) -> DWRITE_FONT_STRETCH {
        ::windows_core::vcall!(self.base__.base__.GetStretch())
    }
    pub unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        ::windows_core::vcall!(self.base__.base__.GetStyle())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.IsSymbolFont())
    }
    pub unsafe fn GetFaceNames(&self) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFaceNames(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetInformationalStrings(informationalstringid, ::core::mem::transmute(informationalstrings), exists)).ok()
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        ::windows_core::vcall!(self.base__.base__.GetSimulations())
    }
    pub unsafe fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS) {
        ::windows_core::vcall!(self.base__.base__.GetMetrics(fontmetrics))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.HasCharacter(unicodevalue, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self) -> ::windows_core::Result<IDWriteFontFace> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateFontFace(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        ::windows_core::vcall!(self.base__.GetMetrics2(fontmetrics))
    }
    pub unsafe fn GetPanose(&self) -> DWRITE_PANOSE {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetPanose(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: ::core::option::Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetUnicodeRanges(unicoderanges.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(unicoderanges.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), actualrangecount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.IsMonospacedFont())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsColorFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.IsColorFont())
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFont2, ::windows_core::IUnknown, IDWriteFont, IDWriteFont1);
unsafe impl ::core::marker::Send for IDWriteFont2 {}
unsafe impl ::core::marker::Sync for IDWriteFont2 {}
unsafe impl ::windows_core::Interface for IDWriteFont2 {
    type Vtable = IDWriteFont2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFont2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29748ed6_8c9c_4a6a_be0b_d912e8538944);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFont2_Vtbl {
    pub base__: IDWriteFont1_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsColorFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsColorFont: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFont3(::windows_core::IUnknown);
impl IDWriteFont3 {
    pub unsafe fn GetFontFamily(&self) -> ::windows_core::Result<IDWriteFontFamily> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFontFamily(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        ::windows_core::vcall!(self.base__.base__.base__.GetWeight())
    }
    pub unsafe fn GetStretch(&self) -> DWRITE_FONT_STRETCH {
        ::windows_core::vcall!(self.base__.base__.base__.GetStretch())
    }
    pub unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        ::windows_core::vcall!(self.base__.base__.base__.GetStyle())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.IsSymbolFont())
    }
    pub unsafe fn GetFaceNames(&self) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFaceNames(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetInformationalStrings(informationalstringid, ::core::mem::transmute(informationalstrings), exists)).ok()
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        ::windows_core::vcall!(self.base__.base__.base__.GetSimulations())
    }
    pub unsafe fn GetMetrics(&self, fontmetrics: *mut DWRITE_FONT_METRICS) {
        ::windows_core::vcall!(self.base__.base__.base__.GetMetrics(fontmetrics))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.HasCharacter(unicodevalue, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self) -> ::windows_core::Result<IDWriteFontFace> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateFontFace(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        ::windows_core::vcall!(self.base__.base__.GetMetrics2(fontmetrics))
    }
    pub unsafe fn GetPanose(&self) -> DWRITE_PANOSE {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetPanose(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: ::core::option::Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetUnicodeRanges(unicoderanges.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(unicoderanges.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), actualrangecount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.IsMonospacedFont())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsColorFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.IsColorFont())
    }
    pub unsafe fn CreateFontFace2(&self) -> ::windows_core::Result<IDWriteFontFace3> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFace2(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Equals<P0>(&self, font: P0) -> super::super::Foundation::BOOL
    where
        P0: ::windows_core::IntoParam<IDWriteFont>,
    {
        ::windows_core::vcall!(self.Equals(font.into_param().abi()))
    }
    pub unsafe fn GetFontFaceReference(&self) -> ::windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontFaceReference(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCharacter2(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.HasCharacter2(unicodevalue))
    }
    pub unsafe fn GetLocality(&self) -> DWRITE_LOCALITY {
        ::windows_core::vcall!(self.GetLocality())
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFont3, ::windows_core::IUnknown, IDWriteFont, IDWriteFont1, IDWriteFont2);
unsafe impl ::core::marker::Send for IDWriteFont3 {}
unsafe impl ::core::marker::Sync for IDWriteFont3 {}
unsafe impl ::windows_core::Interface for IDWriteFont3 {
    type Vtable = IDWriteFont3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFont3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x29748ed6_8c9c_4a6a_be0b_d912e8538944);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFont3_Vtbl {
    pub base__: IDWriteFont2_Vtbl,
    pub CreateFontFace2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Equals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, font: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    Equals: usize,
    pub GetFontFaceReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasCharacter2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasCharacter2: usize,
    pub GetLocality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_LOCALITY,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontCollection(::windows_core::IUnknown);
impl IDWriteFontCollection {
    pub unsafe fn GetFontFamilyCount(&self) -> u32 {
        ::windows_core::vcall!(self.GetFontFamilyCount())
    }
    pub unsafe fn GetFontFamily(&self, index: u32) -> ::windows_core::Result<IDWriteFontFamily> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontFamily(index, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFamilyName<P0>(&self, familyname: P0, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.FindFamilyName(familyname.into_param().abi(), index, exists)).ok()
    }
    pub unsafe fn GetFontFromFontFace<P0>(&self, fontface: P0) -> ::windows_core::Result<IDWriteFont>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontFromFontFace(fontface.into_param().abi(), &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontCollection, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFontCollection {}
unsafe impl ::core::marker::Sync for IDWriteFontCollection {}
unsafe impl ::windows_core::Interface for IDWriteFontCollection {
    type Vtable = IDWriteFontCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa84cee02_3eea_4eee_a827_87c1a02a0fcc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetFontFamilyCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetFontFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, fontfamily: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, familyname: ::windows_core::PCWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindFamilyName: usize,
    pub GetFontFromFontFace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, font: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontCollection1(::windows_core::IUnknown);
impl IDWriteFontCollection1 {
    pub unsafe fn GetFontFamilyCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.GetFontFamilyCount())
    }
    pub unsafe fn GetFontFamily(&self, index: u32) -> ::windows_core::Result<IDWriteFontFamily> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontFamily(index, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFamilyName<P0>(&self, familyname: P0, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.FindFamilyName(familyname.into_param().abi(), index, exists)).ok()
    }
    pub unsafe fn GetFontFromFontFace<P0>(&self, fontface: P0) -> ::windows_core::Result<IDWriteFont>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontFromFontFace(fontface.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontSet(&self) -> ::windows_core::Result<IDWriteFontSet> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontSet(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFamily2(&self, index: u32) -> ::windows_core::Result<IDWriteFontFamily1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontFamily2(index, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontCollection1, ::windows_core::IUnknown, IDWriteFontCollection);
unsafe impl ::core::marker::Send for IDWriteFontCollection1 {}
unsafe impl ::core::marker::Sync for IDWriteFontCollection1 {}
unsafe impl ::windows_core::Interface for IDWriteFontCollection1 {
    type Vtable = IDWriteFontCollection1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontCollection1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53585141_d9f8_4095_8321_d73cf6bd116c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontCollection1_Vtbl {
    pub base__: IDWriteFontCollection_Vtbl,
    pub GetFontSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFontFamily2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, fontfamily: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontCollection2(::windows_core::IUnknown);
impl IDWriteFontCollection2 {
    pub unsafe fn GetFontFamilyCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.GetFontFamilyCount())
    }
    pub unsafe fn GetFontFamily(&self, index: u32) -> ::windows_core::Result<IDWriteFontFamily> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFontFamily(index, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFamilyName<P0>(&self, familyname: P0, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.base__.FindFamilyName(familyname.into_param().abi(), index, exists)).ok()
    }
    pub unsafe fn GetFontFromFontFace<P0>(&self, fontface: P0) -> ::windows_core::Result<IDWriteFont>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFontFromFontFace(fontface.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontSet(&self) -> ::windows_core::Result<IDWriteFontSet> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontSet(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFamily2(&self, index: u32) -> ::windows_core::Result<IDWriteFontFamily1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontFamily2(index, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFamily3(&self, index: u32) -> ::windows_core::Result<IDWriteFontFamily2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontFamily3(index, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts<P0>(&self, familyname: P0, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> ::windows_core::Result<IDWriteFontList2>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetMatchingFonts(familyname.into_param().abi(), ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyModel(&self) -> DWRITE_FONT_FAMILY_MODEL {
        ::windows_core::vcall!(self.GetFontFamilyModel())
    }
    pub unsafe fn GetFontSet2(&self) -> ::windows_core::Result<IDWriteFontSet1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontSet2(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontCollection2, ::windows_core::IUnknown, IDWriteFontCollection, IDWriteFontCollection1);
unsafe impl ::core::marker::Send for IDWriteFontCollection2 {}
unsafe impl ::core::marker::Sync for IDWriteFontCollection2 {}
unsafe impl ::windows_core::Interface for IDWriteFontCollection2 {
    type Vtable = IDWriteFontCollection2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontCollection2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x514039c6_4617_4064_bf8b_92ea83e506e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontCollection2_Vtbl {
    pub base__: IDWriteFontCollection1_Vtbl,
    pub GetFontFamily3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, fontfamily: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetMatchingFonts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, familyname: ::windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFontFamilyModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_FAMILY_MODEL,
    pub GetFontSet2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontCollection3(::windows_core::IUnknown);
impl IDWriteFontCollection3 {
    pub unsafe fn GetFontFamilyCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontFamilyCount())
    }
    pub unsafe fn GetFontFamily(&self, index: u32) -> ::windows_core::Result<IDWriteFontFamily> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFontFamily(index, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFamilyName<P0>(&self, familyname: P0, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.FindFamilyName(familyname.into_param().abi(), index, exists)).ok()
    }
    pub unsafe fn GetFontFromFontFace<P0>(&self, fontface: P0) -> ::windows_core::Result<IDWriteFont>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFontFromFontFace(fontface.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontSet(&self) -> ::windows_core::Result<IDWriteFontSet> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFontSet(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFamily2(&self, index: u32) -> ::windows_core::Result<IDWriteFontFamily1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFontFamily2(index, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFamily3(&self, index: u32) -> ::windows_core::Result<IDWriteFontFamily2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontFamily3(index, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts<P0>(&self, familyname: P0, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> ::windows_core::Result<IDWriteFontList2>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetMatchingFonts(familyname.into_param().abi(), ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyModel(&self) -> DWRITE_FONT_FAMILY_MODEL {
        ::windows_core::vcall!(self.base__.GetFontFamilyModel())
    }
    pub unsafe fn GetFontSet2(&self) -> ::windows_core::Result<IDWriteFontSet1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontSet2(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExpirationEvent(&self) -> super::super::Foundation::HANDLE {
        ::windows_core::vcall!(self.GetExpirationEvent())
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontCollection3, ::windows_core::IUnknown, IDWriteFontCollection, IDWriteFontCollection1, IDWriteFontCollection2);
unsafe impl ::core::marker::Send for IDWriteFontCollection3 {}
unsafe impl ::core::marker::Sync for IDWriteFontCollection3 {}
unsafe impl ::windows_core::Interface for IDWriteFontCollection3 {
    type Vtable = IDWriteFontCollection3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontCollection3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa4d055a6_f9e3_4e25_93b7_9e309f3af8e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontCollection3_Vtbl {
    pub base__: IDWriteFontCollection2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetExpirationEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetExpirationEvent: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontCollectionLoader(::windows_core::IUnknown);
impl IDWriteFontCollectionLoader {
    pub unsafe fn CreateEnumeratorFromKey<P0>(&self, factory: P0, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows_core::Result<IDWriteFontFileEnumerator>
    where
        P0: ::windows_core::IntoParam<IDWriteFactory>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateEnumeratorFromKey(factory.into_param().abi(), collectionkey, collectionkeysize, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontCollectionLoader, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFontCollectionLoader {}
unsafe impl ::core::marker::Sync for IDWriteFontCollectionLoader {}
unsafe impl ::windows_core::Interface for IDWriteFontCollectionLoader {
    type Vtable = IDWriteFontCollectionLoader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontCollectionLoader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcca920e4_52f0_492b_bfa8_29c72ee0a468);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontCollectionLoader_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateEnumeratorFromKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32, fontfileenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontDownloadListener(::windows_core::IUnknown);
impl IDWriteFontDownloadListener {
    pub unsafe fn DownloadCompleted<P0, P1>(&self, downloadqueue: P0, context: P1, downloadresult: ::windows_core::HRESULT)
    where
        P0: ::windows_core::IntoParam<IDWriteFontDownloadQueue>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.DownloadCompleted(downloadqueue.into_param().abi(), context.into_param().abi(), downloadresult))
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontDownloadListener, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFontDownloadListener {}
unsafe impl ::core::marker::Sync for IDWriteFontDownloadListener {}
unsafe impl ::windows_core::Interface for IDWriteFontDownloadListener {
    type Vtable = IDWriteFontDownloadListener_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontDownloadListener {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb06fe5b9_43ec_4393_881b_dbe4dc72fda7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontDownloadListener_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub DownloadCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadqueue: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, downloadresult: ::windows_core::HRESULT),
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontDownloadQueue(::windows_core::IUnknown);
impl IDWriteFontDownloadQueue {
    pub unsafe fn AddListener<P0>(&self, listener: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<IDWriteFontDownloadListener>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.AddListener(listener.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn RemoveListener(&self, token: u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.RemoveListener(token)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEmpty(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.IsEmpty())
    }
    pub unsafe fn BeginDownload<P0>(&self, context: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.BeginDownload(context.into_param().abi())).ok()
    }
    pub unsafe fn CancelDownload(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.CancelDownload()).ok()
    }
    pub unsafe fn GetGenerationCount(&self) -> u64 {
        ::windows_core::vcall!(self.GetGenerationCount())
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontDownloadQueue, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFontDownloadQueue {}
unsafe impl ::core::marker::Sync for IDWriteFontDownloadQueue {}
unsafe impl ::windows_core::Interface for IDWriteFontDownloadQueue {
    type Vtable = IDWriteFontDownloadQueue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontDownloadQueue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb71e6052_5aea_4fa3_832e_f60d431f7e91);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontDownloadQueue_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddListener: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listener: *mut ::core::ffi::c_void, token: *mut u32) -> ::windows_core::HRESULT,
    pub RemoveListener: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEmpty: usize,
    pub BeginDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CancelDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetGenerationCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFace(::windows_core::IUnknown);
impl IDWriteFontFace {
    pub unsafe fn GetType(&self) -> DWRITE_FONT_FACE_TYPE {
        ::windows_core::vcall!(self.GetType())
    }
    pub unsafe fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: ::core::option::Option<*mut ::core::option::Option<IDWriteFontFile>>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFiles(numberoffiles, ::core::mem::transmute(fontfiles.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetIndex(&self) -> u32 {
        ::windows_core::vcall!(self.GetIndex())
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        ::windows_core::vcall!(self.GetSimulations())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.IsSymbolFont())
    }
    pub unsafe fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
        ::windows_core::vcall!(self.GetMetrics(fontfacemetrics))
    }
    pub unsafe fn GetGlyphCount(&self) -> u16 {
        ::windows_core::vcall!(self.GetGlyphCount())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphMetrics<P0>(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.GetDesignGlyphMetrics(glyphindices, glyphcount, glyphmetrics, issideways.into_param().abi())).ok()
    }
    pub unsafe fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetGlyphIndices(codepoints, codepointcount, glyphindices)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.TryGetFontTable(opentypetabletag, tabledata, tablesize, tablecontext, exists)).ok()
    }
    pub unsafe fn ReleaseFontTable(&self, tablecontext: *const ::core::ffi::c_void) {
        ::windows_core::vcall!(self.ReleaseFontTable(tablecontext))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphRunOutline<P0, P1, P2>(&self, emsize: f32, glyphindices: *const u16, glyphadvances: ::core::option::Option<*const f32>, glyphoffsets: ::core::option::Option<*const DWRITE_GLYPH_OFFSET>, glyphcount: u32, issideways: P0, isrighttoleft: P1, geometrysink: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::Direct2D::Common::ID2D1SimplifiedGeometrySink>,
    {
        ::windows_core::vcall!(self.GetGlyphRunOutline(emsize, glyphindices, ::core::mem::transmute(glyphadvances.unwrap_or(::std::ptr::null())), ::core::mem::transmute(glyphoffsets.unwrap_or(::std::ptr::null())), glyphcount, issideways.into_param().abi(), isrighttoleft.into_param().abi(), geometrysink.into_param().abi())).ok()
    }
    pub unsafe fn GetRecommendedRenderingMode<P0>(&self, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P0) -> ::windows_core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::windows_core::IntoParam<IDWriteRenderingParams>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetRecommendedRenderingMode(emsize, pixelsperdip, measuringmode, renderingparams.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetGdiCompatibleMetrics(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontfacemetrics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphMetrics<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.GetGdiCompatibleGlyphMetrics(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), glyphindices, glyphcount, glyphmetrics, issideways.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFace, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFontFace {}
unsafe impl ::core::marker::Sync for IDWriteFontFace {}
unsafe impl ::windows_core::Interface for IDWriteFontFace {
    type Vtable = IDWriteFontFace_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFace {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f49804d_7024_4d43_bfa9_d25984f53849);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFace_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_FACE_TYPE,
    pub GetFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numberoffiles: *mut u32, fontfiles: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetSimulations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSymbolFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSymbolFont: usize,
    pub GetMetrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfacemetrics: *mut DWRITE_FONT_METRICS),
    pub GetGlyphCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u16,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesignGlyphMetrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesignGlyphMetrics: usize,
    pub GetGlyphIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub TryGetFontTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TryGetFontTable: usize,
    pub ReleaseFontTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablecontext: *const ::core::ffi::c_void),
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub GetGlyphRunOutline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emsize: f32, glyphindices: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphcount: u32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, geometrysink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    GetGlyphRunOutline: usize,
    pub GetRecommendedRenderingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: *mut ::core::ffi::c_void, renderingmode: *mut DWRITE_RENDERING_MODE) -> ::windows_core::HRESULT,
    pub GetGdiCompatibleMetrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGdiCompatibleGlyphMetrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGdiCompatibleGlyphMetrics: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFace1(::windows_core::IUnknown);
impl IDWriteFontFace1 {
    pub unsafe fn GetType(&self) -> DWRITE_FONT_FACE_TYPE {
        ::windows_core::vcall!(self.base__.GetType())
    }
    pub unsafe fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: ::core::option::Option<*mut ::core::option::Option<IDWriteFontFile>>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetFiles(numberoffiles, ::core::mem::transmute(fontfiles.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetIndex(&self) -> u32 {
        ::windows_core::vcall!(self.base__.GetIndex())
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        ::windows_core::vcall!(self.base__.GetSimulations())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.IsSymbolFont())
    }
    pub unsafe fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
        ::windows_core::vcall!(self.base__.GetMetrics(fontfacemetrics))
    }
    pub unsafe fn GetGlyphCount(&self) -> u16 {
        ::windows_core::vcall!(self.base__.GetGlyphCount())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphMetrics<P0>(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.GetDesignGlyphMetrics(glyphindices, glyphcount, glyphmetrics, issideways.into_param().abi())).ok()
    }
    pub unsafe fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetGlyphIndices(codepoints, codepointcount, glyphindices)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.TryGetFontTable(opentypetabletag, tabledata, tablesize, tablecontext, exists)).ok()
    }
    pub unsafe fn ReleaseFontTable(&self, tablecontext: *const ::core::ffi::c_void) {
        ::windows_core::vcall!(self.base__.ReleaseFontTable(tablecontext))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphRunOutline<P0, P1, P2>(&self, emsize: f32, glyphindices: *const u16, glyphadvances: ::core::option::Option<*const f32>, glyphoffsets: ::core::option::Option<*const DWRITE_GLYPH_OFFSET>, glyphcount: u32, issideways: P0, isrighttoleft: P1, geometrysink: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::Direct2D::Common::ID2D1SimplifiedGeometrySink>,
    {
        ::windows_core::vcall!(self.base__.GetGlyphRunOutline(emsize, glyphindices, ::core::mem::transmute(glyphadvances.unwrap_or(::std::ptr::null())), ::core::mem::transmute(glyphoffsets.unwrap_or(::std::ptr::null())), glyphcount, issideways.into_param().abi(), isrighttoleft.into_param().abi(), geometrysink.into_param().abi())).ok()
    }
    pub unsafe fn GetRecommendedRenderingMode<P0>(&self, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P0) -> ::windows_core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::windows_core::IntoParam<IDWriteRenderingParams>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetRecommendedRenderingMode(emsize, pixelsperdip, measuringmode, renderingparams.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetGdiCompatibleMetrics(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontfacemetrics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphMetrics<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.GetGdiCompatibleGlyphMetrics(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), glyphindices, glyphcount, glyphmetrics, issideways.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        ::windows_core::vcall!(self.GetMetrics2(fontmetrics))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleMetrics2(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetGdiCompatibleMetrics2(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontmetrics)).ok()
    }
    pub unsafe fn GetCaretMetrics(&self) -> DWRITE_CARET_METRICS {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetCaretMetrics(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: ::core::option::Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetUnicodeRanges(unicoderanges.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(unicoderanges.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), actualrangecount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.IsMonospacedFont())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphAdvances<P0>(&self, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.GetDesignGlyphAdvances(glyphcount, glyphindices, glyphadvances, issideways.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphAdvances<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, issideways: P1, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.GetGdiCompatibleGlyphAdvances(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), issideways.into_param().abi(), glyphcount, glyphindices, glyphadvances)).ok()
    }
    pub unsafe fn GetKerningPairAdjustments(&self, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetKerningPairAdjustments(glyphcount, glyphindices, glyphadvanceadjustments)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasKerningPairs(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.HasKerningPairs())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode2<P0>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE) -> ::windows_core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetRecommendedRenderingMode2(fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into_param().abi(), outlinethreshold, measuringmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetVerticalGlyphVariants(&self, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetVerticalGlyphVariants(glyphcount, nominalglyphindices, verticalglyphindices)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVerticalGlyphVariants(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.HasVerticalGlyphVariants())
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFace1, ::windows_core::IUnknown, IDWriteFontFace);
unsafe impl ::core::marker::Send for IDWriteFontFace1 {}
unsafe impl ::core::marker::Sync for IDWriteFontFace1 {}
unsafe impl ::windows_core::Interface for IDWriteFontFace1 {
    type Vtable = IDWriteFontFace1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFace1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa71efdb4_9fdb_4838_ad90_cfc3be8c3daf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFace1_Vtbl {
    pub base__: IDWriteFontFace_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMetrics2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS1),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMetrics2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGdiCompatibleMetrics2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGdiCompatibleMetrics2: usize,
    pub GetCaretMetrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, caretmetrics: *mut DWRITE_CARET_METRICS),
    pub GetUnicodeRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsMonospacedFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsMonospacedFont: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDesignGlyphAdvances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDesignGlyphAdvances: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGdiCompatibleGlyphAdvances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, issideways: super::super::Foundation::BOOL, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGdiCompatibleGlyphAdvances: usize,
    pub GetKerningPairAdjustments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasKerningPairs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasKerningPairs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRecommendedRenderingMode2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingmode: *mut DWRITE_RENDERING_MODE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRecommendedRenderingMode2: usize,
    pub GetVerticalGlyphVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasVerticalGlyphVariants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasVerticalGlyphVariants: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFace2(::windows_core::IUnknown);
impl IDWriteFontFace2 {
    pub unsafe fn GetType(&self) -> DWRITE_FONT_FACE_TYPE {
        ::windows_core::vcall!(self.base__.base__.GetType())
    }
    pub unsafe fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: ::core::option::Option<*mut ::core::option::Option<IDWriteFontFile>>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetFiles(numberoffiles, ::core::mem::transmute(fontfiles.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetIndex(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.GetIndex())
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        ::windows_core::vcall!(self.base__.base__.GetSimulations())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.IsSymbolFont())
    }
    pub unsafe fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
        ::windows_core::vcall!(self.base__.base__.GetMetrics(fontfacemetrics))
    }
    pub unsafe fn GetGlyphCount(&self) -> u16 {
        ::windows_core::vcall!(self.base__.base__.GetGlyphCount())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphMetrics<P0>(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.GetDesignGlyphMetrics(glyphindices, glyphcount, glyphmetrics, issideways.into_param().abi())).ok()
    }
    pub unsafe fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetGlyphIndices(codepoints, codepointcount, glyphindices)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.TryGetFontTable(opentypetabletag, tabledata, tablesize, tablecontext, exists)).ok()
    }
    pub unsafe fn ReleaseFontTable(&self, tablecontext: *const ::core::ffi::c_void) {
        ::windows_core::vcall!(self.base__.base__.ReleaseFontTable(tablecontext))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphRunOutline<P0, P1, P2>(&self, emsize: f32, glyphindices: *const u16, glyphadvances: ::core::option::Option<*const f32>, glyphoffsets: ::core::option::Option<*const DWRITE_GLYPH_OFFSET>, glyphcount: u32, issideways: P0, isrighttoleft: P1, geometrysink: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::Direct2D::Common::ID2D1SimplifiedGeometrySink>,
    {
        ::windows_core::vcall!(self.base__.base__.GetGlyphRunOutline(emsize, glyphindices, ::core::mem::transmute(glyphadvances.unwrap_or(::std::ptr::null())), ::core::mem::transmute(glyphoffsets.unwrap_or(::std::ptr::null())), glyphcount, issideways.into_param().abi(), isrighttoleft.into_param().abi(), geometrysink.into_param().abi())).ok()
    }
    pub unsafe fn GetRecommendedRenderingMode<P0>(&self, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P0) -> ::windows_core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::windows_core::IntoParam<IDWriteRenderingParams>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetRecommendedRenderingMode(emsize, pixelsperdip, measuringmode, renderingparams.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetGdiCompatibleMetrics(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontfacemetrics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphMetrics<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.GetGdiCompatibleGlyphMetrics(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), glyphindices, glyphcount, glyphmetrics, issideways.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        ::windows_core::vcall!(self.base__.GetMetrics2(fontmetrics))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleMetrics2(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetGdiCompatibleMetrics2(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontmetrics)).ok()
    }
    pub unsafe fn GetCaretMetrics(&self) -> DWRITE_CARET_METRICS {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetCaretMetrics(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: ::core::option::Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetUnicodeRanges(unicoderanges.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(unicoderanges.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), actualrangecount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.IsMonospacedFont())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphAdvances<P0>(&self, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.GetDesignGlyphAdvances(glyphcount, glyphindices, glyphadvances, issideways.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphAdvances<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, issideways: P1, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.GetGdiCompatibleGlyphAdvances(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), issideways.into_param().abi(), glyphcount, glyphindices, glyphadvances)).ok()
    }
    pub unsafe fn GetKerningPairAdjustments(&self, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetKerningPairAdjustments(glyphcount, glyphindices, glyphadvanceadjustments)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasKerningPairs(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.HasKerningPairs())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode2<P0>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE) -> ::windows_core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetRecommendedRenderingMode2(fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into_param().abi(), outlinethreshold, measuringmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetVerticalGlyphVariants(&self, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetVerticalGlyphVariants(glyphcount, nominalglyphindices, verticalglyphindices)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVerticalGlyphVariants(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.HasVerticalGlyphVariants())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsColorFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.IsColorFont())
    }
    pub unsafe fn GetColorPaletteCount(&self) -> u32 {
        ::windows_core::vcall!(self.GetColorPaletteCount())
    }
    pub unsafe fn GetPaletteEntryCount(&self) -> u32 {
        ::windows_core::vcall!(self.GetPaletteEntryCount())
    }
    pub unsafe fn GetPaletteEntries(&self, colorpaletteindex: u32, firstentryindex: u32, paletteentries: &mut [DWRITE_COLOR_F]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetPaletteEntries(colorpaletteindex, firstentryindex, paletteentries.len().try_into().unwrap(), ::core::mem::transmute(paletteentries.as_ptr()))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode3<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<IDWriteRenderingParams>,
    {
        ::windows_core::vcall!(self.GetRecommendedRenderingMode3(fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into_param().abi(), outlinethreshold, measuringmode, renderingparams.into_param().abi(), renderingmode, gridfitmode)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFace2, ::windows_core::IUnknown, IDWriteFontFace, IDWriteFontFace1);
unsafe impl ::core::marker::Send for IDWriteFontFace2 {}
unsafe impl ::core::marker::Sync for IDWriteFontFace2 {}
unsafe impl ::windows_core::Interface for IDWriteFontFace2 {
    type Vtable = IDWriteFontFace2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFace2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8b768ff_64bc_4e66_982b_ec8e87f693f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFace2_Vtbl {
    pub base__: IDWriteFontFace1_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsColorFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsColorFont: usize,
    pub GetColorPaletteCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetPaletteEntryCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetPaletteEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorpaletteindex: u32, firstentryindex: u32, entrycount: u32, paletteentries: *mut DWRITE_COLOR_F) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRecommendedRenderingMode3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: *mut ::core::ffi::c_void, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRecommendedRenderingMode3: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFace3(::windows_core::IUnknown);
impl IDWriteFontFace3 {
    pub unsafe fn GetType(&self) -> DWRITE_FONT_FACE_TYPE {
        ::windows_core::vcall!(self.base__.base__.base__.GetType())
    }
    pub unsafe fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: ::core::option::Option<*mut ::core::option::Option<IDWriteFontFile>>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetFiles(numberoffiles, ::core::mem::transmute(fontfiles.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetIndex(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetIndex())
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        ::windows_core::vcall!(self.base__.base__.base__.GetSimulations())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.IsSymbolFont())
    }
    pub unsafe fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
        ::windows_core::vcall!(self.base__.base__.base__.GetMetrics(fontfacemetrics))
    }
    pub unsafe fn GetGlyphCount(&self) -> u16 {
        ::windows_core::vcall!(self.base__.base__.base__.GetGlyphCount())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphMetrics<P0>(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.GetDesignGlyphMetrics(glyphindices, glyphcount, glyphmetrics, issideways.into_param().abi())).ok()
    }
    pub unsafe fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetGlyphIndices(codepoints, codepointcount, glyphindices)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.TryGetFontTable(opentypetabletag, tabledata, tablesize, tablecontext, exists)).ok()
    }
    pub unsafe fn ReleaseFontTable(&self, tablecontext: *const ::core::ffi::c_void) {
        ::windows_core::vcall!(self.base__.base__.base__.ReleaseFontTable(tablecontext))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphRunOutline<P0, P1, P2>(&self, emsize: f32, glyphindices: *const u16, glyphadvances: ::core::option::Option<*const f32>, glyphoffsets: ::core::option::Option<*const DWRITE_GLYPH_OFFSET>, glyphcount: u32, issideways: P0, isrighttoleft: P1, geometrysink: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::Direct2D::Common::ID2D1SimplifiedGeometrySink>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.GetGlyphRunOutline(emsize, glyphindices, ::core::mem::transmute(glyphadvances.unwrap_or(::std::ptr::null())), ::core::mem::transmute(glyphoffsets.unwrap_or(::std::ptr::null())), glyphcount, issideways.into_param().abi(), isrighttoleft.into_param().abi(), geometrysink.into_param().abi())).ok()
    }
    pub unsafe fn GetRecommendedRenderingMode<P0>(&self, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P0) -> ::windows_core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::windows_core::IntoParam<IDWriteRenderingParams>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetRecommendedRenderingMode(emsize, pixelsperdip, measuringmode, renderingparams.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetGdiCompatibleMetrics(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontfacemetrics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphMetrics<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.GetGdiCompatibleGlyphMetrics(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), glyphindices, glyphcount, glyphmetrics, issideways.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        ::windows_core::vcall!(self.base__.base__.GetMetrics2(fontmetrics))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleMetrics2(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetGdiCompatibleMetrics2(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontmetrics)).ok()
    }
    pub unsafe fn GetCaretMetrics(&self) -> DWRITE_CARET_METRICS {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetCaretMetrics(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: ::core::option::Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetUnicodeRanges(unicoderanges.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(unicoderanges.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), actualrangecount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.IsMonospacedFont())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphAdvances<P0>(&self, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.GetDesignGlyphAdvances(glyphcount, glyphindices, glyphadvances, issideways.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphAdvances<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, issideways: P1, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.GetGdiCompatibleGlyphAdvances(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), issideways.into_param().abi(), glyphcount, glyphindices, glyphadvances)).ok()
    }
    pub unsafe fn GetKerningPairAdjustments(&self, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetKerningPairAdjustments(glyphcount, glyphindices, glyphadvanceadjustments)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasKerningPairs(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.HasKerningPairs())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode2<P0>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE) -> ::windows_core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetRecommendedRenderingMode2(fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into_param().abi(), outlinethreshold, measuringmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetVerticalGlyphVariants(&self, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetVerticalGlyphVariants(glyphcount, nominalglyphindices, verticalglyphindices)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVerticalGlyphVariants(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.HasVerticalGlyphVariants())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsColorFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.IsColorFont())
    }
    pub unsafe fn GetColorPaletteCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.GetColorPaletteCount())
    }
    pub unsafe fn GetPaletteEntryCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.GetPaletteEntryCount())
    }
    pub unsafe fn GetPaletteEntries(&self, colorpaletteindex: u32, firstentryindex: u32, paletteentries: &mut [DWRITE_COLOR_F]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetPaletteEntries(colorpaletteindex, firstentryindex, paletteentries.len().try_into().unwrap(), ::core::mem::transmute(paletteentries.as_ptr()))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode3<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<IDWriteRenderingParams>,
    {
        ::windows_core::vcall!(self.base__.GetRecommendedRenderingMode3(fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into_param().abi(), outlinethreshold, measuringmode, renderingparams.into_param().abi(), renderingmode, gridfitmode)).ok()
    }
    pub unsafe fn GetFontFaceReference(&self) -> ::windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontFaceReference(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetPanose(&self) -> DWRITE_PANOSE {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetPanose(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        ::windows_core::vcall!(self.GetWeight())
    }
    pub unsafe fn GetStretch(&self) -> DWRITE_FONT_STRETCH {
        ::windows_core::vcall!(self.GetStretch())
    }
    pub unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        ::windows_core::vcall!(self.GetStyle())
    }
    pub unsafe fn GetFamilyNames(&self) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFamilyNames(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFaceNames(&self) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFaceNames(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetInformationalStrings(informationalstringid, ::core::mem::transmute(informationalstrings), exists)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.HasCharacter(unicodevalue))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode4<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<IDWriteRenderingParams>,
    {
        ::windows_core::vcall!(self.GetRecommendedRenderingMode4(fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into_param().abi(), outlinethreshold, measuringmode, renderingparams.into_param().abi(), renderingmode, gridfitmode)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCharacterLocal(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.IsCharacterLocal(unicodevalue))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsGlyphLocal(&self, glyphid: u16) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.IsGlyphLocal(glyphid))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AreCharactersLocal<P0>(&self, characters: &[u16], enqueueifnotlocal: P0) -> ::windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.AreCharactersLocal(::core::mem::transmute(characters.as_ptr()), characters.len().try_into().unwrap(), enqueueifnotlocal.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AreGlyphsLocal<P0>(&self, glyphindices: &[u16], enqueueifnotlocal: P0) -> ::windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.AreGlyphsLocal(::core::mem::transmute(glyphindices.as_ptr()), glyphindices.len().try_into().unwrap(), enqueueifnotlocal.into_param().abi(), &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFace3, ::windows_core::IUnknown, IDWriteFontFace, IDWriteFontFace1, IDWriteFontFace2);
unsafe impl ::core::marker::Send for IDWriteFontFace3 {}
unsafe impl ::core::marker::Sync for IDWriteFontFace3 {}
unsafe impl ::windows_core::Interface for IDWriteFontFace3 {
    type Vtable = IDWriteFontFace3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFace3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd37d7598_09be_4222_a236_2081341cc1f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFace3_Vtbl {
    pub base__: IDWriteFontFace2_Vtbl,
    pub GetFontFaceReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPanose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, panose: *mut DWRITE_PANOSE),
    pub GetWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_WEIGHT,
    pub GetStretch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STRETCH,
    pub GetStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STYLE,
    pub GetFamilyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, names: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFaceNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, names: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInformationalStrings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInformationalStrings: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HasCharacter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasCharacter: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRecommendedRenderingMode4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: *mut ::core::ffi::c_void, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRecommendedRenderingMode4: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCharacterLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCharacterLocal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsGlyphLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphid: u16) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsGlyphLocal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AreCharactersLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, characters: ::windows_core::PCWSTR, charactercount: u32, enqueueifnotlocal: super::super::Foundation::BOOL, islocal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AreCharactersLocal: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AreGlyphsLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphindices: *const u16, glyphcount: u32, enqueueifnotlocal: super::super::Foundation::BOOL, islocal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AreGlyphsLocal: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFace4(::windows_core::IUnknown);
impl IDWriteFontFace4 {
    pub unsafe fn GetType(&self) -> DWRITE_FONT_FACE_TYPE {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetType())
    }
    pub unsafe fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: ::core::option::Option<*mut ::core::option::Option<IDWriteFontFile>>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFiles(numberoffiles, ::core::mem::transmute(fontfiles.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetIndex(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetIndex())
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetSimulations())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.base__.IsSymbolFont())
    }
    pub unsafe fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetMetrics(fontfacemetrics))
    }
    pub unsafe fn GetGlyphCount(&self) -> u16 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetGlyphCount())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphMetrics<P0>(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetDesignGlyphMetrics(glyphindices, glyphcount, glyphmetrics, issideways.into_param().abi())).ok()
    }
    pub unsafe fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetGlyphIndices(codepoints, codepointcount, glyphindices)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.TryGetFontTable(opentypetabletag, tabledata, tablesize, tablecontext, exists)).ok()
    }
    pub unsafe fn ReleaseFontTable(&self, tablecontext: *const ::core::ffi::c_void) {
        ::windows_core::vcall!(self.base__.base__.base__.base__.ReleaseFontTable(tablecontext))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphRunOutline<P0, P1, P2>(&self, emsize: f32, glyphindices: *const u16, glyphadvances: ::core::option::Option<*const f32>, glyphoffsets: ::core::option::Option<*const DWRITE_GLYPH_OFFSET>, glyphcount: u32, issideways: P0, isrighttoleft: P1, geometrysink: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::Direct2D::Common::ID2D1SimplifiedGeometrySink>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetGlyphRunOutline(emsize, glyphindices, ::core::mem::transmute(glyphadvances.unwrap_or(::std::ptr::null())), ::core::mem::transmute(glyphoffsets.unwrap_or(::std::ptr::null())), glyphcount, issideways.into_param().abi(), isrighttoleft.into_param().abi(), geometrysink.into_param().abi())).ok()
    }
    pub unsafe fn GetRecommendedRenderingMode<P0>(&self, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P0) -> ::windows_core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::windows_core::IntoParam<IDWriteRenderingParams>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetRecommendedRenderingMode(emsize, pixelsperdip, measuringmode, renderingparams.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetGdiCompatibleMetrics(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontfacemetrics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphMetrics<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetGdiCompatibleGlyphMetrics(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), glyphindices, glyphcount, glyphmetrics, issideways.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        ::windows_core::vcall!(self.base__.base__.base__.GetMetrics2(fontmetrics))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleMetrics2(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetGdiCompatibleMetrics2(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontmetrics)).ok()
    }
    pub unsafe fn GetCaretMetrics(&self) -> DWRITE_CARET_METRICS {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetCaretMetrics(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: ::core::option::Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetUnicodeRanges(unicoderanges.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(unicoderanges.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), actualrangecount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.IsMonospacedFont())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphAdvances<P0>(&self, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.GetDesignGlyphAdvances(glyphcount, glyphindices, glyphadvances, issideways.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphAdvances<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, issideways: P1, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.GetGdiCompatibleGlyphAdvances(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), issideways.into_param().abi(), glyphcount, glyphindices, glyphadvances)).ok()
    }
    pub unsafe fn GetKerningPairAdjustments(&self, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetKerningPairAdjustments(glyphcount, glyphindices, glyphadvanceadjustments)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasKerningPairs(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.HasKerningPairs())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode2<P0>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE) -> ::windows_core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetRecommendedRenderingMode2(fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into_param().abi(), outlinethreshold, measuringmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetVerticalGlyphVariants(&self, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetVerticalGlyphVariants(glyphcount, nominalglyphindices, verticalglyphindices)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVerticalGlyphVariants(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.HasVerticalGlyphVariants())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsColorFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.IsColorFont())
    }
    pub unsafe fn GetColorPaletteCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.GetColorPaletteCount())
    }
    pub unsafe fn GetPaletteEntryCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.GetPaletteEntryCount())
    }
    pub unsafe fn GetPaletteEntries(&self, colorpaletteindex: u32, firstentryindex: u32, paletteentries: &mut [DWRITE_COLOR_F]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetPaletteEntries(colorpaletteindex, firstentryindex, paletteentries.len().try_into().unwrap(), ::core::mem::transmute(paletteentries.as_ptr()))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode3<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<IDWriteRenderingParams>,
    {
        ::windows_core::vcall!(self.base__.base__.GetRecommendedRenderingMode3(fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into_param().abi(), outlinethreshold, measuringmode, renderingparams.into_param().abi(), renderingmode, gridfitmode)).ok()
    }
    pub unsafe fn GetFontFaceReference(&self) -> ::windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontFaceReference(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetPanose(&self) -> DWRITE_PANOSE {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetPanose(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        ::windows_core::vcall!(self.base__.GetWeight())
    }
    pub unsafe fn GetStretch(&self) -> DWRITE_FONT_STRETCH {
        ::windows_core::vcall!(self.base__.GetStretch())
    }
    pub unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        ::windows_core::vcall!(self.base__.GetStyle())
    }
    pub unsafe fn GetFamilyNames(&self) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFamilyNames(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFaceNames(&self) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFaceNames(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetInformationalStrings(informationalstringid, ::core::mem::transmute(informationalstrings), exists)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.HasCharacter(unicodevalue))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode4<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<IDWriteRenderingParams>,
    {
        ::windows_core::vcall!(self.base__.GetRecommendedRenderingMode4(fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into_param().abi(), outlinethreshold, measuringmode, renderingparams.into_param().abi(), renderingmode, gridfitmode)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCharacterLocal(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.IsCharacterLocal(unicodevalue))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsGlyphLocal(&self, glyphid: u16) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.IsGlyphLocal(glyphid))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AreCharactersLocal<P0>(&self, characters: &[u16], enqueueifnotlocal: P0) -> ::windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.AreCharactersLocal(::core::mem::transmute(characters.as_ptr()), characters.len().try_into().unwrap(), enqueueifnotlocal.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AreGlyphsLocal<P0>(&self, glyphindices: &[u16], enqueueifnotlocal: P0) -> ::windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.AreGlyphsLocal(::core::mem::transmute(glyphindices.as_ptr()), glyphindices.len().try_into().unwrap(), enqueueifnotlocal.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGlyphImageFormats(&self, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32) -> ::windows_core::Result<DWRITE_GLYPH_IMAGE_FORMATS> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetGlyphImageFormats(glyphid, pixelsperemfirst, pixelsperemlast, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGlyphImageFormats2(&self) -> DWRITE_GLYPH_IMAGE_FORMATS {
        ::windows_core::vcall!(self.GetGlyphImageFormats2())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphImageData(&self, glyphid: u16, pixelsperem: u32, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetGlyphImageData(glyphid, pixelsperem, glyphimageformat, glyphdata, glyphdatacontext)).ok()
    }
    pub unsafe fn ReleaseGlyphImageData(&self, glyphdatacontext: *mut ::core::ffi::c_void) {
        ::windows_core::vcall!(self.ReleaseGlyphImageData(glyphdatacontext))
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFace4, ::windows_core::IUnknown, IDWriteFontFace, IDWriteFontFace1, IDWriteFontFace2, IDWriteFontFace3);
unsafe impl ::core::marker::Send for IDWriteFontFace4 {}
unsafe impl ::core::marker::Sync for IDWriteFontFace4 {}
unsafe impl ::windows_core::Interface for IDWriteFontFace4 {
    type Vtable = IDWriteFontFace4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFace4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x27f2a904_4eb8_441d_9678_0563f53e3e2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFace4_Vtbl {
    pub base__: IDWriteFontFace3_Vtbl,
    pub GetGlyphImageFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32, glyphimageformats: *mut DWRITE_GLYPH_IMAGE_FORMATS) -> ::windows_core::HRESULT,
    pub GetGlyphImageFormats2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_GLYPH_IMAGE_FORMATS,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub GetGlyphImageData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphid: u16, pixelsperem: u32, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common")))]
    GetGlyphImageData: usize,
    pub ReleaseGlyphImageData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphdatacontext: *mut ::core::ffi::c_void),
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFace5(::windows_core::IUnknown);
impl IDWriteFontFace5 {
    pub unsafe fn GetType(&self) -> DWRITE_FONT_FACE_TYPE {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetType())
    }
    pub unsafe fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: ::core::option::Option<*mut ::core::option::Option<IDWriteFontFile>>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetFiles(numberoffiles, ::core::mem::transmute(fontfiles.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetIndex(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetIndex())
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetSimulations())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.IsSymbolFont())
    }
    pub unsafe fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetMetrics(fontfacemetrics))
    }
    pub unsafe fn GetGlyphCount(&self) -> u16 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetGlyphCount())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphMetrics<P0>(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetDesignGlyphMetrics(glyphindices, glyphcount, glyphmetrics, issideways.into_param().abi())).ok()
    }
    pub unsafe fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetGlyphIndices(codepoints, codepointcount, glyphindices)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.TryGetFontTable(opentypetabletag, tabledata, tablesize, tablecontext, exists)).ok()
    }
    pub unsafe fn ReleaseFontTable(&self, tablecontext: *const ::core::ffi::c_void) {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.ReleaseFontTable(tablecontext))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphRunOutline<P0, P1, P2>(&self, emsize: f32, glyphindices: *const u16, glyphadvances: ::core::option::Option<*const f32>, glyphoffsets: ::core::option::Option<*const DWRITE_GLYPH_OFFSET>, glyphcount: u32, issideways: P0, isrighttoleft: P1, geometrysink: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::Direct2D::Common::ID2D1SimplifiedGeometrySink>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetGlyphRunOutline(emsize, glyphindices, ::core::mem::transmute(glyphadvances.unwrap_or(::std::ptr::null())), ::core::mem::transmute(glyphoffsets.unwrap_or(::std::ptr::null())), glyphcount, issideways.into_param().abi(), isrighttoleft.into_param().abi(), geometrysink.into_param().abi())).ok()
    }
    pub unsafe fn GetRecommendedRenderingMode<P0>(&self, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P0) -> ::windows_core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::windows_core::IntoParam<IDWriteRenderingParams>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetRecommendedRenderingMode(emsize, pixelsperdip, measuringmode, renderingparams.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetGdiCompatibleMetrics(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontfacemetrics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphMetrics<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetGdiCompatibleGlyphMetrics(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), glyphindices, glyphcount, glyphmetrics, issideways.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetMetrics2(fontmetrics))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleMetrics2(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetGdiCompatibleMetrics2(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontmetrics)).ok()
    }
    pub unsafe fn GetCaretMetrics(&self) -> DWRITE_CARET_METRICS {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetCaretMetrics(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: ::core::option::Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetUnicodeRanges(unicoderanges.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(unicoderanges.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), actualrangecount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.base__.IsMonospacedFont())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphAdvances<P0>(&self, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetDesignGlyphAdvances(glyphcount, glyphindices, glyphadvances, issideways.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphAdvances<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, issideways: P1, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetGdiCompatibleGlyphAdvances(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), issideways.into_param().abi(), glyphcount, glyphindices, glyphadvances)).ok()
    }
    pub unsafe fn GetKerningPairAdjustments(&self, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetKerningPairAdjustments(glyphcount, glyphindices, glyphadvanceadjustments)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasKerningPairs(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.base__.HasKerningPairs())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode2<P0>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE) -> ::windows_core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetRecommendedRenderingMode2(fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into_param().abi(), outlinethreshold, measuringmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetVerticalGlyphVariants(&self, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetVerticalGlyphVariants(glyphcount, nominalglyphindices, verticalglyphindices)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVerticalGlyphVariants(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.base__.HasVerticalGlyphVariants())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsColorFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.IsColorFont())
    }
    pub unsafe fn GetColorPaletteCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetColorPaletteCount())
    }
    pub unsafe fn GetPaletteEntryCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetPaletteEntryCount())
    }
    pub unsafe fn GetPaletteEntries(&self, colorpaletteindex: u32, firstentryindex: u32, paletteentries: &mut [DWRITE_COLOR_F]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetPaletteEntries(colorpaletteindex, firstentryindex, paletteentries.len().try_into().unwrap(), ::core::mem::transmute(paletteentries.as_ptr()))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode3<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<IDWriteRenderingParams>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.GetRecommendedRenderingMode3(fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into_param().abi(), outlinethreshold, measuringmode, renderingparams.into_param().abi(), renderingmode, gridfitmode)).ok()
    }
    pub unsafe fn GetFontFaceReference(&self) -> ::windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFontFaceReference(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetPanose(&self) -> DWRITE_PANOSE {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetPanose(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        ::windows_core::vcall!(self.base__.base__.GetWeight())
    }
    pub unsafe fn GetStretch(&self) -> DWRITE_FONT_STRETCH {
        ::windows_core::vcall!(self.base__.base__.GetStretch())
    }
    pub unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        ::windows_core::vcall!(self.base__.base__.GetStyle())
    }
    pub unsafe fn GetFamilyNames(&self) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFamilyNames(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFaceNames(&self) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFaceNames(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetInformationalStrings(informationalstringid, ::core::mem::transmute(informationalstrings), exists)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.HasCharacter(unicodevalue))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode4<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<IDWriteRenderingParams>,
    {
        ::windows_core::vcall!(self.base__.base__.GetRecommendedRenderingMode4(fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into_param().abi(), outlinethreshold, measuringmode, renderingparams.into_param().abi(), renderingmode, gridfitmode)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCharacterLocal(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.IsCharacterLocal(unicodevalue))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsGlyphLocal(&self, glyphid: u16) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.IsGlyphLocal(glyphid))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AreCharactersLocal<P0>(&self, characters: &[u16], enqueueifnotlocal: P0) -> ::windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.AreCharactersLocal(::core::mem::transmute(characters.as_ptr()), characters.len().try_into().unwrap(), enqueueifnotlocal.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AreGlyphsLocal<P0>(&self, glyphindices: &[u16], enqueueifnotlocal: P0) -> ::windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.AreGlyphsLocal(::core::mem::transmute(glyphindices.as_ptr()), glyphindices.len().try_into().unwrap(), enqueueifnotlocal.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGlyphImageFormats(&self, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32) -> ::windows_core::Result<DWRITE_GLYPH_IMAGE_FORMATS> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetGlyphImageFormats(glyphid, pixelsperemfirst, pixelsperemlast, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGlyphImageFormats2(&self) -> DWRITE_GLYPH_IMAGE_FORMATS {
        ::windows_core::vcall!(self.base__.GetGlyphImageFormats2())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphImageData(&self, glyphid: u16, pixelsperem: u32, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetGlyphImageData(glyphid, pixelsperem, glyphimageformat, glyphdata, glyphdatacontext)).ok()
    }
    pub unsafe fn ReleaseGlyphImageData(&self, glyphdatacontext: *mut ::core::ffi::c_void) {
        ::windows_core::vcall!(self.base__.ReleaseGlyphImageData(glyphdatacontext))
    }
    pub unsafe fn GetFontAxisValueCount(&self) -> u32 {
        ::windows_core::vcall!(self.GetFontAxisValueCount())
    }
    pub unsafe fn GetFontAxisValues(&self, fontaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFontAxisValues(::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVariations(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.HasVariations())
    }
    pub unsafe fn GetFontResource(&self) -> ::windows_core::Result<IDWriteFontResource> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontResource(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Equals<P0>(&self, fontface: P0) -> super::super::Foundation::BOOL
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        ::windows_core::vcall!(self.Equals(fontface.into_param().abi()))
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFace5, ::windows_core::IUnknown, IDWriteFontFace, IDWriteFontFace1, IDWriteFontFace2, IDWriteFontFace3, IDWriteFontFace4);
unsafe impl ::core::marker::Send for IDWriteFontFace5 {}
unsafe impl ::core::marker::Sync for IDWriteFontFace5 {}
unsafe impl ::windows_core::Interface for IDWriteFontFace5 {
    type Vtable = IDWriteFontFace5_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFace5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98eff3a5_b667_479a_b145_e2fa5b9fdc29);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFace5_Vtbl {
    pub base__: IDWriteFontFace4_Vtbl,
    pub GetFontAxisValueCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetFontAxisValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasVariations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasVariations: usize,
    pub GetFontResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Equals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    Equals: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFace6(::windows_core::IUnknown);
impl IDWriteFontFace6 {
    pub unsafe fn GetType(&self) -> DWRITE_FONT_FACE_TYPE {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.GetType())
    }
    pub unsafe fn GetFiles(&self, numberoffiles: *mut u32, fontfiles: ::core::option::Option<*mut ::core::option::Option<IDWriteFontFile>>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.GetFiles(numberoffiles, ::core::mem::transmute(fontfiles.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetIndex(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.GetIndex())
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.GetSimulations())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSymbolFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.IsSymbolFont())
    }
    pub unsafe fn GetMetrics(&self, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.GetMetrics(fontfacemetrics))
    }
    pub unsafe fn GetGlyphCount(&self) -> u16 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.GetGlyphCount())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphMetrics<P0>(&self, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.GetDesignGlyphMetrics(glyphindices, glyphcount, glyphmetrics, issideways.into_param().abi())).ok()
    }
    pub unsafe fn GetGlyphIndices(&self, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.GetGlyphIndices(codepoints, codepointcount, glyphindices)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TryGetFontTable(&self, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.TryGetFontTable(opentypetabletag, tabledata, tablesize, tablecontext, exists)).ok()
    }
    pub unsafe fn ReleaseFontTable(&self, tablecontext: *const ::core::ffi::c_void) {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.ReleaseFontTable(tablecontext))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphRunOutline<P0, P1, P2>(&self, emsize: f32, glyphindices: *const u16, glyphadvances: ::core::option::Option<*const f32>, glyphoffsets: ::core::option::Option<*const DWRITE_GLYPH_OFFSET>, glyphcount: u32, issideways: P0, isrighttoleft: P1, geometrysink: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::Direct2D::Common::ID2D1SimplifiedGeometrySink>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.GetGlyphRunOutline(emsize, glyphindices, ::core::mem::transmute(glyphadvances.unwrap_or(::std::ptr::null())), ::core::mem::transmute(glyphoffsets.unwrap_or(::std::ptr::null())), glyphcount, issideways.into_param().abi(), isrighttoleft.into_param().abi(), geometrysink.into_param().abi())).ok()
    }
    pub unsafe fn GetRecommendedRenderingMode<P0>(&self, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P0) -> ::windows_core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::windows_core::IntoParam<IDWriteRenderingParams>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.GetRecommendedRenderingMode(emsize, pixelsperdip, measuringmode, renderingparams.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGdiCompatibleMetrics(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.GetGdiCompatibleMetrics(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontfacemetrics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphMetrics<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.base__.GetGdiCompatibleGlyphMetrics(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), glyphindices, glyphcount, glyphmetrics, issideways.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics2(&self, fontmetrics: *mut DWRITE_FONT_METRICS1) {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetMetrics2(fontmetrics))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleMetrics2(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetGdiCompatibleMetrics2(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), fontmetrics)).ok()
    }
    pub unsafe fn GetCaretMetrics(&self) -> DWRITE_CARET_METRICS {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetCaretMetrics(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn GetUnicodeRanges(&self, unicoderanges: ::core::option::Option<&mut [DWRITE_UNICODE_RANGE]>, actualrangecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetUnicodeRanges(unicoderanges.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(unicoderanges.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), actualrangecount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsMonospacedFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.IsMonospacedFont())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesignGlyphAdvances<P0>(&self, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetDesignGlyphAdvances(glyphcount, glyphindices, glyphadvances, issideways.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphAdvances<P0, P1>(&self, emsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P0, issideways: P1, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetGdiCompatibleGlyphAdvances(emsize, pixelsperdip, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), usegdinatural.into_param().abi(), issideways.into_param().abi(), glyphcount, glyphindices, glyphadvances)).ok()
    }
    pub unsafe fn GetKerningPairAdjustments(&self, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetKerningPairAdjustments(glyphcount, glyphindices, glyphadvanceadjustments)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasKerningPairs(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.HasKerningPairs())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode2<P0>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE) -> ::windows_core::Result<DWRITE_RENDERING_MODE>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetRecommendedRenderingMode2(fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into_param().abi(), outlinethreshold, measuringmode, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetVerticalGlyphVariants(&self, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetVerticalGlyphVariants(glyphcount, nominalglyphindices, verticalglyphindices)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVerticalGlyphVariants(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.HasVerticalGlyphVariants())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsColorFont(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.base__.IsColorFont())
    }
    pub unsafe fn GetColorPaletteCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetColorPaletteCount())
    }
    pub unsafe fn GetPaletteEntryCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetPaletteEntryCount())
    }
    pub unsafe fn GetPaletteEntries(&self, colorpaletteindex: u32, firstentryindex: u32, paletteentries: &mut [DWRITE_COLOR_F]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetPaletteEntries(colorpaletteindex, firstentryindex, paletteentries.len().try_into().unwrap(), ::core::mem::transmute(paletteentries.as_ptr()))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode3<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<IDWriteRenderingParams>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetRecommendedRenderingMode3(fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into_param().abi(), outlinethreshold, measuringmode, renderingparams.into_param().abi(), renderingmode, gridfitmode)).ok()
    }
    pub unsafe fn GetFontFaceReference(&self) -> ::windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFontFaceReference(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetPanose(&self) -> DWRITE_PANOSE {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetPanose(&mut result__));
        ::std::mem::transmute(result__)
    }
    pub unsafe fn GetWeight(&self) -> DWRITE_FONT_WEIGHT {
        ::windows_core::vcall!(self.base__.base__.base__.GetWeight())
    }
    pub unsafe fn GetStretch(&self) -> DWRITE_FONT_STRETCH {
        ::windows_core::vcall!(self.base__.base__.base__.GetStretch())
    }
    pub unsafe fn GetStyle(&self) -> DWRITE_FONT_STYLE {
        ::windows_core::vcall!(self.base__.base__.base__.GetStyle())
    }
    pub unsafe fn GetFamilyNames(&self) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFamilyNames(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFaceNames(&self) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFaceNames(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInformationalStrings(&self, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetInformationalStrings(informationalstringid, ::core::mem::transmute(informationalstrings), exists)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasCharacter(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.HasCharacter(unicodevalue))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRecommendedRenderingMode4<P0, P1>(&self, fontemsize: f32, dpix: f32, dpiy: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, issideways: P0, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: P1, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<IDWriteRenderingParams>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.GetRecommendedRenderingMode4(fontemsize, dpix, dpiy, ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())), issideways.into_param().abi(), outlinethreshold, measuringmode, renderingparams.into_param().abi(), renderingmode, gridfitmode)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCharacterLocal(&self, unicodevalue: u32) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.IsCharacterLocal(unicodevalue))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsGlyphLocal(&self, glyphid: u16) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.base__.IsGlyphLocal(glyphid))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AreCharactersLocal<P0>(&self, characters: &[u16], enqueueifnotlocal: P0) -> ::windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.AreCharactersLocal(::core::mem::transmute(characters.as_ptr()), characters.len().try_into().unwrap(), enqueueifnotlocal.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AreGlyphsLocal<P0>(&self, glyphindices: &[u16], enqueueifnotlocal: P0) -> ::windows_core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.AreGlyphsLocal(::core::mem::transmute(glyphindices.as_ptr()), glyphindices.len().try_into().unwrap(), enqueueifnotlocal.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGlyphImageFormats(&self, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32) -> ::windows_core::Result<DWRITE_GLYPH_IMAGE_FORMATS> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetGlyphImageFormats(glyphid, pixelsperemfirst, pixelsperemlast, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetGlyphImageFormats2(&self) -> DWRITE_GLYPH_IMAGE_FORMATS {
        ::windows_core::vcall!(self.base__.base__.GetGlyphImageFormats2())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
    pub unsafe fn GetGlyphImageData(&self, glyphid: u16, pixelsperem: u32, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetGlyphImageData(glyphid, pixelsperem, glyphimageformat, glyphdata, glyphdatacontext)).ok()
    }
    pub unsafe fn ReleaseGlyphImageData(&self, glyphdatacontext: *mut ::core::ffi::c_void) {
        ::windows_core::vcall!(self.base__.base__.ReleaseGlyphImageData(glyphdatacontext))
    }
    pub unsafe fn GetFontAxisValueCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.GetFontAxisValueCount())
    }
    pub unsafe fn GetFontAxisValues(&self, fontaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetFontAxisValues(::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVariations(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.HasVariations())
    }
    pub unsafe fn GetFontResource(&self) -> ::windows_core::Result<IDWriteFontResource> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontResource(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Equals<P0>(&self, fontface: P0) -> super::super::Foundation::BOOL
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        ::windows_core::vcall!(self.base__.Equals(fontface.into_param().abi()))
    }
    pub unsafe fn GetFamilyNames2(&self, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFamilyNames2(fontfamilymodel, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFaceNames2(&self, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFaceNames2(fontfamilymodel, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFace6, ::windows_core::IUnknown, IDWriteFontFace, IDWriteFontFace1, IDWriteFontFace2, IDWriteFontFace3, IDWriteFontFace4, IDWriteFontFace5);
unsafe impl ::core::marker::Send for IDWriteFontFace6 {}
unsafe impl ::core::marker::Sync for IDWriteFontFace6 {}
unsafe impl ::windows_core::Interface for IDWriteFontFace6 {
    type Vtable = IDWriteFontFace6_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFace6 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4b1fe1b_6e84_47d5_b54c_a597981b06ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFace6_Vtbl {
    pub base__: IDWriteFontFace5_Vtbl,
    pub GetFamilyNames2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, names: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFaceNames2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, names: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFaceReference(::windows_core::IUnknown);
impl IDWriteFontFaceReference {
    pub unsafe fn CreateFontFace(&self) -> ::windows_core::Result<IDWriteFontFace3> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFace(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFaceWithSimulations(&self, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFace3> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFaceWithSimulations(fontfacesimulationflags, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Equals<P0>(&self, fontfacereference: P0) -> super::super::Foundation::BOOL
    where
        P0: ::windows_core::IntoParam<IDWriteFontFaceReference>,
    {
        ::windows_core::vcall!(self.Equals(fontfacereference.into_param().abi()))
    }
    pub unsafe fn GetFontFaceIndex(&self) -> u32 {
        ::windows_core::vcall!(self.GetFontFaceIndex())
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        ::windows_core::vcall!(self.GetSimulations())
    }
    pub unsafe fn GetFontFile(&self) -> ::windows_core::Result<IDWriteFontFile> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontFile(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetLocalFileSize(&self) -> u64 {
        ::windows_core::vcall!(self.GetLocalFileSize())
    }
    pub unsafe fn GetFileSize(&self) -> u64 {
        ::windows_core::vcall!(self.GetFileSize())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileTime(&self) -> ::windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFileTime(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetLocality(&self) -> DWRITE_LOCALITY {
        ::windows_core::vcall!(self.GetLocality())
    }
    pub unsafe fn EnqueueFontDownloadRequest(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.EnqueueFontDownloadRequest()).ok()
    }
    pub unsafe fn EnqueueCharacterDownloadRequest(&self, characters: &[u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.EnqueueCharacterDownloadRequest(::core::mem::transmute(characters.as_ptr()), characters.len().try_into().unwrap())).ok()
    }
    pub unsafe fn EnqueueGlyphDownloadRequest(&self, glyphindices: &[u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.EnqueueGlyphDownloadRequest(::core::mem::transmute(glyphindices.as_ptr()), glyphindices.len().try_into().unwrap())).ok()
    }
    pub unsafe fn EnqueueFileFragmentDownloadRequest(&self, fileoffset: u64, fragmentsize: u64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.EnqueueFileFragmentDownloadRequest(fileoffset, fragmentsize)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFaceReference, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFontFaceReference {}
unsafe impl ::core::marker::Sync for IDWriteFontFaceReference {}
unsafe impl ::windows_core::Interface for IDWriteFontFaceReference {
    type Vtable = IDWriteFontFaceReference_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFaceReference {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e7fa7ca_dde3_424c_89f0_9fcd6fed58cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFaceReference_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateFontFace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFontFaceWithSimulations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Equals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfacereference: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    Equals: usize,
    pub GetFontFaceIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetSimulations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS,
    pub GetFontFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLocalFileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub GetFileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFileTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastwritetime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFileTime: usize,
    pub GetLocality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_LOCALITY,
    pub EnqueueFontDownloadRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EnqueueCharacterDownloadRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, characters: ::windows_core::PCWSTR, charactercount: u32) -> ::windows_core::HRESULT,
    pub EnqueueGlyphDownloadRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphindices: *const u16, glyphcount: u32) -> ::windows_core::HRESULT,
    pub EnqueueFileFragmentDownloadRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFaceReference1(::windows_core::IUnknown);
impl IDWriteFontFaceReference1 {
    pub unsafe fn CreateFontFace(&self) -> ::windows_core::Result<IDWriteFontFace3> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontFace(&mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFaceWithSimulations(&self, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFace3> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontFaceWithSimulations(fontfacesimulationflags, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Equals<P0>(&self, fontfacereference: P0) -> super::super::Foundation::BOOL
    where
        P0: ::windows_core::IntoParam<IDWriteFontFaceReference>,
    {
        ::windows_core::vcall!(self.base__.Equals(fontfacereference.into_param().abi()))
    }
    pub unsafe fn GetFontFaceIndex(&self) -> u32 {
        ::windows_core::vcall!(self.base__.GetFontFaceIndex())
    }
    pub unsafe fn GetSimulations(&self) -> DWRITE_FONT_SIMULATIONS {
        ::windows_core::vcall!(self.base__.GetSimulations())
    }
    pub unsafe fn GetFontFile(&self) -> ::windows_core::Result<IDWriteFontFile> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontFile(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetLocalFileSize(&self) -> u64 {
        ::windows_core::vcall!(self.base__.GetLocalFileSize())
    }
    pub unsafe fn GetFileSize(&self) -> u64 {
        ::windows_core::vcall!(self.base__.GetFileSize())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileTime(&self) -> ::windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFileTime(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetLocality(&self) -> DWRITE_LOCALITY {
        ::windows_core::vcall!(self.base__.GetLocality())
    }
    pub unsafe fn EnqueueFontDownloadRequest(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.EnqueueFontDownloadRequest()).ok()
    }
    pub unsafe fn EnqueueCharacterDownloadRequest(&self, characters: &[u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.EnqueueCharacterDownloadRequest(::core::mem::transmute(characters.as_ptr()), characters.len().try_into().unwrap())).ok()
    }
    pub unsafe fn EnqueueGlyphDownloadRequest(&self, glyphindices: &[u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.EnqueueGlyphDownloadRequest(::core::mem::transmute(glyphindices.as_ptr()), glyphindices.len().try_into().unwrap())).ok()
    }
    pub unsafe fn EnqueueFileFragmentDownloadRequest(&self, fileoffset: u64, fragmentsize: u64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.EnqueueFileFragmentDownloadRequest(fileoffset, fragmentsize)).ok()
    }
    pub unsafe fn CreateFontFace2(&self) -> ::windows_core::Result<IDWriteFontFace5> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFace2(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontAxisValueCount(&self) -> u32 {
        ::windows_core::vcall!(self.GetFontAxisValueCount())
    }
    pub unsafe fn GetFontAxisValues(&self, fontaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFontAxisValues(::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFaceReference1, ::windows_core::IUnknown, IDWriteFontFaceReference);
unsafe impl ::core::marker::Send for IDWriteFontFaceReference1 {}
unsafe impl ::core::marker::Sync for IDWriteFontFaceReference1 {}
unsafe impl ::windows_core::Interface for IDWriteFontFaceReference1 {
    type Vtable = IDWriteFontFaceReference1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFaceReference1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc081fe77_2fd1_41ac_a5a3_34983c4ba61a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFaceReference1_Vtbl {
    pub base__: IDWriteFontFaceReference_Vtbl,
    pub CreateFontFace2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFontAxisValueCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetFontAxisValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFallback(::windows_core::IUnknown);
impl IDWriteFontFallback {
    pub unsafe fn MapCharacters<P0, P1, P2>(&self, analysissource: P0, textposition: u32, textlength: u32, basefontcollection: P1, basefamilyname: P2, baseweight: DWRITE_FONT_WEIGHT, basestyle: DWRITE_FONT_STYLE, basestretch: DWRITE_FONT_STRETCH, mappedlength: *mut u32, mappedfont: *mut ::core::option::Option<IDWriteFont>, scale: *mut f32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextAnalysisSource>,
        P1: ::windows_core::IntoParam<IDWriteFontCollection>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.MapCharacters(analysissource.into_param().abi(), textposition, textlength, basefontcollection.into_param().abi(), basefamilyname.into_param().abi(), baseweight, basestyle, basestretch, mappedlength, ::core::mem::transmute(mappedfont), scale)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFallback, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFontFallback {}
unsafe impl ::core::marker::Sync for IDWriteFontFallback {}
unsafe impl ::windows_core::Interface for IDWriteFontFallback {
    type Vtable = IDWriteFontFallback_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFallback {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xefa008f9_f7a1_48bf_b05c_f224713cc0ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFallback_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub MapCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, basefontcollection: *mut ::core::ffi::c_void, basefamilyname: ::windows_core::PCWSTR, baseweight: DWRITE_FONT_WEIGHT, basestyle: DWRITE_FONT_STYLE, basestretch: DWRITE_FONT_STRETCH, mappedlength: *mut u32, mappedfont: *mut *mut ::core::ffi::c_void, scale: *mut f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFallback1(::windows_core::IUnknown);
impl IDWriteFontFallback1 {
    pub unsafe fn MapCharacters<P0, P1, P2>(&self, analysissource: P0, textposition: u32, textlength: u32, basefontcollection: P1, basefamilyname: P2, baseweight: DWRITE_FONT_WEIGHT, basestyle: DWRITE_FONT_STYLE, basestretch: DWRITE_FONT_STRETCH, mappedlength: *mut u32, mappedfont: *mut ::core::option::Option<IDWriteFont>, scale: *mut f32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextAnalysisSource>,
        P1: ::windows_core::IntoParam<IDWriteFontCollection>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.MapCharacters(analysissource.into_param().abi(), textposition, textlength, basefontcollection.into_param().abi(), basefamilyname.into_param().abi(), baseweight, basestyle, basestretch, mappedlength, ::core::mem::transmute(mappedfont), scale)).ok()
    }
    pub unsafe fn MapCharacters2<P0, P1, P2>(&self, analysissource: P0, textposition: u32, textlength: u32, basefontcollection: P1, basefamilyname: P2, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE], mappedlength: *mut u32, scale: *mut f32, mappedfontface: *mut ::core::option::Option<IDWriteFontFace5>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextAnalysisSource>,
        P1: ::windows_core::IntoParam<IDWriteFontCollection>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.MapCharacters2(analysissource.into_param().abi(), textposition, textlength, basefontcollection.into_param().abi(), basefamilyname.into_param().abi(), ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), mappedlength, scale, ::core::mem::transmute(mappedfontface))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFallback1, ::windows_core::IUnknown, IDWriteFontFallback);
unsafe impl ::core::marker::Send for IDWriteFontFallback1 {}
unsafe impl ::core::marker::Sync for IDWriteFontFallback1 {}
unsafe impl ::windows_core::Interface for IDWriteFontFallback1 {
    type Vtable = IDWriteFontFallback1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFallback1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2397599d_dd0d_4681_bd6a_f4f31eaade77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFallback1_Vtbl {
    pub base__: IDWriteFontFallback_Vtbl,
    pub MapCharacters2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, basefontcollection: *mut ::core::ffi::c_void, basefamilyname: ::windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, mappedlength: *mut u32, scale: *mut f32, mappedfontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFallbackBuilder(::windows_core::IUnknown);
impl IDWriteFontFallbackBuilder {
    pub unsafe fn AddMapping<P0, P1, P2>(&self, ranges: &[DWRITE_UNICODE_RANGE], targetfamilynames: &[*const u16], fontcollection: P0, localename: P1, basefamilyname: P2, scale: f32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollection>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.AddMapping(::core::mem::transmute(ranges.as_ptr()), ranges.len().try_into().unwrap(), ::core::mem::transmute(targetfamilynames.as_ptr()), targetfamilynames.len().try_into().unwrap(), fontcollection.into_param().abi(), localename.into_param().abi(), basefamilyname.into_param().abi(), scale)).ok()
    }
    pub unsafe fn AddMappings<P0>(&self, fontfallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFallback>,
    {
        ::windows_core::vcall!(self.AddMappings(fontfallback.into_param().abi())).ok()
    }
    pub unsafe fn CreateFontFallback(&self) -> ::windows_core::Result<IDWriteFontFallback> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFallback(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFallbackBuilder, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFontFallbackBuilder {}
unsafe impl ::core::marker::Sync for IDWriteFontFallbackBuilder {}
unsafe impl ::windows_core::Interface for IDWriteFontFallbackBuilder {
    type Vtable = IDWriteFontFallbackBuilder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFallbackBuilder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd882d06_8aba_4fb8_b849_8be8b73e14de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFallbackBuilder_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ranges: *const DWRITE_UNICODE_RANGE, rangescount: u32, targetfamilynames: *const *const u16, targetfamilynamescount: u32, fontcollection: *mut ::core::ffi::c_void, localename: ::windows_core::PCWSTR, basefamilyname: ::windows_core::PCWSTR, scale: f32) -> ::windows_core::HRESULT,
    pub AddMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFontFallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfallback: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFamily(::windows_core::IUnknown);
impl IDWriteFontFamily {
    pub unsafe fn GetFontCollection(&self) -> ::windows_core::Result<IDWriteFontCollection> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontCollection(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.GetFontCount())
    }
    pub unsafe fn GetFont(&self, index: u32) -> ::windows_core::Result<IDWriteFont> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFont(index, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFamilyNames(&self) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFamilyNames(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFirstMatchingFont(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> ::windows_core::Result<IDWriteFont> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFirstMatchingFont(weight, stretch, style, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> ::windows_core::Result<IDWriteFontList> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetMatchingFonts(weight, stretch, style, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFamily, ::windows_core::IUnknown, IDWriteFontList);
unsafe impl ::core::marker::Send for IDWriteFontFamily {}
unsafe impl ::core::marker::Sync for IDWriteFontFamily {}
unsafe impl ::windows_core::Interface for IDWriteFontFamily {
    type Vtable = IDWriteFontFamily_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFamily {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda20d8ef_812a_4c43_9802_62ec4abd7add);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFamily_Vtbl {
    pub base__: IDWriteFontList_Vtbl,
    pub GetFamilyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, names: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFirstMatchingFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE, matchingfont: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetMatchingFonts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE, matchingfonts: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFamily1(::windows_core::IUnknown);
impl IDWriteFontFamily1 {
    pub unsafe fn GetFontCollection(&self) -> ::windows_core::Result<IDWriteFontCollection> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFontCollection(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.GetFontCount())
    }
    pub unsafe fn GetFont(&self, index: u32) -> ::windows_core::Result<IDWriteFont> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFont(index, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFamilyNames(&self) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFamilyNames(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFirstMatchingFont(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> ::windows_core::Result<IDWriteFont> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFirstMatchingFont(weight, stretch, style, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> ::windows_core::Result<IDWriteFontList> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetMatchingFonts(weight, stretch, style, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        ::windows_core::vcall!(self.GetFontLocality(listindex))
    }
    pub unsafe fn GetFont2(&self, listindex: u32) -> ::windows_core::Result<IDWriteFont3> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFont2(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontFaceReference(listindex, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFamily1, ::windows_core::IUnknown, IDWriteFontList, IDWriteFontFamily);
unsafe impl ::core::marker::Send for IDWriteFontFamily1 {}
unsafe impl ::core::marker::Sync for IDWriteFontFamily1 {}
unsafe impl ::windows_core::Interface for IDWriteFontFamily1 {
    type Vtable = IDWriteFontFamily1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFamily1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda20d8ef_812a_4c43_9802_62ec4abd7adf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFamily1_Vtbl {
    pub base__: IDWriteFontFamily_Vtbl,
    pub GetFontLocality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY,
    pub GetFont2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32, font: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFontFaceReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFamily2(::windows_core::IUnknown);
impl IDWriteFontFamily2 {
    pub unsafe fn GetFontCollection(&self) -> ::windows_core::Result<IDWriteFontCollection> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFontCollection(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontCount())
    }
    pub unsafe fn GetFont(&self, index: u32) -> ::windows_core::Result<IDWriteFont> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFont(index, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFamilyNames(&self) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFamilyNames(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFirstMatchingFont(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> ::windows_core::Result<IDWriteFont> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFirstMatchingFont(weight, stretch, style, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts(&self, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> ::windows_core::Result<IDWriteFontList> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetMatchingFonts(weight, stretch, style, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        ::windows_core::vcall!(self.base__.GetFontLocality(listindex))
    }
    pub unsafe fn GetFont2(&self, listindex: u32) -> ::windows_core::Result<IDWriteFont3> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFont2(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontFaceReference(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts2(&self, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> ::windows_core::Result<IDWriteFontList2> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetMatchingFonts2(::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontSet(&self) -> ::windows_core::Result<IDWriteFontSet1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontSet(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFamily2, ::windows_core::IUnknown, IDWriteFontList, IDWriteFontFamily, IDWriteFontFamily1);
unsafe impl ::core::marker::Send for IDWriteFontFamily2 {}
unsafe impl ::core::marker::Sync for IDWriteFontFamily2 {}
unsafe impl ::windows_core::Interface for IDWriteFontFamily2 {
    type Vtable = IDWriteFontFamily2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFamily2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ed49e77_a398_4261_b9cf_c126c2131ef3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFamily2_Vtbl {
    pub base__: IDWriteFontFamily1_Vtbl,
    pub GetMatchingFonts2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, matchingfonts: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFontSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFile(::windows_core::IUnknown);
impl IDWriteFontFile {
    pub unsafe fn GetReferenceKey(&self, fontfilereferencekey: *mut *mut ::core::ffi::c_void, fontfilereferencekeysize: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetReferenceKey(fontfilereferencekey, fontfilereferencekeysize)).ok()
    }
    pub unsafe fn GetLoader(&self) -> ::windows_core::Result<IDWriteFontFileLoader> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetLoader(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Analyze(&self, issupportedfonttype: *mut super::super::Foundation::BOOL, fontfiletype: *mut DWRITE_FONT_FILE_TYPE, fontfacetype: ::core::option::Option<*mut DWRITE_FONT_FACE_TYPE>, numberoffaces: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.Analyze(issupportedfonttype, fontfiletype, ::core::mem::transmute(fontfacetype.unwrap_or(::std::ptr::null_mut())), numberoffaces)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFile, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFontFile {}
unsafe impl ::core::marker::Sync for IDWriteFontFile {}
unsafe impl ::windows_core::Interface for IDWriteFontFile {
    type Vtable = IDWriteFontFile_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFile {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x739d886a_cef5_47dc_8769_1a8b41bebbb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFile_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetReferenceKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfilereferencekey: *mut *mut ::core::ffi::c_void, fontfilereferencekeysize: *mut u32) -> ::windows_core::HRESULT,
    pub GetLoader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfileloader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Analyze: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issupportedfonttype: *mut super::super::Foundation::BOOL, fontfiletype: *mut DWRITE_FONT_FILE_TYPE, fontfacetype: *mut DWRITE_FONT_FACE_TYPE, numberoffaces: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Analyze: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFileEnumerator(::windows_core::IUnknown);
impl IDWriteFontFileEnumerator {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.MoveNext(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetCurrentFontFile(&self) -> ::windows_core::Result<IDWriteFontFile> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetCurrentFontFile(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFileEnumerator, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFontFileEnumerator {}
unsafe impl ::core::marker::Sync for IDWriteFontFileEnumerator {}
unsafe impl ::windows_core::Interface for IDWriteFontFileEnumerator {
    type Vtable = IDWriteFontFileEnumerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFileEnumerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72755049_5ff7_435d_8348_4be97cfa6c7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFileEnumerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrentfile: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
    pub GetCurrentFontFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFileLoader(::windows_core::IUnknown);
impl IDWriteFontFileLoader {
    pub unsafe fn CreateStreamFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows_core::Result<IDWriteFontFileStream> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateStreamFromKey(fontfilereferencekey, fontfilereferencekeysize, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFileLoader, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFontFileLoader {}
unsafe impl ::core::marker::Sync for IDWriteFontFileLoader {}
unsafe impl ::windows_core::Interface for IDWriteFontFileLoader {
    type Vtable = IDWriteFontFileLoader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFileLoader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x727cad4e_d6af_4c9e_8a08_d695b11caa49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFileLoader_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateStreamFromKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfilestream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontFileStream(::windows_core::IUnknown);
impl IDWriteFontFileStream {
    pub unsafe fn ReadFileFragment(&self, fragmentstart: *mut *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64, fragmentcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.ReadFileFragment(fragmentstart, fileoffset, fragmentsize, fragmentcontext)).ok()
    }
    pub unsafe fn ReleaseFileFragment(&self, fragmentcontext: *mut ::core::ffi::c_void) {
        ::windows_core::vcall!(self.ReleaseFileFragment(fragmentcontext))
    }
    pub unsafe fn GetFileSize(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFileSize(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetLastWriteTime(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetLastWriteTime(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontFileStream, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFontFileStream {}
unsafe impl ::core::marker::Sync for IDWriteFontFileStream {}
unsafe impl ::windows_core::Interface for IDWriteFontFileStream {
    type Vtable = IDWriteFontFileStream_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontFileStream {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d4865fe_0ab8_4d91_8f62_5dd6be34a3e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontFileStream_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ReadFileFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fragmentstart: *mut *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64, fragmentcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReleaseFileFragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fragmentcontext: *mut ::core::ffi::c_void),
    pub GetFileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows_core::HRESULT,
    pub GetLastWriteTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastwritetime: *mut u64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontList(::windows_core::IUnknown);
impl IDWriteFontList {
    pub unsafe fn GetFontCollection(&self) -> ::windows_core::Result<IDWriteFontCollection> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontCollection(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontCount(&self) -> u32 {
        ::windows_core::vcall!(self.GetFontCount())
    }
    pub unsafe fn GetFont(&self, index: u32) -> ::windows_core::Result<IDWriteFont> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFont(index, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontList, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFontList {}
unsafe impl ::core::marker::Sync for IDWriteFontList {}
unsafe impl ::windows_core::Interface for IDWriteFontList {
    type Vtable = IDWriteFontList_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1a0d8438_1d97_4ec1_aef9_a2fb86ed6acb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontList_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetFontCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFontCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, font: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontList1(::windows_core::IUnknown);
impl IDWriteFontList1 {
    pub unsafe fn GetFontCollection(&self) -> ::windows_core::Result<IDWriteFontCollection> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontCollection(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.GetFontCount())
    }
    pub unsafe fn GetFont(&self, index: u32) -> ::windows_core::Result<IDWriteFont> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFont(index, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        ::windows_core::vcall!(self.GetFontLocality(listindex))
    }
    pub unsafe fn GetFont2(&self, listindex: u32) -> ::windows_core::Result<IDWriteFont3> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFont2(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontFaceReference(listindex, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontList1, ::windows_core::IUnknown, IDWriteFontList);
unsafe impl ::core::marker::Send for IDWriteFontList1 {}
unsafe impl ::core::marker::Sync for IDWriteFontList1 {}
unsafe impl ::windows_core::Interface for IDWriteFontList1 {
    type Vtable = IDWriteFontList1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontList1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda20d8ef_812a_4c43_9802_62ec4abd7ade);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontList1_Vtbl {
    pub base__: IDWriteFontList_Vtbl,
    pub GetFontLocality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY,
    pub GetFont2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32, font: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFontFaceReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontList2(::windows_core::IUnknown);
impl IDWriteFontList2 {
    pub unsafe fn GetFontCollection(&self) -> ::windows_core::Result<IDWriteFontCollection> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFontCollection(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.GetFontCount())
    }
    pub unsafe fn GetFont(&self, index: u32) -> ::windows_core::Result<IDWriteFont> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFont(index, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        ::windows_core::vcall!(self.base__.GetFontLocality(listindex))
    }
    pub unsafe fn GetFont2(&self, listindex: u32) -> ::windows_core::Result<IDWriteFont3> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFont2(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontFaceReference(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontSet(&self) -> ::windows_core::Result<IDWriteFontSet1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontSet(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontList2, ::windows_core::IUnknown, IDWriteFontList, IDWriteFontList1);
unsafe impl ::core::marker::Send for IDWriteFontList2 {}
unsafe impl ::core::marker::Sync for IDWriteFontList2 {}
unsafe impl ::windows_core::Interface for IDWriteFontList2 {
    type Vtable = IDWriteFontList2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontList2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0763a34_77af_445a_b735_08c37b0a5bf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontList2_Vtbl {
    pub base__: IDWriteFontList1_Vtbl,
    pub GetFontSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontResource(::windows_core::IUnknown);
impl IDWriteFontResource {
    pub unsafe fn GetFontFile(&self) -> ::windows_core::Result<IDWriteFontFile> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontFile(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFaceIndex(&self) -> u32 {
        ::windows_core::vcall!(self.GetFontFaceIndex())
    }
    pub unsafe fn GetFontAxisCount(&self) -> u32 {
        ::windows_core::vcall!(self.GetFontAxisCount())
    }
    pub unsafe fn GetDefaultFontAxisValues(&self, fontaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetDefaultFontAxisValues(::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetFontAxisRanges(&self, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFontAxisRanges(::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetFontAxisAttributes(&self, axisindex: u32) -> DWRITE_FONT_AXIS_ATTRIBUTES {
        ::windows_core::vcall!(self.GetFontAxisAttributes(axisindex))
    }
    pub unsafe fn GetAxisNames(&self, axisindex: u32) -> ::windows_core::Result<IDWriteLocalizedStrings> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetAxisNames(axisindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetAxisValueNameCount(&self, axisindex: u32) -> u32 {
        ::windows_core::vcall!(self.GetAxisValueNameCount(axisindex))
    }
    pub unsafe fn GetAxisValueNames(&self, axisindex: u32, axisvalueindex: u32, fontaxisrange: *mut DWRITE_FONT_AXIS_RANGE, names: *mut ::core::option::Option<IDWriteLocalizedStrings>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetAxisValueNames(axisindex, axisvalueindex, fontaxisrange, ::core::mem::transmute(names))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasVariations(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.HasVariations())
    }
    pub unsafe fn CreateFontFace(&self, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> ::windows_core::Result<IDWriteFontFace5> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFace(fontsimulations, ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFaceReference(&self, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> ::windows_core::Result<IDWriteFontFaceReference1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFaceReference(fontsimulations, ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontResource, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFontResource {}
unsafe impl ::core::marker::Sync for IDWriteFontResource {}
unsafe impl ::windows_core::Interface for IDWriteFontResource {
    type Vtable = IDWriteFontResource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontResource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f803a76_6871_48e8_987f_b975551c50f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontResource_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetFontFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFontFaceIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetFontAxisCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetDefaultFontAxisValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::HRESULT,
    pub GetFontAxisRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32) -> ::windows_core::HRESULT,
    pub GetFontAxisAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, axisindex: u32) -> DWRITE_FONT_AXIS_ATTRIBUTES,
    pub GetAxisNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, axisindex: u32, names: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetAxisValueNameCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, axisindex: u32) -> u32,
    pub GetAxisValueNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, axisindex: u32, axisvalueindex: u32, fontaxisrange: *mut DWRITE_FONT_AXIS_RANGE, names: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasVariations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasVariations: usize,
    pub CreateFontFace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFontFaceReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontSet(::windows_core::IUnknown);
impl IDWriteFontSet {
    pub unsafe fn GetFontCount(&self) -> u32 {
        ::windows_core::vcall!(self.GetFontCount())
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontFaceReference(listindex, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFontFaceReference<P0>(&self, fontfacereference: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFaceReference>,
    {
        ::windows_core::vcall!(self.FindFontFaceReference(fontfacereference.into_param().abi(), listindex, exists)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFontFace<P0>(&self, fontface: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        ::windows_core::vcall!(self.FindFontFace(fontface.into_param().abi(), listindex, exists)).ok()
    }
    pub unsafe fn GetPropertyValues(&self, propertyid: DWRITE_FONT_PROPERTY_ID) -> ::windows_core::Result<IDWriteStringList> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetPropertyValues(propertyid, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetPropertyValues2<P0>(&self, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: P0) -> ::windows_core::Result<IDWriteStringList>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetPropertyValues2(propertyid, preferredlocalenames.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyValues3(&self, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut ::core::option::Option<IDWriteLocalizedStrings>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetPropertyValues3(listindex, propertyid, exists, ::core::mem::transmute(values))).ok()
    }
    pub unsafe fn GetPropertyOccurrenceCount(&self, property: *const DWRITE_FONT_PROPERTY) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetPropertyOccurrenceCount(property, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts<P0>(&self, familyname: P0, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE) -> ::windows_core::Result<IDWriteFontSet>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetMatchingFonts(familyname.into_param().abi(), fontweight, fontstretch, fontstyle, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts2(&self, properties: &[DWRITE_FONT_PROPERTY]) -> ::windows_core::Result<IDWriteFontSet> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetMatchingFonts2(::core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontSet, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFontSet {}
unsafe impl ::core::marker::Sync for IDWriteFontSet {}
unsafe impl ::windows_core::Interface for IDWriteFontSet {
    type Vtable = IDWriteFontSet_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontSet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53585141_d9f8_4095_8321_d73cf6bd116b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontSet_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetFontCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetFontFaceReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindFontFaceReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfacereference: *mut ::core::ffi::c_void, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindFontFaceReference: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FindFontFace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindFontFace: usize,
    pub GetPropertyValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: DWRITE_FONT_PROPERTY_ID, values: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPropertyValues2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: ::windows_core::PCWSTR, values: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyValues3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyValues3: usize,
    pub GetPropertyOccurrenceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, property: *const DWRITE_FONT_PROPERTY, propertyoccurrencecount: *mut u32) -> ::windows_core::HRESULT,
    pub GetMatchingFonts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, familyname: ::windows_core::PCWSTR, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE, filteredset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetMatchingFonts2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, filteredset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontSet1(::windows_core::IUnknown);
impl IDWriteFontSet1 {
    pub unsafe fn GetFontCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.GetFontCount())
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontFaceReference(listindex, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFontFaceReference<P0>(&self, fontfacereference: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFaceReference>,
    {
        ::windows_core::vcall!(self.base__.FindFontFaceReference(fontfacereference.into_param().abi(), listindex, exists)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFontFace<P0>(&self, fontface: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        ::windows_core::vcall!(self.base__.FindFontFace(fontface.into_param().abi(), listindex, exists)).ok()
    }
    pub unsafe fn GetPropertyValues(&self, propertyid: DWRITE_FONT_PROPERTY_ID) -> ::windows_core::Result<IDWriteStringList> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetPropertyValues(propertyid, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetPropertyValues2<P0>(&self, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: P0) -> ::windows_core::Result<IDWriteStringList>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetPropertyValues2(propertyid, preferredlocalenames.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyValues3(&self, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut ::core::option::Option<IDWriteLocalizedStrings>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetPropertyValues3(listindex, propertyid, exists, ::core::mem::transmute(values))).ok()
    }
    pub unsafe fn GetPropertyOccurrenceCount(&self, property: *const DWRITE_FONT_PROPERTY) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetPropertyOccurrenceCount(property, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts<P0>(&self, familyname: P0, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE) -> ::windows_core::Result<IDWriteFontSet>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetMatchingFonts(familyname.into_param().abi(), fontweight, fontstretch, fontstyle, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts2(&self, properties: &[DWRITE_FONT_PROPERTY]) -> ::windows_core::Result<IDWriteFontSet> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetMatchingFonts2(::core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts3(&self, fontproperty: ::core::option::Option<*const DWRITE_FONT_PROPERTY>, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> ::windows_core::Result<IDWriteFontSet1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetMatchingFonts3(::core::mem::transmute(fontproperty.unwrap_or(::std::ptr::null())), ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFirstFontResources(&self) -> ::windows_core::Result<IDWriteFontSet1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFirstFontResources(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFilteredFonts(&self, indices: &[u32]) -> ::windows_core::Result<IDWriteFontSet1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFilteredFonts(::core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFonts2<P0>(&self, fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], selectanyrange: P0) -> ::windows_core::Result<IDWriteFontSet1>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFilteredFonts2(::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), selectanyrange.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFonts3<P0>(&self, properties: ::core::option::Option<&[DWRITE_FONT_PROPERTY]>, selectanyproperty: P0) -> ::windows_core::Result<IDWriteFontSet1>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFilteredFonts3(::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), selectanyproperty.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFontIndices<P0>(&self, fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], selectanyrange: P0, indices: &mut [u32], actualindexcount: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.GetFilteredFontIndices(::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), selectanyrange.into_param().abi(), ::core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), actualindexcount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFontIndices2<P0>(&self, properties: &[DWRITE_FONT_PROPERTY], selectanyproperty: P0, indices: &mut [u32], actualindexcount: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.GetFilteredFontIndices2(::core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap(), selectanyproperty.into_param().abi(), ::core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), actualindexcount)).ok()
    }
    pub unsafe fn GetFontAxisRanges(&self, listindex: u32, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE], actualfontaxisrangecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFontAxisRanges(listindex, ::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), actualfontaxisrangecount)).ok()
    }
    pub unsafe fn GetFontAxisRanges2(&self, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE], actualfontaxisrangecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFontAxisRanges2(::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), actualfontaxisrangecount)).ok()
    }
    pub unsafe fn GetFontFaceReference2(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontFaceReference1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontFaceReference2(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontResource(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontResource> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontResource(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontFace5> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFace(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        ::windows_core::vcall!(self.GetFontLocality(listindex))
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontSet1, ::windows_core::IUnknown, IDWriteFontSet);
unsafe impl ::core::marker::Send for IDWriteFontSet1 {}
unsafe impl ::core::marker::Sync for IDWriteFontSet1 {}
unsafe impl ::windows_core::Interface for IDWriteFontSet1 {
    type Vtable = IDWriteFontSet1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontSet1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e9fda85_6c92_4053_bc47_7ae3530db4d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontSet1_Vtbl {
    pub base__: IDWriteFontSet_Vtbl,
    pub GetMatchingFonts3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontproperty: *const DWRITE_FONT_PROPERTY, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, matchingfonts: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFirstFontResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filteredfontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFilteredFonts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indices: *const u32, indexcount: u32, filteredfontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFilteredFonts2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL, filteredfontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFilteredFonts2: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFilteredFonts3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL, filteredfontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFilteredFonts3: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFilteredFontIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFilteredFontIndices: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFilteredFontIndices2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFilteredFontIndices2: usize,
    pub GetFontAxisRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> ::windows_core::HRESULT,
    pub GetFontAxisRanges2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> ::windows_core::HRESULT,
    pub GetFontFaceReference2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFontResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFontFace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFontLocality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontSet2(::windows_core::IUnknown);
impl IDWriteFontSet2 {
    pub unsafe fn GetFontCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.GetFontCount())
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFontFaceReference(listindex, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFontFaceReference<P0>(&self, fontfacereference: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFaceReference>,
    {
        ::windows_core::vcall!(self.base__.base__.FindFontFaceReference(fontfacereference.into_param().abi(), listindex, exists)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFontFace<P0>(&self, fontface: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        ::windows_core::vcall!(self.base__.base__.FindFontFace(fontface.into_param().abi(), listindex, exists)).ok()
    }
    pub unsafe fn GetPropertyValues(&self, propertyid: DWRITE_FONT_PROPERTY_ID) -> ::windows_core::Result<IDWriteStringList> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetPropertyValues(propertyid, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetPropertyValues2<P0>(&self, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: P0) -> ::windows_core::Result<IDWriteStringList>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetPropertyValues2(propertyid, preferredlocalenames.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyValues3(&self, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut ::core::option::Option<IDWriteLocalizedStrings>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetPropertyValues3(listindex, propertyid, exists, ::core::mem::transmute(values))).ok()
    }
    pub unsafe fn GetPropertyOccurrenceCount(&self, property: *const DWRITE_FONT_PROPERTY) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetPropertyOccurrenceCount(property, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts<P0>(&self, familyname: P0, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE) -> ::windows_core::Result<IDWriteFontSet>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetMatchingFonts(familyname.into_param().abi(), fontweight, fontstretch, fontstyle, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts2(&self, properties: &[DWRITE_FONT_PROPERTY]) -> ::windows_core::Result<IDWriteFontSet> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetMatchingFonts2(::core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts3(&self, fontproperty: ::core::option::Option<*const DWRITE_FONT_PROPERTY>, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> ::windows_core::Result<IDWriteFontSet1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetMatchingFonts3(::core::mem::transmute(fontproperty.unwrap_or(::std::ptr::null())), ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFirstFontResources(&self) -> ::windows_core::Result<IDWriteFontSet1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFirstFontResources(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFilteredFonts(&self, indices: &[u32]) -> ::windows_core::Result<IDWriteFontSet1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFilteredFonts(::core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFonts2<P0>(&self, fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], selectanyrange: P0) -> ::windows_core::Result<IDWriteFontSet1>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFilteredFonts2(::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), selectanyrange.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFonts3<P0>(&self, properties: ::core::option::Option<&[DWRITE_FONT_PROPERTY]>, selectanyproperty: P0) -> ::windows_core::Result<IDWriteFontSet1>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFilteredFonts3(::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), selectanyproperty.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFontIndices<P0>(&self, fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], selectanyrange: P0, indices: &mut [u32], actualindexcount: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.GetFilteredFontIndices(::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), selectanyrange.into_param().abi(), ::core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), actualindexcount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFontIndices2<P0>(&self, properties: &[DWRITE_FONT_PROPERTY], selectanyproperty: P0, indices: &mut [u32], actualindexcount: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.GetFilteredFontIndices2(::core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap(), selectanyproperty.into_param().abi(), ::core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), actualindexcount)).ok()
    }
    pub unsafe fn GetFontAxisRanges(&self, listindex: u32, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE], actualfontaxisrangecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetFontAxisRanges(listindex, ::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), actualfontaxisrangecount)).ok()
    }
    pub unsafe fn GetFontAxisRanges2(&self, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE], actualfontaxisrangecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetFontAxisRanges2(::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), actualfontaxisrangecount)).ok()
    }
    pub unsafe fn GetFontFaceReference2(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontFaceReference1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontFaceReference2(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontResource(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontResource> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontResource(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontFace5> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontFace(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        ::windows_core::vcall!(self.base__.GetFontLocality(listindex))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExpirationEvent(&self) -> super::super::Foundation::HANDLE {
        ::windows_core::vcall!(self.GetExpirationEvent())
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontSet2, ::windows_core::IUnknown, IDWriteFontSet, IDWriteFontSet1);
unsafe impl ::core::marker::Send for IDWriteFontSet2 {}
unsafe impl ::core::marker::Sync for IDWriteFontSet2 {}
unsafe impl ::windows_core::Interface for IDWriteFontSet2 {
    type Vtable = IDWriteFontSet2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontSet2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc7ead19_e54c_43af_b2da_4e2b79ba3f7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontSet2_Vtbl {
    pub base__: IDWriteFontSet1_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetExpirationEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetExpirationEvent: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontSet3(::windows_core::IUnknown);
impl IDWriteFontSet3 {
    pub unsafe fn GetFontCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontCount())
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFontFaceReference(listindex, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFontFaceReference<P0>(&self, fontfacereference: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFaceReference>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.FindFontFaceReference(fontfacereference.into_param().abi(), listindex, exists)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFontFace<P0>(&self, fontface: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.FindFontFace(fontface.into_param().abi(), listindex, exists)).ok()
    }
    pub unsafe fn GetPropertyValues(&self, propertyid: DWRITE_FONT_PROPERTY_ID) -> ::windows_core::Result<IDWriteStringList> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetPropertyValues(propertyid, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetPropertyValues2<P0>(&self, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: P0) -> ::windows_core::Result<IDWriteStringList>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetPropertyValues2(propertyid, preferredlocalenames.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyValues3(&self, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut ::core::option::Option<IDWriteLocalizedStrings>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetPropertyValues3(listindex, propertyid, exists, ::core::mem::transmute(values))).ok()
    }
    pub unsafe fn GetPropertyOccurrenceCount(&self, property: *const DWRITE_FONT_PROPERTY) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetPropertyOccurrenceCount(property, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts<P0>(&self, familyname: P0, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE) -> ::windows_core::Result<IDWriteFontSet>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetMatchingFonts(familyname.into_param().abi(), fontweight, fontstretch, fontstyle, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts2(&self, properties: &[DWRITE_FONT_PROPERTY]) -> ::windows_core::Result<IDWriteFontSet> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetMatchingFonts2(::core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts3(&self, fontproperty: ::core::option::Option<*const DWRITE_FONT_PROPERTY>, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> ::windows_core::Result<IDWriteFontSet1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetMatchingFonts3(::core::mem::transmute(fontproperty.unwrap_or(::std::ptr::null())), ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFirstFontResources(&self) -> ::windows_core::Result<IDWriteFontSet1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFirstFontResources(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFilteredFonts(&self, indices: &[u32]) -> ::windows_core::Result<IDWriteFontSet1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFilteredFonts(::core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFonts2<P0>(&self, fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], selectanyrange: P0) -> ::windows_core::Result<IDWriteFontSet1>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFilteredFonts2(::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), selectanyrange.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFonts3<P0>(&self, properties: ::core::option::Option<&[DWRITE_FONT_PROPERTY]>, selectanyproperty: P0) -> ::windows_core::Result<IDWriteFontSet1>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFilteredFonts3(::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), selectanyproperty.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFontIndices<P0>(&self, fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], selectanyrange: P0, indices: &mut [u32], actualindexcount: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.GetFilteredFontIndices(::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), selectanyrange.into_param().abi(), ::core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), actualindexcount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFontIndices2<P0>(&self, properties: &[DWRITE_FONT_PROPERTY], selectanyproperty: P0, indices: &mut [u32], actualindexcount: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.GetFilteredFontIndices2(::core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap(), selectanyproperty.into_param().abi(), ::core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), actualindexcount)).ok()
    }
    pub unsafe fn GetFontAxisRanges(&self, listindex: u32, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE], actualfontaxisrangecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetFontAxisRanges(listindex, ::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), actualfontaxisrangecount)).ok()
    }
    pub unsafe fn GetFontAxisRanges2(&self, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE], actualfontaxisrangecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetFontAxisRanges2(::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), actualfontaxisrangecount)).ok()
    }
    pub unsafe fn GetFontFaceReference2(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontFaceReference1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFontFaceReference2(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontResource(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontResource> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateFontResource(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontFace5> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateFontFace(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        ::windows_core::vcall!(self.base__.base__.GetFontLocality(listindex))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExpirationEvent(&self) -> super::super::Foundation::HANDLE {
        ::windows_core::vcall!(self.base__.GetExpirationEvent())
    }
    pub unsafe fn GetFontSourceType(&self, fontindex: u32) -> DWRITE_FONT_SOURCE_TYPE {
        ::windows_core::vcall!(self.GetFontSourceType(fontindex))
    }
    pub unsafe fn GetFontSourceNameLength(&self, listindex: u32) -> u32 {
        ::windows_core::vcall!(self.GetFontSourceNameLength(listindex))
    }
    pub unsafe fn GetFontSourceName(&self, listindex: u32, stringbuffer: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFontSourceName(listindex, ::core::mem::transmute(stringbuffer.as_ptr()), stringbuffer.len().try_into().unwrap())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontSet3, ::windows_core::IUnknown, IDWriteFontSet, IDWriteFontSet1, IDWriteFontSet2);
unsafe impl ::core::marker::Send for IDWriteFontSet3 {}
unsafe impl ::core::marker::Sync for IDWriteFontSet3 {}
unsafe impl ::windows_core::Interface for IDWriteFontSet3 {
    type Vtable = IDWriteFontSet3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontSet3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c073ef2_a7f4_4045_8c32_8ab8ae640f90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontSet3_Vtbl {
    pub base__: IDWriteFontSet2_Vtbl,
    pub GetFontSourceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontindex: u32) -> DWRITE_FONT_SOURCE_TYPE,
    pub GetFontSourceNameLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32) -> u32,
    pub GetFontSourceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32, stringbuffer: ::windows_core::PWSTR, stringbuffersize: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontSet4(::windows_core::IUnknown);
impl IDWriteFontSet4 {
    pub unsafe fn GetFontCount(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFontCount())
    }
    pub unsafe fn GetFontFaceReference(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontFaceReference> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFontFaceReference(listindex, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFontFaceReference<P0>(&self, fontfacereference: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFaceReference>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.FindFontFaceReference(fontfacereference.into_param().abi(), listindex, exists)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindFontFace<P0>(&self, fontface: P0, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.FindFontFace(fontface.into_param().abi(), listindex, exists)).ok()
    }
    pub unsafe fn GetPropertyValues(&self, propertyid: DWRITE_FONT_PROPERTY_ID) -> ::windows_core::Result<IDWriteStringList> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetPropertyValues(propertyid, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetPropertyValues2<P0>(&self, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: P0) -> ::windows_core::Result<IDWriteStringList>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetPropertyValues2(propertyid, preferredlocalenames.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyValues3(&self, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut ::core::option::Option<IDWriteLocalizedStrings>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetPropertyValues3(listindex, propertyid, exists, ::core::mem::transmute(values))).ok()
    }
    pub unsafe fn GetPropertyOccurrenceCount(&self, property: *const DWRITE_FONT_PROPERTY) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetPropertyOccurrenceCount(property, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts<P0>(&self, familyname: P0, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE) -> ::windows_core::Result<IDWriteFontSet>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetMatchingFonts(familyname.into_param().abi(), fontweight, fontstretch, fontstyle, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts2(&self, properties: &[DWRITE_FONT_PROPERTY]) -> ::windows_core::Result<IDWriteFontSet> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetMatchingFonts2(::core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetMatchingFonts3(&self, fontproperty: ::core::option::Option<*const DWRITE_FONT_PROPERTY>, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> ::windows_core::Result<IDWriteFontSet1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetMatchingFonts3(::core::mem::transmute(fontproperty.unwrap_or(::std::ptr::null())), ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFirstFontResources(&self) -> ::windows_core::Result<IDWriteFontSet1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFirstFontResources(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFilteredFonts(&self, indices: &[u32]) -> ::windows_core::Result<IDWriteFontSet1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFilteredFonts(::core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFonts2<P0>(&self, fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], selectanyrange: P0) -> ::windows_core::Result<IDWriteFontSet1>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFilteredFonts2(::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), selectanyrange.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFonts3<P0>(&self, properties: ::core::option::Option<&[DWRITE_FONT_PROPERTY]>, selectanyproperty: P0) -> ::windows_core::Result<IDWriteFontSet1>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFilteredFonts3(::core::mem::transmute(properties.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), properties.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), selectanyproperty.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFontIndices<P0>(&self, fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], selectanyrange: P0, indices: &mut [u32], actualindexcount: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.GetFilteredFontIndices(::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), selectanyrange.into_param().abi(), ::core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), actualindexcount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilteredFontIndices2<P0>(&self, properties: &[DWRITE_FONT_PROPERTY], selectanyproperty: P0, indices: &mut [u32], actualindexcount: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.GetFilteredFontIndices2(::core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap(), selectanyproperty.into_param().abi(), ::core::mem::transmute(indices.as_ptr()), indices.len().try_into().unwrap(), actualindexcount)).ok()
    }
    pub unsafe fn GetFontAxisRanges(&self, listindex: u32, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE], actualfontaxisrangecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontAxisRanges(listindex, ::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), actualfontaxisrangecount)).ok()
    }
    pub unsafe fn GetFontAxisRanges2(&self, fontaxisranges: &mut [DWRITE_FONT_AXIS_RANGE], actualfontaxisrangecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontAxisRanges2(::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), actualfontaxisrangecount)).ok()
    }
    pub unsafe fn GetFontFaceReference2(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontFaceReference1> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFontFaceReference2(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontResource(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontResource> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateFontResource(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFace(&self, listindex: u32) -> ::windows_core::Result<IDWriteFontFace5> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.CreateFontFace(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontLocality(&self, listindex: u32) -> DWRITE_LOCALITY {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontLocality(listindex))
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExpirationEvent(&self) -> super::super::Foundation::HANDLE {
        ::windows_core::vcall!(self.base__.base__.GetExpirationEvent())
    }
    pub unsafe fn GetFontSourceType(&self, fontindex: u32) -> DWRITE_FONT_SOURCE_TYPE {
        ::windows_core::vcall!(self.base__.GetFontSourceType(fontindex))
    }
    pub unsafe fn GetFontSourceNameLength(&self, listindex: u32) -> u32 {
        ::windows_core::vcall!(self.base__.GetFontSourceNameLength(listindex))
    }
    pub unsafe fn GetFontSourceName(&self, listindex: u32, stringbuffer: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetFontSourceName(listindex, ::core::mem::transmute(stringbuffer.as_ptr()), stringbuffer.len().try_into().unwrap())).ok()
    }
    pub unsafe fn ConvertWeightStretchStyleToFontAxisValues(&self, inputaxisvalues: ::core::option::Option<&[DWRITE_FONT_AXIS_VALUE]>, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE, fontsize: f32, outputaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE; 5]) -> u32 {
        ::windows_core::vcall!(self.ConvertWeightStretchStyleToFontAxisValues(::core::mem::transmute(inputaxisvalues.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), inputaxisvalues.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), fontweight, fontstretch, fontstyle, fontsize, ::core::mem::transmute(outputaxisvalues.as_ptr())))
    }
    pub unsafe fn GetMatchingFonts4<P0>(&self, familyname: P0, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE], allowedsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontSet4>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetMatchingFonts4(familyname.into_param().abi(), ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), allowedsimulations, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontSet4, ::windows_core::IUnknown, IDWriteFontSet, IDWriteFontSet1, IDWriteFontSet2, IDWriteFontSet3);
unsafe impl ::core::marker::Send for IDWriteFontSet4 {}
unsafe impl ::core::marker::Sync for IDWriteFontSet4 {}
unsafe impl ::windows_core::Interface for IDWriteFontSet4 {
    type Vtable = IDWriteFontSet4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontSet4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeec175fc_bea9_4c86_8b53_ccbdd7df0c82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontSet4_Vtbl {
    pub base__: IDWriteFontSet3_Vtbl,
    pub ConvertWeightStretchStyleToFontAxisValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputaxisvalues: *const DWRITE_FONT_AXIS_VALUE, inputaxiscount: u32, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE, fontsize: f32, outputaxisvalues: *mut DWRITE_FONT_AXIS_VALUE) -> u32,
    pub GetMatchingFonts4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, familyname: ::windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, allowedsimulations: DWRITE_FONT_SIMULATIONS, matchingfonts: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontSetBuilder(::windows_core::IUnknown);
impl IDWriteFontSetBuilder {
    pub unsafe fn AddFontFaceReference<P0>(&self, fontfacereference: P0, properties: &[DWRITE_FONT_PROPERTY]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFaceReference>,
    {
        ::windows_core::vcall!(self.AddFontFaceReference(fontfacereference.into_param().abi(), ::core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap())).ok()
    }
    pub unsafe fn AddFontFaceReference2<P0>(&self, fontfacereference: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFaceReference>,
    {
        ::windows_core::vcall!(self.AddFontFaceReference2(fontfacereference.into_param().abi())).ok()
    }
    pub unsafe fn AddFontSet<P0>(&self, fontset: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontSet>,
    {
        ::windows_core::vcall!(self.AddFontSet(fontset.into_param().abi())).ok()
    }
    pub unsafe fn CreateFontSet(&self) -> ::windows_core::Result<IDWriteFontSet> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontSet(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontSetBuilder, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteFontSetBuilder {}
unsafe impl ::core::marker::Sync for IDWriteFontSetBuilder {}
unsafe impl ::windows_core::Interface for IDWriteFontSetBuilder {
    type Vtable = IDWriteFontSetBuilder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontSetBuilder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f642afe_9c68_4f40_b8be_457401afcb3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontSetBuilder_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddFontFaceReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfacereference: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows_core::HRESULT,
    pub AddFontFaceReference2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfacereference: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddFontSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontset: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFontSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontSetBuilder1(::windows_core::IUnknown);
impl IDWriteFontSetBuilder1 {
    pub unsafe fn AddFontFaceReference<P0>(&self, fontfacereference: P0, properties: &[DWRITE_FONT_PROPERTY]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFaceReference>,
    {
        ::windows_core::vcall!(self.base__.AddFontFaceReference(fontfacereference.into_param().abi(), ::core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap())).ok()
    }
    pub unsafe fn AddFontFaceReference2<P0>(&self, fontfacereference: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFaceReference>,
    {
        ::windows_core::vcall!(self.base__.AddFontFaceReference2(fontfacereference.into_param().abi())).ok()
    }
    pub unsafe fn AddFontSet<P0>(&self, fontset: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontSet>,
    {
        ::windows_core::vcall!(self.base__.AddFontSet(fontset.into_param().abi())).ok()
    }
    pub unsafe fn CreateFontSet(&self) -> ::windows_core::Result<IDWriteFontSet> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontSet(&mut result__)).from_abi(result__)
    }
    pub unsafe fn AddFontFile<P0>(&self, fontfile: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFile>,
    {
        ::windows_core::vcall!(self.AddFontFile(fontfile.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontSetBuilder1, ::windows_core::IUnknown, IDWriteFontSetBuilder);
unsafe impl ::core::marker::Send for IDWriteFontSetBuilder1 {}
unsafe impl ::core::marker::Sync for IDWriteFontSetBuilder1 {}
unsafe impl ::windows_core::Interface for IDWriteFontSetBuilder1 {
    type Vtable = IDWriteFontSetBuilder1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontSetBuilder1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3ff7715f_3cdc_4dc6_9b72_ec5621dccafd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontSetBuilder1_Vtbl {
    pub base__: IDWriteFontSetBuilder_Vtbl,
    pub AddFontFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfile: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteFontSetBuilder2(::windows_core::IUnknown);
impl IDWriteFontSetBuilder2 {
    pub unsafe fn AddFontFaceReference<P0>(&self, fontfacereference: P0, properties: &[DWRITE_FONT_PROPERTY]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFaceReference>,
    {
        ::windows_core::vcall!(self.base__.base__.AddFontFaceReference(fontfacereference.into_param().abi(), ::core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap())).ok()
    }
    pub unsafe fn AddFontFaceReference2<P0>(&self, fontfacereference: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFaceReference>,
    {
        ::windows_core::vcall!(self.base__.base__.AddFontFaceReference2(fontfacereference.into_param().abi())).ok()
    }
    pub unsafe fn AddFontSet<P0>(&self, fontset: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontSet>,
    {
        ::windows_core::vcall!(self.base__.base__.AddFontSet(fontset.into_param().abi())).ok()
    }
    pub unsafe fn CreateFontSet(&self) -> ::windows_core::Result<IDWriteFontSet> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.CreateFontSet(&mut result__)).from_abi(result__)
    }
    pub unsafe fn AddFontFile<P0>(&self, fontfile: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFile>,
    {
        ::windows_core::vcall!(self.base__.AddFontFile(fontfile.into_param().abi())).ok()
    }
    pub unsafe fn AddFont<P0>(&self, fontfile: P0, fontfaceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE], fontaxisranges: &[DWRITE_FONT_AXIS_RANGE], properties: &[DWRITE_FONT_PROPERTY]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFile>,
    {
        ::windows_core::vcall!(self.AddFont(fontfile.into_param().abi(), fontfaceindex, fontsimulations, ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), ::core::mem::transmute(fontaxisranges.as_ptr()), fontaxisranges.len().try_into().unwrap(), ::core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap())).ok()
    }
    pub unsafe fn AddFontFile2<P0>(&self, filepath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.AddFontFile2(filepath.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteFontSetBuilder2, ::windows_core::IUnknown, IDWriteFontSetBuilder, IDWriteFontSetBuilder1);
unsafe impl ::core::marker::Send for IDWriteFontSetBuilder2 {}
unsafe impl ::core::marker::Sync for IDWriteFontSetBuilder2 {}
unsafe impl ::windows_core::Interface for IDWriteFontSetBuilder2 {
    type Vtable = IDWriteFontSetBuilder2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteFontSetBuilder2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xee5ba612_b131_463c_8f4f_3189b9401e45);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontSetBuilder2_Vtbl {
    pub base__: IDWriteFontSetBuilder1_Vtbl,
    pub AddFont: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfile: *mut ::core::ffi::c_void, fontfaceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows_core::HRESULT,
    pub AddFontFile2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filepath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteGdiInterop(::windows_core::IUnknown);
impl IDWriteGdiInterop {
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateFontFromLOGFONT(&self, logfont: *const super::Gdi::LOGFONTW) -> ::windows_core::Result<IDWriteFont> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFromLOGFONT(logfont, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn ConvertFontToLOGFONT<P0>(&self, font: P0, logfont: *mut super::Gdi::LOGFONTW, issystemfont: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFont>,
    {
        ::windows_core::vcall!(self.ConvertFontToLOGFONT(font.into_param().abi(), logfont, issystemfont)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ConvertFontFaceToLOGFONT<P0>(&self, font: P0, logfont: *mut super::Gdi::LOGFONTW) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        ::windows_core::vcall!(self.ConvertFontFaceToLOGFONT(font.into_param().abi(), logfont)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateFontFaceFromHdc<P0>(&self, hdc: P0) -> ::windows_core::Result<IDWriteFontFace>
    where
        P0: ::windows_core::IntoParam<super::Gdi::HDC>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFaceFromHdc(hdc.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateBitmapRenderTarget<P0>(&self, hdc: P0, width: u32, height: u32) -> ::windows_core::Result<IDWriteBitmapRenderTarget>
    where
        P0: ::windows_core::IntoParam<super::Gdi::HDC>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateBitmapRenderTarget(hdc.into_param().abi(), width, height, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteGdiInterop, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteGdiInterop {}
unsafe impl ::core::marker::Sync for IDWriteGdiInterop {}
unsafe impl ::windows_core::Interface for IDWriteGdiInterop {
    type Vtable = IDWriteGdiInterop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteGdiInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1edd9491_9853_4299_898f_6432983b6f3a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteGdiInterop_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateFontFromLOGFONT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logfont: *const super::Gdi::LOGFONTW, font: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateFontFromLOGFONT: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub ConvertFontToLOGFONT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, font: *mut ::core::ffi::c_void, logfont: *mut super::Gdi::LOGFONTW, issystemfont: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    ConvertFontToLOGFONT: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ConvertFontFaceToLOGFONT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, font: *mut ::core::ffi::c_void, logfont: *mut super::Gdi::LOGFONTW) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ConvertFontFaceToLOGFONT: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateFontFaceFromHdc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateFontFaceFromHdc: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateBitmapRenderTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC, width: u32, height: u32, rendertarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateBitmapRenderTarget: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteGdiInterop1(::windows_core::IUnknown);
impl IDWriteGdiInterop1 {
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateFontFromLOGFONT(&self, logfont: *const super::Gdi::LOGFONTW) -> ::windows_core::Result<IDWriteFont> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontFromLOGFONT(logfont, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn ConvertFontToLOGFONT<P0>(&self, font: P0, logfont: *mut super::Gdi::LOGFONTW, issystemfont: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFont>,
    {
        ::windows_core::vcall!(self.base__.ConvertFontToLOGFONT(font.into_param().abi(), logfont, issystemfont)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ConvertFontFaceToLOGFONT<P0>(&self, font: P0, logfont: *mut super::Gdi::LOGFONTW) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        ::windows_core::vcall!(self.base__.ConvertFontFaceToLOGFONT(font.into_param().abi(), logfont)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateFontFaceFromHdc<P0>(&self, hdc: P0) -> ::windows_core::Result<IDWriteFontFace>
    where
        P0: ::windows_core::IntoParam<super::Gdi::HDC>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateFontFaceFromHdc(hdc.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateBitmapRenderTarget<P0>(&self, hdc: P0, width: u32, height: u32) -> ::windows_core::Result<IDWriteBitmapRenderTarget>
    where
        P0: ::windows_core::IntoParam<super::Gdi::HDC>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateBitmapRenderTarget(hdc.into_param().abi(), width, height, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateFontFromLOGFONT2<P0>(&self, logfont: *const super::Gdi::LOGFONTW, fontcollection: P0) -> ::windows_core::Result<IDWriteFont>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollection>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFromLOGFONT2(logfont, fontcollection.into_param().abi(), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetFontSignature<P0>(&self, fontface: P0, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        ::windows_core::vcall!(self.GetFontSignature(fontface.into_param().abi(), fontsignature)).ok()
    }
    #[doc = "Required features: `\"Win32_Globalization\"`"]
    #[cfg(feature = "Win32_Globalization")]
    pub unsafe fn GetFontSignature2<P0>(&self, font: P0, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFont>,
    {
        ::windows_core::vcall!(self.GetFontSignature2(font.into_param().abi(), fontsignature)).ok()
    }
    #[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetMatchingFontsByLOGFONT<P0>(&self, logfont: *const super::Gdi::LOGFONTA, fontset: P0) -> ::windows_core::Result<IDWriteFontSet>
    where
        P0: ::windows_core::IntoParam<IDWriteFontSet>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetMatchingFontsByLOGFONT(logfont, fontset.into_param().abi(), &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteGdiInterop1, ::windows_core::IUnknown, IDWriteGdiInterop);
unsafe impl ::core::marker::Send for IDWriteGdiInterop1 {}
unsafe impl ::core::marker::Sync for IDWriteGdiInterop1 {}
unsafe impl ::windows_core::Interface for IDWriteGdiInterop1 {
    type Vtable = IDWriteGdiInterop1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteGdiInterop1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4556be70_3abd_4f70_90be_421780a6f515);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteGdiInterop1_Vtbl {
    pub base__: IDWriteGdiInterop_Vtbl,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateFontFromLOGFONT2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logfont: *const super::Gdi::LOGFONTW, fontcollection: *mut ::core::ffi::c_void, font: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateFontFromLOGFONT2: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetFontSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetFontSignature: usize,
    #[cfg(feature = "Win32_Globalization")]
    pub GetFontSignature2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, font: *mut ::core::ffi::c_void, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Globalization"))]
    GetFontSignature2: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetMatchingFontsByLOGFONT: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, logfont: *const super::Gdi::LOGFONTA, fontset: *mut ::core::ffi::c_void, filteredset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetMatchingFontsByLOGFONT: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteGlyphRunAnalysis(::windows_core::IUnknown);
impl IDWriteGlyphRunAnalysis {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAlphaTextureBounds(&self, texturetype: DWRITE_TEXTURE_TYPE) -> ::windows_core::Result<super::super::Foundation::RECT> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetAlphaTextureBounds(texturetype, &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateAlphaTexture(&self, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *const super::super::Foundation::RECT, alphavalues: &mut [u8]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.CreateAlphaTexture(texturetype, texturebounds, ::core::mem::transmute(alphavalues.as_ptr()), alphavalues.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetAlphaBlendParams<P0>(&self, renderingparams: P0, blendgamma: *mut f32, blendenhancedcontrast: *mut f32, blendcleartypelevel: *mut f32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteRenderingParams>,
    {
        ::windows_core::vcall!(self.GetAlphaBlendParams(renderingparams.into_param().abi(), blendgamma, blendenhancedcontrast, blendcleartypelevel)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteGlyphRunAnalysis, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteGlyphRunAnalysis {}
unsafe impl ::core::marker::Sync for IDWriteGlyphRunAnalysis {}
unsafe impl ::windows_core::Interface for IDWriteGlyphRunAnalysis {
    type Vtable = IDWriteGlyphRunAnalysis_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteGlyphRunAnalysis {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d97dbf7_e085_42d4_81e3_6a883bded118);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteGlyphRunAnalysis_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAlphaTextureBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAlphaTextureBounds: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateAlphaTexture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *const super::super::Foundation::RECT, alphavalues: *mut u8, buffersize: u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateAlphaTexture: usize,
    pub GetAlphaBlendParams: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, renderingparams: *mut ::core::ffi::c_void, blendgamma: *mut f32, blendenhancedcontrast: *mut f32, blendcleartypelevel: *mut f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteInMemoryFontFileLoader(::windows_core::IUnknown);
impl IDWriteInMemoryFontFileLoader {
    pub unsafe fn CreateStreamFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows_core::Result<IDWriteFontFileStream> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateStreamFromKey(fontfilereferencekey, fontfilereferencekeysize, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateInMemoryFontFileReference<P0, P1>(&self, factory: P0, fontdata: *const ::core::ffi::c_void, fontdatasize: u32, ownerobject: P1) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<IDWriteFactory>,
        P1: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateInMemoryFontFileReference(factory.into_param().abi(), fontdata, fontdatasize, ownerobject.into_param().abi(), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFileCount(&self) -> u32 {
        ::windows_core::vcall!(self.GetFileCount())
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteInMemoryFontFileLoader, ::windows_core::IUnknown, IDWriteFontFileLoader);
unsafe impl ::core::marker::Send for IDWriteInMemoryFontFileLoader {}
unsafe impl ::core::marker::Sync for IDWriteInMemoryFontFileLoader {}
unsafe impl ::windows_core::Interface for IDWriteInMemoryFontFileLoader {
    type Vtable = IDWriteInMemoryFontFileLoader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteInMemoryFontFileLoader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc102f47_a12d_4b1c_822d_9e117e33043f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteInMemoryFontFileLoader_Vtbl {
    pub base__: IDWriteFontFileLoader_Vtbl,
    pub CreateInMemoryFontFileReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, fontdata: *const ::core::ffi::c_void, fontdatasize: u32, ownerobject: *mut ::core::ffi::c_void, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFileCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteInlineObject(::windows_core::IUnknown);
impl IDWriteInlineObject {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Draw<P0, P1, P2, P3>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, renderer: P0, originx: f32, originy: f32, issideways: P1, isrighttoleft: P2, clientdrawingeffect: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextRenderer>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.Draw(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), renderer.into_param().abi(), originx, originy, issideways.into_param().abi(), isrighttoleft.into_param().abi(), clientdrawingeffect.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMetrics(&self) -> ::windows_core::Result<DWRITE_INLINE_OBJECT_METRICS> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetMetrics(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetOverhangMetrics(&self) -> ::windows_core::Result<DWRITE_OVERHANG_METRICS> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetOverhangMetrics(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetBreakConditions(&self, breakconditionbefore: *mut DWRITE_BREAK_CONDITION, breakconditionafter: *mut DWRITE_BREAK_CONDITION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetBreakConditions(breakconditionbefore, breakconditionafter)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteInlineObject, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteInlineObject {}
unsafe impl ::core::marker::Sync for IDWriteInlineObject {}
unsafe impl ::windows_core::Interface for IDWriteInlineObject {
    type Vtable = IDWriteInlineObject_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteInlineObject {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8339fde3_106f_47ab_8373_1c6295eb10b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteInlineObject_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, renderer: *mut ::core::ffi::c_void, originx: f32, originy: f32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Draw: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMetrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metrics: *mut DWRITE_INLINE_OBJECT_METRICS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMetrics: usize,
    pub GetOverhangMetrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overhangs: *mut DWRITE_OVERHANG_METRICS) -> ::windows_core::HRESULT,
    pub GetBreakConditions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, breakconditionbefore: *mut DWRITE_BREAK_CONDITION, breakconditionafter: *mut DWRITE_BREAK_CONDITION) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteLocalFontFileLoader(::windows_core::IUnknown);
impl IDWriteLocalFontFileLoader {
    pub unsafe fn CreateStreamFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows_core::Result<IDWriteFontFileStream> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateStreamFromKey(fontfilereferencekey, fontfilereferencekeysize, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFilePathLengthFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFilePathLengthFromKey(fontfilereferencekey, fontfilereferencekeysize, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFilePathFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, filepath: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFilePathFromKey(fontfilereferencekey, fontfilereferencekeysize, ::core::mem::transmute(filepath.as_ptr()), filepath.len().try_into().unwrap())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastWriteTimeFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows_core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetLastWriteTimeFromKey(fontfilereferencekey, fontfilereferencekeysize, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteLocalFontFileLoader, ::windows_core::IUnknown, IDWriteFontFileLoader);
unsafe impl ::core::marker::Send for IDWriteLocalFontFileLoader {}
unsafe impl ::core::marker::Sync for IDWriteLocalFontFileLoader {}
unsafe impl ::windows_core::Interface for IDWriteLocalFontFileLoader {
    type Vtable = IDWriteLocalFontFileLoader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteLocalFontFileLoader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb2d9f3ec_c9fe_4a11_a2ec_d86208f7c0a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteLocalFontFileLoader_Vtbl {
    pub base__: IDWriteFontFileLoader_Vtbl,
    pub GetFilePathLengthFromKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, filepathlength: *mut u32) -> ::windows_core::HRESULT,
    pub GetFilePathFromKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, filepath: ::windows_core::PWSTR, filepathsize: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastWriteTimeFromKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, lastwritetime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastWriteTimeFromKey: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteLocalizedStrings(::windows_core::IUnknown);
impl IDWriteLocalizedStrings {
    pub unsafe fn GetCount(&self) -> u32 {
        ::windows_core::vcall!(self.GetCount())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindLocaleName<P0>(&self, localename: P0, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.FindLocaleName(localename.into_param().abi(), index, exists)).ok()
    }
    pub unsafe fn GetLocaleNameLength(&self, index: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetLocaleNameLength(index, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetLocaleName(&self, index: u32, localename: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetLocaleName(index, ::core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetStringLength(&self, index: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetStringLength(index, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetString(&self, index: u32, stringbuffer: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetString(index, ::core::mem::transmute(stringbuffer.as_ptr()), stringbuffer.len().try_into().unwrap())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteLocalizedStrings, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteLocalizedStrings {}
unsafe impl ::core::marker::Sync for IDWriteLocalizedStrings {}
unsafe impl ::windows_core::Interface for IDWriteLocalizedStrings {
    type Vtable = IDWriteLocalizedStrings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteLocalizedStrings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08256209_099a_4b34_b86d_c22b110e7771);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteLocalizedStrings_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub FindLocaleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localename: ::windows_core::PCWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindLocaleName: usize,
    pub GetLocaleNameLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, length: *mut u32) -> ::windows_core::HRESULT,
    pub GetLocaleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, localename: ::windows_core::PWSTR, size: u32) -> ::windows_core::HRESULT,
    pub GetStringLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, length: *mut u32) -> ::windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, stringbuffer: ::windows_core::PWSTR, size: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteNumberSubstitution(::windows_core::IUnknown);
impl IDWriteNumberSubstitution {}
::windows_core::imp::interface_hierarchy!(IDWriteNumberSubstitution, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteNumberSubstitution {}
unsafe impl ::core::marker::Sync for IDWriteNumberSubstitution {}
unsafe impl ::windows_core::Interface for IDWriteNumberSubstitution {
    type Vtable = IDWriteNumberSubstitution_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteNumberSubstitution {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x14885cc9_bab0_4f90_b6ed_5c366a2cd03d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteNumberSubstitution_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWritePixelSnapping(::windows_core::IUnknown);
impl IDWritePixelSnapping {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPixelSnappingDisabled(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.IsPixelSnappingDisabled(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetCurrentTransform(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, transform: *mut DWRITE_MATRIX) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetCurrentTransform(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), transform)).ok()
    }
    pub unsafe fn GetPixelsPerDip(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetPixelsPerDip(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWritePixelSnapping, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWritePixelSnapping {}
unsafe impl ::core::marker::Sync for IDWritePixelSnapping {}
unsafe impl ::windows_core::Interface for IDWritePixelSnapping {
    type Vtable = IDWritePixelSnapping_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWritePixelSnapping {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeaf3a2da_ecf4_4d24_b644_b34f6842024b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWritePixelSnapping_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPixelSnappingDisabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, isdisabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPixelSnappingDisabled: usize,
    pub GetCurrentTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> ::windows_core::HRESULT,
    pub GetPixelsPerDip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, pixelsperdip: *mut f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteRemoteFontFileLoader(::windows_core::IUnknown);
impl IDWriteRemoteFontFileLoader {
    pub unsafe fn CreateStreamFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows_core::Result<IDWriteFontFileStream> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.CreateStreamFromKey(fontfilereferencekey, fontfilereferencekeysize, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateRemoteStreamFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows_core::Result<IDWriteRemoteFontFileStream> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateRemoteStreamFromKey(fontfilereferencekey, fontfilereferencekeysize, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetLocalityFromKey(&self, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows_core::Result<DWRITE_LOCALITY> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetLocalityFromKey(fontfilereferencekey, fontfilereferencekeysize, &mut result__)).from_abi(result__)
    }
    pub unsafe fn CreateFontFileReferenceFromUrl<P0, P1, P2>(&self, factory: P0, baseurl: P1, fontfileurl: P2) -> ::windows_core::Result<IDWriteFontFile>
    where
        P0: ::windows_core::IntoParam<IDWriteFactory>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.CreateFontFileReferenceFromUrl(factory.into_param().abi(), baseurl.into_param().abi(), fontfileurl.into_param().abi(), &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteRemoteFontFileLoader, ::windows_core::IUnknown, IDWriteFontFileLoader);
unsafe impl ::core::marker::Send for IDWriteRemoteFontFileLoader {}
unsafe impl ::core::marker::Sync for IDWriteRemoteFontFileLoader {}
unsafe impl ::windows_core::Interface for IDWriteRemoteFontFileLoader {
    type Vtable = IDWriteRemoteFontFileLoader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteRemoteFontFileLoader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68648c83_6ede_46c0_ab46_20083a887fde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteRemoteFontFileLoader_Vtbl {
    pub base__: IDWriteFontFileLoader_Vtbl,
    pub CreateRemoteStreamFromKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfilestream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLocalityFromKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, locality: *mut DWRITE_LOCALITY) -> ::windows_core::HRESULT,
    pub CreateFontFileReferenceFromUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, baseurl: ::windows_core::PCWSTR, fontfileurl: ::windows_core::PCWSTR, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteRemoteFontFileStream(::windows_core::IUnknown);
impl IDWriteRemoteFontFileStream {
    pub unsafe fn ReadFileFragment(&self, fragmentstart: *mut *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64, fragmentcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.ReadFileFragment(fragmentstart, fileoffset, fragmentsize, fragmentcontext)).ok()
    }
    pub unsafe fn ReleaseFileFragment(&self, fragmentcontext: *mut ::core::ffi::c_void) {
        ::windows_core::vcall!(self.base__.ReleaseFileFragment(fragmentcontext))
    }
    pub unsafe fn GetFileSize(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFileSize(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetLastWriteTime(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetLastWriteTime(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetLocalFileSize(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetLocalFileSize(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileFragmentLocality(&self, fileoffset: u64, fragmentsize: u64, islocal: *mut super::super::Foundation::BOOL, partialsize: *mut u64) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFileFragmentLocality(fileoffset, fragmentsize, islocal, partialsize)).ok()
    }
    pub unsafe fn GetLocality(&self) -> DWRITE_LOCALITY {
        ::windows_core::vcall!(self.GetLocality())
    }
    pub unsafe fn BeginDownload(&self, downloadoperationid: *const ::windows_core::GUID, filefragments: &[DWRITE_FILE_FRAGMENT]) -> ::windows_core::Result<IDWriteAsyncResult> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.BeginDownload(downloadoperationid, ::core::mem::transmute(filefragments.as_ptr()), filefragments.len().try_into().unwrap(), &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteRemoteFontFileStream, ::windows_core::IUnknown, IDWriteFontFileStream);
unsafe impl ::core::marker::Send for IDWriteRemoteFontFileStream {}
unsafe impl ::core::marker::Sync for IDWriteRemoteFontFileStream {}
unsafe impl ::windows_core::Interface for IDWriteRemoteFontFileStream {
    type Vtable = IDWriteRemoteFontFileStream_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteRemoteFontFileStream {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4db3757a_2c72_4ed9_b2b6_1ababe1aff9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteRemoteFontFileStream_Vtbl {
    pub base__: IDWriteFontFileStream_Vtbl,
    pub GetLocalFileSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localfilesize: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFileFragmentLocality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64, islocal: *mut super::super::Foundation::BOOL, partialsize: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFileFragmentLocality: usize,
    pub GetLocality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_LOCALITY,
    pub BeginDownload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadoperationid: *const ::windows_core::GUID, filefragments: *const DWRITE_FILE_FRAGMENT, fragmentcount: u32, asyncresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteRenderingParams(::windows_core::IUnknown);
impl IDWriteRenderingParams {
    pub unsafe fn GetGamma(&self) -> f32 {
        ::windows_core::vcall!(self.GetGamma())
    }
    pub unsafe fn GetEnhancedContrast(&self) -> f32 {
        ::windows_core::vcall!(self.GetEnhancedContrast())
    }
    pub unsafe fn GetClearTypeLevel(&self) -> f32 {
        ::windows_core::vcall!(self.GetClearTypeLevel())
    }
    pub unsafe fn GetPixelGeometry(&self) -> DWRITE_PIXEL_GEOMETRY {
        ::windows_core::vcall!(self.GetPixelGeometry())
    }
    pub unsafe fn GetRenderingMode(&self) -> DWRITE_RENDERING_MODE {
        ::windows_core::vcall!(self.GetRenderingMode())
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteRenderingParams, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteRenderingParams {}
unsafe impl ::core::marker::Sync for IDWriteRenderingParams {}
unsafe impl ::windows_core::Interface for IDWriteRenderingParams {
    type Vtable = IDWriteRenderingParams_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteRenderingParams {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f0da53a_2add_47cd_82ee_d9ec34688e75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteRenderingParams_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetGamma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> f32,
    pub GetEnhancedContrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> f32,
    pub GetClearTypeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> f32,
    pub GetPixelGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_PIXEL_GEOMETRY,
    pub GetRenderingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_RENDERING_MODE,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteRenderingParams1(::windows_core::IUnknown);
impl IDWriteRenderingParams1 {
    pub unsafe fn GetGamma(&self) -> f32 {
        ::windows_core::vcall!(self.base__.GetGamma())
    }
    pub unsafe fn GetEnhancedContrast(&self) -> f32 {
        ::windows_core::vcall!(self.base__.GetEnhancedContrast())
    }
    pub unsafe fn GetClearTypeLevel(&self) -> f32 {
        ::windows_core::vcall!(self.base__.GetClearTypeLevel())
    }
    pub unsafe fn GetPixelGeometry(&self) -> DWRITE_PIXEL_GEOMETRY {
        ::windows_core::vcall!(self.base__.GetPixelGeometry())
    }
    pub unsafe fn GetRenderingMode(&self) -> DWRITE_RENDERING_MODE {
        ::windows_core::vcall!(self.base__.GetRenderingMode())
    }
    pub unsafe fn GetGrayscaleEnhancedContrast(&self) -> f32 {
        ::windows_core::vcall!(self.GetGrayscaleEnhancedContrast())
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteRenderingParams1, ::windows_core::IUnknown, IDWriteRenderingParams);
unsafe impl ::core::marker::Send for IDWriteRenderingParams1 {}
unsafe impl ::core::marker::Sync for IDWriteRenderingParams1 {}
unsafe impl ::windows_core::Interface for IDWriteRenderingParams1 {
    type Vtable = IDWriteRenderingParams1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteRenderingParams1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94413cf4_a6fc_4248_8b50_6674348fcad3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteRenderingParams1_Vtbl {
    pub base__: IDWriteRenderingParams_Vtbl,
    pub GetGrayscaleEnhancedContrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> f32,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteRenderingParams2(::windows_core::IUnknown);
impl IDWriteRenderingParams2 {
    pub unsafe fn GetGamma(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.GetGamma())
    }
    pub unsafe fn GetEnhancedContrast(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.GetEnhancedContrast())
    }
    pub unsafe fn GetClearTypeLevel(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.GetClearTypeLevel())
    }
    pub unsafe fn GetPixelGeometry(&self) -> DWRITE_PIXEL_GEOMETRY {
        ::windows_core::vcall!(self.base__.base__.GetPixelGeometry())
    }
    pub unsafe fn GetRenderingMode(&self) -> DWRITE_RENDERING_MODE {
        ::windows_core::vcall!(self.base__.base__.GetRenderingMode())
    }
    pub unsafe fn GetGrayscaleEnhancedContrast(&self) -> f32 {
        ::windows_core::vcall!(self.base__.GetGrayscaleEnhancedContrast())
    }
    pub unsafe fn GetGridFitMode(&self) -> DWRITE_GRID_FIT_MODE {
        ::windows_core::vcall!(self.GetGridFitMode())
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteRenderingParams2, ::windows_core::IUnknown, IDWriteRenderingParams, IDWriteRenderingParams1);
unsafe impl ::core::marker::Send for IDWriteRenderingParams2 {}
unsafe impl ::core::marker::Sync for IDWriteRenderingParams2 {}
unsafe impl ::windows_core::Interface for IDWriteRenderingParams2 {
    type Vtable = IDWriteRenderingParams2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteRenderingParams2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf9d711c3_9777_40ae_87e8_3e5af9bf0948);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteRenderingParams2_Vtbl {
    pub base__: IDWriteRenderingParams1_Vtbl,
    pub GetGridFitMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_GRID_FIT_MODE,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteRenderingParams3(::windows_core::IUnknown);
impl IDWriteRenderingParams3 {
    pub unsafe fn GetGamma(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetGamma())
    }
    pub unsafe fn GetEnhancedContrast(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetEnhancedContrast())
    }
    pub unsafe fn GetClearTypeLevel(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetClearTypeLevel())
    }
    pub unsafe fn GetPixelGeometry(&self) -> DWRITE_PIXEL_GEOMETRY {
        ::windows_core::vcall!(self.base__.base__.base__.GetPixelGeometry())
    }
    pub unsafe fn GetRenderingMode(&self) -> DWRITE_RENDERING_MODE {
        ::windows_core::vcall!(self.base__.base__.base__.GetRenderingMode())
    }
    pub unsafe fn GetGrayscaleEnhancedContrast(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.GetGrayscaleEnhancedContrast())
    }
    pub unsafe fn GetGridFitMode(&self) -> DWRITE_GRID_FIT_MODE {
        ::windows_core::vcall!(self.base__.GetGridFitMode())
    }
    pub unsafe fn GetRenderingMode1(&self) -> DWRITE_RENDERING_MODE1 {
        ::windows_core::vcall!(self.GetRenderingMode1())
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteRenderingParams3, ::windows_core::IUnknown, IDWriteRenderingParams, IDWriteRenderingParams1, IDWriteRenderingParams2);
unsafe impl ::core::marker::Send for IDWriteRenderingParams3 {}
unsafe impl ::core::marker::Sync for IDWriteRenderingParams3 {}
unsafe impl ::windows_core::Interface for IDWriteRenderingParams3 {
    type Vtable = IDWriteRenderingParams3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteRenderingParams3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7924baa_391b_412a_8c5c_e44cc2d867dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteRenderingParams3_Vtbl {
    pub base__: IDWriteRenderingParams2_Vtbl,
    pub GetRenderingMode1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_RENDERING_MODE1,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteStringList(::windows_core::IUnknown);
impl IDWriteStringList {
    pub unsafe fn GetCount(&self) -> u32 {
        ::windows_core::vcall!(self.GetCount())
    }
    pub unsafe fn GetLocaleNameLength(&self, listindex: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetLocaleNameLength(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetLocaleName(&self, listindex: u32, localename: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetLocaleName(listindex, ::core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetStringLength(&self, listindex: u32) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetStringLength(listindex, &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetString(&self, listindex: u32, stringbuffer: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetString(listindex, ::core::mem::transmute(stringbuffer.as_ptr()), stringbuffer.len().try_into().unwrap())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteStringList, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteStringList {}
unsafe impl ::core::marker::Sync for IDWriteStringList {}
unsafe impl ::windows_core::Interface for IDWriteStringList {
    type Vtable = IDWriteStringList_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteStringList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcfee3140_1157_47ca_8b85_31bfcf3f2d0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteStringList_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetLocaleNameLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32, length: *mut u32) -> ::windows_core::HRESULT,
    pub GetLocaleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32, localename: ::windows_core::PWSTR, size: u32) -> ::windows_core::HRESULT,
    pub GetStringLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32, length: *mut u32) -> ::windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, listindex: u32, stringbuffer: ::windows_core::PWSTR, stringbuffersize: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextAnalysisSink(::windows_core::IUnknown);
impl IDWriteTextAnalysisSink {
    pub unsafe fn SetScriptAnalysis(&self, textposition: u32, textlength: u32, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetScriptAnalysis(textposition, textlength, scriptanalysis)).ok()
    }
    pub unsafe fn SetLineBreakpoints(&self, textposition: u32, linebreakpoints: &[DWRITE_LINE_BREAKPOINT]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetLineBreakpoints(textposition, linebreakpoints.len().try_into().unwrap(), ::core::mem::transmute(linebreakpoints.as_ptr()))).ok()
    }
    pub unsafe fn SetBidiLevel(&self, textposition: u32, textlength: u32, explicitlevel: u8, resolvedlevel: u8) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetBidiLevel(textposition, textlength, explicitlevel, resolvedlevel)).ok()
    }
    pub unsafe fn SetNumberSubstitution<P0>(&self, textposition: u32, textlength: u32, numbersubstitution: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteNumberSubstitution>,
    {
        ::windows_core::vcall!(self.SetNumberSubstitution(textposition, textlength, numbersubstitution.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextAnalysisSink, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteTextAnalysisSink {}
unsafe impl ::core::marker::Sync for IDWriteTextAnalysisSink {}
unsafe impl ::windows_core::Interface for IDWriteTextAnalysisSink {
    type Vtable = IDWriteTextAnalysisSink_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextAnalysisSink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5810cd44_0ca0_4701_b3fa_bec5182ae4f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextAnalysisSink_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetScriptAnalysis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS) -> ::windows_core::HRESULT,
    pub SetLineBreakpoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, linebreakpoints: *const DWRITE_LINE_BREAKPOINT) -> ::windows_core::HRESULT,
    pub SetBidiLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, explicitlevel: u8, resolvedlevel: u8) -> ::windows_core::HRESULT,
    pub SetNumberSubstitution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, numbersubstitution: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextAnalysisSink1(::windows_core::IUnknown);
impl IDWriteTextAnalysisSink1 {
    pub unsafe fn SetScriptAnalysis(&self, textposition: u32, textlength: u32, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetScriptAnalysis(textposition, textlength, scriptanalysis)).ok()
    }
    pub unsafe fn SetLineBreakpoints(&self, textposition: u32, linebreakpoints: &[DWRITE_LINE_BREAKPOINT]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetLineBreakpoints(textposition, linebreakpoints.len().try_into().unwrap(), ::core::mem::transmute(linebreakpoints.as_ptr()))).ok()
    }
    pub unsafe fn SetBidiLevel(&self, textposition: u32, textlength: u32, explicitlevel: u8, resolvedlevel: u8) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetBidiLevel(textposition, textlength, explicitlevel, resolvedlevel)).ok()
    }
    pub unsafe fn SetNumberSubstitution<P0>(&self, textposition: u32, textlength: u32, numbersubstitution: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteNumberSubstitution>,
    {
        ::windows_core::vcall!(self.base__.SetNumberSubstitution(textposition, textlength, numbersubstitution.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGlyphOrientation<P0, P1>(&self, textposition: u32, textlength: u32, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, adjustedbidilevel: u8, issideways: P0, isrighttoleft: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetGlyphOrientation(textposition, textlength, glyphorientationangle, adjustedbidilevel, issideways.into_param().abi(), isrighttoleft.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextAnalysisSink1, ::windows_core::IUnknown, IDWriteTextAnalysisSink);
unsafe impl ::core::marker::Send for IDWriteTextAnalysisSink1 {}
unsafe impl ::core::marker::Sync for IDWriteTextAnalysisSink1 {}
unsafe impl ::windows_core::Interface for IDWriteTextAnalysisSink1 {
    type Vtable = IDWriteTextAnalysisSink1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextAnalysisSink1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0d941a0_85e7_4d8b_9fd3_5ced9934482a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextAnalysisSink1_Vtbl {
    pub base__: IDWriteTextAnalysisSink_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGlyphOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, adjustedbidilevel: u8, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGlyphOrientation: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextAnalysisSource(::windows_core::IUnknown);
impl IDWriteTextAnalysisSource {
    pub unsafe fn GetTextAtPosition(&self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetTextAtPosition(textposition, textstring, textlength)).ok()
    }
    pub unsafe fn GetTextBeforePosition(&self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetTextBeforePosition(textposition, textstring, textlength)).ok()
    }
    pub unsafe fn GetParagraphReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        ::windows_core::vcall!(self.GetParagraphReadingDirection())
    }
    pub unsafe fn GetLocaleName(&self, textposition: u32, textlength: *mut u32, localename: *mut *mut u16) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetLocaleName(textposition, textlength, localename)).ok()
    }
    pub unsafe fn GetNumberSubstitution(&self, textposition: u32, textlength: *mut u32, numbersubstitution: *mut ::core::option::Option<IDWriteNumberSubstitution>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetNumberSubstitution(textposition, textlength, ::core::mem::transmute(numbersubstitution))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextAnalysisSource, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteTextAnalysisSource {}
unsafe impl ::core::marker::Sync for IDWriteTextAnalysisSource {}
unsafe impl ::windows_core::Interface for IDWriteTextAnalysisSource {
    type Vtable = IDWriteTextAnalysisSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextAnalysisSource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x688e1a58_5094_47c8_adc8_fbcea60ae92b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextAnalysisSource_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetTextAtPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows_core::HRESULT,
    pub GetTextBeforePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows_core::HRESULT,
    pub GetParagraphReadingDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_READING_DIRECTION,
    pub GetLocaleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textposition: u32, textlength: *mut u32, localename: *mut *mut u16) -> ::windows_core::HRESULT,
    pub GetNumberSubstitution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textposition: u32, textlength: *mut u32, numbersubstitution: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextAnalysisSource1(::windows_core::IUnknown);
impl IDWriteTextAnalysisSource1 {
    pub unsafe fn GetTextAtPosition(&self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetTextAtPosition(textposition, textstring, textlength)).ok()
    }
    pub unsafe fn GetTextBeforePosition(&self, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetTextBeforePosition(textposition, textstring, textlength)).ok()
    }
    pub unsafe fn GetParagraphReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        ::windows_core::vcall!(self.base__.GetParagraphReadingDirection())
    }
    pub unsafe fn GetLocaleName(&self, textposition: u32, textlength: *mut u32, localename: *mut *mut u16) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetLocaleName(textposition, textlength, localename)).ok()
    }
    pub unsafe fn GetNumberSubstitution(&self, textposition: u32, textlength: *mut u32, numbersubstitution: *mut ::core::option::Option<IDWriteNumberSubstitution>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetNumberSubstitution(textposition, textlength, ::core::mem::transmute(numbersubstitution))).ok()
    }
    pub unsafe fn GetVerticalGlyphOrientation(&self, textposition: u32, textlength: *mut u32, glyphorientation: *mut DWRITE_VERTICAL_GLYPH_ORIENTATION, bidilevel: *mut u8) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetVerticalGlyphOrientation(textposition, textlength, glyphorientation, bidilevel)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextAnalysisSource1, ::windows_core::IUnknown, IDWriteTextAnalysisSource);
unsafe impl ::core::marker::Send for IDWriteTextAnalysisSource1 {}
unsafe impl ::core::marker::Sync for IDWriteTextAnalysisSource1 {}
unsafe impl ::windows_core::Interface for IDWriteTextAnalysisSource1 {
    type Vtable = IDWriteTextAnalysisSource1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextAnalysisSource1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x639cfad8_0fb4_4b21_a58a_067920120009);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextAnalysisSource1_Vtbl {
    pub base__: IDWriteTextAnalysisSource_Vtbl,
    pub GetVerticalGlyphOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textposition: u32, textlength: *mut u32, glyphorientation: *mut DWRITE_VERTICAL_GLYPH_ORIENTATION, bidilevel: *mut u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextAnalyzer(::windows_core::IUnknown);
impl IDWriteTextAnalyzer {
    pub unsafe fn AnalyzeScript<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextAnalysisSource>,
        P1: ::windows_core::IntoParam<IDWriteTextAnalysisSink>,
    {
        ::windows_core::vcall!(self.AnalyzeScript(analysissource.into_param().abi(), textposition, textlength, analysissink.into_param().abi())).ok()
    }
    pub unsafe fn AnalyzeBidi<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextAnalysisSource>,
        P1: ::windows_core::IntoParam<IDWriteTextAnalysisSink>,
    {
        ::windows_core::vcall!(self.AnalyzeBidi(analysissource.into_param().abi(), textposition, textlength, analysissink.into_param().abi())).ok()
    }
    pub unsafe fn AnalyzeNumberSubstitution<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextAnalysisSource>,
        P1: ::windows_core::IntoParam<IDWriteTextAnalysisSink>,
    {
        ::windows_core::vcall!(self.AnalyzeNumberSubstitution(analysissource.into_param().abi(), textposition, textlength, analysissink.into_param().abi())).ok()
    }
    pub unsafe fn AnalyzeLineBreakpoints<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextAnalysisSource>,
        P1: ::windows_core::IntoParam<IDWriteTextAnalysisSink>,
    {
        ::windows_core::vcall!(self.AnalyzeLineBreakpoints(analysissource.into_param().abi(), textposition, textlength, analysissink.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGlyphs<P0, P1, P2, P3, P4, P5>(&self, textstring: P0, textlength: u32, fontface: P1, issideways: P2, isrighttoleft: P3, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P4, numbersubstitution: P5, features: ::core::option::Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: ::core::option::Option<*const u32>, featureranges: u32, maxglyphcount: u32, clustermap: *mut u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphindices: *mut u16, glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontFace>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P5: ::windows_core::IntoParam<IDWriteNumberSubstitution>,
    {
        ::windows_core::vcall!(self.GetGlyphs(textstring.into_param().abi(), textlength, fontface.into_param().abi(), issideways.into_param().abi(), isrighttoleft.into_param().abi(), scriptanalysis, localename.into_param().abi(), numbersubstitution.into_param().abi(), ::core::mem::transmute(features.unwrap_or(::std::ptr::null())), ::core::mem::transmute(featurerangelengths.unwrap_or(::std::ptr::null())), featureranges, maxglyphcount, clustermap, textprops, glyphindices, glyphprops, actualglyphcount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGlyphPlacements<P0, P1, P2, P3, P4>(&self, textstring: P0, clustermap: *const u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: P1, fontemsize: f32, issideways: P2, isrighttoleft: P3, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P4, features: ::core::option::Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: ::core::option::Option<*const u32>, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontFace>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.GetGlyphPlacements(textstring.into_param().abi(), clustermap, textprops, textlength, glyphindices, glyphprops, glyphcount, fontface.into_param().abi(), fontemsize, issideways.into_param().abi(), isrighttoleft.into_param().abi(), scriptanalysis, localename.into_param().abi(), ::core::mem::transmute(features.unwrap_or(::std::ptr::null())), ::core::mem::transmute(featurerangelengths.unwrap_or(::std::ptr::null())), featureranges, glyphadvances, glyphoffsets)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphPlacements<P0, P1, P2, P3, P4, P5>(&self, textstring: P0, clustermap: *const u16, textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: P1, fontemsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P2, issideways: P3, isrighttoleft: P4, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P5, features: ::core::option::Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: ::core::option::Option<*const u32>, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontFace>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P4: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P5: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.GetGdiCompatibleGlyphPlacements(
            textstring.into_param().abi(),
            clustermap,
            textprops,
            textlength,
            glyphindices,
            glyphprops,
            glyphcount,
            fontface.into_param().abi(),
            fontemsize,
            pixelsperdip,
            ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())),
            usegdinatural.into_param().abi(),
            issideways.into_param().abi(),
            isrighttoleft.into_param().abi(),
            scriptanalysis,
            localename.into_param().abi(),
            ::core::mem::transmute(features.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(featurerangelengths.unwrap_or(::std::ptr::null())),
            featureranges,
            glyphadvances,
            glyphoffsets
        ))
        .ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextAnalyzer, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteTextAnalyzer {}
unsafe impl ::core::marker::Sync for IDWriteTextAnalyzer {}
unsafe impl ::windows_core::Interface for IDWriteTextAnalyzer {
    type Vtable = IDWriteTextAnalyzer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextAnalyzer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb7e6163e_7f46_43b4_84b3_e4e6249c365d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextAnalyzer_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AnalyzeScript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AnalyzeBidi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AnalyzeNumberSubstitution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AnalyzeLineBreakpoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGlyphs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textstring: ::windows_core::PCWSTR, textlength: u32, fontface: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: ::windows_core::PCWSTR, numbersubstitution: *mut ::core::ffi::c_void, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, maxglyphcount: u32, clustermap: *mut u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphindices: *mut u16, glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGlyphs: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGlyphPlacements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textstring: ::windows_core::PCWSTR, clustermap: *const u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: *mut ::core::ffi::c_void, fontemsize: f32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: ::windows_core::PCWSTR, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGlyphPlacements: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGdiCompatibleGlyphPlacements: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        textstring: ::windows_core::PCWSTR,
        clustermap: *const u16,
        textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES,
        textlength: u32,
        glyphindices: *const u16,
        glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
        glyphcount: u32,
        fontface: *mut ::core::ffi::c_void,
        fontemsize: f32,
        pixelsperdip: f32,
        transform: *const DWRITE_MATRIX,
        usegdinatural: super::super::Foundation::BOOL,
        issideways: super::super::Foundation::BOOL,
        isrighttoleft: super::super::Foundation::BOOL,
        scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS,
        localename: ::windows_core::PCWSTR,
        features: *const *const DWRITE_TYPOGRAPHIC_FEATURES,
        featurerangelengths: *const u32,
        featureranges: u32,
        glyphadvances: *mut f32,
        glyphoffsets: *mut DWRITE_GLYPH_OFFSET,
    ) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGdiCompatibleGlyphPlacements: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextAnalyzer1(::windows_core::IUnknown);
impl IDWriteTextAnalyzer1 {
    pub unsafe fn AnalyzeScript<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextAnalysisSource>,
        P1: ::windows_core::IntoParam<IDWriteTextAnalysisSink>,
    {
        ::windows_core::vcall!(self.base__.AnalyzeScript(analysissource.into_param().abi(), textposition, textlength, analysissink.into_param().abi())).ok()
    }
    pub unsafe fn AnalyzeBidi<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextAnalysisSource>,
        P1: ::windows_core::IntoParam<IDWriteTextAnalysisSink>,
    {
        ::windows_core::vcall!(self.base__.AnalyzeBidi(analysissource.into_param().abi(), textposition, textlength, analysissink.into_param().abi())).ok()
    }
    pub unsafe fn AnalyzeNumberSubstitution<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextAnalysisSource>,
        P1: ::windows_core::IntoParam<IDWriteTextAnalysisSink>,
    {
        ::windows_core::vcall!(self.base__.AnalyzeNumberSubstitution(analysissource.into_param().abi(), textposition, textlength, analysissink.into_param().abi())).ok()
    }
    pub unsafe fn AnalyzeLineBreakpoints<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextAnalysisSource>,
        P1: ::windows_core::IntoParam<IDWriteTextAnalysisSink>,
    {
        ::windows_core::vcall!(self.base__.AnalyzeLineBreakpoints(analysissource.into_param().abi(), textposition, textlength, analysissink.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGlyphs<P0, P1, P2, P3, P4, P5>(&self, textstring: P0, textlength: u32, fontface: P1, issideways: P2, isrighttoleft: P3, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P4, numbersubstitution: P5, features: ::core::option::Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: ::core::option::Option<*const u32>, featureranges: u32, maxglyphcount: u32, clustermap: *mut u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphindices: *mut u16, glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontFace>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P5: ::windows_core::IntoParam<IDWriteNumberSubstitution>,
    {
        ::windows_core::vcall!(self.base__.GetGlyphs(textstring.into_param().abi(), textlength, fontface.into_param().abi(), issideways.into_param().abi(), isrighttoleft.into_param().abi(), scriptanalysis, localename.into_param().abi(), numbersubstitution.into_param().abi(), ::core::mem::transmute(features.unwrap_or(::std::ptr::null())), ::core::mem::transmute(featurerangelengths.unwrap_or(::std::ptr::null())), featureranges, maxglyphcount, clustermap, textprops, glyphindices, glyphprops, actualglyphcount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGlyphPlacements<P0, P1, P2, P3, P4>(&self, textstring: P0, clustermap: *const u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: P1, fontemsize: f32, issideways: P2, isrighttoleft: P3, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P4, features: ::core::option::Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: ::core::option::Option<*const u32>, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontFace>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.GetGlyphPlacements(textstring.into_param().abi(), clustermap, textprops, textlength, glyphindices, glyphprops, glyphcount, fontface.into_param().abi(), fontemsize, issideways.into_param().abi(), isrighttoleft.into_param().abi(), scriptanalysis, localename.into_param().abi(), ::core::mem::transmute(features.unwrap_or(::std::ptr::null())), ::core::mem::transmute(featurerangelengths.unwrap_or(::std::ptr::null())), featureranges, glyphadvances, glyphoffsets)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphPlacements<P0, P1, P2, P3, P4, P5>(&self, textstring: P0, clustermap: *const u16, textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: P1, fontemsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P2, issideways: P3, isrighttoleft: P4, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P5, features: ::core::option::Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: ::core::option::Option<*const u32>, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontFace>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P4: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P5: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.GetGdiCompatibleGlyphPlacements(
            textstring.into_param().abi(),
            clustermap,
            textprops,
            textlength,
            glyphindices,
            glyphprops,
            glyphcount,
            fontface.into_param().abi(),
            fontemsize,
            pixelsperdip,
            ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())),
            usegdinatural.into_param().abi(),
            issideways.into_param().abi(),
            isrighttoleft.into_param().abi(),
            scriptanalysis,
            localename.into_param().abi(),
            ::core::mem::transmute(features.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(featurerangelengths.unwrap_or(::std::ptr::null())),
            featureranges,
            glyphadvances,
            glyphoffsets
        ))
        .ok()
    }
    pub unsafe fn ApplyCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, glyphcount: u32, clustermap: &[u16], glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.ApplyCharacterSpacing(leadingspacing, trailingspacing, minimumadvancewidth, clustermap.len().try_into().unwrap(), glyphcount, ::core::mem::transmute(clustermap.as_ptr()), glyphadvances, glyphoffsets, glyphproperties, modifiedglyphadvances, modifiedglyphoffsets)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBaseline<P0, P1, P2, P3>(&self, fontface: P0, baseline: DWRITE_BASELINE, isvertical: P1, issimulationallowed: P2, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: P3, baselinecoordinate: *mut i32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.GetBaseline(fontface.into_param().abi(), baseline, isvertical.into_param().abi(), issimulationallowed.into_param().abi(), ::core::mem::transmute(scriptanalysis), localename.into_param().abi(), baselinecoordinate, exists)).ok()
    }
    pub unsafe fn AnalyzeVerticalGlyphOrientation<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextAnalysisSource1>,
        P1: ::windows_core::IntoParam<IDWriteTextAnalysisSink1>,
    {
        ::windows_core::vcall!(self.AnalyzeVerticalGlyphOrientation(analysissource.into_param().abi(), textposition, textlength, analysissink.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGlyphOrientationTransform<P0>(&self, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: P0, transform: *mut DWRITE_MATRIX) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.GetGlyphOrientationTransform(glyphorientationangle, issideways.into_param().abi(), transform)).ok()
    }
    pub unsafe fn GetScriptProperties(&self, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, scriptproperties: *mut DWRITE_SCRIPT_PROPERTIES) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetScriptProperties(::core::mem::transmute(scriptanalysis), scriptproperties)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTextComplexity<P0, P1>(&self, textstring: P0, textlength: u32, fontface: P1, istextsimple: *mut super::super::Foundation::BOOL, textlengthread: *mut u32, glyphindices: ::core::option::Option<*mut u16>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        ::windows_core::vcall!(self.GetTextComplexity(textstring.into_param().abi(), textlength, fontface.into_param().abi(), istextsimple, textlengthread, ::core::mem::transmute(glyphindices.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetJustificationOpportunities<P0, P1>(&self, fontface: P0, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, textstring: P1, clustermap: *const u16, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, justificationopportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.GetJustificationOpportunities(fontface.into_param().abi(), fontemsize, ::core::mem::transmute(scriptanalysis), textlength, glyphcount, textstring.into_param().abi(), clustermap, glyphproperties, justificationopportunities)).ok()
    }
    pub unsafe fn JustifyGlyphAdvances(&self, linewidth: f32, glyphcount: u32, justificationopportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, justifiedglyphadvances: *mut f32, justifiedglyphoffsets: ::core::option::Option<*mut DWRITE_GLYPH_OFFSET>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.JustifyGlyphAdvances(linewidth, glyphcount, justificationopportunities, glyphadvances, glyphoffsets, justifiedglyphadvances, ::core::mem::transmute(justifiedglyphoffsets.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetJustifiedGlyphs<P0>(&self, fontface: P0, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, maxglyphcount: u32, clustermap: ::core::option::Option<*const u16>, glyphindices: *const u16, glyphadvances: *const f32, justifiedglyphadvances: *const f32, justifiedglyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32, modifiedclustermap: ::core::option::Option<*mut u16>, modifiedglyphindices: *mut u16, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        ::windows_core::vcall!(self.GetJustifiedGlyphs(fontface.into_param().abi(), fontemsize, ::core::mem::transmute(scriptanalysis), textlength, glyphcount, maxglyphcount, ::core::mem::transmute(clustermap.unwrap_or(::std::ptr::null())), glyphindices, glyphadvances, justifiedglyphadvances, justifiedglyphoffsets, glyphproperties, actualglyphcount, ::core::mem::transmute(modifiedclustermap.unwrap_or(::std::ptr::null_mut())), modifiedglyphindices, modifiedglyphadvances, modifiedglyphoffsets)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextAnalyzer1, ::windows_core::IUnknown, IDWriteTextAnalyzer);
unsafe impl ::core::marker::Send for IDWriteTextAnalyzer1 {}
unsafe impl ::core::marker::Sync for IDWriteTextAnalyzer1 {}
unsafe impl ::windows_core::Interface for IDWriteTextAnalyzer1 {
    type Vtable = IDWriteTextAnalyzer1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextAnalyzer1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80dad800_e21f_4e83_96ce_bfcce500db7c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextAnalyzer1_Vtbl {
    pub base__: IDWriteTextAnalyzer_Vtbl,
    pub ApplyCharacterSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textlength: u32, glyphcount: u32, clustermap: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, baseline: DWRITE_BASELINE, isvertical: super::super::Foundation::BOOL, issimulationallowed: super::super::Foundation::BOOL, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: ::windows_core::PCWSTR, baselinecoordinate: *mut i32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBaseline: usize,
    pub AnalyzeVerticalGlyphOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGlyphOrientationTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, transform: *mut DWRITE_MATRIX) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGlyphOrientationTransform: usize,
    pub GetScriptProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, scriptproperties: *mut DWRITE_SCRIPT_PROPERTIES) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTextComplexity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textstring: ::windows_core::PCWSTR, textlength: u32, fontface: *mut ::core::ffi::c_void, istextsimple: *mut super::super::Foundation::BOOL, textlengthread: *mut u32, glyphindices: *mut u16) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTextComplexity: usize,
    pub GetJustificationOpportunities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, textstring: ::windows_core::PCWSTR, clustermap: *const u16, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, justificationopportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> ::windows_core::HRESULT,
    pub JustifyGlyphAdvances: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linewidth: f32, glyphcount: u32, justificationopportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, justifiedglyphadvances: *mut f32, justifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::HRESULT,
    pub GetJustifiedGlyphs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, maxglyphcount: u32, clustermap: *const u16, glyphindices: *const u16, glyphadvances: *const f32, justifiedglyphadvances: *const f32, justifiedglyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32, modifiedclustermap: *mut u16, modifiedglyphindices: *mut u16, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextAnalyzer2(::windows_core::IUnknown);
impl IDWriteTextAnalyzer2 {
    pub unsafe fn AnalyzeScript<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextAnalysisSource>,
        P1: ::windows_core::IntoParam<IDWriteTextAnalysisSink>,
    {
        ::windows_core::vcall!(self.base__.base__.AnalyzeScript(analysissource.into_param().abi(), textposition, textlength, analysissink.into_param().abi())).ok()
    }
    pub unsafe fn AnalyzeBidi<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextAnalysisSource>,
        P1: ::windows_core::IntoParam<IDWriteTextAnalysisSink>,
    {
        ::windows_core::vcall!(self.base__.base__.AnalyzeBidi(analysissource.into_param().abi(), textposition, textlength, analysissink.into_param().abi())).ok()
    }
    pub unsafe fn AnalyzeNumberSubstitution<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextAnalysisSource>,
        P1: ::windows_core::IntoParam<IDWriteTextAnalysisSink>,
    {
        ::windows_core::vcall!(self.base__.base__.AnalyzeNumberSubstitution(analysissource.into_param().abi(), textposition, textlength, analysissink.into_param().abi())).ok()
    }
    pub unsafe fn AnalyzeLineBreakpoints<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextAnalysisSource>,
        P1: ::windows_core::IntoParam<IDWriteTextAnalysisSink>,
    {
        ::windows_core::vcall!(self.base__.base__.AnalyzeLineBreakpoints(analysissource.into_param().abi(), textposition, textlength, analysissink.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGlyphs<P0, P1, P2, P3, P4, P5>(&self, textstring: P0, textlength: u32, fontface: P1, issideways: P2, isrighttoleft: P3, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P4, numbersubstitution: P5, features: ::core::option::Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: ::core::option::Option<*const u32>, featureranges: u32, maxglyphcount: u32, clustermap: *mut u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphindices: *mut u16, glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontFace>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P5: ::windows_core::IntoParam<IDWriteNumberSubstitution>,
    {
        ::windows_core::vcall!(self
            .base__
            .base__
            .GetGlyphs(textstring.into_param().abi(), textlength, fontface.into_param().abi(), issideways.into_param().abi(), isrighttoleft.into_param().abi(), scriptanalysis, localename.into_param().abi(), numbersubstitution.into_param().abi(), ::core::mem::transmute(features.unwrap_or(::std::ptr::null())), ::core::mem::transmute(featurerangelengths.unwrap_or(::std::ptr::null())), featureranges, maxglyphcount, clustermap, textprops, glyphindices, glyphprops, actualglyphcount))
        .ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGlyphPlacements<P0, P1, P2, P3, P4>(&self, textstring: P0, clustermap: *const u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: P1, fontemsize: f32, issideways: P2, isrighttoleft: P3, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P4, features: ::core::option::Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: ::core::option::Option<*const u32>, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontFace>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.base__.GetGlyphPlacements(textstring.into_param().abi(), clustermap, textprops, textlength, glyphindices, glyphprops, glyphcount, fontface.into_param().abi(), fontemsize, issideways.into_param().abi(), isrighttoleft.into_param().abi(), scriptanalysis, localename.into_param().abi(), ::core::mem::transmute(features.unwrap_or(::std::ptr::null())), ::core::mem::transmute(featurerangelengths.unwrap_or(::std::ptr::null())), featureranges, glyphadvances, glyphoffsets)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGdiCompatibleGlyphPlacements<P0, P1, P2, P3, P4, P5>(&self, textstring: P0, clustermap: *const u16, textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: P1, fontemsize: f32, pixelsperdip: f32, transform: ::core::option::Option<*const DWRITE_MATRIX>, usegdinatural: P2, issideways: P3, isrighttoleft: P4, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: P5, features: ::core::option::Option<*const *const DWRITE_TYPOGRAPHIC_FEATURES>, featurerangelengths: ::core::option::Option<*const u32>, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontFace>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P4: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P5: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.base__.GetGdiCompatibleGlyphPlacements(
            textstring.into_param().abi(),
            clustermap,
            textprops,
            textlength,
            glyphindices,
            glyphprops,
            glyphcount,
            fontface.into_param().abi(),
            fontemsize,
            pixelsperdip,
            ::core::mem::transmute(transform.unwrap_or(::std::ptr::null())),
            usegdinatural.into_param().abi(),
            issideways.into_param().abi(),
            isrighttoleft.into_param().abi(),
            scriptanalysis,
            localename.into_param().abi(),
            ::core::mem::transmute(features.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(featurerangelengths.unwrap_or(::std::ptr::null())),
            featureranges,
            glyphadvances,
            glyphoffsets
        ))
        .ok()
    }
    pub unsafe fn ApplyCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, glyphcount: u32, clustermap: &[u16], glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.ApplyCharacterSpacing(leadingspacing, trailingspacing, minimumadvancewidth, clustermap.len().try_into().unwrap(), glyphcount, ::core::mem::transmute(clustermap.as_ptr()), glyphadvances, glyphoffsets, glyphproperties, modifiedglyphadvances, modifiedglyphoffsets)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBaseline<P0, P1, P2, P3>(&self, fontface: P0, baseline: DWRITE_BASELINE, isvertical: P1, issimulationallowed: P2, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: P3, baselinecoordinate: *mut i32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.GetBaseline(fontface.into_param().abi(), baseline, isvertical.into_param().abi(), issimulationallowed.into_param().abi(), ::core::mem::transmute(scriptanalysis), localename.into_param().abi(), baselinecoordinate, exists)).ok()
    }
    pub unsafe fn AnalyzeVerticalGlyphOrientation<P0, P1>(&self, analysissource: P0, textposition: u32, textlength: u32, analysissink: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextAnalysisSource1>,
        P1: ::windows_core::IntoParam<IDWriteTextAnalysisSink1>,
    {
        ::windows_core::vcall!(self.base__.AnalyzeVerticalGlyphOrientation(analysissource.into_param().abi(), textposition, textlength, analysissink.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGlyphOrientationTransform<P0>(&self, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: P0, transform: *mut DWRITE_MATRIX) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.GetGlyphOrientationTransform(glyphorientationangle, issideways.into_param().abi(), transform)).ok()
    }
    pub unsafe fn GetScriptProperties(&self, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, scriptproperties: *mut DWRITE_SCRIPT_PROPERTIES) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetScriptProperties(::core::mem::transmute(scriptanalysis), scriptproperties)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTextComplexity<P0, P1>(&self, textstring: P0, textlength: u32, fontface: P1, istextsimple: *mut super::super::Foundation::BOOL, textlengthread: *mut u32, glyphindices: ::core::option::Option<*mut u16>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        ::windows_core::vcall!(self.base__.GetTextComplexity(textstring.into_param().abi(), textlength, fontface.into_param().abi(), istextsimple, textlengthread, ::core::mem::transmute(glyphindices.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetJustificationOpportunities<P0, P1>(&self, fontface: P0, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, textstring: P1, clustermap: *const u16, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, justificationopportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.GetJustificationOpportunities(fontface.into_param().abi(), fontemsize, ::core::mem::transmute(scriptanalysis), textlength, glyphcount, textstring.into_param().abi(), clustermap, glyphproperties, justificationopportunities)).ok()
    }
    pub unsafe fn JustifyGlyphAdvances(&self, linewidth: f32, glyphcount: u32, justificationopportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, justifiedglyphadvances: *mut f32, justifiedglyphoffsets: ::core::option::Option<*mut DWRITE_GLYPH_OFFSET>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.JustifyGlyphAdvances(linewidth, glyphcount, justificationopportunities, glyphadvances, glyphoffsets, justifiedglyphadvances, ::core::mem::transmute(justifiedglyphoffsets.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetJustifiedGlyphs<P0>(&self, fontface: P0, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, maxglyphcount: u32, clustermap: ::core::option::Option<*const u16>, glyphindices: *const u16, glyphadvances: *const f32, justifiedglyphadvances: *const f32, justifiedglyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32, modifiedclustermap: ::core::option::Option<*mut u16>, modifiedglyphindices: *mut u16, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
    {
        ::windows_core::vcall!(self.base__.GetJustifiedGlyphs(fontface.into_param().abi(), fontemsize, ::core::mem::transmute(scriptanalysis), textlength, glyphcount, maxglyphcount, ::core::mem::transmute(clustermap.unwrap_or(::std::ptr::null())), glyphindices, glyphadvances, justifiedglyphadvances, justifiedglyphoffsets, glyphproperties, actualglyphcount, ::core::mem::transmute(modifiedclustermap.unwrap_or(::std::ptr::null_mut())), modifiedglyphindices, modifiedglyphadvances, modifiedglyphoffsets)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGlyphOrientationTransform2<P0>(&self, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: P0, originx: f32, originy: f32, transform: *mut DWRITE_MATRIX) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.GetGlyphOrientationTransform2(glyphorientationangle, issideways.into_param().abi(), originx, originy, transform)).ok()
    }
    pub unsafe fn GetTypographicFeatures<P0, P1>(&self, fontface: P0, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: P1, actualtagcount: *mut u32, tags: &mut [DWRITE_FONT_FEATURE_TAG]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.GetTypographicFeatures(fontface.into_param().abi(), ::core::mem::transmute(scriptanalysis), localename.into_param().abi(), tags.len().try_into().unwrap(), actualtagcount, ::core::mem::transmute(tags.as_ptr()))).ok()
    }
    pub unsafe fn CheckTypographicFeature<P0, P1>(&self, fontface: P0, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: P1, featuretag: DWRITE_FONT_FEATURE_TAG, glyphcount: u32, glyphindices: *const u16, featureapplies: *mut u8) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFace>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.CheckTypographicFeature(fontface.into_param().abi(), ::core::mem::transmute(scriptanalysis), localename.into_param().abi(), featuretag, glyphcount, glyphindices, featureapplies)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextAnalyzer2, ::windows_core::IUnknown, IDWriteTextAnalyzer, IDWriteTextAnalyzer1);
unsafe impl ::core::marker::Send for IDWriteTextAnalyzer2 {}
unsafe impl ::core::marker::Sync for IDWriteTextAnalyzer2 {}
unsafe impl ::windows_core::Interface for IDWriteTextAnalyzer2 {
    type Vtable = IDWriteTextAnalyzer2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextAnalyzer2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x553a9ff3_5693_4df7_b52b_74806f7f2eb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextAnalyzer2_Vtbl {
    pub base__: IDWriteTextAnalyzer1_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetGlyphOrientationTransform2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, originx: f32, originy: f32, transform: *mut DWRITE_MATRIX) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetGlyphOrientationTransform2: usize,
    pub GetTypographicFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: ::windows_core::PCWSTR, maxtagcount: u32, actualtagcount: *mut u32, tags: *mut DWRITE_FONT_FEATURE_TAG) -> ::windows_core::HRESULT,
    pub CheckTypographicFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: ::windows_core::PCWSTR, featuretag: DWRITE_FONT_FEATURE_TAG, glyphcount: u32, glyphindices: *const u16, featureapplies: *mut u8) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextFormat(::windows_core::IUnknown);
impl IDWriteTextFormat {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetTextAlignment(textalignment)).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetParagraphAlignment(paragraphalignment)).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetWordWrapping(wordwrapping)).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetReadingDirection(readingdirection)).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetFlowDirection(flowdirection)).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetIncrementalTabStop(incrementaltabstop)).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteInlineObject>,
    {
        ::windows_core::vcall!(self.SetTrimming(trimmingoptions, trimmingsign.into_param().abi())).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        ::windows_core::vcall!(self.GetTextAlignment())
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        ::windows_core::vcall!(self.GetParagraphAlignment())
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        ::windows_core::vcall!(self.GetWordWrapping())
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        ::windows_core::vcall!(self.GetReadingDirection())
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        ::windows_core::vcall!(self.GetFlowDirection())
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        ::windows_core::vcall!(self.GetIncrementalTabStop())
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetTrimming(trimmingoptions, ::core::mem::transmute(trimmingsign))).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> ::windows_core::Result<IDWriteFontCollection> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontCollection(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.GetFontFamilyNameLength())
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFontFamilyName(::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        ::windows_core::vcall!(self.GetFontWeight())
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        ::windows_core::vcall!(self.GetFontStyle())
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        ::windows_core::vcall!(self.GetFontStretch())
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        ::windows_core::vcall!(self.GetFontSize())
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.GetLocaleNameLength())
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetLocaleName(::core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextFormat, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteTextFormat {}
unsafe impl ::core::marker::Sync for IDWriteTextFormat {}
unsafe impl ::windows_core::Interface for IDWriteTextFormat {
    type Vtable = IDWriteTextFormat_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextFormat {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c906818_31d7_4fd3_a151_7c5e225db55a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextFormat_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetTextAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows_core::HRESULT,
    pub SetParagraphAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows_core::HRESULT,
    pub SetWordWrapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows_core::HRESULT,
    pub SetReadingDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readingdirection: DWRITE_READING_DIRECTION) -> ::windows_core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows_core::HRESULT,
    pub SetIncrementalTabStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, incrementaltabstop: f32) -> ::windows_core::HRESULT,
    pub SetTrimming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows_core::HRESULT,
    pub GetTextAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_TEXT_ALIGNMENT,
    pub GetParagraphAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_PARAGRAPH_ALIGNMENT,
    pub GetWordWrapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_WORD_WRAPPING,
    pub GetReadingDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_READING_DIRECTION,
    pub GetFlowDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_FLOW_DIRECTION,
    pub GetIncrementalTabStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> f32,
    pub GetTrimming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetLineSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows_core::HRESULT,
    pub GetFontCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFontFamilyNameLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetFontFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfamilyname: ::windows_core::PWSTR, namesize: u32) -> ::windows_core::HRESULT,
    pub GetFontWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_WEIGHT,
    pub GetFontStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STYLE,
    pub GetFontStretch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STRETCH,
    pub GetFontSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> f32,
    pub GetLocaleNameLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetLocaleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localename: ::windows_core::PWSTR, namesize: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextFormat1(::windows_core::IUnknown);
impl IDWriteTextFormat1 {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetTextAlignment(textalignment)).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetParagraphAlignment(paragraphalignment)).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetWordWrapping(wordwrapping)).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetReadingDirection(readingdirection)).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetFlowDirection(flowdirection)).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetIncrementalTabStop(incrementaltabstop)).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteInlineObject>,
    {
        ::windows_core::vcall!(self.base__.SetTrimming(trimmingoptions, trimmingsign.into_param().abi())).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        ::windows_core::vcall!(self.base__.GetTextAlignment())
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        ::windows_core::vcall!(self.base__.GetParagraphAlignment())
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        ::windows_core::vcall!(self.base__.GetWordWrapping())
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        ::windows_core::vcall!(self.base__.GetReadingDirection())
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        ::windows_core::vcall!(self.base__.GetFlowDirection())
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        ::windows_core::vcall!(self.base__.GetIncrementalTabStop())
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetTrimming(trimmingoptions, ::core::mem::transmute(trimmingsign))).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> ::windows_core::Result<IDWriteFontCollection> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontCollection(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.base__.GetFontFamilyNameLength())
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetFontFamilyName(::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        ::windows_core::vcall!(self.base__.GetFontWeight())
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        ::windows_core::vcall!(self.base__.GetFontStyle())
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        ::windows_core::vcall!(self.base__.GetFontStretch())
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        ::windows_core::vcall!(self.base__.GetFontSize())
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.base__.GetLocaleNameLength())
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetLocaleName(::core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap())).ok()
    }
    pub unsafe fn SetVerticalGlyphOrientation(&self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetVerticalGlyphOrientation(glyphorientation)).ok()
    }
    pub unsafe fn GetVerticalGlyphOrientation(&self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
        ::windows_core::vcall!(self.GetVerticalGlyphOrientation())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastLineWrapping<P0>(&self, islastlinewrappingenabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetLastLineWrapping(islastlinewrappingenabled.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastLineWrapping(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.GetLastLineWrapping())
    }
    pub unsafe fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOpticalAlignment(opticalalignment)).ok()
    }
    pub unsafe fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT {
        ::windows_core::vcall!(self.GetOpticalAlignment())
    }
    pub unsafe fn SetFontFallback<P0>(&self, fontfallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFallback>,
    {
        ::windows_core::vcall!(self.SetFontFallback(fontfallback.into_param().abi())).ok()
    }
    pub unsafe fn GetFontFallback(&self) -> ::windows_core::Result<IDWriteFontFallback> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontFallback(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextFormat1, ::windows_core::IUnknown, IDWriteTextFormat);
unsafe impl ::core::marker::Send for IDWriteTextFormat1 {}
unsafe impl ::core::marker::Sync for IDWriteTextFormat1 {}
unsafe impl ::windows_core::Interface for IDWriteTextFormat1 {
    type Vtable = IDWriteTextFormat1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextFormat1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f174b49_0d8b_4cfb_8bca_f1cce9d06c67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextFormat1_Vtbl {
    pub base__: IDWriteTextFormat_Vtbl,
    pub SetVerticalGlyphOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows_core::HRESULT,
    pub GetVerticalGlyphOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_VERTICAL_GLYPH_ORIENTATION,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLastLineWrapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, islastlinewrappingenabled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLastLineWrapping: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastLineWrapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastLineWrapping: usize,
    pub SetOpticalAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows_core::HRESULT,
    pub GetOpticalAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT,
    pub SetFontFallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFontFallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfallback: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextFormat2(::windows_core::IUnknown);
impl IDWriteTextFormat2 {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetTextAlignment(textalignment)).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetParagraphAlignment(paragraphalignment)).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetWordWrapping(wordwrapping)).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetReadingDirection(readingdirection)).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetFlowDirection(flowdirection)).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetIncrementalTabStop(incrementaltabstop)).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteInlineObject>,
    {
        ::windows_core::vcall!(self.base__.base__.SetTrimming(trimmingoptions, trimmingsign.into_param().abi())).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        ::windows_core::vcall!(self.base__.base__.GetTextAlignment())
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        ::windows_core::vcall!(self.base__.base__.GetParagraphAlignment())
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        ::windows_core::vcall!(self.base__.base__.GetWordWrapping())
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        ::windows_core::vcall!(self.base__.base__.GetReadingDirection())
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        ::windows_core::vcall!(self.base__.base__.GetFlowDirection())
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.GetIncrementalTabStop())
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetTrimming(trimmingoptions, ::core::mem::transmute(trimmingsign))).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> ::windows_core::Result<IDWriteFontCollection> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFontCollection(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.GetFontFamilyNameLength())
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetFontFamilyName(::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        ::windows_core::vcall!(self.base__.base__.GetFontWeight())
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        ::windows_core::vcall!(self.base__.base__.GetFontStyle())
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        ::windows_core::vcall!(self.base__.base__.GetFontStretch())
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.GetFontSize())
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.GetLocaleNameLength())
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetLocaleName(::core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap())).ok()
    }
    pub unsafe fn SetVerticalGlyphOrientation(&self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetVerticalGlyphOrientation(glyphorientation)).ok()
    }
    pub unsafe fn GetVerticalGlyphOrientation(&self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
        ::windows_core::vcall!(self.base__.GetVerticalGlyphOrientation())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastLineWrapping<P0>(&self, islastlinewrappingenabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.SetLastLineWrapping(islastlinewrappingenabled.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastLineWrapping(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.GetLastLineWrapping())
    }
    pub unsafe fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetOpticalAlignment(opticalalignment)).ok()
    }
    pub unsafe fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT {
        ::windows_core::vcall!(self.base__.GetOpticalAlignment())
    }
    pub unsafe fn SetFontFallback<P0>(&self, fontfallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFallback>,
    {
        ::windows_core::vcall!(self.base__.SetFontFallback(fontfallback.into_param().abi())).ok()
    }
    pub unsafe fn GetFontFallback(&self) -> ::windows_core::Result<IDWriteFontFallback> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontFallback(&mut result__)).from_abi(result__)
    }
    pub unsafe fn SetLineSpacing2(&self, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetLineSpacing2(linespacingoptions)).ok()
    }
    pub unsafe fn GetLineSpacing2(&self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetLineSpacing2(linespacingoptions)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextFormat2, ::windows_core::IUnknown, IDWriteTextFormat, IDWriteTextFormat1);
unsafe impl ::core::marker::Send for IDWriteTextFormat2 {}
unsafe impl ::core::marker::Sync for IDWriteTextFormat2 {}
unsafe impl ::windows_core::Interface for IDWriteTextFormat2 {
    type Vtable = IDWriteTextFormat2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextFormat2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf67e0edd_9e3d_4ecc_8c32_4183253dfe70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextFormat2_Vtbl {
    pub base__: IDWriteTextFormat1_Vtbl,
    pub SetLineSpacing2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows_core::HRESULT,
    pub GetLineSpacing2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextFormat3(::windows_core::IUnknown);
impl IDWriteTextFormat3 {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetTextAlignment(textalignment)).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetParagraphAlignment(paragraphalignment)).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetWordWrapping(wordwrapping)).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetReadingDirection(readingdirection)).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetFlowDirection(flowdirection)).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetIncrementalTabStop(incrementaltabstop)).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteInlineObject>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetTrimming(trimmingoptions, trimmingsign.into_param().abi())).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        ::windows_core::vcall!(self.base__.base__.base__.GetTextAlignment())
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        ::windows_core::vcall!(self.base__.base__.base__.GetParagraphAlignment())
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        ::windows_core::vcall!(self.base__.base__.base__.GetWordWrapping())
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        ::windows_core::vcall!(self.base__.base__.base__.GetReadingDirection())
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        ::windows_core::vcall!(self.base__.base__.base__.GetFlowDirection())
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetIncrementalTabStop())
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetTrimming(trimmingoptions, ::core::mem::transmute(trimmingsign))).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> ::windows_core::Result<IDWriteFontCollection> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFontCollection(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontFamilyNameLength())
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontFamilyName(::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontWeight())
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontStyle())
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontStretch())
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontSize())
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetLocaleNameLength())
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetLocaleName(::core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap())).ok()
    }
    pub unsafe fn SetVerticalGlyphOrientation(&self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetVerticalGlyphOrientation(glyphorientation)).ok()
    }
    pub unsafe fn GetVerticalGlyphOrientation(&self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
        ::windows_core::vcall!(self.base__.base__.GetVerticalGlyphOrientation())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastLineWrapping<P0>(&self, islastlinewrappingenabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.SetLastLineWrapping(islastlinewrappingenabled.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastLineWrapping(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.GetLastLineWrapping())
    }
    pub unsafe fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetOpticalAlignment(opticalalignment)).ok()
    }
    pub unsafe fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT {
        ::windows_core::vcall!(self.base__.base__.GetOpticalAlignment())
    }
    pub unsafe fn SetFontFallback<P0>(&self, fontfallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFallback>,
    {
        ::windows_core::vcall!(self.base__.base__.SetFontFallback(fontfallback.into_param().abi())).ok()
    }
    pub unsafe fn GetFontFallback(&self) -> ::windows_core::Result<IDWriteFontFallback> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFontFallback(&mut result__)).from_abi(result__)
    }
    pub unsafe fn SetLineSpacing2(&self, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetLineSpacing2(linespacingoptions)).ok()
    }
    pub unsafe fn GetLineSpacing2(&self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetLineSpacing2(linespacingoptions)).ok()
    }
    pub unsafe fn SetFontAxisValues(&self, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetFontAxisValues(::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetFontAxisValueCount(&self) -> u32 {
        ::windows_core::vcall!(self.GetFontAxisValueCount())
    }
    pub unsafe fn GetFontAxisValues(&self, fontaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFontAxisValues(::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetAutomaticFontAxes(&self) -> DWRITE_AUTOMATIC_FONT_AXES {
        ::windows_core::vcall!(self.GetAutomaticFontAxes())
    }
    pub unsafe fn SetAutomaticFontAxes(&self, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAutomaticFontAxes(automaticfontaxes)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextFormat3, ::windows_core::IUnknown, IDWriteTextFormat, IDWriteTextFormat1, IDWriteTextFormat2);
unsafe impl ::core::marker::Send for IDWriteTextFormat3 {}
unsafe impl ::core::marker::Sync for IDWriteTextFormat3 {}
unsafe impl ::windows_core::Interface for IDWriteTextFormat3 {
    type Vtable = IDWriteTextFormat3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextFormat3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6d3b5641_e550_430d_a85b_b7bf48a93427);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextFormat3_Vtbl {
    pub base__: IDWriteTextFormat2_Vtbl,
    pub SetFontAxisValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::HRESULT,
    pub GetFontAxisValueCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetFontAxisValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::HRESULT,
    pub GetAutomaticFontAxes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES,
    pub SetAutomaticFontAxes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextLayout(::windows_core::IUnknown);
impl IDWriteTextLayout {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetTextAlignment(textalignment)).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetParagraphAlignment(paragraphalignment)).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetWordWrapping(wordwrapping)).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetReadingDirection(readingdirection)).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetFlowDirection(flowdirection)).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetIncrementalTabStop(incrementaltabstop)).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteInlineObject>,
    {
        ::windows_core::vcall!(self.base__.SetTrimming(trimmingoptions, trimmingsign.into_param().abi())).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        ::windows_core::vcall!(self.base__.GetTextAlignment())
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        ::windows_core::vcall!(self.base__.GetParagraphAlignment())
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        ::windows_core::vcall!(self.base__.GetWordWrapping())
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        ::windows_core::vcall!(self.base__.GetReadingDirection())
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        ::windows_core::vcall!(self.base__.GetFlowDirection())
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        ::windows_core::vcall!(self.base__.GetIncrementalTabStop())
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetTrimming(trimmingoptions, ::core::mem::transmute(trimmingsign))).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> ::windows_core::Result<IDWriteFontCollection> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontCollection(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.base__.GetFontFamilyNameLength())
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetFontFamilyName(::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        ::windows_core::vcall!(self.base__.GetFontWeight())
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        ::windows_core::vcall!(self.base__.GetFontStyle())
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        ::windows_core::vcall!(self.base__.GetFontStretch())
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        ::windows_core::vcall!(self.base__.GetFontSize())
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.base__.GetLocaleNameLength())
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetLocaleName(::core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap())).ok()
    }
    pub unsafe fn SetMaxWidth(&self, maxwidth: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetMaxWidth(maxwidth)).ok()
    }
    pub unsafe fn SetMaxHeight(&self, maxheight: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetMaxHeight(maxheight)).ok()
    }
    pub unsafe fn SetFontCollection<P0>(&self, fontcollection: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollection>,
    {
        ::windows_core::vcall!(self.SetFontCollection(fontcollection.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontFamilyName<P0>(&self, fontfamilyname: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.SetFontFamilyName(fontfamilyname.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontWeight(&self, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetFontWeight(fontweight, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontStyle(&self, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetFontStyle(fontstyle, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontStretch(&self, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetFontStretch(fontstretch, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontSize(&self, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetFontSize(fontsize, ::core::mem::transmute(textrange))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUnderline<P0>(&self, hasunderline: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetUnderline(hasunderline.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStrikethrough<P0>(&self, hasstrikethrough: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetStrikethrough(hasstrikethrough.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetDrawingEffect<P0>(&self, drawingeffect: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.SetDrawingEffect(drawingeffect.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetInlineObject<P0>(&self, inlineobject: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteInlineObject>,
    {
        ::windows_core::vcall!(self.SetInlineObject(inlineobject.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetTypography<P0>(&self, typography: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTypography>,
    {
        ::windows_core::vcall!(self.SetTypography(typography.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetLocaleName<P0>(&self, localename: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.SetLocaleName(localename.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn GetMaxWidth(&self) -> f32 {
        ::windows_core::vcall!(self.GetMaxWidth())
    }
    pub unsafe fn GetMaxHeight(&self) -> f32 {
        ::windows_core::vcall!(self.GetMaxHeight())
    }
    pub unsafe fn GetFontCollection2(&self, currentposition: u32, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFontCollection2(currentposition, ::core::mem::transmute(fontcollection), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontFamilyNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFontFamilyNameLength2(currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontFamilyName2(&self, currentposition: u32, fontfamilyname: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFontFamilyName2(currentposition, ::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap(), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontWeight2(&self, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFontWeight2(currentposition, fontweight, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontStyle2(&self, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFontStyle2(currentposition, fontstyle, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontStretch2(&self, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFontStretch2(currentposition, fontstretch, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontSize2(&self, currentposition: u32, fontsize: *mut f32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFontSize2(currentposition, fontsize, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUnderline(&self, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetUnderline(currentposition, hasunderline, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStrikethrough(&self, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetStrikethrough(currentposition, hasstrikethrough, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetDrawingEffect(&self, currentposition: u32, drawingeffect: *mut ::core::option::Option<::windows_core::IUnknown>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetDrawingEffect(currentposition, ::core::mem::transmute(drawingeffect), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetInlineObject(&self, currentposition: u32, inlineobject: *mut ::core::option::Option<IDWriteInlineObject>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetInlineObject(currentposition, ::core::mem::transmute(inlineobject), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetTypography(&self, currentposition: u32, typography: *mut ::core::option::Option<IDWriteTypography>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetTypography(currentposition, ::core::mem::transmute(typography), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetLocaleNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetLocaleNameLength2(currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetLocaleName2(&self, currentposition: u32, localename: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetLocaleName2(currentposition, ::core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap(), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn Draw<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, renderer: P0, originx: f32, originy: f32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextRenderer>,
    {
        ::windows_core::vcall!(self.Draw(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), renderer.into_param().abi(), originx, originy)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLineMetrics(&self, linemetrics: ::core::option::Option<&mut [DWRITE_LINE_METRICS]>, actuallinecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetLineMetrics(::core::mem::transmute(linemetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), linemetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actuallinecount)).ok()
    }
    pub unsafe fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetMetrics(textmetrics)).ok()
    }
    pub unsafe fn GetOverhangMetrics(&self) -> ::windows_core::Result<DWRITE_OVERHANG_METRICS> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetOverhangMetrics(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetClusterMetrics(&self, clustermetrics: ::core::option::Option<&mut [DWRITE_CLUSTER_METRICS]>, actualclustercount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetClusterMetrics(::core::mem::transmute(clustermetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), clustermetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actualclustercount)).ok()
    }
    pub unsafe fn DetermineMinWidth(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.DetermineMinWidth(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestPoint(&self, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.HitTestPoint(pointx, pointy, istrailinghit, isinside, hittestmetrics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextPosition<P0>(&self, textposition: u32, istrailinghit: P0, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.HitTestTextPosition(textposition, istrailinghit.into_param().abi(), pointx, pointy, hittestmetrics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextRange(&self, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: ::core::option::Option<&mut [DWRITE_HIT_TEST_METRICS]>, actualhittestmetricscount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.HitTestTextRange(textposition, textlength, originx, originy, ::core::mem::transmute(hittestmetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), hittestmetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actualhittestmetricscount)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextLayout, ::windows_core::IUnknown, IDWriteTextFormat);
unsafe impl ::core::marker::Send for IDWriteTextLayout {}
unsafe impl ::core::marker::Sync for IDWriteTextLayout {}
unsafe impl ::windows_core::Interface for IDWriteTextLayout {
    type Vtable = IDWriteTextLayout_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextLayout {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x53737037_6d14_410b_9bfe_0b182bb70961);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextLayout_Vtbl {
    pub base__: IDWriteTextFormat_Vtbl,
    pub SetMaxWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxwidth: f32) -> ::windows_core::HRESULT,
    pub SetMaxHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxheight: f32) -> ::windows_core::HRESULT,
    pub SetFontCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontcollection: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub SetFontFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfamilyname: ::windows_core::PCWSTR, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub SetFontWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub SetFontStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub SetFontStretch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUnderline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasunderline: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUnderline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStrikethrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasstrikethrough: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStrikethrough: usize,
    pub SetDrawingEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drawingeffect: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub SetInlineObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inlineobject: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub SetTypography: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typography: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub SetLocaleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localename: ::windows_core::PCWSTR, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub GetMaxWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> f32,
    pub GetMaxHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> f32,
    pub GetFontCollection2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32, fontcollection: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub GetFontFamilyNameLength2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub GetFontFamilyName2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32, fontfamilyname: ::windows_core::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub GetFontWeight2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub GetFontStyle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub GetFontStretch2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub GetFontSize2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32, fontsize: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUnderline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUnderline: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetStrikethrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetStrikethrough: usize,
    pub GetDrawingEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32, drawingeffect: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub GetInlineObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32, inlineobject: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub GetTypography: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32, typography: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub GetLocaleNameLength2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub GetLocaleName2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32, localename: ::windows_core::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub Draw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, renderer: *mut ::core::ffi::c_void, originx: f32, originy: f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLineMetrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linemetrics: *mut DWRITE_LINE_METRICS, maxlinecount: u32, actuallinecount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLineMetrics: usize,
    pub GetMetrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textmetrics: *mut DWRITE_TEXT_METRICS) -> ::windows_core::HRESULT,
    pub GetOverhangMetrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, overhangs: *mut DWRITE_OVERHANG_METRICS) -> ::windows_core::HRESULT,
    pub GetClusterMetrics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clustermetrics: *mut DWRITE_CLUSTER_METRICS, maxclustercount: u32, actualclustercount: *mut u32) -> ::windows_core::HRESULT,
    pub DetermineMinWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minwidth: *mut f32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HitTestPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HitTestPoint: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HitTestTextPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textposition: u32, istrailinghit: super::super::Foundation::BOOL, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HitTestTextPosition: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HitTestTextRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS, maxhittestmetricscount: u32, actualhittestmetricscount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HitTestTextRange: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextLayout1(::windows_core::IUnknown);
impl IDWriteTextLayout1 {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetTextAlignment(textalignment)).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetParagraphAlignment(paragraphalignment)).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetWordWrapping(wordwrapping)).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetReadingDirection(readingdirection)).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetFlowDirection(flowdirection)).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetIncrementalTabStop(incrementaltabstop)).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteInlineObject>,
    {
        ::windows_core::vcall!(self.base__.base__.SetTrimming(trimmingoptions, trimmingsign.into_param().abi())).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        ::windows_core::vcall!(self.base__.base__.GetTextAlignment())
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        ::windows_core::vcall!(self.base__.base__.GetParagraphAlignment())
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        ::windows_core::vcall!(self.base__.base__.GetWordWrapping())
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        ::windows_core::vcall!(self.base__.base__.GetReadingDirection())
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        ::windows_core::vcall!(self.base__.base__.GetFlowDirection())
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.GetIncrementalTabStop())
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetTrimming(trimmingoptions, ::core::mem::transmute(trimmingsign))).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> ::windows_core::Result<IDWriteFontCollection> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFontCollection(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.GetFontFamilyNameLength())
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetFontFamilyName(::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        ::windows_core::vcall!(self.base__.base__.GetFontWeight())
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        ::windows_core::vcall!(self.base__.base__.GetFontStyle())
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        ::windows_core::vcall!(self.base__.base__.GetFontStretch())
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.GetFontSize())
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.GetLocaleNameLength())
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetLocaleName(::core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap())).ok()
    }
    pub unsafe fn SetMaxWidth(&self, maxwidth: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetMaxWidth(maxwidth)).ok()
    }
    pub unsafe fn SetMaxHeight(&self, maxheight: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetMaxHeight(maxheight)).ok()
    }
    pub unsafe fn SetFontCollection<P0>(&self, fontcollection: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollection>,
    {
        ::windows_core::vcall!(self.base__.SetFontCollection(fontcollection.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontFamilyName<P0>(&self, fontfamilyname: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.SetFontFamilyName(fontfamilyname.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontWeight(&self, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetFontWeight(fontweight, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontStyle(&self, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetFontStyle(fontstyle, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontStretch(&self, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetFontStretch(fontstretch, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontSize(&self, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetFontSize(fontsize, ::core::mem::transmute(textrange))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUnderline<P0>(&self, hasunderline: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.SetUnderline(hasunderline.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStrikethrough<P0>(&self, hasstrikethrough: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.SetStrikethrough(hasstrikethrough.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetDrawingEffect<P0>(&self, drawingeffect: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.SetDrawingEffect(drawingeffect.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetInlineObject<P0>(&self, inlineobject: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteInlineObject>,
    {
        ::windows_core::vcall!(self.base__.SetInlineObject(inlineobject.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetTypography<P0>(&self, typography: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTypography>,
    {
        ::windows_core::vcall!(self.base__.SetTypography(typography.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetLocaleName<P0>(&self, localename: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.SetLocaleName(localename.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn GetMaxWidth(&self) -> f32 {
        ::windows_core::vcall!(self.base__.GetMaxWidth())
    }
    pub unsafe fn GetMaxHeight(&self) -> f32 {
        ::windows_core::vcall!(self.base__.GetMaxHeight())
    }
    pub unsafe fn GetFontCollection2(&self, currentposition: u32, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetFontCollection2(currentposition, ::core::mem::transmute(fontcollection), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontFamilyNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetFontFamilyNameLength2(currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontFamilyName2(&self, currentposition: u32, fontfamilyname: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetFontFamilyName2(currentposition, ::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap(), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontWeight2(&self, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetFontWeight2(currentposition, fontweight, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontStyle2(&self, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetFontStyle2(currentposition, fontstyle, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontStretch2(&self, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetFontStretch2(currentposition, fontstretch, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontSize2(&self, currentposition: u32, fontsize: *mut f32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetFontSize2(currentposition, fontsize, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUnderline(&self, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetUnderline(currentposition, hasunderline, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStrikethrough(&self, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetStrikethrough(currentposition, hasstrikethrough, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetDrawingEffect(&self, currentposition: u32, drawingeffect: *mut ::core::option::Option<::windows_core::IUnknown>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetDrawingEffect(currentposition, ::core::mem::transmute(drawingeffect), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetInlineObject(&self, currentposition: u32, inlineobject: *mut ::core::option::Option<IDWriteInlineObject>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetInlineObject(currentposition, ::core::mem::transmute(inlineobject), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetTypography(&self, currentposition: u32, typography: *mut ::core::option::Option<IDWriteTypography>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetTypography(currentposition, ::core::mem::transmute(typography), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetLocaleNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetLocaleNameLength2(currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetLocaleName2(&self, currentposition: u32, localename: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetLocaleName2(currentposition, ::core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap(), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn Draw<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, renderer: P0, originx: f32, originy: f32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextRenderer>,
    {
        ::windows_core::vcall!(self.base__.Draw(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), renderer.into_param().abi(), originx, originy)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLineMetrics(&self, linemetrics: ::core::option::Option<&mut [DWRITE_LINE_METRICS]>, actuallinecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetLineMetrics(::core::mem::transmute(linemetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), linemetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actuallinecount)).ok()
    }
    pub unsafe fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetMetrics(textmetrics)).ok()
    }
    pub unsafe fn GetOverhangMetrics(&self) -> ::windows_core::Result<DWRITE_OVERHANG_METRICS> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetOverhangMetrics(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetClusterMetrics(&self, clustermetrics: ::core::option::Option<&mut [DWRITE_CLUSTER_METRICS]>, actualclustercount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetClusterMetrics(::core::mem::transmute(clustermetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), clustermetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actualclustercount)).ok()
    }
    pub unsafe fn DetermineMinWidth(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.DetermineMinWidth(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestPoint(&self, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.HitTestPoint(pointx, pointy, istrailinghit, isinside, hittestmetrics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextPosition<P0>(&self, textposition: u32, istrailinghit: P0, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.HitTestTextPosition(textposition, istrailinghit.into_param().abi(), pointx, pointy, hittestmetrics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextRange(&self, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: ::core::option::Option<&mut [DWRITE_HIT_TEST_METRICS]>, actualhittestmetricscount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.HitTestTextRange(textposition, textlength, originx, originy, ::core::mem::transmute(hittestmetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), hittestmetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actualhittestmetricscount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPairKerning<P0>(&self, ispairkerningenabled: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetPairKerning(ispairkerningenabled.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPairKerning(&self, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetPairKerning(currentposition, ispairkerningenabled, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn SetCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetCharacterSpacing(leadingspacing, trailingspacing, minimumadvancewidth, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn GetCharacterSpacing(&self, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetCharacterSpacing(currentposition, leadingspacing, trailingspacing, minimumadvancewidth, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextLayout1, ::windows_core::IUnknown, IDWriteTextFormat, IDWriteTextLayout);
unsafe impl ::core::marker::Send for IDWriteTextLayout1 {}
unsafe impl ::core::marker::Sync for IDWriteTextLayout1 {}
unsafe impl ::windows_core::Interface for IDWriteTextLayout1 {
    type Vtable = IDWriteTextLayout1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextLayout1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9064d822_80a7_465c_a986_df65f78b8feb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextLayout1_Vtbl {
    pub base__: IDWriteTextLayout_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPairKerning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ispairkerningenabled: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPairKerning: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPairKerning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPairKerning: usize,
    pub SetCharacterSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub GetCharacterSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextLayout2(::windows_core::IUnknown);
impl IDWriteTextLayout2 {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetTextAlignment(textalignment)).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetParagraphAlignment(paragraphalignment)).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetWordWrapping(wordwrapping)).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetReadingDirection(readingdirection)).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetFlowDirection(flowdirection)).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetIncrementalTabStop(incrementaltabstop)).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteInlineObject>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetTrimming(trimmingoptions, trimmingsign.into_param().abi())).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        ::windows_core::vcall!(self.base__.base__.base__.GetTextAlignment())
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        ::windows_core::vcall!(self.base__.base__.base__.GetParagraphAlignment())
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        ::windows_core::vcall!(self.base__.base__.base__.GetWordWrapping())
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        ::windows_core::vcall!(self.base__.base__.base__.GetReadingDirection())
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        ::windows_core::vcall!(self.base__.base__.base__.GetFlowDirection())
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetIncrementalTabStop())
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetTrimming(trimmingoptions, ::core::mem::transmute(trimmingsign))).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> ::windows_core::Result<IDWriteFontCollection> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetFontCollection(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontFamilyNameLength())
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontFamilyName(::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontWeight())
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontStyle())
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontStretch())
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontSize())
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetLocaleNameLength())
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetLocaleName(::core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap())).ok()
    }
    pub unsafe fn SetMaxWidth(&self, maxwidth: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetMaxWidth(maxwidth)).ok()
    }
    pub unsafe fn SetMaxHeight(&self, maxheight: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetMaxHeight(maxheight)).ok()
    }
    pub unsafe fn SetFontCollection<P0>(&self, fontcollection: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollection>,
    {
        ::windows_core::vcall!(self.base__.base__.SetFontCollection(fontcollection.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontFamilyName<P0>(&self, fontfamilyname: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.base__.SetFontFamilyName(fontfamilyname.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontWeight(&self, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetFontWeight(fontweight, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontStyle(&self, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetFontStyle(fontstyle, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontStretch(&self, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetFontStretch(fontstretch, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontSize(&self, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetFontSize(fontsize, ::core::mem::transmute(textrange))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUnderline<P0>(&self, hasunderline: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.SetUnderline(hasunderline.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStrikethrough<P0>(&self, hasstrikethrough: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.SetStrikethrough(hasstrikethrough.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetDrawingEffect<P0>(&self, drawingeffect: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.base__.SetDrawingEffect(drawingeffect.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetInlineObject<P0>(&self, inlineobject: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteInlineObject>,
    {
        ::windows_core::vcall!(self.base__.base__.SetInlineObject(inlineobject.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetTypography<P0>(&self, typography: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTypography>,
    {
        ::windows_core::vcall!(self.base__.base__.SetTypography(typography.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetLocaleName<P0>(&self, localename: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.base__.SetLocaleName(localename.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn GetMaxWidth(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.GetMaxWidth())
    }
    pub unsafe fn GetMaxHeight(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.GetMaxHeight())
    }
    pub unsafe fn GetFontCollection2(&self, currentposition: u32, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetFontCollection2(currentposition, ::core::mem::transmute(fontcollection), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontFamilyNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetFontFamilyNameLength2(currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontFamilyName2(&self, currentposition: u32, fontfamilyname: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetFontFamilyName2(currentposition, ::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap(), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontWeight2(&self, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetFontWeight2(currentposition, fontweight, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontStyle2(&self, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetFontStyle2(currentposition, fontstyle, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontStretch2(&self, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetFontStretch2(currentposition, fontstretch, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontSize2(&self, currentposition: u32, fontsize: *mut f32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetFontSize2(currentposition, fontsize, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUnderline(&self, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetUnderline(currentposition, hasunderline, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStrikethrough(&self, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetStrikethrough(currentposition, hasstrikethrough, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetDrawingEffect(&self, currentposition: u32, drawingeffect: *mut ::core::option::Option<::windows_core::IUnknown>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetDrawingEffect(currentposition, ::core::mem::transmute(drawingeffect), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetInlineObject(&self, currentposition: u32, inlineobject: *mut ::core::option::Option<IDWriteInlineObject>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetInlineObject(currentposition, ::core::mem::transmute(inlineobject), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetTypography(&self, currentposition: u32, typography: *mut ::core::option::Option<IDWriteTypography>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetTypography(currentposition, ::core::mem::transmute(typography), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetLocaleNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetLocaleNameLength2(currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetLocaleName2(&self, currentposition: u32, localename: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetLocaleName2(currentposition, ::core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap(), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn Draw<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, renderer: P0, originx: f32, originy: f32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextRenderer>,
    {
        ::windows_core::vcall!(self.base__.base__.Draw(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), renderer.into_param().abi(), originx, originy)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLineMetrics(&self, linemetrics: ::core::option::Option<&mut [DWRITE_LINE_METRICS]>, actuallinecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetLineMetrics(::core::mem::transmute(linemetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), linemetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actuallinecount)).ok()
    }
    pub unsafe fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetMetrics(textmetrics)).ok()
    }
    pub unsafe fn GetOverhangMetrics(&self) -> ::windows_core::Result<DWRITE_OVERHANG_METRICS> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetOverhangMetrics(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetClusterMetrics(&self, clustermetrics: ::core::option::Option<&mut [DWRITE_CLUSTER_METRICS]>, actualclustercount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetClusterMetrics(::core::mem::transmute(clustermetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), clustermetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actualclustercount)).ok()
    }
    pub unsafe fn DetermineMinWidth(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.DetermineMinWidth(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestPoint(&self, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.HitTestPoint(pointx, pointy, istrailinghit, isinside, hittestmetrics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextPosition<P0>(&self, textposition: u32, istrailinghit: P0, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.HitTestTextPosition(textposition, istrailinghit.into_param().abi(), pointx, pointy, hittestmetrics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextRange(&self, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: ::core::option::Option<&mut [DWRITE_HIT_TEST_METRICS]>, actualhittestmetricscount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.HitTestTextRange(textposition, textlength, originx, originy, ::core::mem::transmute(hittestmetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), hittestmetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actualhittestmetricscount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPairKerning<P0>(&self, ispairkerningenabled: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.SetPairKerning(ispairkerningenabled.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPairKerning(&self, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetPairKerning(currentposition, ispairkerningenabled, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn SetCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetCharacterSpacing(leadingspacing, trailingspacing, minimumadvancewidth, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn GetCharacterSpacing(&self, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetCharacterSpacing(currentposition, leadingspacing, trailingspacing, minimumadvancewidth, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetMetrics2(&self, textmetrics: *mut DWRITE_TEXT_METRICS1) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetMetrics2(textmetrics)).ok()
    }
    pub unsafe fn SetVerticalGlyphOrientation(&self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetVerticalGlyphOrientation(glyphorientation)).ok()
    }
    pub unsafe fn GetVerticalGlyphOrientation(&self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
        ::windows_core::vcall!(self.GetVerticalGlyphOrientation())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastLineWrapping<P0>(&self, islastlinewrappingenabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.SetLastLineWrapping(islastlinewrappingenabled.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastLineWrapping(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.GetLastLineWrapping())
    }
    pub unsafe fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetOpticalAlignment(opticalalignment)).ok()
    }
    pub unsafe fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT {
        ::windows_core::vcall!(self.GetOpticalAlignment())
    }
    pub unsafe fn SetFontFallback<P0>(&self, fontfallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFallback>,
    {
        ::windows_core::vcall!(self.SetFontFallback(fontfallback.into_param().abi())).ok()
    }
    pub unsafe fn GetFontFallback(&self) -> ::windows_core::Result<IDWriteFontFallback> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontFallback(&mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextLayout2, ::windows_core::IUnknown, IDWriteTextFormat, IDWriteTextLayout, IDWriteTextLayout1);
unsafe impl ::core::marker::Send for IDWriteTextLayout2 {}
unsafe impl ::core::marker::Sync for IDWriteTextLayout2 {}
unsafe impl ::windows_core::Interface for IDWriteTextLayout2 {
    type Vtable = IDWriteTextLayout2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextLayout2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1093c18f_8d5e_43f0_b064_0917311b525e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextLayout2_Vtbl {
    pub base__: IDWriteTextLayout1_Vtbl,
    pub GetMetrics2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textmetrics: *mut DWRITE_TEXT_METRICS1) -> ::windows_core::HRESULT,
    pub SetVerticalGlyphOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows_core::HRESULT,
    pub GetVerticalGlyphOrientation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_VERTICAL_GLYPH_ORIENTATION,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLastLineWrapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, islastlinewrappingenabled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLastLineWrapping: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastLineWrapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastLineWrapping: usize,
    pub SetOpticalAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows_core::HRESULT,
    pub GetOpticalAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT,
    pub SetFontFallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFontFallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfallback: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextLayout3(::windows_core::IUnknown);
impl IDWriteTextLayout3 {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetTextAlignment(textalignment)).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetParagraphAlignment(paragraphalignment)).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetWordWrapping(wordwrapping)).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetReadingDirection(readingdirection)).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetFlowDirection(flowdirection)).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetIncrementalTabStop(incrementaltabstop)).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteInlineObject>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetTrimming(trimmingoptions, trimmingsign.into_param().abi())).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetTextAlignment())
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetParagraphAlignment())
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetWordWrapping())
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetReadingDirection())
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFlowDirection())
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetIncrementalTabStop())
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetTrimming(trimmingoptions, ::core::mem::transmute(trimmingsign))).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> ::windows_core::Result<IDWriteFontCollection> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFontCollection(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFontFamilyNameLength())
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFontFamilyName(::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFontWeight())
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFontStyle())
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFontStretch())
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFontSize())
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetLocaleNameLength())
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetLocaleName(::core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap())).ok()
    }
    pub unsafe fn SetMaxWidth(&self, maxwidth: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetMaxWidth(maxwidth)).ok()
    }
    pub unsafe fn SetMaxHeight(&self, maxheight: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetMaxHeight(maxheight)).ok()
    }
    pub unsafe fn SetFontCollection<P0>(&self, fontcollection: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollection>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetFontCollection(fontcollection.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontFamilyName<P0>(&self, fontfamilyname: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetFontFamilyName(fontfamilyname.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontWeight(&self, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetFontWeight(fontweight, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontStyle(&self, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetFontStyle(fontstyle, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontStretch(&self, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetFontStretch(fontstretch, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontSize(&self, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetFontSize(fontsize, ::core::mem::transmute(textrange))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUnderline<P0>(&self, hasunderline: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetUnderline(hasunderline.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStrikethrough<P0>(&self, hasstrikethrough: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetStrikethrough(hasstrikethrough.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetDrawingEffect<P0>(&self, drawingeffect: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetDrawingEffect(drawingeffect.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetInlineObject<P0>(&self, inlineobject: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteInlineObject>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetInlineObject(inlineobject.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetTypography<P0>(&self, typography: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTypography>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetTypography(typography.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetLocaleName<P0>(&self, localename: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetLocaleName(localename.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn GetMaxWidth(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetMaxWidth())
    }
    pub unsafe fn GetMaxHeight(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.base__.GetMaxHeight())
    }
    pub unsafe fn GetFontCollection2(&self, currentposition: u32, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontCollection2(currentposition, ::core::mem::transmute(fontcollection), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontFamilyNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontFamilyNameLength2(currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontFamilyName2(&self, currentposition: u32, fontfamilyname: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontFamilyName2(currentposition, ::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap(), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontWeight2(&self, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontWeight2(currentposition, fontweight, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontStyle2(&self, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontStyle2(currentposition, fontstyle, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontStretch2(&self, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontStretch2(currentposition, fontstretch, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontSize2(&self, currentposition: u32, fontsize: *mut f32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetFontSize2(currentposition, fontsize, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUnderline(&self, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetUnderline(currentposition, hasunderline, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStrikethrough(&self, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetStrikethrough(currentposition, hasstrikethrough, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetDrawingEffect(&self, currentposition: u32, drawingeffect: *mut ::core::option::Option<::windows_core::IUnknown>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetDrawingEffect(currentposition, ::core::mem::transmute(drawingeffect), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetInlineObject(&self, currentposition: u32, inlineobject: *mut ::core::option::Option<IDWriteInlineObject>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetInlineObject(currentposition, ::core::mem::transmute(inlineobject), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetTypography(&self, currentposition: u32, typography: *mut ::core::option::Option<IDWriteTypography>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetTypography(currentposition, ::core::mem::transmute(typography), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetLocaleNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetLocaleNameLength2(currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetLocaleName2(&self, currentposition: u32, localename: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetLocaleName2(currentposition, ::core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap(), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn Draw<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, renderer: P0, originx: f32, originy: f32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextRenderer>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.Draw(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), renderer.into_param().abi(), originx, originy)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLineMetrics(&self, linemetrics: ::core::option::Option<&mut [DWRITE_LINE_METRICS]>, actuallinecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetLineMetrics(::core::mem::transmute(linemetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), linemetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actuallinecount)).ok()
    }
    pub unsafe fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetMetrics(textmetrics)).ok()
    }
    pub unsafe fn GetOverhangMetrics(&self) -> ::windows_core::Result<DWRITE_OVERHANG_METRICS> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.GetOverhangMetrics(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetClusterMetrics(&self, clustermetrics: ::core::option::Option<&mut [DWRITE_CLUSTER_METRICS]>, actualclustercount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetClusterMetrics(::core::mem::transmute(clustermetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), clustermetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actualclustercount)).ok()
    }
    pub unsafe fn DetermineMinWidth(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.DetermineMinWidth(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestPoint(&self, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.HitTestPoint(pointx, pointy, istrailinghit, isinside, hittestmetrics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextPosition<P0>(&self, textposition: u32, istrailinghit: P0, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.HitTestTextPosition(textposition, istrailinghit.into_param().abi(), pointx, pointy, hittestmetrics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextRange(&self, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: ::core::option::Option<&mut [DWRITE_HIT_TEST_METRICS]>, actualhittestmetricscount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.HitTestTextRange(textposition, textlength, originx, originy, ::core::mem::transmute(hittestmetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), hittestmetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actualhittestmetricscount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPairKerning<P0>(&self, ispairkerningenabled: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.SetPairKerning(ispairkerningenabled.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPairKerning(&self, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetPairKerning(currentposition, ispairkerningenabled, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn SetCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetCharacterSpacing(leadingspacing, trailingspacing, minimumadvancewidth, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn GetCharacterSpacing(&self, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetCharacterSpacing(currentposition, leadingspacing, trailingspacing, minimumadvancewidth, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetMetrics2(&self, textmetrics: *mut DWRITE_TEXT_METRICS1) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetMetrics2(textmetrics)).ok()
    }
    pub unsafe fn SetVerticalGlyphOrientation(&self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetVerticalGlyphOrientation(glyphorientation)).ok()
    }
    pub unsafe fn GetVerticalGlyphOrientation(&self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
        ::windows_core::vcall!(self.base__.GetVerticalGlyphOrientation())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastLineWrapping<P0>(&self, islastlinewrappingenabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.SetLastLineWrapping(islastlinewrappingenabled.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastLineWrapping(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.GetLastLineWrapping())
    }
    pub unsafe fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetOpticalAlignment(opticalalignment)).ok()
    }
    pub unsafe fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT {
        ::windows_core::vcall!(self.base__.GetOpticalAlignment())
    }
    pub unsafe fn SetFontFallback<P0>(&self, fontfallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFallback>,
    {
        ::windows_core::vcall!(self.base__.SetFontFallback(fontfallback.into_param().abi())).ok()
    }
    pub unsafe fn GetFontFallback(&self) -> ::windows_core::Result<IDWriteFontFallback> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetFontFallback(&mut result__)).from_abi(result__)
    }
    pub unsafe fn InvalidateLayout(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.InvalidateLayout()).ok()
    }
    pub unsafe fn SetLineSpacing2(&self, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetLineSpacing2(linespacingoptions)).ok()
    }
    pub unsafe fn GetLineSpacing2(&self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetLineSpacing2(linespacingoptions)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLineMetrics2(&self, linemetrics: ::core::option::Option<&mut [DWRITE_LINE_METRICS1]>, actuallinecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetLineMetrics2(::core::mem::transmute(linemetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), linemetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actuallinecount)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextLayout3, ::windows_core::IUnknown, IDWriteTextFormat, IDWriteTextLayout, IDWriteTextLayout1, IDWriteTextLayout2);
unsafe impl ::core::marker::Send for IDWriteTextLayout3 {}
unsafe impl ::core::marker::Sync for IDWriteTextLayout3 {}
unsafe impl ::windows_core::Interface for IDWriteTextLayout3 {
    type Vtable = IDWriteTextLayout3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextLayout3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x07ddcd52_020e_4de8_ac33_6c953d83f92d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextLayout3_Vtbl {
    pub base__: IDWriteTextLayout2_Vtbl,
    pub InvalidateLayout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetLineSpacing2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows_core::HRESULT,
    pub GetLineSpacing2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLineMetrics2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linemetrics: *mut DWRITE_LINE_METRICS1, maxlinecount: u32, actuallinecount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLineMetrics2: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextLayout4(::windows_core::IUnknown);
impl IDWriteTextLayout4 {
    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.SetTextAlignment(textalignment)).ok()
    }
    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.SetParagraphAlignment(paragraphalignment)).ok()
    }
    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.SetWordWrapping(wordwrapping)).ok()
    }
    pub unsafe fn SetReadingDirection(&self, readingdirection: DWRITE_READING_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.SetReadingDirection(readingdirection)).ok()
    }
    pub unsafe fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.SetFlowDirection(flowdirection)).ok()
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.SetIncrementalTabStop(incrementaltabstop)).ok()
    }
    pub unsafe fn SetTrimming<P0>(&self, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteInlineObject>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.SetTrimming(trimmingoptions, trimmingsign.into_param().abi())).ok()
    }
    pub unsafe fn SetLineSpacing(&self, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.SetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetTextAlignment())
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetParagraphAlignment())
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetWordWrapping())
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetReadingDirection())
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetFlowDirection())
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetIncrementalTabStop())
    }
    pub unsafe fn GetTrimming(&self, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetTrimming(trimmingoptions, ::core::mem::transmute(trimmingsign))).ok()
    }
    pub unsafe fn GetLineSpacing(&self, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetLineSpacing(linespacingmethod, linespacing, baseline)).ok()
    }
    pub unsafe fn GetFontCollection(&self) -> ::windows_core::Result<IDWriteFontCollection> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetFontCollection(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetFontFamilyNameLength())
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetFontFamilyName(::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap())).ok()
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetFontWeight())
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetFontStyle())
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetFontStretch())
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetFontSize())
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetLocaleNameLength())
    }
    pub unsafe fn GetLocaleName(&self, localename: &mut [u16]) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.base__.GetLocaleName(::core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap())).ok()
    }
    pub unsafe fn SetMaxWidth(&self, maxwidth: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetMaxWidth(maxwidth)).ok()
    }
    pub unsafe fn SetMaxHeight(&self, maxheight: f32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetMaxHeight(maxheight)).ok()
    }
    pub unsafe fn SetFontCollection<P0>(&self, fontcollection: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontCollection>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetFontCollection(fontcollection.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontFamilyName<P0>(&self, fontfamilyname: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetFontFamilyName(fontfamilyname.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontWeight(&self, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetFontWeight(fontweight, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontStyle(&self, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetFontStyle(fontstyle, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontStretch(&self, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetFontStretch(fontstretch, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetFontSize(&self, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetFontSize(fontsize, ::core::mem::transmute(textrange))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUnderline<P0>(&self, hasunderline: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetUnderline(hasunderline.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStrikethrough<P0>(&self, hasstrikethrough: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetStrikethrough(hasstrikethrough.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetDrawingEffect<P0>(&self, drawingeffect: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetDrawingEffect(drawingeffect.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetInlineObject<P0>(&self, inlineobject: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteInlineObject>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetInlineObject(inlineobject.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetTypography<P0>(&self, typography: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTypography>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetTypography(typography.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn SetLocaleName<P0>(&self, localename: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.SetLocaleName(localename.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn GetMaxWidth(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetMaxWidth())
    }
    pub unsafe fn GetMaxHeight(&self) -> f32 {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetMaxHeight())
    }
    pub unsafe fn GetFontCollection2(&self, currentposition: u32, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFontCollection2(currentposition, ::core::mem::transmute(fontcollection), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontFamilyNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFontFamilyNameLength2(currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontFamilyName2(&self, currentposition: u32, fontfamilyname: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFontFamilyName2(currentposition, ::core::mem::transmute(fontfamilyname.as_ptr()), fontfamilyname.len().try_into().unwrap(), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontWeight2(&self, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFontWeight2(currentposition, fontweight, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontStyle2(&self, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFontStyle2(currentposition, fontstyle, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontStretch2(&self, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFontStretch2(currentposition, fontstretch, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetFontSize2(&self, currentposition: u32, fontsize: *mut f32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetFontSize2(currentposition, fontsize, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUnderline(&self, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetUnderline(currentposition, hasunderline, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetStrikethrough(&self, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetStrikethrough(currentposition, hasstrikethrough, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetDrawingEffect(&self, currentposition: u32, drawingeffect: *mut ::core::option::Option<::windows_core::IUnknown>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetDrawingEffect(currentposition, ::core::mem::transmute(drawingeffect), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetInlineObject(&self, currentposition: u32, inlineobject: *mut ::core::option::Option<IDWriteInlineObject>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetInlineObject(currentposition, ::core::mem::transmute(inlineobject), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetTypography(&self, currentposition: u32, typography: *mut ::core::option::Option<IDWriteTypography>, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetTypography(currentposition, ::core::mem::transmute(typography), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetLocaleNameLength2(&self, currentposition: u32, namelength: *mut u32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetLocaleNameLength2(currentposition, namelength, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetLocaleName2(&self, currentposition: u32, localename: &mut [u16], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetLocaleName2(currentposition, ::core::mem::transmute(localename.as_ptr()), localename.len().try_into().unwrap(), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn Draw<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, renderer: P0, originx: f32, originy: f32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteTextRenderer>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.Draw(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), renderer.into_param().abi(), originx, originy)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLineMetrics(&self, linemetrics: ::core::option::Option<&mut [DWRITE_LINE_METRICS]>, actuallinecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetLineMetrics(::core::mem::transmute(linemetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), linemetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actuallinecount)).ok()
    }
    pub unsafe fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetMetrics(textmetrics)).ok()
    }
    pub unsafe fn GetOverhangMetrics(&self) -> ::windows_core::Result<DWRITE_OVERHANG_METRICS> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetOverhangMetrics(&mut result__)).from_abi(result__)
    }
    pub unsafe fn GetClusterMetrics(&self, clustermetrics: ::core::option::Option<&mut [DWRITE_CLUSTER_METRICS]>, actualclustercount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.GetClusterMetrics(::core::mem::transmute(clustermetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), clustermetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actualclustercount)).ok()
    }
    pub unsafe fn DetermineMinWidth(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.base__.base__.DetermineMinWidth(&mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestPoint(&self, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.HitTestPoint(pointx, pointy, istrailinghit, isinside, hittestmetrics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextPosition<P0>(&self, textposition: u32, istrailinghit: P0, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.base__.HitTestTextPosition(textposition, istrailinghit.into_param().abi(), pointx, pointy, hittestmetrics)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTestTextRange(&self, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: ::core::option::Option<&mut [DWRITE_HIT_TEST_METRICS]>, actualhittestmetricscount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.base__.HitTestTextRange(textposition, textlength, originx, originy, ::core::mem::transmute(hittestmetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), hittestmetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actualhittestmetricscount)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPairKerning<P0>(&self, ispairkerningenabled: P0, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.base__.SetPairKerning(ispairkerningenabled.into_param().abi(), ::core::mem::transmute(textrange))).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPairKerning(&self, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetPairKerning(currentposition, ispairkerningenabled, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn SetCharacterSpacing(&self, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.SetCharacterSpacing(leadingspacing, trailingspacing, minimumadvancewidth, ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn GetCharacterSpacing(&self, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.base__.GetCharacterSpacing(currentposition, leadingspacing, trailingspacing, minimumadvancewidth, ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetMetrics2(&self, textmetrics: *mut DWRITE_TEXT_METRICS1) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetMetrics2(textmetrics)).ok()
    }
    pub unsafe fn SetVerticalGlyphOrientation(&self, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetVerticalGlyphOrientation(glyphorientation)).ok()
    }
    pub unsafe fn GetVerticalGlyphOrientation(&self) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
        ::windows_core::vcall!(self.base__.base__.GetVerticalGlyphOrientation())
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastLineWrapping<P0>(&self, islastlinewrappingenabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        ::windows_core::vcall!(self.base__.base__.SetLastLineWrapping(islastlinewrappingenabled.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastLineWrapping(&self) -> super::super::Foundation::BOOL {
        ::windows_core::vcall!(self.base__.base__.GetLastLineWrapping())
    }
    pub unsafe fn SetOpticalAlignment(&self, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.SetOpticalAlignment(opticalalignment)).ok()
    }
    pub unsafe fn GetOpticalAlignment(&self) -> DWRITE_OPTICAL_ALIGNMENT {
        ::windows_core::vcall!(self.base__.base__.GetOpticalAlignment())
    }
    pub unsafe fn SetFontFallback<P0>(&self, fontfallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteFontFallback>,
    {
        ::windows_core::vcall!(self.base__.base__.SetFontFallback(fontfallback.into_param().abi())).ok()
    }
    pub unsafe fn GetFontFallback(&self) -> ::windows_core::Result<IDWriteFontFallback> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetFontFallback(&mut result__)).from_abi(result__)
    }
    pub unsafe fn InvalidateLayout(&self) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.InvalidateLayout()).ok()
    }
    pub unsafe fn SetLineSpacing2(&self, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.SetLineSpacing2(linespacingoptions)).ok()
    }
    pub unsafe fn GetLineSpacing2(&self, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetLineSpacing2(linespacingoptions)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLineMetrics2(&self, linemetrics: ::core::option::Option<&mut [DWRITE_LINE_METRICS1]>, actuallinecount: *mut u32) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetLineMetrics2(::core::mem::transmute(linemetrics.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), linemetrics.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), actuallinecount)).ok()
    }
    pub unsafe fn SetFontAxisValues(&self, fontaxisvalues: &[DWRITE_FONT_AXIS_VALUE], textrange: DWRITE_TEXT_RANGE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetFontAxisValues(::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), ::core::mem::transmute(textrange))).ok()
    }
    pub unsafe fn GetFontAxisValueCount(&self, currentposition: u32) -> u32 {
        ::windows_core::vcall!(self.GetFontAxisValueCount(currentposition))
    }
    pub unsafe fn GetFontAxisValues(&self, currentposition: u32, fontaxisvalues: &mut [DWRITE_FONT_AXIS_VALUE], textrange: ::core::option::Option<*mut DWRITE_TEXT_RANGE>) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.GetFontAxisValues(currentposition, ::core::mem::transmute(fontaxisvalues.as_ptr()), fontaxisvalues.len().try_into().unwrap(), ::core::mem::transmute(textrange.unwrap_or(::std::ptr::null_mut())))).ok()
    }
    pub unsafe fn GetAutomaticFontAxes(&self) -> DWRITE_AUTOMATIC_FONT_AXES {
        ::windows_core::vcall!(self.GetAutomaticFontAxes())
    }
    pub unsafe fn SetAutomaticFontAxes(&self, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.SetAutomaticFontAxes(automaticfontaxes)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextLayout4, ::windows_core::IUnknown, IDWriteTextFormat, IDWriteTextLayout, IDWriteTextLayout1, IDWriteTextLayout2, IDWriteTextLayout3);
unsafe impl ::core::marker::Send for IDWriteTextLayout4 {}
unsafe impl ::core::marker::Sync for IDWriteTextLayout4 {}
unsafe impl ::windows_core::Interface for IDWriteTextLayout4 {
    type Vtable = IDWriteTextLayout4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextLayout4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05a9bf42_223f_4441_b5fb_8263685f55e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextLayout4_Vtbl {
    pub base__: IDWriteTextLayout3_Vtbl,
    pub SetFontAxisValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub GetFontAxisValueCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32) -> u32,
    pub GetFontAxisValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, currentposition: u32, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT,
    pub GetAutomaticFontAxes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES,
    pub SetAutomaticFontAxes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextRenderer(::windows_core::IUnknown);
impl IDWriteTextRenderer {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPixelSnappingDisabled(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.IsPixelSnappingDisabled(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetCurrentTransform(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, transform: *mut DWRITE_MATRIX) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.GetCurrentTransform(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), transform)).ok()
    }
    pub unsafe fn GetPixelsPerDip(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.GetPixelsPerDip(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DrawGlyphRun<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.DrawGlyphRun(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), baselineoriginx, baselineoriginy, measuringmode, glyphrun, glyphrundescription, clientdrawingeffect.into_param().abi())).ok()
    }
    pub unsafe fn DrawUnderline<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.DrawUnderline(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), baselineoriginx, baselineoriginy, underline, clientdrawingeffect.into_param().abi())).ok()
    }
    pub unsafe fn DrawStrikethrough<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.DrawStrikethrough(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), baselineoriginx, baselineoriginy, strikethrough, clientdrawingeffect.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DrawInlineObject<P0, P1, P2, P3>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, originx: f32, originy: f32, inlineobject: P0, issideways: P1, isrighttoleft: P2, clientdrawingeffect: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteInlineObject>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.DrawInlineObject(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), originx, originy, inlineobject.into_param().abi(), issideways.into_param().abi(), isrighttoleft.into_param().abi(), clientdrawingeffect.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextRenderer, ::windows_core::IUnknown, IDWritePixelSnapping);
unsafe impl ::core::marker::Send for IDWriteTextRenderer {}
unsafe impl ::core::marker::Sync for IDWriteTextRenderer {}
unsafe impl ::windows_core::Interface for IDWriteTextRenderer {
    type Vtable = IDWriteTextRenderer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextRenderer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef8a8135_5cc6_45fe_8825_c5a0724eb819);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextRenderer_Vtbl {
    pub base__: IDWritePixelSnapping_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DrawGlyphRun: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DrawGlyphRun: usize,
    pub DrawUnderline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DrawStrikethrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DrawInlineObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, originx: f32, originy: f32, inlineobject: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DrawInlineObject: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTextRenderer1(::windows_core::IUnknown);
impl IDWriteTextRenderer1 {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPixelSnappingDisabled(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.IsPixelSnappingDisabled(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
    pub unsafe fn GetCurrentTransform(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, transform: *mut DWRITE_MATRIX) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.base__.base__.GetCurrentTransform(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), transform)).ok()
    }
    pub unsafe fn GetPixelsPerDip(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.base__.base__.GetPixelsPerDip(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), &mut result__)).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DrawGlyphRun<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.DrawGlyphRun(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), baselineoriginx, baselineoriginy, measuringmode, glyphrun, glyphrundescription, clientdrawingeffect.into_param().abi())).ok()
    }
    pub unsafe fn DrawUnderline<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.DrawUnderline(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), baselineoriginx, baselineoriginy, underline, clientdrawingeffect.into_param().abi())).ok()
    }
    pub unsafe fn DrawStrikethrough<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.DrawStrikethrough(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), baselineoriginx, baselineoriginy, strikethrough, clientdrawingeffect.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DrawInlineObject<P0, P1, P2, P3>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, originx: f32, originy: f32, inlineobject: P0, issideways: P1, isrighttoleft: P2, clientdrawingeffect: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteInlineObject>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.base__.DrawInlineObject(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), originx, originy, inlineobject.into_param().abi(), issideways.into_param().abi(), isrighttoleft.into_param().abi(), clientdrawingeffect.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DrawGlyphRun2<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.DrawGlyphRun2(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), baselineoriginx, baselineoriginy, orientationangle, measuringmode, glyphrun, glyphrundescription, clientdrawingeffect.into_param().abi())).ok()
    }
    pub unsafe fn DrawUnderline2<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.DrawUnderline2(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), baselineoriginx, baselineoriginy, orientationangle, underline, clientdrawingeffect.into_param().abi())).ok()
    }
    pub unsafe fn DrawStrikethrough2<P0>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.DrawStrikethrough2(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), baselineoriginx, baselineoriginy, orientationangle, strikethrough, clientdrawingeffect.into_param().abi())).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DrawInlineObject2<P0, P1, P2, P3>(&self, clientdrawingcontext: ::core::option::Option<*const ::core::ffi::c_void>, originx: f32, originy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, inlineobject: P0, issideways: P1, isrighttoleft: P2, clientdrawingeffect: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IDWriteInlineObject>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        ::windows_core::vcall!(self.DrawInlineObject2(::core::mem::transmute(clientdrawingcontext.unwrap_or(::std::ptr::null())), originx, originy, orientationangle, inlineobject.into_param().abi(), issideways.into_param().abi(), isrighttoleft.into_param().abi(), clientdrawingeffect.into_param().abi())).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTextRenderer1, ::windows_core::IUnknown, IDWritePixelSnapping, IDWriteTextRenderer);
unsafe impl ::core::marker::Send for IDWriteTextRenderer1 {}
unsafe impl ::core::marker::Sync for IDWriteTextRenderer1 {}
unsafe impl ::windows_core::Interface for IDWriteTextRenderer1 {
    type Vtable = IDWriteTextRenderer1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTextRenderer1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd3e0e934_22a0_427e_aae4_7d9574b59db1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextRenderer1_Vtbl {
    pub base__: IDWriteTextRenderer_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DrawGlyphRun2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DrawGlyphRun2: usize,
    pub DrawUnderline2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DrawStrikethrough2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DrawInlineObject2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, originx: f32, originy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, inlineobject: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DrawInlineObject2: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDWriteTypography(::windows_core::IUnknown);
impl IDWriteTypography {
    pub unsafe fn AddFontFeature(&self, fontfeature: DWRITE_FONT_FEATURE) -> ::windows_core::Result<()> {
        ::windows_core::vcall!(self.AddFontFeature(::core::mem::transmute(fontfeature))).ok()
    }
    pub unsafe fn GetFontFeatureCount(&self) -> u32 {
        ::windows_core::vcall!(self.GetFontFeatureCount())
    }
    pub unsafe fn GetFontFeature(&self, fontfeatureindex: u32) -> ::windows_core::Result<DWRITE_FONT_FEATURE> {
        let mut result__ = ::std::mem::zeroed();
        ::windows_core::vcall!(self.GetFontFeature(fontfeatureindex, &mut result__)).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IDWriteTypography, ::windows_core::IUnknown);
unsafe impl ::core::marker::Send for IDWriteTypography {}
unsafe impl ::core::marker::Sync for IDWriteTypography {}
unsafe impl ::windows_core::Interface for IDWriteTypography {
    type Vtable = IDWriteTypography_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDWriteTypography {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55f1112b_1dc2_4b3c_9541_f46894ed85b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTypography_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddFontFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfeature: DWRITE_FONT_FEATURE) -> ::windows_core::HRESULT,
    pub GetFontFeatureCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetFontFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfeatureindex: u32, fontfeature: *mut DWRITE_FONT_FEATURE) -> ::windows_core::HRESULT,
}
pub const DWRITE_ALPHA_MAX: u32 = 255u32;
pub const DWRITE_AUTOMATIC_FONT_AXES_NONE: DWRITE_AUTOMATIC_FONT_AXES = DWRITE_AUTOMATIC_FONT_AXES(0i32);
pub const DWRITE_AUTOMATIC_FONT_AXES_OPTICAL_SIZE: DWRITE_AUTOMATIC_FONT_AXES = DWRITE_AUTOMATIC_FONT_AXES(1i32);
pub const DWRITE_BASELINE_CENTRAL: DWRITE_BASELINE = DWRITE_BASELINE(2i32);
pub const DWRITE_BASELINE_DEFAULT: DWRITE_BASELINE = DWRITE_BASELINE(0i32);
pub const DWRITE_BASELINE_HANGING: DWRITE_BASELINE = DWRITE_BASELINE(4i32);
pub const DWRITE_BASELINE_IDEOGRAPHIC_BOTTOM: DWRITE_BASELINE = DWRITE_BASELINE(5i32);
pub const DWRITE_BASELINE_IDEOGRAPHIC_TOP: DWRITE_BASELINE = DWRITE_BASELINE(6i32);
pub const DWRITE_BASELINE_MATH: DWRITE_BASELINE = DWRITE_BASELINE(3i32);
pub const DWRITE_BASELINE_MAXIMUM: DWRITE_BASELINE = DWRITE_BASELINE(8i32);
pub const DWRITE_BASELINE_MINIMUM: DWRITE_BASELINE = DWRITE_BASELINE(7i32);
pub const DWRITE_BASELINE_ROMAN: DWRITE_BASELINE = DWRITE_BASELINE(1i32);
pub const DWRITE_BREAK_CONDITION_CAN_BREAK: DWRITE_BREAK_CONDITION = DWRITE_BREAK_CONDITION(1i32);
pub const DWRITE_BREAK_CONDITION_MAY_NOT_BREAK: DWRITE_BREAK_CONDITION = DWRITE_BREAK_CONDITION(2i32);
pub const DWRITE_BREAK_CONDITION_MUST_BREAK: DWRITE_BREAK_CONDITION = DWRITE_BREAK_CONDITION(3i32);
pub const DWRITE_BREAK_CONDITION_NEUTRAL: DWRITE_BREAK_CONDITION = DWRITE_BREAK_CONDITION(0i32);
pub const DWRITE_CONTAINER_TYPE_UNKNOWN: DWRITE_CONTAINER_TYPE = DWRITE_CONTAINER_TYPE(0i32);
pub const DWRITE_CONTAINER_TYPE_WOFF: DWRITE_CONTAINER_TYPE = DWRITE_CONTAINER_TYPE(1i32);
pub const DWRITE_CONTAINER_TYPE_WOFF2: DWRITE_CONTAINER_TYPE = DWRITE_CONTAINER_TYPE(2i32);
pub const DWRITE_ERR_BASE: u32 = 20480u32;
pub const DWRITE_E_DOWNLOADCANCELLED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2003283954i32);
pub const DWRITE_E_DOWNLOADFAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2003283953i32);
pub const DWRITE_E_REMOTEFONT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2003283955i32);
pub const DWRITE_E_TOOMANYDOWNLOADS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2003283952i32);
pub const DWRITE_FACTORY_TYPE_ISOLATED: DWRITE_FACTORY_TYPE = DWRITE_FACTORY_TYPE(1i32);
pub const DWRITE_FACTORY_TYPE_SHARED: DWRITE_FACTORY_TYPE = DWRITE_FACTORY_TYPE(0i32);
pub const DWRITE_FLOW_DIRECTION_BOTTOM_TO_TOP: DWRITE_FLOW_DIRECTION = DWRITE_FLOW_DIRECTION(1i32);
pub const DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT: DWRITE_FLOW_DIRECTION = DWRITE_FLOW_DIRECTION(2i32);
pub const DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT: DWRITE_FLOW_DIRECTION = DWRITE_FLOW_DIRECTION(3i32);
pub const DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM: DWRITE_FLOW_DIRECTION = DWRITE_FLOW_DIRECTION(0i32);
pub const DWRITE_FONT_AXIS_ATTRIBUTES_HIDDEN: DWRITE_FONT_AXIS_ATTRIBUTES = DWRITE_FONT_AXIS_ATTRIBUTES(2i32);
pub const DWRITE_FONT_AXIS_ATTRIBUTES_NONE: DWRITE_FONT_AXIS_ATTRIBUTES = DWRITE_FONT_AXIS_ATTRIBUTES(0i32);
pub const DWRITE_FONT_AXIS_ATTRIBUTES_VARIABLE: DWRITE_FONT_AXIS_ATTRIBUTES = DWRITE_FONT_AXIS_ATTRIBUTES(1i32);
pub const DWRITE_FONT_AXIS_TAG_ITALIC: DWRITE_FONT_AXIS_TAG = DWRITE_FONT_AXIS_TAG(1818326121u32);
pub const DWRITE_FONT_AXIS_TAG_OPTICAL_SIZE: DWRITE_FONT_AXIS_TAG = DWRITE_FONT_AXIS_TAG(2054385775u32);
pub const DWRITE_FONT_AXIS_TAG_SLANT: DWRITE_FONT_AXIS_TAG = DWRITE_FONT_AXIS_TAG(1953393779u32);
pub const DWRITE_FONT_AXIS_TAG_WEIGHT: DWRITE_FONT_AXIS_TAG = DWRITE_FONT_AXIS_TAG(1952999287u32);
pub const DWRITE_FONT_AXIS_TAG_WIDTH: DWRITE_FONT_AXIS_TAG = DWRITE_FONT_AXIS_TAG(1752458359u32);
pub const DWRITE_FONT_FACE_TYPE_BITMAP: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(5i32);
pub const DWRITE_FONT_FACE_TYPE_CFF: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(0i32);
pub const DWRITE_FONT_FACE_TYPE_OPENTYPE_COLLECTION: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(2i32);
pub const DWRITE_FONT_FACE_TYPE_RAW_CFF: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(7i32);
pub const DWRITE_FONT_FACE_TYPE_TRUETYPE: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(1i32);
pub const DWRITE_FONT_FACE_TYPE_TRUETYPE_COLLECTION: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(2i32);
pub const DWRITE_FONT_FACE_TYPE_TYPE1: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(3i32);
pub const DWRITE_FONT_FACE_TYPE_UNKNOWN: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(6i32);
pub const DWRITE_FONT_FACE_TYPE_VECTOR: DWRITE_FONT_FACE_TYPE = DWRITE_FONT_FACE_TYPE(4i32);
pub const DWRITE_FONT_FAMILY_MODEL_TYPOGRAPHIC: DWRITE_FONT_FAMILY_MODEL = DWRITE_FONT_FAMILY_MODEL(0i32);
pub const DWRITE_FONT_FAMILY_MODEL_WEIGHT_STRETCH_STYLE: DWRITE_FONT_FAMILY_MODEL = DWRITE_FONT_FAMILY_MODEL(1i32);
pub const DWRITE_FONT_FEATURE_TAG_ALTERNATE_ANNOTATION_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953259886u32);
pub const DWRITE_FONT_FEATURE_TAG_ALTERNATE_HALF_WIDTH: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953259880u32);
pub const DWRITE_FONT_FEATURE_TAG_ALTERNATIVE_FRACTIONS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1668441697u32);
pub const DWRITE_FONT_FEATURE_TAG_CAPITAL_SPACING: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1886613603u32);
pub const DWRITE_FONT_FEATURE_TAG_CASE_SENSITIVE_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1702060387u32);
pub const DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_ALTERNATES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953259875u32);
pub const DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_LIGATURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1734962275u32);
pub const DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_SWASH: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1752658787u32);
pub const DWRITE_FONT_FEATURE_TAG_CURSIVE_POSITIONING: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1936880995u32);
pub const DWRITE_FONT_FEATURE_TAG_DEFAULT: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953261156u32);
pub const DWRITE_FONT_FEATURE_TAG_DISCRETIONARY_LIGATURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1734962276u32);
pub const DWRITE_FONT_FEATURE_TAG_EXPERT_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953527909u32);
pub const DWRITE_FONT_FEATURE_TAG_FRACTIONS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1667330662u32);
pub const DWRITE_FONT_FEATURE_TAG_FULL_WIDTH: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1684633446u32);
pub const DWRITE_FONT_FEATURE_TAG_GLYPH_COMPOSITION_DECOMPOSITION: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1886217059u32);
pub const DWRITE_FONT_FEATURE_TAG_HALANT_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1852596584u32);
pub const DWRITE_FONT_FEATURE_TAG_HALF_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1718378856u32);
pub const DWRITE_FONT_FEATURE_TAG_HALF_WIDTH: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1684633448u32);
pub const DWRITE_FONT_FEATURE_TAG_HISTORICAL_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953720680u32);
pub const DWRITE_FONT_FEATURE_TAG_HISTORICAL_LIGATURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1734962280u32);
pub const DWRITE_FONT_FEATURE_TAG_HOJO_KANJI_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1869246312u32);
pub const DWRITE_FONT_FEATURE_TAG_HORIZONTAL_KANA_ALTERNATES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1634626408u32);
pub const DWRITE_FONT_FEATURE_TAG_JIS04_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(875589738u32);
pub const DWRITE_FONT_FEATURE_TAG_JIS78_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(943157354u32);
pub const DWRITE_FONT_FEATURE_TAG_JIS83_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(859336810u32);
pub const DWRITE_FONT_FEATURE_TAG_JIS90_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(809070698u32);
pub const DWRITE_FONT_FEATURE_TAG_KERNING: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1852990827u32);
pub const DWRITE_FONT_FEATURE_TAG_LINING_FIGURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1836412524u32);
pub const DWRITE_FONT_FEATURE_TAG_LOCALIZED_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1818455916u32);
pub const DWRITE_FONT_FEATURE_TAG_MARK_POSITIONING: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1802658157u32);
pub const DWRITE_FONT_FEATURE_TAG_MARK_TO_MARK_POSITIONING: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1802333037u32);
pub const DWRITE_FONT_FEATURE_TAG_MATHEMATICAL_GREEK: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1802659693u32);
pub const DWRITE_FONT_FEATURE_TAG_NLC_KANJI_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1801677934u32);
pub const DWRITE_FONT_FEATURE_TAG_OLD_STYLE_FIGURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1836412527u32);
pub const DWRITE_FONT_FEATURE_TAG_ORDINALS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1852076655u32);
pub const DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1885430640u32);
pub const DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS_FROM_CAPITALS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1668297315u32);
pub const DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_ALTERNATE_WIDTH: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953259888u32);
pub const DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_FIGURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1836412528u32);
pub const DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_WIDTHS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1684633456u32);
pub const DWRITE_FONT_FEATURE_TAG_QUARTER_WIDTHS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1684633457u32);
pub const DWRITE_FONT_FEATURE_TAG_REQUIRED_LIGATURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1734962290u32);
pub const DWRITE_FONT_FEATURE_TAG_RUBY_NOTATION_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(2036495730u32);
pub const DWRITE_FONT_FEATURE_TAG_SCIENTIFIC_INFERIORS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1718511987u32);
pub const DWRITE_FONT_FEATURE_TAG_SIMPLIFIED_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1819307379u32);
pub const DWRITE_FONT_FEATURE_TAG_SLASHED_ZERO: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1869768058u32);
pub const DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1885564275u32);
pub const DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS_FROM_CAPITALS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1668493923u32);
pub const DWRITE_FONT_FEATURE_TAG_STANDARD_LIGATURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1634167148u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_ALTERNATES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953259891u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_1: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(825258867u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_10: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(808547187u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_11: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(825324403u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_12: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(842101619u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_13: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(858878835u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_14: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(875656051u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_15: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(892433267u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_16: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(909210483u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_17: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(925987699u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_18: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(942764915u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_19: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(959542131u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_2: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(842036083u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_20: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(808612723u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_3: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(858813299u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_4: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(875590515u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_5: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(892367731u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_6: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(909144947u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_7: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(925922163u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_8: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(942699379u32);
pub const DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_9: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(959476595u32);
pub const DWRITE_FONT_FEATURE_TAG_SUBSCRIPT: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1935832435u32);
pub const DWRITE_FONT_FEATURE_TAG_SUPERSCRIPT: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1936749939u32);
pub const DWRITE_FONT_FEATURE_TAG_SWASH: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1752397683u32);
pub const DWRITE_FONT_FEATURE_TAG_TABULAR_FIGURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1836412532u32);
pub const DWRITE_FONT_FEATURE_TAG_THIRD_WIDTHS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1684633460u32);
pub const DWRITE_FONT_FEATURE_TAG_TITLING: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1819568500u32);
pub const DWRITE_FONT_FEATURE_TAG_TRADITIONAL_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1684107892u32);
pub const DWRITE_FONT_FEATURE_TAG_TRADITIONAL_NAME_FORMS: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1835101812u32);
pub const DWRITE_FONT_FEATURE_TAG_UNICASE: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1667853941u32);
pub const DWRITE_FONT_FEATURE_TAG_VERTICAL_ALTERNATES_AND_ROTATION: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(846492278u32);
pub const DWRITE_FONT_FEATURE_TAG_VERTICAL_WRITING: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1953654134u32);
pub const DWRITE_FONT_FILE_TYPE_BITMAP: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(7i32);
pub const DWRITE_FONT_FILE_TYPE_CFF: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(1i32);
pub const DWRITE_FONT_FILE_TYPE_OPENTYPE_COLLECTION: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(3i32);
pub const DWRITE_FONT_FILE_TYPE_TRUETYPE: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(2i32);
pub const DWRITE_FONT_FILE_TYPE_TRUETYPE_COLLECTION: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(3i32);
pub const DWRITE_FONT_FILE_TYPE_TYPE1_PFB: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(5i32);
pub const DWRITE_FONT_FILE_TYPE_TYPE1_PFM: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(4i32);
pub const DWRITE_FONT_FILE_TYPE_UNKNOWN: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(0i32);
pub const DWRITE_FONT_FILE_TYPE_VECTOR: DWRITE_FONT_FILE_TYPE = DWRITE_FONT_FILE_TYPE(6i32);
pub const DWRITE_FONT_LINE_GAP_USAGE_DEFAULT: DWRITE_FONT_LINE_GAP_USAGE = DWRITE_FONT_LINE_GAP_USAGE(0i32);
pub const DWRITE_FONT_LINE_GAP_USAGE_DISABLED: DWRITE_FONT_LINE_GAP_USAGE = DWRITE_FONT_LINE_GAP_USAGE(1i32);
pub const DWRITE_FONT_LINE_GAP_USAGE_ENABLED: DWRITE_FONT_LINE_GAP_USAGE = DWRITE_FONT_LINE_GAP_USAGE(2i32);
pub const DWRITE_FONT_PROPERTY_ID_DESIGN_SCRIPT_LANGUAGE_TAG: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(7i32);
pub const DWRITE_FONT_PROPERTY_ID_FACE_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(3i32);
pub const DWRITE_FONT_PROPERTY_ID_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(1i32);
pub const DWRITE_FONT_PROPERTY_ID_FULL_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(4i32);
pub const DWRITE_FONT_PROPERTY_ID_NONE: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(0i32);
pub const DWRITE_FONT_PROPERTY_ID_POSTSCRIPT_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(6i32);
pub const DWRITE_FONT_PROPERTY_ID_PREFERRED_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(2i32);
pub const DWRITE_FONT_PROPERTY_ID_SEMANTIC_TAG: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(9i32);
pub const DWRITE_FONT_PROPERTY_ID_STRETCH: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(11i32);
pub const DWRITE_FONT_PROPERTY_ID_STYLE: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(12i32);
pub const DWRITE_FONT_PROPERTY_ID_SUPPORTED_SCRIPT_LANGUAGE_TAG: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(8i32);
pub const DWRITE_FONT_PROPERTY_ID_TOTAL: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(13i32);
pub const DWRITE_FONT_PROPERTY_ID_TOTAL_RS3: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(14i32);
pub const DWRITE_FONT_PROPERTY_ID_TYPOGRAPHIC_FACE_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(13i32);
pub const DWRITE_FONT_PROPERTY_ID_TYPOGRAPHIC_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(2i32);
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(10i32);
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT_STRETCH_STYLE_FACE_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(3i32);
pub const DWRITE_FONT_PROPERTY_ID_WEIGHT_STRETCH_STYLE_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(1i32);
pub const DWRITE_FONT_PROPERTY_ID_WIN32_FAMILY_NAME: DWRITE_FONT_PROPERTY_ID = DWRITE_FONT_PROPERTY_ID(5i32);
pub const DWRITE_FONT_SIMULATIONS_BOLD: DWRITE_FONT_SIMULATIONS = DWRITE_FONT_SIMULATIONS(1i32);
pub const DWRITE_FONT_SIMULATIONS_NONE: DWRITE_FONT_SIMULATIONS = DWRITE_FONT_SIMULATIONS(0i32);
pub const DWRITE_FONT_SIMULATIONS_OBLIQUE: DWRITE_FONT_SIMULATIONS = DWRITE_FONT_SIMULATIONS(2i32);
pub const DWRITE_FONT_SOURCE_TYPE_APPX_PACKAGE: DWRITE_FONT_SOURCE_TYPE = DWRITE_FONT_SOURCE_TYPE(3i32);
pub const DWRITE_FONT_SOURCE_TYPE_PER_MACHINE: DWRITE_FONT_SOURCE_TYPE = DWRITE_FONT_SOURCE_TYPE(1i32);
pub const DWRITE_FONT_SOURCE_TYPE_PER_USER: DWRITE_FONT_SOURCE_TYPE = DWRITE_FONT_SOURCE_TYPE(2i32);
pub const DWRITE_FONT_SOURCE_TYPE_REMOTE_FONT_PROVIDER: DWRITE_FONT_SOURCE_TYPE = DWRITE_FONT_SOURCE_TYPE(4i32);
pub const DWRITE_FONT_SOURCE_TYPE_UNKNOWN: DWRITE_FONT_SOURCE_TYPE = DWRITE_FONT_SOURCE_TYPE(0i32);
pub const DWRITE_FONT_STRETCH_CONDENSED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(3i32);
pub const DWRITE_FONT_STRETCH_EXPANDED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(7i32);
pub const DWRITE_FONT_STRETCH_EXTRA_CONDENSED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(2i32);
pub const DWRITE_FONT_STRETCH_EXTRA_EXPANDED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(8i32);
pub const DWRITE_FONT_STRETCH_MEDIUM: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(5i32);
pub const DWRITE_FONT_STRETCH_NORMAL: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(5i32);
pub const DWRITE_FONT_STRETCH_SEMI_CONDENSED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(4i32);
pub const DWRITE_FONT_STRETCH_SEMI_EXPANDED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(6i32);
pub const DWRITE_FONT_STRETCH_ULTRA_CONDENSED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(1i32);
pub const DWRITE_FONT_STRETCH_ULTRA_EXPANDED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(9i32);
pub const DWRITE_FONT_STRETCH_UNDEFINED: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(0i32);
pub const DWRITE_FONT_STYLE_ITALIC: DWRITE_FONT_STYLE = DWRITE_FONT_STYLE(2i32);
pub const DWRITE_FONT_STYLE_NORMAL: DWRITE_FONT_STYLE = DWRITE_FONT_STYLE(0i32);
pub const DWRITE_FONT_STYLE_OBLIQUE: DWRITE_FONT_STYLE = DWRITE_FONT_STYLE(1i32);
pub const DWRITE_FONT_WEIGHT_BLACK: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(900i32);
pub const DWRITE_FONT_WEIGHT_BOLD: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(700i32);
pub const DWRITE_FONT_WEIGHT_DEMI_BOLD: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(600i32);
pub const DWRITE_FONT_WEIGHT_EXTRA_BLACK: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(950i32);
pub const DWRITE_FONT_WEIGHT_EXTRA_BOLD: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(800i32);
pub const DWRITE_FONT_WEIGHT_EXTRA_LIGHT: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(200i32);
pub const DWRITE_FONT_WEIGHT_HEAVY: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(900i32);
pub const DWRITE_FONT_WEIGHT_LIGHT: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(300i32);
pub const DWRITE_FONT_WEIGHT_MEDIUM: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(500i32);
pub const DWRITE_FONT_WEIGHT_NORMAL: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(400i32);
pub const DWRITE_FONT_WEIGHT_REGULAR: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(400i32);
pub const DWRITE_FONT_WEIGHT_SEMI_BOLD: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(600i32);
pub const DWRITE_FONT_WEIGHT_SEMI_LIGHT: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(350i32);
pub const DWRITE_FONT_WEIGHT_THIN: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(100i32);
pub const DWRITE_FONT_WEIGHT_ULTRA_BLACK: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(950i32);
pub const DWRITE_FONT_WEIGHT_ULTRA_BOLD: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(800i32);
pub const DWRITE_FONT_WEIGHT_ULTRA_LIGHT: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(200i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_CFF: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(2i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_COLR: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(4i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_JPEG: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(32i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_NONE: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(0i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_PNG: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(16i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_PREMULTIPLIED_B8G8R8A8: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(128i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_SVG: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(8i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_TIFF: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(64i32);
pub const DWRITE_GLYPH_IMAGE_FORMATS_TRUETYPE: DWRITE_GLYPH_IMAGE_FORMATS = DWRITE_GLYPH_IMAGE_FORMATS(1i32);
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = DWRITE_GLYPH_ORIENTATION_ANGLE(0i32);
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_180_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = DWRITE_GLYPH_ORIENTATION_ANGLE(2i32);
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_270_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = DWRITE_GLYPH_ORIENTATION_ANGLE(3i32);
pub const DWRITE_GLYPH_ORIENTATION_ANGLE_90_DEGREES: DWRITE_GLYPH_ORIENTATION_ANGLE = DWRITE_GLYPH_ORIENTATION_ANGLE(1i32);
pub const DWRITE_GRID_FIT_MODE_DEFAULT: DWRITE_GRID_FIT_MODE = DWRITE_GRID_FIT_MODE(0i32);
pub const DWRITE_GRID_FIT_MODE_DISABLED: DWRITE_GRID_FIT_MODE = DWRITE_GRID_FIT_MODE(1i32);
pub const DWRITE_GRID_FIT_MODE_ENABLED: DWRITE_GRID_FIT_MODE = DWRITE_GRID_FIT_MODE(2i32);
pub const DWRITE_INFORMATIONAL_STRING_COPYRIGHT_NOTICE: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(1i32);
pub const DWRITE_INFORMATIONAL_STRING_DESCRIPTION: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(7i32);
pub const DWRITE_INFORMATIONAL_STRING_DESIGNER: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(5i32);
pub const DWRITE_INFORMATIONAL_STRING_DESIGNER_URL: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(6i32);
pub const DWRITE_INFORMATIONAL_STRING_DESIGN_SCRIPT_LANGUAGE_TAG: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(20i32);
pub const DWRITE_INFORMATIONAL_STRING_FONT_VENDOR_URL: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(8i32);
pub const DWRITE_INFORMATIONAL_STRING_FULL_NAME: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(16i32);
pub const DWRITE_INFORMATIONAL_STRING_LICENSE_DESCRIPTION: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(9i32);
pub const DWRITE_INFORMATIONAL_STRING_LICENSE_INFO_URL: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(10i32);
pub const DWRITE_INFORMATIONAL_STRING_MANUFACTURER: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(4i32);
pub const DWRITE_INFORMATIONAL_STRING_NONE: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(0i32);
pub const DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_CID_NAME: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(18i32);
pub const DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(17i32);
pub const DWRITE_INFORMATIONAL_STRING_PREFERRED_FAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(13i32);
pub const DWRITE_INFORMATIONAL_STRING_PREFERRED_SUBFAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(14i32);
pub const DWRITE_INFORMATIONAL_STRING_SAMPLE_TEXT: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(15i32);
pub const DWRITE_INFORMATIONAL_STRING_SUPPORTED_SCRIPT_LANGUAGE_TAG: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(21i32);
pub const DWRITE_INFORMATIONAL_STRING_TRADEMARK: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(3i32);
pub const DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_FAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(13i32);
pub const DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_SUBFAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(14i32);
pub const DWRITE_INFORMATIONAL_STRING_VERSION_STRINGS: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(2i32);
pub const DWRITE_INFORMATIONAL_STRING_WEIGHT_STRETCH_STYLE_FAMILY_NAME: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(19i32);
pub const DWRITE_INFORMATIONAL_STRING_WIN32_FAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(11i32);
pub const DWRITE_INFORMATIONAL_STRING_WIN32_SUBFAMILY_NAMES: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(12i32);
pub const DWRITE_INFORMATIONAL_STRING_WWS_FAMILY_NAME: DWRITE_INFORMATIONAL_STRING_ID = DWRITE_INFORMATIONAL_STRING_ID(19i32);
pub const DWRITE_LINE_SPACING_METHOD_DEFAULT: DWRITE_LINE_SPACING_METHOD = DWRITE_LINE_SPACING_METHOD(0i32);
pub const DWRITE_LINE_SPACING_METHOD_PROPORTIONAL: DWRITE_LINE_SPACING_METHOD = DWRITE_LINE_SPACING_METHOD(2i32);
pub const DWRITE_LINE_SPACING_METHOD_UNIFORM: DWRITE_LINE_SPACING_METHOD = DWRITE_LINE_SPACING_METHOD(1i32);
pub const DWRITE_LOCALITY_LOCAL: DWRITE_LOCALITY = DWRITE_LOCALITY(2i32);
pub const DWRITE_LOCALITY_PARTIAL: DWRITE_LOCALITY = DWRITE_LOCALITY(1i32);
pub const DWRITE_LOCALITY_REMOTE: DWRITE_LOCALITY = DWRITE_LOCALITY(0i32);
pub const DWRITE_MEASURING_MODE_GDI_CLASSIC: DWRITE_MEASURING_MODE = DWRITE_MEASURING_MODE(1i32);
pub const DWRITE_MEASURING_MODE_GDI_NATURAL: DWRITE_MEASURING_MODE = DWRITE_MEASURING_MODE(2i32);
pub const DWRITE_MEASURING_MODE_NATURAL: DWRITE_MEASURING_MODE = DWRITE_MEASURING_MODE(0i32);
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_CONTEXTUAL: DWRITE_NUMBER_SUBSTITUTION_METHOD = DWRITE_NUMBER_SUBSTITUTION_METHOD(1i32);
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_FROM_CULTURE: DWRITE_NUMBER_SUBSTITUTION_METHOD = DWRITE_NUMBER_SUBSTITUTION_METHOD(0i32);
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_NATIONAL: DWRITE_NUMBER_SUBSTITUTION_METHOD = DWRITE_NUMBER_SUBSTITUTION_METHOD(3i32);
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_NONE: DWRITE_NUMBER_SUBSTITUTION_METHOD = DWRITE_NUMBER_SUBSTITUTION_METHOD(2i32);
pub const DWRITE_NUMBER_SUBSTITUTION_METHOD_TRADITIONAL: DWRITE_NUMBER_SUBSTITUTION_METHOD = DWRITE_NUMBER_SUBSTITUTION_METHOD(4i32);
pub const DWRITE_OPTICAL_ALIGNMENT_NONE: DWRITE_OPTICAL_ALIGNMENT = DWRITE_OPTICAL_ALIGNMENT(0i32);
pub const DWRITE_OPTICAL_ALIGNMENT_NO_SIDE_BEARINGS: DWRITE_OPTICAL_ALIGNMENT = DWRITE_OPTICAL_ALIGNMENT(1i32);
pub const DWRITE_OUTLINE_THRESHOLD_ALIASED: DWRITE_OUTLINE_THRESHOLD = DWRITE_OUTLINE_THRESHOLD(1i32);
pub const DWRITE_OUTLINE_THRESHOLD_ANTIALIASED: DWRITE_OUTLINE_THRESHOLD = DWRITE_OUTLINE_THRESHOLD(0i32);
pub const DWRITE_PANOSE_ARM_STYLE_ANY: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(0i32);
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_DOUBLE_SERIF: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(11i32);
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_HORZ: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(7i32);
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_SINGLE_SERIF: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(10i32);
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_VERT: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(9i32);
pub const DWRITE_PANOSE_ARM_STYLE_BENT_ARMS_WEDGE: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(8i32);
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_DOUBLE_SERIF: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(11i32);
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_HORIZONTAL: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(7i32);
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_SINGLE_SERIF: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(10i32);
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_VERTICAL: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(9i32);
pub const DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_WEDGE: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(8i32);
pub const DWRITE_PANOSE_ARM_STYLE_NO_FIT: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(1i32);
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_DOUBLE_SERIF: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(6i32);
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_HORIZONTAL: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(2i32);
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_HORZ: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(2i32);
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_SINGLE_SERIF: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(5i32);
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_VERT: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(4i32);
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_VERTICAL: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(4i32);
pub const DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_WEDGE: DWRITE_PANOSE_ARM_STYLE = DWRITE_PANOSE_ARM_STYLE(3i32);
pub const DWRITE_PANOSE_ASPECT_ANY: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(0i32);
pub const DWRITE_PANOSE_ASPECT_CONDENSED: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(4i32);
pub const DWRITE_PANOSE_ASPECT_EXTENDED: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(6i32);
pub const DWRITE_PANOSE_ASPECT_MONOSPACED: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(9i32);
pub const DWRITE_PANOSE_ASPECT_NORMAL: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(5i32);
pub const DWRITE_PANOSE_ASPECT_NO_FIT: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(1i32);
pub const DWRITE_PANOSE_ASPECT_RATIO_ANY: DWRITE_PANOSE_ASPECT_RATIO = DWRITE_PANOSE_ASPECT_RATIO(0i32);
pub const DWRITE_PANOSE_ASPECT_RATIO_CONDENSED: DWRITE_PANOSE_ASPECT_RATIO = DWRITE_PANOSE_ASPECT_RATIO(3i32);
pub const DWRITE_PANOSE_ASPECT_RATIO_EXPANDED: DWRITE_PANOSE_ASPECT_RATIO = DWRITE_PANOSE_ASPECT_RATIO(5i32);
pub const DWRITE_PANOSE_ASPECT_RATIO_NORMAL: DWRITE_PANOSE_ASPECT_RATIO = DWRITE_PANOSE_ASPECT_RATIO(4i32);
pub const DWRITE_PANOSE_ASPECT_RATIO_NO_FIT: DWRITE_PANOSE_ASPECT_RATIO = DWRITE_PANOSE_ASPECT_RATIO(1i32);
pub const DWRITE_PANOSE_ASPECT_RATIO_VERY_CONDENSED: DWRITE_PANOSE_ASPECT_RATIO = DWRITE_PANOSE_ASPECT_RATIO(2i32);
pub const DWRITE_PANOSE_ASPECT_RATIO_VERY_EXPANDED: DWRITE_PANOSE_ASPECT_RATIO = DWRITE_PANOSE_ASPECT_RATIO(6i32);
pub const DWRITE_PANOSE_ASPECT_SUPER_CONDENSED: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(2i32);
pub const DWRITE_PANOSE_ASPECT_SUPER_EXTENDED: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(8i32);
pub const DWRITE_PANOSE_ASPECT_VERY_CONDENSED: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(3i32);
pub const DWRITE_PANOSE_ASPECT_VERY_EXTENDED: DWRITE_PANOSE_ASPECT = DWRITE_PANOSE_ASPECT(7i32);
pub const DWRITE_PANOSE_CHARACTER_RANGES_ANY: DWRITE_PANOSE_CHARACTER_RANGES = DWRITE_PANOSE_CHARACTER_RANGES(0i32);
pub const DWRITE_PANOSE_CHARACTER_RANGES_EXTENDED_COLLECTION: DWRITE_PANOSE_CHARACTER_RANGES = DWRITE_PANOSE_CHARACTER_RANGES(2i32);
pub const DWRITE_PANOSE_CHARACTER_RANGES_LITERALS: DWRITE_PANOSE_CHARACTER_RANGES = DWRITE_PANOSE_CHARACTER_RANGES(3i32);
pub const DWRITE_PANOSE_CHARACTER_RANGES_NO_FIT: DWRITE_PANOSE_CHARACTER_RANGES = DWRITE_PANOSE_CHARACTER_RANGES(1i32);
pub const DWRITE_PANOSE_CHARACTER_RANGES_NO_LOWER_CASE: DWRITE_PANOSE_CHARACTER_RANGES = DWRITE_PANOSE_CHARACTER_RANGES(4i32);
pub const DWRITE_PANOSE_CHARACTER_RANGES_SMALL_CAPS: DWRITE_PANOSE_CHARACTER_RANGES = DWRITE_PANOSE_CHARACTER_RANGES(5i32);
pub const DWRITE_PANOSE_CONTRAST_ANY: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(0i32);
pub const DWRITE_PANOSE_CONTRAST_BROKEN: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(13i32);
pub const DWRITE_PANOSE_CONTRAST_HIGH: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(8i32);
pub const DWRITE_PANOSE_CONTRAST_HORIZONTAL_HIGH: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(12i32);
pub const DWRITE_PANOSE_CONTRAST_HORIZONTAL_LOW: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(10i32);
pub const DWRITE_PANOSE_CONTRAST_HORIZONTAL_MEDIUM: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(11i32);
pub const DWRITE_PANOSE_CONTRAST_LOW: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(4i32);
pub const DWRITE_PANOSE_CONTRAST_MEDIUM: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(6i32);
pub const DWRITE_PANOSE_CONTRAST_MEDIUM_HIGH: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(7i32);
pub const DWRITE_PANOSE_CONTRAST_MEDIUM_LOW: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(5i32);
pub const DWRITE_PANOSE_CONTRAST_NONE: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(2i32);
pub const DWRITE_PANOSE_CONTRAST_NO_FIT: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(1i32);
pub const DWRITE_PANOSE_CONTRAST_VERY_HIGH: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(9i32);
pub const DWRITE_PANOSE_CONTRAST_VERY_LOW: DWRITE_PANOSE_CONTRAST = DWRITE_PANOSE_CONTRAST(3i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_ANY: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(0i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_CARTOON: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(7i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_COLLAGE: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(11i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_DERIVATIVE: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(2i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_INITIALS: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(6i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_MONTAGE: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(12i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_ASPECT: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(5i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_ELEMENTS: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(4i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_TOPOLOGY: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(3i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_NO_FIT: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(1i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_ORNAMENTED: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(9i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_PICTURE_STEMS: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(8i32);
pub const DWRITE_PANOSE_DECORATIVE_CLASS_TEXT_AND_BACKGROUND: DWRITE_PANOSE_DECORATIVE_CLASS = DWRITE_PANOSE_DECORATIVE_CLASS(10i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_ANY: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(0i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_ART_DECO: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(5i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_BLACKLETTER: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(14i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_CURSIVE: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(13i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_DIVERSE_ARMS: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(7i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_DIVERSE_FORMS: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(8i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_HORSESHOE_E_AND_A: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(12i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_IMPLIED_TOPOLOGY: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(11i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_LOMBARDIC_FORMS: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(9i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_MULTIPLE_SEGMENT: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(4i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_NO_FIT: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(1i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_SQUARE: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(3i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_STANDARD: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(2i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_SWASH_VARIANCE: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(15i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_UNEVEN_WEIGHTING: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(6i32);
pub const DWRITE_PANOSE_DECORATIVE_TOPOLOGY_UPPER_CASE_IN_LOWER_CASE: DWRITE_PANOSE_DECORATIVE_TOPOLOGY = DWRITE_PANOSE_DECORATIVE_TOPOLOGY(10i32);
pub const DWRITE_PANOSE_FAMILY_ANY: DWRITE_PANOSE_FAMILY = DWRITE_PANOSE_FAMILY(0i32);
pub const DWRITE_PANOSE_FAMILY_DECORATIVE: DWRITE_PANOSE_FAMILY = DWRITE_PANOSE_FAMILY(4i32);
pub const DWRITE_PANOSE_FAMILY_NO_FIT: DWRITE_PANOSE_FAMILY = DWRITE_PANOSE_FAMILY(1i32);
pub const DWRITE_PANOSE_FAMILY_PICTORIAL: DWRITE_PANOSE_FAMILY = DWRITE_PANOSE_FAMILY(5i32);
pub const DWRITE_PANOSE_FAMILY_SCRIPT: DWRITE_PANOSE_FAMILY = DWRITE_PANOSE_FAMILY(3i32);
pub const DWRITE_PANOSE_FAMILY_SYMBOL: DWRITE_PANOSE_FAMILY = DWRITE_PANOSE_FAMILY(5i32);
pub const DWRITE_PANOSE_FAMILY_TEXT_DISPLAY: DWRITE_PANOSE_FAMILY = DWRITE_PANOSE_FAMILY(2i32);
pub const DWRITE_PANOSE_FILL_ANY: DWRITE_PANOSE_FILL = DWRITE_PANOSE_FILL(0i32);
pub const DWRITE_PANOSE_FILL_COMPLEX_FILL: DWRITE_PANOSE_FILL = DWRITE_PANOSE_FILL(5i32);
pub const DWRITE_PANOSE_FILL_DRAWN_DISTRESSED: DWRITE_PANOSE_FILL = DWRITE_PANOSE_FILL(7i32);
pub const DWRITE_PANOSE_FILL_NO_FILL: DWRITE_PANOSE_FILL = DWRITE_PANOSE_FILL(3i32);
pub const DWRITE_PANOSE_FILL_NO_FIT: DWRITE_PANOSE_FILL = DWRITE_PANOSE_FILL(1i32);
pub const DWRITE_PANOSE_FILL_PATTERNED_FILL: DWRITE_PANOSE_FILL = DWRITE_PANOSE_FILL(4i32);
pub const DWRITE_PANOSE_FILL_SHAPED_FILL: DWRITE_PANOSE_FILL = DWRITE_PANOSE_FILL(6i32);
pub const DWRITE_PANOSE_FILL_STANDARD_SOLID_FILL: DWRITE_PANOSE_FILL = DWRITE_PANOSE_FILL(2i32);
pub const DWRITE_PANOSE_FINIALS_ANY: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(0i32);
pub const DWRITE_PANOSE_FINIALS_NONE_CLOSED_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(3i32);
pub const DWRITE_PANOSE_FINIALS_NONE_NO_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(2i32);
pub const DWRITE_PANOSE_FINIALS_NONE_OPEN_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(4i32);
pub const DWRITE_PANOSE_FINIALS_NO_FIT: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(1i32);
pub const DWRITE_PANOSE_FINIALS_ROUND_CLOSED_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(12i32);
pub const DWRITE_PANOSE_FINIALS_ROUND_NO_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(11i32);
pub const DWRITE_PANOSE_FINIALS_ROUND_OPEN_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(13i32);
pub const DWRITE_PANOSE_FINIALS_SHARP_CLOSED_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(6i32);
pub const DWRITE_PANOSE_FINIALS_SHARP_NO_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(5i32);
pub const DWRITE_PANOSE_FINIALS_SHARP_OPEN_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(7i32);
pub const DWRITE_PANOSE_FINIALS_TAPERED_CLOSED_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(9i32);
pub const DWRITE_PANOSE_FINIALS_TAPERED_NO_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(8i32);
pub const DWRITE_PANOSE_FINIALS_TAPERED_OPEN_LOOPS: DWRITE_PANOSE_FINIALS = DWRITE_PANOSE_FINIALS(10i32);
pub const DWRITE_PANOSE_LETTERFORM_ANY: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(0i32);
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_BOXED: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(4i32);
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_CONTACT: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(2i32);
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_FLATTENED: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(5i32);
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_OFF_CENTER: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(7i32);
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_ROUNDED: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(6i32);
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_SQUARE: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(8i32);
pub const DWRITE_PANOSE_LETTERFORM_NORMAL_WEIGHTED: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(3i32);
pub const DWRITE_PANOSE_LETTERFORM_NO_FIT: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(1i32);
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_BOXED: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(11i32);
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_CONTACT: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(9i32);
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_FLATTENED: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(12i32);
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_OFF_CENTER: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(14i32);
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_ROUNDED: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(13i32);
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_SQUARE: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(15i32);
pub const DWRITE_PANOSE_LETTERFORM_OBLIQUE_WEIGHTED: DWRITE_PANOSE_LETTERFORM = DWRITE_PANOSE_LETTERFORM(10i32);
pub const DWRITE_PANOSE_LINING_ANY: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(0i32);
pub const DWRITE_PANOSE_LINING_BACKDROP: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(8i32);
pub const DWRITE_PANOSE_LINING_ENGRAVED: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(5i32);
pub const DWRITE_PANOSE_LINING_INLINE: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(3i32);
pub const DWRITE_PANOSE_LINING_NONE: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(2i32);
pub const DWRITE_PANOSE_LINING_NO_FIT: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(1i32);
pub const DWRITE_PANOSE_LINING_OUTLINE: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(4i32);
pub const DWRITE_PANOSE_LINING_RELIEF: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(7i32);
pub const DWRITE_PANOSE_LINING_SHADOW: DWRITE_PANOSE_LINING = DWRITE_PANOSE_LINING(6i32);
pub const DWRITE_PANOSE_MIDLINE_ANY: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(0i32);
pub const DWRITE_PANOSE_MIDLINE_CONSTANT_POINTED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(9i32);
pub const DWRITE_PANOSE_MIDLINE_CONSTANT_SERIFED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(10i32);
pub const DWRITE_PANOSE_MIDLINE_CONSTANT_TRIMMED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(8i32);
pub const DWRITE_PANOSE_MIDLINE_HIGH_POINTED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(6i32);
pub const DWRITE_PANOSE_MIDLINE_HIGH_SERIFED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(7i32);
pub const DWRITE_PANOSE_MIDLINE_HIGH_TRIMMED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(5i32);
pub const DWRITE_PANOSE_MIDLINE_LOW_POINTED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(12i32);
pub const DWRITE_PANOSE_MIDLINE_LOW_SERIFED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(13i32);
pub const DWRITE_PANOSE_MIDLINE_LOW_TRIMMED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(11i32);
pub const DWRITE_PANOSE_MIDLINE_NO_FIT: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(1i32);
pub const DWRITE_PANOSE_MIDLINE_STANDARD_POINTED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(3i32);
pub const DWRITE_PANOSE_MIDLINE_STANDARD_SERIFED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(4i32);
pub const DWRITE_PANOSE_MIDLINE_STANDARD_TRIMMED: DWRITE_PANOSE_MIDLINE = DWRITE_PANOSE_MIDLINE(2i32);
pub const DWRITE_PANOSE_PROPORTION_ANY: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(0i32);
pub const DWRITE_PANOSE_PROPORTION_CONDENSED: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(6i32);
pub const DWRITE_PANOSE_PROPORTION_EVEN_WIDTH: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(4i32);
pub const DWRITE_PANOSE_PROPORTION_EXPANDED: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(5i32);
pub const DWRITE_PANOSE_PROPORTION_MODERN: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(3i32);
pub const DWRITE_PANOSE_PROPORTION_MONOSPACED: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(9i32);
pub const DWRITE_PANOSE_PROPORTION_NO_FIT: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(1i32);
pub const DWRITE_PANOSE_PROPORTION_OLD_STYLE: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(2i32);
pub const DWRITE_PANOSE_PROPORTION_VERY_CONDENSED: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(8i32);
pub const DWRITE_PANOSE_PROPORTION_VERY_EXPANDED: DWRITE_PANOSE_PROPORTION = DWRITE_PANOSE_PROPORTION(7i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_ANY: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(0i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_EXTREME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(13i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_MORE_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(12i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_NO_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(10i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_SOME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(11i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_NO_FIT: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(1i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_EXTREME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(9i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_MORE_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(8i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_NO_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(6i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_SOME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(7i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_EXTREME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(5i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_MORE_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(4i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_NO_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(2i32);
pub const DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_SOME_WRAPPING: DWRITE_PANOSE_SCRIPT_FORM = DWRITE_PANOSE_SCRIPT_FORM(3i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ANY: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(0i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_CONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(10i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_DISCONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(8i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_TRAILING: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(9i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_CONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(7i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_DISCONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(5i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_TRAILING: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(6i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_NO_FIT: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(1i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_CONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(4i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_DISCONNECTED: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(2i32);
pub const DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_TRAILING: DWRITE_PANOSE_SCRIPT_TOPOLOGY = DWRITE_PANOSE_SCRIPT_TOPOLOGY(3i32);
pub const DWRITE_PANOSE_SERIF_STYLE_ANY: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(0i32);
pub const DWRITE_PANOSE_SERIF_STYLE_BONE: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(8i32);
pub const DWRITE_PANOSE_SERIF_STYLE_COVE: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(2i32);
pub const DWRITE_PANOSE_SERIF_STYLE_EXAGGERATED: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(9i32);
pub const DWRITE_PANOSE_SERIF_STYLE_FLARED: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(14i32);
pub const DWRITE_PANOSE_SERIF_STYLE_NORMAL_SANS: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(11i32);
pub const DWRITE_PANOSE_SERIF_STYLE_NO_FIT: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(1i32);
pub const DWRITE_PANOSE_SERIF_STYLE_OBTUSE_COVE: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(3i32);
pub const DWRITE_PANOSE_SERIF_STYLE_OBTUSE_SANS: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(12i32);
pub const DWRITE_PANOSE_SERIF_STYLE_OBTUSE_SQUARE_COVE: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(5i32);
pub const DWRITE_PANOSE_SERIF_STYLE_OVAL: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(8i32);
pub const DWRITE_PANOSE_SERIF_STYLE_PERPENDICULAR_SANS: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(13i32);
pub const DWRITE_PANOSE_SERIF_STYLE_PERP_SANS: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(13i32);
pub const DWRITE_PANOSE_SERIF_STYLE_ROUNDED: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(15i32);
pub const DWRITE_PANOSE_SERIF_STYLE_SCRIPT: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(16i32);
pub const DWRITE_PANOSE_SERIF_STYLE_SQUARE: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(6i32);
pub const DWRITE_PANOSE_SERIF_STYLE_SQUARE_COVE: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(4i32);
pub const DWRITE_PANOSE_SERIF_STYLE_THIN: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(7i32);
pub const DWRITE_PANOSE_SERIF_STYLE_TRIANGLE: DWRITE_PANOSE_SERIF_STYLE = DWRITE_PANOSE_SERIF_STYLE(10i32);
pub const DWRITE_PANOSE_SPACING_ANY: DWRITE_PANOSE_SPACING = DWRITE_PANOSE_SPACING(0i32);
pub const DWRITE_PANOSE_SPACING_MONOSPACED: DWRITE_PANOSE_SPACING = DWRITE_PANOSE_SPACING(3i32);
pub const DWRITE_PANOSE_SPACING_NO_FIT: DWRITE_PANOSE_SPACING = DWRITE_PANOSE_SPACING(1i32);
pub const DWRITE_PANOSE_SPACING_PROPORTIONAL_SPACED: DWRITE_PANOSE_SPACING = DWRITE_PANOSE_SPACING(2i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_ANY: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(0i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_DIAGONAL: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(3i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_HORIZONTAL: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(6i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_TRANSITIONAL: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(4i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_VERTICAL: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(5i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_INSTANT_HORIZONTAL: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(10i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_INSTANT_VERTICAL: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(9i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_NO_FIT: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(1i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_NO_VARIATION: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(2i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_RAPID_HORIZONTAL: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(8i32);
pub const DWRITE_PANOSE_STROKE_VARIATION_RAPID_VERTICAL: DWRITE_PANOSE_STROKE_VARIATION = DWRITE_PANOSE_STROKE_VARIATION(7i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_ANY: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(0i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_EXCEPTIONALLY_WIDE: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(3i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NARROW: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(8i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NORMAL: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(7i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NO_FIT: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(1i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NO_WIDTH: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(2i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_SUPER_WIDE: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(4i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_VERY_NARROW: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(9i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_VERY_WIDE: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(5i32);
pub const DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_WIDE: DWRITE_PANOSE_SYMBOL_ASPECT_RATIO = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(6i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_ANY: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(0i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_BOARDERS: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(9i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_EXPERT: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(7i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_ICONS: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(10i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_INDUSTRY_SPECIFIC: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(12i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_LOGOS: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(11i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_MONTAGES: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(2i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_MUSIC: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(6i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_NO_FIT: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(1i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_PATTERNS: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(8i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_PICTURES: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(3i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_SCIENTIFIC: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(5i32);
pub const DWRITE_PANOSE_SYMBOL_KIND_SHAPES: DWRITE_PANOSE_SYMBOL_KIND = DWRITE_PANOSE_SYMBOL_KIND(4i32);
pub const DWRITE_PANOSE_TOOL_KIND_ANY: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(0i32);
pub const DWRITE_PANOSE_TOOL_KIND_BALL: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(5i32);
pub const DWRITE_PANOSE_TOOL_KIND_BRUSH: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(6i32);
pub const DWRITE_PANOSE_TOOL_KIND_ENGRAVED: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(4i32);
pub const DWRITE_PANOSE_TOOL_KIND_FELT_PEN_BRUSH_TIP: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(8i32);
pub const DWRITE_PANOSE_TOOL_KIND_FLAT_NIB: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(2i32);
pub const DWRITE_PANOSE_TOOL_KIND_NO_FIT: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(1i32);
pub const DWRITE_PANOSE_TOOL_KIND_PRESSURE_POINT: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(3i32);
pub const DWRITE_PANOSE_TOOL_KIND_ROUGH: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(7i32);
pub const DWRITE_PANOSE_TOOL_KIND_WILD_BRUSH: DWRITE_PANOSE_TOOL_KIND = DWRITE_PANOSE_TOOL_KIND(9i32);
pub const DWRITE_PANOSE_WEIGHT_ANY: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(0i32);
pub const DWRITE_PANOSE_WEIGHT_BLACK: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(10i32);
pub const DWRITE_PANOSE_WEIGHT_BOLD: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(8i32);
pub const DWRITE_PANOSE_WEIGHT_BOOK: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(5i32);
pub const DWRITE_PANOSE_WEIGHT_DEMI: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(7i32);
pub const DWRITE_PANOSE_WEIGHT_EXTRA_BLACK: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(11i32);
pub const DWRITE_PANOSE_WEIGHT_HEAVY: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(9i32);
pub const DWRITE_PANOSE_WEIGHT_LIGHT: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(3i32);
pub const DWRITE_PANOSE_WEIGHT_MEDIUM: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(6i32);
pub const DWRITE_PANOSE_WEIGHT_NORD: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(11i32);
pub const DWRITE_PANOSE_WEIGHT_NO_FIT: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(1i32);
pub const DWRITE_PANOSE_WEIGHT_THIN: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(4i32);
pub const DWRITE_PANOSE_WEIGHT_VERY_LIGHT: DWRITE_PANOSE_WEIGHT = DWRITE_PANOSE_WEIGHT(2i32);
pub const DWRITE_PANOSE_XASCENT_ANY: DWRITE_PANOSE_XASCENT = DWRITE_PANOSE_XASCENT(0i32);
pub const DWRITE_PANOSE_XASCENT_HIGH: DWRITE_PANOSE_XASCENT = DWRITE_PANOSE_XASCENT(5i32);
pub const DWRITE_PANOSE_XASCENT_LOW: DWRITE_PANOSE_XASCENT = DWRITE_PANOSE_XASCENT(3i32);
pub const DWRITE_PANOSE_XASCENT_MEDIUM: DWRITE_PANOSE_XASCENT = DWRITE_PANOSE_XASCENT(4i32);
pub const DWRITE_PANOSE_XASCENT_NO_FIT: DWRITE_PANOSE_XASCENT = DWRITE_PANOSE_XASCENT(1i32);
pub const DWRITE_PANOSE_XASCENT_VERY_HIGH: DWRITE_PANOSE_XASCENT = DWRITE_PANOSE_XASCENT(6i32);
pub const DWRITE_PANOSE_XASCENT_VERY_LOW: DWRITE_PANOSE_XASCENT = DWRITE_PANOSE_XASCENT(2i32);
pub const DWRITE_PANOSE_XHEIGHT_ANY: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(0i32);
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_LARGE: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(4i32);
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_SMALL: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(2i32);
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_STANDARD: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(3i32);
pub const DWRITE_PANOSE_XHEIGHT_CONSTANT_STD: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(3i32);
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_LARGE: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(7i32);
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_SMALL: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(5i32);
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_STANDARD: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(6i32);
pub const DWRITE_PANOSE_XHEIGHT_DUCKING_STD: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(6i32);
pub const DWRITE_PANOSE_XHEIGHT_NO_FIT: DWRITE_PANOSE_XHEIGHT = DWRITE_PANOSE_XHEIGHT(1i32);
pub const DWRITE_PARAGRAPH_ALIGNMENT_CENTER: DWRITE_PARAGRAPH_ALIGNMENT = DWRITE_PARAGRAPH_ALIGNMENT(2i32);
pub const DWRITE_PARAGRAPH_ALIGNMENT_FAR: DWRITE_PARAGRAPH_ALIGNMENT = DWRITE_PARAGRAPH_ALIGNMENT(1i32);
pub const DWRITE_PARAGRAPH_ALIGNMENT_NEAR: DWRITE_PARAGRAPH_ALIGNMENT = DWRITE_PARAGRAPH_ALIGNMENT(0i32);
pub const DWRITE_PIXEL_GEOMETRY_BGR: DWRITE_PIXEL_GEOMETRY = DWRITE_PIXEL_GEOMETRY(2i32);
pub const DWRITE_PIXEL_GEOMETRY_FLAT: DWRITE_PIXEL_GEOMETRY = DWRITE_PIXEL_GEOMETRY(0i32);
pub const DWRITE_PIXEL_GEOMETRY_RGB: DWRITE_PIXEL_GEOMETRY = DWRITE_PIXEL_GEOMETRY(1i32);
pub const DWRITE_READING_DIRECTION_BOTTOM_TO_TOP: DWRITE_READING_DIRECTION = DWRITE_READING_DIRECTION(3i32);
pub const DWRITE_READING_DIRECTION_LEFT_TO_RIGHT: DWRITE_READING_DIRECTION = DWRITE_READING_DIRECTION(0i32);
pub const DWRITE_READING_DIRECTION_RIGHT_TO_LEFT: DWRITE_READING_DIRECTION = DWRITE_READING_DIRECTION(1i32);
pub const DWRITE_READING_DIRECTION_TOP_TO_BOTTOM: DWRITE_READING_DIRECTION = DWRITE_READING_DIRECTION(2i32);
pub const DWRITE_RENDERING_MODE1_ALIASED: DWRITE_RENDERING_MODE1 = DWRITE_RENDERING_MODE1(1i32);
pub const DWRITE_RENDERING_MODE1_DEFAULT: DWRITE_RENDERING_MODE1 = DWRITE_RENDERING_MODE1(0i32);
pub const DWRITE_RENDERING_MODE1_GDI_CLASSIC: DWRITE_RENDERING_MODE1 = DWRITE_RENDERING_MODE1(2i32);
pub const DWRITE_RENDERING_MODE1_GDI_NATURAL: DWRITE_RENDERING_MODE1 = DWRITE_RENDERING_MODE1(3i32);
pub const DWRITE_RENDERING_MODE1_NATURAL: DWRITE_RENDERING_MODE1 = DWRITE_RENDERING_MODE1(4i32);
pub const DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC: DWRITE_RENDERING_MODE1 = DWRITE_RENDERING_MODE1(5i32);
pub const DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC_DOWNSAMPLED: DWRITE_RENDERING_MODE1 = DWRITE_RENDERING_MODE1(7i32);
pub const DWRITE_RENDERING_MODE1_OUTLINE: DWRITE_RENDERING_MODE1 = DWRITE_RENDERING_MODE1(6i32);
pub const DWRITE_RENDERING_MODE_ALIASED: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(1i32);
pub const DWRITE_RENDERING_MODE_CLEARTYPE_GDI_CLASSIC: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(2i32);
pub const DWRITE_RENDERING_MODE_CLEARTYPE_GDI_NATURAL: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(3i32);
pub const DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(4i32);
pub const DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL_SYMMETRIC: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(5i32);
pub const DWRITE_RENDERING_MODE_DEFAULT: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(0i32);
pub const DWRITE_RENDERING_MODE_GDI_CLASSIC: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(2i32);
pub const DWRITE_RENDERING_MODE_GDI_NATURAL: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(3i32);
pub const DWRITE_RENDERING_MODE_NATURAL: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(4i32);
pub const DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(5i32);
pub const DWRITE_RENDERING_MODE_OUTLINE: DWRITE_RENDERING_MODE = DWRITE_RENDERING_MODE(6i32);
pub const DWRITE_SCRIPT_SHAPES_DEFAULT: DWRITE_SCRIPT_SHAPES = DWRITE_SCRIPT_SHAPES(0i32);
pub const DWRITE_SCRIPT_SHAPES_NO_VISUAL: DWRITE_SCRIPT_SHAPES = DWRITE_SCRIPT_SHAPES(1i32);
pub const DWRITE_STANDARD_FONT_AXIS_COUNT: u32 = 5u32;
pub const DWRITE_TEXTURE_ALIASED_1x1: DWRITE_TEXTURE_TYPE = DWRITE_TEXTURE_TYPE(0i32);
pub const DWRITE_TEXTURE_CLEARTYPE_3x1: DWRITE_TEXTURE_TYPE = DWRITE_TEXTURE_TYPE(1i32);
pub const DWRITE_TEXT_ALIGNMENT_CENTER: DWRITE_TEXT_ALIGNMENT = DWRITE_TEXT_ALIGNMENT(2i32);
pub const DWRITE_TEXT_ALIGNMENT_JUSTIFIED: DWRITE_TEXT_ALIGNMENT = DWRITE_TEXT_ALIGNMENT(3i32);
pub const DWRITE_TEXT_ALIGNMENT_LEADING: DWRITE_TEXT_ALIGNMENT = DWRITE_TEXT_ALIGNMENT(0i32);
pub const DWRITE_TEXT_ALIGNMENT_TRAILING: DWRITE_TEXT_ALIGNMENT = DWRITE_TEXT_ALIGNMENT(1i32);
pub const DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE: DWRITE_TEXT_ANTIALIAS_MODE = DWRITE_TEXT_ANTIALIAS_MODE(0i32);
pub const DWRITE_TEXT_ANTIALIAS_MODE_GRAYSCALE: DWRITE_TEXT_ANTIALIAS_MODE = DWRITE_TEXT_ANTIALIAS_MODE(1i32);
pub const DWRITE_TRIMMING_GRANULARITY_CHARACTER: DWRITE_TRIMMING_GRANULARITY = DWRITE_TRIMMING_GRANULARITY(1i32);
pub const DWRITE_TRIMMING_GRANULARITY_NONE: DWRITE_TRIMMING_GRANULARITY = DWRITE_TRIMMING_GRANULARITY(0i32);
pub const DWRITE_TRIMMING_GRANULARITY_WORD: DWRITE_TRIMMING_GRANULARITY = DWRITE_TRIMMING_GRANULARITY(2i32);
pub const DWRITE_VERTICAL_GLYPH_ORIENTATION_DEFAULT: DWRITE_VERTICAL_GLYPH_ORIENTATION = DWRITE_VERTICAL_GLYPH_ORIENTATION(0i32);
pub const DWRITE_VERTICAL_GLYPH_ORIENTATION_STACKED: DWRITE_VERTICAL_GLYPH_ORIENTATION = DWRITE_VERTICAL_GLYPH_ORIENTATION(1i32);
pub const DWRITE_WORD_WRAPPING_CHARACTER: DWRITE_WORD_WRAPPING = DWRITE_WORD_WRAPPING(4i32);
pub const DWRITE_WORD_WRAPPING_EMERGENCY_BREAK: DWRITE_WORD_WRAPPING = DWRITE_WORD_WRAPPING(2i32);
pub const DWRITE_WORD_WRAPPING_NO_WRAP: DWRITE_WORD_WRAPPING = DWRITE_WORD_WRAPPING(1i32);
pub const DWRITE_WORD_WRAPPING_WHOLE_WORD: DWRITE_WORD_WRAPPING = DWRITE_WORD_WRAPPING(3i32);
pub const DWRITE_WORD_WRAPPING_WRAP: DWRITE_WORD_WRAPPING = DWRITE_WORD_WRAPPING(0i32);
pub const FACILITY_DWRITE: u32 = 2200u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_AUTOMATIC_FONT_AXES(pub i32);
impl ::core::marker::Copy for DWRITE_AUTOMATIC_FONT_AXES {}
impl ::core::clone::Clone for DWRITE_AUTOMATIC_FONT_AXES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_AUTOMATIC_FONT_AXES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_AUTOMATIC_FONT_AXES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_AUTOMATIC_FONT_AXES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_AUTOMATIC_FONT_AXES").field(&self.0).finish()
    }
}
impl DWRITE_AUTOMATIC_FONT_AXES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DWRITE_AUTOMATIC_FONT_AXES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DWRITE_AUTOMATIC_FONT_AXES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DWRITE_AUTOMATIC_FONT_AXES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DWRITE_AUTOMATIC_FONT_AXES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DWRITE_AUTOMATIC_FONT_AXES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_BASELINE(pub i32);
impl ::core::marker::Copy for DWRITE_BASELINE {}
impl ::core::clone::Clone for DWRITE_BASELINE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_BASELINE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_BASELINE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_BASELINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_BASELINE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_BREAK_CONDITION(pub i32);
impl ::core::marker::Copy for DWRITE_BREAK_CONDITION {}
impl ::core::clone::Clone for DWRITE_BREAK_CONDITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_BREAK_CONDITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_BREAK_CONDITION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_BREAK_CONDITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_BREAK_CONDITION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_CONTAINER_TYPE(pub i32);
impl ::core::marker::Copy for DWRITE_CONTAINER_TYPE {}
impl ::core::clone::Clone for DWRITE_CONTAINER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_CONTAINER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_CONTAINER_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_CONTAINER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_CONTAINER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_FACTORY_TYPE(pub i32);
impl ::core::marker::Copy for DWRITE_FACTORY_TYPE {}
impl ::core::clone::Clone for DWRITE_FACTORY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_FACTORY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_FACTORY_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_FACTORY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FACTORY_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_FLOW_DIRECTION(pub i32);
impl ::core::marker::Copy for DWRITE_FLOW_DIRECTION {}
impl ::core::clone::Clone for DWRITE_FLOW_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_FLOW_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_FLOW_DIRECTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_FLOW_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FLOW_DIRECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_FONT_AXIS_ATTRIBUTES(pub i32);
impl ::core::marker::Copy for DWRITE_FONT_AXIS_ATTRIBUTES {}
impl ::core::clone::Clone for DWRITE_FONT_AXIS_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_FONT_AXIS_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_AXIS_ATTRIBUTES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_FONT_AXIS_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_AXIS_ATTRIBUTES").field(&self.0).finish()
    }
}
impl DWRITE_FONT_AXIS_ATTRIBUTES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DWRITE_FONT_AXIS_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DWRITE_FONT_AXIS_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DWRITE_FONT_AXIS_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DWRITE_FONT_AXIS_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DWRITE_FONT_AXIS_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_FONT_AXIS_TAG(pub u32);
impl ::core::marker::Copy for DWRITE_FONT_AXIS_TAG {}
impl ::core::clone::Clone for DWRITE_FONT_AXIS_TAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_FONT_AXIS_TAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_AXIS_TAG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_FONT_AXIS_TAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_AXIS_TAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_FONT_FACE_TYPE(pub i32);
impl ::core::marker::Copy for DWRITE_FONT_FACE_TYPE {}
impl ::core::clone::Clone for DWRITE_FONT_FACE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_FONT_FACE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_FACE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_FONT_FACE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_FACE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_FONT_FAMILY_MODEL(pub i32);
impl ::core::marker::Copy for DWRITE_FONT_FAMILY_MODEL {}
impl ::core::clone::Clone for DWRITE_FONT_FAMILY_MODEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_FONT_FAMILY_MODEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_FAMILY_MODEL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_FONT_FAMILY_MODEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_FAMILY_MODEL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_FONT_FEATURE_TAG(pub u32);
impl ::core::marker::Copy for DWRITE_FONT_FEATURE_TAG {}
impl ::core::clone::Clone for DWRITE_FONT_FEATURE_TAG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_FONT_FEATURE_TAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_FEATURE_TAG {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_FONT_FEATURE_TAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_FEATURE_TAG").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_FONT_FILE_TYPE(pub i32);
impl ::core::marker::Copy for DWRITE_FONT_FILE_TYPE {}
impl ::core::clone::Clone for DWRITE_FONT_FILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_FONT_FILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_FILE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_FONT_FILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_FILE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_FONT_LINE_GAP_USAGE(pub i32);
impl ::core::marker::Copy for DWRITE_FONT_LINE_GAP_USAGE {}
impl ::core::clone::Clone for DWRITE_FONT_LINE_GAP_USAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_FONT_LINE_GAP_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_LINE_GAP_USAGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_FONT_LINE_GAP_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_LINE_GAP_USAGE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_FONT_PROPERTY_ID(pub i32);
impl ::core::marker::Copy for DWRITE_FONT_PROPERTY_ID {}
impl ::core::clone::Clone for DWRITE_FONT_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_FONT_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_PROPERTY_ID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_FONT_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_PROPERTY_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_FONT_SIMULATIONS(pub i32);
impl ::core::marker::Copy for DWRITE_FONT_SIMULATIONS {}
impl ::core::clone::Clone for DWRITE_FONT_SIMULATIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_FONT_SIMULATIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_SIMULATIONS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_FONT_SIMULATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_SIMULATIONS").field(&self.0).finish()
    }
}
impl DWRITE_FONT_SIMULATIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DWRITE_FONT_SIMULATIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DWRITE_FONT_SIMULATIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DWRITE_FONT_SIMULATIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DWRITE_FONT_SIMULATIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DWRITE_FONT_SIMULATIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_FONT_SOURCE_TYPE(pub i32);
impl ::core::marker::Copy for DWRITE_FONT_SOURCE_TYPE {}
impl ::core::clone::Clone for DWRITE_FONT_SOURCE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_FONT_SOURCE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_SOURCE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_FONT_SOURCE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_SOURCE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_FONT_STRETCH(pub i32);
impl ::core::marker::Copy for DWRITE_FONT_STRETCH {}
impl ::core::clone::Clone for DWRITE_FONT_STRETCH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_FONT_STRETCH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_STRETCH {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_FONT_STRETCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_STRETCH").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_FONT_STYLE(pub i32);
impl ::core::marker::Copy for DWRITE_FONT_STYLE {}
impl ::core::clone::Clone for DWRITE_FONT_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_FONT_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_STYLE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_FONT_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_STYLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_FONT_WEIGHT(pub i32);
impl ::core::marker::Copy for DWRITE_FONT_WEIGHT {}
impl ::core::clone::Clone for DWRITE_FONT_WEIGHT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_FONT_WEIGHT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_WEIGHT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_FONT_WEIGHT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_FONT_WEIGHT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_GLYPH_IMAGE_FORMATS(pub i32);
impl ::core::marker::Copy for DWRITE_GLYPH_IMAGE_FORMATS {}
impl ::core::clone::Clone for DWRITE_GLYPH_IMAGE_FORMATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_GLYPH_IMAGE_FORMATS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_GLYPH_IMAGE_FORMATS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_GLYPH_IMAGE_FORMATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_GLYPH_IMAGE_FORMATS").field(&self.0).finish()
    }
}
impl DWRITE_GLYPH_IMAGE_FORMATS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DWRITE_GLYPH_IMAGE_FORMATS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DWRITE_GLYPH_IMAGE_FORMATS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DWRITE_GLYPH_IMAGE_FORMATS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DWRITE_GLYPH_IMAGE_FORMATS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DWRITE_GLYPH_IMAGE_FORMATS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_GLYPH_ORIENTATION_ANGLE(pub i32);
impl ::core::marker::Copy for DWRITE_GLYPH_ORIENTATION_ANGLE {}
impl ::core::clone::Clone for DWRITE_GLYPH_ORIENTATION_ANGLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_GLYPH_ORIENTATION_ANGLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_GLYPH_ORIENTATION_ANGLE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_GLYPH_ORIENTATION_ANGLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_GLYPH_ORIENTATION_ANGLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_GRID_FIT_MODE(pub i32);
impl ::core::marker::Copy for DWRITE_GRID_FIT_MODE {}
impl ::core::clone::Clone for DWRITE_GRID_FIT_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_GRID_FIT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_GRID_FIT_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_GRID_FIT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_GRID_FIT_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_INFORMATIONAL_STRING_ID(pub i32);
impl ::core::marker::Copy for DWRITE_INFORMATIONAL_STRING_ID {}
impl ::core::clone::Clone for DWRITE_INFORMATIONAL_STRING_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_INFORMATIONAL_STRING_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_INFORMATIONAL_STRING_ID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_INFORMATIONAL_STRING_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_INFORMATIONAL_STRING_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_LINE_SPACING_METHOD(pub i32);
impl ::core::marker::Copy for DWRITE_LINE_SPACING_METHOD {}
impl ::core::clone::Clone for DWRITE_LINE_SPACING_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_LINE_SPACING_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_LINE_SPACING_METHOD {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_LINE_SPACING_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_LINE_SPACING_METHOD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_LOCALITY(pub i32);
impl ::core::marker::Copy for DWRITE_LOCALITY {}
impl ::core::clone::Clone for DWRITE_LOCALITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_LOCALITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_LOCALITY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_LOCALITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_LOCALITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_MEASURING_MODE(pub i32);
impl ::core::marker::Copy for DWRITE_MEASURING_MODE {}
impl ::core::clone::Clone for DWRITE_MEASURING_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_MEASURING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_MEASURING_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_MEASURING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_MEASURING_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_NUMBER_SUBSTITUTION_METHOD(pub i32);
impl ::core::marker::Copy for DWRITE_NUMBER_SUBSTITUTION_METHOD {}
impl ::core::clone::Clone for DWRITE_NUMBER_SUBSTITUTION_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_NUMBER_SUBSTITUTION_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_NUMBER_SUBSTITUTION_METHOD {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_NUMBER_SUBSTITUTION_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_NUMBER_SUBSTITUTION_METHOD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_OPTICAL_ALIGNMENT(pub i32);
impl ::core::marker::Copy for DWRITE_OPTICAL_ALIGNMENT {}
impl ::core::clone::Clone for DWRITE_OPTICAL_ALIGNMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_OPTICAL_ALIGNMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_OPTICAL_ALIGNMENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_OPTICAL_ALIGNMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_OPTICAL_ALIGNMENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_OUTLINE_THRESHOLD(pub i32);
impl ::core::marker::Copy for DWRITE_OUTLINE_THRESHOLD {}
impl ::core::clone::Clone for DWRITE_OUTLINE_THRESHOLD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_OUTLINE_THRESHOLD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_OUTLINE_THRESHOLD {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_OUTLINE_THRESHOLD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_OUTLINE_THRESHOLD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_ARM_STYLE(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_ARM_STYLE {}
impl ::core::clone::Clone for DWRITE_PANOSE_ARM_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_ARM_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_ARM_STYLE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_ARM_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_ARM_STYLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_ASPECT(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_ASPECT {}
impl ::core::clone::Clone for DWRITE_PANOSE_ASPECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_ASPECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_ASPECT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_ASPECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_ASPECT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_ASPECT_RATIO(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_ASPECT_RATIO {}
impl ::core::clone::Clone for DWRITE_PANOSE_ASPECT_RATIO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_ASPECT_RATIO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_ASPECT_RATIO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_ASPECT_RATIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_ASPECT_RATIO").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_CHARACTER_RANGES(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_CHARACTER_RANGES {}
impl ::core::clone::Clone for DWRITE_PANOSE_CHARACTER_RANGES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_CHARACTER_RANGES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_CHARACTER_RANGES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_CHARACTER_RANGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_CHARACTER_RANGES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_CONTRAST(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_CONTRAST {}
impl ::core::clone::Clone for DWRITE_PANOSE_CONTRAST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_CONTRAST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_CONTRAST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_CONTRAST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_CONTRAST").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_DECORATIVE_CLASS(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_DECORATIVE_CLASS {}
impl ::core::clone::Clone for DWRITE_PANOSE_DECORATIVE_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_DECORATIVE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_DECORATIVE_CLASS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_DECORATIVE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_DECORATIVE_CLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_DECORATIVE_TOPOLOGY(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_DECORATIVE_TOPOLOGY {}
impl ::core::clone::Clone for DWRITE_PANOSE_DECORATIVE_TOPOLOGY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_DECORATIVE_TOPOLOGY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_DECORATIVE_TOPOLOGY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_DECORATIVE_TOPOLOGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_DECORATIVE_TOPOLOGY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_FAMILY(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_FAMILY {}
impl ::core::clone::Clone for DWRITE_PANOSE_FAMILY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_FAMILY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_FAMILY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_FAMILY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_FAMILY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_FILL(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_FILL {}
impl ::core::clone::Clone for DWRITE_PANOSE_FILL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_FILL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_FILL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_FILL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_FILL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_FINIALS(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_FINIALS {}
impl ::core::clone::Clone for DWRITE_PANOSE_FINIALS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_FINIALS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_FINIALS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_FINIALS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_FINIALS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_LETTERFORM(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_LETTERFORM {}
impl ::core::clone::Clone for DWRITE_PANOSE_LETTERFORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_LETTERFORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_LETTERFORM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_LETTERFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_LETTERFORM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_LINING(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_LINING {}
impl ::core::clone::Clone for DWRITE_PANOSE_LINING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_LINING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_LINING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_LINING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_LINING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_MIDLINE(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_MIDLINE {}
impl ::core::clone::Clone for DWRITE_PANOSE_MIDLINE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_MIDLINE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_MIDLINE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_MIDLINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_MIDLINE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_PROPORTION(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_PROPORTION {}
impl ::core::clone::Clone for DWRITE_PANOSE_PROPORTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_PROPORTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_PROPORTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_PROPORTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_PROPORTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_SCRIPT_FORM(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_SCRIPT_FORM {}
impl ::core::clone::Clone for DWRITE_PANOSE_SCRIPT_FORM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_SCRIPT_FORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_SCRIPT_FORM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_SCRIPT_FORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SCRIPT_FORM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_SCRIPT_TOPOLOGY(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_SCRIPT_TOPOLOGY {}
impl ::core::clone::Clone for DWRITE_PANOSE_SCRIPT_TOPOLOGY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_SCRIPT_TOPOLOGY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_SCRIPT_TOPOLOGY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_SCRIPT_TOPOLOGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SCRIPT_TOPOLOGY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_SERIF_STYLE(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_SERIF_STYLE {}
impl ::core::clone::Clone for DWRITE_PANOSE_SERIF_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_SERIF_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_SERIF_STYLE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_SERIF_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SERIF_STYLE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_SPACING(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_SPACING {}
impl ::core::clone::Clone for DWRITE_PANOSE_SPACING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_SPACING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_SPACING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_SPACING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SPACING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_STROKE_VARIATION(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_STROKE_VARIATION {}
impl ::core::clone::Clone for DWRITE_PANOSE_STROKE_VARIATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_STROKE_VARIATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_STROKE_VARIATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_STROKE_VARIATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_STROKE_VARIATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_SYMBOL_ASPECT_RATIO(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_SYMBOL_ASPECT_RATIO {}
impl ::core::clone::Clone for DWRITE_PANOSE_SYMBOL_ASPECT_RATIO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_SYMBOL_ASPECT_RATIO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_SYMBOL_ASPECT_RATIO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_SYMBOL_ASPECT_RATIO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SYMBOL_ASPECT_RATIO").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_SYMBOL_KIND(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_SYMBOL_KIND {}
impl ::core::clone::Clone for DWRITE_PANOSE_SYMBOL_KIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_SYMBOL_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_SYMBOL_KIND {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_SYMBOL_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_SYMBOL_KIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_TOOL_KIND(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_TOOL_KIND {}
impl ::core::clone::Clone for DWRITE_PANOSE_TOOL_KIND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_TOOL_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_TOOL_KIND {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_TOOL_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_TOOL_KIND").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_WEIGHT(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_WEIGHT {}
impl ::core::clone::Clone for DWRITE_PANOSE_WEIGHT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_WEIGHT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_WEIGHT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_WEIGHT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_WEIGHT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_XASCENT(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_XASCENT {}
impl ::core::clone::Clone for DWRITE_PANOSE_XASCENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_XASCENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_XASCENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_XASCENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_XASCENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PANOSE_XHEIGHT(pub i32);
impl ::core::marker::Copy for DWRITE_PANOSE_XHEIGHT {}
impl ::core::clone::Clone for DWRITE_PANOSE_XHEIGHT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PANOSE_XHEIGHT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_XHEIGHT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PANOSE_XHEIGHT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PANOSE_XHEIGHT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PARAGRAPH_ALIGNMENT(pub i32);
impl ::core::marker::Copy for DWRITE_PARAGRAPH_ALIGNMENT {}
impl ::core::clone::Clone for DWRITE_PARAGRAPH_ALIGNMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PARAGRAPH_ALIGNMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PARAGRAPH_ALIGNMENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PARAGRAPH_ALIGNMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PARAGRAPH_ALIGNMENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_PIXEL_GEOMETRY(pub i32);
impl ::core::marker::Copy for DWRITE_PIXEL_GEOMETRY {}
impl ::core::clone::Clone for DWRITE_PIXEL_GEOMETRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_PIXEL_GEOMETRY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_PIXEL_GEOMETRY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_PIXEL_GEOMETRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_PIXEL_GEOMETRY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_READING_DIRECTION(pub i32);
impl ::core::marker::Copy for DWRITE_READING_DIRECTION {}
impl ::core::clone::Clone for DWRITE_READING_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_READING_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_READING_DIRECTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_READING_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_READING_DIRECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_RENDERING_MODE(pub i32);
impl ::core::marker::Copy for DWRITE_RENDERING_MODE {}
impl ::core::clone::Clone for DWRITE_RENDERING_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_RENDERING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_RENDERING_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_RENDERING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_RENDERING_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_RENDERING_MODE1(pub i32);
impl ::core::marker::Copy for DWRITE_RENDERING_MODE1 {}
impl ::core::clone::Clone for DWRITE_RENDERING_MODE1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_RENDERING_MODE1 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_RENDERING_MODE1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_RENDERING_MODE1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_RENDERING_MODE1").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_SCRIPT_SHAPES(pub i32);
impl ::core::marker::Copy for DWRITE_SCRIPT_SHAPES {}
impl ::core::clone::Clone for DWRITE_SCRIPT_SHAPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_SCRIPT_SHAPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_SCRIPT_SHAPES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_SCRIPT_SHAPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_SCRIPT_SHAPES").field(&self.0).finish()
    }
}
impl DWRITE_SCRIPT_SHAPES {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for DWRITE_SCRIPT_SHAPES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DWRITE_SCRIPT_SHAPES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DWRITE_SCRIPT_SHAPES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DWRITE_SCRIPT_SHAPES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DWRITE_SCRIPT_SHAPES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_TEXTURE_TYPE(pub i32);
impl ::core::marker::Copy for DWRITE_TEXTURE_TYPE {}
impl ::core::clone::Clone for DWRITE_TEXTURE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_TEXTURE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_TEXTURE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_TEXTURE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_TEXTURE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_TEXT_ALIGNMENT(pub i32);
impl ::core::marker::Copy for DWRITE_TEXT_ALIGNMENT {}
impl ::core::clone::Clone for DWRITE_TEXT_ALIGNMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_TEXT_ALIGNMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_TEXT_ALIGNMENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_TEXT_ALIGNMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_TEXT_ALIGNMENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_TEXT_ANTIALIAS_MODE(pub i32);
impl ::core::marker::Copy for DWRITE_TEXT_ANTIALIAS_MODE {}
impl ::core::clone::Clone for DWRITE_TEXT_ANTIALIAS_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_TEXT_ANTIALIAS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_TEXT_ANTIALIAS_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_TEXT_ANTIALIAS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_TEXT_ANTIALIAS_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_TRIMMING_GRANULARITY(pub i32);
impl ::core::marker::Copy for DWRITE_TRIMMING_GRANULARITY {}
impl ::core::clone::Clone for DWRITE_TRIMMING_GRANULARITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_TRIMMING_GRANULARITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_TRIMMING_GRANULARITY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_TRIMMING_GRANULARITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_TRIMMING_GRANULARITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_VERTICAL_GLYPH_ORIENTATION(pub i32);
impl ::core::marker::Copy for DWRITE_VERTICAL_GLYPH_ORIENTATION {}
impl ::core::clone::Clone for DWRITE_VERTICAL_GLYPH_ORIENTATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_VERTICAL_GLYPH_ORIENTATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_VERTICAL_GLYPH_ORIENTATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_VERTICAL_GLYPH_ORIENTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_VERTICAL_GLYPH_ORIENTATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DWRITE_WORD_WRAPPING(pub i32);
impl ::core::marker::Copy for DWRITE_WORD_WRAPPING {}
impl ::core::clone::Clone for DWRITE_WORD_WRAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DWRITE_WORD_WRAPPING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DWRITE_WORD_WRAPPING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DWRITE_WORD_WRAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DWRITE_WORD_WRAPPING").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct DWRITE_CARET_METRICS {
    pub slopeRise: i16,
    pub slopeRun: i16,
    pub offset: i16,
}
impl ::core::marker::Copy for DWRITE_CARET_METRICS {}
impl ::core::clone::Clone for DWRITE_CARET_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_CARET_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_CARET_METRICS").field("slopeRise", &self.slopeRise).field("slopeRun", &self.slopeRun).field("offset", &self.offset).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_CARET_METRICS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_CARET_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.slopeRise == other.slopeRise && self.slopeRun == other.slopeRun && self.offset == other.offset
    }
}
impl ::core::cmp::Eq for DWRITE_CARET_METRICS {}
impl ::core::default::Default for DWRITE_CARET_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_CLUSTER_METRICS {
    pub width: f32,
    pub length: u16,
    pub _bitfield: u16,
}
impl ::core::marker::Copy for DWRITE_CLUSTER_METRICS {}
impl ::core::clone::Clone for DWRITE_CLUSTER_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_CLUSTER_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_CLUSTER_METRICS").field("width", &self.width).field("length", &self.length).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_CLUSTER_METRICS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_CLUSTER_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.length == other.length && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DWRITE_CLUSTER_METRICS {}
impl ::core::default::Default for DWRITE_CLUSTER_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl ::core::marker::Copy for DWRITE_COLOR_F {}
impl ::core::clone::Clone for DWRITE_COLOR_F {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_COLOR_F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_COLOR_F").field("r", &self.r).field("g", &self.g).field("b", &self.b).field("a", &self.a).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_COLOR_F {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_COLOR_F {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl ::core::cmp::Eq for DWRITE_COLOR_F {}
impl ::core::default::Default for DWRITE_COLOR_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_COLOR_GLYPH_RUN {
    pub glyphRun: DWRITE_GLYPH_RUN,
    pub glyphRunDescription: *mut DWRITE_GLYPH_RUN_DESCRIPTION,
    pub baselineOriginX: f32,
    pub baselineOriginY: f32,
    pub runColor: DWRITE_COLOR_F,
    pub paletteIndex: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_COLOR_GLYPH_RUN {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DWRITE_COLOR_GLYPH_RUN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_COLOR_GLYPH_RUN").field("glyphRun", &self.glyphRun).field("glyphRunDescription", &self.glyphRunDescription).field("baselineOriginX", &self.baselineOriginX).field("baselineOriginY", &self.baselineOriginY).field("runColor", &self.runColor).field("paletteIndex", &self.paletteIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DWRITE_COLOR_GLYPH_RUN {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWRITE_COLOR_GLYPH_RUN {
    fn eq(&self, other: &Self) -> bool {
        self.glyphRun == other.glyphRun && self.glyphRunDescription == other.glyphRunDescription && self.baselineOriginX == other.baselineOriginX && self.baselineOriginY == other.baselineOriginY && self.runColor == other.runColor && self.paletteIndex == other.paletteIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWRITE_COLOR_GLYPH_RUN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWRITE_COLOR_GLYPH_RUN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_COLOR_GLYPH_RUN1 {
    pub Base: DWRITE_COLOR_GLYPH_RUN,
    pub glyphImageFormat: DWRITE_GLYPH_IMAGE_FORMATS,
    pub measuringMode: DWRITE_MEASURING_MODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_COLOR_GLYPH_RUN1 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DWRITE_COLOR_GLYPH_RUN1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_COLOR_GLYPH_RUN1").field("Base", &self.Base).field("glyphImageFormat", &self.glyphImageFormat).field("measuringMode", &self.measuringMode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DWRITE_COLOR_GLYPH_RUN1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWRITE_COLOR_GLYPH_RUN1 {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.glyphImageFormat == other.glyphImageFormat && self.measuringMode == other.measuringMode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWRITE_COLOR_GLYPH_RUN1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWRITE_COLOR_GLYPH_RUN1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_FILE_FRAGMENT {
    pub fileOffset: u64,
    pub fragmentSize: u64,
}
impl ::core::marker::Copy for DWRITE_FILE_FRAGMENT {}
impl ::core::clone::Clone for DWRITE_FILE_FRAGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_FILE_FRAGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_FILE_FRAGMENT").field("fileOffset", &self.fileOffset).field("fragmentSize", &self.fragmentSize).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_FILE_FRAGMENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_FILE_FRAGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.fileOffset == other.fileOffset && self.fragmentSize == other.fragmentSize
    }
}
impl ::core::cmp::Eq for DWRITE_FILE_FRAGMENT {}
impl ::core::default::Default for DWRITE_FILE_FRAGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_FONT_AXIS_RANGE {
    pub axisTag: DWRITE_FONT_AXIS_TAG,
    pub minValue: f32,
    pub maxValue: f32,
}
impl ::core::marker::Copy for DWRITE_FONT_AXIS_RANGE {}
impl ::core::clone::Clone for DWRITE_FONT_AXIS_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_AXIS_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_FONT_AXIS_RANGE").field("axisTag", &self.axisTag).field("minValue", &self.minValue).field("maxValue", &self.maxValue).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_AXIS_RANGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_FONT_AXIS_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.axisTag == other.axisTag && self.minValue == other.minValue && self.maxValue == other.maxValue
    }
}
impl ::core::cmp::Eq for DWRITE_FONT_AXIS_RANGE {}
impl ::core::default::Default for DWRITE_FONT_AXIS_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_FONT_AXIS_VALUE {
    pub axisTag: DWRITE_FONT_AXIS_TAG,
    pub value: f32,
}
impl ::core::marker::Copy for DWRITE_FONT_AXIS_VALUE {}
impl ::core::clone::Clone for DWRITE_FONT_AXIS_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_AXIS_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_FONT_AXIS_VALUE").field("axisTag", &self.axisTag).field("value", &self.value).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_AXIS_VALUE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_FONT_AXIS_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.axisTag == other.axisTag && self.value == other.value
    }
}
impl ::core::cmp::Eq for DWRITE_FONT_AXIS_VALUE {}
impl ::core::default::Default for DWRITE_FONT_AXIS_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_FONT_FEATURE {
    pub nameTag: DWRITE_FONT_FEATURE_TAG,
    pub parameter: u32,
}
impl ::core::marker::Copy for DWRITE_FONT_FEATURE {}
impl ::core::clone::Clone for DWRITE_FONT_FEATURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_FONT_FEATURE").field("nameTag", &self.nameTag).field("parameter", &self.parameter).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_FEATURE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_FONT_FEATURE {
    fn eq(&self, other: &Self) -> bool {
        self.nameTag == other.nameTag && self.parameter == other.parameter
    }
}
impl ::core::cmp::Eq for DWRITE_FONT_FEATURE {}
impl ::core::default::Default for DWRITE_FONT_FEATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_FONT_METRICS {
    pub designUnitsPerEm: u16,
    pub ascent: u16,
    pub descent: u16,
    pub lineGap: i16,
    pub capHeight: u16,
    pub xHeight: u16,
    pub underlinePosition: i16,
    pub underlineThickness: u16,
    pub strikethroughPosition: i16,
    pub strikethroughThickness: u16,
}
impl ::core::marker::Copy for DWRITE_FONT_METRICS {}
impl ::core::clone::Clone for DWRITE_FONT_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_FONT_METRICS")
            .field("designUnitsPerEm", &self.designUnitsPerEm)
            .field("ascent", &self.ascent)
            .field("descent", &self.descent)
            .field("lineGap", &self.lineGap)
            .field("capHeight", &self.capHeight)
            .field("xHeight", &self.xHeight)
            .field("underlinePosition", &self.underlinePosition)
            .field("underlineThickness", &self.underlineThickness)
            .field("strikethroughPosition", &self.strikethroughPosition)
            .field("strikethroughThickness", &self.strikethroughThickness)
            .finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_METRICS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_FONT_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.designUnitsPerEm == other.designUnitsPerEm && self.ascent == other.ascent && self.descent == other.descent && self.lineGap == other.lineGap && self.capHeight == other.capHeight && self.xHeight == other.xHeight && self.underlinePosition == other.underlinePosition && self.underlineThickness == other.underlineThickness && self.strikethroughPosition == other.strikethroughPosition && self.strikethroughThickness == other.strikethroughThickness
    }
}
impl ::core::cmp::Eq for DWRITE_FONT_METRICS {}
impl ::core::default::Default for DWRITE_FONT_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_FONT_METRICS1 {
    pub Base: DWRITE_FONT_METRICS,
    pub glyphBoxLeft: i16,
    pub glyphBoxTop: i16,
    pub glyphBoxRight: i16,
    pub glyphBoxBottom: i16,
    pub subscriptPositionX: i16,
    pub subscriptPositionY: i16,
    pub subscriptSizeX: i16,
    pub subscriptSizeY: i16,
    pub superscriptPositionX: i16,
    pub superscriptPositionY: i16,
    pub superscriptSizeX: i16,
    pub superscriptSizeY: i16,
    pub hasTypographicMetrics: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_FONT_METRICS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_FONT_METRICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DWRITE_FONT_METRICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_FONT_METRICS1")
            .field("Base", &self.Base)
            .field("glyphBoxLeft", &self.glyphBoxLeft)
            .field("glyphBoxTop", &self.glyphBoxTop)
            .field("glyphBoxRight", &self.glyphBoxRight)
            .field("glyphBoxBottom", &self.glyphBoxBottom)
            .field("subscriptPositionX", &self.subscriptPositionX)
            .field("subscriptPositionY", &self.subscriptPositionY)
            .field("subscriptSizeX", &self.subscriptSizeX)
            .field("subscriptSizeY", &self.subscriptSizeY)
            .field("superscriptPositionX", &self.superscriptPositionX)
            .field("superscriptPositionY", &self.superscriptPositionY)
            .field("superscriptSizeX", &self.superscriptSizeX)
            .field("superscriptSizeY", &self.superscriptSizeY)
            .field("hasTypographicMetrics", &self.hasTypographicMetrics)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DWRITE_FONT_METRICS1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWRITE_FONT_METRICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.glyphBoxLeft == other.glyphBoxLeft && self.glyphBoxTop == other.glyphBoxTop && self.glyphBoxRight == other.glyphBoxRight && self.glyphBoxBottom == other.glyphBoxBottom && self.subscriptPositionX == other.subscriptPositionX && self.subscriptPositionY == other.subscriptPositionY && self.subscriptSizeX == other.subscriptSizeX && self.subscriptSizeY == other.subscriptSizeY && self.superscriptPositionX == other.superscriptPositionX && self.superscriptPositionY == other.superscriptPositionY && self.superscriptSizeX == other.superscriptSizeX && self.superscriptSizeY == other.superscriptSizeY && self.hasTypographicMetrics == other.hasTypographicMetrics
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWRITE_FONT_METRICS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWRITE_FONT_METRICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_FONT_PROPERTY {
    pub propertyId: DWRITE_FONT_PROPERTY_ID,
    pub propertyValue: ::windows_core::PCWSTR,
    pub localeName: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for DWRITE_FONT_PROPERTY {}
impl ::core::clone::Clone for DWRITE_FONT_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_FONT_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_FONT_PROPERTY").field("propertyId", &self.propertyId).field("propertyValue", &self.propertyValue).field("localeName", &self.localeName).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_FONT_PROPERTY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_FONT_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.propertyId == other.propertyId && self.propertyValue == other.propertyValue && self.localeName == other.localeName
    }
}
impl ::core::cmp::Eq for DWRITE_FONT_PROPERTY {}
impl ::core::default::Default for DWRITE_FONT_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub struct DWRITE_GLYPH_IMAGE_DATA {
    pub imageData: *const ::core::ffi::c_void,
    pub imageDataSize: u32,
    pub uniqueDataId: u32,
    pub pixelsPerEm: u32,
    pub pixelSize: super::Direct2D::Common::D2D_SIZE_U,
    pub horizontalLeftOrigin: super::super::Foundation::POINT,
    pub horizontalRightOrigin: super::super::Foundation::POINT,
    pub verticalTopOrigin: super::super::Foundation::POINT,
    pub verticalBottomOrigin: super::super::Foundation::POINT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::marker::Copy for DWRITE_GLYPH_IMAGE_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::clone::Clone for DWRITE_GLYPH_IMAGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::fmt::Debug for DWRITE_GLYPH_IMAGE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_GLYPH_IMAGE_DATA").field("imageData", &self.imageData).field("imageDataSize", &self.imageDataSize).field("uniqueDataId", &self.uniqueDataId).field("pixelsPerEm", &self.pixelsPerEm).field("pixelSize", &self.pixelSize).field("horizontalLeftOrigin", &self.horizontalLeftOrigin).field("horizontalRightOrigin", &self.horizontalRightOrigin).field("verticalTopOrigin", &self.verticalTopOrigin).field("verticalBottomOrigin", &self.verticalBottomOrigin).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::TypeKind for DWRITE_GLYPH_IMAGE_DATA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::cmp::PartialEq for DWRITE_GLYPH_IMAGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.imageData == other.imageData && self.imageDataSize == other.imageDataSize && self.uniqueDataId == other.uniqueDataId && self.pixelsPerEm == other.pixelsPerEm && self.pixelSize == other.pixelSize && self.horizontalLeftOrigin == other.horizontalLeftOrigin && self.horizontalRightOrigin == other.horizontalRightOrigin && self.verticalTopOrigin == other.verticalTopOrigin && self.verticalBottomOrigin == other.verticalBottomOrigin
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::cmp::Eq for DWRITE_GLYPH_IMAGE_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::core::default::Default for DWRITE_GLYPH_IMAGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_GLYPH_METRICS {
    pub leftSideBearing: i32,
    pub advanceWidth: u32,
    pub rightSideBearing: i32,
    pub topSideBearing: i32,
    pub advanceHeight: u32,
    pub bottomSideBearing: i32,
    pub verticalOriginY: i32,
}
impl ::core::marker::Copy for DWRITE_GLYPH_METRICS {}
impl ::core::clone::Clone for DWRITE_GLYPH_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_GLYPH_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_GLYPH_METRICS").field("leftSideBearing", &self.leftSideBearing).field("advanceWidth", &self.advanceWidth).field("rightSideBearing", &self.rightSideBearing).field("topSideBearing", &self.topSideBearing).field("advanceHeight", &self.advanceHeight).field("bottomSideBearing", &self.bottomSideBearing).field("verticalOriginY", &self.verticalOriginY).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_GLYPH_METRICS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_GLYPH_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.leftSideBearing == other.leftSideBearing && self.advanceWidth == other.advanceWidth && self.rightSideBearing == other.rightSideBearing && self.topSideBearing == other.topSideBearing && self.advanceHeight == other.advanceHeight && self.bottomSideBearing == other.bottomSideBearing && self.verticalOriginY == other.verticalOriginY
    }
}
impl ::core::cmp::Eq for DWRITE_GLYPH_METRICS {}
impl ::core::default::Default for DWRITE_GLYPH_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_GLYPH_OFFSET {
    pub advanceOffset: f32,
    pub ascenderOffset: f32,
}
impl ::core::marker::Copy for DWRITE_GLYPH_OFFSET {}
impl ::core::clone::Clone for DWRITE_GLYPH_OFFSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_GLYPH_OFFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_GLYPH_OFFSET").field("advanceOffset", &self.advanceOffset).field("ascenderOffset", &self.ascenderOffset).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_GLYPH_OFFSET {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_GLYPH_OFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.advanceOffset == other.advanceOffset && self.ascenderOffset == other.ascenderOffset
    }
}
impl ::core::cmp::Eq for DWRITE_GLYPH_OFFSET {}
impl ::core::default::Default for DWRITE_GLYPH_OFFSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_GLYPH_RUN {
    pub fontFace: ::std::mem::ManuallyDrop<::core::option::Option<IDWriteFontFace>>,
    pub fontEmSize: f32,
    pub glyphCount: u32,
    pub glyphIndices: *const u16,
    pub glyphAdvances: *const f32,
    pub glyphOffsets: *const DWRITE_GLYPH_OFFSET,
    pub isSideways: super::super::Foundation::BOOL,
    pub bidiLevel: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_GLYPH_RUN {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DWRITE_GLYPH_RUN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_GLYPH_RUN").field("fontFace", &self.fontFace).field("fontEmSize", &self.fontEmSize).field("glyphCount", &self.glyphCount).field("glyphIndices", &self.glyphIndices).field("glyphAdvances", &self.glyphAdvances).field("glyphOffsets", &self.glyphOffsets).field("isSideways", &self.isSideways).field("bidiLevel", &self.bidiLevel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DWRITE_GLYPH_RUN {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWRITE_GLYPH_RUN {
    fn eq(&self, other: &Self) -> bool {
        self.fontFace == other.fontFace && self.fontEmSize == other.fontEmSize && self.glyphCount == other.glyphCount && self.glyphIndices == other.glyphIndices && self.glyphAdvances == other.glyphAdvances && self.glyphOffsets == other.glyphOffsets && self.isSideways == other.isSideways && self.bidiLevel == other.bidiLevel
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWRITE_GLYPH_RUN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWRITE_GLYPH_RUN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_GLYPH_RUN_DESCRIPTION {
    pub localeName: ::windows_core::PCWSTR,
    pub string: ::windows_core::PCWSTR,
    pub stringLength: u32,
    pub clusterMap: *const u16,
    pub textPosition: u32,
}
impl ::core::marker::Copy for DWRITE_GLYPH_RUN_DESCRIPTION {}
impl ::core::clone::Clone for DWRITE_GLYPH_RUN_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_GLYPH_RUN_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_GLYPH_RUN_DESCRIPTION").field("localeName", &self.localeName).field("string", &self.string).field("stringLength", &self.stringLength).field("clusterMap", &self.clusterMap).field("textPosition", &self.textPosition).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_GLYPH_RUN_DESCRIPTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_GLYPH_RUN_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.localeName == other.localeName && self.string == other.string && self.stringLength == other.stringLength && self.clusterMap == other.clusterMap && self.textPosition == other.textPosition
    }
}
impl ::core::cmp::Eq for DWRITE_GLYPH_RUN_DESCRIPTION {}
impl ::core::default::Default for DWRITE_GLYPH_RUN_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_HIT_TEST_METRICS {
    pub textPosition: u32,
    pub length: u32,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub bidiLevel: u32,
    pub isText: super::super::Foundation::BOOL,
    pub isTrimmed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_HIT_TEST_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_HIT_TEST_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DWRITE_HIT_TEST_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_HIT_TEST_METRICS").field("textPosition", &self.textPosition).field("length", &self.length).field("left", &self.left).field("top", &self.top).field("width", &self.width).field("height", &self.height).field("bidiLevel", &self.bidiLevel).field("isText", &self.isText).field("isTrimmed", &self.isTrimmed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DWRITE_HIT_TEST_METRICS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWRITE_HIT_TEST_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.textPosition == other.textPosition && self.length == other.length && self.left == other.left && self.top == other.top && self.width == other.width && self.height == other.height && self.bidiLevel == other.bidiLevel && self.isText == other.isText && self.isTrimmed == other.isTrimmed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWRITE_HIT_TEST_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWRITE_HIT_TEST_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_INLINE_OBJECT_METRICS {
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
    pub supportsSideways: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_INLINE_OBJECT_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_INLINE_OBJECT_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DWRITE_INLINE_OBJECT_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_INLINE_OBJECT_METRICS").field("width", &self.width).field("height", &self.height).field("baseline", &self.baseline).field("supportsSideways", &self.supportsSideways).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DWRITE_INLINE_OBJECT_METRICS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWRITE_INLINE_OBJECT_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.baseline == other.baseline && self.supportsSideways == other.supportsSideways
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWRITE_INLINE_OBJECT_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWRITE_INLINE_OBJECT_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_JUSTIFICATION_OPPORTUNITY {
    pub expansionMinimum: f32,
    pub expansionMaximum: f32,
    pub compressionMaximum: f32,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for DWRITE_JUSTIFICATION_OPPORTUNITY {}
impl ::core::clone::Clone for DWRITE_JUSTIFICATION_OPPORTUNITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_JUSTIFICATION_OPPORTUNITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_JUSTIFICATION_OPPORTUNITY").field("expansionMinimum", &self.expansionMinimum).field("expansionMaximum", &self.expansionMaximum).field("compressionMaximum", &self.compressionMaximum).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_JUSTIFICATION_OPPORTUNITY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_JUSTIFICATION_OPPORTUNITY {
    fn eq(&self, other: &Self) -> bool {
        self.expansionMinimum == other.expansionMinimum && self.expansionMaximum == other.expansionMaximum && self.compressionMaximum == other.compressionMaximum && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DWRITE_JUSTIFICATION_OPPORTUNITY {}
impl ::core::default::Default for DWRITE_JUSTIFICATION_OPPORTUNITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_LINE_BREAKPOINT {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for DWRITE_LINE_BREAKPOINT {}
impl ::core::clone::Clone for DWRITE_LINE_BREAKPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_LINE_BREAKPOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_LINE_BREAKPOINT").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_LINE_BREAKPOINT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_LINE_BREAKPOINT {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DWRITE_LINE_BREAKPOINT {}
impl ::core::default::Default for DWRITE_LINE_BREAKPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_LINE_METRICS {
    pub length: u32,
    pub trailingWhitespaceLength: u32,
    pub newlineLength: u32,
    pub height: f32,
    pub baseline: f32,
    pub isTrimmed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_LINE_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_LINE_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DWRITE_LINE_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_LINE_METRICS").field("length", &self.length).field("trailingWhitespaceLength", &self.trailingWhitespaceLength).field("newlineLength", &self.newlineLength).field("height", &self.height).field("baseline", &self.baseline).field("isTrimmed", &self.isTrimmed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DWRITE_LINE_METRICS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWRITE_LINE_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.trailingWhitespaceLength == other.trailingWhitespaceLength && self.newlineLength == other.newlineLength && self.height == other.height && self.baseline == other.baseline && self.isTrimmed == other.isTrimmed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWRITE_LINE_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWRITE_LINE_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct DWRITE_LINE_METRICS1 {
    pub Base: DWRITE_LINE_METRICS,
    pub leadingBefore: f32,
    pub leadingAfter: f32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DWRITE_LINE_METRICS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DWRITE_LINE_METRICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DWRITE_LINE_METRICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_LINE_METRICS1").field("Base", &self.Base).field("leadingBefore", &self.leadingBefore).field("leadingAfter", &self.leadingAfter).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DWRITE_LINE_METRICS1 {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DWRITE_LINE_METRICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.leadingBefore == other.leadingBefore && self.leadingAfter == other.leadingAfter
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DWRITE_LINE_METRICS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DWRITE_LINE_METRICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_LINE_SPACING {
    pub method: DWRITE_LINE_SPACING_METHOD,
    pub height: f32,
    pub baseline: f32,
    pub leadingBefore: f32,
    pub fontLineGapUsage: DWRITE_FONT_LINE_GAP_USAGE,
}
impl ::core::marker::Copy for DWRITE_LINE_SPACING {}
impl ::core::clone::Clone for DWRITE_LINE_SPACING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_LINE_SPACING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_LINE_SPACING").field("method", &self.method).field("height", &self.height).field("baseline", &self.baseline).field("leadingBefore", &self.leadingBefore).field("fontLineGapUsage", &self.fontLineGapUsage).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_LINE_SPACING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_LINE_SPACING {
    fn eq(&self, other: &Self) -> bool {
        self.method == other.method && self.height == other.height && self.baseline == other.baseline && self.leadingBefore == other.leadingBefore && self.fontLineGapUsage == other.fontLineGapUsage
    }
}
impl ::core::cmp::Eq for DWRITE_LINE_SPACING {}
impl ::core::default::Default for DWRITE_LINE_SPACING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}
impl ::core::marker::Copy for DWRITE_MATRIX {}
impl ::core::clone::Clone for DWRITE_MATRIX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_MATRIX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_MATRIX").field("m11", &self.m11).field("m12", &self.m12).field("m21", &self.m21).field("m22", &self.m22).field("dx", &self.dx).field("dy", &self.dy).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_MATRIX {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_MATRIX {
    fn eq(&self, other: &Self) -> bool {
        self.m11 == other.m11 && self.m12 == other.m12 && self.m21 == other.m21 && self.m22 == other.m22 && self.dx == other.dx && self.dy == other.dy
    }
}
impl ::core::cmp::Eq for DWRITE_MATRIX {}
impl ::core::default::Default for DWRITE_MATRIX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_OVERHANG_METRICS {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl ::core::marker::Copy for DWRITE_OVERHANG_METRICS {}
impl ::core::clone::Clone for DWRITE_OVERHANG_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_OVERHANG_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_OVERHANG_METRICS").field("left", &self.left).field("top", &self.top).field("right", &self.right).field("bottom", &self.bottom).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_OVERHANG_METRICS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_OVERHANG_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.right == other.right && self.bottom == other.bottom
    }
}
impl ::core::cmp::Eq for DWRITE_OVERHANG_METRICS {}
impl ::core::default::Default for DWRITE_OVERHANG_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DWRITE_PANOSE {
    pub values: [u8; 10],
    pub familyKind: u8,
    pub text: DWRITE_PANOSE_3,
    pub script: DWRITE_PANOSE_1,
    pub decorative: DWRITE_PANOSE_0,
    pub symbol: DWRITE_PANOSE_2,
}
impl ::core::marker::Copy for DWRITE_PANOSE {}
impl ::core::clone::Clone for DWRITE_PANOSE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for DWRITE_PANOSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_PANOSE_0 {
    pub familyKind: u8,
    pub decorativeClass: u8,
    pub weight: u8,
    pub aspect: u8,
    pub contrast: u8,
    pub serifVariant: u8,
    pub fill: u8,
    pub lining: u8,
    pub decorativeTopology: u8,
    pub characterRange: u8,
}
impl ::core::marker::Copy for DWRITE_PANOSE_0 {}
impl ::core::clone::Clone for DWRITE_PANOSE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_PANOSE_0").field("familyKind", &self.familyKind).field("decorativeClass", &self.decorativeClass).field("weight", &self.weight).field("aspect", &self.aspect).field("contrast", &self.contrast).field("serifVariant", &self.serifVariant).field("fill", &self.fill).field("lining", &self.lining).field("decorativeTopology", &self.decorativeTopology).field("characterRange", &self.characterRange).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_PANOSE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.familyKind == other.familyKind && self.decorativeClass == other.decorativeClass && self.weight == other.weight && self.aspect == other.aspect && self.contrast == other.contrast && self.serifVariant == other.serifVariant && self.fill == other.fill && self.lining == other.lining && self.decorativeTopology == other.decorativeTopology && self.characterRange == other.characterRange
    }
}
impl ::core::cmp::Eq for DWRITE_PANOSE_0 {}
impl ::core::default::Default for DWRITE_PANOSE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_PANOSE_1 {
    pub familyKind: u8,
    pub toolKind: u8,
    pub weight: u8,
    pub spacing: u8,
    pub aspectRatio: u8,
    pub contrast: u8,
    pub scriptTopology: u8,
    pub scriptForm: u8,
    pub finials: u8,
    pub xAscent: u8,
}
impl ::core::marker::Copy for DWRITE_PANOSE_1 {}
impl ::core::clone::Clone for DWRITE_PANOSE_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_PANOSE_1").field("familyKind", &self.familyKind).field("toolKind", &self.toolKind).field("weight", &self.weight).field("spacing", &self.spacing).field("aspectRatio", &self.aspectRatio).field("contrast", &self.contrast).field("scriptTopology", &self.scriptTopology).field("scriptForm", &self.scriptForm).field("finials", &self.finials).field("xAscent", &self.xAscent).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_PANOSE_1 {
    fn eq(&self, other: &Self) -> bool {
        self.familyKind == other.familyKind && self.toolKind == other.toolKind && self.weight == other.weight && self.spacing == other.spacing && self.aspectRatio == other.aspectRatio && self.contrast == other.contrast && self.scriptTopology == other.scriptTopology && self.scriptForm == other.scriptForm && self.finials == other.finials && self.xAscent == other.xAscent
    }
}
impl ::core::cmp::Eq for DWRITE_PANOSE_1 {}
impl ::core::default::Default for DWRITE_PANOSE_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_PANOSE_2 {
    pub familyKind: u8,
    pub symbolKind: u8,
    pub weight: u8,
    pub spacing: u8,
    pub aspectRatioAndContrast: u8,
    pub aspectRatio94: u8,
    pub aspectRatio119: u8,
    pub aspectRatio157: u8,
    pub aspectRatio163: u8,
    pub aspectRatio211: u8,
}
impl ::core::marker::Copy for DWRITE_PANOSE_2 {}
impl ::core::clone::Clone for DWRITE_PANOSE_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_PANOSE_2").field("familyKind", &self.familyKind).field("symbolKind", &self.symbolKind).field("weight", &self.weight).field("spacing", &self.spacing).field("aspectRatioAndContrast", &self.aspectRatioAndContrast).field("aspectRatio94", &self.aspectRatio94).field("aspectRatio119", &self.aspectRatio119).field("aspectRatio157", &self.aspectRatio157).field("aspectRatio163", &self.aspectRatio163).field("aspectRatio211", &self.aspectRatio211).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_PANOSE_2 {
    fn eq(&self, other: &Self) -> bool {
        self.familyKind == other.familyKind && self.symbolKind == other.symbolKind && self.weight == other.weight && self.spacing == other.spacing && self.aspectRatioAndContrast == other.aspectRatioAndContrast && self.aspectRatio94 == other.aspectRatio94 && self.aspectRatio119 == other.aspectRatio119 && self.aspectRatio157 == other.aspectRatio157 && self.aspectRatio163 == other.aspectRatio163 && self.aspectRatio211 == other.aspectRatio211
    }
}
impl ::core::cmp::Eq for DWRITE_PANOSE_2 {}
impl ::core::default::Default for DWRITE_PANOSE_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_PANOSE_3 {
    pub familyKind: u8,
    pub serifStyle: u8,
    pub weight: u8,
    pub proportion: u8,
    pub contrast: u8,
    pub strokeVariation: u8,
    pub armStyle: u8,
    pub letterform: u8,
    pub midline: u8,
    pub xHeight: u8,
}
impl ::core::marker::Copy for DWRITE_PANOSE_3 {}
impl ::core::clone::Clone for DWRITE_PANOSE_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_PANOSE_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_PANOSE_3").field("familyKind", &self.familyKind).field("serifStyle", &self.serifStyle).field("weight", &self.weight).field("proportion", &self.proportion).field("contrast", &self.contrast).field("strokeVariation", &self.strokeVariation).field("armStyle", &self.armStyle).field("letterform", &self.letterform).field("midline", &self.midline).field("xHeight", &self.xHeight).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_PANOSE_3 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_PANOSE_3 {
    fn eq(&self, other: &Self) -> bool {
        self.familyKind == other.familyKind && self.serifStyle == other.serifStyle && self.weight == other.weight && self.proportion == other.proportion && self.contrast == other.contrast && self.strokeVariation == other.strokeVariation && self.armStyle == other.armStyle && self.letterform == other.letterform && self.midline == other.midline && self.xHeight == other.xHeight
    }
}
impl ::core::cmp::Eq for DWRITE_PANOSE_3 {}
impl ::core::default::Default for DWRITE_PANOSE_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_SCRIPT_ANALYSIS {
    pub script: u16,
    pub shapes: DWRITE_SCRIPT_SHAPES,
}
impl ::core::marker::Copy for DWRITE_SCRIPT_ANALYSIS {}
impl ::core::clone::Clone for DWRITE_SCRIPT_ANALYSIS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_SCRIPT_ANALYSIS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_SCRIPT_ANALYSIS").field("script", &self.script).field("shapes", &self.shapes).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_SCRIPT_ANALYSIS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_SCRIPT_ANALYSIS {
    fn eq(&self, other: &Self) -> bool {
        self.script == other.script && self.shapes == other.shapes
    }
}
impl ::core::cmp::Eq for DWRITE_SCRIPT_ANALYSIS {}
impl ::core::default::Default for DWRITE_SCRIPT_ANALYSIS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_SCRIPT_PROPERTIES {
    pub isoScriptCode: u32,
    pub isoScriptNumber: u32,
    pub clusterLookahead: u32,
    pub justificationCharacter: u32,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for DWRITE_SCRIPT_PROPERTIES {}
impl ::core::clone::Clone for DWRITE_SCRIPT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_SCRIPT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_SCRIPT_PROPERTIES").field("isoScriptCode", &self.isoScriptCode).field("isoScriptNumber", &self.isoScriptNumber).field("clusterLookahead", &self.clusterLookahead).field("justificationCharacter", &self.justificationCharacter).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_SCRIPT_PROPERTIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_SCRIPT_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.isoScriptCode == other.isoScriptCode && self.isoScriptNumber == other.isoScriptNumber && self.clusterLookahead == other.clusterLookahead && self.justificationCharacter == other.justificationCharacter && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DWRITE_SCRIPT_PROPERTIES {}
impl ::core::default::Default for DWRITE_SCRIPT_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_SHAPING_GLYPH_PROPERTIES {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for DWRITE_SHAPING_GLYPH_PROPERTIES {}
impl ::core::clone::Clone for DWRITE_SHAPING_GLYPH_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_SHAPING_GLYPH_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_SHAPING_GLYPH_PROPERTIES").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_SHAPING_GLYPH_PROPERTIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_SHAPING_GLYPH_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DWRITE_SHAPING_GLYPH_PROPERTIES {}
impl ::core::default::Default for DWRITE_SHAPING_GLYPH_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_SHAPING_TEXT_PROPERTIES {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for DWRITE_SHAPING_TEXT_PROPERTIES {}
impl ::core::clone::Clone for DWRITE_SHAPING_TEXT_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_SHAPING_TEXT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_SHAPING_TEXT_PROPERTIES").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_SHAPING_TEXT_PROPERTIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_SHAPING_TEXT_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DWRITE_SHAPING_TEXT_PROPERTIES {}
impl ::core::default::Default for DWRITE_SHAPING_TEXT_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_STRIKETHROUGH {
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub readingDirection: DWRITE_READING_DIRECTION,
    pub flowDirection: DWRITE_FLOW_DIRECTION,
    pub localeName: ::windows_core::PCWSTR,
    pub measuringMode: DWRITE_MEASURING_MODE,
}
impl ::core::marker::Copy for DWRITE_STRIKETHROUGH {}
impl ::core::clone::Clone for DWRITE_STRIKETHROUGH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_STRIKETHROUGH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_STRIKETHROUGH").field("width", &self.width).field("thickness", &self.thickness).field("offset", &self.offset).field("readingDirection", &self.readingDirection).field("flowDirection", &self.flowDirection).field("localeName", &self.localeName).field("measuringMode", &self.measuringMode).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_STRIKETHROUGH {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_STRIKETHROUGH {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.thickness == other.thickness && self.offset == other.offset && self.readingDirection == other.readingDirection && self.flowDirection == other.flowDirection && self.localeName == other.localeName && self.measuringMode == other.measuringMode
    }
}
impl ::core::cmp::Eq for DWRITE_STRIKETHROUGH {}
impl ::core::default::Default for DWRITE_STRIKETHROUGH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_TEXT_METRICS {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub widthIncludingTrailingWhitespace: f32,
    pub height: f32,
    pub layoutWidth: f32,
    pub layoutHeight: f32,
    pub maxBidiReorderingDepth: u32,
    pub lineCount: u32,
}
impl ::core::marker::Copy for DWRITE_TEXT_METRICS {}
impl ::core::clone::Clone for DWRITE_TEXT_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_TEXT_METRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_TEXT_METRICS").field("left", &self.left).field("top", &self.top).field("width", &self.width).field("widthIncludingTrailingWhitespace", &self.widthIncludingTrailingWhitespace).field("height", &self.height).field("layoutWidth", &self.layoutWidth).field("layoutHeight", &self.layoutHeight).field("maxBidiReorderingDepth", &self.maxBidiReorderingDepth).field("lineCount", &self.lineCount).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_TEXT_METRICS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_TEXT_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.width == other.width && self.widthIncludingTrailingWhitespace == other.widthIncludingTrailingWhitespace && self.height == other.height && self.layoutWidth == other.layoutWidth && self.layoutHeight == other.layoutHeight && self.maxBidiReorderingDepth == other.maxBidiReorderingDepth && self.lineCount == other.lineCount
    }
}
impl ::core::cmp::Eq for DWRITE_TEXT_METRICS {}
impl ::core::default::Default for DWRITE_TEXT_METRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_TEXT_METRICS1 {
    pub Base: DWRITE_TEXT_METRICS,
    pub heightIncludingTrailingWhitespace: f32,
}
impl ::core::marker::Copy for DWRITE_TEXT_METRICS1 {}
impl ::core::clone::Clone for DWRITE_TEXT_METRICS1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_TEXT_METRICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_TEXT_METRICS1").field("Base", &self.Base).field("heightIncludingTrailingWhitespace", &self.heightIncludingTrailingWhitespace).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_TEXT_METRICS1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_TEXT_METRICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.Base == other.Base && self.heightIncludingTrailingWhitespace == other.heightIncludingTrailingWhitespace
    }
}
impl ::core::cmp::Eq for DWRITE_TEXT_METRICS1 {}
impl ::core::default::Default for DWRITE_TEXT_METRICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_TEXT_RANGE {
    pub startPosition: u32,
    pub length: u32,
}
impl ::core::marker::Copy for DWRITE_TEXT_RANGE {}
impl ::core::clone::Clone for DWRITE_TEXT_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_TEXT_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_TEXT_RANGE").field("startPosition", &self.startPosition).field("length", &self.length).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_TEXT_RANGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_TEXT_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.startPosition == other.startPosition && self.length == other.length
    }
}
impl ::core::cmp::Eq for DWRITE_TEXT_RANGE {}
impl ::core::default::Default for DWRITE_TEXT_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_TRIMMING {
    pub granularity: DWRITE_TRIMMING_GRANULARITY,
    pub delimiter: u32,
    pub delimiterCount: u32,
}
impl ::core::marker::Copy for DWRITE_TRIMMING {}
impl ::core::clone::Clone for DWRITE_TRIMMING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_TRIMMING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_TRIMMING").field("granularity", &self.granularity).field("delimiter", &self.delimiter).field("delimiterCount", &self.delimiterCount).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_TRIMMING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_TRIMMING {
    fn eq(&self, other: &Self) -> bool {
        self.granularity == other.granularity && self.delimiter == other.delimiter && self.delimiterCount == other.delimiterCount
    }
}
impl ::core::cmp::Eq for DWRITE_TRIMMING {}
impl ::core::default::Default for DWRITE_TRIMMING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_TYPOGRAPHIC_FEATURES {
    pub features: *mut DWRITE_FONT_FEATURE,
    pub featureCount: u32,
}
impl ::core::marker::Copy for DWRITE_TYPOGRAPHIC_FEATURES {}
impl ::core::clone::Clone for DWRITE_TYPOGRAPHIC_FEATURES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_TYPOGRAPHIC_FEATURES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_TYPOGRAPHIC_FEATURES").field("features", &self.features).field("featureCount", &self.featureCount).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_TYPOGRAPHIC_FEATURES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_TYPOGRAPHIC_FEATURES {
    fn eq(&self, other: &Self) -> bool {
        self.features == other.features && self.featureCount == other.featureCount
    }
}
impl ::core::cmp::Eq for DWRITE_TYPOGRAPHIC_FEATURES {}
impl ::core::default::Default for DWRITE_TYPOGRAPHIC_FEATURES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_UNDERLINE {
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub runHeight: f32,
    pub readingDirection: DWRITE_READING_DIRECTION,
    pub flowDirection: DWRITE_FLOW_DIRECTION,
    pub localeName: ::windows_core::PCWSTR,
    pub measuringMode: DWRITE_MEASURING_MODE,
}
impl ::core::marker::Copy for DWRITE_UNDERLINE {}
impl ::core::clone::Clone for DWRITE_UNDERLINE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_UNDERLINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_UNDERLINE").field("width", &self.width).field("thickness", &self.thickness).field("offset", &self.offset).field("runHeight", &self.runHeight).field("readingDirection", &self.readingDirection).field("flowDirection", &self.flowDirection).field("localeName", &self.localeName).field("measuringMode", &self.measuringMode).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_UNDERLINE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_UNDERLINE {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.thickness == other.thickness && self.offset == other.offset && self.runHeight == other.runHeight && self.readingDirection == other.readingDirection && self.flowDirection == other.flowDirection && self.localeName == other.localeName && self.measuringMode == other.measuringMode
    }
}
impl ::core::cmp::Eq for DWRITE_UNDERLINE {}
impl ::core::default::Default for DWRITE_UNDERLINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DWRITE_UNICODE_RANGE {
    pub first: u32,
    pub last: u32,
}
impl ::core::marker::Copy for DWRITE_UNICODE_RANGE {}
impl ::core::clone::Clone for DWRITE_UNICODE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DWRITE_UNICODE_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DWRITE_UNICODE_RANGE").field("first", &self.first).field("last", &self.last).finish()
    }
}
impl ::windows_core::TypeKind for DWRITE_UNICODE_RANGE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DWRITE_UNICODE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.last == other.last
    }
}
impl ::core::cmp::Eq for DWRITE_UNICODE_RANGE {}
impl ::core::default::Default for DWRITE_UNICODE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
