#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICastingConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICastingConnection {
    type Vtable = ICastingConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICastingConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd951653_c2f1_4498_8b78_5fb4cd3640dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingConnection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CastingConnectionState) -> ::windows_core::HRESULT,
    pub Device: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStartCastingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStartCastingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisconnectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisconnectAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICastingConnectionErrorOccurredEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICastingConnectionErrorOccurredEventArgs {
    type Vtable = ICastingConnectionErrorOccurredEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICastingConnectionErrorOccurredEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa7fb3c69_8719_4f00_81fb_961863c79a32);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingConnectionErrorOccurredEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ErrorStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CastingConnectionErrorStatus) -> ::windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICastingDevice(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICastingDevice {
    type Vtable = ICastingDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICastingDevice {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xde721c83_4a43_4ad1_a6d2_2492a796c3f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDevice_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Icon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Icon: usize,
    #[cfg(feature = "Foundation")]
    pub GetSupportedCastingPlaybackTypesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSupportedCastingPlaybackTypesAsync: usize,
    pub CreateCastingConnection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICastingDevicePicker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICastingDevicePicker {
    type Vtable = ICastingDevicePicker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICastingDevicePicker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdcd39924_0591_49be_aacb_4b82ee756a95);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDevicePicker_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Filter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Devices_Enumeration")]
    pub Appearance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Devices_Enumeration"))]
    Appearance: usize,
    #[cfg(feature = "Foundation")]
    pub CastingDeviceSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CastingDeviceSelected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCastingDeviceSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCastingDeviceSelected: usize,
    #[cfg(feature = "Foundation")]
    pub CastingDevicePickerDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CastingDevicePickerDismissed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCastingDevicePickerDismissed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCastingDevicePickerDismissed: usize,
    #[cfg(feature = "Foundation")]
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Show: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowWithPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowWithPlacement: usize,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICastingDevicePickerFilter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICastingDevicePickerFilter {
    type Vtable = ICastingDevicePickerFilter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICastingDevicePickerFilter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbe8c619c_b563_4354_ae33_9fdaad8c6291);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDevicePickerFilter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SupportsAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSupportsAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SupportsVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSupportsVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SupportsPictures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetSupportsPictures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCastingSources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCastingSources: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICastingDeviceSelectedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICastingDeviceSelectedEventArgs {
    type Vtable = ICastingDeviceSelectedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICastingDeviceSelectedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc439e86_dd57_4d0d_9400_af45e4fb3663);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDeviceSelectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SelectedCastingDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICastingDeviceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICastingDeviceStatics {
    type Vtable = ICastingDeviceStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICastingDeviceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7d958d7_4d13_4237_a365_4c4f6a4cfd2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingDeviceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: CastingPlaybackTypes, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeviceSelectorFromCastingSourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, castingsource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeviceSelectorFromCastingSourceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub DeviceInfoSupportsCastingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Devices_Enumeration", feature = "Foundation")))]
    DeviceInfoSupportsCastingAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICastingSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICastingSource {
    type Vtable = ICastingSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICastingSource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf429ea72_3467_47e6_a027_522923e9d727);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PreferredSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PreferredSourceUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetPreferredSourceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredSourceUri: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CastingConnection(::windows_core::IUnknown);
impl CastingConnection {
    pub fn State(&self) -> ::windows_core::Result<CastingConnectionState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.State(&mut result__)).from_abi(result__)
        }
    }
    pub fn Device(&self) -> ::windows_core::Result<CastingDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Device(&mut result__)).from_abi(result__)
        }
    }
    pub fn Source(&self) -> ::windows_core::Result<CastingSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Source(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetSource<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<CastingSource>,
    {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetSource(value.into_param().abi())).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<CastingConnection, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.StateChanged(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveStateChanged(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorOccurred<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<CastingConnection, CastingConnectionErrorOccurredEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ErrorOccurred(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveErrorOccurred(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveErrorOccurred(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStartCastingAsync<P0>(&self, value: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>>
    where
        P0: ::windows_core::IntoParam<CastingSource>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.RequestStartCastingAsync(value.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DisconnectAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<CastingConnectionErrorStatus>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DisconnectAsync(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { ::windows_core::vcall!(this.Close()).ok() }
    }
}
impl ::windows_core::RuntimeType for CastingConnection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingConnection;{cd951653-c2f1-4498-8b78-5fb4cd3640dd})");
}
unsafe impl ::windows_core::Interface for CastingConnection {
    type Vtable = ICastingConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CastingConnection {
    const IID: ::windows_core::GUID = <ICastingConnection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CastingConnection {
    const NAME: &'static str = "Windows.Media.Casting.CastingConnection";
}
::windows_core::imp::interface_hierarchy!(CastingConnection, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for CastingConnection {}
unsafe impl ::core::marker::Send for CastingConnection {}
unsafe impl ::core::marker::Sync for CastingConnection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CastingConnectionErrorOccurredEventArgs(::windows_core::IUnknown);
impl CastingConnectionErrorOccurredEventArgs {
    pub fn ErrorStatus(&self) -> ::windows_core::Result<CastingConnectionErrorStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ErrorStatus(&mut result__)).from_abi(result__)
        }
    }
    pub fn Message(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Message(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CastingConnectionErrorOccurredEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingConnectionErrorOccurredEventArgs;{a7fb3c69-8719-4f00-81fb-961863c79a32})");
}
unsafe impl ::windows_core::Interface for CastingConnectionErrorOccurredEventArgs {
    type Vtable = ICastingConnectionErrorOccurredEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CastingConnectionErrorOccurredEventArgs {
    const IID: ::windows_core::GUID = <ICastingConnectionErrorOccurredEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CastingConnectionErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Media.Casting.CastingConnectionErrorOccurredEventArgs";
}
::windows_core::imp::interface_hierarchy!(CastingConnectionErrorOccurredEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CastingConnectionErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for CastingConnectionErrorOccurredEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CastingDevice(::windows_core::IUnknown);
impl CastingDevice {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Id(&mut result__)).from_abi(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FriendlyName(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Icon(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Icon(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetSupportedCastingPlaybackTypesAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<CastingPlaybackTypes>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetSupportedCastingPlaybackTypesAsync(&mut result__)).from_abi(result__)
        }
    }
    pub fn CreateCastingConnection(&self) -> ::windows_core::Result<CastingConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateCastingConnection(&mut result__)).from_abi(result__)
        }
    }
    pub fn GetDeviceSelector(r#type: CastingPlaybackTypes) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICastingDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetDeviceSelector(r#type, &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeviceSelectorFromCastingSourceAsync<P0>(castingsource: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<CastingSource>,
    {
        Self::ICastingDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetDeviceSelectorFromCastingSourceAsync(castingsource.into_param().abi(), &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(value: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<CastingDevice>> {
        Self::ICastingDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FromIdAsync(::core::mem::transmute_copy(value), &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Devices_Enumeration\"`, `\"Foundation\"`"]
    #[cfg(all(feature = "Devices_Enumeration", feature = "Foundation"))]
    pub fn DeviceInfoSupportsCastingAsync<P0>(device: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<super::super::Devices::Enumeration::DeviceInformation>,
    {
        Self::ICastingDeviceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DeviceInfoSupportsCastingAsync(device.into_param().abi(), &mut result__)).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICastingDeviceStatics<R, F: FnOnce(&ICastingDeviceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CastingDevice, ICastingDeviceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for CastingDevice {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingDevice;{de721c83-4a43-4ad1-a6d2-2492a796c3f2})");
}
unsafe impl ::windows_core::Interface for CastingDevice {
    type Vtable = ICastingDevice_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CastingDevice {
    const IID: ::windows_core::GUID = <ICastingDevice as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CastingDevice {
    const NAME: &'static str = "Windows.Media.Casting.CastingDevice";
}
::windows_core::imp::interface_hierarchy!(CastingDevice, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CastingDevice {}
unsafe impl ::core::marker::Sync for CastingDevice {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CastingDevicePicker(::windows_core::IUnknown);
impl CastingDevicePicker {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CastingDevicePicker, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Filter(&self) -> ::windows_core::Result<CastingDevicePickerFilter> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Filter(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Devices_Enumeration\"`"]
    #[cfg(feature = "Devices_Enumeration")]
    pub fn Appearance(&self) -> ::windows_core::Result<super::super::Devices::Enumeration::DevicePickerAppearance> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Appearance(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CastingDeviceSelected<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<CastingDevicePicker, CastingDeviceSelectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CastingDeviceSelected(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCastingDeviceSelected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveCastingDeviceSelected(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CastingDevicePickerDismissed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<CastingDevicePicker, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CastingDevicePickerDismissed(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCastingDevicePickerDismissed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveCastingDevicePickerDismissed(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Show(&self, selection: super::super::Foundation::Rect) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Show(selection)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"UI_Popups\"`"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowWithPlacement(&self, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.ShowWithPlacement(selection, preferredplacement)).ok() }
    }
    pub fn Hide(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Hide()).ok() }
    }
}
impl ::windows_core::RuntimeType for CastingDevicePicker {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingDevicePicker;{dcd39924-0591-49be-aacb-4b82ee756a95})");
}
unsafe impl ::windows_core::Interface for CastingDevicePicker {
    type Vtable = ICastingDevicePicker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CastingDevicePicker {
    const IID: ::windows_core::GUID = <ICastingDevicePicker as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CastingDevicePicker {
    const NAME: &'static str = "Windows.Media.Casting.CastingDevicePicker";
}
::windows_core::imp::interface_hierarchy!(CastingDevicePicker, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CastingDevicePicker {}
unsafe impl ::core::marker::Sync for CastingDevicePicker {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CastingDevicePickerFilter(::windows_core::IUnknown);
impl CastingDevicePickerFilter {
    pub fn SupportsAudio(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SupportsAudio(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetSupportsAudio(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetSupportsAudio(value)).ok() }
    }
    pub fn SupportsVideo(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SupportsVideo(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetSupportsVideo(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetSupportsVideo(value)).ok() }
    }
    pub fn SupportsPictures(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SupportsPictures(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetSupportsPictures(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetSupportsPictures(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCastingSources(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<CastingSource>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SupportedCastingSources(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CastingDevicePickerFilter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingDevicePickerFilter;{be8c619c-b563-4354-ae33-9fdaad8c6291})");
}
unsafe impl ::windows_core::Interface for CastingDevicePickerFilter {
    type Vtable = ICastingDevicePickerFilter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CastingDevicePickerFilter {
    const IID: ::windows_core::GUID = <ICastingDevicePickerFilter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CastingDevicePickerFilter {
    const NAME: &'static str = "Windows.Media.Casting.CastingDevicePickerFilter";
}
::windows_core::imp::interface_hierarchy!(CastingDevicePickerFilter, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CastingDevicePickerFilter {}
unsafe impl ::core::marker::Sync for CastingDevicePickerFilter {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CastingDeviceSelectedEventArgs(::windows_core::IUnknown);
impl CastingDeviceSelectedEventArgs {
    pub fn SelectedCastingDevice(&self) -> ::windows_core::Result<CastingDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SelectedCastingDevice(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for CastingDeviceSelectedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingDeviceSelectedEventArgs;{dc439e86-dd57-4d0d-9400-af45e4fb3663})");
}
unsafe impl ::windows_core::Interface for CastingDeviceSelectedEventArgs {
    type Vtable = ICastingDeviceSelectedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CastingDeviceSelectedEventArgs {
    const IID: ::windows_core::GUID = <ICastingDeviceSelectedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CastingDeviceSelectedEventArgs {
    const NAME: &'static str = "Windows.Media.Casting.CastingDeviceSelectedEventArgs";
}
::windows_core::imp::interface_hierarchy!(CastingDeviceSelectedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CastingDeviceSelectedEventArgs {}
unsafe impl ::core::marker::Sync for CastingDeviceSelectedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CastingSource(::windows_core::IUnknown);
impl CastingSource {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PreferredSourceUri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.PreferredSourceUri(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetPreferredSourceUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetPreferredSourceUri(value.into_param().abi())).ok() }
    }
}
impl ::windows_core::RuntimeType for CastingSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Casting.CastingSource;{f429ea72-3467-47e6-a027-522923e9d727})");
}
unsafe impl ::windows_core::Interface for CastingSource {
    type Vtable = ICastingSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CastingSource {
    const IID: ::windows_core::GUID = <ICastingSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CastingSource {
    const NAME: &'static str = "Windows.Media.Casting.CastingSource";
}
::windows_core::imp::interface_hierarchy!(CastingSource, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CastingSource {}
unsafe impl ::core::marker::Sync for CastingSource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CastingConnectionErrorStatus(pub i32);
impl CastingConnectionErrorStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const DeviceDidNotRespond: Self = Self(1i32);
    pub const DeviceError: Self = Self(2i32);
    pub const DeviceLocked: Self = Self(3i32);
    pub const ProtectedPlaybackFailed: Self = Self(4i32);
    pub const InvalidCastingSource: Self = Self(5i32);
    pub const Unknown: Self = Self(6i32);
}
impl ::core::marker::Copy for CastingConnectionErrorStatus {}
impl ::core::clone::Clone for CastingConnectionErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CastingConnectionErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CastingConnectionErrorStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CastingConnectionErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingConnectionErrorStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CastingConnectionErrorStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Casting.CastingConnectionErrorStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CastingConnectionState(pub i32);
impl CastingConnectionState {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Rendering: Self = Self(2i32);
    pub const Disconnecting: Self = Self(3i32);
    pub const Connecting: Self = Self(4i32);
}
impl ::core::marker::Copy for CastingConnectionState {}
impl ::core::clone::Clone for CastingConnectionState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CastingConnectionState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CastingConnectionState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CastingConnectionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingConnectionState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CastingConnectionState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Casting.CastingConnectionState;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CastingPlaybackTypes(pub u32);
impl CastingPlaybackTypes {
    pub const None: Self = Self(0u32);
    pub const Audio: Self = Self(1u32);
    pub const Video: Self = Self(2u32);
    pub const Picture: Self = Self(4u32);
}
impl ::core::marker::Copy for CastingPlaybackTypes {}
impl ::core::clone::Clone for CastingPlaybackTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CastingPlaybackTypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CastingPlaybackTypes {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CastingPlaybackTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CastingPlaybackTypes").field(&self.0).finish()
    }
}
impl CastingPlaybackTypes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CastingPlaybackTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CastingPlaybackTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CastingPlaybackTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CastingPlaybackTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CastingPlaybackTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for CastingPlaybackTypes {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Casting.CastingPlaybackTypes;u4)");
}
