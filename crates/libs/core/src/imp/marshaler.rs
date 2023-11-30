// TODO: maybe just scrap this and use CoCreateFreeThreadedMarshaler directly even though it theoretically slower 
// its not needed very often. But that has its own problems since the inner would need to be kept alive by every 
// agile object... so this is probably the best we can do. 

use super::{RefCount, E_OUTOFMEMORY, E_POINTER, S_OK};
use crate::{ComInterface, IUnknown, IUnknown_Vtbl, Interface, GUID, HRESULT};
use std::ffi::c_void;
use std::mem::{transmute, transmute_copy};

pub unsafe fn marshaler(outer: IUnknown, result: *mut *mut c_void) -> HRESULT {
    let mut marshaler = std::ptr::null_mut();
    _ = CoCreateFreeThreadedMarshaler(std::ptr::null_mut(), &mut marshaler);

    if marshaler.is_null() {
        return E_OUTOFMEMORY;
    }

    let marshaler: IMarshal = {
        let unknown: IUnknown = transmute_copy(&marshaler);
        let mut marshaler = std::ptr::null_mut();
        _ = (unknown.vtable().QueryInterface)(transmute_copy(&unknown), &IMarshal::IID, &mut marshaler);
        transmute_copy(&marshaler)
    };

    let com = Marshaler { vtable: &Marshaler::VTABLE, outer, marshaler, count: RefCount::new(1) };
    *result = transmute(::std::boxed::Box::new(com));
    S_OK
}

struct Marshaler {
    #[allow(dead_code)]
    vtable: *const IMarshal_Vtbl,
    outer: IUnknown,
    marshaler: IMarshal,
    count: RefCount,
}

impl Marshaler {
    const VTABLE: IMarshal_Vtbl = IMarshal_Vtbl {
        base__: IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        GetUnmarshalClass: Self::GetUnmarshalClass,
        GetMarshalSizeMax: Self::GetMarshalSizeMax,
        MarshalInterface: Self::MarshalInterface,
        UnmarshalInterface: Self::UnmarshalInterface,
        ReleaseMarshalData: Self::ReleaseMarshalData,
        DisconnectObject: Self::DisconnectObject,
    };

    unsafe extern "system" fn QueryInterface(this: *mut c_void, iid: *const GUID, interface: *mut *mut c_void) -> HRESULT {
        let this = this as *mut *mut c_void as *mut Self;
        if iid.is_null() || interface.is_null() {
            return E_POINTER;
        }

        if *iid == IMarshal::IID {
            *interface = transmute_copy(&(*this).marshaler);
            (*this).count.add_ref();
            return S_OK;
        }

        ((*this).outer.vtable().QueryInterface)(transmute_copy(&(*this).outer), iid, interface)
    }

    unsafe extern "system" fn AddRef(this: *mut c_void) -> u32 {
        let this = this as *mut *mut c_void as *mut Self;
        (*this).count.add_ref()
    }

    unsafe extern "system" fn Release(this: *mut c_void) -> u32 {
        let this = this as *mut *mut c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }

    unsafe extern "system" fn GetUnmarshalClass(this: *mut c_void, riid: *const GUID, pv: *const c_void, dwdestcontext: u32, pvdestcontext: *const c_void, mshlflags: u32, pcid: *mut GUID) -> HRESULT {
        let this = this as *mut *mut c_void as *mut Self;
        ((*this).marshaler.vtable().GetUnmarshalClass)(transmute_copy(&(*this).marshaler), riid, pv, dwdestcontext, pvdestcontext, mshlflags, pcid)
    }

    unsafe extern "system" fn GetMarshalSizeMax(this: *mut c_void, riid: *const GUID, pv: *const c_void, dwdestcontext: u32, pvdestcontext: *const c_void, mshlflags: u32, psize: *mut u32) -> HRESULT {
        let this = this as *mut *mut c_void as *mut Self;
        ((*this).marshaler.vtable().GetMarshalSizeMax)(transmute_copy(&(*this).marshaler), riid, pv, dwdestcontext, pvdestcontext, mshlflags, psize)
    }

    unsafe extern "system" fn MarshalInterface(this: *mut c_void, pstm: *mut c_void, riid: *const GUID, pv: *const c_void, dwdestcontext: u32, pvdestcontext: *const c_void, mshlflags: u32) -> HRESULT {
        let this = this as *mut *mut c_void as *mut Self;
        ((*this).marshaler.vtable().MarshalInterface)(transmute_copy(&(*this).marshaler), pstm, riid, pv, dwdestcontext, pvdestcontext, mshlflags)
    }

    unsafe extern "system" fn UnmarshalInterface(this: *mut c_void, pstm: *mut c_void, riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT {
        let this = this as *mut *mut c_void as *mut Self;
        ((*this).marshaler.vtable().UnmarshalInterface)(transmute_copy(&(*this).marshaler), pstm, riid, ppv)
    }

    unsafe extern "system" fn ReleaseMarshalData(this: *mut c_void, pstm: *mut c_void) -> HRESULT {
        let this = this as *mut *mut c_void as *mut Self;
        ((*this).marshaler.vtable().ReleaseMarshalData)(transmute_copy(&(*this).marshaler), pstm)
    }

    unsafe extern "system" fn DisconnectObject(this: *mut c_void, dwreserved: u32) -> HRESULT {
        let this = this as *mut *mut c_void as *mut Self;
        ((*this).marshaler.vtable().DisconnectObject)(transmute_copy(&(*this).marshaler), dwreserved)
    }
}

#[repr(transparent)]
#[derive(Clone)]
struct IMarshal(IUnknown);

unsafe impl Interface for IMarshal {
    type Vtable = IMarshal_Vtbl;
}

unsafe impl ComInterface for IMarshal {
    const IID: GUID = GUID::from_u128(0x00000003_0000_0000_c000_000000000046);
}

#[repr(C)]
#[doc(hidden)]
struct IMarshal_Vtbl {
    base__: IUnknown_Vtbl,
    GetUnmarshalClass: unsafe extern "system" fn(this: *mut c_void, riid: *const GUID, pv: *const c_void, dwdestcontext: u32, pvdestcontext: *const c_void, mshlflags: u32, pcid: *mut GUID) -> HRESULT,
    GetMarshalSizeMax: unsafe extern "system" fn(this: *mut c_void, riid: *const GUID, pv: *const c_void, dwdestcontext: u32, pvdestcontext: *const c_void, mshlflags: u32, psize: *mut u32) -> HRESULT,
    MarshalInterface: unsafe extern "system" fn(this: *mut c_void, pstm: *mut c_void, riid: *const GUID, pv: *const c_void, dwdestcontext: u32, pvdestcontext: *const c_void, mshlflags: u32) -> HRESULT,
    UnmarshalInterface: unsafe extern "system" fn(this: *mut c_void, pstm: *mut c_void, riid: *const GUID, ppv: *mut *mut c_void) -> HRESULT,
    ReleaseMarshalData: unsafe extern "system" fn(this: *mut c_void, pstm: *mut c_void) -> HRESULT,
    DisconnectObject: unsafe extern "system" fn(this: *mut c_void, dwreserved: u32) -> HRESULT,
}

windows_targets::link!("ole32.dll" "system" fn CoCreateFreeThreadedMarshaler(outer: *mut c_void, marshaler: *mut *mut c_void) -> HRESULT);
