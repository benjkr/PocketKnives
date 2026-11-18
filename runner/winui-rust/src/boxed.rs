//! `UIElement.AddHandler` takes its handler as an `IInspectable`, and a delegate isn't one:
//! C++/WinRT passes `box_value(handler)`, an `IReference<KeyEventHandler>`, and C#/WinRT boxes it
//! the same way. windows-rs has no box for delegates, so this is one. Only `Value` matters; the
//! `IPropertyValue` part reports "other type" and refuses every conversion, like any other box.
use crate::bindings::Microsoft::UI::Xaml::Input::KeyEventHandler;
use windows::Foundation::{
    DateTime, IPropertyValue, IPropertyValue_Impl, IReference, IReference_Impl, Point, PropertyType, Rect, Size, TimeSpan,
};
use windows_core::{Array, Error, GUID, HSTRING, IInspectable, Result, implement};

#[implement(IReference<KeyEventHandler>, IPropertyValue)]
pub struct KeyHandlerBox(pub KeyEventHandler);

/// The handler, boxed for `AddHandler`.
pub fn key_handler(handler: KeyEventHandler) -> IInspectable {
    KeyHandlerBox(handler).into()
}

impl IReference_Impl<KeyEventHandler> for KeyHandlerBox_Impl {
    fn Value(&self) -> Result<KeyEventHandler> {
        Ok(self.0.clone())
    }
}

fn no<T>() -> Result<T> {
    Err(Error::from_hresult(windows_core::HRESULT(0x8007_0057_u32 as i32))) // E_INVALIDARG, as a box of another type
}

impl IPropertyValue_Impl for KeyHandlerBox_Impl {
    fn Type(&self) -> Result<PropertyType> {
        Ok(PropertyType::OtherType)
    }
    fn IsNumericScalar(&self) -> Result<bool> {
        Ok(false)
    }
    fn GetUInt8(&self) -> Result<u8> { no() }
    fn GetInt16(&self) -> Result<i16> { no() }
    fn GetUInt16(&self) -> Result<u16> { no() }
    fn GetInt32(&self) -> Result<i32> { no() }
    fn GetUInt32(&self) -> Result<u32> { no() }
    fn GetInt64(&self) -> Result<i64> { no() }
    fn GetUInt64(&self) -> Result<u64> { no() }
    fn GetSingle(&self) -> Result<f32> { no() }
    fn GetDouble(&self) -> Result<f64> { no() }
    fn GetChar16(&self) -> Result<u16> { no() }
    fn GetBoolean(&self) -> Result<bool> { no() }
    fn GetString(&self) -> Result<HSTRING> { no() }
    fn GetGuid(&self) -> Result<GUID> { no() }
    fn GetDateTime(&self) -> Result<DateTime> { no() }
    fn GetTimeSpan(&self) -> Result<TimeSpan> { no() }
    fn GetPoint(&self) -> Result<Point> { no() }
    fn GetSize(&self) -> Result<Size> { no() }
    fn GetRect(&self) -> Result<Rect> { no() }
    fn GetUInt8Array(&self, _: &mut Array<u8>) -> Result<()> { no() }
    fn GetInt16Array(&self, _: &mut Array<i16>) -> Result<()> { no() }
    fn GetUInt16Array(&self, _: &mut Array<u16>) -> Result<()> { no() }
    fn GetInt32Array(&self, _: &mut Array<i32>) -> Result<()> { no() }
    fn GetUInt32Array(&self, _: &mut Array<u32>) -> Result<()> { no() }
    fn GetInt64Array(&self, _: &mut Array<i64>) -> Result<()> { no() }
    fn GetUInt64Array(&self, _: &mut Array<u64>) -> Result<()> { no() }
    fn GetSingleArray(&self, _: &mut Array<f32>) -> Result<()> { no() }
    fn GetDoubleArray(&self, _: &mut Array<f64>) -> Result<()> { no() }
    fn GetChar16Array(&self, _: &mut Array<u16>) -> Result<()> { no() }
    fn GetBooleanArray(&self, _: &mut Array<bool>) -> Result<()> { no() }
    fn GetStringArray(&self, _: &mut Array<HSTRING>) -> Result<()> { no() }
    fn GetInspectableArray(&self, _: &mut Array<IInspectable>) -> Result<()> { no() }
    fn GetGuidArray(&self, _: &mut Array<GUID>) -> Result<()> { no() }
    fn GetDateTimeArray(&self, _: &mut Array<DateTime>) -> Result<()> { no() }
    fn GetTimeSpanArray(&self, _: &mut Array<TimeSpan>) -> Result<()> { no() }
    fn GetPointArray(&self, _: &mut Array<Point>) -> Result<()> { no() }
    fn GetSizeArray(&self, _: &mut Array<Size>) -> Result<()> { no() }
    fn GetRectArray(&self, _: &mut Array<Rect>) -> Result<()> { no() }
}
