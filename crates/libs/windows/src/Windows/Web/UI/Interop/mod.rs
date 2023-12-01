#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebViewControlAcceleratorKeyPressedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlAcceleratorKeyPressedEventArgs {
    type Vtable = IWebViewControlAcceleratorKeyPressedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebViewControlAcceleratorKeyPressedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77a2a53e_7c74_437d_a290_3ac0d8cd5655);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlAcceleratorKeyPressedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Core")]
    pub EventType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Core::CoreAcceleratorKeyEventType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    EventType: usize,
    #[cfg(feature = "System")]
    pub VirtualKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    VirtualKey: usize,
    #[cfg(feature = "UI_Core")]
    pub KeyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Core::CorePhysicalKeyStatus) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    KeyStatus: usize,
    pub RoutingStage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlAcceleratorKeyRoutingStage) -> ::windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebViewControlMoveFocusRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlMoveFocusRequestedEventArgs {
    type Vtable = IWebViewControlMoveFocusRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebViewControlMoveFocusRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b2a340d_4bd0_405e_b7c1_1e72a492f446);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlMoveFocusRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlMoveFocusReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebViewControlProcess(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlProcess {
    type Vtable = IWebViewControlProcess_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebViewControlProcess {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x02c723ec_98d6_424a_b63e_c6136c36a0f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlProcess_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProcessId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsPrivateNetworkClientServerCapabilityEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateWebViewControlAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostwindowhandle: i64, bounds: super::super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWebViewControlAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetWebViewControls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetWebViewControls: usize,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProcessExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProcessExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProcessExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProcessExited: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebViewControlProcessFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlProcessFactory {
    type Vtable = IWebViewControlProcessFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebViewControlProcessFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47b65cf9_a2d2_453c_b097_f6779d4b8e02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlProcessFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, processoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebViewControlProcessOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlProcessOptions {
    type Vtable = IWebViewControlProcessOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebViewControlProcessOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1cca72a7_3bd6_4826_8261_6c8189505d89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlProcessOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetEnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPrivateNetworkClientServerCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: WebViewControlProcessCapabilityState) -> ::windows_core::HRESULT,
    pub PrivateNetworkClientServerCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebViewControlProcessCapabilityState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebViewControlSite(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlSite {
    type Vtable = IWebViewControlSite_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebViewControlSite {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x133f47c6_12dc_4898_bd47_04967de648ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlSite_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Process: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub Scale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBounds: usize,
    #[cfg(feature = "Foundation")]
    pub Bounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bounds: usize,
    pub SetIsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MoveFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reason: WebViewControlMoveFocusReason) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub MoveFocusRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveFocusRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMoveFocusRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMoveFocusRequested: usize,
    #[cfg(feature = "Foundation")]
    pub AcceleratorKeyPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcceleratorKeyPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAcceleratorKeyPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAcceleratorKeyPressed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWebViewControlSite2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWebViewControlSite2 {
    type Vtable = IWebViewControlSite2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWebViewControlSite2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd13b2e3f_48ee_4730_8243_d2ed0c05606a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebViewControlSite2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGotFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGotFocus: usize,
    #[cfg(feature = "Foundation")]
    pub LostFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LostFocus: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLostFocus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLostFocus: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebViewControl(::windows_core::IUnknown);
impl WebViewControl {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Source(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Source(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetSource<P0>(&self, source: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetSource(source.into_param().abi())).ok() }
    }
    pub fn DocumentTitle(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DocumentTitle(&mut result__)).from_abi(result__)
        }
    }
    pub fn CanGoBack(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CanGoBack(&mut result__)).from_abi(result__)
        }
    }
    pub fn CanGoForward(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CanGoForward(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI\"`"]
    #[cfg(feature = "UI")]
    pub fn SetDefaultBackgroundColor(&self, value: super::super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetDefaultBackgroundColor(value)).ok() }
    }
    #[doc = "Required features: `\"UI\"`"]
    #[cfg(feature = "UI")]
    pub fn DefaultBackgroundColor(&self) -> ::windows_core::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DefaultBackgroundColor(&mut result__)).from_abi(result__)
        }
    }
    pub fn ContainsFullScreenElement(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ContainsFullScreenElement(&mut result__)).from_abi(result__)
        }
    }
    pub fn Settings(&self) -> ::windows_core::Result<super::WebViewControlSettings> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Settings(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeferredPermissionRequests(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<super::WebViewControlDeferredPermissionRequest>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DeferredPermissionRequests(&mut result__)).from_abi(result__)
        }
    }
    pub fn GoForward(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.GoForward()).ok() }
    }
    pub fn GoBack(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.GoBack()).ok() }
    }
    pub fn Refresh(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Refresh()).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Stop()).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Navigate<P0>(&self, source: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Navigate(source.into_param().abi())).ok() }
    }
    pub fn NavigateToString(&self, text: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.NavigateToString(::core::mem::transmute_copy(text))).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn NavigateToLocalStreamUri<P0, P1>(&self, source: P0, streamresolver: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Uri>,
        P1: ::windows_core::TryIntoParam<super::super::IUriToStreamResolver>,
    {
        let this = self;
        unsafe { ::windows_core::vcall!(this.NavigateToLocalStreamUri(source.into_param().abi(), streamresolver.try_into_param()?.abi())).ok() }
    }
    #[doc = "Required features: `\"Web_Http\"`"]
    #[cfg(feature = "Web_Http")]
    pub fn NavigateWithHttpRequestMessage<P0>(&self, requestmessage: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Http::HttpRequestMessage>,
    {
        let this = self;
        unsafe { ::windows_core::vcall!(this.NavigateWithHttpRequestMessage(requestmessage.into_param().abi())).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InvokeScriptAsync<P0>(&self, scriptname: &::windows_core::HSTRING, arguments: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.InvokeScriptAsync(::core::mem::transmute_copy(scriptname), arguments.try_into_param()?.abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CapturePreviewToStreamAsync<P0>(&self, stream: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CapturePreviewToStreamAsync(stream.try_into_param()?.abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"ApplicationModel_DataTransfer\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation"))]
    pub fn CaptureSelectedContentToDataPackageAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::ApplicationModel::DataTransfer::DataPackage>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CaptureSelectedContentToDataPackageAsync(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn BuildLocalStreamUri(&self, contentidentifier: &::windows_core::HSTRING, relativepath: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BuildLocalStreamUri(::core::mem::transmute_copy(contentidentifier), ::core::mem::transmute_copy(relativepath), &mut result__)).from_abi(result__)
        }
    }
    pub fn GetDeferredPermissionRequestById(&self, id: u32, result: &mut ::core::option::Option<super::WebViewControlDeferredPermissionRequest>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.GetDeferredPermissionRequestById(id, result as *mut _ as _)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn NavigationStarting<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNavigationStartingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.NavigationStarting(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNavigationStarting(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveNavigationStarting(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ContentLoading<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlContentLoadingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ContentLoading(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContentLoading(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveContentLoading(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DOMContentLoaded<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlDOMContentLoadedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DOMContentLoaded(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDOMContentLoaded(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveDOMContentLoaded(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn NavigationCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNavigationCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.NavigationCompleted(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNavigationCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveNavigationCompleted(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn FrameNavigationStarting<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNavigationStartingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FrameNavigationStarting(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameNavigationStarting(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveFrameNavigationStarting(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn FrameContentLoading<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlContentLoadingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FrameContentLoading(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameContentLoading(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveFrameContentLoading(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn FrameDOMContentLoaded<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlDOMContentLoadedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FrameDOMContentLoaded(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameDOMContentLoaded(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveFrameDOMContentLoaded(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn FrameNavigationCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNavigationCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FrameNavigationCompleted(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFrameNavigationCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveFrameNavigationCompleted(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ScriptNotify<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlScriptNotifyEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ScriptNotify(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveScriptNotify(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveScriptNotify(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LongRunningScriptDetected<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlLongRunningScriptDetectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LongRunningScriptDetected(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLongRunningScriptDetected(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveLongRunningScriptDetected(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn UnsafeContentWarningDisplaying<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.UnsafeContentWarningDisplaying(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnsafeContentWarningDisplaying(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveUnsafeContentWarningDisplaying(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn UnviewableContentIdentified<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlUnviewableContentIdentifiedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.UnviewableContentIdentified(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnviewableContentIdentified(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveUnviewableContentIdentified(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PermissionRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlPermissionRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.PermissionRequested(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePermissionRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemovePermissionRequested(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn UnsupportedUriSchemeIdentified<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.UnsupportedUriSchemeIdentified(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUnsupportedUriSchemeIdentified(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveUnsupportedUriSchemeIdentified(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn NewWindowRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlNewWindowRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.NewWindowRequested(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNewWindowRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveNewWindowRequested(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ContainsFullScreenElementChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ContainsFullScreenElementChanged(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContainsFullScreenElementChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveContainsFullScreenElementChanged(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn WebResourceRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<super::IWebViewControl, super::WebViewControlWebResourceRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.WebResourceRequested(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveWebResourceRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveWebResourceRequested(token)).ok() }
    }
    pub fn AddInitializeScript(&self, script: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IWebViewControl2>(self)?;
        unsafe { ::windows_core::vcall!(this.AddInitializeScript(::core::mem::transmute_copy(script))).ok() }
    }
    pub fn Process(&self) -> ::windows_core::Result<WebViewControlProcess> {
        let this = &::windows_core::ComInterface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Process(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetScale(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebViewControlSite>(self)?;
        unsafe { ::windows_core::vcall!(this.SetScale(value)).ok() }
    }
    pub fn Scale(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Scale(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetBounds(&self, value: super::super::super::Foundation::Rect) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebViewControlSite>(self)?;
        unsafe { ::windows_core::vcall!(this.SetBounds(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Bounds(&self) -> ::windows_core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows_core::ComInterface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Bounds(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebViewControlSite>(self)?;
        unsafe { ::windows_core::vcall!(this.SetIsVisible(value)).ok() }
    }
    pub fn IsVisible(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsVisible(&mut result__)).from_abi(result__)
        }
    }
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebViewControlSite>(self)?;
        unsafe { ::windows_core::vcall!(this.Close()).ok() }
    }
    pub fn MoveFocus(&self, reason: WebViewControlMoveFocusReason) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebViewControlSite>(self)?;
        unsafe { ::windows_core::vcall!(this.MoveFocus(reason)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MoveFocusRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<WebViewControl, WebViewControlMoveFocusRequestedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MoveFocusRequested(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMoveFocusRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebViewControlSite>(self)?;
        unsafe { ::windows_core::vcall!(this.RemoveMoveFocusRequested(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AcceleratorKeyPressed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<WebViewControl, WebViewControlAcceleratorKeyPressedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<IWebViewControlSite>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.AcceleratorKeyPressed(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAcceleratorKeyPressed(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebViewControlSite>(self)?;
        unsafe { ::windows_core::vcall!(this.RemoveAcceleratorKeyPressed(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GotFocus<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<WebViewControl, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IWebViewControlSite2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GotFocus(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGotFocus(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebViewControlSite2>(self)?;
        unsafe { ::windows_core::vcall!(this.RemoveGotFocus(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LostFocus<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<WebViewControl, ::windows_core::IInspectable>>,
    {
        let this = &::windows_core::ComInterface::cast::<IWebViewControlSite2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LostFocus(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLostFocus(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IWebViewControlSite2>(self)?;
        unsafe { ::windows_core::vcall!(this.RemoveLostFocus(token)).ok() }
    }
}
impl ::windows_core::RuntimeType for WebViewControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.UI.Interop.WebViewControl;{3f921316-bc70-4bda-9136-c94370899fab})");
}
unsafe impl ::windows_core::Interface for WebViewControl {
    type Vtable = super::IWebViewControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebViewControl {
    const IID: ::windows_core::GUID = <super::IWebViewControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControl {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControl";
}
::windows_core::imp::interface_hierarchy!(WebViewControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IWebViewControl> for WebViewControl {}
impl ::windows_core::CanTryInto<super::IWebViewControl2> for WebViewControl {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebViewControlAcceleratorKeyPressedEventArgs(::windows_core::IUnknown);
impl WebViewControlAcceleratorKeyPressedEventArgs {
    #[doc = "Required features: `\"UI_Core\"`"]
    #[cfg(feature = "UI_Core")]
    pub fn EventType(&self) -> ::windows_core::Result<super::super::super::UI::Core::CoreAcceleratorKeyEventType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.EventType(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn VirtualKey(&self) -> ::windows_core::Result<super::super::super::System::VirtualKey> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.VirtualKey(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_Core\"`"]
    #[cfg(feature = "UI_Core")]
    pub fn KeyStatus(&self) -> ::windows_core::Result<super::super::super::UI::Core::CorePhysicalKeyStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.KeyStatus(&mut result__)).from_abi(result__)
        }
    }
    pub fn RoutingStage(&self) -> ::windows_core::Result<WebViewControlAcceleratorKeyRoutingStage> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.RoutingStage(&mut result__)).from_abi(result__)
        }
    }
    pub fn Handled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Handled(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetHandled(value)).ok() }
    }
}
impl ::windows_core::RuntimeType for WebViewControlAcceleratorKeyPressedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.UI.Interop.WebViewControlAcceleratorKeyPressedEventArgs;{77a2a53e-7c74-437d-a290-3ac0d8cd5655})");
}
unsafe impl ::windows_core::Interface for WebViewControlAcceleratorKeyPressedEventArgs {
    type Vtable = IWebViewControlAcceleratorKeyPressedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebViewControlAcceleratorKeyPressedEventArgs {
    const IID: ::windows_core::GUID = <IWebViewControlAcceleratorKeyPressedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlAcceleratorKeyPressedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControlAcceleratorKeyPressedEventArgs";
}
::windows_core::imp::interface_hierarchy!(WebViewControlAcceleratorKeyPressedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebViewControlMoveFocusRequestedEventArgs(::windows_core::IUnknown);
impl WebViewControlMoveFocusRequestedEventArgs {
    pub fn Reason(&self) -> ::windows_core::Result<WebViewControlMoveFocusReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Reason(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WebViewControlMoveFocusRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.UI.Interop.WebViewControlMoveFocusRequestedEventArgs;{6b2a340d-4bd0-405e-b7c1-1e72a492f446})");
}
unsafe impl ::windows_core::Interface for WebViewControlMoveFocusRequestedEventArgs {
    type Vtable = IWebViewControlMoveFocusRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebViewControlMoveFocusRequestedEventArgs {
    const IID: ::windows_core::GUID = <IWebViewControlMoveFocusRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlMoveFocusRequestedEventArgs {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControlMoveFocusRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(WebViewControlMoveFocusRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebViewControlProcess(::windows_core::IUnknown);
impl WebViewControlProcess {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebViewControlProcess, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProcessId(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ProcessId(&mut result__)).from_abi(result__)
        }
    }
    pub fn EnterpriseId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.EnterpriseId(&mut result__)).from_abi(result__)
        }
    }
    pub fn IsPrivateNetworkClientServerCapabilityEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsPrivateNetworkClientServerCapabilityEnabled(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWebViewControlAsync(&self, hostwindowhandle: i64, bounds: super::super::super::Foundation::Rect) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<WebViewControl>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateWebViewControlAsync(hostwindowhandle, bounds, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetWebViewControls(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<WebViewControl>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetWebViewControls(&mut result__)).from_abi(result__)
        }
    }
    pub fn Terminate(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Terminate()).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ProcessExited<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<WebViewControlProcess, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ProcessExited(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProcessExited(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveProcessExited(token)).ok() }
    }
    pub fn CreateWithOptions<P0>(processoptions: P0) -> ::windows_core::Result<WebViewControlProcess>
    where
        P0: ::windows_core::IntoParam<WebViewControlProcessOptions>,
    {
        Self::IWebViewControlProcessFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateWithOptions(processoptions.into_param().abi(), &mut result__)).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebViewControlProcessFactory<R, F: FnOnce(&IWebViewControlProcessFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebViewControlProcess, IWebViewControlProcessFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for WebViewControlProcess {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.UI.Interop.WebViewControlProcess;{02c723ec-98d6-424a-b63e-c6136c36a0f2})");
}
unsafe impl ::windows_core::Interface for WebViewControlProcess {
    type Vtable = IWebViewControlProcess_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebViewControlProcess {
    const IID: ::windows_core::GUID = <IWebViewControlProcess as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlProcess {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControlProcess";
}
::windows_core::imp::interface_hierarchy!(WebViewControlProcess, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WebViewControlProcessOptions(::windows_core::IUnknown);
impl WebViewControlProcessOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<WebViewControlProcessOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetEnterpriseId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetEnterpriseId(::core::mem::transmute_copy(value))).ok() }
    }
    pub fn EnterpriseId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.EnterpriseId(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetPrivateNetworkClientServerCapability(&self, value: WebViewControlProcessCapabilityState) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetPrivateNetworkClientServerCapability(value)).ok() }
    }
    pub fn PrivateNetworkClientServerCapability(&self) -> ::windows_core::Result<WebViewControlProcessCapabilityState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.PrivateNetworkClientServerCapability(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WebViewControlProcessOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Web.UI.Interop.WebViewControlProcessOptions;{1cca72a7-3bd6-4826-8261-6c8189505d89})");
}
unsafe impl ::windows_core::Interface for WebViewControlProcessOptions {
    type Vtable = IWebViewControlProcessOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WebViewControlProcessOptions {
    const IID: ::windows_core::GUID = <IWebViewControlProcessOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WebViewControlProcessOptions {
    const NAME: &'static str = "Windows.Web.UI.Interop.WebViewControlProcessOptions";
}
::windows_core::imp::interface_hierarchy!(WebViewControlProcessOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WebViewControlAcceleratorKeyRoutingStage(pub i32);
impl WebViewControlAcceleratorKeyRoutingStage {
    pub const Tunneling: Self = Self(0i32);
    pub const Bubbling: Self = Self(1i32);
}
impl ::core::marker::Copy for WebViewControlAcceleratorKeyRoutingStage {}
impl ::core::clone::Clone for WebViewControlAcceleratorKeyRoutingStage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebViewControlAcceleratorKeyRoutingStage {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WebViewControlAcceleratorKeyRoutingStage {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WebViewControlAcceleratorKeyRoutingStage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlAcceleratorKeyRoutingStage").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebViewControlAcceleratorKeyRoutingStage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.UI.Interop.WebViewControlAcceleratorKeyRoutingStage;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WebViewControlMoveFocusReason(pub i32);
impl WebViewControlMoveFocusReason {
    pub const Programmatic: Self = Self(0i32);
    pub const Next: Self = Self(1i32);
    pub const Previous: Self = Self(2i32);
}
impl ::core::marker::Copy for WebViewControlMoveFocusReason {}
impl ::core::clone::Clone for WebViewControlMoveFocusReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebViewControlMoveFocusReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WebViewControlMoveFocusReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WebViewControlMoveFocusReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlMoveFocusReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebViewControlMoveFocusReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.UI.Interop.WebViewControlMoveFocusReason;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WebViewControlProcessCapabilityState(pub i32);
impl WebViewControlProcessCapabilityState {
    pub const Default: Self = Self(0i32);
    pub const Disabled: Self = Self(1i32);
    pub const Enabled: Self = Self(2i32);
}
impl ::core::marker::Copy for WebViewControlProcessCapabilityState {}
impl ::core::clone::Clone for WebViewControlProcessCapabilityState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebViewControlProcessCapabilityState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WebViewControlProcessCapabilityState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WebViewControlProcessCapabilityState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebViewControlProcessCapabilityState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WebViewControlProcessCapabilityState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Web.UI.Interop.WebViewControlProcessCapabilityState;i4)");
}
