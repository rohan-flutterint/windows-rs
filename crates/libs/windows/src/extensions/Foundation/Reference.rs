use crate::{core::*, Foundation::*};

#[implement(IReference<T>)]
struct Reference<T>(T)
where
    T: RuntimeType + 'static;

impl<T: RuntimeType> IReference<T> {
    pub fn new(value: T) -> Self {
        Reference(value).into()
    }
}

impl<T: RuntimeType + 'static> IReference_Impl<T> for Reference_Impl<T> {
    fn Value(&self) -> Result<T> {
        Ok(self.0.clone())
    }
}

const fn not_implemented() -> Error {
    Error::from_hresult(HRESULT(0x80004001u32 as i32)) // E_NOTIMPL
}

impl<T: RuntimeType + 'static> IPropertyValue_Impl for Reference_Impl<T> {
    fn Type(&self) -> Result<PropertyType> {
        Err(not_implemented())
    }
    fn IsNumericScalar(&self) -> Result<bool> {
        Err(not_implemented())
    }
    fn GetUInt8(&self) -> Result<u8> {
        Err(not_implemented())
    }
    fn GetRect(&self) -> Result<Rect> {
        Err(not_implemented())
    }
    fn GetUInt8Array(&self, _: &mut Array<u8>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetInt16Array(&self, _: &mut Array<i16>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetUInt16Array(&self, _: &mut Array<u16>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetInt32Array(&self, _: &mut Array<i32>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetUInt32Array(&self, _: &mut Array<u32>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetInt64Array(&self, _: &mut Array<i64>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetUInt64Array(&self, _: &mut Array<u64>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetSingleArray(&self, _: &mut Array<f32>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetDoubleArray(&self, _: &mut Array<f64>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetChar16Array(&self, _: &mut Array<u16>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetBooleanArray(&self, _: &mut Array<bool>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetStringArray(&self, _: &mut Array<HSTRING>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetInspectableArray(&self, _: &mut Array<IInspectable>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetGuidArray(&self, _: &mut Array<GUID>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetDateTimeArray(&self, _: &mut Array<DateTime>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetTimeSpanArray(&self, _: &mut Array<TimeSpan>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetPointArray(&self, _: &mut Array<Point>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetSizeArray(&self, _: &mut Array<Size>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetRectArray(&self, _: &mut Array<Rect>) -> Result<()> {
        Err(not_implemented())
    }
    fn GetInt16(&self) -> Result<i16> {
        Err(not_implemented())
    }
    fn GetUInt16(&self) -> Result<u16> {
        Err(not_implemented())
    }
    fn GetInt32(&self) -> Result<i32> {
        Err(not_implemented())
    }
    fn GetUInt32(&self) -> Result<u32> {
        Err(not_implemented())
    }
    fn GetInt64(&self) -> Result<i64> {
        Err(not_implemented())
    }
    fn GetUInt64(&self) -> Result<u64> {
        Err(not_implemented())
    }
    fn GetSingle(&self) -> Result<f32> {
        Err(not_implemented())
    }
    fn GetDouble(&self) -> Result<f64> {
        Err(not_implemented())
    }
    fn GetChar16(&self) -> Result<u16> {
        Err(not_implemented())
    }
    fn GetBoolean(&self) -> Result<bool> {
        Err(not_implemented())
    }
    fn GetString(&self) -> Result<HSTRING> {
        Err(not_implemented())
    }
    fn GetGuid(&self) -> Result<GUID> {
        Err(not_implemented())
    }
    fn GetDateTime(&self) -> Result<DateTime> {
        Err(not_implemented())
    }
    fn GetTimeSpan(&self) -> Result<TimeSpan> {
        Err(not_implemented())
    }
    fn GetPoint(&self) -> Result<Point> {
        Err(not_implemented())
    }
    fn GetSize(&self) -> Result<Size> {
        Err(not_implemented())
    }
}
