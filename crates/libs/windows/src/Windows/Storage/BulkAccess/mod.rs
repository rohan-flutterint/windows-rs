#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileInformationFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileInformationFactory {
    type Vtable = IFileInformationFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileInformationFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x401d88be_960f_4d6d_a7d0_1a3861e76c83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileInformationFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsyncDefaultStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsyncDefaultStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsyncDefaultStartAndCount: usize,
    pub GetVirtualizedItemsVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetVirtualizedFilesVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetVirtualizedFoldersVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileInformationFactoryFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileInformationFactoryFactory {
    type Vtable = IFileInformationFactoryFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileInformationFactoryFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x84ea0e7d_e4a2_4f00_8afa_af5e0f826bd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileInformationFactoryFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryresult: *mut ::core::ffi::c_void, mode: super::FileProperties::ThumbnailMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithMode: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithModeAndSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryresult: *mut ::core::ffi::c_void, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithModeAndSize: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithModeAndSizeAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryresult: *mut ::core::ffi::c_void, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithModeAndSizeAndOptions: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithModeAndSizeAndOptionsAndFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryresult: *mut ::core::ffi::c_void, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, delayload: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithModeAndSizeAndOptionsAndFlags: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IStorageItemInformation(::windows_core::IUnknown);
impl IStorageItemInformation {
    #[doc = "Required features: `\"Storage_FileProperties\"`"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> ::windows_core::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MusicProperties(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> ::windows_core::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.VideoProperties(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn ImageProperties(&self) -> ::windows_core::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ImageProperties(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn DocumentProperties(&self) -> ::windows_core::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DocumentProperties(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn BasicProperties(&self) -> ::windows_core::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BasicProperties(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn Thumbnail(&self) -> ::windows_core::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Thumbnail(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ThumbnailUpdated<P0>(&self, changedhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ThumbnailUpdated(changedhandler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveThumbnailUpdated(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveThumbnailUpdated(eventcookie)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PropertiesUpdated<P0>(&self, changedhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.PropertiesUpdated(changedhandler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePropertiesUpdated(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemovePropertiesUpdated(eventcookie)).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(IStorageItemInformation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IStorageItemInformation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{87a5cb8b-8972-4f40-8de0-d86fb179d8fa}");
}
unsafe impl ::windows_core::Interface for IStorageItemInformation {
    type Vtable = IStorageItemInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IStorageItemInformation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x87a5cb8b_8972_4f40_8de0_d86fb179d8fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemInformation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_FileProperties")]
    pub MusicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    MusicProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub VideoProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    VideoProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub ImageProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    ImageProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub DocumentProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    DocumentProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub BasicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    BasicProperties: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Streams")))]
    Thumbnail: usize,
    #[cfg(feature = "Foundation")]
    pub ThumbnailUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ThumbnailUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveThumbnailUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveThumbnailUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub PropertiesUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PropertiesUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePropertiesUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePropertiesUpdated: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FileInformation(::windows_core::IUnknown);
impl FileInformation {
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenSequentialReadAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::Streams::IInputStream>> {
        let this = &::windows_core::ComInterface::cast::<super::Streams::IInputStreamReference>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OpenSequentialReadAsync(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenReadAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStreamWithContentType>> {
        let this = &::windows_core::ComInterface::cast::<super::Streams::IRandomAccessStreamReference>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OpenReadAsync(&mut result__)).from_abi(result__)
        }
    }
    pub fn FileType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FileType(&mut result__)).from_abi(result__)
        }
    }
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ContentType(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenAsync(&self, accessmode: super::FileAccessMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OpenAsync(accessmode, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OpenTransactedWriteAsync(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverloadDefaultNameAndOptions<P0>(&self, destinationfolder: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageFolder>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CopyOverloadDefaultNameAndOptions(destinationfolder.try_into_param()?.abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverloadDefaultOptions<P0>(&self, destinationfolder: P0, desirednewname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageFolder>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CopyOverloadDefaultOptions(destinationfolder.try_into_param()?.abi(), ::core::mem::transmute_copy(desirednewname), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverload<P0>(&self, destinationfolder: P0, desirednewname: &::windows_core::HSTRING, option: super::NameCollisionOption) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageFolder>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CopyOverload(destinationfolder.try_into_param()?.abi(), ::core::mem::transmute_copy(desirednewname), option, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CopyAndReplaceAsync<P0>(&self, filetoreplace: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageFile>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CopyAndReplaceAsync(filetoreplace.try_into_param()?.abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverloadDefaultNameAndOptions<P0>(&self, destinationfolder: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageFolder>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MoveOverloadDefaultNameAndOptions(destinationfolder.try_into_param()?.abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverloadDefaultOptions<P0>(&self, destinationfolder: P0, desirednewname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageFolder>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MoveOverloadDefaultOptions(destinationfolder.try_into_param()?.abi(), ::core::mem::transmute_copy(desirednewname), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverload<P0>(&self, destinationfolder: P0, desirednewname: &::windows_core::HSTRING, option: super::NameCollisionOption) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageFolder>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MoveOverload(destinationfolder.try_into_param()?.abi(), ::core::mem::transmute_copy(desirednewname), option, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn MoveAndReplaceAsync<P0>(&self, filetoreplace: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageFile>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MoveAndReplaceAsync(filetoreplace.try_into_param()?.abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenWithOptionsAsync(&self, accessmode: super::FileAccessMode, options: super::StorageOpenOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OpenWithOptionsAsync(accessmode, options, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteWithOptionsAsync(&self, options: super::StorageOpenOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.OpenTransactedWriteWithOptionsAsync(options, &mut result__)).from_abi(result__)
        }
    }
    pub fn IsAvailable(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFilePropertiesWithAvailability>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsAvailable(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsyncOverloadDefaultOptions(&self, desiredname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.RenameAsyncOverloadDefaultOptions(::core::mem::transmute_copy(desiredname), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsync(&self, desiredname: &::windows_core::HSTRING, option: super::NameCollisionOption) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.RenameAsync(::core::mem::transmute_copy(desiredname), option, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DeleteAsyncOverloadDefaultOptions(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self, option: super::StorageDeleteOption) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DeleteAsync(option, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetBasicPropertiesAsync(&mut result__)).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Name(&mut result__)).from_abi(result__)
        }
    }
    pub fn Path(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Path(&mut result__)).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<super::FileAttributes> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Attributes(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DateCreated(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DateCreated(&mut result__)).from_abi(result__)
        }
    }
    pub fn IsOfType(&self, r#type: super::StorageItemTypes) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsOfType(r#type, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetParentAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetParentAsync(&mut result__)).from_abi(result__)
        }
    }
    pub fn IsEqual<P0>(&self, item: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsEqual(item.try_into_param()?.abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> ::windows_core::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MusicProperties(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> ::windows_core::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.VideoProperties(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn ImageProperties(&self) -> ::windows_core::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ImageProperties(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn DocumentProperties(&self) -> ::windows_core::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DocumentProperties(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn BasicProperties(&self) -> ::windows_core::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BasicProperties(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn Thumbnail(&self) -> ::windows_core::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Thumbnail(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ThumbnailUpdated<P0>(&self, changedhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ThumbnailUpdated(changedhandler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveThumbnailUpdated(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveThumbnailUpdated(eventcookie)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PropertiesUpdated<P0>(&self, changedhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.PropertiesUpdated(changedhandler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePropertiesUpdated(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemovePropertiesUpdated(eventcookie)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(mode, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetThumbnailAsyncOverloadDefaultOptions(mode, requestedsize, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetThumbnailAsync(mode, requestedsize, options, &mut result__)).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DisplayName(&mut result__)).from_abi(result__)
        }
    }
    pub fn DisplayType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DisplayType(&mut result__)).from_abi(result__)
        }
    }
    pub fn FolderRelativeId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FolderRelativeId(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn Properties(&self) -> ::windows_core::Result<super::FileProperties::StorageItemContentProperties> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Properties(&mut result__)).from_abi(result__)
        }
    }
    pub fn Provider(&self) -> ::windows_core::Result<super::StorageProvider> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItemPropertiesWithProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Provider(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for FileInformation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.BulkAccess.FileInformation;{87a5cb8b-8972-4f40-8de0-d86fb179d8fa})");
}
unsafe impl ::windows_core::Interface for FileInformation {
    type Vtable = IStorageItemInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FileInformation {
    const IID: ::windows_core::GUID = <IStorageItemInformation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FileInformation {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FileInformation";
}
::windows_core::imp::interface_hierarchy!(FileInformation, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::Streams::IInputStreamReference> for FileInformation {}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::Streams::IRandomAccessStreamReference> for FileInformation {}
impl ::windows_core::CanTryInto<super::IStorageFile> for FileInformation {}
impl ::windows_core::CanTryInto<super::IStorageFile2> for FileInformation {}
impl ::windows_core::CanTryInto<super::IStorageFilePropertiesWithAvailability> for FileInformation {}
impl ::windows_core::CanTryInto<super::IStorageItem> for FileInformation {}
impl ::windows_core::CanTryInto<super::IStorageItem2> for FileInformation {}
impl ::windows_core::CanTryInto<IStorageItemInformation> for FileInformation {}
impl ::windows_core::CanTryInto<super::IStorageItemProperties> for FileInformation {}
impl ::windows_core::CanTryInto<super::IStorageItemPropertiesWithProvider> for FileInformation {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FileInformationFactory(::windows_core::IUnknown);
impl FileInformationFactory {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetItemsAsync(startindex, maxitemstoretrieve, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsyncDefaultStartAndCount(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetItemsAsyncDefaultStartAndCount(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetFilesAsync(startindex, maxitemstoretrieve, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsyncDefaultStartAndCount(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetFilesAsyncDefaultStartAndCount(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetFoldersAsync(startindex, maxitemstoretrieve, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsyncDefaultStartAndCount(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetFoldersAsyncDefaultStartAndCount(&mut result__)).from_abi(result__)
        }
    }
    pub fn GetVirtualizedItemsVector(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetVirtualizedItemsVector(&mut result__)).from_abi(result__)
        }
    }
    pub fn GetVirtualizedFilesVector(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetVirtualizedFilesVector(&mut result__)).from_abi(result__)
        }
    }
    pub fn GetVirtualizedFoldersVector(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetVirtualizedFoldersVector(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`, `\"Storage_Search\"`"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithMode<P0>(queryresult: P0, mode: super::FileProperties::ThumbnailMode) -> ::windows_core::Result<FileInformationFactory>
    where
        P0: ::windows_core::TryIntoParam<super::Search::IStorageQueryResultBase>,
    {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateWithMode(queryresult.try_into_param()?.abi(), mode, &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`, `\"Storage_Search\"`"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithModeAndSize<P0>(queryresult: P0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32) -> ::windows_core::Result<FileInformationFactory>
    where
        P0: ::windows_core::TryIntoParam<super::Search::IStorageQueryResultBase>,
    {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateWithModeAndSize(queryresult.try_into_param()?.abi(), mode, requestedthumbnailsize, &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`, `\"Storage_Search\"`"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithModeAndSizeAndOptions<P0>(queryresult: P0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions) -> ::windows_core::Result<FileInformationFactory>
    where
        P0: ::windows_core::TryIntoParam<super::Search::IStorageQueryResultBase>,
    {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateWithModeAndSizeAndOptions(queryresult.try_into_param()?.abi(), mode, requestedthumbnailsize, thumbnailoptions, &mut result__)).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`, `\"Storage_Search\"`"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithModeAndSizeAndOptionsAndFlags<P0>(queryresult: P0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, delayload: bool) -> ::windows_core::Result<FileInformationFactory>
    where
        P0: ::windows_core::TryIntoParam<super::Search::IStorageQueryResultBase>,
    {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateWithModeAndSizeAndOptionsAndFlags(queryresult.try_into_param()?.abi(), mode, requestedthumbnailsize, thumbnailoptions, delayload, &mut result__)).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFileInformationFactoryFactory<R, F: FnOnce(&IFileInformationFactoryFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FileInformationFactory, IFileInformationFactoryFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for FileInformationFactory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.BulkAccess.FileInformationFactory;{401d88be-960f-4d6d-a7d0-1a3861e76c83})");
}
unsafe impl ::windows_core::Interface for FileInformationFactory {
    type Vtable = IFileInformationFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FileInformationFactory {
    const IID: ::windows_core::GUID = <IFileInformationFactory as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FileInformationFactory {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FileInformationFactory";
}
::windows_core::imp::interface_hierarchy!(FileInformationFactory, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for FileInformationFactory {}
unsafe impl ::core::marker::Sync for FileInformationFactory {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FolderInformation(::windows_core::IUnknown);
impl FolderInformation {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFileAsyncOverloadDefaultOptions(&self, desiredname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateFileAsyncOverloadDefaultOptions(::core::mem::transmute_copy(desiredname), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFileAsync(&self, desiredname: &::windows_core::HSTRING, options: super::CreationCollisionOption) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateFileAsync(::core::mem::transmute_copy(desiredname), options, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFolderAsyncOverloadDefaultOptions(&self, desiredname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateFolderAsyncOverloadDefaultOptions(::core::mem::transmute_copy(desiredname), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFolderAsync(&self, desiredname: &::windows_core::HSTRING, options: super::CreationCollisionOption) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateFolderAsync(::core::mem::transmute_copy(desiredname), options, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileAsync(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetFileAsync(::core::mem::transmute_copy(name), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetFolderAsync(::core::mem::transmute_copy(name), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemAsync(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetItemAsync(::core::mem::transmute_copy(name), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetFilesAsyncOverloadDefaultOptionsStartAndCount(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsyncOverloadDefaultStartAndCount(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetItemsAsyncOverloadDefaultStartAndCount(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetItemAsync(&self, name: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageFolder2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.TryGetItemAsync(::core::mem::transmute_copy(name), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Search\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Search"))]
    pub fn GetIndexedStateAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::Search::IndexedState>> {
        let this = &::windows_core::ComInterface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetIndexedStateAsync(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Search\"`"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFileQueryOverloadDefault(&self) -> ::windows_core::Result<super::Search::StorageFileQueryResult> {
        let this = &::windows_core::ComInterface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateFileQueryOverloadDefault(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Search\"`"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFileQuery(&self, query: super::Search::CommonFileQuery) -> ::windows_core::Result<super::Search::StorageFileQueryResult> {
        let this = &::windows_core::ComInterface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateFileQuery(query, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Search\"`"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFileQueryWithOptions<P0>(&self, queryoptions: P0) -> ::windows_core::Result<super::Search::StorageFileQueryResult>
    where
        P0: ::windows_core::IntoParam<super::Search::QueryOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateFileQueryWithOptions(queryoptions.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Search\"`"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFolderQueryOverloadDefault(&self) -> ::windows_core::Result<super::Search::StorageFolderQueryResult> {
        let this = &::windows_core::ComInterface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateFolderQueryOverloadDefault(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Search\"`"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFolderQuery(&self, query: super::Search::CommonFolderQuery) -> ::windows_core::Result<super::Search::StorageFolderQueryResult> {
        let this = &::windows_core::ComInterface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateFolderQuery(query, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Search\"`"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFolderQueryWithOptions<P0>(&self, queryoptions: P0) -> ::windows_core::Result<super::Search::StorageFolderQueryResult>
    where
        P0: ::windows_core::IntoParam<super::Search::QueryOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateFolderQueryWithOptions(queryoptions.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Search\"`"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateItemQuery(&self) -> ::windows_core::Result<super::Search::StorageItemQueryResult> {
        let this = &::windows_core::ComInterface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateItemQuery(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Search\"`"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateItemQueryWithOptions<P0>(&self, queryoptions: P0) -> ::windows_core::Result<super::Search::StorageItemQueryResult>
    where
        P0: ::windows_core::IntoParam<super::Search::QueryOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.CreateItemQueryWithOptions(queryoptions.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage_Search\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFilesAsync(&self, query: super::Search::CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = &::windows_core::ComInterface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetFilesAsync(query, startindex, maxitemstoretrieve, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage_Search\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: super::Search::CommonFileQuery) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = &::windows_core::ComInterface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetFilesAsyncOverloadDefaultStartAndCount(query, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage_Search\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFoldersAsync(&self, query: super::Search::CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = &::windows_core::ComInterface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetFoldersAsync(query, startindex, maxitemstoretrieve, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage_Search\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: super::Search::CommonFolderQuery) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = &::windows_core::ComInterface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetFoldersAsyncOverloadDefaultStartAndCount(query, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage_Search\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = &::windows_core::ComInterface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetItemsAsync(startindex, maxitemstoretrieve, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Search\"`"]
    #[cfg(feature = "Storage_Search")]
    pub fn AreQueryOptionsSupported<P0>(&self, queryoptions: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<super::Search::QueryOptions>,
    {
        let this = &::windows_core::ComInterface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.AreQueryOptionsSupported(queryoptions.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Search\"`"]
    #[cfg(feature = "Storage_Search")]
    pub fn IsCommonFolderQuerySupported(&self, query: super::Search::CommonFolderQuery) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsCommonFolderQuerySupported(query, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Search\"`"]
    #[cfg(feature = "Storage_Search")]
    pub fn IsCommonFileQuerySupported(&self, query: super::Search::CommonFileQuery) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsCommonFileQuerySupported(query, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsyncOverloadDefaultOptions(&self, desiredname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.RenameAsyncOverloadDefaultOptions(::core::mem::transmute_copy(desiredname), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsync(&self, desiredname: &::windows_core::HSTRING, option: super::NameCollisionOption) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.RenameAsync(::core::mem::transmute_copy(desiredname), option, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DeleteAsyncOverloadDefaultOptions(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self, option: super::StorageDeleteOption) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DeleteAsync(option, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetBasicPropertiesAsync(&mut result__)).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Name(&mut result__)).from_abi(result__)
        }
    }
    pub fn Path(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Path(&mut result__)).from_abi(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows_core::Result<super::FileAttributes> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Attributes(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn DateCreated(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DateCreated(&mut result__)).from_abi(result__)
        }
    }
    pub fn IsOfType(&self, r#type: super::StorageItemTypes) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsOfType(r#type, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetParentAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetParentAsync(&mut result__)).from_abi(result__)
        }
    }
    pub fn IsEqual<P0>(&self, item: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsEqual(item.try_into_param()?.abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> ::windows_core::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.MusicProperties(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> ::windows_core::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.VideoProperties(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn ImageProperties(&self) -> ::windows_core::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ImageProperties(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn DocumentProperties(&self) -> ::windows_core::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DocumentProperties(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn BasicProperties(&self) -> ::windows_core::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.BasicProperties(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn Thumbnail(&self) -> ::windows_core::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Thumbnail(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ThumbnailUpdated<P0>(&self, changedhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ThumbnailUpdated(changedhandler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveThumbnailUpdated(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemoveThumbnailUpdated(eventcookie)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PropertiesUpdated<P0>(&self, changedhandler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.PropertiesUpdated(changedhandler.into_param().abi(), &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePropertiesUpdated(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { ::windows_core::vcall!(this.RemovePropertiesUpdated(eventcookie)).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(mode, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetThumbnailAsyncOverloadDefaultOptions(mode, requestedsize, &mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.GetThumbnailAsync(mode, requestedsize, options, &mut result__)).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DisplayName(&mut result__)).from_abi(result__)
        }
    }
    pub fn DisplayType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DisplayType(&mut result__)).from_abi(result__)
        }
    }
    pub fn FolderRelativeId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.FolderRelativeId(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_FileProperties\"`"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn Properties(&self) -> ::windows_core::Result<super::FileProperties::StorageItemContentProperties> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Properties(&mut result__)).from_abi(result__)
        }
    }
    pub fn Provider(&self) -> ::windows_core::Result<super::StorageProvider> {
        let this = &::windows_core::ComInterface::cast::<super::IStorageItemPropertiesWithProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Provider(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for FolderInformation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.BulkAccess.FolderInformation;{87a5cb8b-8972-4f40-8de0-d86fb179d8fa})");
}
unsafe impl ::windows_core::Interface for FolderInformation {
    type Vtable = IStorageItemInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FolderInformation {
    const IID: ::windows_core::GUID = <IStorageItemInformation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FolderInformation {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FolderInformation";
}
::windows_core::imp::interface_hierarchy!(FolderInformation, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IStorageFolder> for FolderInformation {}
impl ::windows_core::CanTryInto<super::IStorageFolder2> for FolderInformation {}
#[cfg(feature = "Storage_Search")]
impl ::windows_core::CanTryInto<super::Search::IStorageFolderQueryOperations> for FolderInformation {}
impl ::windows_core::CanTryInto<super::IStorageItem> for FolderInformation {}
impl ::windows_core::CanTryInto<super::IStorageItem2> for FolderInformation {}
impl ::windows_core::CanTryInto<IStorageItemInformation> for FolderInformation {}
impl ::windows_core::CanTryInto<super::IStorageItemProperties> for FolderInformation {}
impl ::windows_core::CanTryInto<super::IStorageItemPropertiesWithProvider> for FolderInformation {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
