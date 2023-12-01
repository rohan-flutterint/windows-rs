#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDesktopWindowTarget(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDesktopWindowTarget {
    type Vtable = IDesktopWindowTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDesktopWindowTarget {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6329d6ca_3366_490e_9db3_25312929ac51);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowTarget_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsTopmost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DesktopWindowTarget(::windows_core::IUnknown);
impl DesktopWindowTarget {
    pub fn PopulatePropertyInfo<P0>(&self, propertyname: &::windows_core::HSTRING, propertyinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::AnimationPropertyInfo>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IAnimationObject>(self)?;
        unsafe { ::windows_core::vcall!(this.PopulatePropertyInfo(::core::mem::transmute_copy(propertyname), propertyinfo.into_param().abi())).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { ::windows_core::vcall!(this.Close()).ok() }
    }
    pub fn Compositor(&self) -> ::windows_core::Result<super::Compositor> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Compositor(&mut result__)).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"UI_Core\"`"]
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows_core::Result<super::super::Core::CoreDispatcher> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Dispatcher(&mut result__)).from_abi(result__)
        }
    }
    pub fn Properties(&self) -> ::windows_core::Result<super::CompositionPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Properties(&mut result__)).from_abi(result__)
        }
    }
    pub fn StartAnimation<P0>(&self, propertyname: &::windows_core::HSTRING, animation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { ::windows_core::vcall!(this.StartAnimation(::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi())).ok() }
    }
    pub fn StopAnimation(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject>(self)?;
        unsafe { ::windows_core::vcall!(this.StopAnimation(::core::mem::transmute_copy(propertyname))).ok() }
    }
    pub fn Comment(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Comment(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetComment(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetComment(::core::mem::transmute_copy(value))).ok() }
    }
    pub fn ImplicitAnimations(&self) -> ::windows_core::Result<super::ImplicitAnimationCollection> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.ImplicitAnimations(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetImplicitAnimations<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::ImplicitAnimationCollection>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { ::windows_core::vcall!(this.SetImplicitAnimations(value.into_param().abi())).ok() }
    }
    pub fn StartAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { ::windows_core::vcall!(this.StartAnimationGroup(value.try_into_param()?.abi())).ok() }
    }
    pub fn StopAnimationGroup<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::ICompositionAnimationBase>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject2>(self)?;
        unsafe { ::windows_core::vcall!(this.StopAnimationGroup(value.try_into_param()?.abi())).ok() }
    }
    #[doc = "Required features: `\"System\"`"]
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(&self) -> ::windows_core::Result<super::super::super::System::DispatcherQueue> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.DispatcherQueue(&mut result__)).from_abi(result__)
        }
    }
    pub fn TryGetAnimationController(&self, propertyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::AnimationController> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.TryGetAnimationController(::core::mem::transmute_copy(propertyname), &mut result__)).from_abi(result__)
        }
    }
    pub fn StartAnimationWithController<P0, P1>(&self, propertyname: &::windows_core::HSTRING, animation: P0, animationcontroller: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::CompositionAnimation>,
        P1: ::windows_core::IntoParam<super::AnimationController>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionObject5>(self)?;
        unsafe { ::windows_core::vcall!(this.StartAnimationWithController(::core::mem::transmute_copy(propertyname), animation.try_into_param()?.abi(), animationcontroller.into_param().abi())).ok() }
    }
    pub fn Root(&self) -> ::windows_core::Result<super::Visual> {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionTarget>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.Root(&mut result__)).from_abi(result__)
        }
    }
    pub fn SetRoot<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::Visual>,
    {
        let this = &::windows_core::ComInterface::cast::<super::ICompositionTarget>(self)?;
        unsafe { ::windows_core::vcall!(this.SetRoot(value.try_into_param()?.abi())).ok() }
    }
    pub fn IsTopmost(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            ::windows_core::vcall!(this.IsTopmost(&mut result__)).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for DesktopWindowTarget {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Composition.Desktop.DesktopWindowTarget;{6329d6ca-3366-490e-9db3-25312929ac51})");
}
unsafe impl ::windows_core::Interface for DesktopWindowTarget {
    type Vtable = IDesktopWindowTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DesktopWindowTarget {
    const IID: ::windows_core::GUID = <IDesktopWindowTarget as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DesktopWindowTarget {
    const NAME: &'static str = "Windows.UI.Composition.Desktop.DesktopWindowTarget";
}
::windows_core::imp::interface_hierarchy!(DesktopWindowTarget, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IAnimationObject> for DesktopWindowTarget {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for DesktopWindowTarget {}
impl ::windows_core::CanTryInto<super::CompositionTarget> for DesktopWindowTarget {}
impl ::windows_core::CanTryInto<super::CompositionObject> for DesktopWindowTarget {}
unsafe impl ::core::marker::Send for DesktopWindowTarget {}
unsafe impl ::core::marker::Sync for DesktopWindowTarget {}
