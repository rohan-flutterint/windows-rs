#[cfg(feature = "ApplicationModel_Appointments_AppointmentsProvider")]
#[doc = "Required features: `\"ApplicationModel_Appointments_AppointmentsProvider\"`"]
pub mod AppointmentsProvider;
#[cfg(feature = "ApplicationModel_Appointments_DataProvider")]
#[doc = "Required features: `\"ApplicationModel_Appointments_DataProvider\"`"]
pub mod DataProvider;
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointment(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointment {
    type Vtable = IAppointment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointment {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd002f2f_2bdd_4076_90a3_22c275312965);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointment_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Reminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Reminder: usize,
    #[cfg(feature = "Foundation")]
    pub SetReminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetReminder: usize,
    pub Organizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetOrganizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Invitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Invitees: usize,
    pub Recurrence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRecurrence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BusyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentBusyStatus) -> ::windows_core::HRESULT,
    pub SetBusyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentBusyStatus) -> ::windows_core::HRESULT,
    pub AllDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Sensitivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentSensitivity) -> ::windows_core::HRESULT,
    pub SetSensitivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentSensitivity) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointment2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointment2 {
    type Vtable = IAppointment2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointment2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e85983c_540f_3452_9b5c_0dd7ad4c65a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointment2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CalendarId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RoamingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRoamingId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OriginalStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OriginalStartTime: usize,
    pub IsResponseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsResponseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub AllowNewTimeProposal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetAllowNewTimeProposal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub OnlineMeetingLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetOnlineMeetingLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReplyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReplyTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetReplyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetReplyTime: usize,
    pub UserResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentParticipantResponse) -> ::windows_core::HRESULT,
    pub SetUserResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentParticipantResponse) -> ::windows_core::HRESULT,
    pub HasInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsCanceledMeeting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsCanceledMeeting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsOrganizedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsOrganizedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointment3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointment3 {
    type Vtable = IAppointment3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointment3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbfcc45a9_8961_4991_934b_c48768e5a96c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointment3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub RemoteChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows_core::HRESULT,
    pub SetRemoteChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64) -> ::windows_core::HRESULT,
    pub DetailsKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentDetailsKind) -> ::windows_core::HRESULT,
    pub SetDetailsKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentDetailsKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentCalendar(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentCalendar {
    type Vtable = IAppointmentCalendar_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentCalendar {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5273819d_8339_3d4f_a02f_64084452bb5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendar_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub DisplayColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DisplayColor: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub LocalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentCalendarOtherAppReadAccess) -> ::windows_core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentCalendarOtherAppReadAccess) -> ::windows_core::HRESULT,
    pub OtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentCalendarOtherAppWriteAccess) -> ::windows_core::HRESULT,
    pub SetOtherAppWriteAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentCalendarOtherAppWriteAccess) -> ::windows_core::HRESULT,
    pub SourceDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SummaryCardView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentSummaryCardView) -> ::windows_core::HRESULT,
    pub SetSummaryCardView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentSummaryCardView) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentsAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentsAsyncWithOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindExceptionsFromMasterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, masterlocalid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindExceptionsFromMasterAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllInstancesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, masterlocalid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllInstancesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllInstancesAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, masterlocalid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, poptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllInstancesAsyncWithOptions: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppointmentInstanceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, instancestarttime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppointmentInstanceAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindUnexpandedAppointmentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindUnexpandedAppointmentsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindUnexpandedAppointmentsAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindUnexpandedAppointmentsAsyncWithOptions: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAppointmentInstanceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, instancestarttime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAppointmentInstanceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pappointment: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAppointmentAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentCalendar2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentCalendar2 {
    type Vtable = IAppointmentCalendar2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentCalendar2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18e7e422_2467_4e1c_a459_d8a29303d092);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendar2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SyncManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetRemoteId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub SetDisplayColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetDisplayColor: usize,
    pub SetIsHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub UserDataAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub CanCreateOrUpdateAppointments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanCreateOrUpdateAppointments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanCancelMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanCancelMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanForwardMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanForwardMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanProposeNewTimeForMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanProposeNewTimeForMeetings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanUpdateMeetingResponses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanUpdateMeetingResponses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanNotifyInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetCanNotifyInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub MustNofityInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetMustNofityInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryCreateOrUpdateAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: *mut ::core::ffi::c_void, notifyinvitees: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCreateOrUpdateAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryCancelMeetingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: *mut ::core::ffi::c_void, subject: ::std::mem::MaybeUninit<::windows_core::HSTRING>, comment: ::std::mem::MaybeUninit<::windows_core::HSTRING>, notifyinvitees: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCancelMeetingAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TryForwardMeetingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: *mut ::core::ffi::c_void, invitees: *mut ::core::ffi::c_void, subject: ::std::mem::MaybeUninit<::windows_core::HSTRING>, forwardheader: ::std::mem::MaybeUninit<::windows_core::HSTRING>, comment: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TryForwardMeetingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryProposeNewTimeForMeetingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: *mut ::core::ffi::c_void, newstarttime: super::super::Foundation::DateTime, newduration: super::super::Foundation::TimeSpan, subject: ::std::mem::MaybeUninit<::windows_core::HSTRING>, comment: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryProposeNewTimeForMeetingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryUpdateMeetingResponseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, meeting: *mut ::core::ffi::c_void, response: AppointmentParticipantResponse, subject: ::std::mem::MaybeUninit<::windows_core::HSTRING>, comment: ::std::mem::MaybeUninit<::windows_core::HSTRING>, sendupdate: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUpdateMeetingResponseAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentCalendar3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentCalendar3 {
    type Vtable = IAppointmentCalendar3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentCalendar3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb23d22b_a685_42ae_8495_b3119adb4167);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendar3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RegisterSyncManagerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RegisterSyncManagerAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentCalendarSyncManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentCalendarSyncManager {
    type Vtable = IAppointmentCalendarSyncManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentCalendarSyncManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b21b3a0_4aff_4392_bc5f_5645ffcffb17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarSyncManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentCalendarSyncStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub LastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LastAttemptedSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SyncAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncStatusChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSyncStatusChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSyncStatusChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentCalendarSyncManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentCalendarSyncManager2 {
    type Vtable = IAppointmentCalendarSyncManager2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentCalendarSyncManager2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x647528ad_0d29_4c7c_aaa7_bf996805537c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentCalendarSyncManager2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentCalendarSyncStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetLastSuccessfulSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastSuccessfulSyncTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetLastAttemptedSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLastAttemptedSyncTime: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentConflictResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentConflictResult {
    type Vtable = IAppointmentConflictResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentConflictResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5cdf0be_2f2f_3b7d_af0a_a7e20f3a46e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentConflictResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentConflictType) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentException(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentException {
    type Vtable = IAppointmentException_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentException {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2076767_16f6_4bce_9f5a_8600b8019fcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentException_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Appointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExceptionProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExceptionProperties: usize,
    pub IsDeleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentInvitee(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentInvitee {
    type Vtable = IAppointmentInvitee_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentInvitee {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13bf0796_9842_495b_b0e7_ef8f79c0701d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentInvitee_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Role: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentParticipantRole) -> ::windows_core::HRESULT,
    pub SetRole: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentParticipantRole) -> ::windows_core::HRESULT,
    pub Response: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentParticipantResponse) -> ::windows_core::HRESULT,
    pub SetResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentParticipantResponse) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentManagerForUser(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentManagerForUser {
    type Vtable = IAppointmentManagerForUser_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentManagerForUser {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70261423_73cc_4660_b318_b01365302a03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentManagerForUser_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ShowAddAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAddAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowAddAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowAddAppointmentWithPlacementAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowReplaceAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowReplaceAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowReplaceAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowReplaceAppointmentWithPlacementAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowReplaceAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowReplaceAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowRemoveAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowRemoveAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowRemoveAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowRemoveAppointmentWithPlacementAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowRemoveAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowRemoveAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowTimeFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timetoshow: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowTimeFrameAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsWithDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsWithDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowEditNewAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowEditNewAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: AppointmentStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentManagerStatics {
    type Vtable = IAppointmentManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3a30fa01_5c40_499d_b33f_a43050f74fc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ShowAddAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAddAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowAddAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowAddAppointmentWithPlacementAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowReplaceAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowReplaceAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowReplaceAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowReplaceAppointmentWithPlacementAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowReplaceAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowReplaceAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowRemoveAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowRemoveAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowRemoveAppointmentWithPlacementAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowRemoveAppointmentWithPlacementAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowRemoveAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowRemoveAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowTimeFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timetoshow: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowTimeFrameAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentManagerStatics2 {
    type Vtable = IAppointmentManagerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a81f60d_d04f_4034_af72_a36573b45ff0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsWithDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointmentid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsWithDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowEditNewAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowEditNewAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: AppointmentStoreAccessType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentManagerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentManagerStatics3 {
    type Vtable = IAppointmentManagerStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentManagerStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2f9ae09c_b34c_4dc7_a35d_cafd88ae3ec6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentParticipant(::windows_core::IUnknown);
impl IAppointmentParticipant {
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DisplayName(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetDisplayName(::core::mem::transmute_copy(value))).ok() }
    }
    pub fn Address(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Address(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetAddress(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetAddress(::core::mem::transmute_copy(value))).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IAppointmentParticipant, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IAppointmentParticipant {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{615e2902-9718-467b-83fb-b293a19121de}");
}
unsafe impl ::windows_core::Interface for IAppointmentParticipant {
    type Vtable = IAppointmentParticipant_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentParticipant {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x615e2902_9718_467b_83fb_b293a19121de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentParticipant_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentPropertiesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentPropertiesStatics {
    type Vtable = IAppointmentPropertiesStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentPropertiesStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25141fe9_68ae_3aae_855f_bc4441caa234);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentPropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Subject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Reminder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub BusyStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Sensitivity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OriginalStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsResponseRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AllowNewTimeProposal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AllDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Details: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub OnlineMeetingLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReplyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Organizer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub UserResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HasInvitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsCanceledMeeting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsOrganizedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Recurrence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Invitees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DefaultProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DefaultProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentPropertiesStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentPropertiesStatics2 {
    type Vtable = IAppointmentPropertiesStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentPropertiesStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdffc434b_b017_45dd_8af5_d163d10801bb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentPropertiesStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub RemoteChangeNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DetailsKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentRecurrence(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentRecurrence {
    type Vtable = IAppointmentRecurrence_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentRecurrence {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd87b3e83_15a6_487b_b959_0c361e60e954);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentRecurrence_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Unit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentRecurrenceUnit) -> ::windows_core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentRecurrenceUnit) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Occurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Occurrences: usize,
    #[cfg(feature = "Foundation")]
    pub SetOccurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetOccurrences: usize,
    #[cfg(feature = "Foundation")]
    pub Until: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Until: usize,
    #[cfg(feature = "Foundation")]
    pub SetUntil: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUntil: usize,
    pub Interval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub DaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentDaysOfWeek) -> ::windows_core::HRESULT,
    pub SetDaysOfWeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentDaysOfWeek) -> ::windows_core::HRESULT,
    pub WeekOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentWeekOfMonth) -> ::windows_core::HRESULT,
    pub SetWeekOfMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: AppointmentWeekOfMonth) -> ::windows_core::HRESULT,
    pub Month: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMonth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Day: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetDay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentRecurrence2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentRecurrence2 {
    type Vtable = IAppointmentRecurrence2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentRecurrence2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3df3a2e0_05a7_4f50_9f86_b03f9436254d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentRecurrence2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RecurrenceType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RecurrenceType) -> ::windows_core::HRESULT,
    pub TimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTimeZone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentRecurrence3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentRecurrence3 {
    type Vtable = IAppointmentRecurrence3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentRecurrence3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x89ff96d9_da4d_4a17_8dd2_1cebc2b5ff9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentRecurrence3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CalendarIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentStore(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStore {
    type Vtable = IAppointmentStore_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentStore {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa461918c_7a47_4d96_96c9_15cd8a05a735);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStore_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateAppointmentCalendarAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAppointmentCalendarAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppointmentCalendarAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, calendarid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppointmentCalendarAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAppointmentInstanceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, instancestarttime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAppointmentInstanceAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentCalendarsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentCalendarsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentCalendarsAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: FindAppointmentCalendarsOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentCalendarsAsyncWithOptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAppointmentsAsyncWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAppointmentsAsyncWithOptions: usize,
    #[cfg(feature = "Foundation")]
    pub FindConflictAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FindConflictAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FindConflictAsyncWithInstanceStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: *mut ::core::ffi::c_void, instancestarttime: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FindConflictAsyncWithInstanceStart: usize,
    #[cfg(feature = "Foundation")]
    pub MoveAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: *mut ::core::ffi::c_void, destinationcalendar: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MoveAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAddAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAddAppointmentAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowReplaceAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowReplaceAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowReplaceAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, appointment: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowReplaceAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowRemoveAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowRemoveAppointmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub ShowRemoveAppointmentWithPlacementAndDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Popups")))]
    ShowRemoveAppointmentWithPlacementAndDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAppointmentDetailsWithDateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, instancestartdate: super::super::Foundation::DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAppointmentDetailsWithDateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowEditNewAppointmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appointment: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowEditNewAppointmentAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindLocalIdsFromRoamingIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, roamingid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindLocalIdsFromRoamingIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentStore2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStore2 {
    type Vtable = IAppointmentStore2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentStore2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25c48c20_1c41_424f_8084_67c1cfe0a854);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStore2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StoreChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StoreChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStoreChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStoreChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CreateAppointmentCalendarInAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, userdataaccountid: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAppointmentCalendarInAccountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentStore3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStore3 {
    type Vtable = IAppointmentStore3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentStore3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4251940b_b078_470a_9a40_c2e01761f72f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStore3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentStoreChange(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStoreChange {
    type Vtable = IAppointmentStoreChange_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentStoreChange {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5a6e035_0a33_3654_8463_b543e90c3b79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChange_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Appointment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AppointmentStoreChangeType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentStoreChange2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStoreChange2 {
    type Vtable = IAppointmentStoreChange2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentStoreChange2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb37d0dce_5211_4402_a608_a96fe70b8ee2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChange2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AppointmentCalendar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentStoreChangeReader(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStoreChangeReader {
    type Vtable = IAppointmentStoreChangeReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentStoreChangeReader {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b2409f1_65f3_42a0_961d_4c209bf30370);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangeReader_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ReadBatchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReadBatchAsync: usize,
    pub AcceptChanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AcceptChangesThrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastchangetoaccept: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentStoreChangeTracker(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStoreChangeTracker {
    type Vtable = IAppointmentStoreChangeTracker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentStoreChangeTracker {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b25f4b1_8ece_4f17_93c8_e6412458fd5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangeTracker_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetChangeReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentStoreChangeTracker2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStoreChangeTracker2 {
    type Vtable = IAppointmentStoreChangeTracker2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentStoreChangeTracker2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb66aaf45_9542_4cf7_8550_eb370e0c08d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangeTracker2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsTracking: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentStoreChangedDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStoreChangedDeferral {
    type Vtable = IAppointmentStoreChangedDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentStoreChangedDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4cb82026_fedb_4bc3_9662_95a9befdf4df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangedDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentStoreChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStoreChangedEventArgs {
    type Vtable = IAppointmentStoreChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentStoreChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2285f8b9_0791_417e_bfea_cc6d41636c8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IAppointmentStoreNotificationTriggerDetails(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAppointmentStoreNotificationTriggerDetails {
    type Vtable = IAppointmentStoreNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IAppointmentStoreNotificationTriggerDetails {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b33cb11_c301_421e_afef_047ecfa76adb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppointmentStoreNotificationTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFindAppointmentsOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFindAppointmentsOptions {
    type Vtable = IFindAppointmentsOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFindAppointmentsOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55f7dc55_9942_3086_82b5_2cb29f64d5f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindAppointmentsOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CalendarIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CalendarIds: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FetchProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FetchProperties: usize,
    pub IncludeHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIncludeHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub MaxCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetMaxCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct Appointment(::windows_core::IUnknown);
impl Appointment {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Appointment, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
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
    pub fn SetStartTime(&self, value: super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetStartTime(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Duration(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetDuration(value)).ok() }
    }
    pub fn Location(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Location(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetLocation(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetLocation(::core::mem::transmute_copy(value))).ok() }
    }
    pub fn Subject(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Subject(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetSubject(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetSubject(::core::mem::transmute_copy(value))).ok() }
    }
    pub fn Details(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Details(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetDetails(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetDetails(::core::mem::transmute_copy(value))).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Reminder(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Reminder(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetReminder<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetReminder(value.try_into_param()?.abi())).ok() }
    }
    pub fn Organizer(&self) -> ::windows_core::Result<AppointmentOrganizer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Organizer(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetOrganizer<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<AppointmentOrganizer>,
    {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetOrganizer(value.into_param().abi())).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Invitees(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<AppointmentInvitee>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Invitees(&mut result__)).from_abi(result__)
        }
    }
    pub fn Recurrence(&self) -> ::windows_core::Result<AppointmentRecurrence> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Recurrence(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetRecurrence<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<AppointmentRecurrence>,
    {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetRecurrence(value.into_param().abi())).ok() }
    }
    pub fn BusyStatus(&self) -> ::windows_core::Result<AppointmentBusyStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BusyStatus(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetBusyStatus(&self, value: AppointmentBusyStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetBusyStatus(value)).ok() }
    }
    pub fn AllDay(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.AllDay(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetAllDay(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetAllDay(value)).ok() }
    }
    pub fn Sensitivity(&self) -> ::windows_core::Result<AppointmentSensitivity> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Sensitivity(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetSensitivity(&self, value: AppointmentSensitivity) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetSensitivity(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Uri(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetUri(value.into_param().abi())).ok() }
    }
    pub fn LocalId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LocalId(&mut result__)).from_abi(result__)
        }
    }
    pub fn CalendarId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CalendarId(&mut result__)).from_abi(result__)
        }
    }
    pub fn RoamingId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.RoamingId(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetRoamingId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetRoamingId(::core::mem::transmute_copy(value))).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn OriginalStartTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OriginalStartTime(&mut result__)).from_abi(result__)
        }
    }
    pub fn IsResponseRequested(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsResponseRequested(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetIsResponseRequested(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetIsResponseRequested(value)).ok() }
    }
    pub fn AllowNewTimeProposal(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.AllowNewTimeProposal(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetAllowNewTimeProposal(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetAllowNewTimeProposal(value)).ok() }
    }
    pub fn OnlineMeetingLink(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OnlineMeetingLink(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetOnlineMeetingLink(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetOnlineMeetingLink(::core::mem::transmute_copy(value))).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ReplyTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ReplyTime(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetReplyTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetReplyTime(value.try_into_param()?.abi())).ok() }
    }
    pub fn UserResponse(&self) -> ::windows_core::Result<AppointmentParticipantResponse> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.UserResponse(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetUserResponse(&self, value: AppointmentParticipantResponse) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetUserResponse(value)).ok() }
    }
    pub fn HasInvitees(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.HasInvitees(&mut result__)).from_abi(result__)
        }
    }
    pub fn IsCanceledMeeting(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsCanceledMeeting(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetIsCanceledMeeting(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetIsCanceledMeeting(value)).ok() }
    }
    pub fn IsOrganizedByUser(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsOrganizedByUser(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetIsOrganizedByUser(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointment2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetIsOrganizedByUser(value)).ok() }
    }
    pub fn ChangeNumber(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::ComInterface::cast::<IAppointment3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ChangeNumber(&mut result__)).from_abi(result__)
        }
    }
    pub fn RemoteChangeNumber(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::ComInterface::cast::<IAppointment3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.RemoteChangeNumber(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetRemoteChangeNumber(&self, value: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointment3>(self)?;
        unsafe { ::windows_core::vcall!(this.SetRemoteChangeNumber(value)).ok() }
    }
    pub fn DetailsKind(&self) -> ::windows_core::Result<AppointmentDetailsKind> {
        let this = &::windows_core::ComInterface::cast::<IAppointment3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DetailsKind(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetDetailsKind(&self, value: AppointmentDetailsKind) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointment3>(self)?;
        unsafe { ::windows_core::vcall!(this.SetDetailsKind(value)).ok() }
    }
}
impl ::windows_core::RuntimeType for Appointment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.Appointment;{dd002f2f-2bdd-4076-90a3-22c275312965})");
}
unsafe impl ::windows_core::Interface for Appointment {
    type Vtable = IAppointment_Vtbl;
}
unsafe impl ::windows_core::ComInterface for Appointment {
    const IID: ::windows_core::GUID = <IAppointment as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for Appointment {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.Appointment";
}
::windows_core::imp::interface_hierarchy!(Appointment, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for Appointment {}
unsafe impl ::core::marker::Sync for Appointment {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentCalendar(::windows_core::IUnknown);
impl AppointmentCalendar {
    #[doc = "Required features: `\"UI\"`"]
    #[cfg(feature = "UI")]
    pub fn DisplayColor(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DisplayColor(&mut result__)).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DisplayName(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetDisplayName(::core::mem::transmute_copy(value))).ok() }
    }
    pub fn LocalId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LocalId(&mut result__)).from_abi(result__)
        }
    }
    pub fn IsHidden(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsHidden(&mut result__)).from_abi(result__)
        }
    }
    pub fn OtherAppReadAccess(&self) -> ::windows_core::Result<AppointmentCalendarOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OtherAppReadAccess(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetOtherAppReadAccess(&self, value: AppointmentCalendarOtherAppReadAccess) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetOtherAppReadAccess(value)).ok() }
    }
    pub fn OtherAppWriteAccess(&self) -> ::windows_core::Result<AppointmentCalendarOtherAppWriteAccess> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OtherAppWriteAccess(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetOtherAppWriteAccess(&self, value: AppointmentCalendarOtherAppWriteAccess) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetOtherAppWriteAccess(value)).ok() }
    }
    pub fn SourceDisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SourceDisplayName(&mut result__)).from_abi(result__)
        }
    }
    pub fn SummaryCardView(&self) -> ::windows_core::Result<AppointmentSummaryCardView> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SummaryCardView(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetSummaryCardView(&self, value: AppointmentSummaryCardView) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetSummaryCardView(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentsAsync(&self, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FindAppointmentsAsync(rangestart, rangelength, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentsAsyncWithOptions<P0>(&self, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, options: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>
    where
        P0: ::windows_core::IntoParam<FindAppointmentsOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FindAppointmentsAsyncWithOptions(rangestart, rangelength, options.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindExceptionsFromMasterAsync(&self, masterlocalid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentException>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FindExceptionsFromMasterAsync(::core::mem::transmute_copy(masterlocalid), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllInstancesAsync(&self, masterlocalid: &::windows_core::HSTRING, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FindAllInstancesAsync(::core::mem::transmute_copy(masterlocalid), rangestart, rangelength, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllInstancesAsyncWithOptions<P0>(&self, masterlocalid: &::windows_core::HSTRING, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, poptions: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>
    where
        P0: ::windows_core::IntoParam<FindAppointmentsOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FindAllInstancesAsyncWithOptions(::core::mem::transmute_copy(masterlocalid), rangestart, rangelength, poptions.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetAppointmentAsync(&self, localid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetAppointmentAsync(::core::mem::transmute_copy(localid), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetAppointmentInstanceAsync(&self, localid: &::windows_core::HSTRING, instancestarttime: super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetAppointmentInstanceAsync(::core::mem::transmute_copy(localid), instancestarttime, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindUnexpandedAppointmentsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FindUnexpandedAppointmentsAsync(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindUnexpandedAppointmentsAsyncWithOptions<P0>(&self, options: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>
    where
        P0: ::windows_core::IntoParam<FindAppointmentsOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FindUnexpandedAppointmentsAsyncWithOptions(options.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DeleteAsync(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SaveAsync(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAppointmentAsync(&self, localid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DeleteAppointmentAsync(::core::mem::transmute_copy(localid), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAppointmentInstanceAsync(&self, localid: &::windows_core::HSTRING, instancestarttime: super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DeleteAppointmentInstanceAsync(::core::mem::transmute_copy(localid), instancestarttime, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAppointmentAsync<P0>(&self, pappointment: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SaveAppointmentAsync(pappointment.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    pub fn SyncManager(&self) -> ::windows_core::Result<AppointmentCalendarSyncManager> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SyncManager(&mut result__)).from_abi(result__)
        }
    }
    pub fn RemoteId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.RemoteId(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetRemoteId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetRemoteId(::core::mem::transmute_copy(value))).ok() }
    }
    #[doc = "Required features: `\"UI\"`"]
    #[cfg(feature = "UI")]
    pub fn SetDisplayColor(&self, value: super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetDisplayColor(value)).ok() }
    }
    pub fn SetIsHidden(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetIsHidden(value)).ok() }
    }
    pub fn UserDataAccountId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.UserDataAccountId(&mut result__)).from_abi(result__)
        }
    }
    pub fn CanCreateOrUpdateAppointments(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CanCreateOrUpdateAppointments(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetCanCreateOrUpdateAppointments(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetCanCreateOrUpdateAppointments(value)).ok() }
    }
    pub fn CanCancelMeetings(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CanCancelMeetings(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetCanCancelMeetings(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetCanCancelMeetings(value)).ok() }
    }
    pub fn CanForwardMeetings(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CanForwardMeetings(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetCanForwardMeetings(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetCanForwardMeetings(value)).ok() }
    }
    pub fn CanProposeNewTimeForMeetings(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CanProposeNewTimeForMeetings(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetCanProposeNewTimeForMeetings(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetCanProposeNewTimeForMeetings(value)).ok() }
    }
    pub fn CanUpdateMeetingResponses(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CanUpdateMeetingResponses(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetCanUpdateMeetingResponses(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetCanUpdateMeetingResponses(value)).ok() }
    }
    pub fn CanNotifyInvitees(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CanNotifyInvitees(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetCanNotifyInvitees(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetCanNotifyInvitees(value)).ok() }
    }
    pub fn MustNofityInvitees(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MustNofityInvitees(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetMustNofityInvitees(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetMustNofityInvitees(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryCreateOrUpdateAppointmentAsync<P0>(&self, appointment: P0, notifyinvitees: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.TryCreateOrUpdateAppointmentAsync(appointment.into_param().abi(), notifyinvitees, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryCancelMeetingAsync<P0>(&self, meeting: P0, subject: &::windows_core::HSTRING, comment: &::windows_core::HSTRING, notifyinvitees: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.TryCancelMeetingAsync(meeting.into_param().abi(), ::core::mem::transmute_copy(subject), ::core::mem::transmute_copy(comment), notifyinvitees, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TryForwardMeetingAsync<P0, P1>(&self, meeting: P0, invitees: P1, subject: &::windows_core::HSTRING, forwardheader: &::windows_core::HSTRING, comment: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<AppointmentInvitee>>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.TryForwardMeetingAsync(meeting.into_param().abi(), invitees.try_into_param()?.abi(), ::core::mem::transmute_copy(subject), ::core::mem::transmute_copy(forwardheader), ::core::mem::transmute_copy(comment), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryProposeNewTimeForMeetingAsync<P0>(&self, meeting: P0, newstarttime: super::super::Foundation::DateTime, newduration: super::super::Foundation::TimeSpan, subject: &::windows_core::HSTRING, comment: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.TryProposeNewTimeForMeetingAsync(meeting.into_param().abi(), newstarttime, newduration, ::core::mem::transmute_copy(subject), ::core::mem::transmute_copy(comment), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryUpdateMeetingResponseAsync<P0>(&self, meeting: P0, response: AppointmentParticipantResponse, subject: &::windows_core::HSTRING, comment: &::windows_core::HSTRING, sendupdate: bool) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.TryUpdateMeetingResponseAsync(meeting.into_param().abi(), response, ::core::mem::transmute_copy(subject), ::core::mem::transmute_copy(comment), sendupdate, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RegisterSyncManagerAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendar3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.RegisterSyncManagerAsync(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppointmentCalendar {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentCalendar;{5273819d-8339-3d4f-a02f-64084452bb5d})");
}
unsafe impl ::windows_core::Interface for AppointmentCalendar {
    type Vtable = IAppointmentCalendar_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentCalendar {
    const IID: ::windows_core::GUID = <IAppointmentCalendar as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentCalendar {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentCalendar";
}
::windows_core::imp::interface_hierarchy!(AppointmentCalendar, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppointmentCalendar {}
unsafe impl ::core::marker::Sync for AppointmentCalendar {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentCalendarSyncManager(::windows_core::IUnknown);
impl AppointmentCalendarSyncManager {
    pub fn Status(&self) -> ::windows_core::Result<AppointmentCalendarSyncStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Status(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LastSuccessfulSyncTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LastSuccessfulSyncTime(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn LastAttemptedSyncTime(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.LastAttemptedSyncTime(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SyncAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SyncAsync(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SyncStatusChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppointmentCalendarSyncManager, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.SyncStatusChanged(handler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSyncStatusChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveSyncStatusChanged(token)).ok() }
    }
    pub fn SetStatus(&self, value: AppointmentCalendarSyncStatus) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendarSyncManager2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetStatus(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastSuccessfulSyncTime(&self, value: super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendarSyncManager2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetLastSuccessfulSyncTime(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetLastAttemptedSyncTime(&self, value: super::super::Foundation::DateTime) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentCalendarSyncManager2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetLastAttemptedSyncTime(value)).ok() }
    }
}
impl ::windows_core::RuntimeType for AppointmentCalendarSyncManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager;{2b21b3a0-4aff-4392-bc5f-5645ffcffb17})");
}
unsafe impl ::windows_core::Interface for AppointmentCalendarSyncManager {
    type Vtable = IAppointmentCalendarSyncManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentCalendarSyncManager {
    const IID: ::windows_core::GUID = <IAppointmentCalendarSyncManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentCalendarSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager";
}
::windows_core::imp::interface_hierarchy!(AppointmentCalendarSyncManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppointmentCalendarSyncManager {}
unsafe impl ::core::marker::Sync for AppointmentCalendarSyncManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentConflictResult(::windows_core::IUnknown);
impl AppointmentConflictResult {
    pub fn Type(&self) -> ::windows_core::Result<AppointmentConflictType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Type(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Date(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Date(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppointmentConflictResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentConflictResult;{d5cdf0be-2f2f-3b7d-af0a-a7e20f3a46e3})");
}
unsafe impl ::windows_core::Interface for AppointmentConflictResult {
    type Vtable = IAppointmentConflictResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentConflictResult {
    const IID: ::windows_core::GUID = <IAppointmentConflictResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentConflictResult {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentConflictResult";
}
::windows_core::imp::interface_hierarchy!(AppointmentConflictResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppointmentConflictResult {}
unsafe impl ::core::marker::Sync for AppointmentConflictResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentException(::windows_core::IUnknown);
impl AppointmentException {
    pub fn Appointment(&self) -> ::windows_core::Result<Appointment> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Appointment(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExceptionProperties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ExceptionProperties(&mut result__)).from_abi(result__)
        }
    }
    pub fn IsDeleted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsDeleted(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppointmentException {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentException;{a2076767-16f6-4bce-9f5a-8600b8019fcb})");
}
unsafe impl ::windows_core::Interface for AppointmentException {
    type Vtable = IAppointmentException_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentException {
    const IID: ::windows_core::GUID = <IAppointmentException as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentException {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentException";
}
::windows_core::imp::interface_hierarchy!(AppointmentException, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppointmentException {}
unsafe impl ::core::marker::Sync for AppointmentException {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentInvitee(::windows_core::IUnknown);
impl AppointmentInvitee {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppointmentInvitee, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Role(&self) -> ::windows_core::Result<AppointmentParticipantRole> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Role(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetRole(&self, value: AppointmentParticipantRole) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetRole(value)).ok() }
    }
    pub fn Response(&self) -> ::windows_core::Result<AppointmentParticipantResponse> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Response(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetResponse(&self, value: AppointmentParticipantResponse) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetResponse(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentParticipant>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DisplayName(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentParticipant>(self)?;
        unsafe { ::windows_core::vcall!(this.SetDisplayName(::core::mem::transmute_copy(value))).ok() }
    }
    pub fn Address(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentParticipant>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Address(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetAddress(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentParticipant>(self)?;
        unsafe { ::windows_core::vcall!(this.SetAddress(::core::mem::transmute_copy(value))).ok() }
    }
}
impl ::windows_core::RuntimeType for AppointmentInvitee {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentInvitee;{13bf0796-9842-495b-b0e7-ef8f79c0701d})");
}
unsafe impl ::windows_core::Interface for AppointmentInvitee {
    type Vtable = IAppointmentInvitee_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentInvitee {
    const IID: ::windows_core::GUID = <IAppointmentInvitee as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentInvitee {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentInvitee";
}
::windows_core::imp::interface_hierarchy!(AppointmentInvitee, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IAppointmentParticipant> for AppointmentInvitee {}
unsafe impl ::core::marker::Send for AppointmentInvitee {}
unsafe impl ::core::marker::Sync for AppointmentInvitee {}
pub struct AppointmentManager;
impl AppointmentManager {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAddAppointmentAsync<P0>(appointment: P0, selection: super::super::Foundation::Rect) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowAddAppointmentAsync(appointment.into_param().abi(), selection, &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"UI_Popups\"`"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowAddAppointmentWithPlacementAsync<P0>(appointment: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowAddAppointmentWithPlacementAsync(appointment.into_param().abi(), selection, preferredplacement, &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowReplaceAppointmentAsync<P0>(appointmentid: &::windows_core::HSTRING, appointment: P0, selection: super::super::Foundation::Rect) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowReplaceAppointmentAsync(::core::mem::transmute_copy(appointmentid), appointment.into_param().abi(), selection, &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"UI_Popups\"`"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowReplaceAppointmentWithPlacementAsync<P0>(appointmentid: &::windows_core::HSTRING, appointment: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowReplaceAppointmentWithPlacementAsync(::core::mem::transmute_copy(appointmentid), appointment.into_param().abi(), selection, preferredplacement, &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"UI_Popups\"`"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowReplaceAppointmentWithPlacementAndDateAsync<P0>(appointmentid: &::windows_core::HSTRING, appointment: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowReplaceAppointmentWithPlacementAndDateAsync(::core::mem::transmute_copy(appointmentid), appointment.into_param().abi(), selection, preferredplacement, instancestartdate, &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowRemoveAppointmentAsync(appointmentid: &::windows_core::HSTRING, selection: super::super::Foundation::Rect) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowRemoveAppointmentAsync(::core::mem::transmute_copy(appointmentid), selection, &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"UI_Popups\"`"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowRemoveAppointmentWithPlacementAsync(appointmentid: &::windows_core::HSTRING, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowRemoveAppointmentWithPlacementAsync(::core::mem::transmute_copy(appointmentid), selection, preferredplacement, &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"UI_Popups\"`"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowRemoveAppointmentWithPlacementAndDateAsync(appointmentid: &::windows_core::HSTRING, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowRemoveAppointmentWithPlacementAndDateAsync(::core::mem::transmute_copy(appointmentid), selection, preferredplacement, instancestartdate, &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowTimeFrameAsync(timetoshow: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAppointmentManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowTimeFrameAsync(timetoshow, duration, &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAppointmentDetailsAsync(appointmentid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowAppointmentDetailsAsync(::core::mem::transmute_copy(appointmentid), &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAppointmentDetailsWithDateAsync(appointmentid: &::windows_core::HSTRING, instancestartdate: super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowAppointmentDetailsWithDateAsync(::core::mem::transmute_copy(appointmentid), instancestartdate, &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowEditNewAppointmentAsync<P0>(appointment: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowEditNewAppointmentAsync(appointment.into_param().abi(), &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(options: AppointmentStoreAccessType) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppointmentStore>> {
        Self::IAppointmentManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.RequestStoreAsync(options, &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn GetForUser<P0>(user: P0) -> ::windows_core::Result<AppointmentManagerForUser>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IAppointmentManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetForUser(user.into_param().abi(), &mut result__)).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppointmentManagerStatics<R, F: FnOnce(&IAppointmentManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppointmentManager, IAppointmentManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppointmentManagerStatics2<R, F: FnOnce(&IAppointmentManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppointmentManager, IAppointmentManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppointmentManagerStatics3<R, F: FnOnce(&IAppointmentManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppointmentManager, IAppointmentManagerStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for AppointmentManager {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentManagerForUser(::windows_core::IUnknown);
impl AppointmentManagerForUser {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAddAppointmentAsync<P0>(&self, appointment: P0, selection: super::super::Foundation::Rect) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowAddAppointmentAsync(appointment.into_param().abi(), selection, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"UI_Popups\"`"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowAddAppointmentWithPlacementAsync<P0>(&self, appointment: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowAddAppointmentWithPlacementAsync(appointment.into_param().abi(), selection, preferredplacement, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowReplaceAppointmentAsync<P0>(&self, appointmentid: &::windows_core::HSTRING, appointment: P0, selection: super::super::Foundation::Rect) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowReplaceAppointmentAsync(::core::mem::transmute_copy(appointmentid), appointment.into_param().abi(), selection, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"UI_Popups\"`"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowReplaceAppointmentWithPlacementAsync<P0>(&self, appointmentid: &::windows_core::HSTRING, appointment: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowReplaceAppointmentWithPlacementAsync(::core::mem::transmute_copy(appointmentid), appointment.into_param().abi(), selection, preferredplacement, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"UI_Popups\"`"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowReplaceAppointmentWithPlacementAndDateAsync<P0>(&self, appointmentid: &::windows_core::HSTRING, appointment: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowReplaceAppointmentWithPlacementAndDateAsync(::core::mem::transmute_copy(appointmentid), appointment.into_param().abi(), selection, preferredplacement, instancestartdate, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowRemoveAppointmentAsync(&self, appointmentid: &::windows_core::HSTRING, selection: super::super::Foundation::Rect) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowRemoveAppointmentAsync(::core::mem::transmute_copy(appointmentid), selection, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"UI_Popups\"`"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowRemoveAppointmentWithPlacementAsync(&self, appointmentid: &::windows_core::HSTRING, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowRemoveAppointmentWithPlacementAsync(::core::mem::transmute_copy(appointmentid), selection, preferredplacement, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"UI_Popups\"`"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowRemoveAppointmentWithPlacementAndDateAsync(&self, appointmentid: &::windows_core::HSTRING, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowRemoveAppointmentWithPlacementAndDateAsync(::core::mem::transmute_copy(appointmentid), selection, preferredplacement, instancestartdate, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowTimeFrameAsync(&self, timetoshow: super::super::Foundation::DateTime, duration: super::super::Foundation::TimeSpan) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowTimeFrameAsync(timetoshow, duration, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAppointmentDetailsAsync(&self, appointmentid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowAppointmentDetailsAsync(::core::mem::transmute_copy(appointmentid), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAppointmentDetailsWithDateAsync(&self, appointmentid: &::windows_core::HSTRING, instancestartdate: super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowAppointmentDetailsWithDateAsync(::core::mem::transmute_copy(appointmentid), instancestartdate, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowEditNewAppointmentAsync<P0>(&self, appointment: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowEditNewAppointmentAsync(appointment.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(&self, options: AppointmentStoreAccessType) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppointmentStore>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.RequestStoreAsync(options, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows_core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.User(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppointmentManagerForUser {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentManagerForUser;{70261423-73cc-4660-b318-b01365302a03})");
}
unsafe impl ::windows_core::Interface for AppointmentManagerForUser {
    type Vtable = IAppointmentManagerForUser_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentManagerForUser {
    const IID: ::windows_core::GUID = <IAppointmentManagerForUser as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentManagerForUser";
}
::windows_core::imp::interface_hierarchy!(AppointmentManagerForUser, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppointmentManagerForUser {}
unsafe impl ::core::marker::Sync for AppointmentManagerForUser {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentOrganizer(::windows_core::IUnknown);
impl AppointmentOrganizer {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppointmentOrganizer, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DisplayName(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetDisplayName(::core::mem::transmute_copy(value))).ok() }
    }
    pub fn Address(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Address(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetAddress(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetAddress(::core::mem::transmute_copy(value))).ok() }
    }
}
impl ::windows_core::RuntimeType for AppointmentOrganizer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentOrganizer;{615e2902-9718-467b-83fb-b293a19121de})");
}
unsafe impl ::windows_core::Interface for AppointmentOrganizer {
    type Vtable = IAppointmentParticipant_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentOrganizer {
    const IID: ::windows_core::GUID = <IAppointmentParticipant as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentOrganizer {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentOrganizer";
}
::windows_core::imp::interface_hierarchy!(AppointmentOrganizer, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IAppointmentParticipant> for AppointmentOrganizer {}
unsafe impl ::core::marker::Send for AppointmentOrganizer {}
unsafe impl ::core::marker::Sync for AppointmentOrganizer {}
pub struct AppointmentProperties;
impl AppointmentProperties {
    pub fn Subject() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Subject(&mut result__)).from_abi(result__)
        })
    }
    pub fn Location() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Location(&mut result__)).from_abi(result__)
        })
    }
    pub fn StartTime() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.StartTime(&mut result__)).from_abi(result__)
        })
    }
    pub fn Duration() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Duration(&mut result__)).from_abi(result__)
        })
    }
    pub fn Reminder() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Reminder(&mut result__)).from_abi(result__)
        })
    }
    pub fn BusyStatus() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BusyStatus(&mut result__)).from_abi(result__)
        })
    }
    pub fn Sensitivity() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Sensitivity(&mut result__)).from_abi(result__)
        })
    }
    pub fn OriginalStartTime() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OriginalStartTime(&mut result__)).from_abi(result__)
        })
    }
    pub fn IsResponseRequested() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsResponseRequested(&mut result__)).from_abi(result__)
        })
    }
    pub fn AllowNewTimeProposal() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.AllowNewTimeProposal(&mut result__)).from_abi(result__)
        })
    }
    pub fn AllDay() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.AllDay(&mut result__)).from_abi(result__)
        })
    }
    pub fn Details() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Details(&mut result__)).from_abi(result__)
        })
    }
    pub fn OnlineMeetingLink() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OnlineMeetingLink(&mut result__)).from_abi(result__)
        })
    }
    pub fn ReplyTime() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ReplyTime(&mut result__)).from_abi(result__)
        })
    }
    pub fn Organizer() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Organizer(&mut result__)).from_abi(result__)
        })
    }
    pub fn UserResponse() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.UserResponse(&mut result__)).from_abi(result__)
        })
    }
    pub fn HasInvitees() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.HasInvitees(&mut result__)).from_abi(result__)
        })
    }
    pub fn IsCanceledMeeting() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsCanceledMeeting(&mut result__)).from_abi(result__)
        })
    }
    pub fn IsOrganizedByUser() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsOrganizedByUser(&mut result__)).from_abi(result__)
        })
    }
    pub fn Recurrence() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Recurrence(&mut result__)).from_abi(result__)
        })
    }
    pub fn Uri() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Uri(&mut result__)).from_abi(result__)
        })
    }
    pub fn Invitees() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Invitees(&mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DefaultProperties() -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        Self::IAppointmentPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DefaultProperties(&mut result__)).from_abi(result__)
        })
    }
    pub fn ChangeNumber() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ChangeNumber(&mut result__)).from_abi(result__)
        })
    }
    pub fn RemoteChangeNumber() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.RemoteChangeNumber(&mut result__)).from_abi(result__)
        })
    }
    pub fn DetailsKind() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IAppointmentPropertiesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DetailsKind(&mut result__)).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAppointmentPropertiesStatics<R, F: FnOnce(&IAppointmentPropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppointmentProperties, IAppointmentPropertiesStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAppointmentPropertiesStatics2<R, F: FnOnce(&IAppointmentPropertiesStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppointmentProperties, IAppointmentPropertiesStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for AppointmentProperties {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentProperties";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentRecurrence(::windows_core::IUnknown);
impl AppointmentRecurrence {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AppointmentRecurrence, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Unit(&self) -> ::windows_core::Result<AppointmentRecurrenceUnit> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Unit(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetUnit(&self, value: AppointmentRecurrenceUnit) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetUnit(value)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Occurrences(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Occurrences(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetOccurrences<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetOccurrences(value.try_into_param()?.abi())).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Until(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Until(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn SetUntil<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::DateTime>>,
    {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetUntil(value.try_into_param()?.abi())).ok() }
    }
    pub fn Interval(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Interval(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetInterval(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetInterval(value)).ok() }
    }
    pub fn DaysOfWeek(&self) -> ::windows_core::Result<AppointmentDaysOfWeek> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DaysOfWeek(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetDaysOfWeek(&self, value: AppointmentDaysOfWeek) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetDaysOfWeek(value)).ok() }
    }
    pub fn WeekOfMonth(&self) -> ::windows_core::Result<AppointmentWeekOfMonth> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.WeekOfMonth(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetWeekOfMonth(&self, value: AppointmentWeekOfMonth) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetWeekOfMonth(value)).ok() }
    }
    pub fn Month(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Month(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetMonth(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetMonth(value)).ok() }
    }
    pub fn Day(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Day(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetDay(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetDay(value)).ok() }
    }
    pub fn RecurrenceType(&self) -> ::windows_core::Result<RecurrenceType> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentRecurrence2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.RecurrenceType(&mut result__)).from_abi(result__)
        }
    }
    pub fn TimeZone(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentRecurrence2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.TimeZone(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetTimeZone(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentRecurrence2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetTimeZone(::core::mem::transmute_copy(value))).ok() }
    }
    pub fn CalendarIdentifier(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentRecurrence3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CalendarIdentifier(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppointmentRecurrence {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentRecurrence;{d87b3e83-15a6-487b-b959-0c361e60e954})");
}
unsafe impl ::windows_core::Interface for AppointmentRecurrence {
    type Vtable = IAppointmentRecurrence_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentRecurrence {
    const IID: ::windows_core::GUID = <IAppointmentRecurrence as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentRecurrence {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentRecurrence";
}
::windows_core::imp::interface_hierarchy!(AppointmentRecurrence, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppointmentRecurrence {}
unsafe impl ::core::marker::Sync for AppointmentRecurrence {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentStore(::windows_core::IUnknown);
impl AppointmentStore {
    pub fn ChangeTracker(&self) -> ::windows_core::Result<AppointmentStoreChangeTracker> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ChangeTracker(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAppointmentCalendarAsync(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateAppointmentCalendarAsync(::core::mem::transmute_copy(name), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetAppointmentCalendarAsync(&self, calendarid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetAppointmentCalendarAsync(::core::mem::transmute_copy(calendarid), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetAppointmentAsync(&self, localid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetAppointmentAsync(::core::mem::transmute_copy(localid), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetAppointmentInstanceAsync(&self, localid: &::windows_core::HSTRING, instancestarttime: super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<Appointment>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetAppointmentInstanceAsync(::core::mem::transmute_copy(localid), instancestarttime, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentCalendarsAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentCalendar>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FindAppointmentCalendarsAsync(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentCalendarsAsyncWithOptions(&self, options: FindAppointmentCalendarsOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentCalendar>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FindAppointmentCalendarsAsyncWithOptions(options, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentsAsync(&self, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FindAppointmentsAsync(rangestart, rangelength, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAppointmentsAsyncWithOptions<P0>(&self, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan, options: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Appointment>>>
    where
        P0: ::windows_core::IntoParam<FindAppointmentsOptions>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FindAppointmentsAsyncWithOptions(rangestart, rangelength, options.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn FindConflictAsync<P0>(&self, appointment: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppointmentConflictResult>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FindConflictAsync(appointment.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn FindConflictAsyncWithInstanceStart<P0>(&self, appointment: P0, instancestarttime: super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppointmentConflictResult>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FindConflictAsyncWithInstanceStart(appointment.into_param().abi(), instancestarttime, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MoveAppointmentAsync<P0, P1>(&self, appointment: P0, destinationcalendar: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<Appointment>,
        P1: ::windows_core::IntoParam<AppointmentCalendar>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MoveAppointmentAsync(appointment.into_param().abi(), destinationcalendar.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAddAppointmentAsync<P0>(&self, appointment: P0, selection: super::super::Foundation::Rect) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowAddAppointmentAsync(appointment.into_param().abi(), selection, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowReplaceAppointmentAsync<P0>(&self, localid: &::windows_core::HSTRING, appointment: P0, selection: super::super::Foundation::Rect) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowReplaceAppointmentAsync(::core::mem::transmute_copy(localid), appointment.into_param().abi(), selection, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"UI_Popups\"`"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowReplaceAppointmentWithPlacementAndDateAsync<P0>(&self, localid: &::windows_core::HSTRING, appointment: P0, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowReplaceAppointmentWithPlacementAndDateAsync(::core::mem::transmute_copy(localid), appointment.into_param().abi(), selection, preferredplacement, instancestartdate, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowRemoveAppointmentAsync(&self, localid: &::windows_core::HSTRING, selection: super::super::Foundation::Rect) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowRemoveAppointmentAsync(::core::mem::transmute_copy(localid), selection, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"UI_Popups\"`"]
    #[cfg(all(feature = "Foundation", feature = "UI_Popups"))]
    pub fn ShowRemoveAppointmentWithPlacementAndDateAsync(&self, localid: &::windows_core::HSTRING, selection: super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement, instancestartdate: super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowRemoveAppointmentWithPlacementAndDateAsync(::core::mem::transmute_copy(localid), selection, preferredplacement, instancestartdate, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAppointmentDetailsAsync(&self, localid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowAppointmentDetailsAsync(::core::mem::transmute_copy(localid), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAppointmentDetailsWithDateAsync(&self, localid: &::windows_core::HSTRING, instancestartdate: super::super::Foundation::DateTime) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowAppointmentDetailsWithDateAsync(::core::mem::transmute_copy(localid), instancestartdate, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ShowEditNewAppointmentAsync<P0>(&self, appointment: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<Appointment>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ShowEditNewAppointmentAsync(appointment.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindLocalIdsFromRoamingIdAsync(&self, roamingid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FindLocalIdsFromRoamingIdAsync(::core::mem::transmute_copy(roamingid), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn StoreChanged<P0>(&self, phandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AppointmentStore, AppointmentStoreChangedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<IAppointmentStore2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.StoreChanged(phandler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStoreChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentStore2>(self)?;
        unsafe { ::windows_core::vcall!(this.RemoveStoreChanged(token)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAppointmentCalendarInAccountAsync(&self, name: &::windows_core::HSTRING, userdataaccountid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<AppointmentCalendar>> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentStore2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateAppointmentCalendarInAccountAsync(::core::mem::transmute_copy(name), ::core::mem::transmute_copy(userdataaccountid), &mut result__)).from_abi(result__)
        }
    }
    pub fn GetChangeTracker(&self, identity: &::windows_core::HSTRING) -> ::windows_core::Result<AppointmentStoreChangeTracker> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentStore3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetChangeTracker(::core::mem::transmute_copy(identity), &mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppointmentStore {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStore;{a461918c-7a47-4d96-96c9-15cd8a05a735})");
}
unsafe impl ::windows_core::Interface for AppointmentStore {
    type Vtable = IAppointmentStore_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentStore {
    const IID: ::windows_core::GUID = <IAppointmentStore as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentStore {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStore";
}
::windows_core::imp::interface_hierarchy!(AppointmentStore, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppointmentStore {}
unsafe impl ::core::marker::Sync for AppointmentStore {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentStoreChange(::windows_core::IUnknown);
impl AppointmentStoreChange {
    pub fn Appointment(&self) -> ::windows_core::Result<Appointment> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Appointment(&mut result__)).from_abi(result__)
        }
    }
    pub fn ChangeType(&self) -> ::windows_core::Result<AppointmentStoreChangeType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ChangeType(&mut result__)).from_abi(result__)
        }
    }
    pub fn AppointmentCalendar(&self) -> ::windows_core::Result<AppointmentCalendar> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentStoreChange2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.AppointmentCalendar(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppointmentStoreChange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChange;{a5a6e035-0a33-3654-8463-b543e90c3b79})");
}
unsafe impl ::windows_core::Interface for AppointmentStoreChange {
    type Vtable = IAppointmentStoreChange_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentStoreChange {
    const IID: ::windows_core::GUID = <IAppointmentStoreChange as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentStoreChange {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChange";
}
::windows_core::imp::interface_hierarchy!(AppointmentStoreChange, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppointmentStoreChange {}
unsafe impl ::core::marker::Sync for AppointmentStoreChange {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentStoreChangeReader(::windows_core::IUnknown);
impl AppointmentStoreChangeReader {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReadBatchAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AppointmentStoreChange>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ReadBatchAsync(&mut result__)).from_abi(result__)
        }
    }
    pub fn AcceptChanges(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.AcceptChanges()).ok() }
    }
    pub fn AcceptChangesThrough<P0>(&self, lastchangetoaccept: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<AppointmentStoreChange>,
    {
        let this = self;
        unsafe { ::windows_core::vcall!(this.AcceptChangesThrough(lastchangetoaccept.into_param().abi())).ok() }
    }
}
impl ::windows_core::RuntimeType for AppointmentStoreChangeReader {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChangeReader;{8b2409f1-65f3-42a0-961d-4c209bf30370})");
}
unsafe impl ::windows_core::Interface for AppointmentStoreChangeReader {
    type Vtable = IAppointmentStoreChangeReader_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentStoreChangeReader {
    const IID: ::windows_core::GUID = <IAppointmentStoreChangeReader as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentStoreChangeReader {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangeReader";
}
::windows_core::imp::interface_hierarchy!(AppointmentStoreChangeReader, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppointmentStoreChangeReader {}
unsafe impl ::core::marker::Sync for AppointmentStoreChangeReader {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentStoreChangeTracker(::windows_core::IUnknown);
impl AppointmentStoreChangeTracker {
    pub fn GetChangeReader(&self) -> ::windows_core::Result<AppointmentStoreChangeReader> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetChangeReader(&mut result__)).from_abi(result__)
        }
    }
    pub fn Enable(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Enable()).ok() }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Reset()).ok() }
    }
    pub fn IsTracking(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAppointmentStoreChangeTracker2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsTracking(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppointmentStoreChangeTracker {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChangeTracker;{1b25f4b1-8ece-4f17-93c8-e6412458fd5c})");
}
unsafe impl ::windows_core::Interface for AppointmentStoreChangeTracker {
    type Vtable = IAppointmentStoreChangeTracker_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentStoreChangeTracker {
    const IID: ::windows_core::GUID = <IAppointmentStoreChangeTracker as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentStoreChangeTracker {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangeTracker";
}
::windows_core::imp::interface_hierarchy!(AppointmentStoreChangeTracker, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppointmentStoreChangeTracker {}
unsafe impl ::core::marker::Sync for AppointmentStoreChangeTracker {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentStoreChangedDeferral(::windows_core::IUnknown);
impl AppointmentStoreChangedDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.Complete()).ok() }
    }
}
impl ::windows_core::RuntimeType for AppointmentStoreChangedDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChangedDeferral;{4cb82026-fedb-4bc3-9662-95a9befdf4df})");
}
unsafe impl ::windows_core::Interface for AppointmentStoreChangedDeferral {
    type Vtable = IAppointmentStoreChangedDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentStoreChangedDeferral {
    const IID: ::windows_core::GUID = <IAppointmentStoreChangedDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentStoreChangedDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangedDeferral";
}
::windows_core::imp::interface_hierarchy!(AppointmentStoreChangedDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppointmentStoreChangedDeferral {}
unsafe impl ::core::marker::Sync for AppointmentStoreChangedDeferral {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentStoreChangedEventArgs(::windows_core::IUnknown);
impl AppointmentStoreChangedEventArgs {
    pub fn GetDeferral(&self) -> ::windows_core::Result<AppointmentStoreChangedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetDeferral(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for AppointmentStoreChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreChangedEventArgs;{2285f8b9-0791-417e-bfea-cc6d41636c8c})");
}
unsafe impl ::windows_core::Interface for AppointmentStoreChangedEventArgs {
    type Vtable = IAppointmentStoreChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentStoreChangedEventArgs {
    const IID: ::windows_core::GUID = <IAppointmentStoreChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentStoreChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AppointmentStoreChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppointmentStoreChangedEventArgs {}
unsafe impl ::core::marker::Sync for AppointmentStoreChangedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AppointmentStoreNotificationTriggerDetails(::windows_core::IUnknown);
impl AppointmentStoreNotificationTriggerDetails {}
impl ::windows_core::RuntimeType for AppointmentStoreNotificationTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.AppointmentStoreNotificationTriggerDetails;{9b33cb11-c301-421e-afef-047ecfa76adb})");
}
unsafe impl ::windows_core::Interface for AppointmentStoreNotificationTriggerDetails {
    type Vtable = IAppointmentStoreNotificationTriggerDetails_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AppointmentStoreNotificationTriggerDetails {
    const IID: ::windows_core::GUID = <IAppointmentStoreNotificationTriggerDetails as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AppointmentStoreNotificationTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.AppointmentStoreNotificationTriggerDetails";
}
::windows_core::imp::interface_hierarchy!(AppointmentStoreNotificationTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AppointmentStoreNotificationTriggerDetails {}
unsafe impl ::core::marker::Sync for AppointmentStoreNotificationTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FindAppointmentsOptions(::windows_core::IUnknown);
impl FindAppointmentsOptions {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FindAppointmentsOptions, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CalendarIds(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CalendarIds(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FetchProperties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FetchProperties(&mut result__)).from_abi(result__)
        }
    }
    pub fn IncludeHidden(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IncludeHidden(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetIncludeHidden(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetIncludeHidden(value)).ok() }
    }
    pub fn MaxCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MaxCount(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetMaxCount(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.SetMaxCount(value)).ok() }
    }
}
impl ::windows_core::RuntimeType for FindAppointmentsOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Appointments.FindAppointmentsOptions;{55f7dc55-9942-3086-82b5-2cb29f64d5f5})");
}
unsafe impl ::windows_core::Interface for FindAppointmentsOptions {
    type Vtable = IFindAppointmentsOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FindAppointmentsOptions {
    const IID: ::windows_core::GUID = <IFindAppointmentsOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FindAppointmentsOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Appointments.FindAppointmentsOptions";
}
::windows_core::imp::interface_hierarchy!(FindAppointmentsOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for FindAppointmentsOptions {}
unsafe impl ::core::marker::Sync for FindAppointmentsOptions {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppointmentBusyStatus(pub i32);
impl AppointmentBusyStatus {
    pub const Busy: Self = Self(0i32);
    pub const Tentative: Self = Self(1i32);
    pub const Free: Self = Self(2i32);
    pub const OutOfOffice: Self = Self(3i32);
    pub const WorkingElsewhere: Self = Self(4i32);
}
impl ::core::marker::Copy for AppointmentBusyStatus {}
impl ::core::clone::Clone for AppointmentBusyStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentBusyStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppointmentBusyStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppointmentBusyStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentBusyStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppointmentBusyStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentBusyStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppointmentCalendarOtherAppReadAccess(pub i32);
impl AppointmentCalendarOtherAppReadAccess {
    pub const SystemOnly: Self = Self(0i32);
    pub const Limited: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
    pub const None: Self = Self(3i32);
}
impl ::core::marker::Copy for AppointmentCalendarOtherAppReadAccess {}
impl ::core::clone::Clone for AppointmentCalendarOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentCalendarOtherAppReadAccess {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppointmentCalendarOtherAppReadAccess {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppointmentCalendarOtherAppReadAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarOtherAppReadAccess").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppointmentCalendarOtherAppReadAccess {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentCalendarOtherAppReadAccess;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppointmentCalendarOtherAppWriteAccess(pub i32);
impl AppointmentCalendarOtherAppWriteAccess {
    pub const None: Self = Self(0i32);
    pub const SystemOnly: Self = Self(1i32);
    pub const Limited: Self = Self(2i32);
}
impl ::core::marker::Copy for AppointmentCalendarOtherAppWriteAccess {}
impl ::core::clone::Clone for AppointmentCalendarOtherAppWriteAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentCalendarOtherAppWriteAccess {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppointmentCalendarOtherAppWriteAccess {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppointmentCalendarOtherAppWriteAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarOtherAppWriteAccess").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppointmentCalendarOtherAppWriteAccess {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentCalendarOtherAppWriteAccess;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppointmentCalendarSyncStatus(pub i32);
impl AppointmentCalendarSyncStatus {
    pub const Idle: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const UpToDate: Self = Self(2i32);
    pub const AuthenticationError: Self = Self(3i32);
    pub const PolicyError: Self = Self(4i32);
    pub const UnknownError: Self = Self(5i32);
    pub const ManualAccountRemovalRequired: Self = Self(6i32);
}
impl ::core::marker::Copy for AppointmentCalendarSyncStatus {}
impl ::core::clone::Clone for AppointmentCalendarSyncStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentCalendarSyncStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppointmentCalendarSyncStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppointmentCalendarSyncStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentCalendarSyncStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppointmentCalendarSyncStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentCalendarSyncStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppointmentConflictType(pub i32);
impl AppointmentConflictType {
    pub const None: Self = Self(0i32);
    pub const Adjacent: Self = Self(1i32);
    pub const Overlap: Self = Self(2i32);
}
impl ::core::marker::Copy for AppointmentConflictType {}
impl ::core::clone::Clone for AppointmentConflictType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentConflictType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppointmentConflictType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppointmentConflictType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentConflictType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppointmentConflictType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentConflictType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppointmentDaysOfWeek(pub u32);
impl AppointmentDaysOfWeek {
    pub const None: Self = Self(0u32);
    pub const Sunday: Self = Self(1u32);
    pub const Monday: Self = Self(2u32);
    pub const Tuesday: Self = Self(4u32);
    pub const Wednesday: Self = Self(8u32);
    pub const Thursday: Self = Self(16u32);
    pub const Friday: Self = Self(32u32);
    pub const Saturday: Self = Self(64u32);
}
impl ::core::marker::Copy for AppointmentDaysOfWeek {}
impl ::core::clone::Clone for AppointmentDaysOfWeek {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentDaysOfWeek {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppointmentDaysOfWeek {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppointmentDaysOfWeek {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentDaysOfWeek").field(&self.0).finish()
    }
}
impl AppointmentDaysOfWeek {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for AppointmentDaysOfWeek {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AppointmentDaysOfWeek {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AppointmentDaysOfWeek {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AppointmentDaysOfWeek {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AppointmentDaysOfWeek {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for AppointmentDaysOfWeek {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentDaysOfWeek;u4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppointmentDetailsKind(pub i32);
impl AppointmentDetailsKind {
    pub const PlainText: Self = Self(0i32);
    pub const Html: Self = Self(1i32);
}
impl ::core::marker::Copy for AppointmentDetailsKind {}
impl ::core::clone::Clone for AppointmentDetailsKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentDetailsKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppointmentDetailsKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppointmentDetailsKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentDetailsKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppointmentDetailsKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentDetailsKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppointmentParticipantResponse(pub i32);
impl AppointmentParticipantResponse {
    pub const None: Self = Self(0i32);
    pub const Tentative: Self = Self(1i32);
    pub const Accepted: Self = Self(2i32);
    pub const Declined: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
}
impl ::core::marker::Copy for AppointmentParticipantResponse {}
impl ::core::clone::Clone for AppointmentParticipantResponse {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentParticipantResponse {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppointmentParticipantResponse {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppointmentParticipantResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentParticipantResponse").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppointmentParticipantResponse {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentParticipantResponse;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppointmentParticipantRole(pub i32);
impl AppointmentParticipantRole {
    pub const RequiredAttendee: Self = Self(0i32);
    pub const OptionalAttendee: Self = Self(1i32);
    pub const Resource: Self = Self(2i32);
}
impl ::core::marker::Copy for AppointmentParticipantRole {}
impl ::core::clone::Clone for AppointmentParticipantRole {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentParticipantRole {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppointmentParticipantRole {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppointmentParticipantRole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentParticipantRole").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppointmentParticipantRole {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentParticipantRole;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppointmentRecurrenceUnit(pub i32);
impl AppointmentRecurrenceUnit {
    pub const Daily: Self = Self(0i32);
    pub const Weekly: Self = Self(1i32);
    pub const Monthly: Self = Self(2i32);
    pub const MonthlyOnDay: Self = Self(3i32);
    pub const Yearly: Self = Self(4i32);
    pub const YearlyOnDay: Self = Self(5i32);
}
impl ::core::marker::Copy for AppointmentRecurrenceUnit {}
impl ::core::clone::Clone for AppointmentRecurrenceUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentRecurrenceUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppointmentRecurrenceUnit {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppointmentRecurrenceUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentRecurrenceUnit").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppointmentRecurrenceUnit {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentRecurrenceUnit;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppointmentSensitivity(pub i32);
impl AppointmentSensitivity {
    pub const Public: Self = Self(0i32);
    pub const Private: Self = Self(1i32);
}
impl ::core::marker::Copy for AppointmentSensitivity {}
impl ::core::clone::Clone for AppointmentSensitivity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentSensitivity {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppointmentSensitivity {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppointmentSensitivity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentSensitivity").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppointmentSensitivity {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentSensitivity;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppointmentStoreAccessType(pub i32);
impl AppointmentStoreAccessType {
    pub const AppCalendarsReadWrite: Self = Self(0i32);
    pub const AllCalendarsReadOnly: Self = Self(1i32);
    pub const AllCalendarsReadWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for AppointmentStoreAccessType {}
impl ::core::clone::Clone for AppointmentStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentStoreAccessType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppointmentStoreAccessType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppointmentStoreAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreAccessType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppointmentStoreAccessType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentStoreAccessType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppointmentStoreChangeType(pub i32);
impl AppointmentStoreChangeType {
    pub const AppointmentCreated: Self = Self(0i32);
    pub const AppointmentModified: Self = Self(1i32);
    pub const AppointmentDeleted: Self = Self(2i32);
    pub const ChangeTrackingLost: Self = Self(3i32);
    pub const CalendarCreated: Self = Self(4i32);
    pub const CalendarModified: Self = Self(5i32);
    pub const CalendarDeleted: Self = Self(6i32);
}
impl ::core::marker::Copy for AppointmentStoreChangeType {}
impl ::core::clone::Clone for AppointmentStoreChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentStoreChangeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppointmentStoreChangeType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppointmentStoreChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentStoreChangeType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppointmentStoreChangeType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentStoreChangeType;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppointmentSummaryCardView(pub i32);
impl AppointmentSummaryCardView {
    pub const System: Self = Self(0i32);
    pub const App: Self = Self(1i32);
}
impl ::core::marker::Copy for AppointmentSummaryCardView {}
impl ::core::clone::Clone for AppointmentSummaryCardView {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentSummaryCardView {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppointmentSummaryCardView {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppointmentSummaryCardView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentSummaryCardView").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppointmentSummaryCardView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentSummaryCardView;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppointmentWeekOfMonth(pub i32);
impl AppointmentWeekOfMonth {
    pub const First: Self = Self(0i32);
    pub const Second: Self = Self(1i32);
    pub const Third: Self = Self(2i32);
    pub const Fourth: Self = Self(3i32);
    pub const Last: Self = Self(4i32);
}
impl ::core::marker::Copy for AppointmentWeekOfMonth {}
impl ::core::clone::Clone for AppointmentWeekOfMonth {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppointmentWeekOfMonth {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AppointmentWeekOfMonth {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AppointmentWeekOfMonth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppointmentWeekOfMonth").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AppointmentWeekOfMonth {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.AppointmentWeekOfMonth;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FindAppointmentCalendarsOptions(pub u32);
impl FindAppointmentCalendarsOptions {
    pub const None: Self = Self(0u32);
    pub const IncludeHidden: Self = Self(1u32);
}
impl ::core::marker::Copy for FindAppointmentCalendarsOptions {}
impl ::core::clone::Clone for FindAppointmentCalendarsOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FindAppointmentCalendarsOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FindAppointmentCalendarsOptions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FindAppointmentCalendarsOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindAppointmentCalendarsOptions").field(&self.0).finish()
    }
}
impl FindAppointmentCalendarsOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FindAppointmentCalendarsOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FindAppointmentCalendarsOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FindAppointmentCalendarsOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for FindAppointmentCalendarsOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.FindAppointmentCalendarsOptions;u4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RecurrenceType(pub i32);
impl RecurrenceType {
    pub const Master: Self = Self(0i32);
    pub const Instance: Self = Self(1i32);
    pub const ExceptionInstance: Self = Self(2i32);
}
impl ::core::marker::Copy for RecurrenceType {}
impl ::core::clone::Clone for RecurrenceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RecurrenceType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RecurrenceType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RecurrenceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RecurrenceType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RecurrenceType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Appointments.RecurrenceType;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
