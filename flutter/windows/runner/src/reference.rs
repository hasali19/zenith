use windows::core::{Array, IInspectable, Result, RuntimeType, GUID, HSTRING};
use windows::Foundation::{
    DateTime, IPropertyValue_Impl, IReference, IReference_Impl, Point, PropertyType, Rect, Size,
    TimeSpan,
};

#[windows::core::implement(IReference<T>)]
pub struct Reference<T>(T)
where
    T: RuntimeType + Clone + 'static;

impl<T: RuntimeType + Clone> Reference<T> {
    pub fn box_value(value: T) -> IReference<T> {
        Self(value).into()
    }
}

impl<T: RuntimeType + Clone + 'static> IReference_Impl<T> for Reference<T> {
    fn Value(&self) -> Result<T> {
        Ok(self.0.clone())
    }
}

impl<T: RuntimeType + Clone + 'static> IPropertyValue_Impl for Reference<T> {
    fn Type(&self) -> Result<PropertyType> {
        todo!()
    }
    fn IsNumericScalar(&self) -> Result<bool> {
        todo!()
    }
    fn GetUInt8(&self) -> Result<u8> {
        todo!()
    }
    fn GetRect(&self) -> Result<Rect> {
        todo!()
    }
    fn GetUInt8Array(&self, _: &mut Array<u8>) -> Result<()> {
        todo!()
    }
    fn GetInt16Array(&self, _: &mut Array<i16>) -> Result<()> {
        todo!()
    }
    fn GetUInt16Array(&self, _: &mut Array<u16>) -> Result<()> {
        todo!()
    }
    fn GetInt32Array(&self, _: &mut Array<i32>) -> Result<()> {
        todo!()
    }
    fn GetUInt32Array(&self, _: &mut Array<u32>) -> Result<()> {
        todo!()
    }
    fn GetInt64Array(&self, _: &mut Array<i64>) -> Result<()> {
        todo!()
    }
    fn GetUInt64Array(&self, _: &mut Array<u64>) -> Result<()> {
        todo!()
    }
    fn GetSingleArray(&self, _: &mut Array<f32>) -> Result<()> {
        todo!()
    }
    fn GetDoubleArray(&self, _: &mut Array<f64>) -> Result<()> {
        todo!()
    }
    fn GetChar16Array(&self, _: &mut Array<u16>) -> Result<()> {
        todo!()
    }
    fn GetBooleanArray(&self, _: &mut Array<bool>) -> Result<()> {
        todo!()
    }
    fn GetStringArray(&self, _: &mut Array<HSTRING>) -> Result<()> {
        todo!()
    }
    fn GetInspectableArray(&self, _: &mut Array<IInspectable>) -> Result<()> {
        todo!()
    }
    fn GetGuidArray(&self, _: &mut Array<GUID>) -> Result<()> {
        todo!()
    }
    fn GetDateTimeArray(&self, _: &mut Array<DateTime>) -> Result<()> {
        todo!()
    }
    fn GetTimeSpanArray(&self, _: &mut Array<TimeSpan>) -> Result<()> {
        todo!()
    }
    fn GetPointArray(&self, _: &mut Array<Point>) -> Result<()> {
        todo!()
    }
    fn GetSizeArray(&self, _: &mut Array<Size>) -> Result<()> {
        todo!()
    }
    fn GetRectArray(&self, _: &mut Array<Rect>) -> Result<()> {
        todo!()
    }
    fn GetInt16(&self) -> Result<i16> {
        todo!()
    }
    fn GetUInt16(&self) -> Result<u16> {
        todo!()
    }
    fn GetInt32(&self) -> Result<i32> {
        todo!()
    }
    fn GetUInt32(&self) -> Result<u32> {
        todo!()
    }
    fn GetInt64(&self) -> Result<i64> {
        todo!()
    }
    fn GetUInt64(&self) -> Result<u64> {
        todo!()
    }
    fn GetSingle(&self) -> Result<f32> {
        todo!()
    }
    fn GetDouble(&self) -> Result<f64> {
        todo!()
    }
    fn GetChar16(&self) -> Result<u16> {
        todo!()
    }
    fn GetBoolean(&self) -> Result<bool> {
        todo!()
    }
    fn GetString(&self) -> Result<HSTRING> {
        todo!()
    }
    fn GetGuid(&self) -> Result<GUID> {
        todo!()
    }
    fn GetDateTime(&self) -> Result<DateTime> {
        todo!()
    }
    fn GetTimeSpan(&self) -> Result<TimeSpan> {
        todo!()
    }
    fn GetPoint(&self) -> Result<Point> {
        todo!()
    }
    fn GetSize(&self) -> Result<Size> {
        todo!()
    }
}
