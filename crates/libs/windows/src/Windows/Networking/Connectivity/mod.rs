#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAttributedNetworkUsage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAttributedNetworkUsage {
    type Vtable = IAttributedNetworkUsage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAttributedNetworkUsage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf769b039_eca2_45eb_ade1_b0368b756c49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAttributedNetworkUsage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BytesSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub AttributionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AttributionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AttributionThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AttributionThumbnail: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICellularApnContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICellularApnContext {
    type Vtable = ICellularApnContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICellularApnContext {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6fa529f4_effd_4542_9ab2_705bbf94943a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICellularApnContext_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AccessPointName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAccessPointName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetUserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Password: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsCompressionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCompressionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AuthenticationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CellularApnAuthenticationType) -> ::windows_core::HRESULT,
    pub SetAuthenticationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: CellularApnAuthenticationType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICellularApnContext2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICellularApnContext2 {
    type Vtable = ICellularApnContext2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICellularApnContext2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x76b0eb1a_ac49_4350_b1e5_dc4763bc69c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICellularApnContext2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProfileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetProfileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IConnectionCost(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionCost {
    type Vtable = IConnectionCost_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IConnectionCost {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbad7d829_3416_4b10_a202_bac0b075bdae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionCost_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NetworkCostType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkCostType) -> ::windows_core::HRESULT,
    pub Roaming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub OverDataLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ApproachingDataLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IConnectionCost2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionCost2 {
    type Vtable = IConnectionCost2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IConnectionCost2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e113a05_e209_4549_bb25_5e0db691cb05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionCost2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BackgroundDataUsageRestricted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IConnectionProfile(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionProfile {
    type Vtable = IConnectionProfile_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IConnectionProfile {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71ba143c_598e_49d0_84eb_8febaedcc195);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfile_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ProfileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetNetworkConnectivityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkConnectivityLevel) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNetworkNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNetworkNames: usize,
    pub GetConnectionCost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDataPlanStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NetworkAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetLocalUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetLocalUsage: usize,
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub GetLocalUsagePerRoamingStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: RoamingStates, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "deprecated")))]
    GetLocalUsagePerRoamingStates: usize,
    pub NetworkSecuritySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IConnectionProfile2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionProfile2 {
    type Vtable = IConnectionProfile2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IConnectionProfile2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe2045145_4c9f_400c_9150_7ec7d6e2888a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfile2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsWwanConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsWlanConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub WwanConnectionProfileDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub WlanConnectionProfileDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ServiceProviderGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceProviderGuid: usize,
    #[cfg(feature = "Foundation")]
    pub GetSignalBars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSignalBars: usize,
    pub GetDomainConnectivityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DomainConnectivityLevel) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNetworkUsageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, granularity: DataUsageGranularity, states: NetworkUsageStates, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNetworkUsageAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConnectivityIntervalsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConnectivityIntervalsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IConnectionProfile3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionProfile3 {
    type Vtable = IConnectionProfile3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IConnectionProfile3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x578c2528_4cd9_4161_8045_201cfd5b115c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfile3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAttributedNetworkUsageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAttributedNetworkUsageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IConnectionProfile4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionProfile4 {
    type Vtable = IConnectionProfile4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IConnectionProfile4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a2d42cd_81e0_4ae6_abed_ab9ca13eb714);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfile4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetProviderNetworkUsageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetProviderNetworkUsageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IConnectionProfile5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionProfile5 {
    type Vtable = IConnectionProfile5_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IConnectionProfile5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85361ec7_9c73_4be0_8f14_578eec71ee0e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfile5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CanDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryDeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDeleteAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IConnectionProfile6(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionProfile6 {
    type Vtable = IConnectionProfile6_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IConnectionProfile6 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdc27dfe2_7a6f_5d0e_9589_2fe2e5b6f9aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfile6_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsDomainAuthenticatedBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: DomainAuthenticationKind, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IConnectionProfileFilter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionProfileFilter {
    type Vtable = IConnectionProfileFilter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IConnectionProfileFilter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x204c7cc8_bd2d_4e8d_a4b3_455ec337388a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfileFilter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetIsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsWwanConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsWwanConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsWlanConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsWlanConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetNetworkCostType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: NetworkCostType) -> ::windows_core::HRESULT,
    pub NetworkCostType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkCostType) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetServiceProviderGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetServiceProviderGuid: usize,
    #[cfg(feature = "Foundation")]
    pub ServiceProviderGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ServiceProviderGuid: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IConnectionProfileFilter2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionProfileFilter2 {
    type Vtable = IConnectionProfileFilter2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IConnectionProfileFilter2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd068ee1_c3fc_4fad_9ddc_593faa4b7885);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfileFilter2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetIsRoaming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIsRoaming: usize,
    #[cfg(feature = "Foundation")]
    pub IsRoaming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsRoaming: usize,
    #[cfg(feature = "Foundation")]
    pub SetIsOverDataLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIsOverDataLimit: usize,
    #[cfg(feature = "Foundation")]
    pub IsOverDataLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsOverDataLimit: usize,
    #[cfg(feature = "Foundation")]
    pub SetIsBackgroundDataUsageRestricted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetIsBackgroundDataUsageRestricted: usize,
    #[cfg(feature = "Foundation")]
    pub IsBackgroundDataUsageRestricted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsBackgroundDataUsageRestricted: usize,
    #[cfg(feature = "Storage_Streams")]
    pub RawData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RawData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IConnectionProfileFilter3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionProfileFilter3 {
    type Vtable = IConnectionProfileFilter3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IConnectionProfileFilter3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0aaa09c0_5014_447c_8809_aee4cb0af94a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionProfileFilter3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetPurposeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPurposeGuid: usize,
    #[cfg(feature = "Foundation")]
    pub PurposeGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PurposeGuid: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IConnectionSession(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectionSession {
    type Vtable = IConnectionSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IConnectionSession {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xff905d4c_f83b_41b0_8a0c_1462d9c56b73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectionSession_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IConnectivityInterval(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectivityInterval {
    type Vtable = IConnectivityInterval_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IConnectivityInterval {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4faa3fff_6746_4824_a964_eed8e87f8709);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectivityInterval_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub ConnectionDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IConnectivityManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IConnectivityManagerStatics {
    type Vtable = IConnectivityManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IConnectivityManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5120d4b1_4fb1_48b0_afc9_42e0092a8164);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnectivityManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AcquireConnectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cellularapncontext: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AcquireConnectionAsync: usize,
    pub AddHttpRoutePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, routepolicy: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveHttpRoutePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, routepolicy: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDataPlanStatus(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataPlanStatus {
    type Vtable = IDataPlanStatus_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDataPlanStatus {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x977a8b8c_3885_40f3_8851_42cd2bd568bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPlanStatus_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DataPlanUsage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DataLimitInMegabytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataLimitInMegabytes: usize,
    #[cfg(feature = "Foundation")]
    pub InboundBitsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InboundBitsPerSecond: usize,
    #[cfg(feature = "Foundation")]
    pub OutboundBitsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OutboundBitsPerSecond: usize,
    #[cfg(feature = "Foundation")]
    pub NextBillingCycle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NextBillingCycle: usize,
    #[cfg(feature = "Foundation")]
    pub MaxTransferSizeInMegabytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxTransferSizeInMegabytes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDataPlanUsage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataPlanUsage {
    type Vtable = IDataPlanUsage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDataPlanUsage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb921492d_3b44_47ff_b361_be59e69ed1b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataPlanUsage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MegabytesUsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSyncTime: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDataUsage(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IDataUsage {
    type Vtable = IDataUsage_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IDataUsage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1431dd3_b146_4d39_b959_0c69b096c512);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IDataUsage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub BytesSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BytesSent: usize,
    #[cfg(feature = "deprecated")]
    pub BytesReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    BytesReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IIPInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IIPInformation {
    type Vtable = IIPInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IIPInformation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd85145e0_138f_47d7_9b3a_36bb488cef33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIPInformation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NetworkAdapter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PrefixLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrefixLength: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILanIdentifier(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILanIdentifier {
    type Vtable = ILanIdentifier_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILanIdentifier {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48aa53aa_1108_4546_a6cb_9a74da4b7ba0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanIdentifier_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub InfrastructureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PortId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NetworkAdapterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ILanIdentifierData(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILanIdentifierData {
    type Vtable = ILanIdentifierData_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ILanIdentifierData {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa74e83c3_d639_45be_a36a_c4e4aeaf6d9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanIdentifierData_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Value: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkAdapter(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkAdapter {
    type Vtable = INetworkAdapter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkAdapter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b542e03_5388_496c_a8a3_affd39aec2e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkAdapter_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OutboundMaxBitsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub InboundMaxBitsPerSecond: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub IanaInterfaceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub NetworkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NetworkAdapterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetConnectedProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConnectedProfileAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkInformationStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkInformationStatics {
    type Vtable = INetworkInformationStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkInformationStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5074f851_950d_4165_9c15_365619481eea);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkInformationStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConnectionProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConnectionProfiles: usize,
    pub GetInternetConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetLanIdentifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetLanIdentifiers: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetHostNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetHostNames: usize,
    #[cfg(feature = "Foundation")]
    pub GetProxyConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetProxyConfigurationAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSortedEndpointPairs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, destinationlist: *mut ::core::ffi::c_void, sortoptions: super::HostNameSortOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSortedEndpointPairs: usize,
    #[cfg(feature = "Foundation")]
    pub NetworkStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networkstatushandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NetworkStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNetworkStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNetworkStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkInformationStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkInformationStatics2 {
    type Vtable = INetworkInformationStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkInformationStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x459ced14_2832_49b6_ba6e_e265f04786a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkInformationStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindConnectionProfilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprofilefilter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindConnectionProfilesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkItem(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkItem {
    type Vtable = INetworkItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkItem {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x01bc4d39_f5e0_4567_a28c_42080c831b2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkItem_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NetworkId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetNetworkTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkTypes) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkSecuritySettings(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkSecuritySettings {
    type Vtable = INetworkSecuritySettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkSecuritySettings {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ca07e8d_917b_4b5f_b84d_28f7a5ac5402);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkSecuritySettings_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub NetworkAuthenticationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkAuthenticationType) -> ::windows_core::HRESULT,
    pub NetworkEncryptionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut NetworkEncryptionType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkStateChangeEventDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkStateChangeEventDetails {
    type Vtable = INetworkStateChangeEventDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkStateChangeEventDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1f0cf333_d7a6_44dd_a4e9_687c476b903d);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkStateChangeEventDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HasNewInternetConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasNewConnectionCost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasNewNetworkConnectivityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasNewDomainConnectivityLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasNewHostNameList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasNewWwanRegistrationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkStateChangeEventDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkStateChangeEventDetails2 {
    type Vtable = INetworkStateChangeEventDetails2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkStateChangeEventDetails2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd643c0e8_30d3_4f6a_ad47_6a1873ceb3c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkStateChangeEventDetails2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HasNewTetheringOperationalState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub HasNewTetheringClientCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INetworkUsage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INetworkUsage {
    type Vtable = INetworkUsage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INetworkUsage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49da8fce_9985_4927_bf5b_072b5c65f8d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetworkUsage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BytesSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ConnectionDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ConnectionDuration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProviderNetworkUsage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProviderNetworkUsage {
    type Vtable = IProviderNetworkUsage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProviderNetworkUsage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5ec69e04_7931_48c8_b8f3_46300fa42728);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderNetworkUsage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BytesSent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub BytesReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProxyConfiguration(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProxyConfiguration {
    type Vtable = IProxyConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProxyConfiguration {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef3a60b4_9004_4dd6_b7d8_b3e502f4aad0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProxyConfiguration_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ProxyUris: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProxyUris: usize,
    pub CanConnectDirectly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRoutePolicy(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRoutePolicy {
    type Vtable = IRoutePolicy_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRoutePolicy {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11abc4ac_0fc7_42e4_8742_569923b1ca11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoutePolicy_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ConnectionProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub HostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub HostNameType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::DomainNameType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRoutePolicyFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRoutePolicyFactory {
    type Vtable = IRoutePolicyFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRoutePolicyFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x36027933_a18e_4db5_a697_f58fa7364e44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoutePolicyFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateRoutePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionprofile: *mut ::core::ffi::c_void, hostname: *mut ::core::ffi::c_void, r#type: super::DomainNameType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWlanConnectionProfileDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWlanConnectionProfileDetails {
    type Vtable = IWlanConnectionProfileDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWlanConnectionProfileDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x562098cb_b35a_4bf1_a884_b7557e88ff86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWlanConnectionProfileDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetConnectedSsid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWwanConnectionProfileDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWwanConnectionProfileDetails {
    type Vtable = IWwanConnectionProfileDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWwanConnectionProfileDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e4da8fe_835f_4df3_82fd_df556ebc09ef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwanConnectionProfileDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HomeProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AccessPointName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetNetworkRegistrationState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WwanNetworkRegistrationState) -> ::windows_core::HRESULT,
    pub GetCurrentDataClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WwanDataClass) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IWwanConnectionProfileDetails2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IWwanConnectionProfileDetails2 {
    type Vtable = IWwanConnectionProfileDetails2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IWwanConnectionProfileDetails2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a754ede_a1ed_48b2_8e92_b460033d52e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwanConnectionProfileDetails2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IPKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WwanNetworkIPKind) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub PurposeGuids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PurposeGuids: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AttributedNetworkUsage(::windows_core::IUnknown);
impl AttributedNetworkUsage {
    pub fn BytesSent(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BytesSent(&mut result__)).from_abi(result__)
        }
    }
    pub fn BytesReceived(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BytesReceived(&mut result__)).from_abi(result__)
        }
    }
    pub fn AttributionId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.AttributionId(&mut result__)).from_abi(result__)
        }
    }
    pub fn AttributionName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.AttributionName(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AttributionThumbnail(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.AttributionThumbnail(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AttributedNetworkUsage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.AttributedNetworkUsage;{f769b039-eca2-45eb-ade1-b0368b756c49})");
}
unsafe impl ::windows_core::Interface for AttributedNetworkUsage {
    type Vtable = IAttributedNetworkUsage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AttributedNetworkUsage {
    const IID: ::windows_core::GUID = <IAttributedNetworkUsage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AttributedNetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.AttributedNetworkUsage";
}
::windows_core::imp::interface_hierarchy!(AttributedNetworkUsage, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AttributedNetworkUsage {}
unsafe impl ::core::marker::Sync for AttributedNetworkUsage {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct CellularApnContext(::windows_core::IUnknown);
impl CellularApnContext {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CellularApnContext, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ProviderId(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetProviderId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetProviderId(::core::mem::transmute_copy(value))).ok() }
    }
    pub fn AccessPointName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.AccessPointName(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetAccessPointName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetAccessPointName(::core::mem::transmute_copy(value))).ok() }
    }
    pub fn UserName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.UserName(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetUserName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetUserName(::core::mem::transmute_copy(value))).ok() }
    }
    pub fn Password(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Password(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetPassword(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetPassword(::core::mem::transmute_copy(value))).ok() }
    }
    pub fn IsCompressionEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsCompressionEnabled(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetIsCompressionEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetIsCompressionEnabled(value)).ok() }
    }
    pub fn AuthenticationType(&self) -> ::windows_core::Result<CellularApnAuthenticationType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.AuthenticationType(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetAuthenticationType(&self, value: CellularApnAuthenticationType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetAuthenticationType(value)).ok() }
    }
    pub fn ProfileName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ICellularApnContext2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ProfileName(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetProfileName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ICellularApnContext2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetProfileName(::core::mem::transmute_copy(value))).ok() }
    }
}
impl ::windows_core::RuntimeType for CellularApnContext {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.CellularApnContext;{6fa529f4-effd-4542-9ab2-705bbf94943a})");
}
unsafe impl ::windows_core::Interface for CellularApnContext {
    type Vtable = ICellularApnContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CellularApnContext {
    const IID: ::windows_core::GUID = <ICellularApnContext as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CellularApnContext {
    const NAME: &'static str = "Windows.Networking.Connectivity.CellularApnContext";
}
::windows_core::imp::interface_hierarchy!(CellularApnContext, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CellularApnContext {}
unsafe impl ::core::marker::Sync for CellularApnContext {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ConnectionCost(::windows_core::IUnknown);
impl ConnectionCost {
    pub fn NetworkCostType(&self) -> ::windows_core::Result<NetworkCostType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.NetworkCostType(&mut result__)).from_abi(result__)
        }
    }
    pub fn Roaming(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Roaming(&mut result__)).from_abi(result__)
        }
    }
    pub fn OverDataLimit(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OverDataLimit(&mut result__)).from_abi(result__)
        }
    }
    pub fn ApproachingDataLimit(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ApproachingDataLimit(&mut result__)).from_abi(result__)
        }
    }
    pub fn BackgroundDataUsageRestricted(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IConnectionCost2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BackgroundDataUsageRestricted(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ConnectionCost {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ConnectionCost;{bad7d829-3416-4b10-a202-bac0b075bdae})");
}
unsafe impl ::windows_core::Interface for ConnectionCost {
    type Vtable = IConnectionCost_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ConnectionCost {
    const IID: ::windows_core::GUID = <IConnectionCost as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ConnectionCost {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectionCost";
}
::windows_core::imp::interface_hierarchy!(ConnectionCost, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ConnectionCost {}
unsafe impl ::core::marker::Sync for ConnectionCost {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ConnectionProfile(::windows_core::IUnknown);
impl ConnectionProfile {
    pub fn ProfileName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ProfileName(&mut result__)).from_abi(result__)
        }
    }
    pub fn GetNetworkConnectivityLevel(&self) -> ::windows_core::Result<NetworkConnectivityLevel> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetNetworkConnectivityLevel(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNetworkNames(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetNetworkNames(&mut result__)).from_abi(result__)
        }
    }
    pub fn GetConnectionCost(&self) -> ::windows_core::Result<ConnectionCost> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetConnectionCost(&mut result__)).from_abi(result__)
        }
    }
    pub fn GetDataPlanStatus(&self) -> ::windows_core::Result<DataPlanStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetDataPlanStatus(&mut result__)).from_abi(result__)
        }
    }
    pub fn NetworkAdapter(&self) -> ::windows_core::Result<NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.NetworkAdapter(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetLocalUsage(&self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime) -> ::windows_core::Result<DataUsage> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetLocalUsage(starttime, endtime, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "deprecated"))]
    pub fn GetLocalUsagePerRoamingStates(&self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: RoamingStates) -> ::windows_core::Result<DataUsage> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetLocalUsagePerRoamingStates(starttime, endtime, states, &mut result__)).from_abi(result__)
        }
    }
    pub fn NetworkSecuritySettings(&self) -> ::windows_core::Result<NetworkSecuritySettings> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.NetworkSecuritySettings(&mut result__)).from_abi(result__)
        }
    }
    pub fn IsWwanConnectionProfile(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsWwanConnectionProfile(&mut result__)).from_abi(result__)
        }
    }
    pub fn IsWlanConnectionProfile(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsWlanConnectionProfile(&mut result__)).from_abi(result__)
        }
    }
    pub fn WwanConnectionProfileDetails(&self) -> ::windows_core::Result<WwanConnectionProfileDetails> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.WwanConnectionProfileDetails(&mut result__)).from_abi(result__)
        }
    }
    pub fn WlanConnectionProfileDetails(&self) -> ::windows_core::Result<WlanConnectionProfileDetails> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.WlanConnectionProfileDetails(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ServiceProviderGuid(&self) -> ::windows_core::Result<super::super::Foundation::IReference<::windows_core::GUID>> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ServiceProviderGuid(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetSignalBars(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u8>> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetSignalBars(&mut result__)).from_abi(result__)
        }
    }
    pub fn GetDomainConnectivityLevel(&self) -> ::windows_core::Result<DomainConnectivityLevel> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetDomainConnectivityLevel(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNetworkUsageAsync(&self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, granularity: DataUsageGranularity, states: NetworkUsageStates) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<NetworkUsage>>> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetNetworkUsageAsync(starttime, endtime, granularity, states, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConnectivityIntervalsAsync(&self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectivityInterval>>> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetConnectivityIntervalsAsync(starttime, endtime, states, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAttributedNetworkUsageAsync(&self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AttributedNetworkUsage>>> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfile3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetAttributedNetworkUsageAsync(starttime, endtime, states, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetProviderNetworkUsageAsync(&self, starttime: super::super::Foundation::DateTime, endtime: super::super::Foundation::DateTime, states: NetworkUsageStates) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ProviderNetworkUsage>>> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfile4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetProviderNetworkUsageAsync(starttime, endtime, states, &mut result__)).from_abi(result__)
        }
    }
    pub fn CanDelete(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfile5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CanDelete(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryDeleteAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ConnectionProfileDeleteStatus>> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfile5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.TryDeleteAsync(&mut result__)).from_abi(result__)
        }
    }
    pub fn IsDomainAuthenticatedBy(&self, kind: DomainAuthenticationKind) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfile6>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsDomainAuthenticatedBy(kind, &mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ConnectionProfile {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ConnectionProfile;{71ba143c-598e-49d0-84eb-8febaedcc195})");
}
unsafe impl ::windows_core::Interface for ConnectionProfile {
    type Vtable = IConnectionProfile_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ConnectionProfile {
    const IID: ::windows_core::GUID = <IConnectionProfile as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ConnectionProfile {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectionProfile";
}
::windows_core::imp::interface_hierarchy!(ConnectionProfile, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ConnectionProfile {}
unsafe impl ::core::marker::Sync for ConnectionProfile {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ConnectionProfileFilter(::windows_core::IUnknown);
impl ConnectionProfileFilter {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ConnectionProfileFilter, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetIsConnected(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetIsConnected(value)).ok() }
    }
    pub fn IsConnected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsConnected(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetIsWwanConnectionProfile(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetIsWwanConnectionProfile(value)).ok() }
    }
    pub fn IsWwanConnectionProfile(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsWwanConnectionProfile(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetIsWlanConnectionProfile(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetIsWlanConnectionProfile(value)).ok() }
    }
    pub fn IsWlanConnectionProfile(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsWlanConnectionProfile(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetNetworkCostType(&self, value: NetworkCostType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetNetworkCostType(value)).ok() }
    }
    pub fn NetworkCostType(&self) -> ::windows_core::Result<NetworkCostType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.NetworkCostType(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetServiceProviderGuid<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<::windows_core::GUID>>,
    {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetServiceProviderGuid(value.try_into_param()?.abi())).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ServiceProviderGuid(&self) -> ::windows_core::Result<super::super::Foundation::IReference<::windows_core::GUID>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ServiceProviderGuid(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetIsRoaming<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<bool>>,
    {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetIsRoaming(value.try_into_param()?.abi())).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn IsRoaming(&self) -> ::windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsRoaming(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetIsOverDataLimit<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<bool>>,
    {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetIsOverDataLimit(value.try_into_param()?.abi())).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn IsOverDataLimit(&self) -> ::windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsOverDataLimit(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetIsBackgroundDataUsageRestricted<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<bool>>,
    {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetIsBackgroundDataUsageRestricted(value.try_into_param()?.abi())).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn IsBackgroundDataUsageRestricted(&self) -> ::windows_core::Result<super::super::Foundation::IReference<bool>> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsBackgroundDataUsageRestricted(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RawData(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfileFilter2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.RawData(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetPurposeGuid<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<::windows_core::GUID>>,
    {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfileFilter3>(self)?;
        unsafe { ::windows_core::vcall!(this.SetPurposeGuid(value.try_into_param()?.abi())).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PurposeGuid(&self) -> ::windows_core::Result<super::super::Foundation::IReference<::windows_core::GUID>> {
        let this = &::windows_core::ComInterface::cast::<IConnectionProfileFilter3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.PurposeGuid(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ConnectionProfileFilter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ConnectionProfileFilter;{204c7cc8-bd2d-4e8d-a4b3-455ec337388a})");
}
unsafe impl ::windows_core::Interface for ConnectionProfileFilter {
    type Vtable = IConnectionProfileFilter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ConnectionProfileFilter {
    const IID: ::windows_core::GUID = <IConnectionProfileFilter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ConnectionProfileFilter {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectionProfileFilter";
}
::windows_core::imp::interface_hierarchy!(ConnectionProfileFilter, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ConnectionProfileFilter {}
unsafe impl ::core::marker::Sync for ConnectionProfileFilter {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ConnectionSession(::windows_core::IUnknown);
impl ConnectionSession {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { ::windows_core::vcall!(this.Close()).ok() }
    }
    pub fn ConnectionProfile(&self) -> ::windows_core::Result<ConnectionProfile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ConnectionProfile(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ConnectionSession {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ConnectionSession;{ff905d4c-f83b-41b0-8a0c-1462d9c56b73})");
}
unsafe impl ::windows_core::Interface for ConnectionSession {
    type Vtable = IConnectionSession_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ConnectionSession {
    const IID: ::windows_core::GUID = <IConnectionSession as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ConnectionSession {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectionSession";
}
::windows_core::imp::interface_hierarchy!(ConnectionSession, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for ConnectionSession {}
unsafe impl ::core::marker::Send for ConnectionSession {}
unsafe impl ::core::marker::Sync for ConnectionSession {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ConnectivityInterval(::windows_core::IUnknown);
impl ConnectivityInterval {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.StartTime(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionDuration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ConnectionDuration(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ConnectivityInterval {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ConnectivityInterval;{4faa3fff-6746-4824-a964-eed8e87f8709})");
}
unsafe impl ::windows_core::Interface for ConnectivityInterval {
    type Vtable = IConnectivityInterval_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ConnectivityInterval {
    const IID: ::windows_core::GUID = <IConnectivityInterval as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ConnectivityInterval {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectivityInterval";
}
::windows_core::imp::interface_hierarchy!(ConnectivityInterval, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ConnectivityInterval {}
unsafe impl ::core::marker::Sync for ConnectivityInterval {}
pub struct ConnectivityManager;
impl ConnectivityManager {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn AcquireConnectionAsync<P0>(cellularapncontext: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ConnectionSession>>
    where
        P0: ::windows_core::IntoParam<CellularApnContext>,
    {
        Self::IConnectivityManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.AcquireConnectionAsync(cellularapncontext.into_param().abi(), &mut result__)).from_abi(result__)
        })
    }
    pub fn AddHttpRoutePolicy<P0>(routepolicy: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<RoutePolicy>,
    {
        Self::IConnectivityManagerStatics(|this| unsafe { ::windows_core::vcall!(this.AddHttpRoutePolicy(routepolicy.into_param().abi())).ok() })
    }
    pub fn RemoveHttpRoutePolicy<P0>(routepolicy: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<RoutePolicy>,
    {
        Self::IConnectivityManagerStatics(|this| unsafe { ::windows_core::vcall!(this.RemoveHttpRoutePolicy(routepolicy.into_param().abi())).ok() })
    }
    #[doc(hidden)]
    pub fn IConnectivityManagerStatics<R, F: FnOnce(&IConnectivityManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ConnectivityManager, IConnectivityManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for ConnectivityManager {
    const NAME: &'static str = "Windows.Networking.Connectivity.ConnectivityManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DataPlanStatus(::windows_core::IUnknown);
impl DataPlanStatus {
    pub fn DataPlanUsage(&self) -> ::windows_core::Result<DataPlanUsage> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DataPlanUsage(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DataLimitInMegabytes(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DataLimitInMegabytes(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn InboundBitsPerSecond(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.InboundBitsPerSecond(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn OutboundBitsPerSecond(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u64>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OutboundBitsPerSecond(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn NextBillingCycle(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.NextBillingCycle(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MaxTransferSizeInMegabytes(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MaxTransferSizeInMegabytes(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for DataPlanStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.DataPlanStatus;{977a8b8c-3885-40f3-8851-42cd2bd568bb})");
}
unsafe impl ::windows_core::Interface for DataPlanStatus {
    type Vtable = IDataPlanStatus_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DataPlanStatus {
    const IID: ::windows_core::GUID = <IDataPlanStatus as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DataPlanStatus {
    const NAME: &'static str = "Windows.Networking.Connectivity.DataPlanStatus";
}
::windows_core::imp::interface_hierarchy!(DataPlanStatus, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DataPlanStatus {}
unsafe impl ::core::marker::Sync for DataPlanStatus {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DataPlanUsage(::windows_core::IUnknown);
impl DataPlanUsage {
    pub fn MegabytesUsed(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MegabytesUsed(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LastSyncTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LastSyncTime(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for DataPlanUsage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.DataPlanUsage;{b921492d-3b44-47ff-b361-be59e69ed1b0})");
}
unsafe impl ::windows_core::Interface for DataPlanUsage {
    type Vtable = IDataPlanUsage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DataPlanUsage {
    const IID: ::windows_core::GUID = <IDataPlanUsage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DataPlanUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.DataPlanUsage";
}
::windows_core::imp::interface_hierarchy!(DataPlanUsage, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DataPlanUsage {}
unsafe impl ::core::marker::Sync for DataPlanUsage {}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DataUsage(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
impl DataUsage {
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn BytesSent(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BytesSent(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn BytesReceived(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BytesReceived(&mut result__)).from_abi(result__)
        }
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeType for DataUsage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.DataUsage;{c1431dd3-b146-4d39-b959-0c69b096c512})");
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for DataUsage {
    type Vtable = IDataUsage_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for DataUsage {
    const IID: ::windows_core::GUID = <IDataUsage as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for DataUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.DataUsage";
}
#[cfg(feature = "deprecated")]
::windows_core::imp::interface_hierarchy!(DataUsage, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Send for DataUsage {}
#[cfg(feature = "deprecated")]
unsafe impl ::core::marker::Sync for DataUsage {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IPInformation(::windows_core::IUnknown);
impl IPInformation {
    pub fn NetworkAdapter(&self) -> ::windows_core::Result<NetworkAdapter> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.NetworkAdapter(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PrefixLength(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.PrefixLength(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for IPInformation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.IPInformation;{d85145e0-138f-47d7-9b3a-36bb488cef33})");
}
unsafe impl ::windows_core::Interface for IPInformation {
    type Vtable = IIPInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IPInformation {
    const IID: ::windows_core::GUID = <IIPInformation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for IPInformation {
    const NAME: &'static str = "Windows.Networking.Connectivity.IPInformation";
}
::windows_core::imp::interface_hierarchy!(IPInformation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for IPInformation {}
unsafe impl ::core::marker::Sync for IPInformation {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LanIdentifier(::windows_core::IUnknown);
impl LanIdentifier {
    pub fn InfrastructureId(&self) -> ::windows_core::Result<LanIdentifierData> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.InfrastructureId(&mut result__)).from_abi(result__)
        }
    }
    pub fn PortId(&self) -> ::windows_core::Result<LanIdentifierData> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.PortId(&mut result__)).from_abi(result__)
        }
    }
    pub fn NetworkAdapterId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.NetworkAdapterId(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for LanIdentifier {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.LanIdentifier;{48aa53aa-1108-4546-a6cb-9a74da4b7ba0})");
}
unsafe impl ::windows_core::Interface for LanIdentifier {
    type Vtable = ILanIdentifier_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LanIdentifier {
    const IID: ::windows_core::GUID = <ILanIdentifier as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LanIdentifier {
    const NAME: &'static str = "Windows.Networking.Connectivity.LanIdentifier";
}
::windows_core::imp::interface_hierarchy!(LanIdentifier, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LanIdentifier {}
unsafe impl ::core::marker::Sync for LanIdentifier {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct LanIdentifierData(::windows_core::IUnknown);
impl LanIdentifierData {
    pub fn Type(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Type(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Value(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Value(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for LanIdentifierData {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.LanIdentifierData;{a74e83c3-d639-45be-a36a-c4e4aeaf6d9b})");
}
unsafe impl ::windows_core::Interface for LanIdentifierData {
    type Vtable = ILanIdentifierData_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LanIdentifierData {
    const IID: ::windows_core::GUID = <ILanIdentifierData as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LanIdentifierData {
    const NAME: &'static str = "Windows.Networking.Connectivity.LanIdentifierData";
}
::windows_core::imp::interface_hierarchy!(LanIdentifierData, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LanIdentifierData {}
unsafe impl ::core::marker::Sync for LanIdentifierData {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NetworkAdapter(::windows_core::IUnknown);
impl NetworkAdapter {
    pub fn OutboundMaxBitsPerSecond(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OutboundMaxBitsPerSecond(&mut result__)).from_abi(result__)
        }
    }
    pub fn InboundMaxBitsPerSecond(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.InboundMaxBitsPerSecond(&mut result__)).from_abi(result__)
        }
    }
    pub fn IanaInterfaceType(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IanaInterfaceType(&mut result__)).from_abi(result__)
        }
    }
    pub fn NetworkItem(&self) -> ::windows_core::Result<NetworkItem> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.NetworkItem(&mut result__)).from_abi(result__)
        }
    }
    pub fn NetworkAdapterId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.NetworkAdapterId(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetConnectedProfileAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ConnectionProfile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetConnectedProfileAsync(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for NetworkAdapter {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.NetworkAdapter;{3b542e03-5388-496c-a8a3-affd39aec2e6})");
}
unsafe impl ::windows_core::Interface for NetworkAdapter {
    type Vtable = INetworkAdapter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NetworkAdapter {
    const IID: ::windows_core::GUID = <INetworkAdapter as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NetworkAdapter {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkAdapter";
}
::windows_core::imp::interface_hierarchy!(NetworkAdapter, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for NetworkAdapter {}
unsafe impl ::core::marker::Sync for NetworkAdapter {}
pub struct NetworkInformation;
impl NetworkInformation {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConnectionProfiles() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<ConnectionProfile>> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetConnectionProfiles(&mut result__)).from_abi(result__)
        })
    }
    pub fn GetInternetConnectionProfile() -> ::windows_core::Result<ConnectionProfile> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetInternetConnectionProfile(&mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetLanIdentifiers() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<LanIdentifier>> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetLanIdentifiers(&mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetHostNames() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>> {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetHostNames(&mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetProxyConfigurationAsync<P0>(uri: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProxyConfiguration>>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetProxyConfigurationAsync(uri.into_param().abi(), &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSortedEndpointPairs<P0>(destinationlist: P0, sortoptions: super::HostNameSortOptions) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::EndpointPair>>,
    {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetSortedEndpointPairs(destinationlist.try_into_param()?.abi(), sortoptions, &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn NetworkStatusChanged<P0>(networkstatushandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<NetworkStatusChangedEventHandler>,
    {
        Self::INetworkInformationStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.NetworkStatusChanged(networkstatushandler.into_param().abi(), &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNetworkStatusChanged(eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::INetworkInformationStatics(|this| unsafe { ::windows_core::vcall!(this.RemoveNetworkStatusChanged(eventcookie)).ok() })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindConnectionProfilesAsync<P0>(pprofilefilter: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConnectionProfile>>>
    where
        P0: ::windows_core::IntoParam<ConnectionProfileFilter>,
    {
        Self::INetworkInformationStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FindConnectionProfilesAsync(pprofilefilter.into_param().abi(), &mut result__)).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn INetworkInformationStatics<R, F: FnOnce(&INetworkInformationStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NetworkInformation, INetworkInformationStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn INetworkInformationStatics2<R, F: FnOnce(&INetworkInformationStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<NetworkInformation, INetworkInformationStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for NetworkInformation {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkInformation";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NetworkItem(::windows_core::IUnknown);
impl NetworkItem {
    pub fn NetworkId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.NetworkId(&mut result__)).from_abi(result__)
        }
    }
    pub fn GetNetworkTypes(&self) -> ::windows_core::Result<NetworkTypes> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetNetworkTypes(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for NetworkItem {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.NetworkItem;{01bc4d39-f5e0-4567-a28c-42080c831b2b})");
}
unsafe impl ::windows_core::Interface for NetworkItem {
    type Vtable = INetworkItem_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NetworkItem {
    const IID: ::windows_core::GUID = <INetworkItem as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NetworkItem {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkItem";
}
::windows_core::imp::interface_hierarchy!(NetworkItem, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for NetworkItem {}
unsafe impl ::core::marker::Sync for NetworkItem {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NetworkSecuritySettings(::windows_core::IUnknown);
impl NetworkSecuritySettings {
    pub fn NetworkAuthenticationType(&self) -> ::windows_core::Result<NetworkAuthenticationType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.NetworkAuthenticationType(&mut result__)).from_abi(result__)
        }
    }
    pub fn NetworkEncryptionType(&self) -> ::windows_core::Result<NetworkEncryptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.NetworkEncryptionType(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for NetworkSecuritySettings {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.NetworkSecuritySettings;{7ca07e8d-917b-4b5f-b84d-28f7a5ac5402})");
}
unsafe impl ::windows_core::Interface for NetworkSecuritySettings {
    type Vtable = INetworkSecuritySettings_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NetworkSecuritySettings {
    const IID: ::windows_core::GUID = <INetworkSecuritySettings as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NetworkSecuritySettings {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkSecuritySettings";
}
::windows_core::imp::interface_hierarchy!(NetworkSecuritySettings, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for NetworkSecuritySettings {}
unsafe impl ::core::marker::Sync for NetworkSecuritySettings {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NetworkStateChangeEventDetails(::windows_core::IUnknown);
impl NetworkStateChangeEventDetails {
    pub fn HasNewInternetConnectionProfile(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.HasNewInternetConnectionProfile(&mut result__)).from_abi(result__)
        }
    }
    pub fn HasNewConnectionCost(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.HasNewConnectionCost(&mut result__)).from_abi(result__)
        }
    }
    pub fn HasNewNetworkConnectivityLevel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.HasNewNetworkConnectivityLevel(&mut result__)).from_abi(result__)
        }
    }
    pub fn HasNewDomainConnectivityLevel(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.HasNewDomainConnectivityLevel(&mut result__)).from_abi(result__)
        }
    }
    pub fn HasNewHostNameList(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.HasNewHostNameList(&mut result__)).from_abi(result__)
        }
    }
    pub fn HasNewWwanRegistrationState(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.HasNewWwanRegistrationState(&mut result__)).from_abi(result__)
        }
    }
    pub fn HasNewTetheringOperationalState(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<INetworkStateChangeEventDetails2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.HasNewTetheringOperationalState(&mut result__)).from_abi(result__)
        }
    }
    pub fn HasNewTetheringClientCount(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<INetworkStateChangeEventDetails2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.HasNewTetheringClientCount(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for NetworkStateChangeEventDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.NetworkStateChangeEventDetails;{1f0cf333-d7a6-44dd-a4e9-687c476b903d})");
}
unsafe impl ::windows_core::Interface for NetworkStateChangeEventDetails {
    type Vtable = INetworkStateChangeEventDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NetworkStateChangeEventDetails {
    const IID: ::windows_core::GUID = <INetworkStateChangeEventDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NetworkStateChangeEventDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkStateChangeEventDetails";
}
::windows_core::imp::interface_hierarchy!(NetworkStateChangeEventDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for NetworkStateChangeEventDetails {}
unsafe impl ::core::marker::Sync for NetworkStateChangeEventDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NetworkUsage(::windows_core::IUnknown);
impl NetworkUsage {
    pub fn BytesSent(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BytesSent(&mut result__)).from_abi(result__)
        }
    }
    pub fn BytesReceived(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BytesReceived(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ConnectionDuration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ConnectionDuration(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for NetworkUsage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.NetworkUsage;{49da8fce-9985-4927-bf5b-072b5c65f8d9})");
}
unsafe impl ::windows_core::Interface for NetworkUsage {
    type Vtable = INetworkUsage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NetworkUsage {
    const IID: ::windows_core::GUID = <INetworkUsage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.NetworkUsage";
}
::windows_core::imp::interface_hierarchy!(NetworkUsage, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for NetworkUsage {}
unsafe impl ::core::marker::Sync for NetworkUsage {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProviderNetworkUsage(::windows_core::IUnknown);
impl ProviderNetworkUsage {
    pub fn BytesSent(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BytesSent(&mut result__)).from_abi(result__)
        }
    }
    pub fn BytesReceived(&self) -> ::windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BytesReceived(&mut result__)).from_abi(result__)
        }
    }
    pub fn ProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ProviderId(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ProviderNetworkUsage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ProviderNetworkUsage;{5ec69e04-7931-48c8-b8f3-46300fa42728})");
}
unsafe impl ::windows_core::Interface for ProviderNetworkUsage {
    type Vtable = IProviderNetworkUsage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProviderNetworkUsage {
    const IID: ::windows_core::GUID = <IProviderNetworkUsage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProviderNetworkUsage {
    const NAME: &'static str = "Windows.Networking.Connectivity.ProviderNetworkUsage";
}
::windows_core::imp::interface_hierarchy!(ProviderNetworkUsage, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProviderNetworkUsage {}
unsafe impl ::core::marker::Sync for ProviderNetworkUsage {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProxyConfiguration(::windows_core::IUnknown);
impl ProxyConfiguration {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProxyUris(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ProxyUris(&mut result__)).from_abi(result__)
        }
    }
    pub fn CanConnectDirectly(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CanConnectDirectly(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ProxyConfiguration {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.ProxyConfiguration;{ef3a60b4-9004-4dd6-b7d8-b3e502f4aad0})");
}
unsafe impl ::windows_core::Interface for ProxyConfiguration {
    type Vtable = IProxyConfiguration_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProxyConfiguration {
    const IID: ::windows_core::GUID = <IProxyConfiguration as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProxyConfiguration {
    const NAME: &'static str = "Windows.Networking.Connectivity.ProxyConfiguration";
}
::windows_core::imp::interface_hierarchy!(ProxyConfiguration, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProxyConfiguration {}
unsafe impl ::core::marker::Sync for ProxyConfiguration {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct RoutePolicy(::windows_core::IUnknown);
impl RoutePolicy {
    pub fn ConnectionProfile(&self) -> ::windows_core::Result<ConnectionProfile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ConnectionProfile(&mut result__)).from_abi(result__)
        }
    }
    pub fn HostName(&self) -> ::windows_core::Result<super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.HostName(&mut result__)).from_abi(result__)
        }
    }
    pub fn HostNameType(&self) -> ::windows_core::Result<super::DomainNameType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.HostNameType(&mut result__)).from_abi(result__)
        }
    }
    pub fn CreateRoutePolicy<P0, P1>(connectionprofile: P0, hostname: P1, r#type: super::DomainNameType) -> ::windows_core::Result<RoutePolicy>
    where
        P0: ::windows_core::IntoParam<ConnectionProfile>,
        P1: ::windows_core::IntoParam<super::HostName>,
    {
        Self::IRoutePolicyFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateRoutePolicy(connectionprofile.into_param().abi(), hostname.into_param().abi(), r#type, &mut result__)).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRoutePolicyFactory<R, F: FnOnce(&IRoutePolicyFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RoutePolicy, IRoutePolicyFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for RoutePolicy {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.RoutePolicy;{11abc4ac-0fc7-42e4-8742-569923b1ca11})");
}
unsafe impl ::windows_core::Interface for RoutePolicy {
    type Vtable = IRoutePolicy_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RoutePolicy {
    const IID: ::windows_core::GUID = <IRoutePolicy as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RoutePolicy {
    const NAME: &'static str = "Windows.Networking.Connectivity.RoutePolicy";
}
::windows_core::imp::interface_hierarchy!(RoutePolicy, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for RoutePolicy {}
unsafe impl ::core::marker::Sync for RoutePolicy {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WlanConnectionProfileDetails(::windows_core::IUnknown);
impl WlanConnectionProfileDetails {
    pub fn GetConnectedSsid(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetConnectedSsid(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WlanConnectionProfileDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.WlanConnectionProfileDetails;{562098cb-b35a-4bf1-a884-b7557e88ff86})");
}
unsafe impl ::windows_core::Interface for WlanConnectionProfileDetails {
    type Vtable = IWlanConnectionProfileDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WlanConnectionProfileDetails {
    const IID: ::windows_core::GUID = <IWlanConnectionProfileDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WlanConnectionProfileDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.WlanConnectionProfileDetails";
}
::windows_core::imp::interface_hierarchy!(WlanConnectionProfileDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WlanConnectionProfileDetails {}
unsafe impl ::core::marker::Sync for WlanConnectionProfileDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct WwanConnectionProfileDetails(::windows_core::IUnknown);
impl WwanConnectionProfileDetails {
    pub fn HomeProviderId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.HomeProviderId(&mut result__)).from_abi(result__)
        }
    }
    pub fn AccessPointName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.AccessPointName(&mut result__)).from_abi(result__)
        }
    }
    pub fn GetNetworkRegistrationState(&self) -> ::windows_core::Result<WwanNetworkRegistrationState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetNetworkRegistrationState(&mut result__)).from_abi(result__)
        }
    }
    pub fn GetCurrentDataClass(&self) -> ::windows_core::Result<WwanDataClass> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetCurrentDataClass(&mut result__)).from_abi(result__)
        }
    }
    pub fn IPKind(&self) -> ::windows_core::Result<WwanNetworkIPKind> {
        let this = &::windows_core::ComInterface::cast::<IWwanConnectionProfileDetails2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IPKind(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PurposeGuids(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::GUID>> {
        let this = &::windows_core::ComInterface::cast::<IWwanConnectionProfileDetails2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.PurposeGuids(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for WwanConnectionProfileDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.Connectivity.WwanConnectionProfileDetails;{0e4da8fe-835f-4df3-82fd-df556ebc09ef})");
}
unsafe impl ::windows_core::Interface for WwanConnectionProfileDetails {
    type Vtable = IWwanConnectionProfileDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for WwanConnectionProfileDetails {
    const IID: ::windows_core::GUID = <IWwanConnectionProfileDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for WwanConnectionProfileDetails {
    const NAME: &'static str = "Windows.Networking.Connectivity.WwanConnectionProfileDetails";
}
::windows_core::imp::interface_hierarchy!(WwanConnectionProfileDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for WwanConnectionProfileDetails {}
unsafe impl ::core::marker::Sync for WwanConnectionProfileDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CellularApnAuthenticationType(pub i32);
impl CellularApnAuthenticationType {
    pub const None: Self = Self(0i32);
    pub const Pap: Self = Self(1i32);
    pub const Chap: Self = Self(2i32);
    pub const Mschapv2: Self = Self(3i32);
}
impl ::core::marker::Copy for CellularApnAuthenticationType {}
impl ::core::clone::Clone for CellularApnAuthenticationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CellularApnAuthenticationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CellularApnAuthenticationType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CellularApnAuthenticationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CellularApnAuthenticationType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CellularApnAuthenticationType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.CellularApnAuthenticationType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ConnectionProfileDeleteStatus(pub i32);
impl ConnectionProfileDeleteStatus {
    pub const Success: Self = Self(0i32);
    pub const DeniedByUser: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl ::core::marker::Copy for ConnectionProfileDeleteStatus {}
impl ::core::clone::Clone for ConnectionProfileDeleteStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConnectionProfileDeleteStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ConnectionProfileDeleteStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ConnectionProfileDeleteStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConnectionProfileDeleteStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ConnectionProfileDeleteStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.ConnectionProfileDeleteStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DataUsageGranularity(pub i32);
impl DataUsageGranularity {
    pub const PerMinute: Self = Self(0i32);
    pub const PerHour: Self = Self(1i32);
    pub const PerDay: Self = Self(2i32);
    pub const Total: Self = Self(3i32);
}
impl ::core::marker::Copy for DataUsageGranularity {}
impl ::core::clone::Clone for DataUsageGranularity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataUsageGranularity {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DataUsageGranularity {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DataUsageGranularity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataUsageGranularity").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DataUsageGranularity {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.DataUsageGranularity;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DomainAuthenticationKind(pub i32);
impl DomainAuthenticationKind {
    pub const None: Self = Self(0i32);
    pub const Ldap: Self = Self(1i32);
    pub const Tls: Self = Self(2i32);
}
impl ::core::marker::Copy for DomainAuthenticationKind {}
impl ::core::clone::Clone for DomainAuthenticationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DomainAuthenticationKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DomainAuthenticationKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DomainAuthenticationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DomainAuthenticationKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DomainAuthenticationKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.DomainAuthenticationKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DomainConnectivityLevel(pub i32);
impl DomainConnectivityLevel {
    pub const None: Self = Self(0i32);
    pub const Unauthenticated: Self = Self(1i32);
    pub const Authenticated: Self = Self(2i32);
}
impl ::core::marker::Copy for DomainConnectivityLevel {}
impl ::core::clone::Clone for DomainConnectivityLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DomainConnectivityLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DomainConnectivityLevel {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DomainConnectivityLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DomainConnectivityLevel").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DomainConnectivityLevel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.DomainConnectivityLevel;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NetworkAuthenticationType(pub i32);
impl NetworkAuthenticationType {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const Open80211: Self = Self(2i32);
    pub const SharedKey80211: Self = Self(3i32);
    pub const Wpa: Self = Self(4i32);
    pub const WpaPsk: Self = Self(5i32);
    pub const WpaNone: Self = Self(6i32);
    pub const Rsna: Self = Self(7i32);
    pub const RsnaPsk: Self = Self(8i32);
    pub const Ihv: Self = Self(9i32);
    pub const Wpa3: Self = Self(10i32);
    pub const Wpa3Enterprise192Bits: Self = Self(10i32);
    pub const Wpa3Sae: Self = Self(11i32);
    pub const Owe: Self = Self(12i32);
    pub const Wpa3Enterprise: Self = Self(13i32);
}
impl ::core::marker::Copy for NetworkAuthenticationType {}
impl ::core::clone::Clone for NetworkAuthenticationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkAuthenticationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NetworkAuthenticationType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NetworkAuthenticationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkAuthenticationType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for NetworkAuthenticationType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkAuthenticationType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NetworkConnectivityLevel(pub i32);
impl NetworkConnectivityLevel {
    pub const None: Self = Self(0i32);
    pub const LocalAccess: Self = Self(1i32);
    pub const ConstrainedInternetAccess: Self = Self(2i32);
    pub const InternetAccess: Self = Self(3i32);
}
impl ::core::marker::Copy for NetworkConnectivityLevel {}
impl ::core::clone::Clone for NetworkConnectivityLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkConnectivityLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NetworkConnectivityLevel {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NetworkConnectivityLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkConnectivityLevel").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for NetworkConnectivityLevel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkConnectivityLevel;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NetworkCostType(pub i32);
impl NetworkCostType {
    pub const Unknown: Self = Self(0i32);
    pub const Unrestricted: Self = Self(1i32);
    pub const Fixed: Self = Self(2i32);
    pub const Variable: Self = Self(3i32);
}
impl ::core::marker::Copy for NetworkCostType {}
impl ::core::clone::Clone for NetworkCostType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkCostType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NetworkCostType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NetworkCostType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkCostType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for NetworkCostType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkCostType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NetworkEncryptionType(pub i32);
impl NetworkEncryptionType {
    pub const None: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
    pub const Wep: Self = Self(2i32);
    pub const Wep40: Self = Self(3i32);
    pub const Wep104: Self = Self(4i32);
    pub const Tkip: Self = Self(5i32);
    pub const Ccmp: Self = Self(6i32);
    pub const WpaUseGroup: Self = Self(7i32);
    pub const RsnUseGroup: Self = Self(8i32);
    pub const Ihv: Self = Self(9i32);
    pub const Gcmp: Self = Self(10i32);
    pub const Gcmp256: Self = Self(11i32);
}
impl ::core::marker::Copy for NetworkEncryptionType {}
impl ::core::clone::Clone for NetworkEncryptionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkEncryptionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NetworkEncryptionType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NetworkEncryptionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkEncryptionType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for NetworkEncryptionType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkEncryptionType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NetworkTypes(pub u32);
impl NetworkTypes {
    pub const None: Self = Self(0u32);
    pub const Internet: Self = Self(1u32);
    pub const PrivateNetwork: Self = Self(2u32);
}
impl ::core::marker::Copy for NetworkTypes {}
impl ::core::clone::Clone for NetworkTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NetworkTypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for NetworkTypes {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NetworkTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkTypes").field(&self.0).finish()
    }
}
impl NetworkTypes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for NetworkTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NetworkTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NetworkTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NetworkTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NetworkTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for NetworkTypes {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.NetworkTypes;u4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RoamingStates(pub u32);
impl RoamingStates {
    pub const None: Self = Self(0u32);
    pub const NotRoaming: Self = Self(1u32);
    pub const Roaming: Self = Self(2u32);
}
impl ::core::marker::Copy for RoamingStates {}
impl ::core::clone::Clone for RoamingStates {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RoamingStates {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RoamingStates {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RoamingStates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RoamingStates").field(&self.0).finish()
    }
}
impl RoamingStates {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for RoamingStates {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RoamingStates {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RoamingStates {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RoamingStates {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RoamingStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for RoamingStates {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.RoamingStates;u4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TriStates(pub i32);
impl TriStates {
    pub const DoNotCare: Self = Self(0i32);
    pub const No: Self = Self(1i32);
    pub const Yes: Self = Self(2i32);
}
impl ::core::marker::Copy for TriStates {}
impl ::core::clone::Clone for TriStates {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TriStates {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TriStates {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TriStates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TriStates").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TriStates {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.TriStates;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WwanDataClass(pub u32);
impl WwanDataClass {
    pub const None: Self = Self(0u32);
    pub const Gprs: Self = Self(1u32);
    pub const Edge: Self = Self(2u32);
    pub const Umts: Self = Self(4u32);
    pub const Hsdpa: Self = Self(8u32);
    pub const Hsupa: Self = Self(16u32);
    pub const LteAdvanced: Self = Self(32u32);
    pub const Cdma1xRtt: Self = Self(65536u32);
    pub const Cdma1xEvdo: Self = Self(131072u32);
    pub const Cdma1xEvdoRevA: Self = Self(262144u32);
    pub const Cdma1xEvdv: Self = Self(524288u32);
    pub const Cdma3xRtt: Self = Self(1048576u32);
    pub const Cdma1xEvdoRevB: Self = Self(2097152u32);
    pub const CdmaUmb: Self = Self(4194304u32);
    pub const Custom: Self = Self(2147483648u32);
}
impl ::core::marker::Copy for WwanDataClass {}
impl ::core::clone::Clone for WwanDataClass {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WwanDataClass {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WwanDataClass {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WwanDataClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WwanDataClass").field(&self.0).finish()
    }
}
impl WwanDataClass {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for WwanDataClass {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WwanDataClass {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WwanDataClass {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WwanDataClass {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WwanDataClass {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for WwanDataClass {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.WwanDataClass;u4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WwanNetworkIPKind(pub i32);
impl WwanNetworkIPKind {
    pub const None: Self = Self(0i32);
    pub const Ipv4: Self = Self(1i32);
    pub const Ipv6: Self = Self(2i32);
    pub const Ipv4v6: Self = Self(3i32);
    pub const Ipv4v6v4Xlat: Self = Self(4i32);
}
impl ::core::marker::Copy for WwanNetworkIPKind {}
impl ::core::clone::Clone for WwanNetworkIPKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WwanNetworkIPKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WwanNetworkIPKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WwanNetworkIPKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WwanNetworkIPKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WwanNetworkIPKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.WwanNetworkIPKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WwanNetworkRegistrationState(pub i32);
impl WwanNetworkRegistrationState {
    pub const None: Self = Self(0i32);
    pub const Deregistered: Self = Self(1i32);
    pub const Searching: Self = Self(2i32);
    pub const Home: Self = Self(3i32);
    pub const Roaming: Self = Self(4i32);
    pub const Partner: Self = Self(5i32);
    pub const Denied: Self = Self(6i32);
}
impl ::core::marker::Copy for WwanNetworkRegistrationState {}
impl ::core::clone::Clone for WwanNetworkRegistrationState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WwanNetworkRegistrationState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for WwanNetworkRegistrationState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for WwanNetworkRegistrationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WwanNetworkRegistrationState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for WwanNetworkRegistrationState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.Connectivity.WwanNetworkRegistrationState;i4)");
}
#[repr(C)]
pub struct NetworkUsageStates {
    pub Roaming: TriStates,
    pub Shared: TriStates,
}
impl ::core::marker::Copy for NetworkUsageStates {}
impl ::core::clone::Clone for NetworkUsageStates {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NetworkUsageStates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NetworkUsageStates").field("Roaming", &self.Roaming).field("Shared", &self.Shared).finish()
    }
}
impl ::windows_core::TypeKind for NetworkUsageStates {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for NetworkUsageStates {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Networking.Connectivity.NetworkUsageStates;enum(Windows.Networking.Connectivity.TriStates;i4);enum(Windows.Networking.Connectivity.TriStates;i4))");
}
impl ::core::cmp::PartialEq for NetworkUsageStates {
    fn eq(&self, other: &Self) -> bool {
        self.Roaming == other.Roaming && self.Shared == other.Shared
    }
}
impl ::core::cmp::Eq for NetworkUsageStates {}
impl ::core::default::Default for NetworkUsageStates {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NetworkStatusChangedEventHandler(pub ::windows_core::IUnknown);
impl NetworkStatusChangedEventHandler {
    pub fn new<F: FnMut(::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = NetworkStatusChangedEventHandlerBox::<F> { vtable: &NetworkStatusChangedEventHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, sender: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Invoke(sender.into_param().abi())).ok() }
    }
}
#[repr(C)]
struct NetworkStatusChangedEventHandlerBox<F: FnMut(::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const NetworkStatusChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<()> + ::core::marker::Send + 'static> NetworkStatusChangedEventHandlerBox<F> {
    const VTABLE: NetworkStatusChangedEventHandler_Vtbl = NetworkStatusChangedEventHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return ::windows_core::HRESULT(-2147467261);
        }
        *interface = if *iid == <NetworkStatusChangedEventHandler as ::windows_core::ComInterface>::IID || *iid == <::windows_core::IUnknown as ::windows_core::ComInterface>::IID || *iid == <::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows_core::from_raw_borrowed(&sender)).into()
    }
}
unsafe impl ::windows_core::Interface for NetworkStatusChangedEventHandler {
    type Vtable = NetworkStatusChangedEventHandler_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NetworkStatusChangedEventHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x71ba143f_598e_49d0_84eb_8febaedcc195);
}
impl ::windows_core::RuntimeType for NetworkStatusChangedEventHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{71ba143f-598e-49d0-84eb-8febaedcc195}");
}
#[repr(C)]
#[doc(hidden)]
pub struct NetworkStatusChangedEventHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
