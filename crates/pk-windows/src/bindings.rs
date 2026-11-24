#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, clippy::all)]

pub mod Windows {
    pub mod Devices {
        pub mod Bluetooth {
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct BluetoothError(pub i32);
            impl BluetoothError {
                pub const Success: Self = Self(0i32);
                pub const RadioNotAvailable: Self = Self(1i32);
                pub const ResourceInUse: Self = Self(2i32);
                pub const DeviceNotConnected: Self = Self(3i32);
                pub const OtherError: Self = Self(4i32);
                pub const DisabledByPolicy: Self = Self(5i32);
                pub const NotSupported: Self = Self(6i32);
                pub const DisabledByUser: Self = Self(7i32);
                pub const ConsentRequired: Self = Self(8i32);
                pub const TransportNotSupported: Self = Self(9i32);
            }
            impl windows_core::TypeKind for BluetoothError {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for BluetoothError {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Bluetooth.BluetoothError;i4)");
            }
            pub mod Advertisement {
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct BluetoothLEAdvertisement(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(BluetoothLEAdvertisement, windows_core::IUnknown, windows_core::IInspectable);
                impl BluetoothLEAdvertisement {
                    pub fn new() -> windows_core::Result<Self> {
                        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                    }
                    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAdvertisement, windows_core::imp::IGenericFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    pub fn ManufacturerData(&self) -> windows_core::Result<windows_collections::IVector<BluetoothLEManufacturerData>> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).ManufacturerData)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                }
                impl windows_core::RuntimeType for BluetoothLEAdvertisement {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisement>();
                }
                unsafe impl windows_core::Interface for BluetoothLEAdvertisement {
                    type Vtable = <IBluetoothLEAdvertisement as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IBluetoothLEAdvertisement as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for BluetoothLEAdvertisement {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisement";
                }
                unsafe impl Send for BluetoothLEAdvertisement {}
                unsafe impl Sync for BluetoothLEAdvertisement {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct BluetoothLEAdvertisementFilter(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(BluetoothLEAdvertisementFilter, windows_core::IUnknown, windows_core::IInspectable);
                impl BluetoothLEAdvertisementFilter {
                    pub fn new() -> windows_core::Result<Self> {
                        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                    }
                    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<BluetoothLEAdvertisementFilter, windows_core::imp::IGenericFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    pub fn Advertisement(&self) -> windows_core::Result<BluetoothLEAdvertisement> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Advertisement)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                }
                impl windows_core::RuntimeType for BluetoothLEAdvertisementFilter {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisementFilter>();
                }
                unsafe impl windows_core::Interface for BluetoothLEAdvertisementFilter {
                    type Vtable = <IBluetoothLEAdvertisementFilter as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IBluetoothLEAdvertisementFilter as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for BluetoothLEAdvertisementFilter {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFilter";
                }
                unsafe impl Send for BluetoothLEAdvertisementFilter {}
                unsafe impl Sync for BluetoothLEAdvertisementFilter {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct BluetoothLEAdvertisementReceivedEventArgs(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    BluetoothLEAdvertisementReceivedEventArgs,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl BluetoothLEAdvertisementReceivedEventArgs {
                    pub fn RawSignalStrengthInDBm(&self) -> windows_core::Result<i16> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).RawSignalStrengthInDBm)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn Advertisement(&self) -> windows_core::Result<BluetoothLEAdvertisement> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Advertisement)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                }
                impl windows_core::RuntimeType for BluetoothLEAdvertisementReceivedEventArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisementReceivedEventArgs>();
                }
                unsafe impl windows_core::Interface for BluetoothLEAdvertisementReceivedEventArgs {
                    type Vtable = <IBluetoothLEAdvertisementReceivedEventArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IBluetoothLEAdvertisementReceivedEventArgs as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for BluetoothLEAdvertisementReceivedEventArgs {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs";
                }
                unsafe impl Send for BluetoothLEAdvertisementReceivedEventArgs {}
                unsafe impl Sync for BluetoothLEAdvertisementReceivedEventArgs {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct BluetoothLEAdvertisementWatcher(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    BluetoothLEAdvertisementWatcher,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl BluetoothLEAdvertisementWatcher {
                    pub fn new() -> windows_core::Result<Self> {
                        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                    }
                    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<
                            BluetoothLEAdvertisementWatcher,
                            windows_core::imp::IGenericFactory,
                        > = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    pub fn SetScanningMode(&self, value: BluetoothLEScanningMode) -> windows_core::Result<()> {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetScanningMode)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn AdvertisementFilter(&self) -> windows_core::Result<BluetoothLEAdvertisementFilter> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).AdvertisementFilter)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn Start(&self) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this)).ok() }
                    }
                    pub fn Stop(&self) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).Stop)(windows_core::Interface::as_raw(this)).ok() }
                    }
                    pub fn Received<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<
                            super::super::super::Foundation::TypedEventHandler<
                                BluetoothLEAdvertisementWatcher,
                                BluetoothLEAdvertisementReceivedEventArgs,
                            >,
                        >,
                    {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Received)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn Stopped<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<
                            super::super::super::Foundation::TypedEventHandler<
                                BluetoothLEAdvertisementWatcher,
                                BluetoothLEAdvertisementWatcherStoppedEventArgs,
                            >,
                        >,
                    {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Stopped)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn Create<P0>(advertisementfilter: P0) -> windows_core::Result<BluetoothLEAdvertisementWatcher>
                    where
                        P0: windows_core::Param<BluetoothLEAdvertisementFilter>,
                    {
                        Self::IBluetoothLEAdvertisementWatcherFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Create)(
                                windows_core::Interface::as_raw(this),
                                advertisementfilter.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    fn IBluetoothLEAdvertisementWatcherFactory<
                        R,
                        F: FnOnce(&IBluetoothLEAdvertisementWatcherFactory) -> windows_core::Result<R>,
                    >(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<
                            BluetoothLEAdvertisementWatcher,
                            IBluetoothLEAdvertisementWatcherFactory,
                        > = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for BluetoothLEAdvertisementWatcher {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisementWatcher>();
                }
                unsafe impl windows_core::Interface for BluetoothLEAdvertisementWatcher {
                    type Vtable = <IBluetoothLEAdvertisementWatcher as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IBluetoothLEAdvertisementWatcher as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for BluetoothLEAdvertisementWatcher {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher";
                }
                unsafe impl Send for BluetoothLEAdvertisementWatcher {}
                unsafe impl Sync for BluetoothLEAdvertisementWatcher {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct BluetoothLEAdvertisementWatcherStoppedEventArgs(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    BluetoothLEAdvertisementWatcherStoppedEventArgs,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl BluetoothLEAdvertisementWatcherStoppedEventArgs {
                    pub fn Error(&self) -> windows_core::Result<super::BluetoothError> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Error)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                }
                impl windows_core::RuntimeType for BluetoothLEAdvertisementWatcherStoppedEventArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEAdvertisementWatcherStoppedEventArgs>();
                }
                unsafe impl windows_core::Interface for BluetoothLEAdvertisementWatcherStoppedEventArgs {
                    type Vtable = <IBluetoothLEAdvertisementWatcherStoppedEventArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IBluetoothLEAdvertisementWatcherStoppedEventArgs as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for BluetoothLEAdvertisementWatcherStoppedEventArgs {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs";
                }
                unsafe impl Send for BluetoothLEAdvertisementWatcherStoppedEventArgs {}
                unsafe impl Sync for BluetoothLEAdvertisementWatcherStoppedEventArgs {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct BluetoothLEManufacturerData(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(BluetoothLEManufacturerData, windows_core::IUnknown, windows_core::IInspectable);
                impl BluetoothLEManufacturerData {
                    pub fn new() -> windows_core::Result<Self> {
                        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                    }
                    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<BluetoothLEManufacturerData, windows_core::imp::IGenericFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    pub fn CompanyId(&self) -> windows_core::Result<u16> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CompanyId)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetCompanyId(&self, value: u16) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetCompanyId)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Data(&self) -> windows_core::Result<super::super::super::Storage::Streams::IBuffer> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn Create<P1>(companyid: u16, data: P1) -> windows_core::Result<BluetoothLEManufacturerData>
                    where
                        P1: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
                    {
                        Self::IBluetoothLEManufacturerDataFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Create)(
                                windows_core::Interface::as_raw(this),
                                companyid,
                                data.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    fn IBluetoothLEManufacturerDataFactory<
                        R,
                        F: FnOnce(&IBluetoothLEManufacturerDataFactory) -> windows_core::Result<R>,
                    >(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<BluetoothLEManufacturerData, IBluetoothLEManufacturerDataFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for BluetoothLEManufacturerData {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IBluetoothLEManufacturerData>();
                }
                unsafe impl windows_core::Interface for BluetoothLEManufacturerData {
                    type Vtable = <IBluetoothLEManufacturerData as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IBluetoothLEManufacturerData as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for BluetoothLEManufacturerData {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData";
                }
                unsafe impl Send for BluetoothLEManufacturerData {}
                unsafe impl Sync for BluetoothLEManufacturerData {}
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct BluetoothLEScanningMode(pub i32);
                impl BluetoothLEScanningMode {
                    pub const Passive: Self = Self(0i32);
                    pub const Active: Self = Self(1i32);
                    pub const None: Self = Self(2i32);
                }
                impl windows_core::TypeKind for BluetoothLEScanningMode {
                    type TypeKind = windows_core::CopyType;
                }
                impl windows_core::RuntimeType for BluetoothLEScanningMode {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                        b"enum(Windows.Devices.Bluetooth.Advertisement.BluetoothLEScanningMode;i4)",
                    );
                }
                windows_core::imp::define_interface!(
                    IBluetoothLEAdvertisement,
                    IBluetoothLEAdvertisement_Vtbl,
                    0x066fb2b7_33d1_4e7d_8367_cf81d0f79653
                );
                impl windows_core::RuntimeType for IBluetoothLEAdvertisement {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                impl windows_core::RuntimeName for IBluetoothLEAdvertisement {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisement";
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBluetoothLEAdvertisement_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    Flags: usize,
                    SetFlags: usize,
                    pub LocalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetLocalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ServiceUuids:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ManufacturerData:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    DataSections: usize,
                    pub GetManufacturerDataByCompanyId:
                        unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetSectionsByType: usize,
                }
                windows_core::imp::define_interface!(
                    IBluetoothLEAdvertisementFilter,
                    IBluetoothLEAdvertisementFilter_Vtbl,
                    0x131eb0d3_d04e_47b1_837e_49405bf6f80f
                );
                impl windows_core::RuntimeType for IBluetoothLEAdvertisementFilter {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                impl windows_core::RuntimeName for IBluetoothLEAdvertisementFilter {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementFilter";
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBluetoothLEAdvertisementFilter_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Advertisement:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetAdvertisement:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    BytePatterns: usize,
                }
                windows_core::imp::define_interface!(
                    IBluetoothLEAdvertisementReceivedEventArgs,
                    IBluetoothLEAdvertisementReceivedEventArgs_Vtbl,
                    0x27987ddf_e596_41be_8d43_9e6731d4a913
                );
                impl windows_core::RuntimeType for IBluetoothLEAdvertisementReceivedEventArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                impl windows_core::RuntimeName for IBluetoothLEAdvertisementReceivedEventArgs {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs";
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBluetoothLEAdvertisementReceivedEventArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub RawSignalStrengthInDBm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
                    pub BluetoothAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
                    AdvertisementType: usize,
                    pub Timestamp: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut super::super::super::Foundation::DateTime,
                    ) -> windows_core::HRESULT,
                    pub Advertisement:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IBluetoothLEAdvertisementReceivedEventArgs2,
                    IBluetoothLEAdvertisementReceivedEventArgs2_Vtbl,
                    0x12d9c87b_0399_5f0e_a348_53b02b6b162e
                );
                impl windows_core::RuntimeType for IBluetoothLEAdvertisementReceivedEventArgs2 {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                impl windows_core::RuntimeName for IBluetoothLEAdvertisementReceivedEventArgs2 {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs2";
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBluetoothLEAdvertisementReceivedEventArgs2_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    BluetoothAddressType: usize,
                    pub TransmitPowerLevelInDBm:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsAnonymous: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub IsConnectable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub IsScannable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub IsDirected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub IsScanResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IBluetoothLEAdvertisementReceivedEventArgs3,
                    IBluetoothLEAdvertisementReceivedEventArgs3_Vtbl,
                    0x8d204b54_ff86_5d84_a25a_137dccd96f7a
                );
                impl windows_core::RuntimeType for IBluetoothLEAdvertisementReceivedEventArgs3 {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                impl windows_core::RuntimeName for IBluetoothLEAdvertisementReceivedEventArgs3 {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementReceivedEventArgs3";
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBluetoothLEAdvertisementReceivedEventArgs3_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    PrimaryPhy: usize,
                    SecondaryPhy: usize,
                }
                windows_core::imp::define_interface!(
                    IBluetoothLEAdvertisementWatcher,
                    IBluetoothLEAdvertisementWatcher_Vtbl,
                    0xa6ac336f_f3d3_4297_8d6c_c81ea6623f40
                );
                impl windows_core::RuntimeType for IBluetoothLEAdvertisementWatcher {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                impl windows_core::RuntimeName for IBluetoothLEAdvertisementWatcher {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher";
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBluetoothLEAdvertisementWatcher_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub MinSamplingInterval: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut super::super::super::Foundation::TimeSpan,
                    ) -> windows_core::HRESULT,
                    pub MaxSamplingInterval: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut super::super::super::Foundation::TimeSpan,
                    ) -> windows_core::HRESULT,
                    pub MinOutOfRangeTimeout: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut super::super::super::Foundation::TimeSpan,
                    ) -> windows_core::HRESULT,
                    pub MaxOutOfRangeTimeout: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut super::super::super::Foundation::TimeSpan,
                    ) -> windows_core::HRESULT,
                    Status: usize,
                    pub ScanningMode:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut BluetoothLEScanningMode) -> windows_core::HRESULT,
                    pub SetScanningMode:
                        unsafe extern "system" fn(*mut core::ffi::c_void, BluetoothLEScanningMode) -> windows_core::HRESULT,
                    SignalStrengthFilter: usize,
                    SetSignalStrengthFilter: usize,
                    pub AdvertisementFilter:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetAdvertisementFilter:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub Received:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                    pub RemoveReceived: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    pub Stopped:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                    pub RemoveStopped: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IBluetoothLEAdvertisementWatcher2,
                    IBluetoothLEAdvertisementWatcher2_Vtbl,
                    0x01bf26bc_b164_5805_90a3_e8a7997ff225
                );
                impl windows_core::RuntimeType for IBluetoothLEAdvertisementWatcher2 {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                impl windows_core::RuntimeName for IBluetoothLEAdvertisementWatcher2 {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher2";
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBluetoothLEAdvertisementWatcher2_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub AllowExtendedAdvertisements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetAllowExtendedAdvertisements: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IBluetoothLEAdvertisementWatcher3,
                    IBluetoothLEAdvertisementWatcher3_Vtbl,
                    0x14d980be_4002_5dbe_8519_ffca6ca389f0
                );
                impl windows_core::RuntimeType for IBluetoothLEAdvertisementWatcher3 {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                impl windows_core::RuntimeName for IBluetoothLEAdvertisementWatcher3 {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcher3";
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBluetoothLEAdvertisementWatcher3_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub UseUncoded1MPhy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetUseUncoded1MPhy: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub UseCodedPhy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetUseCodedPhy: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    ScanParameters: usize,
                    SetScanParameters: usize,
                    pub UseHardwareFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetUseHardwareFilter: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IBluetoothLEAdvertisementWatcherFactory,
                    IBluetoothLEAdvertisementWatcherFactory_Vtbl,
                    0x9aaf2d56_39ac_453e_b32a_85c657e017f1
                );
                impl windows_core::RuntimeType for IBluetoothLEAdvertisementWatcherFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                impl windows_core::RuntimeName for IBluetoothLEAdvertisementWatcherFactory {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcherFactory";
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBluetoothLEAdvertisementWatcherFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Create: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IBluetoothLEAdvertisementWatcherStoppedEventArgs,
                    IBluetoothLEAdvertisementWatcherStoppedEventArgs_Vtbl,
                    0xdd40f84d_e7b9_43e3_9c04_0685d085fd8c
                );
                impl windows_core::RuntimeType for IBluetoothLEAdvertisementWatcherStoppedEventArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                impl windows_core::RuntimeName for IBluetoothLEAdvertisementWatcherStoppedEventArgs {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEAdvertisementWatcherStoppedEventArgs";
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBluetoothLEAdvertisementWatcherStoppedEventArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BluetoothError) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IBluetoothLEManufacturerData,
                    IBluetoothLEManufacturerData_Vtbl,
                    0x912dba18_6963_4533_b061_4694dafb34e5
                );
                impl windows_core::RuntimeType for IBluetoothLEManufacturerData {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                impl windows_core::RuntimeName for IBluetoothLEManufacturerData {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEManufacturerData";
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBluetoothLEManufacturerData_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CompanyId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
                    pub SetCompanyId: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
                    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IBluetoothLEManufacturerDataFactory,
                    IBluetoothLEManufacturerDataFactory_Vtbl,
                    0xc09b39f8_319a_441e_8de5_66a81e877a6c
                );
                impl windows_core::RuntimeType for IBluetoothLEManufacturerDataFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                impl windows_core::RuntimeName for IBluetoothLEManufacturerDataFactory {
                    const NAME: &'static str = "Windows.Devices.Bluetooth.Advertisement.IBluetoothLEManufacturerDataFactory";
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBluetoothLEManufacturerDataFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Create: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u16,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
            }
        }
    }
    pub mod Foundation {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct DateTime {
            pub UniversalTime: i64,
        }
        impl windows_core::TypeKind for DateTime {
            type TypeKind = windows_core::CopyType;
        }
        impl windows_core::RuntimeType for DateTime {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.DateTime;i8)");
        }
        #[repr(transparent)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct EventHandler<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
        where
            T: windows_core::RuntimeType + 'static;
        unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for EventHandler<T> {
            type Vtable = EventHandler_Vtbl<T>;
            const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
        }
        impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for EventHandler<T> {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
                .push_slice(b"pinterface({9de1c535-6ae1-11e0-84e1-18a905bcc53f}")
                .push_slice(b";")
                .push_other(T::SIGNATURE)
                .push_slice(b")");
        }
        impl<T: windows_core::RuntimeType + 'static> EventHandler<T> {
            pub fn new<
                F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<T>) -> windows_core::Result<()> + Send + 'static,
            >(
                invoke: F,
            ) -> Self {
                let com = EventHandlerBox { vtable: &EventHandlerBox::<T, F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
                unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
            }
            pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
            where
                P0: windows_core::Param<windows_core::IInspectable>,
                P1: windows_core::Param<T>,
            {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).Invoke)(
                        windows_core::Interface::as_raw(this),
                        sender.param().abi(),
                        args.param().abi(),
                    )
                    .ok()
                }
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct EventHandler_Vtbl<T>
        where
            T: windows_core::RuntimeType + 'static,
        {
            base__: windows_core::IUnknown_Vtbl,
            Invoke: unsafe extern "system" fn(
                this: *mut core::ffi::c_void,
                sender: *mut core::ffi::c_void,
                args: windows_core::AbiType<T>,
            ) -> windows_core::HRESULT,
            T: core::marker::PhantomData<T>,
        }
        #[repr(C)]
        struct EventHandlerBox<
            T,
            F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<T>) -> windows_core::Result<()> + Send + 'static,
        >
        where
            T: windows_core::RuntimeType + 'static,
        {
            vtable: *const EventHandler_Vtbl<T>,
            invoke: F,
            count: windows_core::imp::RefCount,
        }
        impl<
                T: windows_core::RuntimeType + 'static,
                F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<T>) -> windows_core::Result<()> + Send + 'static,
            > EventHandlerBox<T, F>
        {
            const VTABLE: EventHandler_Vtbl<T> = EventHandler_Vtbl::<T> {
                base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
                Invoke: Self::Invoke,
                T: core::marker::PhantomData::<T>,
            };
            unsafe extern "system" fn QueryInterface(
                this: *mut core::ffi::c_void,
                iid: *const windows_core::GUID,
                interface: *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this = this as *mut *mut core::ffi::c_void as *mut Self;
                    if iid.is_null() || interface.is_null() {
                        return windows_core::HRESULT(-2147467261);
                    }
                    *interface = if *iid == <EventHandler<T> as windows_core::Interface>::IID
                        || *iid == <windows_core::IUnknown as windows_core::Interface>::IID
                        || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID
                    {
                        &mut (*this).vtable as *mut _ as _
                    } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                        (*this).count.add_ref();
                        return windows_core::imp::marshaler(
                            core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void),
                            interface,
                        );
                    } else {
                        core::ptr::null_mut()
                    };
                    if (*interface).is_null() {
                        windows_core::HRESULT(-2147467262)
                    } else {
                        (*this).count.add_ref();
                        windows_core::HRESULT(0)
                    }
                }
            }
            unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
                unsafe {
                    let this = this as *mut *mut core::ffi::c_void as *mut Self;
                    (*this).count.add_ref()
                }
            }
            unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
                unsafe {
                    let this = this as *mut *mut core::ffi::c_void as *mut Self;
                    let remaining = (*this).count.release();
                    if remaining == 0 {
                        let _ = windows_core::imp::Box::from_raw(this);
                    }
                    remaining
                }
            }
            unsafe extern "system" fn Invoke(
                this: *mut core::ffi::c_void,
                sender: *mut core::ffi::c_void,
                args: windows_core::AbiType<T>,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
                    (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&args)).into()
                }
            }
        }
        windows_core::imp::define_interface!(IClosable, IClosable_Vtbl, 0x30d5a829_7fa4_4026_83bb_d75bae4ea99e);
        impl windows_core::RuntimeType for IClosable {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
        }
        windows_core::imp::interface_hierarchy!(IClosable, windows_core::IUnknown, windows_core::IInspectable);
        impl IClosable {
            pub fn Close(&self) -> windows_core::Result<()> {
                let this = self;
                unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
            }
        }
        impl windows_core::RuntimeName for IClosable {
            const NAME: &'static str = "Windows.Foundation.IClosable";
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IClosable_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
        }
        windows_core::imp::define_interface!(IPropertyValue, IPropertyValue_Vtbl, 0x4bd682dd_7554_40e9_9a9b_82654ede7e62);
        impl windows_core::RuntimeType for IPropertyValue {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
        }
        windows_core::imp::interface_hierarchy!(IPropertyValue, windows_core::IUnknown, windows_core::IInspectable);
        impl IPropertyValue {
            pub fn Type(&self) -> windows_core::Result<PropertyType> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn IsNumericScalar(&self) -> windows_core::Result<bool> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).IsNumericScalar)(windows_core::Interface::as_raw(this), &mut result__)
                        .map(|| result__)
                }
            }
            pub fn GetUInt8(&self) -> windows_core::Result<u8> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetUInt8)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetInt16(&self) -> windows_core::Result<i16> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetInt16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetUInt16(&self) -> windows_core::Result<u16> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetUInt16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetInt32(&self) -> windows_core::Result<i32> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetInt32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetUInt32(&self) -> windows_core::Result<u32> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetUInt32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetInt64(&self) -> windows_core::Result<i64> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetInt64)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetUInt64(&self) -> windows_core::Result<u64> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetUInt64)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetSingle(&self) -> windows_core::Result<f32> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetSingle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetDouble(&self) -> windows_core::Result<f64> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetDouble)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetChar16(&self) -> windows_core::Result<u16> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetChar16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetBoolean(&self) -> windows_core::Result<bool> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetBoolean)(windows_core::Interface::as_raw(this), &mut result__)
                        .map(|| result__)
                }
            }
            pub fn GetString(&self) -> windows_core::Result<windows_core::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetString)(windows_core::Interface::as_raw(this), &mut result__)
                        .map(|| core::mem::transmute(result__))
                }
            }
            pub fn GetGuid(&self) -> windows_core::Result<windows_core::GUID> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetGuid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetDateTime(&self) -> windows_core::Result<DateTime> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetDateTime)(windows_core::Interface::as_raw(this), &mut result__)
                        .map(|| result__)
                }
            }
            pub fn GetTimeSpan(&self) -> windows_core::Result<TimeSpan> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetTimeSpan)(windows_core::Interface::as_raw(this), &mut result__)
                        .map(|| result__)
                }
            }
            pub fn GetPoint(&self) -> windows_core::Result<Point> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetSize(&self) -> windows_core::Result<Size> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetRect(&self) -> windows_core::Result<Rect> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetUInt8Array(&self, value: &mut windows_core::Array<u8>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetUInt8Array)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetInt16Array(&self, value: &mut windows_core::Array<i16>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetInt16Array)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetUInt16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetUInt16Array)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetInt32Array(&self, value: &mut windows_core::Array<i32>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetInt32Array)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetUInt32Array(&self, value: &mut windows_core::Array<u32>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetUInt32Array)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetInt64Array(&self, value: &mut windows_core::Array<i64>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetInt64Array)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetUInt64Array(&self, value: &mut windows_core::Array<u64>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetUInt64Array)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetSingleArray(&self, value: &mut windows_core::Array<f32>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetSingleArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetDoubleArray(&self, value: &mut windows_core::Array<f64>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetDoubleArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetChar16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetChar16Array)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetBooleanArray(&self, value: &mut windows_core::Array<bool>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetBooleanArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetStringArray(&self, value: &mut windows_core::Array<windows_core::HSTRING>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetStringArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetInspectableArray(&self, value: &mut windows_core::Array<windows_core::IInspectable>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetInspectableArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetGuidArray(&self, value: &mut windows_core::Array<windows_core::GUID>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetGuidArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetDateTimeArray(&self, value: &mut windows_core::Array<DateTime>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetDateTimeArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetTimeSpanArray(&self, value: &mut windows_core::Array<TimeSpan>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetTimeSpanArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetPointArray(&self, value: &mut windows_core::Array<Point>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetPointArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetSizeArray(&self, value: &mut windows_core::Array<Size>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetSizeArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetRectArray(&self, value: &mut windows_core::Array<Rect>) -> windows_core::Result<()> {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).GetRectArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
        }
        impl windows_core::RuntimeName for IPropertyValue {
            const NAME: &'static str = "Windows.Foundation.IPropertyValue";
        }
        pub trait IPropertyValue_Impl: windows_core::IUnknownImpl {
            fn Type(&self) -> windows_core::Result<PropertyType>;
            fn IsNumericScalar(&self) -> windows_core::Result<bool>;
            fn GetUInt8(&self) -> windows_core::Result<u8>;
            fn GetInt16(&self) -> windows_core::Result<i16>;
            fn GetUInt16(&self) -> windows_core::Result<u16>;
            fn GetInt32(&self) -> windows_core::Result<i32>;
            fn GetUInt32(&self) -> windows_core::Result<u32>;
            fn GetInt64(&self) -> windows_core::Result<i64>;
            fn GetUInt64(&self) -> windows_core::Result<u64>;
            fn GetSingle(&self) -> windows_core::Result<f32>;
            fn GetDouble(&self) -> windows_core::Result<f64>;
            fn GetChar16(&self) -> windows_core::Result<u16>;
            fn GetBoolean(&self) -> windows_core::Result<bool>;
            fn GetString(&self) -> windows_core::Result<windows_core::HSTRING>;
            fn GetGuid(&self) -> windows_core::Result<windows_core::GUID>;
            fn GetDateTime(&self) -> windows_core::Result<DateTime>;
            fn GetTimeSpan(&self) -> windows_core::Result<TimeSpan>;
            fn GetPoint(&self) -> windows_core::Result<Point>;
            fn GetSize(&self) -> windows_core::Result<Size>;
            fn GetRect(&self) -> windows_core::Result<Rect>;
            fn GetUInt8Array(&self, value: &mut windows_core::Array<u8>) -> windows_core::Result<()>;
            fn GetInt16Array(&self, value: &mut windows_core::Array<i16>) -> windows_core::Result<()>;
            fn GetUInt16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()>;
            fn GetInt32Array(&self, value: &mut windows_core::Array<i32>) -> windows_core::Result<()>;
            fn GetUInt32Array(&self, value: &mut windows_core::Array<u32>) -> windows_core::Result<()>;
            fn GetInt64Array(&self, value: &mut windows_core::Array<i64>) -> windows_core::Result<()>;
            fn GetUInt64Array(&self, value: &mut windows_core::Array<u64>) -> windows_core::Result<()>;
            fn GetSingleArray(&self, value: &mut windows_core::Array<f32>) -> windows_core::Result<()>;
            fn GetDoubleArray(&self, value: &mut windows_core::Array<f64>) -> windows_core::Result<()>;
            fn GetChar16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()>;
            fn GetBooleanArray(&self, value: &mut windows_core::Array<bool>) -> windows_core::Result<()>;
            fn GetStringArray(&self, value: &mut windows_core::Array<windows_core::HSTRING>) -> windows_core::Result<()>;
            fn GetInspectableArray(&self, value: &mut windows_core::Array<windows_core::IInspectable>) -> windows_core::Result<()>;
            fn GetGuidArray(&self, value: &mut windows_core::Array<windows_core::GUID>) -> windows_core::Result<()>;
            fn GetDateTimeArray(&self, value: &mut windows_core::Array<DateTime>) -> windows_core::Result<()>;
            fn GetTimeSpanArray(&self, value: &mut windows_core::Array<TimeSpan>) -> windows_core::Result<()>;
            fn GetPointArray(&self, value: &mut windows_core::Array<Point>) -> windows_core::Result<()>;
            fn GetSizeArray(&self, value: &mut windows_core::Array<Size>) -> windows_core::Result<()>;
            fn GetRectArray(&self, value: &mut windows_core::Array<Rect>) -> windows_core::Result<()>;
        }
        impl IPropertyValue_Vtbl {
            pub const fn new<Identity: IPropertyValue_Impl, const OFFSET: isize>() -> Self {
                unsafe extern "system" fn Type<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut PropertyType,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::Type(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn IsNumericScalar<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut bool,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::IsNumericScalar(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetUInt8<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut u8,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetUInt8(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetInt16<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut i16,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetInt16(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetUInt16<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut u16,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetUInt16(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetInt32<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut i32,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetInt32(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetUInt32<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut u32,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetUInt32(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetInt64<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut i64,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetInt64(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetUInt64<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut u64,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetUInt64(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetSingle<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut f32,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetSingle(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetDouble<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut f64,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetDouble(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetChar16<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut u16,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetChar16(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetBoolean<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut bool,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetBoolean(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetString<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetString(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                core::mem::forget(ok__);
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetGuid<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut windows_core::GUID,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetGuid(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetDateTime<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut DateTime,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetDateTime(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetTimeSpan<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut TimeSpan,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetTimeSpan(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetPoint<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut Point,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetPoint(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetSize<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut Size,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetSize(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetRect<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    result__: *mut Rect,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IPropertyValue_Impl::GetRect(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                unsafe extern "system" fn GetUInt8Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut u8,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetUInt8Array(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetInt16Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut i16,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetInt16Array(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetUInt16Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut u16,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetUInt16Array(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetInt32Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut i32,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetInt32Array(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetUInt32Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut u32,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetUInt32Array(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetInt64Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut i64,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetInt64Array(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetUInt64Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut u64,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetUInt64Array(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetSingleArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut f32,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetSingleArray(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetDoubleArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut f64,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetDoubleArray(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetChar16Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut u16,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetChar16Array(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetBooleanArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut bool,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetBooleanArray(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetStringArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut windows_core::HSTRING,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetStringArray(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetInspectableArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut windows_core::IInspectable,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetInspectableArray(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetGuidArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut windows_core::GUID,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetGuidArray(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetDateTimeArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut DateTime,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetDateTimeArray(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetTimeSpanArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut TimeSpan,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetTimeSpanArray(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetPointArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut Point,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetPointArray(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetSizeArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut Size,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetSizeArray(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                unsafe extern "system" fn GetRectArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(
                    this: *mut core::ffi::c_void,
                    value_array_size: *mut u32,
                    value: *mut *mut Rect,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        IPropertyValue_Impl::GetRectArray(
                            this,
                            &mut windows_core::imp::array_proxy(core::mem::transmute_copy(&value), value_array_size),
                        )
                        .into()
                    }
                }
                Self {
                    base__: windows_core::IInspectable_Vtbl::new::<Identity, IPropertyValue, OFFSET>(),
                    Type: Type::<Identity, OFFSET>,
                    IsNumericScalar: IsNumericScalar::<Identity, OFFSET>,
                    GetUInt8: GetUInt8::<Identity, OFFSET>,
                    GetInt16: GetInt16::<Identity, OFFSET>,
                    GetUInt16: GetUInt16::<Identity, OFFSET>,
                    GetInt32: GetInt32::<Identity, OFFSET>,
                    GetUInt32: GetUInt32::<Identity, OFFSET>,
                    GetInt64: GetInt64::<Identity, OFFSET>,
                    GetUInt64: GetUInt64::<Identity, OFFSET>,
                    GetSingle: GetSingle::<Identity, OFFSET>,
                    GetDouble: GetDouble::<Identity, OFFSET>,
                    GetChar16: GetChar16::<Identity, OFFSET>,
                    GetBoolean: GetBoolean::<Identity, OFFSET>,
                    GetString: GetString::<Identity, OFFSET>,
                    GetGuid: GetGuid::<Identity, OFFSET>,
                    GetDateTime: GetDateTime::<Identity, OFFSET>,
                    GetTimeSpan: GetTimeSpan::<Identity, OFFSET>,
                    GetPoint: GetPoint::<Identity, OFFSET>,
                    GetSize: GetSize::<Identity, OFFSET>,
                    GetRect: GetRect::<Identity, OFFSET>,
                    GetUInt8Array: GetUInt8Array::<Identity, OFFSET>,
                    GetInt16Array: GetInt16Array::<Identity, OFFSET>,
                    GetUInt16Array: GetUInt16Array::<Identity, OFFSET>,
                    GetInt32Array: GetInt32Array::<Identity, OFFSET>,
                    GetUInt32Array: GetUInt32Array::<Identity, OFFSET>,
                    GetInt64Array: GetInt64Array::<Identity, OFFSET>,
                    GetUInt64Array: GetUInt64Array::<Identity, OFFSET>,
                    GetSingleArray: GetSingleArray::<Identity, OFFSET>,
                    GetDoubleArray: GetDoubleArray::<Identity, OFFSET>,
                    GetChar16Array: GetChar16Array::<Identity, OFFSET>,
                    GetBooleanArray: GetBooleanArray::<Identity, OFFSET>,
                    GetStringArray: GetStringArray::<Identity, OFFSET>,
                    GetInspectableArray: GetInspectableArray::<Identity, OFFSET>,
                    GetGuidArray: GetGuidArray::<Identity, OFFSET>,
                    GetDateTimeArray: GetDateTimeArray::<Identity, OFFSET>,
                    GetTimeSpanArray: GetTimeSpanArray::<Identity, OFFSET>,
                    GetPointArray: GetPointArray::<Identity, OFFSET>,
                    GetSizeArray: GetSizeArray::<Identity, OFFSET>,
                    GetRectArray: GetRectArray::<Identity, OFFSET>,
                }
            }
            pub fn matches(iid: &windows_core::GUID) -> bool {
                iid == &<IPropertyValue as windows_core::Interface>::IID
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IPropertyValue_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PropertyType) -> windows_core::HRESULT,
            pub IsNumericScalar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
            pub GetUInt8: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
            pub GetInt16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
            pub GetUInt16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
            pub GetInt32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
            pub GetUInt32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
            pub GetInt64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
            pub GetUInt64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
            pub GetSingle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
            pub GetDouble: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
            pub GetChar16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
            pub GetBoolean: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
            pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub GetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
            pub GetDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DateTime) -> windows_core::HRESULT,
            pub GetTimeSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TimeSpan) -> windows_core::HRESULT,
            pub GetPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
            pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Size) -> windows_core::HRESULT,
            pub GetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Rect) -> windows_core::HRESULT,
            pub GetUInt8Array: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
            pub GetInt16Array: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut i16) -> windows_core::HRESULT,
            pub GetUInt16Array: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u16) -> windows_core::HRESULT,
            pub GetInt32Array: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut i32) -> windows_core::HRESULT,
            pub GetUInt32Array: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u32) -> windows_core::HRESULT,
            pub GetInt64Array: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut i64) -> windows_core::HRESULT,
            pub GetUInt64Array: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u64) -> windows_core::HRESULT,
            pub GetSingleArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut f32) -> windows_core::HRESULT,
            pub GetDoubleArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut f64) -> windows_core::HRESULT,
            pub GetChar16Array: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u16) -> windows_core::HRESULT,
            pub GetBooleanArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut bool) -> windows_core::HRESULT,
            pub GetStringArray:
                unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut windows_core::HSTRING) -> windows_core::HRESULT,
            pub GetInspectableArray:
                unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut windows_core::IInspectable) -> windows_core::HRESULT,
            pub GetGuidArray:
                unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
            pub GetDateTimeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut DateTime) -> windows_core::HRESULT,
            pub GetTimeSpanArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut TimeSpan) -> windows_core::HRESULT,
            pub GetPointArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut Point) -> windows_core::HRESULT,
            pub GetSizeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut Size) -> windows_core::HRESULT,
            pub GetRectArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut Rect) -> windows_core::HRESULT,
        }
        windows_core::imp::define_interface!(IPropertyValueStatics, IPropertyValueStatics_Vtbl, 0x629bdbc8_d932_4ff4_96b9_8d96c5c1e858);
        impl windows_core::RuntimeType for IPropertyValueStatics {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
        }
        impl windows_core::RuntimeName for IPropertyValueStatics {
            const NAME: &'static str = "Windows.Foundation.IPropertyValueStatics";
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IPropertyValueStatics_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub CreateEmpty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateUInt8: unsafe extern "system" fn(*mut core::ffi::c_void, u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateInt16: unsafe extern "system" fn(*mut core::ffi::c_void, i16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateUInt16: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateInt32: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateUInt32: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateInt64: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateUInt64: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateSingle: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateDouble: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateChar16: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateBoolean:
                unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateString: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateInspectable: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateGuid:
                unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateDateTime:
                unsafe extern "system" fn(*mut core::ffi::c_void, DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateTimeSpan:
                unsafe extern "system" fn(*mut core::ffi::c_void, TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreatePoint: unsafe extern "system" fn(*mut core::ffi::c_void, Point, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateSize: unsafe extern "system" fn(*mut core::ffi::c_void, Size, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateRect: unsafe extern "system" fn(*mut core::ffi::c_void, Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateUInt8Array:
                unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateInt16Array:
                unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateUInt16Array:
                unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateInt32Array:
                unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateUInt32Array:
                unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateInt64Array:
                unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateUInt64Array:
                unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateSingleArray:
                unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateDoubleArray:
                unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateChar16Array:
                unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateBooleanArray:
                unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateStringArray: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const windows_core::HSTRING,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateInspectableArray: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const windows_core::IInspectable,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateGuidArray: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const windows_core::GUID,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateDateTimeArray: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const DateTime,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateTimeSpanArray: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                u32,
                *const TimeSpan,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreatePointArray:
                unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const Point, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateSizeArray:
                unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const Size, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub CreateRectArray:
                unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
        }
        #[repr(transparent)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct IReference<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
        where
            T: windows_core::RuntimeType + 'static;
        impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IReference<T> {}
        impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IReference<T> {}
        unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IReference<T> {
            type Vtable = IReference_Vtbl<T>;
            const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
        }
        impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IReference<T> {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
                .push_slice(b"pinterface({61c17706-2d65-11e0-9ae8-d48564015472}")
                .push_slice(b";")
                .push_other(T::SIGNATURE)
                .push_slice(b")");
        }
        impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IPropertyValue> for IReference<T> {
            const QUERY: bool = true;
        }
        impl<T: windows_core::RuntimeType + 'static> IReference<T> {
            pub fn Value(&self) -> windows_core::Result<T> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__)
                        .and_then(|| windows_core::Type::from_abi(result__))
                }
            }
            pub fn Type(&self) -> windows_core::Result<PropertyType> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn IsNumericScalar(&self) -> windows_core::Result<bool> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).IsNumericScalar)(windows_core::Interface::as_raw(this), &mut result__)
                        .map(|| result__)
                }
            }
            pub fn GetUInt8(&self) -> windows_core::Result<u8> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetUInt8)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetInt16(&self) -> windows_core::Result<i16> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetInt16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetUInt16(&self) -> windows_core::Result<u16> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetUInt16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetInt32(&self) -> windows_core::Result<i32> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetInt32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetUInt32(&self) -> windows_core::Result<u32> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetUInt32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetInt64(&self) -> windows_core::Result<i64> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetInt64)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetUInt64(&self) -> windows_core::Result<u64> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetUInt64)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetSingle(&self) -> windows_core::Result<f32> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetSingle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetDouble(&self) -> windows_core::Result<f64> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetDouble)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetChar16(&self) -> windows_core::Result<u16> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetChar16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetBoolean(&self) -> windows_core::Result<bool> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetBoolean)(windows_core::Interface::as_raw(this), &mut result__)
                        .map(|| result__)
                }
            }
            pub fn GetString(&self) -> windows_core::Result<windows_core::HSTRING> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetString)(windows_core::Interface::as_raw(this), &mut result__)
                        .map(|| core::mem::transmute(result__))
                }
            }
            pub fn GetGuid(&self) -> windows_core::Result<windows_core::GUID> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetGuid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetDateTime(&self) -> windows_core::Result<DateTime> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetDateTime)(windows_core::Interface::as_raw(this), &mut result__)
                        .map(|| result__)
                }
            }
            pub fn GetTimeSpan(&self) -> windows_core::Result<TimeSpan> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetTimeSpan)(windows_core::Interface::as_raw(this), &mut result__)
                        .map(|| result__)
                }
            }
            pub fn GetPoint(&self) -> windows_core::Result<Point> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetSize(&self) -> windows_core::Result<Size> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetRect(&self) -> windows_core::Result<Rect> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).GetRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                }
            }
            pub fn GetUInt8Array(&self, value: &mut windows_core::Array<u8>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetUInt8Array)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetInt16Array(&self, value: &mut windows_core::Array<i16>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetInt16Array)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetUInt16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetUInt16Array)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetInt32Array(&self, value: &mut windows_core::Array<i32>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetInt32Array)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetUInt32Array(&self, value: &mut windows_core::Array<u32>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetUInt32Array)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetInt64Array(&self, value: &mut windows_core::Array<i64>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetInt64Array)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetUInt64Array(&self, value: &mut windows_core::Array<u64>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetUInt64Array)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetSingleArray(&self, value: &mut windows_core::Array<f32>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetSingleArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetDoubleArray(&self, value: &mut windows_core::Array<f64>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetDoubleArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetChar16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetChar16Array)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetBooleanArray(&self, value: &mut windows_core::Array<bool>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetBooleanArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetStringArray(&self, value: &mut windows_core::Array<windows_core::HSTRING>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetStringArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetInspectableArray(&self, value: &mut windows_core::Array<windows_core::IInspectable>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetInspectableArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetGuidArray(&self, value: &mut windows_core::Array<windows_core::GUID>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetGuidArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetDateTimeArray(&self, value: &mut windows_core::Array<DateTime>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetDateTimeArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetTimeSpanArray(&self, value: &mut windows_core::Array<TimeSpan>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetTimeSpanArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetPointArray(&self, value: &mut windows_core::Array<Point>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetPointArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetSizeArray(&self, value: &mut windows_core::Array<Size>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetSizeArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
            pub fn GetRectArray(&self, value: &mut windows_core::Array<Rect>) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).GetRectArray)(
                        windows_core::Interface::as_raw(this),
                        value.set_abi_len(),
                        value as *mut _ as _,
                    )
                    .ok()
                }
            }
        }
        impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IReference<T> {
            const NAME: &'static str = "Windows.Foundation.IReference";
        }
        pub trait IReference_Impl<T>: IPropertyValue_Impl
        where
            T: windows_core::RuntimeType + 'static,
        {
            fn Value(&self) -> windows_core::Result<T>;
        }
        impl<T: windows_core::RuntimeType + 'static> IReference_Vtbl<T> {
            pub const fn new<Identity: IReference_Impl<T>, const OFFSET: isize>() -> Self {
                unsafe extern "system" fn Value<
                    T: windows_core::RuntimeType + 'static,
                    Identity: IReference_Impl<T>,
                    const OFFSET: isize,
                >(
                    this: *mut core::ffi::c_void,
                    result__: *mut windows_core::AbiType<T>,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IReference_Impl::Value(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                core::mem::forget(ok__);
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                Self {
                    base__: windows_core::IInspectable_Vtbl::new::<Identity, IReference<T>, OFFSET>(),
                    Value: Value::<T, Identity, OFFSET>,
                    T: core::marker::PhantomData::<T>,
                }
            }
            pub fn matches(iid: &windows_core::GUID) -> bool {
                iid == &<IReference<T> as windows_core::Interface>::IID
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IReference_Vtbl<T>
        where
            T: windows_core::RuntimeType + 'static,
        {
            pub base__: windows_core::IInspectable_Vtbl,
            pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::AbiType<T>) -> windows_core::HRESULT,
            T: core::marker::PhantomData<T>,
        }
        windows_core::imp::define_interface!(IStringable, IStringable_Vtbl, 0x96369f54_8eb6_48f0_abce_c1b211e627c3);
        impl windows_core::RuntimeType for IStringable {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
        }
        windows_core::imp::interface_hierarchy!(IStringable, windows_core::IUnknown, windows_core::IInspectable);
        impl IStringable {
            pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__)
                        .map(|| core::mem::transmute(result__))
                }
            }
        }
        impl windows_core::RuntimeName for IStringable {
            const NAME: &'static str = "Windows.Foundation.IStringable";
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IStringable_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub ToString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
        }
        windows_core::imp::define_interface!(IUriEscapeStatics, IUriEscapeStatics_Vtbl, 0xc1d432ba_c824_4452_a7fd_512bc3bbe9a1);
        impl windows_core::RuntimeType for IUriEscapeStatics {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
        }
        impl windows_core::RuntimeName for IUriEscapeStatics {
            const NAME: &'static str = "Windows.Foundation.IUriEscapeStatics";
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IUriEscapeStatics_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub UnescapeComponent: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub EscapeComponent: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
        }
        windows_core::imp::define_interface!(IUriRuntimeClass, IUriRuntimeClass_Vtbl, 0x9e365e57_48b2_4160_956f_c7385120bbfc);
        impl windows_core::RuntimeType for IUriRuntimeClass {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
        }
        impl windows_core::RuntimeName for IUriRuntimeClass {
            const NAME: &'static str = "Windows.Foundation.IUriRuntimeClass";
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IUriRuntimeClass_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub AbsoluteUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub DisplayUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub Domain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub Extension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub Fragment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub Host: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub Password: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            QueryParsed: usize,
            pub RawUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub SchemeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub Port: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
            pub Suspicious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
            pub Equals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
            pub CombineUri: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
        }
        windows_core::imp::define_interface!(IUriRuntimeClassFactory, IUriRuntimeClassFactory_Vtbl, 0x44a9796f_723e_4fdf_a218_033e75b0c084);
        impl windows_core::RuntimeType for IUriRuntimeClassFactory {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
        }
        impl windows_core::RuntimeName for IUriRuntimeClassFactory {
            const NAME: &'static str = "Windows.Foundation.IUriRuntimeClassFactory";
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IUriRuntimeClassFactory_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub CreateUri: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
            pub CreateWithRelativeUri: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
        }
        windows_core::imp::define_interface!(
            IUriRuntimeClassWithAbsoluteCanonicalUri,
            IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl,
            0x758d9661_221c_480f_a339_50656673f46f
        );
        impl windows_core::RuntimeType for IUriRuntimeClassWithAbsoluteCanonicalUri {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
        }
        impl windows_core::RuntimeName for IUriRuntimeClassWithAbsoluteCanonicalUri {
            const NAME: &'static str = "Windows.Foundation.IUriRuntimeClassWithAbsoluteCanonicalUri";
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub AbsoluteCanonicalUri:
                unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            pub DisplayIri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
        }
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct Point {
            pub X: f32,
            pub Y: f32,
        }
        impl windows_core::TypeKind for Point {
            type TypeKind = windows_core::CopyType;
        }
        impl windows_core::RuntimeType for Point {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Point;f4;f4)");
        }
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct PropertyType(pub i32);
        impl PropertyType {
            pub const Empty: Self = Self(0i32);
            pub const UInt8: Self = Self(1i32);
            pub const Int16: Self = Self(2i32);
            pub const UInt16: Self = Self(3i32);
            pub const Int32: Self = Self(4i32);
            pub const UInt32: Self = Self(5i32);
            pub const Int64: Self = Self(6i32);
            pub const UInt64: Self = Self(7i32);
            pub const Single: Self = Self(8i32);
            pub const Double: Self = Self(9i32);
            pub const Char16: Self = Self(10i32);
            pub const Boolean: Self = Self(11i32);
            pub const String: Self = Self(12i32);
            pub const Inspectable: Self = Self(13i32);
            pub const DateTime: Self = Self(14i32);
            pub const TimeSpan: Self = Self(15i32);
            pub const Guid: Self = Self(16i32);
            pub const Point: Self = Self(17i32);
            pub const Size: Self = Self(18i32);
            pub const Rect: Self = Self(19i32);
            pub const OtherType: Self = Self(20i32);
            pub const UInt8Array: Self = Self(1025i32);
            pub const Int16Array: Self = Self(1026i32);
            pub const UInt16Array: Self = Self(1027i32);
            pub const Int32Array: Self = Self(1028i32);
            pub const UInt32Array: Self = Self(1029i32);
            pub const Int64Array: Self = Self(1030i32);
            pub const UInt64Array: Self = Self(1031i32);
            pub const SingleArray: Self = Self(1032i32);
            pub const DoubleArray: Self = Self(1033i32);
            pub const Char16Array: Self = Self(1034i32);
            pub const BooleanArray: Self = Self(1035i32);
            pub const StringArray: Self = Self(1036i32);
            pub const InspectableArray: Self = Self(1037i32);
            pub const DateTimeArray: Self = Self(1038i32);
            pub const TimeSpanArray: Self = Self(1039i32);
            pub const GuidArray: Self = Self(1040i32);
            pub const PointArray: Self = Self(1041i32);
            pub const SizeArray: Self = Self(1042i32);
            pub const RectArray: Self = Self(1043i32);
            pub const OtherTypeArray: Self = Self(1044i32);
        }
        impl windows_core::TypeKind for PropertyType {
            type TypeKind = windows_core::CopyType;
        }
        impl windows_core::RuntimeType for PropertyType {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.PropertyType;i4)");
        }
        pub struct PropertyValue;
        impl PropertyValue {
            pub fn CreateString(value: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
                Self::IPropertyValueStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateString)(
                        windows_core::Interface::as_raw(this),
                        core::mem::transmute_copy(value),
                        &mut result__,
                    )
                    .and_then(|| windows_core::Type::from_abi(result__))
                })
            }
            fn IPropertyValueStatics<R, F: FnOnce(&IPropertyValueStatics) -> windows_core::Result<R>>(
                callback: F,
            ) -> windows_core::Result<R> {
                static SHARED: windows_core::imp::FactoryCache<PropertyValue, IPropertyValueStatics> =
                    windows_core::imp::FactoryCache::new();
                SHARED.call(callback)
            }
        }
        impl windows_core::RuntimeName for PropertyValue {
            const NAME: &'static str = "Windows.Foundation.PropertyValue";
        }
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct Rect {
            pub X: f32,
            pub Y: f32,
            pub Width: f32,
            pub Height: f32,
        }
        impl windows_core::TypeKind for Rect {
            type TypeKind = windows_core::CopyType;
        }
        impl windows_core::RuntimeType for Rect {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Rect;f4;f4;f4;f4)");
        }
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct Size {
            pub Width: f32,
            pub Height: f32,
        }
        impl windows_core::TypeKind for Size {
            type TypeKind = windows_core::CopyType;
        }
        impl windows_core::RuntimeType for Size {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Size;f4;f4)");
        }
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct TimeSpan {
            pub Duration: i64,
        }
        impl windows_core::TypeKind for TimeSpan {
            type TypeKind = windows_core::CopyType;
        }
        impl windows_core::RuntimeType for TimeSpan {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.TimeSpan;i8)");
        }
        #[repr(transparent)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct TypedEventHandler<TSender, TResult>(
            windows_core::IUnknown,
            core::marker::PhantomData<TSender>,
            core::marker::PhantomData<TResult>,
        )
        where
            TSender: windows_core::RuntimeType + 'static,
            TResult: windows_core::RuntimeType + 'static;
        unsafe impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static> windows_core::Interface
            for TypedEventHandler<TSender, TResult>
        {
            type Vtable = TypedEventHandler_Vtbl<TSender, TResult>;
            const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
        }
        impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static> windows_core::RuntimeType
            for TypedEventHandler<TSender, TResult>
        {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
                .push_slice(b"pinterface({9de1c534-6ae1-11e0-84e1-18a905bcc53f}")
                .push_slice(b";")
                .push_other(TSender::SIGNATURE)
                .push_slice(b";")
                .push_other(TResult::SIGNATURE)
                .push_slice(b")");
        }
        impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static> TypedEventHandler<TSender, TResult> {
            pub fn new<F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) -> windows_core::Result<()> + Send + 'static>(
                invoke: F,
            ) -> Self {
                let com = TypedEventHandlerBox {
                    vtable: &TypedEventHandlerBox::<TSender, TResult, F>::VTABLE,
                    count: windows_core::imp::RefCount::new(1),
                    invoke,
                };
                unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
            }
            pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
            where
                P0: windows_core::Param<TSender>,
                P1: windows_core::Param<TResult>,
            {
                let this = self;
                unsafe {
                    (windows_core::Interface::vtable(this).Invoke)(
                        windows_core::Interface::as_raw(this),
                        sender.param().abi(),
                        args.param().abi(),
                    )
                    .ok()
                }
            }
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct TypedEventHandler_Vtbl<TSender, TResult>
        where
            TSender: windows_core::RuntimeType + 'static,
            TResult: windows_core::RuntimeType + 'static,
        {
            base__: windows_core::IUnknown_Vtbl,
            Invoke: unsafe extern "system" fn(
                this: *mut core::ffi::c_void,
                sender: windows_core::AbiType<TSender>,
                args: windows_core::AbiType<TResult>,
            ) -> windows_core::HRESULT,
            TSender: core::marker::PhantomData<TSender>,
            TResult: core::marker::PhantomData<TResult>,
        }
        #[repr(C)]
        struct TypedEventHandlerBox<
            TSender,
            TResult,
            F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) -> windows_core::Result<()> + Send + 'static,
        >
        where
            TSender: windows_core::RuntimeType + 'static,
            TResult: windows_core::RuntimeType + 'static,
        {
            vtable: *const TypedEventHandler_Vtbl<TSender, TResult>,
            invoke: F,
            count: windows_core::imp::RefCount,
        }
        impl<
                TSender: windows_core::RuntimeType + 'static,
                TResult: windows_core::RuntimeType + 'static,
                F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) -> windows_core::Result<()> + Send + 'static,
            > TypedEventHandlerBox<TSender, TResult, F>
        {
            const VTABLE: TypedEventHandler_Vtbl<TSender, TResult> = TypedEventHandler_Vtbl::<TSender, TResult> {
                base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
                Invoke: Self::Invoke,
                TSender: core::marker::PhantomData::<TSender>,
                TResult: core::marker::PhantomData::<TResult>,
            };
            unsafe extern "system" fn QueryInterface(
                this: *mut core::ffi::c_void,
                iid: *const windows_core::GUID,
                interface: *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this = this as *mut *mut core::ffi::c_void as *mut Self;
                    if iid.is_null() || interface.is_null() {
                        return windows_core::HRESULT(-2147467261);
                    }
                    *interface = if *iid == <TypedEventHandler<TSender, TResult> as windows_core::Interface>::IID
                        || *iid == <windows_core::IUnknown as windows_core::Interface>::IID
                        || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID
                    {
                        &mut (*this).vtable as *mut _ as _
                    } else if *iid == <windows_core::imp::IMarshal as windows_core::Interface>::IID {
                        (*this).count.add_ref();
                        return windows_core::imp::marshaler(
                            core::mem::transmute(&mut (*this).vtable as *mut _ as *mut core::ffi::c_void),
                            interface,
                        );
                    } else {
                        core::ptr::null_mut()
                    };
                    if (*interface).is_null() {
                        windows_core::HRESULT(-2147467262)
                    } else {
                        (*this).count.add_ref();
                        windows_core::HRESULT(0)
                    }
                }
            }
            unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
                unsafe {
                    let this = this as *mut *mut core::ffi::c_void as *mut Self;
                    (*this).count.add_ref()
                }
            }
            unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
                unsafe {
                    let this = this as *mut *mut core::ffi::c_void as *mut Self;
                    let remaining = (*this).count.release();
                    if remaining == 0 {
                        let _ = windows_core::imp::Box::from_raw(this);
                    }
                    remaining
                }
            }
            unsafe extern "system" fn Invoke(
                this: *mut core::ffi::c_void,
                sender: windows_core::AbiType<TSender>,
                args: windows_core::AbiType<TResult>,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
                    (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&args)).into()
                }
            }
        }
        #[repr(transparent)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Uri(windows_core::IUnknown);
        windows_core::imp::interface_hierarchy!(Uri, windows_core::IUnknown, windows_core::IInspectable);
        windows_core::imp::required_hierarchy!(Uri, IStringable);
        impl Uri {
            pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
                let this = &windows_core::Interface::cast::<IStringable>(self)?;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__)
                        .map(|| core::mem::transmute(result__))
                }
            }
            pub fn Host(&self) -> windows_core::Result<windows_core::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).Host)(windows_core::Interface::as_raw(this), &mut result__)
                        .map(|| core::mem::transmute(result__))
                }
            }
            pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).Path)(windows_core::Interface::as_raw(this), &mut result__)
                        .map(|| core::mem::transmute(result__))
                }
            }
            pub fn CreateUri(uri: &windows_core::HSTRING) -> windows_core::Result<Uri> {
                Self::IUriRuntimeClassFactory(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateUri)(
                        windows_core::Interface::as_raw(this),
                        core::mem::transmute_copy(uri),
                        &mut result__,
                    )
                    .and_then(|| windows_core::Type::from_abi(result__))
                })
            }
            fn IUriEscapeStatics<R, F: FnOnce(&IUriEscapeStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                static SHARED: windows_core::imp::FactoryCache<Uri, IUriEscapeStatics> = windows_core::imp::FactoryCache::new();
                SHARED.call(callback)
            }
            fn IUriRuntimeClassFactory<R, F: FnOnce(&IUriRuntimeClassFactory) -> windows_core::Result<R>>(
                callback: F,
            ) -> windows_core::Result<R> {
                static SHARED: windows_core::imp::FactoryCache<Uri, IUriRuntimeClassFactory> = windows_core::imp::FactoryCache::new();
                SHARED.call(callback)
            }
        }
        impl windows_core::RuntimeType for Uri {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUriRuntimeClass>();
        }
        unsafe impl windows_core::Interface for Uri {
            type Vtable = <IUriRuntimeClass as windows_core::Interface>::Vtable;
            const IID: windows_core::GUID = <IUriRuntimeClass as windows_core::Interface>::IID;
        }
        impl windows_core::RuntimeName for Uri {
            const NAME: &'static str = "Windows.Foundation.Uri";
        }
        unsafe impl Send for Uri {}
        unsafe impl Sync for Uri {}
        pub mod Collections {
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct IObservableVector<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
            where
                T: windows_core::RuntimeType + 'static;
            impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IObservableVector<T> {}
            impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IObservableVector<T> {}
            unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IObservableVector<T> {
                type Vtable = IObservableVector_Vtbl<T>;
                const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
            }
            impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IObservableVector<T> {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
                    .push_slice(b"pinterface({5917eb53-50b4-4a0d-b309-65862b3f1dbc}")
                    .push_slice(b";")
                    .push_other(T::SIGNATURE)
                    .push_slice(b")");
            }
            impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_collections::IIterable<T>> for IObservableVector<T> {
                const QUERY: bool = true;
            }
            impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_collections::IVector<T>> for IObservableVector<T> {
                const QUERY: bool = true;
            }
            impl<T: windows_core::RuntimeType + 'static> IObservableVector<T> {
                pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<T>> {
                    let this = &windows_core::Interface::cast::<windows_collections::IIterable<T>>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn GetAt(&self, index: u32) -> windows_core::Result<T> {
                    let this = &windows_core::Interface::cast::<windows_collections::IVector<T>>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn Size(&self) -> windows_core::Result<u32> {
                    let this = &windows_core::Interface::cast::<windows_collections::IVector<T>>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                    }
                }
                pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<T>,
                {
                    let this = &windows_core::Interface::cast::<windows_collections::IVector<T>>(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok()
                    }
                }
                pub fn Clear(&self) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<windows_collections::IVector<T>>(self)?;
                    unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
                }
            }
            impl<T: windows_core::RuntimeType + 'static> IntoIterator for IObservableVector<T> {
                type Item = T;
                type IntoIter = windows_collections::IIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    IntoIterator::into_iter(&self)
                }
            }
            impl<T: windows_core::RuntimeType + 'static> IntoIterator for &IObservableVector<T> {
                type Item = T;
                type IntoIter = windows_collections::IIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    self.First().unwrap()
                }
            }
            impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IObservableVector<T> {
                const NAME: &'static str = "Windows.Foundation.Collections.IObservableVector";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IObservableVector_Vtbl<T>
            where
                T: windows_core::RuntimeType + 'static,
            {
                pub base__: windows_core::IInspectable_Vtbl,
                VectorChanged: usize,
                pub RemoveVectorChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                T: core::marker::PhantomData<T>,
            }
        }
    }
    pub mod Graphics {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct PointInt32 {
            pub X: i32,
            pub Y: i32,
        }
        impl windows_core::TypeKind for PointInt32 {
            type TypeKind = windows_core::CopyType;
        }
        impl windows_core::RuntimeType for PointInt32 {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.PointInt32;i4;i4)");
        }
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct RectInt32 {
            pub X: i32,
            pub Y: i32,
            pub Width: i32,
            pub Height: i32,
        }
        impl windows_core::TypeKind for RectInt32 {
            type TypeKind = windows_core::CopyType;
        }
        impl windows_core::RuntimeType for RectInt32 {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.RectInt32;i4;i4;i4;i4)");
        }
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct SizeInt32 {
            pub Width: i32,
            pub Height: i32,
        }
        impl windows_core::TypeKind for SizeInt32 {
            type TypeKind = windows_core::CopyType;
        }
        impl windows_core::RuntimeType for SizeInt32 {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.SizeInt32;i4;i4)");
        }
        pub mod Capture {
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Direct3D11CaptureFrame(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(Direct3D11CaptureFrame, windows_core::IUnknown, windows_core::IInspectable);
            windows_core::imp::required_hierarchy!(Direct3D11CaptureFrame, super::super::Foundation::IClosable);
            impl Direct3D11CaptureFrame {
                pub fn Close(&self) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
                    unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
                }
                pub fn Surface(&self) -> windows_core::Result<super::DirectX::Direct3D11::IDirect3DSurface> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Surface)(windows_core::Interface::as_raw(this), &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn SystemRelativeTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).SystemRelativeTime)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn ContentSize(&self) -> windows_core::Result<super::SizeInt32> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).ContentSize)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for Direct3D11CaptureFrame {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IDirect3D11CaptureFrame>();
            }
            unsafe impl windows_core::Interface for Direct3D11CaptureFrame {
                type Vtable = <IDirect3D11CaptureFrame as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IDirect3D11CaptureFrame as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for Direct3D11CaptureFrame {
                const NAME: &'static str = "Windows.Graphics.Capture.Direct3D11CaptureFrame";
            }
            unsafe impl Send for Direct3D11CaptureFrame {}
            unsafe impl Sync for Direct3D11CaptureFrame {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Direct3D11CaptureFramePool(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(Direct3D11CaptureFramePool, windows_core::IUnknown, windows_core::IInspectable);
            windows_core::imp::required_hierarchy!(Direct3D11CaptureFramePool, super::super::Foundation::IClosable);
            impl Direct3D11CaptureFramePool {
                pub fn Close(&self) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
                    unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
                }
                pub fn Recreate<P0>(
                    &self,
                    device: P0,
                    pixelformat: super::DirectX::DirectXPixelFormat,
                    numberofbuffers: i32,
                    size: super::SizeInt32,
                ) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<super::DirectX::Direct3D11::IDirect3DDevice>,
                {
                    let this = self;
                    unsafe {
                        (windows_core::Interface::vtable(this).Recreate)(
                            windows_core::Interface::as_raw(this),
                            device.param().abi(),
                            pixelformat,
                            numberofbuffers,
                            size,
                        )
                        .ok()
                    }
                }
                pub fn TryGetNextFrame(&self) -> windows_core::Result<Direct3D11CaptureFrame> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).TryGetNextFrame)(windows_core::Interface::as_raw(this), &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn FrameArrived<P0>(&self, handler: P0) -> windows_core::Result<i64>
                where
                    P0: windows_core::Param<
                        super::super::Foundation::TypedEventHandler<Direct3D11CaptureFramePool, windows_core::IInspectable>,
                    >,
                {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).FrameArrived)(
                            windows_core::Interface::as_raw(this),
                            handler.param().abi(),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn CreateCaptureSession<P0>(&self, item: P0) -> windows_core::Result<GraphicsCaptureSession>
                where
                    P0: windows_core::Param<GraphicsCaptureItem>,
                {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateCaptureSession)(
                            windows_core::Interface::as_raw(this),
                            item.param().abi(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn Create<P0>(
                    device: P0,
                    pixelformat: super::DirectX::DirectXPixelFormat,
                    numberofbuffers: i32,
                    size: super::SizeInt32,
                ) -> windows_core::Result<Direct3D11CaptureFramePool>
                where
                    P0: windows_core::Param<super::DirectX::Direct3D11::IDirect3DDevice>,
                {
                    Self::IDirect3D11CaptureFramePoolStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Create)(
                            windows_core::Interface::as_raw(this),
                            device.param().abi(),
                            pixelformat,
                            numberofbuffers,
                            size,
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    })
                }
                pub fn CreateFreeThreaded<P0>(
                    device: P0,
                    pixelformat: super::DirectX::DirectXPixelFormat,
                    numberofbuffers: i32,
                    size: super::SizeInt32,
                ) -> windows_core::Result<Direct3D11CaptureFramePool>
                where
                    P0: windows_core::Param<super::DirectX::Direct3D11::IDirect3DDevice>,
                {
                    Self::IDirect3D11CaptureFramePoolStatics2(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateFreeThreaded)(
                            windows_core::Interface::as_raw(this),
                            device.param().abi(),
                            pixelformat,
                            numberofbuffers,
                            size,
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    })
                }
                fn IDirect3D11CaptureFramePoolStatics<R, F: FnOnce(&IDirect3D11CaptureFramePoolStatics) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<Direct3D11CaptureFramePool, IDirect3D11CaptureFramePoolStatics> =
                        windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                fn IDirect3D11CaptureFramePoolStatics2<R, F: FnOnce(&IDirect3D11CaptureFramePoolStatics2) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<Direct3D11CaptureFramePool, IDirect3D11CaptureFramePoolStatics2> =
                        windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for Direct3D11CaptureFramePool {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IDirect3D11CaptureFramePool>();
            }
            unsafe impl windows_core::Interface for Direct3D11CaptureFramePool {
                type Vtable = <IDirect3D11CaptureFramePool as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IDirect3D11CaptureFramePool as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for Direct3D11CaptureFramePool {
                const NAME: &'static str = "Windows.Graphics.Capture.Direct3D11CaptureFramePool";
            }
            unsafe impl Send for Direct3D11CaptureFramePool {}
            unsafe impl Sync for Direct3D11CaptureFramePool {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct GraphicsCaptureItem(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(GraphicsCaptureItem, windows_core::IUnknown, windows_core::IInspectable);
            impl GraphicsCaptureItem {
                pub fn Size(&self) -> windows_core::Result<super::SizeInt32> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                    }
                }
                pub fn Closed<P0>(&self, handler: P0) -> windows_core::Result<i64>
                where
                    P0: windows_core::Param<super::super::Foundation::TypedEventHandler<GraphicsCaptureItem, windows_core::IInspectable>>,
                {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Closed)(
                            windows_core::Interface::as_raw(this),
                            handler.param().abi(),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn RemoveClosed(&self, token: i64) -> windows_core::Result<()> {
                    let this = self;
                    unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), token).ok() }
                }
                fn IGraphicsCaptureItemStatics<R, F: FnOnce(&IGraphicsCaptureItemStatics) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<GraphicsCaptureItem, IGraphicsCaptureItemStatics> =
                        windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                fn IGraphicsCaptureItemStatics2<R, F: FnOnce(&IGraphicsCaptureItemStatics2) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<GraphicsCaptureItem, IGraphicsCaptureItemStatics2> =
                        windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for GraphicsCaptureItem {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGraphicsCaptureItem>();
            }
            unsafe impl windows_core::Interface for GraphicsCaptureItem {
                type Vtable = <IGraphicsCaptureItem as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IGraphicsCaptureItem as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for GraphicsCaptureItem {
                const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCaptureItem";
            }
            unsafe impl Send for GraphicsCaptureItem {}
            unsafe impl Sync for GraphicsCaptureItem {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct GraphicsCaptureSession(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(GraphicsCaptureSession, windows_core::IUnknown, windows_core::IInspectable);
            windows_core::imp::required_hierarchy!(GraphicsCaptureSession, super::super::Foundation::IClosable);
            impl GraphicsCaptureSession {
                pub fn Close(&self) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
                    unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
                }
                pub fn StartCapture(&self) -> windows_core::Result<()> {
                    let this = self;
                    unsafe { (windows_core::Interface::vtable(this).StartCapture)(windows_core::Interface::as_raw(this)).ok() }
                }
                pub fn SetIsCursorCaptureEnabled(&self, value: bool) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<IGraphicsCaptureSession2>(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this).SetIsCursorCaptureEnabled)(windows_core::Interface::as_raw(this), value).ok()
                    }
                }
                pub fn SetIsBorderRequired(&self, value: bool) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<IGraphicsCaptureSession3>(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this).SetIsBorderRequired)(windows_core::Interface::as_raw(this), value).ok()
                    }
                }
                fn IGraphicsCaptureSessionStatics<R, F: FnOnce(&IGraphicsCaptureSessionStatics) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<GraphicsCaptureSession, IGraphicsCaptureSessionStatics> =
                        windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for GraphicsCaptureSession {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IGraphicsCaptureSession>();
            }
            unsafe impl windows_core::Interface for GraphicsCaptureSession {
                type Vtable = <IGraphicsCaptureSession as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IGraphicsCaptureSession as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for GraphicsCaptureSession {
                const NAME: &'static str = "Windows.Graphics.Capture.GraphicsCaptureSession";
            }
            unsafe impl Send for GraphicsCaptureSession {}
            unsafe impl Sync for GraphicsCaptureSession {}
            windows_core::imp::define_interface!(
                IDirect3D11CaptureFrame,
                IDirect3D11CaptureFrame_Vtbl,
                0xfa50c623_38da_4b32_acf3_fa9734ad800e
            );
            impl windows_core::RuntimeType for IDirect3D11CaptureFrame {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IDirect3D11CaptureFrame {
                const NAME: &'static str = "Windows.Graphics.Capture.IDirect3D11CaptureFrame";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IDirect3D11CaptureFrame_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Surface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SystemRelativeTime:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
                pub ContentSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SizeInt32) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IDirect3D11CaptureFrame2,
                IDirect3D11CaptureFrame2_Vtbl,
                0x37869cfa_2b48_5ebf_9afb_dffd805defdb
            );
            impl windows_core::RuntimeType for IDirect3D11CaptureFrame2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IDirect3D11CaptureFrame2 {
                const NAME: &'static str = "Windows.Graphics.Capture.IDirect3D11CaptureFrame2";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IDirect3D11CaptureFrame2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub DirtyRegions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                DirtyRegionMode: usize,
            }
            windows_core::imp::define_interface!(
                IDirect3D11CaptureFramePool,
                IDirect3D11CaptureFramePool_Vtbl,
                0x24eb6d22_1975_422e_82e7_780dbd8ddf24
            );
            impl windows_core::RuntimeType for IDirect3D11CaptureFramePool {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IDirect3D11CaptureFramePool {
                const NAME: &'static str = "Windows.Graphics.Capture.IDirect3D11CaptureFramePool";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IDirect3D11CaptureFramePool_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Recreate: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    super::DirectX::DirectXPixelFormat,
                    i32,
                    super::SizeInt32,
                ) -> windows_core::HRESULT,
                pub TryGetNextFrame:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub FrameArrived:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveFrameArrived: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub CreateCaptureSession: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                DispatcherQueue: usize,
            }
            windows_core::imp::define_interface!(
                IDirect3D11CaptureFramePoolStatics,
                IDirect3D11CaptureFramePoolStatics_Vtbl,
                0x7784056a_67aa_4d53_ae54_1088d5a8ca21
            );
            impl windows_core::RuntimeType for IDirect3D11CaptureFramePoolStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IDirect3D11CaptureFramePoolStatics {
                const NAME: &'static str = "Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IDirect3D11CaptureFramePoolStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Create: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    super::DirectX::DirectXPixelFormat,
                    i32,
                    super::SizeInt32,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IDirect3D11CaptureFramePoolStatics2,
                IDirect3D11CaptureFramePoolStatics2_Vtbl,
                0x589b103f_6bbc_5df5_a991_02e28b3b66d5
            );
            impl windows_core::RuntimeType for IDirect3D11CaptureFramePoolStatics2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IDirect3D11CaptureFramePoolStatics2 {
                const NAME: &'static str = "Windows.Graphics.Capture.IDirect3D11CaptureFramePoolStatics2";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IDirect3D11CaptureFramePoolStatics2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CreateFreeThreaded: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    super::DirectX::DirectXPixelFormat,
                    i32,
                    super::SizeInt32,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IGraphicsCaptureItem, IGraphicsCaptureItem_Vtbl, 0x79c3f95b_31f7_4ec2_a464_632ef5d30760);
            impl windows_core::RuntimeType for IGraphicsCaptureItem {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IGraphicsCaptureItem {
                const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureItem";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IGraphicsCaptureItem_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SizeInt32) -> windows_core::HRESULT,
                pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IGraphicsCaptureItemStatics,
                IGraphicsCaptureItemStatics_Vtbl,
                0xa87ebea5_457c_5788_ab47_0cf1d3637e74
            );
            impl windows_core::RuntimeType for IGraphicsCaptureItemStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IGraphicsCaptureItemStatics {
                const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureItemStatics";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IGraphicsCaptureItemStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                CreateFromVisual: usize,
            }
            windows_core::imp::define_interface!(
                IGraphicsCaptureItemStatics2,
                IGraphicsCaptureItemStatics2_Vtbl,
                0x3b92acc9_e584_5862_bf5c_9c316c6d2dbb
            );
            impl windows_core::RuntimeType for IGraphicsCaptureItemStatics2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IGraphicsCaptureItemStatics2 {
                const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureItemStatics2";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IGraphicsCaptureItemStatics2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                TryCreateFromWindowId: usize,
                TryCreateFromDisplayId: usize,
            }
            windows_core::imp::define_interface!(
                IGraphicsCaptureSession,
                IGraphicsCaptureSession_Vtbl,
                0x814e42a9_f70f_4ad7_939b_fddcc6eb880d
            );
            impl windows_core::RuntimeType for IGraphicsCaptureSession {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IGraphicsCaptureSession {
                const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureSession";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IGraphicsCaptureSession_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub StartCapture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IGraphicsCaptureSession2,
                IGraphicsCaptureSession2_Vtbl,
                0x2c39ae40_7d2e_5044_804e_8b6799d4cf9e
            );
            impl windows_core::RuntimeType for IGraphicsCaptureSession2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IGraphicsCaptureSession2 {
                const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureSession2";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IGraphicsCaptureSession2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub IsCursorCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetIsCursorCaptureEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IGraphicsCaptureSession3,
                IGraphicsCaptureSession3_Vtbl,
                0xf2cdd966_22ae_5ea1_9596_3a289344c3be
            );
            impl windows_core::RuntimeType for IGraphicsCaptureSession3 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IGraphicsCaptureSession3 {
                const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureSession3";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IGraphicsCaptureSession3_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub IsBorderRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetIsBorderRequired: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IGraphicsCaptureSession4,
                IGraphicsCaptureSession4_Vtbl,
                0xae99813c_c257_5759_8ed0_668c9b557ed4
            );
            impl windows_core::RuntimeType for IGraphicsCaptureSession4 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IGraphicsCaptureSession4 {
                const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureSession4";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IGraphicsCaptureSession4_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                DirtyRegionMode: usize,
                SetDirtyRegionMode: usize,
            }
            windows_core::imp::define_interface!(
                IGraphicsCaptureSession5,
                IGraphicsCaptureSession5_Vtbl,
                0x67c0ea62_1f85_5061_925a_239be0ac09cb
            );
            impl windows_core::RuntimeType for IGraphicsCaptureSession5 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IGraphicsCaptureSession5 {
                const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureSession5";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IGraphicsCaptureSession5_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub MinUpdateInterval:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
                pub SetMinUpdateInterval:
                    unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IGraphicsCaptureSession6,
                IGraphicsCaptureSession6_Vtbl,
                0xd7419236_be20_5e9f_bcd6_c4e98fd6afdc
            );
            impl windows_core::RuntimeType for IGraphicsCaptureSession6 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IGraphicsCaptureSession6 {
                const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureSession6";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IGraphicsCaptureSession6_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub IncludeSecondaryWindows: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetIncludeSecondaryWindows: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IGraphicsCaptureSessionStatics,
                IGraphicsCaptureSessionStatics_Vtbl,
                0x2224a540_5974_49aa_b232_0882536f4cb5
            );
            impl windows_core::RuntimeType for IGraphicsCaptureSessionStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IGraphicsCaptureSessionStatics {
                const NAME: &'static str = "Windows.Graphics.Capture.IGraphicsCaptureSessionStatics";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IGraphicsCaptureSessionStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
            }
        }
        pub mod DirectX {
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct DirectXPixelFormat(pub i32);
            impl DirectXPixelFormat {
                pub const Unknown: Self = Self(0i32);
                pub const R32G32B32A32Typeless: Self = Self(1i32);
                pub const R32G32B32A32Float: Self = Self(2i32);
                pub const R32G32B32A32UInt: Self = Self(3i32);
                pub const R32G32B32A32Int: Self = Self(4i32);
                pub const R32G32B32Typeless: Self = Self(5i32);
                pub const R32G32B32Float: Self = Self(6i32);
                pub const R32G32B32UInt: Self = Self(7i32);
                pub const R32G32B32Int: Self = Self(8i32);
                pub const R16G16B16A16Typeless: Self = Self(9i32);
                pub const R16G16B16A16Float: Self = Self(10i32);
                pub const R16G16B16A16UIntNormalized: Self = Self(11i32);
                pub const R16G16B16A16UInt: Self = Self(12i32);
                pub const R16G16B16A16IntNormalized: Self = Self(13i32);
                pub const R16G16B16A16Int: Self = Self(14i32);
                pub const R32G32Typeless: Self = Self(15i32);
                pub const R32G32Float: Self = Self(16i32);
                pub const R32G32UInt: Self = Self(17i32);
                pub const R32G32Int: Self = Self(18i32);
                pub const R32G8X24Typeless: Self = Self(19i32);
                pub const D32FloatS8X24UInt: Self = Self(20i32);
                pub const R32FloatX8X24Typeless: Self = Self(21i32);
                pub const X32TypelessG8X24UInt: Self = Self(22i32);
                pub const R10G10B10A2Typeless: Self = Self(23i32);
                pub const R10G10B10A2UIntNormalized: Self = Self(24i32);
                pub const R10G10B10A2UInt: Self = Self(25i32);
                pub const R11G11B10Float: Self = Self(26i32);
                pub const R8G8B8A8Typeless: Self = Self(27i32);
                pub const R8G8B8A8UIntNormalized: Self = Self(28i32);
                pub const R8G8B8A8UIntNormalizedSrgb: Self = Self(29i32);
                pub const R8G8B8A8UInt: Self = Self(30i32);
                pub const R8G8B8A8IntNormalized: Self = Self(31i32);
                pub const R8G8B8A8Int: Self = Self(32i32);
                pub const R16G16Typeless: Self = Self(33i32);
                pub const R16G16Float: Self = Self(34i32);
                pub const R16G16UIntNormalized: Self = Self(35i32);
                pub const R16G16UInt: Self = Self(36i32);
                pub const R16G16IntNormalized: Self = Self(37i32);
                pub const R16G16Int: Self = Self(38i32);
                pub const R32Typeless: Self = Self(39i32);
                pub const D32Float: Self = Self(40i32);
                pub const R32Float: Self = Self(41i32);
                pub const R32UInt: Self = Self(42i32);
                pub const R32Int: Self = Self(43i32);
                pub const R24G8Typeless: Self = Self(44i32);
                pub const D24UIntNormalizedS8UInt: Self = Self(45i32);
                pub const R24UIntNormalizedX8Typeless: Self = Self(46i32);
                pub const X24TypelessG8UInt: Self = Self(47i32);
                pub const R8G8Typeless: Self = Self(48i32);
                pub const R8G8UIntNormalized: Self = Self(49i32);
                pub const R8G8UInt: Self = Self(50i32);
                pub const R8G8IntNormalized: Self = Self(51i32);
                pub const R8G8Int: Self = Self(52i32);
                pub const R16Typeless: Self = Self(53i32);
                pub const R16Float: Self = Self(54i32);
                pub const D16UIntNormalized: Self = Self(55i32);
                pub const R16UIntNormalized: Self = Self(56i32);
                pub const R16UInt: Self = Self(57i32);
                pub const R16IntNormalized: Self = Self(58i32);
                pub const R16Int: Self = Self(59i32);
                pub const R8Typeless: Self = Self(60i32);
                pub const R8UIntNormalized: Self = Self(61i32);
                pub const R8UInt: Self = Self(62i32);
                pub const R8IntNormalized: Self = Self(63i32);
                pub const R8Int: Self = Self(64i32);
                pub const A8UIntNormalized: Self = Self(65i32);
                pub const R1UIntNormalized: Self = Self(66i32);
                pub const R9G9B9E5SharedExponent: Self = Self(67i32);
                pub const R8G8B8G8UIntNormalized: Self = Self(68i32);
                pub const G8R8G8B8UIntNormalized: Self = Self(69i32);
                pub const BC1Typeless: Self = Self(70i32);
                pub const BC1UIntNormalized: Self = Self(71i32);
                pub const BC1UIntNormalizedSrgb: Self = Self(72i32);
                pub const BC2Typeless: Self = Self(73i32);
                pub const BC2UIntNormalized: Self = Self(74i32);
                pub const BC2UIntNormalizedSrgb: Self = Self(75i32);
                pub const BC3Typeless: Self = Self(76i32);
                pub const BC3UIntNormalized: Self = Self(77i32);
                pub const BC3UIntNormalizedSrgb: Self = Self(78i32);
                pub const BC4Typeless: Self = Self(79i32);
                pub const BC4UIntNormalized: Self = Self(80i32);
                pub const BC4IntNormalized: Self = Self(81i32);
                pub const BC5Typeless: Self = Self(82i32);
                pub const BC5UIntNormalized: Self = Self(83i32);
                pub const BC5IntNormalized: Self = Self(84i32);
                pub const B5G6R5UIntNormalized: Self = Self(85i32);
                pub const B5G5R5A1UIntNormalized: Self = Self(86i32);
                pub const B8G8R8A8UIntNormalized: Self = Self(87i32);
                pub const B8G8R8X8UIntNormalized: Self = Self(88i32);
                pub const R10G10B10XRBiasA2UIntNormalized: Self = Self(89i32);
                pub const B8G8R8A8Typeless: Self = Self(90i32);
                pub const B8G8R8A8UIntNormalizedSrgb: Self = Self(91i32);
                pub const B8G8R8X8Typeless: Self = Self(92i32);
                pub const B8G8R8X8UIntNormalizedSrgb: Self = Self(93i32);
                pub const BC6HTypeless: Self = Self(94i32);
                pub const BC6H16UnsignedFloat: Self = Self(95i32);
                pub const BC6H16Float: Self = Self(96i32);
                pub const BC7Typeless: Self = Self(97i32);
                pub const BC7UIntNormalized: Self = Self(98i32);
                pub const BC7UIntNormalizedSrgb: Self = Self(99i32);
                pub const Ayuv: Self = Self(100i32);
                pub const Y410: Self = Self(101i32);
                pub const Y416: Self = Self(102i32);
                pub const NV12: Self = Self(103i32);
                pub const P010: Self = Self(104i32);
                pub const P016: Self = Self(105i32);
                pub const Opaque420: Self = Self(106i32);
                pub const Yuy2: Self = Self(107i32);
                pub const Y210: Self = Self(108i32);
                pub const Y216: Self = Self(109i32);
                pub const NV11: Self = Self(110i32);
                pub const AI44: Self = Self(111i32);
                pub const IA44: Self = Self(112i32);
                pub const P8: Self = Self(113i32);
                pub const A8P8: Self = Self(114i32);
                pub const B4G4R4A4UIntNormalized: Self = Self(115i32);
                pub const P208: Self = Self(130i32);
                pub const V208: Self = Self(131i32);
                pub const V408: Self = Self(132i32);
                pub const SamplerFeedbackMinMipOpaque: Self = Self(189i32);
                pub const SamplerFeedbackMipRegionUsedOpaque: Self = Self(190i32);
                pub const A4B4G4R4: Self = Self(191i32);
            }
            impl windows_core::TypeKind for DirectXPixelFormat {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for DirectXPixelFormat {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.DirectX.DirectXPixelFormat;i4)");
            }
            pub mod Direct3D11 {
                windows_core::imp::define_interface!(IDirect3DDevice, IDirect3DDevice_Vtbl, 0xa37624ab_8d5f_4650_9d3e_9eae3d9bc670);
                impl windows_core::RuntimeType for IDirect3DDevice {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                windows_core::imp::interface_hierarchy!(IDirect3DDevice, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(IDirect3DDevice, super::super::super::Foundation::IClosable);
                impl IDirect3DDevice {
                    pub fn Close(&self) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
                    }
                }
                impl windows_core::RuntimeName for IDirect3DDevice {
                    const NAME: &'static str = "Windows.Graphics.DirectX.Direct3D11.IDirect3DDevice";
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IDirect3DDevice_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Trim: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IDirect3DSurface, IDirect3DSurface_Vtbl, 0x0bf4a146_13c1_4694_bee3_7abf15eaf586);
                impl windows_core::RuntimeType for IDirect3DSurface {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                windows_core::imp::interface_hierarchy!(IDirect3DSurface, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(IDirect3DSurface, super::super::super::Foundation::IClosable);
                impl IDirect3DSurface {
                    pub fn Close(&self) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
                    }
                }
                impl windows_core::RuntimeName for IDirect3DSurface {
                    const NAME: &'static str = "Windows.Graphics.DirectX.Direct3D11.IDirect3DSurface";
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IDirect3DSurface_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    Description: usize,
                }
            }
        }
    }
    pub mod Media {
        pub mod Control {
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct CurrentSessionChangedEventArgs(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(CurrentSessionChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
            impl CurrentSessionChangedEventArgs {}
            impl windows_core::RuntimeType for CurrentSessionChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, ICurrentSessionChangedEventArgs>();
            }
            unsafe impl windows_core::Interface for CurrentSessionChangedEventArgs {
                type Vtable = <ICurrentSessionChangedEventArgs as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <ICurrentSessionChangedEventArgs as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for CurrentSessionChangedEventArgs {
                const NAME: &'static str = "Windows.Media.Control.CurrentSessionChangedEventArgs";
            }
            unsafe impl Send for CurrentSessionChangedEventArgs {}
            unsafe impl Sync for CurrentSessionChangedEventArgs {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct GlobalSystemMediaTransportControlsSession(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                GlobalSystemMediaTransportControlsSession,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl GlobalSystemMediaTransportControlsSession {
                pub fn SourceAppUserModelId(&self) -> windows_core::Result<windows_core::HSTRING> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).SourceAppUserModelId)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn TryGetMediaPropertiesAsync(
                    &self,
                ) -> windows_core::Result<windows_future::IAsyncOperation<GlobalSystemMediaTransportControlsSessionMediaProperties>>
                {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).TryGetMediaPropertiesAsync)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn GetTimelineProperties(&self) -> windows_core::Result<GlobalSystemMediaTransportControlsSessionTimelineProperties> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).GetTimelineProperties)(windows_core::Interface::as_raw(this), &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn GetPlaybackInfo(&self) -> windows_core::Result<GlobalSystemMediaTransportControlsSessionPlaybackInfo> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).GetPlaybackInfo)(windows_core::Interface::as_raw(this), &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn TrySkipNextAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).TrySkipNextAsync)(windows_core::Interface::as_raw(this), &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn TrySkipPreviousAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).TrySkipPreviousAsync)(windows_core::Interface::as_raw(this), &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn TryTogglePlayPauseAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).TryTogglePlayPauseAsync)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn TryChangePlaybackPositionAsync(
                    &self,
                    requestedplaybackposition: i64,
                ) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).TryChangePlaybackPositionAsync)(
                            windows_core::Interface::as_raw(this),
                            requestedplaybackposition,
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn TimelinePropertiesChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
                where
                    P0: windows_core::Param<
                        super::super::Foundation::TypedEventHandler<
                            GlobalSystemMediaTransportControlsSession,
                            TimelinePropertiesChangedEventArgs,
                        >,
                    >,
                {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).TimelinePropertiesChanged)(
                            windows_core::Interface::as_raw(this),
                            handler.param().abi(),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn PlaybackInfoChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
                where
                    P0: windows_core::Param<
                        super::super::Foundation::TypedEventHandler<
                            GlobalSystemMediaTransportControlsSession,
                            PlaybackInfoChangedEventArgs,
                        >,
                    >,
                {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).PlaybackInfoChanged)(
                            windows_core::Interface::as_raw(this),
                            handler.param().abi(),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn MediaPropertiesChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
                where
                    P0: windows_core::Param<
                        super::super::Foundation::TypedEventHandler<
                            GlobalSystemMediaTransportControlsSession,
                            MediaPropertiesChangedEventArgs,
                        >,
                    >,
                {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).MediaPropertiesChanged)(
                            windows_core::Interface::as_raw(this),
                            handler.param().abi(),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for GlobalSystemMediaTransportControlsSession {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IGlobalSystemMediaTransportControlsSession>();
            }
            unsafe impl windows_core::Interface for GlobalSystemMediaTransportControlsSession {
                type Vtable = <IGlobalSystemMediaTransportControlsSession as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IGlobalSystemMediaTransportControlsSession as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for GlobalSystemMediaTransportControlsSession {
                const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSession";
            }
            unsafe impl Send for GlobalSystemMediaTransportControlsSession {}
            unsafe impl Sync for GlobalSystemMediaTransportControlsSession {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct GlobalSystemMediaTransportControlsSessionManager(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                GlobalSystemMediaTransportControlsSessionManager,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl GlobalSystemMediaTransportControlsSessionManager {
                pub fn GetCurrentSession(&self) -> windows_core::Result<GlobalSystemMediaTransportControlsSession> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).GetCurrentSession)(windows_core::Interface::as_raw(this), &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn GetSessions(
                    &self,
                ) -> windows_core::Result<windows_collections::IVectorView<GlobalSystemMediaTransportControlsSession>> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).GetSessions)(windows_core::Interface::as_raw(this), &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn CurrentSessionChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
                where
                    P0: windows_core::Param<
                        super::super::Foundation::TypedEventHandler<
                            GlobalSystemMediaTransportControlsSessionManager,
                            CurrentSessionChangedEventArgs,
                        >,
                    >,
                {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CurrentSessionChanged)(
                            windows_core::Interface::as_raw(this),
                            handler.param().abi(),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn SessionsChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
                where
                    P0: windows_core::Param<
                        super::super::Foundation::TypedEventHandler<
                            GlobalSystemMediaTransportControlsSessionManager,
                            SessionsChangedEventArgs,
                        >,
                    >,
                {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).SessionsChanged)(
                            windows_core::Interface::as_raw(this),
                            handler.param().abi(),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn RequestAsync(
                ) -> windows_core::Result<windows_future::IAsyncOperation<GlobalSystemMediaTransportControlsSessionManager>>
                {
                    Self::IGlobalSystemMediaTransportControlsSessionManagerStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).RequestAsync)(windows_core::Interface::as_raw(this), &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    })
                }
                fn IGlobalSystemMediaTransportControlsSessionManagerStatics<
                    R,
                    F: FnOnce(&IGlobalSystemMediaTransportControlsSessionManagerStatics) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        GlobalSystemMediaTransportControlsSessionManager,
                        IGlobalSystemMediaTransportControlsSessionManagerStatics,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionManager {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IGlobalSystemMediaTransportControlsSessionManager>();
            }
            unsafe impl windows_core::Interface for GlobalSystemMediaTransportControlsSessionManager {
                type Vtable = <IGlobalSystemMediaTransportControlsSessionManager as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionManager as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionManager {
                const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager";
            }
            unsafe impl Send for GlobalSystemMediaTransportControlsSessionManager {}
            unsafe impl Sync for GlobalSystemMediaTransportControlsSessionManager {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct GlobalSystemMediaTransportControlsSessionMediaProperties(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                GlobalSystemMediaTransportControlsSessionMediaProperties,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl GlobalSystemMediaTransportControlsSessionMediaProperties {
                pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Artist(&self) -> windows_core::Result<windows_core::HSTRING> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Artist)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| core::mem::transmute(result__))
                    }
                }
                pub fn Thumbnail(&self) -> windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Thumbnail)(windows_core::Interface::as_raw(this), &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
            }
            impl windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionMediaProperties {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IGlobalSystemMediaTransportControlsSessionMediaProperties>();
            }
            unsafe impl windows_core::Interface for GlobalSystemMediaTransportControlsSessionMediaProperties {
                type Vtable = <IGlobalSystemMediaTransportControlsSessionMediaProperties as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionMediaProperties as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionMediaProperties {
                const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionMediaProperties";
            }
            unsafe impl Send for GlobalSystemMediaTransportControlsSessionMediaProperties {}
            unsafe impl Sync for GlobalSystemMediaTransportControlsSessionMediaProperties {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct GlobalSystemMediaTransportControlsSessionPlaybackControls(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                GlobalSystemMediaTransportControlsSessionPlaybackControls,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl GlobalSystemMediaTransportControlsSessionPlaybackControls {
                pub fn IsPlaybackPositionEnabled(&self) -> windows_core::Result<bool> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).IsPlaybackPositionEnabled)(
                            windows_core::Interface::as_raw(this),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackControls {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IGlobalSystemMediaTransportControlsSessionPlaybackControls>();
            }
            unsafe impl windows_core::Interface for GlobalSystemMediaTransportControlsSessionPlaybackControls {
                type Vtable = <IGlobalSystemMediaTransportControlsSessionPlaybackControls as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID =
                    <IGlobalSystemMediaTransportControlsSessionPlaybackControls as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionPlaybackControls {
                const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackControls";
            }
            unsafe impl Send for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
            unsafe impl Sync for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct GlobalSystemMediaTransportControlsSessionPlaybackInfo(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                GlobalSystemMediaTransportControlsSessionPlaybackInfo,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl GlobalSystemMediaTransportControlsSessionPlaybackInfo {
                pub fn Controls(&self) -> windows_core::Result<GlobalSystemMediaTransportControlsSessionPlaybackControls> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Controls)(windows_core::Interface::as_raw(this), &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn PlaybackStatus(&self) -> windows_core::Result<GlobalSystemMediaTransportControlsSessionPlaybackStatus> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).PlaybackStatus)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IGlobalSystemMediaTransportControlsSessionPlaybackInfo>();
            }
            unsafe impl windows_core::Interface for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
                type Vtable = <IGlobalSystemMediaTransportControlsSessionPlaybackInfo as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IGlobalSystemMediaTransportControlsSessionPlaybackInfo as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
                const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackInfo";
            }
            unsafe impl Send for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
            unsafe impl Sync for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct GlobalSystemMediaTransportControlsSessionPlaybackStatus(pub i32);
            impl GlobalSystemMediaTransportControlsSessionPlaybackStatus {
                pub const Closed: Self = Self(0i32);
                pub const Opened: Self = Self(1i32);
                pub const Changing: Self = Self(2i32);
                pub const Stopped: Self = Self(3i32);
                pub const Playing: Self = Self(4i32);
                pub const Paused: Self = Self(5i32);
            }
            impl windows_core::TypeKind for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"enum(Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackStatus;i4)",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct GlobalSystemMediaTransportControlsSessionTimelineProperties(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                GlobalSystemMediaTransportControlsSessionTimelineProperties,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl GlobalSystemMediaTransportControlsSessionTimelineProperties {
                pub fn StartTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn EndTime(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).EndTime)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn Position(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn LastUpdatedTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).LastUpdatedTime)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeType for GlobalSystemMediaTransportControlsSessionTimelineProperties {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IGlobalSystemMediaTransportControlsSessionTimelineProperties>();
            }
            unsafe impl windows_core::Interface for GlobalSystemMediaTransportControlsSessionTimelineProperties {
                type Vtable = <IGlobalSystemMediaTransportControlsSessionTimelineProperties as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID =
                    <IGlobalSystemMediaTransportControlsSessionTimelineProperties as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for GlobalSystemMediaTransportControlsSessionTimelineProperties {
                const NAME: &'static str = "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionTimelineProperties";
            }
            unsafe impl Send for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
            unsafe impl Sync for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
            windows_core::imp::define_interface!(
                ICurrentSessionChangedEventArgs,
                ICurrentSessionChangedEventArgs_Vtbl,
                0x6969cb39_0bfa_5fe0_8d73_09cc5e5408e1
            );
            impl windows_core::RuntimeType for ICurrentSessionChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for ICurrentSessionChangedEventArgs {
                const NAME: &'static str = "Windows.Media.Control.ICurrentSessionChangedEventArgs";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ICurrentSessionChangedEventArgs_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                IGlobalSystemMediaTransportControlsSession,
                IGlobalSystemMediaTransportControlsSession_Vtbl,
                0x7148c835_9b14_5ae2_ab85_dc9b1c14e1a8
            );
            impl windows_core::RuntimeType for IGlobalSystemMediaTransportControlsSession {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IGlobalSystemMediaTransportControlsSession {
                const NAME: &'static str = "Windows.Media.Control.IGlobalSystemMediaTransportControlsSession";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IGlobalSystemMediaTransportControlsSession_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub SourceAppUserModelId:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TryGetMediaPropertiesAsync:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub GetTimelineProperties:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub GetPlaybackInfo:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TryPlayAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TryPauseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TryStopAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TryRecordAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TryFastForwardAsync:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TryRewindAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TrySkipNextAsync:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TrySkipPreviousAsync:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TryChangeChannelUpAsync:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TryChangeChannelDownAsync:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TryTogglePlayPauseAsync:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                TryChangeAutoRepeatModeAsync: usize,
                pub TryChangePlaybackRateAsync:
                    unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TryChangeShuffleActiveAsync:
                    unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TryChangePlaybackPositionAsync:
                    unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TimelinePropertiesChanged:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveTimelinePropertiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub PlaybackInfoChanged:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemovePlaybackInfoChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub MediaPropertiesChanged:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveMediaPropertiesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IGlobalSystemMediaTransportControlsSessionManager,
                IGlobalSystemMediaTransportControlsSessionManager_Vtbl,
                0xcace8eac_e86e_504a_ab31_5ff8ff1bce49
            );
            impl windows_core::RuntimeType for IGlobalSystemMediaTransportControlsSessionManager {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IGlobalSystemMediaTransportControlsSessionManager {
                const NAME: &'static str = "Windows.Media.Control.IGlobalSystemMediaTransportControlsSessionManager";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IGlobalSystemMediaTransportControlsSessionManager_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub GetCurrentSession:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub GetSessions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub CurrentSessionChanged:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveCurrentSessionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub SessionsChanged:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveSessionsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IGlobalSystemMediaTransportControlsSessionManagerStatics,
                IGlobalSystemMediaTransportControlsSessionManagerStatics_Vtbl,
                0x2050c4ee_11a0_57de_aed7_c97c70338245
            );
            impl windows_core::RuntimeType for IGlobalSystemMediaTransportControlsSessionManagerStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IGlobalSystemMediaTransportControlsSessionManagerStatics {
                const NAME: &'static str = "Windows.Media.Control.IGlobalSystemMediaTransportControlsSessionManagerStatics";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IGlobalSystemMediaTransportControlsSessionManagerStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub RequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IGlobalSystemMediaTransportControlsSessionMediaProperties,
                IGlobalSystemMediaTransportControlsSessionMediaProperties_Vtbl,
                0x68856cf6_adb4_54b2_ac16_05837907acb6
            );
            impl windows_core::RuntimeType for IGlobalSystemMediaTransportControlsSessionMediaProperties {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IGlobalSystemMediaTransportControlsSessionMediaProperties {
                const NAME: &'static str = "Windows.Media.Control.IGlobalSystemMediaTransportControlsSessionMediaProperties";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IGlobalSystemMediaTransportControlsSessionMediaProperties_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub Subtitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub AlbumArtist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub Artist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub AlbumTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                pub Genres: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub AlbumTrackCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                PlaybackType: usize,
                pub Thumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IGlobalSystemMediaTransportControlsSessionPlaybackControls,
                IGlobalSystemMediaTransportControlsSessionPlaybackControls_Vtbl,
                0x6501a3e6_bc7a_503a_bb1b_68f158f3fb03
            );
            impl windows_core::RuntimeType for IGlobalSystemMediaTransportControlsSessionPlaybackControls {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IGlobalSystemMediaTransportControlsSessionPlaybackControls {
                const NAME: &'static str = "Windows.Media.Control.IGlobalSystemMediaTransportControlsSessionPlaybackControls";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IGlobalSystemMediaTransportControlsSessionPlaybackControls_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub IsPlayEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub IsPauseEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub IsStopEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub IsRecordEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub IsFastForwardEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub IsRewindEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub IsNextEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub IsPreviousEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub IsChannelUpEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub IsChannelDownEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub IsPlayPauseToggleEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub IsShuffleEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub IsRepeatEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub IsPlaybackRateEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub IsPlaybackPositionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IGlobalSystemMediaTransportControlsSessionPlaybackInfo,
                IGlobalSystemMediaTransportControlsSessionPlaybackInfo_Vtbl,
                0x94b4b6cf_e8ba_51ad_87a7_c10ade106127
            );
            impl windows_core::RuntimeType for IGlobalSystemMediaTransportControlsSessionPlaybackInfo {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IGlobalSystemMediaTransportControlsSessionPlaybackInfo {
                const NAME: &'static str = "Windows.Media.Control.IGlobalSystemMediaTransportControlsSessionPlaybackInfo";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IGlobalSystemMediaTransportControlsSessionPlaybackInfo_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Controls: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub PlaybackStatus: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut GlobalSystemMediaTransportControlsSessionPlaybackStatus,
                ) -> windows_core::HRESULT,
                PlaybackType: usize,
                AutoRepeatMode: usize,
                pub PlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub IsShuffleActive:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IGlobalSystemMediaTransportControlsSessionTimelineProperties,
                IGlobalSystemMediaTransportControlsSessionTimelineProperties_Vtbl,
                0xede34136_6f25_588d_8ecf_ea5b6735aaa5
            );
            impl windows_core::RuntimeType for IGlobalSystemMediaTransportControlsSessionTimelineProperties {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IGlobalSystemMediaTransportControlsSessionTimelineProperties {
                const NAME: &'static str = "Windows.Media.Control.IGlobalSystemMediaTransportControlsSessionTimelineProperties";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IGlobalSystemMediaTransportControlsSessionTimelineProperties_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub StartTime:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
                pub EndTime:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
                pub MinSeekTime:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
                pub MaxSeekTime:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
                pub Position:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
                pub LastUpdatedTime:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IMediaPropertiesChangedEventArgs,
                IMediaPropertiesChangedEventArgs_Vtbl,
                0x7d3741cb_adf0_5cef_91ba_cfabcdd77678
            );
            impl windows_core::RuntimeType for IMediaPropertiesChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IMediaPropertiesChangedEventArgs {
                const NAME: &'static str = "Windows.Media.Control.IMediaPropertiesChangedEventArgs";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IMediaPropertiesChangedEventArgs_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                IPlaybackInfoChangedEventArgs,
                IPlaybackInfoChangedEventArgs_Vtbl,
                0x786756c2_bc0d_50a5_8807_054291fef139
            );
            impl windows_core::RuntimeType for IPlaybackInfoChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IPlaybackInfoChangedEventArgs {
                const NAME: &'static str = "Windows.Media.Control.IPlaybackInfoChangedEventArgs";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IPlaybackInfoChangedEventArgs_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                ISessionsChangedEventArgs,
                ISessionsChangedEventArgs_Vtbl,
                0xbbf0cd32_42c4_5a58_b317_f34bbfbd26e0
            );
            impl windows_core::RuntimeType for ISessionsChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for ISessionsChangedEventArgs {
                const NAME: &'static str = "Windows.Media.Control.ISessionsChangedEventArgs";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ISessionsChangedEventArgs_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                ITimelinePropertiesChangedEventArgs,
                ITimelinePropertiesChangedEventArgs_Vtbl,
                0x29033a2f_c923_5a77_bcaf_055ff415ad32
            );
            impl windows_core::RuntimeType for ITimelinePropertiesChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for ITimelinePropertiesChangedEventArgs {
                const NAME: &'static str = "Windows.Media.Control.ITimelinePropertiesChangedEventArgs";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ITimelinePropertiesChangedEventArgs_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct MediaPropertiesChangedEventArgs(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(MediaPropertiesChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
            impl MediaPropertiesChangedEventArgs {}
            impl windows_core::RuntimeType for MediaPropertiesChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IMediaPropertiesChangedEventArgs>();
            }
            unsafe impl windows_core::Interface for MediaPropertiesChangedEventArgs {
                type Vtable = <IMediaPropertiesChangedEventArgs as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IMediaPropertiesChangedEventArgs as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for MediaPropertiesChangedEventArgs {
                const NAME: &'static str = "Windows.Media.Control.MediaPropertiesChangedEventArgs";
            }
            unsafe impl Send for MediaPropertiesChangedEventArgs {}
            unsafe impl Sync for MediaPropertiesChangedEventArgs {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct PlaybackInfoChangedEventArgs(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(PlaybackInfoChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
            impl PlaybackInfoChangedEventArgs {}
            impl windows_core::RuntimeType for PlaybackInfoChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IPlaybackInfoChangedEventArgs>();
            }
            unsafe impl windows_core::Interface for PlaybackInfoChangedEventArgs {
                type Vtable = <IPlaybackInfoChangedEventArgs as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IPlaybackInfoChangedEventArgs as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for PlaybackInfoChangedEventArgs {
                const NAME: &'static str = "Windows.Media.Control.PlaybackInfoChangedEventArgs";
            }
            unsafe impl Send for PlaybackInfoChangedEventArgs {}
            unsafe impl Sync for PlaybackInfoChangedEventArgs {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct SessionsChangedEventArgs(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(SessionsChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
            impl SessionsChangedEventArgs {}
            impl windows_core::RuntimeType for SessionsChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, ISessionsChangedEventArgs>();
            }
            unsafe impl windows_core::Interface for SessionsChangedEventArgs {
                type Vtable = <ISessionsChangedEventArgs as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <ISessionsChangedEventArgs as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for SessionsChangedEventArgs {
                const NAME: &'static str = "Windows.Media.Control.SessionsChangedEventArgs";
            }
            unsafe impl Send for SessionsChangedEventArgs {}
            unsafe impl Sync for SessionsChangedEventArgs {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct TimelinePropertiesChangedEventArgs(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(TimelinePropertiesChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
            impl TimelinePropertiesChangedEventArgs {}
            impl windows_core::RuntimeType for TimelinePropertiesChangedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, ITimelinePropertiesChangedEventArgs>();
            }
            unsafe impl windows_core::Interface for TimelinePropertiesChangedEventArgs {
                type Vtable = <ITimelinePropertiesChangedEventArgs as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <ITimelinePropertiesChangedEventArgs as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for TimelinePropertiesChangedEventArgs {
                const NAME: &'static str = "Windows.Media.Control.TimelinePropertiesChangedEventArgs";
            }
            unsafe impl Send for TimelinePropertiesChangedEventArgs {}
            unsafe impl Sync for TimelinePropertiesChangedEventArgs {}
        }
    }
    pub mod Storage {
        pub mod Streams {
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Buffer(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(Buffer, windows_core::IUnknown, windows_core::IInspectable, IBuffer);
            impl Buffer {
                pub fn Length(&self) -> windows_core::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn Create(capacity: u32) -> windows_core::Result<Buffer> {
                    Self::IBufferFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), capacity, &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    })
                }
                fn IBufferFactory<R, F: FnOnce(&IBufferFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<Buffer, IBufferFactory> = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                fn IBufferStatics<R, F: FnOnce(&IBufferStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<Buffer, IBufferStatics> = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for Buffer {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBuffer>();
            }
            unsafe impl windows_core::Interface for Buffer {
                type Vtable = <IBuffer as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IBuffer as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for Buffer {
                const NAME: &'static str = "Windows.Storage.Streams.Buffer";
            }
            unsafe impl Send for Buffer {}
            unsafe impl Sync for Buffer {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct DataReader(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(DataReader, windows_core::IUnknown, windows_core::IInspectable, IDataReader);
            windows_core::imp::required_hierarchy!(DataReader, super::super::Foundation::IClosable);
            impl DataReader {
                pub fn Close(&self) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
                    unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
                }
                pub fn InputStreamOptions(&self) -> windows_core::Result<InputStreamOptions> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).InputStreamOptions)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn ReadBytes(&self, value: &mut [u8]) -> windows_core::Result<()> {
                    let this = self;
                    unsafe {
                        (windows_core::Interface::vtable(this).ReadBytes)(
                            windows_core::Interface::as_raw(this),
                            value.len().try_into().unwrap(),
                            value.as_mut_ptr(),
                        )
                        .ok()
                    }
                }
                pub fn FromBuffer<P0>(buffer: P0) -> windows_core::Result<DataReader>
                where
                    P0: windows_core::Param<IBuffer>,
                {
                    Self::IDataReaderStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).FromBuffer)(
                            windows_core::Interface::as_raw(this),
                            buffer.param().abi(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    })
                }
                fn IDataReaderFactory<R, F: FnOnce(&IDataReaderFactory) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<DataReader, IDataReaderFactory> = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                fn IDataReaderStatics<R, F: FnOnce(&IDataReaderStatics) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<DataReader, IDataReaderStatics> = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for DataReader {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDataReader>();
            }
            unsafe impl windows_core::Interface for DataReader {
                type Vtable = <IDataReader as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IDataReader as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for DataReader {
                const NAME: &'static str = "Windows.Storage.Streams.DataReader";
            }
            unsafe impl Send for DataReader {}
            unsafe impl Sync for DataReader {}
            windows_core::imp::define_interface!(IBuffer, IBuffer_Vtbl, 0x905a0fe0_bc53_11df_8c49_001e4fc686da);
            impl windows_core::RuntimeType for IBuffer {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            windows_core::imp::interface_hierarchy!(IBuffer, windows_core::IUnknown, windows_core::IInspectable);
            impl IBuffer {
                pub fn Length(&self) -> windows_core::Result<u32> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
            }
            impl windows_core::RuntimeName for IBuffer {
                const NAME: &'static str = "Windows.Storage.Streams.IBuffer";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IBuffer_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Capacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IBufferFactory, IBufferFactory_Vtbl, 0x71af914d_c10f_484b_bc50_14bc623b3a27);
            impl windows_core::RuntimeType for IBufferFactory {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IBufferFactory {
                const NAME: &'static str = "Windows.Storage.Streams.IBufferFactory";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IBufferFactory_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IBufferStatics, IBufferStatics_Vtbl, 0xe901e65b_d716_475a_a90a_af7229b1e741);
            impl windows_core::RuntimeType for IBufferStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IBufferStatics {
                const NAME: &'static str = "Windows.Storage.Streams.IBufferStatics";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IBufferStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                CreateCopyFromMemoryBuffer: usize,
                CreateMemoryBufferOverIBuffer: usize,
            }
            windows_core::imp::define_interface!(IContentTypeProvider, IContentTypeProvider_Vtbl, 0x97d098a5_3b99_4de9_88a5_e11d2f50c795);
            impl windows_core::RuntimeType for IContentTypeProvider {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            windows_core::imp::interface_hierarchy!(IContentTypeProvider, windows_core::IUnknown, windows_core::IInspectable);
            impl IContentTypeProvider {
            }
            impl windows_core::RuntimeName for IContentTypeProvider {
                const NAME: &'static str = "Windows.Storage.Streams.IContentTypeProvider";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IContentTypeProvider_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub ContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IDataReader, IDataReader_Vtbl, 0xe2b50029_b4c1_4314_a4b8_fb813a2f275e);
            impl windows_core::RuntimeType for IDataReader {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            windows_core::imp::interface_hierarchy!(IDataReader, windows_core::IUnknown, windows_core::IInspectable);
            impl IDataReader {
                pub fn InputStreamOptions(&self) -> windows_core::Result<InputStreamOptions> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).InputStreamOptions)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn ReadBytes(&self, value: &mut [u8]) -> windows_core::Result<()> {
                    let this = self;
                    unsafe {
                        (windows_core::Interface::vtable(this).ReadBytes)(
                            windows_core::Interface::as_raw(this),
                            value.len().try_into().unwrap(),
                            value.as_mut_ptr(),
                        )
                        .ok()
                    }
                }
            }
            impl windows_core::RuntimeName for IDataReader {
                const NAME: &'static str = "Windows.Storage.Streams.IDataReader";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IDataReader_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub UnconsumedBufferLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                UnicodeEncoding: usize,
                SetUnicodeEncoding: usize,
                ByteOrder: usize,
                SetByteOrder: usize,
                pub InputStreamOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InputStreamOptions) -> windows_core::HRESULT,
                pub SetInputStreamOptions: unsafe extern "system" fn(*mut core::ffi::c_void, InputStreamOptions) -> windows_core::HRESULT,
                pub ReadByte: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
                pub ReadBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
                pub ReadBuffer:
                    unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ReadBoolean: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub ReadGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
                pub ReadInt16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
                pub ReadInt32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                pub ReadInt64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub ReadUInt16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
                pub ReadUInt32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                pub ReadUInt64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
                pub ReadSingle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
                pub ReadDouble: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                pub ReadString:
                    unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ReadDateTime:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
                pub ReadTimeSpan:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
                LoadAsync: usize,
                pub DetachBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub DetachStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IDataReaderFactory, IDataReaderFactory_Vtbl, 0xd7527847_57da_4e15_914c_06806699a098);
            impl windows_core::RuntimeType for IDataReaderFactory {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IDataReaderFactory {
                const NAME: &'static str = "Windows.Storage.Streams.IDataReaderFactory";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IDataReaderFactory_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CreateDataReader: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IDataReaderStatics, IDataReaderStatics_Vtbl, 0x11fcbfc8_f93a_471b_b121_f379e349313c);
            impl windows_core::RuntimeType for IDataReaderStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            impl windows_core::RuntimeName for IDataReaderStatics {
                const NAME: &'static str = "Windows.Storage.Streams.IDataReaderStatics";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IDataReaderStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub FromBuffer: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IInputStream, IInputStream_Vtbl, 0x905a0fe2_bc53_11df_8c49_001e4fc686da);
            impl windows_core::RuntimeType for IInputStream {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            windows_core::imp::interface_hierarchy!(IInputStream, windows_core::IUnknown, windows_core::IInspectable);
            windows_core::imp::required_hierarchy!(IInputStream, super::super::Foundation::IClosable);
            impl IInputStream {
                pub fn ReadAsync<P0>(
                    &self,
                    buffer: P0,
                    count: u32,
                    options: InputStreamOptions,
                ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
                where
                    P0: windows_core::Param<IBuffer>,
                {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).ReadAsync)(
                            windows_core::Interface::as_raw(this),
                            buffer.param().abi(),
                            count,
                            options,
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn Close(&self) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
                    unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
                }
            }
            impl windows_core::RuntimeName for IInputStream {
                const NAME: &'static str = "Windows.Storage.Streams.IInputStream";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IInputStream_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub ReadAsync: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    u32,
                    InputStreamOptions,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IOutputStream, IOutputStream_Vtbl, 0x905a0fe6_bc53_11df_8c49_001e4fc686da);
            impl windows_core::RuntimeType for IOutputStream {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            windows_core::imp::interface_hierarchy!(IOutputStream, windows_core::IUnknown, windows_core::IInspectable);
            windows_core::imp::required_hierarchy!(IOutputStream, super::super::Foundation::IClosable);
            impl IOutputStream {
                pub fn Close(&self) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
                    unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
                }
            }
            impl windows_core::RuntimeName for IOutputStream {
                const NAME: &'static str = "Windows.Storage.Streams.IOutputStream";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IOutputStream_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub WriteAsync: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub FlushAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IRandomAccessStream, IRandomAccessStream_Vtbl, 0x905a0fe1_bc53_11df_8c49_001e4fc686da);
            impl windows_core::RuntimeType for IRandomAccessStream {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            windows_core::imp::interface_hierarchy!(IRandomAccessStream, windows_core::IUnknown, windows_core::IInspectable);
            windows_core::imp::required_hierarchy!(IRandomAccessStream, super::super::Foundation::IClosable, IInputStream, IOutputStream);
            impl IRandomAccessStream {
                pub fn Size(&self) -> windows_core::Result<u64> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                    }
                }
                pub fn SetSize(&self, value: u64) -> windows_core::Result<()> {
                    let this = self;
                    unsafe { (windows_core::Interface::vtable(this).SetSize)(windows_core::Interface::as_raw(this), value).ok() }
                }
                pub fn Position(&self) -> windows_core::Result<u64> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn Seek(&self, position: u64) -> windows_core::Result<()> {
                    let this = self;
                    unsafe { (windows_core::Interface::vtable(this).Seek)(windows_core::Interface::as_raw(this), position).ok() }
                }
                pub fn Close(&self) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
                    unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
                }
                pub fn ReadAsync<P0>(
                    &self,
                    buffer: P0,
                    count: u32,
                    options: InputStreamOptions,
                ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
                where
                    P0: windows_core::Param<IBuffer>,
                {
                    let this = &windows_core::Interface::cast::<IInputStream>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).ReadAsync)(
                            windows_core::Interface::as_raw(this),
                            buffer.param().abi(),
                            count,
                            options,
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
            }
            impl windows_core::RuntimeName for IRandomAccessStream {
                const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStream";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IRandomAccessStream_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
                pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
                pub GetInputStreamAt:
                    unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub GetOutputStreamAt:
                    unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
                pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
                pub CloneStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub CanRead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub CanWrite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IRandomAccessStreamReference,
                IRandomAccessStreamReference_Vtbl,
                0x33ee3134_1dd6_4e3a_8067_d1c162e8642b
            );
            impl windows_core::RuntimeType for IRandomAccessStreamReference {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            windows_core::imp::interface_hierarchy!(IRandomAccessStreamReference, windows_core::IUnknown, windows_core::IInspectable);
            impl IRandomAccessStreamReference {
                pub fn OpenReadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<IRandomAccessStreamWithContentType>> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).OpenReadAsync)(windows_core::Interface::as_raw(this), &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
            }
            impl windows_core::RuntimeName for IRandomAccessStreamReference {
                const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamReference";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IRandomAccessStreamReference_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub OpenReadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IRandomAccessStreamWithContentType,
                IRandomAccessStreamWithContentType_Vtbl,
                0xcc254827_4b3d_438f_9232_10c76bc7e038
            );
            impl windows_core::RuntimeType for IRandomAccessStreamWithContentType {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            windows_core::imp::interface_hierarchy!(IRandomAccessStreamWithContentType, windows_core::IUnknown, windows_core::IInspectable);
            windows_core::imp::required_hierarchy!(
                IRandomAccessStreamWithContentType,
                super::super::Foundation::IClosable,
                IContentTypeProvider,
                IInputStream,
                IOutputStream,
                IRandomAccessStream
            );
            impl IRandomAccessStreamWithContentType {
                pub fn Close(&self) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
                    unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
                }
                pub fn ReadAsync<P0>(
                    &self,
                    buffer: P0,
                    count: u32,
                    options: InputStreamOptions,
                ) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
                where
                    P0: windows_core::Param<IBuffer>,
                {
                    let this = &windows_core::Interface::cast::<IInputStream>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).ReadAsync)(
                            windows_core::Interface::as_raw(this),
                            buffer.param().abi(),
                            count,
                            options,
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn Size(&self) -> windows_core::Result<u64> {
                    let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                    }
                }
                pub fn SetSize(&self, value: u64) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
                    unsafe { (windows_core::Interface::vtable(this).SetSize)(windows_core::Interface::as_raw(this), value).ok() }
                }
                pub fn Position(&self) -> windows_core::Result<u64> {
                    let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn Seek(&self, position: u64) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
                    unsafe { (windows_core::Interface::vtable(this).Seek)(windows_core::Interface::as_raw(this), position).ok() }
                }
            }
            impl windows_core::RuntimeName for IRandomAccessStreamWithContentType {
                const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamWithContentType";
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IRandomAccessStreamWithContentType_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct InputStreamOptions(pub u32);
            impl InputStreamOptions {
                pub const None: Self = Self(0u32);
                pub const Partial: Self = Self(1u32);
                pub const ReadAhead: Self = Self(2u32);
            }
            impl windows_core::TypeKind for InputStreamOptions {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for InputStreamOptions {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.InputStreamOptions;u4)");
            }
            impl InputStreamOptions {
                pub const fn contains(&self, other: Self) -> bool {
                    self.0 & other.0 == other.0
                }
            }
            impl core::ops::BitOr for InputStreamOptions {
                type Output = Self;
                fn bitor(self, other: Self) -> Self {
                    Self(self.0 | other.0)
                }
            }
            impl core::ops::BitAnd for InputStreamOptions {
                type Output = Self;
                fn bitand(self, other: Self) -> Self {
                    Self(self.0 & other.0)
                }
            }
            impl core::ops::BitOrAssign for InputStreamOptions {
                fn bitor_assign(&mut self, other: Self) {
                    self.0.bitor_assign(other.0)
                }
            }
            impl core::ops::BitAndAssign for InputStreamOptions {
                fn bitand_assign(&mut self, other: Self) {
                    self.0.bitand_assign(other.0)
                }
            }
            impl core::ops::Not for InputStreamOptions {
                type Output = Self;
                fn not(self) -> Self {
                    Self(self.0.not())
                }
            }
        }
    }
    pub mod System {
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct VirtualKey(pub i32);
        impl VirtualKey {
            pub const None: Self = Self(0i32);
            pub const LeftButton: Self = Self(1i32);
            pub const RightButton: Self = Self(2i32);
            pub const Cancel: Self = Self(3i32);
            pub const MiddleButton: Self = Self(4i32);
            pub const XButton1: Self = Self(5i32);
            pub const XButton2: Self = Self(6i32);
            pub const Back: Self = Self(8i32);
            pub const Tab: Self = Self(9i32);
            pub const Clear: Self = Self(12i32);
            pub const Enter: Self = Self(13i32);
            pub const Shift: Self = Self(16i32);
            pub const Control: Self = Self(17i32);
            pub const Menu: Self = Self(18i32);
            pub const Pause: Self = Self(19i32);
            pub const CapitalLock: Self = Self(20i32);
            pub const Kana: Self = Self(21i32);
            pub const Hangul: Self = Self(21i32);
            pub const ImeOn: Self = Self(22i32);
            pub const Junja: Self = Self(23i32);
            pub const Final: Self = Self(24i32);
            pub const Hanja: Self = Self(25i32);
            pub const Kanji: Self = Self(25i32);
            pub const ImeOff: Self = Self(26i32);
            pub const Escape: Self = Self(27i32);
            pub const Convert: Self = Self(28i32);
            pub const NonConvert: Self = Self(29i32);
            pub const Accept: Self = Self(30i32);
            pub const ModeChange: Self = Self(31i32);
            pub const Space: Self = Self(32i32);
            pub const PageUp: Self = Self(33i32);
            pub const PageDown: Self = Self(34i32);
            pub const End: Self = Self(35i32);
            pub const Home: Self = Self(36i32);
            pub const Left: Self = Self(37i32);
            pub const Up: Self = Self(38i32);
            pub const Right: Self = Self(39i32);
            pub const Down: Self = Self(40i32);
            pub const Select: Self = Self(41i32);
            pub const Print: Self = Self(42i32);
            pub const Execute: Self = Self(43i32);
            pub const Snapshot: Self = Self(44i32);
            pub const Insert: Self = Self(45i32);
            pub const Delete: Self = Self(46i32);
            pub const Help: Self = Self(47i32);
            pub const Number0: Self = Self(48i32);
            pub const Number1: Self = Self(49i32);
            pub const Number2: Self = Self(50i32);
            pub const Number3: Self = Self(51i32);
            pub const Number4: Self = Self(52i32);
            pub const Number5: Self = Self(53i32);
            pub const Number6: Self = Self(54i32);
            pub const Number7: Self = Self(55i32);
            pub const Number8: Self = Self(56i32);
            pub const Number9: Self = Self(57i32);
            pub const A: Self = Self(65i32);
            pub const B: Self = Self(66i32);
            pub const C: Self = Self(67i32);
            pub const D: Self = Self(68i32);
            pub const E: Self = Self(69i32);
            pub const F: Self = Self(70i32);
            pub const G: Self = Self(71i32);
            pub const H: Self = Self(72i32);
            pub const I: Self = Self(73i32);
            pub const J: Self = Self(74i32);
            pub const K: Self = Self(75i32);
            pub const L: Self = Self(76i32);
            pub const M: Self = Self(77i32);
            pub const N: Self = Self(78i32);
            pub const O: Self = Self(79i32);
            pub const P: Self = Self(80i32);
            pub const Q: Self = Self(81i32);
            pub const R: Self = Self(82i32);
            pub const S: Self = Self(83i32);
            pub const T: Self = Self(84i32);
            pub const U: Self = Self(85i32);
            pub const V: Self = Self(86i32);
            pub const W: Self = Self(87i32);
            pub const X: Self = Self(88i32);
            pub const Y: Self = Self(89i32);
            pub const Z: Self = Self(90i32);
            pub const LeftWindows: Self = Self(91i32);
            pub const RightWindows: Self = Self(92i32);
            pub const Application: Self = Self(93i32);
            pub const Sleep: Self = Self(95i32);
            pub const NumberPad0: Self = Self(96i32);
            pub const NumberPad1: Self = Self(97i32);
            pub const NumberPad2: Self = Self(98i32);
            pub const NumberPad3: Self = Self(99i32);
            pub const NumberPad4: Self = Self(100i32);
            pub const NumberPad5: Self = Self(101i32);
            pub const NumberPad6: Self = Self(102i32);
            pub const NumberPad7: Self = Self(103i32);
            pub const NumberPad8: Self = Self(104i32);
            pub const NumberPad9: Self = Self(105i32);
            pub const Multiply: Self = Self(106i32);
            pub const Add: Self = Self(107i32);
            pub const Separator: Self = Self(108i32);
            pub const Subtract: Self = Self(109i32);
            pub const Decimal: Self = Self(110i32);
            pub const Divide: Self = Self(111i32);
            pub const F1: Self = Self(112i32);
            pub const F2: Self = Self(113i32);
            pub const F3: Self = Self(114i32);
            pub const F4: Self = Self(115i32);
            pub const F5: Self = Self(116i32);
            pub const F6: Self = Self(117i32);
            pub const F7: Self = Self(118i32);
            pub const F8: Self = Self(119i32);
            pub const F9: Self = Self(120i32);
            pub const F10: Self = Self(121i32);
            pub const F11: Self = Self(122i32);
            pub const F12: Self = Self(123i32);
            pub const F13: Self = Self(124i32);
            pub const F14: Self = Self(125i32);
            pub const F15: Self = Self(126i32);
            pub const F16: Self = Self(127i32);
            pub const F17: Self = Self(128i32);
            pub const F18: Self = Self(129i32);
            pub const F19: Self = Self(130i32);
            pub const F20: Self = Self(131i32);
            pub const F21: Self = Self(132i32);
            pub const F22: Self = Self(133i32);
            pub const F23: Self = Self(134i32);
            pub const F24: Self = Self(135i32);
            pub const NavigationView: Self = Self(136i32);
            pub const NavigationMenu: Self = Self(137i32);
            pub const NavigationUp: Self = Self(138i32);
            pub const NavigationDown: Self = Self(139i32);
            pub const NavigationLeft: Self = Self(140i32);
            pub const NavigationRight: Self = Self(141i32);
            pub const NavigationAccept: Self = Self(142i32);
            pub const NavigationCancel: Self = Self(143i32);
            pub const NumberKeyLock: Self = Self(144i32);
            pub const Scroll: Self = Self(145i32);
            pub const LeftShift: Self = Self(160i32);
            pub const RightShift: Self = Self(161i32);
            pub const LeftControl: Self = Self(162i32);
            pub const RightControl: Self = Self(163i32);
            pub const LeftMenu: Self = Self(164i32);
            pub const RightMenu: Self = Self(165i32);
            pub const GoBack: Self = Self(166i32);
            pub const GoForward: Self = Self(167i32);
            pub const Refresh: Self = Self(168i32);
            pub const Stop: Self = Self(169i32);
            pub const Search: Self = Self(170i32);
            pub const Favorites: Self = Self(171i32);
            pub const GoHome: Self = Self(172i32);
            pub const GamepadA: Self = Self(195i32);
            pub const GamepadB: Self = Self(196i32);
            pub const GamepadX: Self = Self(197i32);
            pub const GamepadY: Self = Self(198i32);
            pub const GamepadRightShoulder: Self = Self(199i32);
            pub const GamepadLeftShoulder: Self = Self(200i32);
            pub const GamepadLeftTrigger: Self = Self(201i32);
            pub const GamepadRightTrigger: Self = Self(202i32);
            pub const GamepadDPadUp: Self = Self(203i32);
            pub const GamepadDPadDown: Self = Self(204i32);
            pub const GamepadDPadLeft: Self = Self(205i32);
            pub const GamepadDPadRight: Self = Self(206i32);
            pub const GamepadMenu: Self = Self(207i32);
            pub const GamepadView: Self = Self(208i32);
            pub const GamepadLeftThumbstickButton: Self = Self(209i32);
            pub const GamepadRightThumbstickButton: Self = Self(210i32);
            pub const GamepadLeftThumbstickUp: Self = Self(211i32);
            pub const GamepadLeftThumbstickDown: Self = Self(212i32);
            pub const GamepadLeftThumbstickRight: Self = Self(213i32);
            pub const GamepadLeftThumbstickLeft: Self = Self(214i32);
            pub const GamepadRightThumbstickUp: Self = Self(215i32);
            pub const GamepadRightThumbstickDown: Self = Self(216i32);
            pub const GamepadRightThumbstickRight: Self = Self(217i32);
            pub const GamepadRightThumbstickLeft: Self = Self(218i32);
        }
        impl windows_core::TypeKind for VirtualKey {
            type TypeKind = windows_core::CopyType;
        }
        impl windows_core::RuntimeType for VirtualKey {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.System.VirtualKey;i4)");
        }
    }
    pub mod UI {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct Color {
            pub A: u8,
            pub R: u8,
            pub G: u8,
            pub B: u8,
        }
        impl windows_core::TypeKind for Color {
            type TypeKind = windows_core::CopyType;
        }
        impl windows_core::RuntimeType for Color {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Color;u1;u1;u1;u1)");
        }
        pub mod Core {
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct CorePhysicalKeyStatus {
                pub RepeatCount: u32,
                pub ScanCode: u32,
                pub IsExtendedKey: bool,
                pub IsMenuKeyDown: bool,
                pub WasKeyDown: bool,
                pub IsKeyReleased: bool,
            }
            impl windows_core::TypeKind for CorePhysicalKeyStatus {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for CorePhysicalKeyStatus {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Core.CorePhysicalKeyStatus;u4;u4;b1;b1;b1;b1)");
            }
        }
        pub mod Text {
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct FontStretch(pub i32);
            impl FontStretch {
                pub const Undefined: Self = Self(0i32);
                pub const UltraCondensed: Self = Self(1i32);
                pub const ExtraCondensed: Self = Self(2i32);
                pub const Condensed: Self = Self(3i32);
                pub const SemiCondensed: Self = Self(4i32);
                pub const Normal: Self = Self(5i32);
                pub const SemiExpanded: Self = Self(6i32);
                pub const Expanded: Self = Self(7i32);
                pub const ExtraExpanded: Self = Self(8i32);
                pub const UltraExpanded: Self = Self(9i32);
            }
            impl windows_core::TypeKind for FontStretch {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for FontStretch {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.FontStretch;i4)");
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct FontStyle(pub i32);
            impl FontStyle {
                pub const Normal: Self = Self(0i32);
                pub const Oblique: Self = Self(1i32);
                pub const Italic: Self = Self(2i32);
            }
            impl windows_core::TypeKind for FontStyle {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for FontStyle {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.FontStyle;i4)");
            }
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct FontWeight {
                pub Weight: u16,
            }
            impl windows_core::TypeKind for FontWeight {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for FontWeight {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Text.FontWeight;u2)");
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct TextDecorations(pub u32);
            impl TextDecorations {
                pub const None: Self = Self(0u32);
                pub const Underline: Self = Self(1u32);
                pub const Strikethrough: Self = Self(2u32);
            }
            impl windows_core::TypeKind for TextDecorations {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for TextDecorations {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TextDecorations;u4)");
            }
            impl TextDecorations {
                pub const fn contains(&self, other: Self) -> bool {
                    self.0 & other.0 == other.0
                }
            }
            impl core::ops::BitOr for TextDecorations {
                type Output = Self;
                fn bitor(self, other: Self) -> Self {
                    Self(self.0 | other.0)
                }
            }
            impl core::ops::BitAnd for TextDecorations {
                type Output = Self;
                fn bitand(self, other: Self) -> Self {
                    Self(self.0 & other.0)
                }
            }
            impl core::ops::BitOrAssign for TextDecorations {
                fn bitor_assign(&mut self, other: Self) {
                    self.0.bitor_assign(other.0)
                }
            }
            impl core::ops::BitAndAssign for TextDecorations {
                fn bitand_assign(&mut self, other: Self) {
                    self.0.bitand_assign(other.0)
                }
            }
            impl core::ops::Not for TextDecorations {
                type Output = Self;
                fn not(self) -> Self {
                    Self(self.0.not())
                }
            }
        }
    }
    pub mod Win32 {
        pub mod Devices {
            pub mod FunctionDiscovery {
                pub const PKEY_DeviceInterface_FriendlyName: super::super::Foundation::PROPERTYKEY =
                    super::super::Foundation::PROPERTYKEY {
                        fmtid: windows_core::GUID::from_u128(0x026e516e_b814_414b_83cd_856d6fef4822),
                        pid: 2,
                    };
                pub const PKEY_Device_ContainerId: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY {
                    fmtid: windows_core::GUID::from_u128(0x8c7ed206_3f8a_4827_b3ab_ae9e1faefc6c),
                    pid: 2,
                };
                pub const PKEY_Device_FriendlyName: super::super::Foundation::PROPERTYKEY = super::super::Foundation::PROPERTYKEY {
                    fmtid: windows_core::GUID::from_u128(0xa45c254e_df1c_4efd_8020_67d146a850e0),
                    pid: 14,
                };
            }
        }
        pub mod Foundation {
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct DECIMAL {
                pub wReserved: u16,
                pub Anonymous1: DECIMAL_0,
                pub Hi32: u32,
                pub Anonymous2: DECIMAL_1,
            }
            impl Default for DECIMAL {
                fn default() -> Self {
                    unsafe { core::mem::zeroed() }
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub union DECIMAL_0 {
                pub Anonymous: DECIMAL_0_0,
                pub signscale: u16,
            }
            impl Default for DECIMAL_0 {
                fn default() -> Self {
                    unsafe { core::mem::zeroed() }
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct DECIMAL_0_0 {
                pub scale: u8,
                pub sign: u8,
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub union DECIMAL_1 {
                pub Anonymous: DECIMAL_1_0,
                pub Lo64: u64,
            }
            impl Default for DECIMAL_1 {
                fn default() -> Self {
                    unsafe { core::mem::zeroed() }
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct DECIMAL_1_0 {
                pub Lo32: u32,
                pub Mid32: u32,
            }
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct FILETIME {
                pub dwLowDateTime: u32,
                pub dwHighDateTime: u32,
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub struct HANDLE(pub *mut core::ffi::c_void);
            impl HANDLE {
                pub fn is_invalid(&self) -> bool {
                    self.0 == -1 as _ || self.0 == 0 as _
                }
            }
            impl windows_core::Free for HANDLE {
                #[inline]
                unsafe fn free(&mut self) {
                    if !self.is_invalid() {
                        windows_core::link!("kernel32.dll" "system" fn CloseHandle(hobject : *mut core::ffi::c_void) -> i32);
                        unsafe {
                            CloseHandle(self.0);
                        }
                    }
                }
            }
            impl Default for HANDLE {
                fn default() -> Self {
                    unsafe { core::mem::zeroed() }
                }
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub struct HINSTANCE(pub *mut core::ffi::c_void);
            impl HINSTANCE {
                pub fn is_invalid(&self) -> bool {
                    self.0.is_null()
                }
            }
            impl windows_core::Free for HINSTANCE {
                #[inline]
                unsafe fn free(&mut self) {
                    if !self.is_invalid() {
                        windows_core::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : *mut core::ffi::c_void) -> i32);
                        unsafe {
                            FreeLibrary(self.0);
                        }
                    }
                }
            }
            impl Default for HINSTANCE {
                fn default() -> Self {
                    unsafe { core::mem::zeroed() }
                }
            }
            impl windows_core::imp::CanInto<HMODULE> for HINSTANCE {}
            impl From<HINSTANCE> for HMODULE {
                fn from(value: HINSTANCE) -> Self {
                    Self(value.0)
                }
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub struct HMODULE(pub *mut core::ffi::c_void);
            impl HMODULE {
                pub fn is_invalid(&self) -> bool {
                    self.0.is_null()
                }
            }
            impl windows_core::Free for HMODULE {
                #[inline]
                unsafe fn free(&mut self) {
                    if !self.is_invalid() {
                        windows_core::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : *mut core::ffi::c_void) -> i32);
                        unsafe {
                            FreeLibrary(self.0);
                        }
                    }
                }
            }
            impl Default for HMODULE {
                fn default() -> Self {
                    unsafe { core::mem::zeroed() }
                }
            }
            impl windows_core::imp::CanInto<HINSTANCE> for HMODULE {}
            impl From<HMODULE> for HINSTANCE {
                fn from(value: HMODULE) -> Self {
                    Self(value.0)
                }
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub struct HWND(pub *mut core::ffi::c_void);
            impl HWND {
                pub fn is_invalid(&self) -> bool {
                    self.0.is_null()
                }
            }
            impl Default for HWND {
                fn default() -> Self {
                    unsafe { core::mem::zeroed() }
                }
            }
            impl windows_core::imp::CanInto<HANDLE> for HWND {}
            impl From<HWND> for HANDLE {
                fn from(value: HWND) -> Self {
                    Self(value.0)
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct PROPERTYKEY {
                pub fmtid: windows_core::GUID,
                pub pid: u32,
            }
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct RECT {
                pub left: i32,
                pub top: i32,
                pub right: i32,
                pub bottom: i32,
            }
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct SIZE {
                pub cx: i32,
                pub cy: i32,
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
            pub struct VARIANT_BOOL(pub i16);
        }
        pub mod Graphics {
            pub mod Direct2D {
                #[inline]
                pub unsafe fn D2D1CreateFactory<T>(
                    factorytype: D2D1_FACTORY_TYPE,
                    pfactoryoptions: Option<*const D2D1_FACTORY_OPTIONS>,
                ) -> windows_core::Result<T>
                where
                    T: windows_core::Interface,
                {
                    windows_core::link!("d2d1.dll" "system" fn D2D1CreateFactory(factorytype : D2D1_FACTORY_TYPE, riid : *const windows_core::GUID, pfactoryoptions : *const D2D1_FACTORY_OPTIONS, ppifactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
                    let mut result__ = core::ptr::null_mut();
                    unsafe {
                        D2D1CreateFactory(factorytype, &T::IID, pfactoryoptions.unwrap_or(core::mem::zeroed()) as _, &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D2D1_ANTIALIAS_MODE(pub i32);
                pub const D2D1_ANTIALIAS_MODE_PER_PRIMITIVE: D2D1_ANTIALIAS_MODE = D2D1_ANTIALIAS_MODE(0i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D2D1_BITMAP_INTERPOLATION_MODE(pub i32);
                pub const D2D1_BITMAP_INTERPOLATION_MODE_LINEAR: D2D1_BITMAP_INTERPOLATION_MODE = D2D1_BITMAP_INTERPOLATION_MODE(1i32);
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct D2D1_BITMAP_PROPERTIES {
                    pub pixelFormat: Common::D2D1_PIXEL_FORMAT,
                    pub dpiX: f32,
                    pub dpiY: f32,
                }
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct D2D1_BRUSH_PROPERTIES {
                    pub opacity: f32,
                    pub transform: windows_numerics::Matrix3x2,
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D2D1_DEBUG_LEVEL(pub i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D2D1_DRAW_TEXT_OPTIONS(pub i32);
                impl D2D1_DRAW_TEXT_OPTIONS {
                    pub const fn contains(&self, other: Self) -> bool {
                        self.0 & other.0 == other.0
                    }
                }
                impl core::ops::BitOr for D2D1_DRAW_TEXT_OPTIONS {
                    type Output = Self;
                    fn bitor(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }
                impl core::ops::BitAnd for D2D1_DRAW_TEXT_OPTIONS {
                    type Output = Self;
                    fn bitand(self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
                impl core::ops::BitOrAssign for D2D1_DRAW_TEXT_OPTIONS {
                    fn bitor_assign(&mut self, other: Self) {
                        self.0.bitor_assign(other.0)
                    }
                }
                impl core::ops::BitAndAssign for D2D1_DRAW_TEXT_OPTIONS {
                    fn bitand_assign(&mut self, other: Self) {
                        self.0.bitand_assign(other.0)
                    }
                }
                impl core::ops::Not for D2D1_DRAW_TEXT_OPTIONS {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                pub const D2D1_DRAW_TEXT_OPTIONS_CLIP: D2D1_DRAW_TEXT_OPTIONS = D2D1_DRAW_TEXT_OPTIONS(2i32);
                pub const D2D1_DRAW_TEXT_OPTIONS_NONE: D2D1_DRAW_TEXT_OPTIONS = D2D1_DRAW_TEXT_OPTIONS(0i32);
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct D2D1_ELLIPSE {
                    pub point: windows_numerics::Vector2,
                    pub radiusX: f32,
                    pub radiusY: f32,
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D2D1_EXTEND_MODE(pub i32);
                pub const D2D1_EXTEND_MODE_CLAMP: D2D1_EXTEND_MODE = D2D1_EXTEND_MODE(0i32);
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct D2D1_FACTORY_OPTIONS {
                    pub debugLevel: D2D1_DEBUG_LEVEL,
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D2D1_FACTORY_TYPE(pub i32);
                pub const D2D1_FACTORY_TYPE_SINGLE_THREADED: D2D1_FACTORY_TYPE = D2D1_FACTORY_TYPE(0i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D2D1_FEATURE_LEVEL(pub i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D2D1_GAMMA(pub i32);
                pub const D2D1_GAMMA_2_2: D2D1_GAMMA = D2D1_GAMMA(0i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D2D1_LAYER_OPTIONS(pub i32);
                impl D2D1_LAYER_OPTIONS {
                    pub const fn contains(&self, other: Self) -> bool {
                        self.0 & other.0 == other.0
                    }
                }
                impl core::ops::BitOr for D2D1_LAYER_OPTIONS {
                    type Output = Self;
                    fn bitor(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }
                impl core::ops::BitAnd for D2D1_LAYER_OPTIONS {
                    type Output = Self;
                    fn bitand(self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
                impl core::ops::BitOrAssign for D2D1_LAYER_OPTIONS {
                    fn bitor_assign(&mut self, other: Self) {
                        self.0.bitor_assign(other.0)
                    }
                }
                impl core::ops::BitAndAssign for D2D1_LAYER_OPTIONS {
                    fn bitand_assign(&mut self, other: Self) {
                        self.0.bitand_assign(other.0)
                    }
                }
                impl core::ops::Not for D2D1_LAYER_OPTIONS {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                pub const D2D1_LAYER_OPTIONS_NONE: D2D1_LAYER_OPTIONS = D2D1_LAYER_OPTIONS(0i32);
                #[repr(C)]
                #[derive(Clone, Debug, Default, PartialEq)]
                pub struct D2D1_LAYER_PARAMETERS {
                    pub contentBounds: Common::D2D_RECT_F,
                    pub geometricMask: core::mem::ManuallyDrop<Option<ID2D1Geometry>>,
                    pub maskAntialiasMode: D2D1_ANTIALIAS_MODE,
                    pub maskTransform: windows_numerics::Matrix3x2,
                    pub opacity: f32,
                    pub opacityBrush: core::mem::ManuallyDrop<Option<ID2D1Brush>>,
                    pub layerOptions: D2D1_LAYER_OPTIONS,
                }
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                    pub startPoint: windows_numerics::Vector2,
                    pub endPoint: windows_numerics::Vector2,
                }
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct D2D1_RENDER_TARGET_PROPERTIES {
                    pub r#type: D2D1_RENDER_TARGET_TYPE,
                    pub pixelFormat: Common::D2D1_PIXEL_FORMAT,
                    pub dpiX: f32,
                    pub dpiY: f32,
                    pub usage: D2D1_RENDER_TARGET_USAGE,
                    pub minLevel: D2D1_FEATURE_LEVEL,
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D2D1_RENDER_TARGET_TYPE(pub i32);
                pub const D2D1_RENDER_TARGET_TYPE_DEFAULT: D2D1_RENDER_TARGET_TYPE = D2D1_RENDER_TARGET_TYPE(0i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D2D1_RENDER_TARGET_USAGE(pub i32);
                impl D2D1_RENDER_TARGET_USAGE {
                    pub const fn contains(&self, other: Self) -> bool {
                        self.0 & other.0 == other.0
                    }
                }
                impl core::ops::BitOr for D2D1_RENDER_TARGET_USAGE {
                    type Output = Self;
                    fn bitor(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }
                impl core::ops::BitAnd for D2D1_RENDER_TARGET_USAGE {
                    type Output = Self;
                    fn bitand(self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
                impl core::ops::BitOrAssign for D2D1_RENDER_TARGET_USAGE {
                    fn bitor_assign(&mut self, other: Self) {
                        self.0.bitor_assign(other.0)
                    }
                }
                impl core::ops::BitAndAssign for D2D1_RENDER_TARGET_USAGE {
                    fn bitand_assign(&mut self, other: Self) {
                        self.0.bitand_assign(other.0)
                    }
                }
                impl core::ops::Not for D2D1_RENDER_TARGET_USAGE {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct D2D1_ROUNDED_RECT {
                    pub rect: Common::D2D_RECT_F,
                    pub radiusX: f32,
                    pub radiusY: f32,
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D2D1_TEXT_ANTIALIAS_MODE(pub i32);
                pub const D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE: D2D1_TEXT_ANTIALIAS_MODE = D2D1_TEXT_ANTIALIAS_MODE(2i32);
                windows_core::imp::define_interface!(ID2D1Bitmap, ID2D1Bitmap_Vtbl, 0xa2296057_ea42_4099_983b_539fb6505426);
                impl core::ops::Deref for ID2D1Bitmap {
                    type Target = ID2D1Image;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID2D1Bitmap, windows_core::IUnknown, ID2D1Resource, ID2D1Image);
                impl ID2D1Bitmap {
                    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__);
                            result__
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1Bitmap_Vtbl {
                    pub base__: ID2D1Image_Vtbl,
                    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_SIZE_F),
                    pub GetPixelSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_SIZE_U),
                    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D1_PIXEL_FORMAT),
                    pub GetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
                    CopyFromBitmap: usize,
                    CopyFromRenderTarget: usize,
                    CopyFromMemory: usize,
                }
                unsafe impl Send for ID2D1Bitmap {}
                unsafe impl Sync for ID2D1Bitmap {}
                impl windows_core::RuntimeName for ID2D1Bitmap {}
                windows_core::imp::define_interface!(ID2D1Brush, ID2D1Brush_Vtbl, 0x2cd906a8_12e2_11dc_9fed_001143a055f9);
                impl core::ops::Deref for ID2D1Brush {
                    type Target = ID2D1Resource;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID2D1Brush, windows_core::IUnknown, ID2D1Resource);
                impl ID2D1Brush {
                    pub unsafe fn SetOpacity(&self, opacity: f32) {
                        unsafe { (windows_core::Interface::vtable(self).SetOpacity)(windows_core::Interface::as_raw(self), opacity) }
                    }
                    pub unsafe fn SetTransform(&self, transform: *const windows_numerics::Matrix3x2) {
                        unsafe { (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), transform) }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1Brush_Vtbl {
                    pub base__: ID2D1Resource_Vtbl,
                    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f32),
                    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Matrix3x2),
                    pub GetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
                    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Matrix3x2),
                }
                unsafe impl Send for ID2D1Brush {}
                unsafe impl Sync for ID2D1Brush {}
                impl windows_core::RuntimeName for ID2D1Brush {}
                windows_core::imp::define_interface!(ID2D1DCRenderTarget, ID2D1DCRenderTarget_Vtbl, 0x1c51bc64_de61_46fd_9899_63a5d8f03950);
                impl core::ops::Deref for ID2D1DCRenderTarget {
                    type Target = ID2D1RenderTarget;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID2D1DCRenderTarget, windows_core::IUnknown, ID2D1Resource, ID2D1RenderTarget);
                impl ID2D1DCRenderTarget {
                    pub unsafe fn BindDC(
                        &self,
                        hdc: super::Gdi::HDC,
                        psubrect: *const super::super::Foundation::RECT,
                    ) -> windows_core::Result<()> {
                        unsafe { (windows_core::Interface::vtable(self).BindDC)(windows_core::Interface::as_raw(self), hdc, psubrect).ok() }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1DCRenderTarget_Vtbl {
                    pub base__: ID2D1RenderTarget_Vtbl,
                    pub BindDC: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        super::Gdi::HDC,
                        *const super::super::Foundation::RECT,
                    ) -> windows_core::HRESULT,
                }
                unsafe impl Send for ID2D1DCRenderTarget {}
                unsafe impl Sync for ID2D1DCRenderTarget {}
                impl windows_core::RuntimeName for ID2D1DCRenderTarget {}
                windows_core::imp::define_interface!(ID2D1Factory, ID2D1Factory_Vtbl, 0x06152247_6f50_465a_9245_118bfd3b6007);
                windows_core::imp::interface_hierarchy!(ID2D1Factory, windows_core::IUnknown);
                impl ID2D1Factory {
                    pub unsafe fn CreateRoundedRectangleGeometry(
                        &self,
                        roundedrectangle: *const D2D1_ROUNDED_RECT,
                    ) -> windows_core::Result<ID2D1RoundedRectangleGeometry> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateRoundedRectangleGeometry)(
                                windows_core::Interface::as_raw(self),
                                roundedrectangle,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn CreatePathGeometry(&self) -> windows_core::Result<ID2D1PathGeometry> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreatePathGeometry)(windows_core::Interface::as_raw(self), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn CreateDCRenderTarget(
                        &self,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                    ) -> windows_core::Result<ID2D1DCRenderTarget> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateDCRenderTarget)(
                                windows_core::Interface::as_raw(self),
                                rendertargetproperties,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1Factory_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub ReloadSystemMetrics: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetDesktopDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
                    CreateRectangleGeometry: usize,
                    pub CreateRoundedRectangleGeometry: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const D2D1_ROUNDED_RECT,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    CreateEllipseGeometry: usize,
                    CreateGeometryGroup: usize,
                    CreateTransformedGeometry: usize,
                    pub CreatePathGeometry:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    CreateStrokeStyle: usize,
                    CreateDrawingStateBlock: usize,
                    pub CreateWicBitmapRenderTarget: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *const D2D1_RENDER_TARGET_PROPERTIES,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    CreateHwndRenderTarget: usize,
                    CreateDxgiSurfaceRenderTarget: usize,
                    pub CreateDCRenderTarget: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const D2D1_RENDER_TARGET_PROPERTIES,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                unsafe impl Send for ID2D1Factory {}
                unsafe impl Sync for ID2D1Factory {}
                impl windows_core::RuntimeName for ID2D1Factory {}
                windows_core::imp::define_interface!(ID2D1Geometry, ID2D1Geometry_Vtbl, 0x2cd906a1_12e2_11dc_9fed_001143a055f9);
                impl core::ops::Deref for ID2D1Geometry {
                    type Target = ID2D1Resource;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID2D1Geometry, windows_core::IUnknown, ID2D1Resource);
                impl ID2D1Geometry {
                    pub unsafe fn Outline<P2>(
                        &self,
                        worldtransform: Option<*const windows_numerics::Matrix3x2>,
                        flatteningtolerance: f32,
                        geometrysink: P2,
                    ) -> windows_core::Result<()>
                    where
                        P2: windows_core::Param<Common::ID2D1SimplifiedGeometrySink>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).Outline)(
                                windows_core::Interface::as_raw(self),
                                worldtransform.unwrap_or(core::mem::zeroed()) as _,
                                flatteningtolerance,
                                geometrysink.param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1Geometry_Vtbl {
                    pub base__: ID2D1Resource_Vtbl,
                    pub GetBounds: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_numerics::Matrix3x2,
                        *mut Common::D2D_RECT_F,
                    ) -> windows_core::HRESULT,
                    pub GetWidenedBounds: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        f32,
                        *mut core::ffi::c_void,
                        *const windows_numerics::Matrix3x2,
                        f32,
                        *mut Common::D2D_RECT_F,
                    ) -> windows_core::HRESULT,
                    pub StrokeContainsPoint: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        windows_numerics::Vector2,
                        f32,
                        *mut core::ffi::c_void,
                        *const windows_numerics::Matrix3x2,
                        f32,
                        *mut windows_core::BOOL,
                    ) -> windows_core::HRESULT,
                    pub FillContainsPoint: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        windows_numerics::Vector2,
                        *const windows_numerics::Matrix3x2,
                        f32,
                        *mut windows_core::BOOL,
                    ) -> windows_core::HRESULT,
                    CompareWithGeometry: usize,
                    Simplify: usize,
                    Tessellate: usize,
                    CombineWithGeometry: usize,
                    pub Outline: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_numerics::Matrix3x2,
                        f32,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub ComputeArea: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_numerics::Matrix3x2,
                        f32,
                        *mut f32,
                    ) -> windows_core::HRESULT,
                    pub ComputeLength: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_numerics::Matrix3x2,
                        f32,
                        *mut f32,
                    ) -> windows_core::HRESULT,
                    pub ComputePointAtLength: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        f32,
                        *const windows_numerics::Matrix3x2,
                        f32,
                        *mut windows_numerics::Vector2,
                        *mut windows_numerics::Vector2,
                    ) -> windows_core::HRESULT,
                    pub Widen: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        f32,
                        *mut core::ffi::c_void,
                        *const windows_numerics::Matrix3x2,
                        f32,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                unsafe impl Send for ID2D1Geometry {}
                unsafe impl Sync for ID2D1Geometry {}
                impl windows_core::RuntimeName for ID2D1Geometry {}
                windows_core::imp::define_interface!(ID2D1GeometrySink, ID2D1GeometrySink_Vtbl, 0x2cd9069f_12e2_11dc_9fed_001143a055f9);
                impl core::ops::Deref for ID2D1GeometrySink {
                    type Target = Common::ID2D1SimplifiedGeometrySink;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID2D1GeometrySink, windows_core::IUnknown, Common::ID2D1SimplifiedGeometrySink);
                impl ID2D1GeometrySink {
                    pub unsafe fn AddLine(&self, point: windows_numerics::Vector2) {
                        unsafe {
                            (windows_core::Interface::vtable(self).AddLine)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute(point),
                            )
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1GeometrySink_Vtbl {
                    pub base__: Common::ID2D1SimplifiedGeometrySink_Vtbl,
                    pub AddLine: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2),
                    AddBezier: usize,
                    AddQuadraticBezier: usize,
                    AddQuadraticBeziers: usize,
                    AddArc: usize,
                }
                unsafe impl Send for ID2D1GeometrySink {}
                unsafe impl Sync for ID2D1GeometrySink {}
                impl windows_core::RuntimeName for ID2D1GeometrySink {}
                windows_core::imp::define_interface!(
                    ID2D1GradientStopCollection,
                    ID2D1GradientStopCollection_Vtbl,
                    0x2cd906a7_12e2_11dc_9fed_001143a055f9
                );
                impl core::ops::Deref for ID2D1GradientStopCollection {
                    type Target = ID2D1Resource;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID2D1GradientStopCollection, windows_core::IUnknown, ID2D1Resource);
                impl ID2D1GradientStopCollection {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1GradientStopCollection_Vtbl {
                    pub base__: ID2D1Resource_Vtbl,
                    pub GetGradientStopCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
                    pub GetGradientStops: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D1_GRADIENT_STOP, u32),
                    pub GetColorInterpolationGamma: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_GAMMA,
                    pub GetExtendMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_EXTEND_MODE,
                }
                unsafe impl Send for ID2D1GradientStopCollection {}
                unsafe impl Sync for ID2D1GradientStopCollection {}
                impl windows_core::RuntimeName for ID2D1GradientStopCollection {}
                windows_core::imp::define_interface!(ID2D1Image, ID2D1Image_Vtbl, 0x65019f75_8da2_497c_b32c_dfa34e48ede6);
                impl core::ops::Deref for ID2D1Image {
                    type Target = ID2D1Resource;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID2D1Image, windows_core::IUnknown, ID2D1Resource);
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1Image_Vtbl {
                    pub base__: ID2D1Resource_Vtbl,
                }
                unsafe impl Send for ID2D1Image {}
                unsafe impl Sync for ID2D1Image {}
                impl windows_core::RuntimeName for ID2D1Image {}
                windows_core::imp::define_interface!(ID2D1Layer, ID2D1Layer_Vtbl, 0x2cd9069b_12e2_11dc_9fed_001143a055f9);
                impl core::ops::Deref for ID2D1Layer {
                    type Target = ID2D1Resource;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID2D1Layer, windows_core::IUnknown, ID2D1Resource);
                impl ID2D1Layer {
                    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__);
                            result__
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1Layer_Vtbl {
                    pub base__: ID2D1Resource_Vtbl,
                    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_SIZE_F),
                }
                unsafe impl Send for ID2D1Layer {}
                unsafe impl Sync for ID2D1Layer {}
                impl windows_core::RuntimeName for ID2D1Layer {}
                windows_core::imp::define_interface!(
                    ID2D1LinearGradientBrush,
                    ID2D1LinearGradientBrush_Vtbl,
                    0x2cd906ab_12e2_11dc_9fed_001143a055f9
                );
                impl core::ops::Deref for ID2D1LinearGradientBrush {
                    type Target = ID2D1Brush;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID2D1LinearGradientBrush, windows_core::IUnknown, ID2D1Resource, ID2D1Brush);
                impl ID2D1LinearGradientBrush {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1LinearGradientBrush_Vtbl {
                    pub base__: ID2D1Brush_Vtbl,
                    pub SetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2),
                    pub SetEndPoint: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2),
                    pub GetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_numerics::Vector2,
                    pub GetEndPoint: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_numerics::Vector2,
                    pub GetGradientStopCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
                }
                unsafe impl Send for ID2D1LinearGradientBrush {}
                unsafe impl Sync for ID2D1LinearGradientBrush {}
                impl windows_core::RuntimeName for ID2D1LinearGradientBrush {}
                windows_core::imp::define_interface!(ID2D1PathGeometry, ID2D1PathGeometry_Vtbl, 0x2cd906a5_12e2_11dc_9fed_001143a055f9);
                impl core::ops::Deref for ID2D1PathGeometry {
                    type Target = ID2D1Geometry;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID2D1PathGeometry, windows_core::IUnknown, ID2D1Resource, ID2D1Geometry);
                impl ID2D1PathGeometry {
                    pub unsafe fn Open(&self) -> windows_core::Result<ID2D1GeometrySink> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1PathGeometry_Vtbl {
                    pub base__: ID2D1Geometry_Vtbl,
                    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub Stream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetSegmentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub GetFigureCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                }
                unsafe impl Send for ID2D1PathGeometry {}
                unsafe impl Sync for ID2D1PathGeometry {}
                impl windows_core::RuntimeName for ID2D1PathGeometry {}
                windows_core::imp::define_interface!(ID2D1RenderTarget, ID2D1RenderTarget_Vtbl, 0x2cd90694_12e2_11dc_9fed_001143a055f9);
                impl core::ops::Deref for ID2D1RenderTarget {
                    type Target = ID2D1Resource;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID2D1RenderTarget, windows_core::IUnknown, ID2D1Resource);
                impl ID2D1RenderTarget {
                    pub unsafe fn CreateBitmap(
                        &self,
                        size: Common::D2D_SIZE_U,
                        srcdata: Option<*const core::ffi::c_void>,
                        pitch: u32,
                        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
                    ) -> windows_core::Result<ID2D1Bitmap> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateBitmap)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute(size),
                                srcdata.unwrap_or(core::mem::zeroed()) as _,
                                pitch,
                                bitmapproperties,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn CreateSolidColorBrush(
                        &self,
                        color: *const Common::D2D1_COLOR_F,
                        brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>,
                    ) -> windows_core::Result<ID2D1SolidColorBrush> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateSolidColorBrush)(
                                windows_core::Interface::as_raw(self),
                                color,
                                brushproperties.unwrap_or(core::mem::zeroed()) as _,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn CreateGradientStopCollection(
                        &self,
                        gradientstops: &[Common::D2D1_GRADIENT_STOP],
                        colorinterpolationgamma: D2D1_GAMMA,
                        extendmode: D2D1_EXTEND_MODE,
                    ) -> windows_core::Result<ID2D1GradientStopCollection> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateGradientStopCollection)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute(gradientstops.as_ptr()),
                                gradientstops.len().try_into().unwrap(),
                                colorinterpolationgamma,
                                extendmode,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn CreateLinearGradientBrush<P2>(
                        &self,
                        lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
                        brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>,
                        gradientstopcollection: P2,
                    ) -> windows_core::Result<ID2D1LinearGradientBrush>
                    where
                        P2: windows_core::Param<ID2D1GradientStopCollection>,
                    {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateLinearGradientBrush)(
                                windows_core::Interface::as_raw(self),
                                lineargradientbrushproperties,
                                brushproperties.unwrap_or(core::mem::zeroed()) as _,
                                gradientstopcollection.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn CreateLayer(&self, size: Option<*const Common::D2D_SIZE_F>) -> windows_core::Result<ID2D1Layer> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateLayer)(
                                windows_core::Interface::as_raw(self),
                                size.unwrap_or(core::mem::zeroed()) as _,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn DrawLine<P2, P4>(
                        &self,
                        point0: windows_numerics::Vector2,
                        point1: windows_numerics::Vector2,
                        brush: P2,
                        strokewidth: f32,
                        strokestyle: P4,
                    ) where
                        P2: windows_core::Param<ID2D1Brush>,
                        P4: windows_core::Param<ID2D1StrokeStyle>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).DrawLine)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute(point0),
                                core::mem::transmute(point1),
                                brush.param().abi(),
                                strokewidth,
                                strokestyle.param().abi(),
                            )
                        }
                    }
                    pub unsafe fn DrawRectangle<P1, P3>(
                        &self,
                        rect: *const Common::D2D_RECT_F,
                        brush: P1,
                        strokewidth: f32,
                        strokestyle: P3,
                    ) where
                        P1: windows_core::Param<ID2D1Brush>,
                        P3: windows_core::Param<ID2D1StrokeStyle>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).DrawRectangle)(
                                windows_core::Interface::as_raw(self),
                                rect,
                                brush.param().abi(),
                                strokewidth,
                                strokestyle.param().abi(),
                            )
                        }
                    }
                    pub unsafe fn FillRectangle<P1>(&self, rect: *const Common::D2D_RECT_F, brush: P1)
                    where
                        P1: windows_core::Param<ID2D1Brush>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).FillRectangle)(
                                windows_core::Interface::as_raw(self),
                                rect,
                                brush.param().abi(),
                            )
                        }
                    }
                    pub unsafe fn DrawRoundedRectangle<P1, P3>(
                        &self,
                        roundedrect: *const D2D1_ROUNDED_RECT,
                        brush: P1,
                        strokewidth: f32,
                        strokestyle: P3,
                    ) where
                        P1: windows_core::Param<ID2D1Brush>,
                        P3: windows_core::Param<ID2D1StrokeStyle>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).DrawRoundedRectangle)(
                                windows_core::Interface::as_raw(self),
                                roundedrect,
                                brush.param().abi(),
                                strokewidth,
                                strokestyle.param().abi(),
                            )
                        }
                    }
                    pub unsafe fn FillRoundedRectangle<P1>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P1)
                    where
                        P1: windows_core::Param<ID2D1Brush>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).FillRoundedRectangle)(
                                windows_core::Interface::as_raw(self),
                                roundedrect,
                                brush.param().abi(),
                            )
                        }
                    }
                    pub unsafe fn DrawEllipse<P1, P3>(&self, ellipse: *const D2D1_ELLIPSE, brush: P1, strokewidth: f32, strokestyle: P3)
                    where
                        P1: windows_core::Param<ID2D1Brush>,
                        P3: windows_core::Param<ID2D1StrokeStyle>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).DrawEllipse)(
                                windows_core::Interface::as_raw(self),
                                ellipse,
                                brush.param().abi(),
                                strokewidth,
                                strokestyle.param().abi(),
                            )
                        }
                    }
                    pub unsafe fn FillEllipse<P1>(&self, ellipse: *const D2D1_ELLIPSE, brush: P1)
                    where
                        P1: windows_core::Param<ID2D1Brush>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).FillEllipse)(
                                windows_core::Interface::as_raw(self),
                                ellipse,
                                brush.param().abi(),
                            )
                        }
                    }
                    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2)
                    where
                        P0: windows_core::Param<ID2D1Geometry>,
                        P1: windows_core::Param<ID2D1Brush>,
                        P2: windows_core::Param<ID2D1Brush>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).FillGeometry)(
                                windows_core::Interface::as_raw(self),
                                geometry.param().abi(),
                                brush.param().abi(),
                                opacitybrush.param().abi(),
                            )
                        }
                    }
                    pub unsafe fn DrawBitmap<P0>(
                        &self,
                        bitmap: P0,
                        destinationrectangle: Option<*const Common::D2D_RECT_F>,
                        opacity: f32,
                        interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE,
                        sourcerectangle: Option<*const Common::D2D_RECT_F>,
                    ) where
                        P0: windows_core::Param<ID2D1Bitmap>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).DrawBitmap)(
                                windows_core::Interface::as_raw(self),
                                bitmap.param().abi(),
                                destinationrectangle.unwrap_or(core::mem::zeroed()) as _,
                                opacity,
                                interpolationmode,
                                sourcerectangle.unwrap_or(core::mem::zeroed()) as _,
                            )
                        }
                    }
                    pub unsafe fn DrawTextLayout<P1, P2>(
                        &self,
                        origin: windows_numerics::Vector2,
                        textlayout: P1,
                        defaultfillbrush: P2,
                        options: D2D1_DRAW_TEXT_OPTIONS,
                    ) where
                        P1: windows_core::Param<super::DirectWrite::IDWriteTextLayout>,
                        P2: windows_core::Param<ID2D1Brush>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).DrawTextLayout)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute(origin),
                                textlayout.param().abi(),
                                defaultfillbrush.param().abi(),
                                options,
                            )
                        }
                    }
                    pub unsafe fn SetTransform(&self, transform: *const windows_numerics::Matrix3x2) {
                        unsafe { (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), transform) }
                    }
                    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetTextAntialiasMode)(
                                windows_core::Interface::as_raw(self),
                                textantialiasmode,
                            )
                        }
                    }
                    pub unsafe fn PushLayer<P1>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: P1)
                    where
                        P1: windows_core::Param<ID2D1Layer>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).PushLayer)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute(layerparameters),
                                layer.param().abi(),
                            )
                        }
                    }
                    pub unsafe fn PopLayer(&self) {
                        unsafe { (windows_core::Interface::vtable(self).PopLayer)(windows_core::Interface::as_raw(self)) }
                    }
                    pub unsafe fn Clear(&self, clearcolor: Option<*const Common::D2D1_COLOR_F>) {
                        unsafe {
                            (windows_core::Interface::vtable(self).Clear)(
                                windows_core::Interface::as_raw(self),
                                clearcolor.unwrap_or(core::mem::zeroed()) as _,
                            )
                        }
                    }
                    pub unsafe fn BeginDraw(&self) {
                        unsafe { (windows_core::Interface::vtable(self).BeginDraw)(windows_core::Interface::as_raw(self)) }
                    }
                    pub unsafe fn EndDraw(&self, tag1: Option<*mut u64>, tag2: Option<*mut u64>) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).EndDraw)(
                                windows_core::Interface::as_raw(self),
                                tag1.unwrap_or(core::mem::zeroed()) as _,
                                tag2.unwrap_or(core::mem::zeroed()) as _,
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
                        unsafe { (windows_core::Interface::vtable(self).SetDpi)(windows_core::Interface::as_raw(self), dpix, dpiy) }
                    }
                    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__);
                            result__
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1RenderTarget_Vtbl {
                    pub base__: ID2D1Resource_Vtbl,
                    pub CreateBitmap: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        Common::D2D_SIZE_U,
                        *const core::ffi::c_void,
                        u32,
                        *const D2D1_BITMAP_PROPERTIES,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub CreateBitmapFromWicBitmap: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *const D2D1_BITMAP_PROPERTIES,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub CreateSharedBitmap: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *mut core::ffi::c_void,
                        *const D2D1_BITMAP_PROPERTIES,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    CreateBitmapBrush: usize,
                    pub CreateSolidColorBrush: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const Common::D2D1_COLOR_F,
                        *const D2D1_BRUSH_PROPERTIES,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub CreateGradientStopCollection: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const Common::D2D1_GRADIENT_STOP,
                        u32,
                        D2D1_GAMMA,
                        D2D1_EXTEND_MODE,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub CreateLinearGradientBrush: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
                        *const D2D1_BRUSH_PROPERTIES,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    CreateRadialGradientBrush: usize,
                    CreateCompatibleRenderTarget: usize,
                    pub CreateLayer: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const Common::D2D_SIZE_F,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    CreateMesh: usize,
                    pub DrawLine: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        windows_numerics::Vector2,
                        windows_numerics::Vector2,
                        *mut core::ffi::c_void,
                        f32,
                        *mut core::ffi::c_void,
                    ),
                    pub DrawRectangle: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const Common::D2D_RECT_F,
                        *mut core::ffi::c_void,
                        f32,
                        *mut core::ffi::c_void,
                    ),
                    pub FillRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_RECT_F, *mut core::ffi::c_void),
                    pub DrawRoundedRectangle: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const D2D1_ROUNDED_RECT,
                        *mut core::ffi::c_void,
                        f32,
                        *mut core::ffi::c_void,
                    ),
                    pub FillRoundedRectangle:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ROUNDED_RECT, *mut core::ffi::c_void),
                    pub DrawEllipse: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const D2D1_ELLIPSE,
                        *mut core::ffi::c_void,
                        f32,
                        *mut core::ffi::c_void,
                    ),
                    pub FillEllipse: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ELLIPSE, *mut core::ffi::c_void),
                    pub DrawGeometry: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        f32,
                        *mut core::ffi::c_void,
                    ),
                    pub FillGeometry: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ),
                    FillMesh: usize,
                    FillOpacityMask: usize,
                    pub DrawBitmap: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *const Common::D2D_RECT_F,
                        f32,
                        D2D1_BITMAP_INTERPOLATION_MODE,
                        *const Common::D2D_RECT_F,
                    ),
                    DrawText: usize,
                    pub DrawTextLayout: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        windows_numerics::Vector2,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        D2D1_DRAW_TEXT_OPTIONS,
                    ),
                    DrawGlyphRun: usize,
                    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Matrix3x2),
                    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Matrix3x2),
                    pub SetAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_ANTIALIAS_MODE),
                    pub GetAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_ANTIALIAS_MODE,
                    pub SetTextAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_TEXT_ANTIALIAS_MODE),
                    pub GetTextAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_TEXT_ANTIALIAS_MODE,
                    SetTextRenderingParams: usize,
                    GetTextRenderingParams: usize,
                    pub SetTags: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64),
                    pub GetTags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64),
                    pub PushLayer: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_LAYER_PARAMETERS, *mut core::ffi::c_void),
                    pub PopLayer: unsafe extern "system" fn(*mut core::ffi::c_void),
                    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
                    SaveDrawingState: usize,
                    RestoreDrawingState: usize,
                    pub PushAxisAlignedClip:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_RECT_F, D2D1_ANTIALIAS_MODE),
                    pub PopAxisAlignedClip: unsafe extern "system" fn(*mut core::ffi::c_void),
                    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D1_COLOR_F),
                    pub BeginDraw: unsafe extern "system" fn(*mut core::ffi::c_void),
                    pub EndDraw: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
                    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D1_PIXEL_FORMAT),
                    pub SetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32),
                    pub GetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
                    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_SIZE_F),
                    pub GetPixelSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_SIZE_U),
                    pub GetMaximumBitmapSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
                    pub IsSupported:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_RENDER_TARGET_PROPERTIES) -> windows_core::BOOL,
                }
                unsafe impl Send for ID2D1RenderTarget {}
                unsafe impl Sync for ID2D1RenderTarget {}
                impl windows_core::RuntimeName for ID2D1RenderTarget {}
                windows_core::imp::define_interface!(ID2D1Resource, ID2D1Resource_Vtbl, 0x2cd90691_12e2_11dc_9fed_001143a055f9);
                windows_core::imp::interface_hierarchy!(ID2D1Resource, windows_core::IUnknown);
                impl ID2D1Resource {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1Resource_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub GetFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
                }
                unsafe impl Send for ID2D1Resource {}
                unsafe impl Sync for ID2D1Resource {}
                impl windows_core::RuntimeName for ID2D1Resource {}
                windows_core::imp::define_interface!(
                    ID2D1RoundedRectangleGeometry,
                    ID2D1RoundedRectangleGeometry_Vtbl,
                    0x2cd906a3_12e2_11dc_9fed_001143a055f9
                );
                impl core::ops::Deref for ID2D1RoundedRectangleGeometry {
                    type Target = ID2D1Geometry;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(
                    ID2D1RoundedRectangleGeometry,
                    windows_core::IUnknown,
                    ID2D1Resource,
                    ID2D1Geometry
                );
                impl ID2D1RoundedRectangleGeometry {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1RoundedRectangleGeometry_Vtbl {
                    pub base__: ID2D1Geometry_Vtbl,
                    pub GetRoundedRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_ROUNDED_RECT),
                }
                unsafe impl Send for ID2D1RoundedRectangleGeometry {}
                unsafe impl Sync for ID2D1RoundedRectangleGeometry {}
                impl windows_core::RuntimeName for ID2D1RoundedRectangleGeometry {}
                windows_core::imp::define_interface!(
                    ID2D1SolidColorBrush,
                    ID2D1SolidColorBrush_Vtbl,
                    0x2cd906a9_12e2_11dc_9fed_001143a055f9
                );
                impl core::ops::Deref for ID2D1SolidColorBrush {
                    type Target = ID2D1Brush;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID2D1SolidColorBrush, windows_core::IUnknown, ID2D1Resource, ID2D1Brush);
                impl ID2D1SolidColorBrush {
                    pub unsafe fn SetColor(&self, color: *const Common::D2D1_COLOR_F) {
                        unsafe { (windows_core::Interface::vtable(self).SetColor)(windows_core::Interface::as_raw(self), color) }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1SolidColorBrush_Vtbl {
                    pub base__: ID2D1Brush_Vtbl,
                    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D1_COLOR_F),
                    pub GetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D1_COLOR_F),
                }
                unsafe impl Send for ID2D1SolidColorBrush {}
                unsafe impl Sync for ID2D1SolidColorBrush {}
                impl windows_core::RuntimeName for ID2D1SolidColorBrush {}
                windows_core::imp::define_interface!(ID2D1StrokeStyle, ID2D1StrokeStyle_Vtbl, 0x2cd9069d_12e2_11dc_9fed_001143a055f9);
                impl core::ops::Deref for ID2D1StrokeStyle {
                    type Target = ID2D1Resource;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID2D1StrokeStyle, windows_core::IUnknown, ID2D1Resource);
                impl ID2D1StrokeStyle {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1StrokeStyle_Vtbl {
                    pub base__: ID2D1Resource_Vtbl,
                    GetStartCap: usize,
                    GetEndCap: usize,
                    GetDashCap: usize,
                    pub GetMiterLimit: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
                    GetLineJoin: usize,
                    pub GetDashOffset: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
                    GetDashStyle: usize,
                    pub GetDashesCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
                    pub GetDashes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32),
                }
                unsafe impl Send for ID2D1StrokeStyle {}
                unsafe impl Sync for ID2D1StrokeStyle {}
                impl windows_core::RuntimeName for ID2D1StrokeStyle {}
                pub mod Common {
                    #[repr(transparent)]
                    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                    pub struct D2D1_ALPHA_MODE(pub i32);
                    pub const D2D1_ALPHA_MODE_PREMULTIPLIED: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(1i32);
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, Default, PartialEq)]
                    pub struct D2D1_COLOR_F {
                        pub r: f32,
                        pub g: f32,
                        pub b: f32,
                        pub a: f32,
                    }
                    #[repr(transparent)]
                    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                    pub struct D2D1_FIGURE_BEGIN(pub i32);
                    pub const D2D1_FIGURE_BEGIN_FILLED: D2D1_FIGURE_BEGIN = D2D1_FIGURE_BEGIN(0i32);
                    #[repr(transparent)]
                    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                    pub struct D2D1_FIGURE_END(pub i32);
                    pub const D2D1_FIGURE_END_CLOSED: D2D1_FIGURE_END = D2D1_FIGURE_END(1i32);
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, Default, PartialEq)]
                    pub struct D2D1_GRADIENT_STOP {
                        pub position: f32,
                        pub color: D2D1_COLOR_F,
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, Default, PartialEq)]
                    pub struct D2D1_PIXEL_FORMAT {
                        pub format: super::super::Dxgi::Common::DXGI_FORMAT,
                        pub alphaMode: D2D1_ALPHA_MODE,
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, Default, PartialEq)]
                    pub struct D2D_RECT_F {
                        pub left: f32,
                        pub top: f32,
                        pub right: f32,
                        pub bottom: f32,
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, Default, PartialEq)]
                    pub struct D2D_SIZE_F {
                        pub width: f32,
                        pub height: f32,
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, Default, PartialEq)]
                    pub struct D2D_SIZE_U {
                        pub width: u32,
                        pub height: u32,
                    }
                    windows_core::imp::define_interface!(
                        ID2D1SimplifiedGeometrySink,
                        ID2D1SimplifiedGeometrySink_Vtbl,
                        0x2cd9069e_12e2_11dc_9fed_001143a055f9
                    );
                    windows_core::imp::interface_hierarchy!(ID2D1SimplifiedGeometrySink, windows_core::IUnknown);
                    impl ID2D1SimplifiedGeometrySink {
                        pub unsafe fn BeginFigure(&self, startpoint: windows_numerics::Vector2, figurebegin: D2D1_FIGURE_BEGIN) {
                            unsafe {
                                (windows_core::Interface::vtable(self).BeginFigure)(
                                    windows_core::Interface::as_raw(self),
                                    core::mem::transmute(startpoint),
                                    figurebegin,
                                )
                            }
                        }
                        pub unsafe fn EndFigure(&self, figureend: D2D1_FIGURE_END) {
                            unsafe { (windows_core::Interface::vtable(self).EndFigure)(windows_core::Interface::as_raw(self), figureend) }
                        }
                        pub unsafe fn Close(&self) -> windows_core::Result<()> {
                            unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
                        }
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct ID2D1SimplifiedGeometrySink_Vtbl {
                        pub base__: windows_core::IUnknown_Vtbl,
                        SetFillMode: usize,
                        SetSegmentFlags: usize,
                        pub BeginFigure: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector2, D2D1_FIGURE_BEGIN),
                        pub AddLines: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Vector2, u32),
                        AddBeziers: usize,
                        pub EndFigure: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_FIGURE_END),
                        pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    }
                    unsafe impl Send for ID2D1SimplifiedGeometrySink {}
                    unsafe impl Sync for ID2D1SimplifiedGeometrySink {}
                    impl windows_core::RuntimeName for ID2D1SimplifiedGeometrySink {}
                }
            }
            pub mod Direct3D {
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D3D_DRIVER_TYPE(pub i32);
                pub const D3D_DRIVER_TYPE_HARDWARE: D3D_DRIVER_TYPE = D3D_DRIVER_TYPE(1i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D3D_FEATURE_LEVEL(pub i32);
                pub const D3D_FEATURE_LEVEL_11_0: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL(45056i32);
                pub const D3D_FEATURE_LEVEL_11_1: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL(45312i32);
            }
            pub mod Direct3D11 {
                #[inline]
                pub unsafe fn D3D11CreateDevice<P0>(
                    padapter: P0,
                    drivertype: super::Direct3D::D3D_DRIVER_TYPE,
                    software: super::super::Foundation::HMODULE,
                    flags: D3D11_CREATE_DEVICE_FLAG,
                    pfeaturelevels: Option<&[super::Direct3D::D3D_FEATURE_LEVEL]>,
                    sdkversion: u32,
                    ppdevice: Option<*mut Option<ID3D11Device>>,
                    pfeaturelevel: Option<*mut super::Direct3D::D3D_FEATURE_LEVEL>,
                    ppimmediatecontext: Option<*mut Option<ID3D11DeviceContext>>,
                ) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<super::Dxgi::IDXGIAdapter>,
                {
                    windows_core::link!("d3d11.dll" "system" fn D3D11CreateDevice(padapter : * mut core::ffi::c_void, drivertype : super::Direct3D:: D3D_DRIVER_TYPE, software : super::super::Foundation:: HMODULE, flags : D3D11_CREATE_DEVICE_FLAG, pfeaturelevels : *const super::Direct3D:: D3D_FEATURE_LEVEL, featurelevels : u32, sdkversion : u32, ppdevice : *mut * mut core::ffi::c_void, pfeaturelevel : *mut super::Direct3D:: D3D_FEATURE_LEVEL, ppimmediatecontext : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
                    unsafe {
                        D3D11CreateDevice(
                            padapter.param().abi(),
                            drivertype,
                            software,
                            flags,
                            core::mem::transmute(pfeaturelevels.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
                            pfeaturelevels.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
                            sdkversion,
                            ppdevice.unwrap_or(core::mem::zeroed()) as _,
                            pfeaturelevel.unwrap_or(core::mem::zeroed()) as _,
                            ppimmediatecontext.unwrap_or(core::mem::zeroed()) as _,
                        )
                        .ok()
                    }
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D3D11_BIND_FLAG(pub i32);
                impl D3D11_BIND_FLAG {
                    pub const fn contains(&self, other: Self) -> bool {
                        self.0 & other.0 == other.0
                    }
                }
                impl core::ops::BitOr for D3D11_BIND_FLAG {
                    type Output = Self;
                    fn bitor(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }
                impl core::ops::BitAnd for D3D11_BIND_FLAG {
                    type Output = Self;
                    fn bitand(self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
                impl core::ops::BitOrAssign for D3D11_BIND_FLAG {
                    fn bitor_assign(&mut self, other: Self) {
                        self.0.bitor_assign(other.0)
                    }
                }
                impl core::ops::BitAndAssign for D3D11_BIND_FLAG {
                    fn bitand_assign(&mut self, other: Self) {
                        self.0.bitand_assign(other.0)
                    }
                }
                impl core::ops::Not for D3D11_BIND_FLAG {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                pub const D3D11_BIND_RENDER_TARGET: D3D11_BIND_FLAG = D3D11_BIND_FLAG(32i32);
                pub const D3D11_BIND_SHADER_RESOURCE: D3D11_BIND_FLAG = D3D11_BIND_FLAG(8i32);
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct D3D11_BOX {
                    pub left: u32,
                    pub top: u32,
                    pub front: u32,
                    pub right: u32,
                    pub bottom: u32,
                    pub back: u32,
                }
                pub const D3D11_CREATE_DEVICE_BGRA_SUPPORT: D3D11_CREATE_DEVICE_FLAG = D3D11_CREATE_DEVICE_FLAG(32u32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D3D11_CREATE_DEVICE_FLAG(pub u32);
                impl D3D11_CREATE_DEVICE_FLAG {
                    pub const fn contains(&self, other: Self) -> bool {
                        self.0 & other.0 == other.0
                    }
                }
                impl core::ops::BitOr for D3D11_CREATE_DEVICE_FLAG {
                    type Output = Self;
                    fn bitor(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }
                impl core::ops::BitAnd for D3D11_CREATE_DEVICE_FLAG {
                    type Output = Self;
                    fn bitand(self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
                impl core::ops::BitOrAssign for D3D11_CREATE_DEVICE_FLAG {
                    fn bitor_assign(&mut self, other: Self) {
                        self.0.bitor_assign(other.0)
                    }
                }
                impl core::ops::BitAndAssign for D3D11_CREATE_DEVICE_FLAG {
                    fn bitand_assign(&mut self, other: Self) {
                        self.0.bitand_assign(other.0)
                    }
                }
                impl core::ops::Not for D3D11_CREATE_DEVICE_FLAG {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                pub const D3D11_CREATE_DEVICE_VIDEO_SUPPORT: D3D11_CREATE_DEVICE_FLAG = D3D11_CREATE_DEVICE_FLAG(2048u32);
                pub const D3D11_SDK_VERSION: u32 = 7u32;
                #[repr(C)]
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct D3D11_SUBRESOURCE_DATA {
                    pub pSysMem: *const core::ffi::c_void,
                    pub SysMemPitch: u32,
                    pub SysMemSlicePitch: u32,
                }
                impl Default for D3D11_SUBRESOURCE_DATA {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct D3D11_TEX2D_ARRAY_VPOV {
                    pub MipSlice: u32,
                    pub FirstArraySlice: u32,
                    pub ArraySize: u32,
                }
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct D3D11_TEX2D_VPIV {
                    pub MipSlice: u32,
                    pub ArraySlice: u32,
                }
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct D3D11_TEX2D_VPOV {
                    pub MipSlice: u32,
                }
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct D3D11_TEXTURE2D_DESC {
                    pub Width: u32,
                    pub Height: u32,
                    pub MipLevels: u32,
                    pub ArraySize: u32,
                    pub Format: super::Dxgi::Common::DXGI_FORMAT,
                    pub SampleDesc: super::Dxgi::Common::DXGI_SAMPLE_DESC,
                    pub Usage: D3D11_USAGE,
                    pub BindFlags: u32,
                    pub CPUAccessFlags: u32,
                    pub MiscFlags: u32,
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D3D11_USAGE(pub i32);
                pub const D3D11_USAGE_DEFAULT: D3D11_USAGE = D3D11_USAGE(0i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D3D11_VIDEO_FRAME_FORMAT(pub i32);
                pub const D3D11_VIDEO_FRAME_FORMAT_PROGRESSIVE: D3D11_VIDEO_FRAME_FORMAT = D3D11_VIDEO_FRAME_FORMAT(0i32);
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct D3D11_VIDEO_PROCESSOR_CONTENT_DESC {
                    pub InputFrameFormat: D3D11_VIDEO_FRAME_FORMAT,
                    pub InputFrameRate: super::Dxgi::Common::DXGI_RATIONAL,
                    pub InputWidth: u32,
                    pub InputHeight: u32,
                    pub OutputFrameRate: super::Dxgi::Common::DXGI_RATIONAL,
                    pub OutputWidth: u32,
                    pub OutputHeight: u32,
                    pub Usage: D3D11_VIDEO_USAGE,
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
                    pub FourCC: u32,
                    pub ViewDimension: D3D11_VPIV_DIMENSION,
                    pub Anonymous: D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0,
                }
                impl Default for D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub union D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0 {
                    pub Texture2D: D3D11_TEX2D_VPIV,
                }
                impl Default for D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0 {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
                    pub ViewDimension: D3D11_VPOV_DIMENSION,
                    pub Anonymous: D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0,
                }
                impl Default for D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub union D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0 {
                    pub Texture2D: D3D11_TEX2D_VPOV,
                    pub Texture2DArray: D3D11_TEX2D_ARRAY_VPOV,
                }
                impl Default for D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0 {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Debug, PartialEq)]
                pub struct D3D11_VIDEO_PROCESSOR_STREAM {
                    pub Enable: windows_core::BOOL,
                    pub OutputIndex: u32,
                    pub InputFrameOrField: u32,
                    pub PastFrames: u32,
                    pub FutureFrames: u32,
                    pub ppPastSurfaces: *mut Option<ID3D11VideoProcessorInputView>,
                    pub pInputSurface: core::mem::ManuallyDrop<Option<ID3D11VideoProcessorInputView>>,
                    pub ppFutureSurfaces: *mut Option<ID3D11VideoProcessorInputView>,
                    pub ppPastSurfacesRight: *mut Option<ID3D11VideoProcessorInputView>,
                    pub pInputSurfaceRight: core::mem::ManuallyDrop<Option<ID3D11VideoProcessorInputView>>,
                    pub ppFutureSurfacesRight: *mut Option<ID3D11VideoProcessorInputView>,
                }
                impl Default for D3D11_VIDEO_PROCESSOR_STREAM {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D3D11_VIDEO_USAGE(pub i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D3D11_VPIV_DIMENSION(pub i32);
                pub const D3D11_VPIV_DIMENSION_TEXTURE2D: D3D11_VPIV_DIMENSION = D3D11_VPIV_DIMENSION(1i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct D3D11_VPOV_DIMENSION(pub i32);
                pub const D3D11_VPOV_DIMENSION_TEXTURE2D: D3D11_VPOV_DIMENSION = D3D11_VPOV_DIMENSION(1i32);
                windows_core::imp::define_interface!(ID3D11Device, ID3D11Device_Vtbl, 0xdb6f6ddb_ac77_4e88_8253_819df9bbf140);
                windows_core::imp::interface_hierarchy!(ID3D11Device, windows_core::IUnknown);
                impl ID3D11Device {
                    pub unsafe fn CreateTexture2D(
                        &self,
                        pdesc: *const D3D11_TEXTURE2D_DESC,
                        pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>,
                        pptexture2d: Option<*mut Option<ID3D11Texture2D>>,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).CreateTexture2D)(
                                windows_core::Interface::as_raw(self),
                                pdesc,
                                pinitialdata.unwrap_or(core::mem::zeroed()) as _,
                                pptexture2d.unwrap_or(core::mem::zeroed()) as _,
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID3D11Device_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    CreateBuffer: usize,
                    CreateTexture1D: usize,
                    pub CreateTexture2D: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const D3D11_TEXTURE2D_DESC,
                        *const D3D11_SUBRESOURCE_DATA,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    CreateTexture3D: usize,
                    CreateShaderResourceView: usize,
                    CreateUnorderedAccessView: usize,
                    CreateRenderTargetView: usize,
                    CreateDepthStencilView: usize,
                    CreateInputLayout: usize,
                    CreateVertexShader: usize,
                    CreateGeometryShader: usize,
                    CreateGeometryShaderWithStreamOutput: usize,
                    CreatePixelShader: usize,
                    CreateHullShader: usize,
                    CreateDomainShader: usize,
                    CreateComputeShader: usize,
                    CreateClassLinkage: usize,
                    CreateBlendState: usize,
                    CreateDepthStencilState: usize,
                    CreateRasterizerState: usize,
                    CreateSamplerState: usize,
                    CreateQuery: usize,
                    CreatePredicate: usize,
                    CreateCounter: usize,
                    pub CreateDeferredContext:
                        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub OpenSharedResource: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        super::super::Foundation::HANDLE,
                        *const windows_core::GUID,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub CheckFormatSupport: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        super::Dxgi::Common::DXGI_FORMAT,
                        *mut u32,
                    ) -> windows_core::HRESULT,
                    pub CheckMultisampleQualityLevels: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        super::Dxgi::Common::DXGI_FORMAT,
                        u32,
                        *mut u32,
                    ) -> windows_core::HRESULT,
                    CheckCounterInfo: usize,
                    CheckCounter: usize,
                    CheckFeatureSupport: usize,
                    pub GetPrivateData: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *mut u32,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetPrivateData: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        u32,
                        *const core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetPrivateDataInterface: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub GetFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::Direct3D::D3D_FEATURE_LEVEL,
                    pub GetCreationFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
                    pub GetDeviceRemovedReason: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetImmediateContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
                    pub SetExceptionMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
                    pub GetExceptionMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
                }
                unsafe impl Send for ID3D11Device {}
                unsafe impl Sync for ID3D11Device {}
                impl windows_core::RuntimeName for ID3D11Device {}
                windows_core::imp::define_interface!(ID3D11DeviceChild, ID3D11DeviceChild_Vtbl, 0x1841e5c8_16b0_489b_bcc8_44cfb0d5deae);
                windows_core::imp::interface_hierarchy!(ID3D11DeviceChild, windows_core::IUnknown);
                impl ID3D11DeviceChild {
                    pub unsafe fn GetDevice(&self) -> windows_core::Result<ID3D11Device> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__);
                            windows_core::Type::from_abi(result__)
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID3D11DeviceChild_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
                    pub GetPrivateData: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *mut u32,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetPrivateData: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        u32,
                        *const core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetPrivateDataInterface: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                unsafe impl Send for ID3D11DeviceChild {}
                unsafe impl Sync for ID3D11DeviceChild {}
                impl windows_core::RuntimeName for ID3D11DeviceChild {}
                windows_core::imp::define_interface!(ID3D11DeviceContext, ID3D11DeviceContext_Vtbl, 0xc0bfa96c_e089_44fb_8eaf_26f8796190da);
                impl core::ops::Deref for ID3D11DeviceContext {
                    type Target = ID3D11DeviceChild;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID3D11DeviceContext, windows_core::IUnknown, ID3D11DeviceChild);
                impl ID3D11DeviceContext {
                    pub unsafe fn CopySubresourceRegion<P0, P5>(
                        &self,
                        pdstresource: P0,
                        dstsubresource: u32,
                        dstx: u32,
                        dsty: u32,
                        dstz: u32,
                        psrcresource: P5,
                        srcsubresource: u32,
                        psrcbox: Option<*const D3D11_BOX>,
                    ) where
                        P0: windows_core::Param<ID3D11Resource>,
                        P5: windows_core::Param<ID3D11Resource>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).CopySubresourceRegion)(
                                windows_core::Interface::as_raw(self),
                                pdstresource.param().abi(),
                                dstsubresource,
                                dstx,
                                dsty,
                                dstz,
                                psrcresource.param().abi(),
                                srcsubresource,
                                psrcbox.unwrap_or(core::mem::zeroed()) as _,
                            )
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID3D11DeviceContext_Vtbl {
                    pub base__: ID3D11DeviceChild_Vtbl,
                    VSSetConstantBuffers: usize,
                    PSSetShaderResources: usize,
                    PSSetShader: usize,
                    PSSetSamplers: usize,
                    VSSetShader: usize,
                    pub DrawIndexed: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, i32),
                    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32),
                    Map: usize,
                    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
                    PSSetConstantBuffers: usize,
                    IASetInputLayout: usize,
                    IASetVertexBuffers: usize,
                    IASetIndexBuffer: usize,
                    pub DrawIndexedInstanced: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, i32, u32),
                    pub DrawInstanced: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32),
                    GSSetConstantBuffers: usize,
                    GSSetShader: usize,
                    IASetPrimitiveTopology: usize,
                    VSSetShaderResources: usize,
                    VSSetSamplers: usize,
                    Begin: usize,
                    End: usize,
                    GetData: usize,
                    SetPredication: usize,
                    GSSetShaderResources: usize,
                    GSSetSamplers: usize,
                    OMSetRenderTargets: usize,
                    OMSetRenderTargetsAndUnorderedAccessViews: usize,
                    OMSetBlendState: usize,
                    OMSetDepthStencilState: usize,
                    SOSetTargets: usize,
                    pub DrawAuto: unsafe extern "system" fn(*mut core::ffi::c_void),
                    DrawIndexedInstancedIndirect: usize,
                    DrawInstancedIndirect: usize,
                    pub Dispatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32),
                    DispatchIndirect: usize,
                    RSSetState: usize,
                    RSSetViewports: usize,
                    pub RSSetScissorRects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Foundation::RECT),
                    pub CopySubresourceRegion: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        u32,
                        u32,
                        u32,
                        *mut core::ffi::c_void,
                        u32,
                        *const D3D11_BOX,
                    ),
                    pub CopyResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
                    pub UpdateSubresource: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        *const D3D11_BOX,
                        *const core::ffi::c_void,
                        u32,
                        u32,
                    ),
                    CopyStructureCount: usize,
                    ClearRenderTargetView: usize,
                    ClearUnorderedAccessViewUint: usize,
                    ClearUnorderedAccessViewFloat: usize,
                    ClearDepthStencilView: usize,
                    GenerateMips: usize,
                    pub SetResourceMinLOD: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32),
                    pub GetResourceMinLOD: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> f32,
                    pub ResolveSubresource: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        *mut core::ffi::c_void,
                        u32,
                        super::Dxgi::Common::DXGI_FORMAT,
                    ),
                    ExecuteCommandList: usize,
                    HSSetShaderResources: usize,
                    HSSetShader: usize,
                    HSSetSamplers: usize,
                    HSSetConstantBuffers: usize,
                    DSSetShaderResources: usize,
                    DSSetShader: usize,
                    DSSetSamplers: usize,
                    DSSetConstantBuffers: usize,
                    CSSetShaderResources: usize,
                    CSSetUnorderedAccessViews: usize,
                    CSSetShader: usize,
                    CSSetSamplers: usize,
                    CSSetConstantBuffers: usize,
                    VSGetConstantBuffers: usize,
                    PSGetShaderResources: usize,
                    PSGetShader: usize,
                    PSGetSamplers: usize,
                    VSGetShader: usize,
                    PSGetConstantBuffers: usize,
                    IAGetInputLayout: usize,
                    IAGetVertexBuffers: usize,
                    IAGetIndexBuffer: usize,
                    GSGetConstantBuffers: usize,
                    GSGetShader: usize,
                    IAGetPrimitiveTopology: usize,
                    VSGetShaderResources: usize,
                    VSGetSamplers: usize,
                    GetPredication: usize,
                    GSGetShaderResources: usize,
                    GSGetSamplers: usize,
                    OMGetRenderTargets: usize,
                    OMGetRenderTargetsAndUnorderedAccessViews: usize,
                    OMGetBlendState: usize,
                    OMGetDepthStencilState: usize,
                    SOGetTargets: usize,
                    RSGetState: usize,
                    RSGetViewports: usize,
                    pub RSGetScissorRects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut super::super::Foundation::RECT),
                    HSGetShaderResources: usize,
                    HSGetShader: usize,
                    HSGetSamplers: usize,
                    HSGetConstantBuffers: usize,
                    DSGetShaderResources: usize,
                    DSGetShader: usize,
                    DSGetSamplers: usize,
                    DSGetConstantBuffers: usize,
                    CSGetShaderResources: usize,
                    CSGetUnorderedAccessViews: usize,
                    CSGetShader: usize,
                    CSGetSamplers: usize,
                    CSGetConstantBuffers: usize,
                    pub ClearState: unsafe extern "system" fn(*mut core::ffi::c_void),
                    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void),
                    GetType: usize,
                    pub GetContextFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
                    FinishCommandList: usize,
                }
                unsafe impl Send for ID3D11DeviceContext {}
                unsafe impl Sync for ID3D11DeviceContext {}
                impl windows_core::RuntimeName for ID3D11DeviceContext {}
                windows_core::imp::define_interface!(ID3D11Multithread, ID3D11Multithread_Vtbl, 0x9b7e4e00_342c_4106_a19f_4f2704f689f0);
                windows_core::imp::interface_hierarchy!(ID3D11Multithread, windows_core::IUnknown);
                impl ID3D11Multithread {
                    pub unsafe fn Enter(&self) {
                        unsafe { (windows_core::Interface::vtable(self).Enter)(windows_core::Interface::as_raw(self)) }
                    }
                    pub unsafe fn SetMultithreadProtected(&self, bmtprotect: bool) -> windows_core::BOOL {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetMultithreadProtected)(
                                windows_core::Interface::as_raw(self),
                                bmtprotect.into(),
                            )
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID3D11Multithread_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub Enter: unsafe extern "system" fn(*mut core::ffi::c_void),
                    pub Leave: unsafe extern "system" fn(*mut core::ffi::c_void),
                    pub SetMultithreadProtected:
                        unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::BOOL,
                    pub GetMultithreadProtected: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
                }
                unsafe impl Send for ID3D11Multithread {}
                unsafe impl Sync for ID3D11Multithread {}
                impl windows_core::RuntimeName for ID3D11Multithread {}
                windows_core::imp::define_interface!(ID3D11Resource, ID3D11Resource_Vtbl, 0xdc8e63f3_d12b_4952_b47b_5e45026a862d);
                impl core::ops::Deref for ID3D11Resource {
                    type Target = ID3D11DeviceChild;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID3D11Resource, windows_core::IUnknown, ID3D11DeviceChild);
                impl ID3D11Resource {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID3D11Resource_Vtbl {
                    pub base__: ID3D11DeviceChild_Vtbl,
                    GetType: usize,
                    pub SetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
                    pub GetEvictionPriority: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
                }
                unsafe impl Send for ID3D11Resource {}
                unsafe impl Sync for ID3D11Resource {}
                impl windows_core::RuntimeName for ID3D11Resource {}
                windows_core::imp::define_interface!(ID3D11Texture2D, ID3D11Texture2D_Vtbl, 0x6f15aaf2_d208_4e89_9ab4_489535d34f9c);
                impl core::ops::Deref for ID3D11Texture2D {
                    type Target = ID3D11Resource;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID3D11Texture2D, windows_core::IUnknown, ID3D11DeviceChild, ID3D11Resource);
                impl ID3D11Texture2D {
                    pub unsafe fn GetDesc(&self, pdesc: *mut D3D11_TEXTURE2D_DESC) {
                        unsafe { (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), pdesc as _) }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID3D11Texture2D_Vtbl {
                    pub base__: ID3D11Resource_Vtbl,
                    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_TEXTURE2D_DESC),
                }
                unsafe impl Send for ID3D11Texture2D {}
                unsafe impl Sync for ID3D11Texture2D {}
                impl windows_core::RuntimeName for ID3D11Texture2D {}
                windows_core::imp::define_interface!(ID3D11VideoContext, ID3D11VideoContext_Vtbl, 0x61f21c45_3c0e_4a74_9cea_67100d9ad5e4);
                impl core::ops::Deref for ID3D11VideoContext {
                    type Target = ID3D11DeviceChild;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID3D11VideoContext, windows_core::IUnknown, ID3D11DeviceChild);
                impl ID3D11VideoContext {
                    pub unsafe fn VideoProcessorSetOutputTargetRect<P0>(
                        &self,
                        pvideoprocessor: P0,
                        enable: bool,
                        prect: Option<*const super::super::Foundation::RECT>,
                    ) where
                        P0: windows_core::Param<ID3D11VideoProcessor>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).VideoProcessorSetOutputTargetRect)(
                                windows_core::Interface::as_raw(self),
                                pvideoprocessor.param().abi(),
                                enable.into(),
                                prect.unwrap_or(core::mem::zeroed()) as _,
                            )
                        }
                    }
                    pub unsafe fn VideoProcessorSetStreamSourceRect<P0>(
                        &self,
                        pvideoprocessor: P0,
                        streamindex: u32,
                        enable: bool,
                        prect: Option<*const super::super::Foundation::RECT>,
                    ) where
                        P0: windows_core::Param<ID3D11VideoProcessor>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).VideoProcessorSetStreamSourceRect)(
                                windows_core::Interface::as_raw(self),
                                pvideoprocessor.param().abi(),
                                streamindex,
                                enable.into(),
                                prect.unwrap_or(core::mem::zeroed()) as _,
                            )
                        }
                    }
                    pub unsafe fn VideoProcessorSetStreamDestRect<P0>(
                        &self,
                        pvideoprocessor: P0,
                        streamindex: u32,
                        enable: bool,
                        prect: Option<*const super::super::Foundation::RECT>,
                    ) where
                        P0: windows_core::Param<ID3D11VideoProcessor>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).VideoProcessorSetStreamDestRect)(
                                windows_core::Interface::as_raw(self),
                                pvideoprocessor.param().abi(),
                                streamindex,
                                enable.into(),
                                prect.unwrap_or(core::mem::zeroed()) as _,
                            )
                        }
                    }
                    pub unsafe fn VideoProcessorBlt<P0, P1>(
                        &self,
                        pvideoprocessor: P0,
                        pview: P1,
                        outputframe: u32,
                        pstreams: &[D3D11_VIDEO_PROCESSOR_STREAM],
                    ) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<ID3D11VideoProcessor>,
                        P1: windows_core::Param<ID3D11VideoProcessorOutputView>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).VideoProcessorBlt)(
                                windows_core::Interface::as_raw(self),
                                pvideoprocessor.param().abi(),
                                pview.param().abi(),
                                outputframe,
                                pstreams.len().try_into().unwrap(),
                                core::mem::transmute(pstreams.as_ptr()),
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID3D11VideoContext_Vtbl {
                    pub base__: ID3D11DeviceChild_Vtbl,
                    GetDecoderBuffer: usize,
                    ReleaseDecoderBuffer: usize,
                    DecoderBeginFrame: usize,
                    DecoderEndFrame: usize,
                    SubmitDecoderBuffers: usize,
                    DecoderExtension: usize,
                    pub VideoProcessorSetOutputTargetRect: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        windows_core::BOOL,
                        *const super::super::Foundation::RECT,
                    ),
                    VideoProcessorSetOutputBackgroundColor: usize,
                    VideoProcessorSetOutputColorSpace: usize,
                    VideoProcessorSetOutputAlphaFillMode: usize,
                    pub VideoProcessorSetOutputConstriction: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        windows_core::BOOL,
                        super::super::Foundation::SIZE,
                    ),
                    pub VideoProcessorSetOutputStereoMode:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL),
                    pub VideoProcessorSetOutputExtension: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        u32,
                        *const core::ffi::c_void,
                    ) -> i32,
                    pub VideoProcessorGetOutputTargetRect: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut windows_core::BOOL,
                        *mut super::super::Foundation::RECT,
                    ),
                    VideoProcessorGetOutputBackgroundColor: usize,
                    VideoProcessorGetOutputColorSpace: usize,
                    VideoProcessorGetOutputAlphaFillMode: usize,
                    pub VideoProcessorGetOutputConstriction: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut windows_core::BOOL,
                        *mut super::super::Foundation::SIZE,
                    ),
                    pub VideoProcessorGetOutputStereoMode:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL),
                    pub VideoProcessorGetOutputExtension: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        u32,
                        *mut core::ffi::c_void,
                    ) -> i32,
                    pub VideoProcessorSetStreamFrameFormat:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, D3D11_VIDEO_FRAME_FORMAT),
                    VideoProcessorSetStreamColorSpace: usize,
                    VideoProcessorSetStreamOutputRate: usize,
                    pub VideoProcessorSetStreamSourceRect: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        windows_core::BOOL,
                        *const super::super::Foundation::RECT,
                    ),
                    pub VideoProcessorSetStreamDestRect: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        windows_core::BOOL,
                        *const super::super::Foundation::RECT,
                    ),
                    pub VideoProcessorSetStreamAlpha:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::BOOL, f32),
                    pub VideoProcessorSetStreamPalette:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *const u32),
                    pub VideoProcessorSetStreamPixelAspectRatio: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        windows_core::BOOL,
                        *const super::Dxgi::Common::DXGI_RATIONAL,
                        *const super::Dxgi::Common::DXGI_RATIONAL,
                    ),
                    pub VideoProcessorSetStreamLumaKey:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::BOOL, f32, f32),
                    VideoProcessorSetStreamStereoFormat: usize,
                    pub VideoProcessorSetStreamAutoProcessingMode:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, windows_core::BOOL),
                    VideoProcessorSetStreamFilter: usize,
                    pub VideoProcessorSetStreamExtension: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        *const windows_core::GUID,
                        u32,
                        *const core::ffi::c_void,
                    ) -> i32,
                    pub VideoProcessorGetStreamFrameFormat:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut D3D11_VIDEO_FRAME_FORMAT),
                    VideoProcessorGetStreamColorSpace: usize,
                    VideoProcessorGetStreamOutputRate: usize,
                    pub VideoProcessorGetStreamSourceRect: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        *mut windows_core::BOOL,
                        *mut super::super::Foundation::RECT,
                    ),
                    pub VideoProcessorGetStreamDestRect: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        *mut windows_core::BOOL,
                        *mut super::super::Foundation::RECT,
                    ),
                    pub VideoProcessorGetStreamAlpha:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut windows_core::BOOL, *mut f32),
                    pub VideoProcessorGetStreamPalette:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut u32),
                    pub VideoProcessorGetStreamPixelAspectRatio: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        *mut windows_core::BOOL,
                        *mut super::Dxgi::Common::DXGI_RATIONAL,
                        *mut super::Dxgi::Common::DXGI_RATIONAL,
                    ),
                    pub VideoProcessorGetStreamLumaKey: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        *mut windows_core::BOOL,
                        *mut f32,
                        *mut f32,
                    ),
                    VideoProcessorGetStreamStereoFormat: usize,
                    pub VideoProcessorGetStreamAutoProcessingMode:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut windows_core::BOOL),
                    VideoProcessorGetStreamFilter: usize,
                    pub VideoProcessorGetStreamExtension: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        *const windows_core::GUID,
                        u32,
                        *mut core::ffi::c_void,
                    ) -> i32,
                    pub VideoProcessorBlt: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        u32,
                        *const D3D11_VIDEO_PROCESSOR_STREAM,
                    ) -> windows_core::HRESULT,
                    NegotiateCryptoSessionKeyExchange: usize,
                    EncryptionBlt: usize,
                    DecryptionBlt: usize,
                    StartSessionKeyRefresh: usize,
                    FinishSessionKeyRefresh: usize,
                    GetEncryptionBltKey: usize,
                    NegotiateAuthenticatedChannelKeyExchange: usize,
                    QueryAuthenticatedChannel: usize,
                    ConfigureAuthenticatedChannel: usize,
                    VideoProcessorSetStreamRotation: usize,
                    VideoProcessorGetStreamRotation: usize,
                }
                unsafe impl Send for ID3D11VideoContext {}
                unsafe impl Sync for ID3D11VideoContext {}
                impl windows_core::RuntimeName for ID3D11VideoContext {}
                windows_core::imp::define_interface!(ID3D11VideoDevice, ID3D11VideoDevice_Vtbl, 0x10ec4d5b_975a_4689_b9e4_d0aac30fe333);
                windows_core::imp::interface_hierarchy!(ID3D11VideoDevice, windows_core::IUnknown);
                impl ID3D11VideoDevice {
                    pub unsafe fn CreateVideoProcessor<P0>(
                        &self,
                        penum: P0,
                        rateconversionindex: u32,
                    ) -> windows_core::Result<ID3D11VideoProcessor>
                    where
                        P0: windows_core::Param<ID3D11VideoProcessorEnumerator>,
                    {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateVideoProcessor)(
                                windows_core::Interface::as_raw(self),
                                penum.param().abi(),
                                rateconversionindex,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn CreateVideoProcessorInputView<P0, P1>(
                        &self,
                        presource: P0,
                        penum: P1,
                        pdesc: *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC,
                        ppvpiview: Option<*mut Option<ID3D11VideoProcessorInputView>>,
                    ) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<ID3D11Resource>,
                        P1: windows_core::Param<ID3D11VideoProcessorEnumerator>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).CreateVideoProcessorInputView)(
                                windows_core::Interface::as_raw(self),
                                presource.param().abi(),
                                penum.param().abi(),
                                pdesc,
                                ppvpiview.unwrap_or(core::mem::zeroed()) as _,
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn CreateVideoProcessorOutputView<P0, P1>(
                        &self,
                        presource: P0,
                        penum: P1,
                        pdesc: *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC,
                        ppvpoview: Option<*mut Option<ID3D11VideoProcessorOutputView>>,
                    ) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<ID3D11Resource>,
                        P1: windows_core::Param<ID3D11VideoProcessorEnumerator>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).CreateVideoProcessorOutputView)(
                                windows_core::Interface::as_raw(self),
                                presource.param().abi(),
                                penum.param().abi(),
                                pdesc,
                                ppvpoview.unwrap_or(core::mem::zeroed()) as _,
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn CreateVideoProcessorEnumerator(
                        &self,
                        pdesc: *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC,
                    ) -> windows_core::Result<ID3D11VideoProcessorEnumerator> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateVideoProcessorEnumerator)(
                                windows_core::Interface::as_raw(self),
                                pdesc,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID3D11VideoDevice_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    CreateVideoDecoder: usize,
                    pub CreateVideoProcessor: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    CreateAuthenticatedChannel: usize,
                    CreateCryptoSession: usize,
                    CreateVideoDecoderOutputView: usize,
                    pub CreateVideoProcessorInputView: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub CreateVideoProcessorOutputView: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub CreateVideoProcessorEnumerator: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const D3D11_VIDEO_PROCESSOR_CONTENT_DESC,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub GetVideoDecoderProfileCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
                    pub GetVideoDecoderProfile:
                        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID) -> windows_core::HRESULT,
                    pub CheckVideoDecoderFormat: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        super::Dxgi::Common::DXGI_FORMAT,
                        *mut windows_core::BOOL,
                    ) -> windows_core::HRESULT,
                    GetVideoDecoderConfigCount: usize,
                    GetVideoDecoderConfig: usize,
                    GetContentProtectionCaps: usize,
                    pub CheckCryptoKeyExchange: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *const windows_core::GUID,
                        u32,
                        *mut windows_core::GUID,
                    ) -> windows_core::HRESULT,
                    pub SetPrivateData: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        u32,
                        *const core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetPrivateDataInterface: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                unsafe impl Send for ID3D11VideoDevice {}
                unsafe impl Sync for ID3D11VideoDevice {}
                impl windows_core::RuntimeName for ID3D11VideoDevice {}
                windows_core::imp::define_interface!(
                    ID3D11VideoProcessor,
                    ID3D11VideoProcessor_Vtbl,
                    0x1d7b0652_185f_41c6_85ce_0c5be3d4ae6c
                );
                impl core::ops::Deref for ID3D11VideoProcessor {
                    type Target = ID3D11DeviceChild;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID3D11VideoProcessor, windows_core::IUnknown, ID3D11DeviceChild);
                impl ID3D11VideoProcessor {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID3D11VideoProcessor_Vtbl {
                    pub base__: ID3D11DeviceChild_Vtbl,
                    pub GetContentDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC),
                    GetRateConversionCaps: usize,
                }
                unsafe impl Send for ID3D11VideoProcessor {}
                unsafe impl Sync for ID3D11VideoProcessor {}
                impl windows_core::RuntimeName for ID3D11VideoProcessor {}
                windows_core::imp::define_interface!(
                    ID3D11VideoProcessorEnumerator,
                    ID3D11VideoProcessorEnumerator_Vtbl,
                    0x31627037_53ab_4200_9061_05faa9ab45f9
                );
                impl core::ops::Deref for ID3D11VideoProcessorEnumerator {
                    type Target = ID3D11DeviceChild;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID3D11VideoProcessorEnumerator, windows_core::IUnknown, ID3D11DeviceChild);
                impl ID3D11VideoProcessorEnumerator {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID3D11VideoProcessorEnumerator_Vtbl {
                    pub base__: ID3D11DeviceChild_Vtbl,
                    pub GetVideoProcessorContentDesc:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_CONTENT_DESC) -> windows_core::HRESULT,
                    pub CheckVideoProcessorFormat: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        super::Dxgi::Common::DXGI_FORMAT,
                        *mut u32,
                    ) -> windows_core::HRESULT,
                    GetVideoProcessorCaps: usize,
                    GetVideoProcessorRateConversionCaps: usize,
                    GetVideoProcessorCustomRate: usize,
                    GetVideoProcessorFilterRange: usize,
                }
                unsafe impl Send for ID3D11VideoProcessorEnumerator {}
                unsafe impl Sync for ID3D11VideoProcessorEnumerator {}
                impl windows_core::RuntimeName for ID3D11VideoProcessorEnumerator {}
                windows_core::imp::define_interface!(
                    ID3D11VideoProcessorInputView,
                    ID3D11VideoProcessorInputView_Vtbl,
                    0x11ec5a5f_51dc_4945_ab34_6e8c21300ea5
                );
                impl core::ops::Deref for ID3D11VideoProcessorInputView {
                    type Target = ID3D11View;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(
                    ID3D11VideoProcessorInputView,
                    windows_core::IUnknown,
                    ID3D11DeviceChild,
                    ID3D11View
                );
                impl ID3D11VideoProcessorInputView {
                    pub unsafe fn GetDesc(&self) -> D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
                            result__
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID3D11VideoProcessorInputView_Vtbl {
                    pub base__: ID3D11View_Vtbl,
                    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC),
                }
                unsafe impl Send for ID3D11VideoProcessorInputView {}
                unsafe impl Sync for ID3D11VideoProcessorInputView {}
                impl windows_core::RuntimeName for ID3D11VideoProcessorInputView {}
                windows_core::imp::define_interface!(
                    ID3D11VideoProcessorOutputView,
                    ID3D11VideoProcessorOutputView_Vtbl,
                    0xa048285e_25a9_4527_bd93_d68b68c44254
                );
                impl core::ops::Deref for ID3D11VideoProcessorOutputView {
                    type Target = ID3D11View;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(
                    ID3D11VideoProcessorOutputView,
                    windows_core::IUnknown,
                    ID3D11DeviceChild,
                    ID3D11View
                );
                impl ID3D11VideoProcessorOutputView {
                    pub unsafe fn GetDesc(&self) -> D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetDesc)(windows_core::Interface::as_raw(self), &mut result__);
                            result__
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID3D11VideoProcessorOutputView_Vtbl {
                    pub base__: ID3D11View_Vtbl,
                    pub GetDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC),
                }
                unsafe impl Send for ID3D11VideoProcessorOutputView {}
                unsafe impl Sync for ID3D11VideoProcessorOutputView {}
                impl windows_core::RuntimeName for ID3D11VideoProcessorOutputView {}
                windows_core::imp::define_interface!(ID3D11View, ID3D11View_Vtbl, 0x839d1216_bb2e_412b_b7f4_a9dbebe08ed1);
                impl core::ops::Deref for ID3D11View {
                    type Target = ID3D11DeviceChild;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(ID3D11View, windows_core::IUnknown, ID3D11DeviceChild);
                impl ID3D11View {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID3D11View_Vtbl {
                    pub base__: ID3D11DeviceChild_Vtbl,
                    pub GetResource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
                }
                unsafe impl Send for ID3D11View {}
                unsafe impl Sync for ID3D11View {}
                impl windows_core::RuntimeName for ID3D11View {}
            }
            pub mod DirectWrite {
                #[inline]
                pub unsafe fn DWriteCreateFactory<T>(factorytype: DWRITE_FACTORY_TYPE) -> windows_core::Result<T>
                where
                    T: windows_core::Interface,
                {
                    windows_core::link!("dwrite.dll" "system" fn DWriteCreateFactory(factorytype : DWRITE_FACTORY_TYPE, iid : *const windows_core::GUID, factory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
                    let mut result__ = core::ptr::null_mut();
                    unsafe { DWriteCreateFactory(factorytype, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct DWRITE_FACTORY_TYPE(pub i32);
                pub const DWRITE_FACTORY_TYPE_SHARED: DWRITE_FACTORY_TYPE = DWRITE_FACTORY_TYPE(0i32);
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct DWRITE_FONT_FEATURE {
                    pub nameTag: DWRITE_FONT_FEATURE_TAG,
                    pub parameter: u32,
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct DWRITE_FONT_FEATURE_TAG(pub u32);
                pub const DWRITE_FONT_FEATURE_TAG_TABULAR_FIGURES: DWRITE_FONT_FEATURE_TAG = DWRITE_FONT_FEATURE_TAG(1836412532u32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct DWRITE_FONT_STRETCH(pub i32);
                pub const DWRITE_FONT_STRETCH_NORMAL: DWRITE_FONT_STRETCH = DWRITE_FONT_STRETCH(5i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct DWRITE_FONT_STYLE(pub i32);
                pub const DWRITE_FONT_STYLE_NORMAL: DWRITE_FONT_STYLE = DWRITE_FONT_STYLE(0i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct DWRITE_FONT_WEIGHT(pub i32);
                pub const DWRITE_FONT_WEIGHT_BOLD: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(700i32);
                pub const DWRITE_FONT_WEIGHT_MEDIUM: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(500i32);
                pub const DWRITE_FONT_WEIGHT_NORMAL: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(400i32);
                pub const DWRITE_FONT_WEIGHT_SEMI_BOLD: DWRITE_FONT_WEIGHT = DWRITE_FONT_WEIGHT(600i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct DWRITE_PARAGRAPH_ALIGNMENT(pub i32);
                pub const DWRITE_PARAGRAPH_ALIGNMENT_CENTER: DWRITE_PARAGRAPH_ALIGNMENT = DWRITE_PARAGRAPH_ALIGNMENT(2i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct DWRITE_TEXT_ALIGNMENT(pub i32);
                pub const DWRITE_TEXT_ALIGNMENT_CENTER: DWRITE_TEXT_ALIGNMENT = DWRITE_TEXT_ALIGNMENT(2i32);
                pub const DWRITE_TEXT_ALIGNMENT_LEADING: DWRITE_TEXT_ALIGNMENT = DWRITE_TEXT_ALIGNMENT(0i32);
                pub const DWRITE_TEXT_ALIGNMENT_TRAILING: DWRITE_TEXT_ALIGNMENT = DWRITE_TEXT_ALIGNMENT(1i32);
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct DWRITE_TEXT_METRICS {
                    pub left: f32,
                    pub top: f32,
                    pub width: f32,
                    pub widthIncludingTrailingWhitespace: f32,
                    pub height: f32,
                    pub layoutWidth: f32,
                    pub layoutHeight: f32,
                    pub maxBidiReorderingDepth: u32,
                    pub lineCount: u32,
                }
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct DWRITE_TEXT_RANGE {
                    pub startPosition: u32,
                    pub length: u32,
                }
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct DWRITE_TRIMMING {
                    pub granularity: DWRITE_TRIMMING_GRANULARITY,
                    pub delimiter: u32,
                    pub delimiterCount: u32,
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct DWRITE_TRIMMING_GRANULARITY(pub i32);
                pub const DWRITE_TRIMMING_GRANULARITY_CHARACTER: DWRITE_TRIMMING_GRANULARITY = DWRITE_TRIMMING_GRANULARITY(1i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct DWRITE_WORD_WRAPPING(pub i32);
                pub const DWRITE_WORD_WRAPPING_NO_WRAP: DWRITE_WORD_WRAPPING = DWRITE_WORD_WRAPPING(1i32);
                windows_core::imp::define_interface!(IDWriteFactory, IDWriteFactory_Vtbl, 0xb859ee5a_d838_4b5b_a2e8_1adc7d93db48);
                windows_core::imp::interface_hierarchy!(IDWriteFactory, windows_core::IUnknown);
                impl IDWriteFactory {
                    pub unsafe fn CreateTextFormat<P0, P1, P6>(
                        &self,
                        fontfamilyname: P0,
                        fontcollection: P1,
                        fontweight: DWRITE_FONT_WEIGHT,
                        fontstyle: DWRITE_FONT_STYLE,
                        fontstretch: DWRITE_FONT_STRETCH,
                        fontsize: f32,
                        localename: P6,
                    ) -> windows_core::Result<IDWriteTextFormat>
                    where
                        P0: windows_core::Param<windows_core::PCWSTR>,
                        P1: windows_core::Param<IDWriteFontCollection>,
                        P6: windows_core::Param<windows_core::PCWSTR>,
                    {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateTextFormat)(
                                windows_core::Interface::as_raw(self),
                                fontfamilyname.param().abi(),
                                fontcollection.param().abi(),
                                fontweight,
                                fontstyle,
                                fontstretch,
                                fontsize,
                                localename.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn CreateTypography(&self) -> windows_core::Result<IDWriteTypography> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateTypography)(windows_core::Interface::as_raw(self), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn CreateTextLayout<P2>(
                        &self,
                        string: &[u16],
                        textformat: P2,
                        maxwidth: f32,
                        maxheight: f32,
                    ) -> windows_core::Result<IDWriteTextLayout>
                    where
                        P2: windows_core::Param<IDWriteTextFormat>,
                    {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateTextLayout)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute(string.as_ptr()),
                                string.len().try_into().unwrap(),
                                textformat.param().abi(),
                                maxwidth,
                                maxheight,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn CreateEllipsisTrimmingSign<P0>(&self, textformat: P0) -> windows_core::Result<IDWriteInlineObject>
                    where
                        P0: windows_core::Param<IDWriteTextFormat>,
                    {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateEllipsisTrimmingSign)(
                                windows_core::Interface::as_raw(self),
                                textformat.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IDWriteFactory_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub GetSystemFontCollection: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        windows_core::BOOL,
                    ) -> windows_core::HRESULT,
                    CreateCustomFontCollection: usize,
                    RegisterFontCollectionLoader: usize,
                    UnregisterFontCollectionLoader: usize,
                    CreateFontFileReference: usize,
                    CreateCustomFontFileReference: usize,
                    CreateFontFace: usize,
                    CreateRenderingParams: usize,
                    CreateMonitorRenderingParams: usize,
                    CreateCustomRenderingParams: usize,
                    RegisterFontFileLoader: usize,
                    UnregisterFontFileLoader: usize,
                    pub CreateTextFormat: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        windows_core::PCWSTR,
                        *mut core::ffi::c_void,
                        DWRITE_FONT_WEIGHT,
                        DWRITE_FONT_STYLE,
                        DWRITE_FONT_STRETCH,
                        f32,
                        windows_core::PCWSTR,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub CreateTypography:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetGdiInterop: usize,
                    pub CreateTextLayout: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        windows_core::PCWSTR,
                        u32,
                        *mut core::ffi::c_void,
                        f32,
                        f32,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    CreateGdiCompatibleTextLayout: usize,
                    pub CreateEllipsisTrimmingSign: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    CreateTextAnalyzer: usize,
                    CreateNumberSubstitution: usize,
                    CreateGlyphRunAnalysis: usize,
                }
                unsafe impl Send for IDWriteFactory {}
                unsafe impl Sync for IDWriteFactory {}
                impl windows_core::RuntimeName for IDWriteFactory {}
                windows_core::imp::define_interface!(
                    IDWriteFontCollection,
                    IDWriteFontCollection_Vtbl,
                    0xa84cee02_3eea_4eee_a827_87c1a02a0fcc
                );
                windows_core::imp::interface_hierarchy!(IDWriteFontCollection, windows_core::IUnknown);
                impl IDWriteFontCollection {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IDWriteFontCollection_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub GetFontFamilyCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
                    GetFontFamily: usize,
                    pub FindFamilyName: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        windows_core::PCWSTR,
                        *mut u32,
                        *mut windows_core::BOOL,
                    ) -> windows_core::HRESULT,
                    GetFontFromFontFace: usize,
                }
                unsafe impl Send for IDWriteFontCollection {}
                unsafe impl Sync for IDWriteFontCollection {}
                impl windows_core::RuntimeName for IDWriteFontCollection {}
                windows_core::imp::define_interface!(IDWriteInlineObject, IDWriteInlineObject_Vtbl, 0x8339fde3_106f_47ab_8373_1c6295eb10b3);
                windows_core::imp::interface_hierarchy!(IDWriteInlineObject, windows_core::IUnknown);
                #[repr(C)]
                #[doc(hidden)]
                pub struct IDWriteInlineObject_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    Draw: usize,
                    GetMetrics: usize,
                    GetOverhangMetrics: usize,
                    GetBreakConditions: usize,
                }
                unsafe impl Send for IDWriteInlineObject {}
                unsafe impl Sync for IDWriteInlineObject {}
                impl windows_core::RuntimeName for IDWriteInlineObject {}
                windows_core::imp::define_interface!(IDWriteTextFormat, IDWriteTextFormat_Vtbl, 0x9c906818_31d7_4fd3_a151_7c5e225db55a);
                windows_core::imp::interface_hierarchy!(IDWriteTextFormat, windows_core::IUnknown);
                impl IDWriteTextFormat {
                    pub unsafe fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetTextAlignment)(windows_core::Interface::as_raw(self), textalignment)
                                .ok()
                        }
                    }
                    pub unsafe fn SetParagraphAlignment(&self, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetParagraphAlignment)(
                                windows_core::Interface::as_raw(self),
                                paragraphalignment,
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetWordWrapping)(windows_core::Interface::as_raw(self), wordwrapping)
                                .ok()
                        }
                    }
                    pub unsafe fn SetTrimming<P1>(
                        &self,
                        trimmingoptions: *const DWRITE_TRIMMING,
                        trimmingsign: P1,
                    ) -> windows_core::Result<()>
                    where
                        P1: windows_core::Param<IDWriteInlineObject>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetTrimming)(
                                windows_core::Interface::as_raw(self),
                                trimmingoptions,
                                trimmingsign.param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IDWriteTextFormat_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub SetTextAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_TEXT_ALIGNMENT) -> windows_core::HRESULT,
                    pub SetParagraphAlignment:
                        unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_PARAGRAPH_ALIGNMENT) -> windows_core::HRESULT,
                    pub SetWordWrapping: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_WORD_WRAPPING) -> windows_core::HRESULT,
                    SetReadingDirection: usize,
                    SetFlowDirection: usize,
                    pub SetIncrementalTabStop: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
                    pub SetTrimming: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const DWRITE_TRIMMING,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    SetLineSpacing: usize,
                    pub GetTextAlignment: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_TEXT_ALIGNMENT,
                    pub GetParagraphAlignment: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_PARAGRAPH_ALIGNMENT,
                    pub GetWordWrapping: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_WORD_WRAPPING,
                    GetReadingDirection: usize,
                    GetFlowDirection: usize,
                    pub GetIncrementalTabStop: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
                    pub GetTrimming: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut DWRITE_TRIMMING,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    GetLineSpacing: usize,
                    pub GetFontCollection:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetFontFamilyNameLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
                    pub GetFontFamilyName:
                        unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
                    pub GetFontWeight: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_WEIGHT,
                    pub GetFontStyle: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_STYLE,
                    pub GetFontStretch: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_STRETCH,
                    pub GetFontSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
                    pub GetLocaleNameLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
                    pub GetLocaleName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
                }
                unsafe impl Send for IDWriteTextFormat {}
                unsafe impl Sync for IDWriteTextFormat {}
                impl windows_core::RuntimeName for IDWriteTextFormat {}
                windows_core::imp::define_interface!(IDWriteTextLayout, IDWriteTextLayout_Vtbl, 0x53737037_6d14_410b_9bfe_0b182bb70961);
                impl core::ops::Deref for IDWriteTextLayout {
                    type Target = IDWriteTextFormat;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(IDWriteTextLayout, windows_core::IUnknown, IDWriteTextFormat);
                impl IDWriteTextLayout {
                    pub unsafe fn SetMaxWidth(&self, maxwidth: f32) -> windows_core::Result<()> {
                        unsafe { (windows_core::Interface::vtable(self).SetMaxWidth)(windows_core::Interface::as_raw(self), maxwidth).ok() }
                    }
                    pub unsafe fn SetMaxHeight(&self, maxheight: f32) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetMaxHeight)(windows_core::Interface::as_raw(self), maxheight).ok()
                        }
                    }
                    pub unsafe fn SetFontWeight(
                        &self,
                        fontweight: DWRITE_FONT_WEIGHT,
                        textrange: DWRITE_TEXT_RANGE,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetFontWeight)(
                                windows_core::Interface::as_raw(self),
                                fontweight,
                                core::mem::transmute(textrange),
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn SetFontStyle(
                        &self,
                        fontstyle: DWRITE_FONT_STYLE,
                        textrange: DWRITE_TEXT_RANGE,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetFontStyle)(
                                windows_core::Interface::as_raw(self),
                                fontstyle,
                                core::mem::transmute(textrange),
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn SetFontStretch(
                        &self,
                        fontstretch: DWRITE_FONT_STRETCH,
                        textrange: DWRITE_TEXT_RANGE,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetFontStretch)(
                                windows_core::Interface::as_raw(self),
                                fontstretch,
                                core::mem::transmute(textrange),
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn SetFontSize(&self, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetFontSize)(
                                windows_core::Interface::as_raw(self),
                                fontsize,
                                core::mem::transmute(textrange),
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn SetDrawingEffect<P0>(&self, drawingeffect: P0, textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::IUnknown>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetDrawingEffect)(
                                windows_core::Interface::as_raw(self),
                                drawingeffect.param().abi(),
                                core::mem::transmute(textrange),
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn SetTypography<P0>(&self, typography: P0, textrange: DWRITE_TEXT_RANGE) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<IDWriteTypography>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetTypography)(
                                windows_core::Interface::as_raw(self),
                                typography.param().abi(),
                                core::mem::transmute(textrange),
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), textmetrics as _).ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IDWriteTextLayout_Vtbl {
                    pub base__: IDWriteTextFormat_Vtbl,
                    pub SetMaxWidth: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
                    pub SetMaxHeight: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
                    pub SetFontCollection: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        DWRITE_TEXT_RANGE,
                    ) -> windows_core::HRESULT,
                    pub SetFontFamilyName:
                        unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
                    pub SetFontWeight:
                        unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_WEIGHT, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
                    pub SetFontStyle:
                        unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_STYLE, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
                    pub SetFontStretch:
                        unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_STRETCH, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
                    pub SetFontSize: unsafe extern "system" fn(*mut core::ffi::c_void, f32, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
                    pub SetUnderline:
                        unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
                    pub SetStrikethrough:
                        unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
                    pub SetDrawingEffect: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        DWRITE_TEXT_RANGE,
                    ) -> windows_core::HRESULT,
                    pub SetInlineObject: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        DWRITE_TEXT_RANGE,
                    ) -> windows_core::HRESULT,
                    pub SetTypography: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        DWRITE_TEXT_RANGE,
                    ) -> windows_core::HRESULT,
                    pub SetLocaleName:
                        unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
                    pub GetMaxWidth: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
                    pub GetMaxHeight: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
                    pub GetFontCollection: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        *mut *mut core::ffi::c_void,
                        *mut DWRITE_TEXT_RANGE,
                    ) -> windows_core::HRESULT,
                    pub GetFontFamilyNameLength:
                        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
                    pub GetFontFamilyName: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        windows_core::PWSTR,
                        u32,
                        *mut DWRITE_TEXT_RANGE,
                    ) -> windows_core::HRESULT,
                    pub GetFontWeight: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        *mut DWRITE_FONT_WEIGHT,
                        *mut DWRITE_TEXT_RANGE,
                    ) -> windows_core::HRESULT,
                    pub GetFontStyle: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        *mut DWRITE_FONT_STYLE,
                        *mut DWRITE_TEXT_RANGE,
                    ) -> windows_core::HRESULT,
                    pub GetFontStretch: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        *mut DWRITE_FONT_STRETCH,
                        *mut DWRITE_TEXT_RANGE,
                    ) -> windows_core::HRESULT,
                    pub GetFontSize:
                        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
                    pub GetUnderline: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        *mut windows_core::BOOL,
                        *mut DWRITE_TEXT_RANGE,
                    ) -> windows_core::HRESULT,
                    pub GetStrikethrough: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        *mut windows_core::BOOL,
                        *mut DWRITE_TEXT_RANGE,
                    ) -> windows_core::HRESULT,
                    pub GetDrawingEffect: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        *mut *mut core::ffi::c_void,
                        *mut DWRITE_TEXT_RANGE,
                    ) -> windows_core::HRESULT,
                    pub GetInlineObject: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        *mut *mut core::ffi::c_void,
                        *mut DWRITE_TEXT_RANGE,
                    ) -> windows_core::HRESULT,
                    pub GetTypography: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        *mut *mut core::ffi::c_void,
                        *mut DWRITE_TEXT_RANGE,
                    ) -> windows_core::HRESULT,
                    pub GetLocaleNameLength:
                        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut DWRITE_TEXT_RANGE) -> windows_core::HRESULT,
                    pub GetLocaleName: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        windows_core::PWSTR,
                        u32,
                        *mut DWRITE_TEXT_RANGE,
                    ) -> windows_core::HRESULT,
                    Draw: usize,
                    GetLineMetrics: usize,
                    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_TEXT_METRICS) -> windows_core::HRESULT,
                    GetOverhangMetrics: usize,
                    GetClusterMetrics: usize,
                    pub DetermineMinWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
                    HitTestPoint: usize,
                    HitTestTextPosition: usize,
                    HitTestTextRange: usize,
                }
                unsafe impl Send for IDWriteTextLayout {}
                unsafe impl Sync for IDWriteTextLayout {}
                impl windows_core::RuntimeName for IDWriteTextLayout {}
                windows_core::imp::define_interface!(IDWriteTypography, IDWriteTypography_Vtbl, 0x55f1112b_1dc2_4b3c_9541_f46894ed85b6);
                windows_core::imp::interface_hierarchy!(IDWriteTypography, windows_core::IUnknown);
                impl IDWriteTypography {
                    pub unsafe fn AddFontFeature(&self, fontfeature: DWRITE_FONT_FEATURE) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).AddFontFeature)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute(fontfeature),
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IDWriteTypography_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub AddFontFeature: unsafe extern "system" fn(*mut core::ffi::c_void, DWRITE_FONT_FEATURE) -> windows_core::HRESULT,
                    pub GetFontFeatureCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
                    pub GetFontFeature:
                        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DWRITE_FONT_FEATURE) -> windows_core::HRESULT,
                }
                unsafe impl Send for IDWriteTypography {}
                unsafe impl Sync for IDWriteTypography {}
                impl windows_core::RuntimeName for IDWriteTypography {}
            }
            pub mod Dxgi {
                windows_core::imp::define_interface!(IDXGIAdapter, IDXGIAdapter_Vtbl, 0x2411e7e1_12ac_4ccf_bd14_9798e8534dc0);
                impl core::ops::Deref for IDXGIAdapter {
                    type Target = IDXGIObject;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(IDXGIAdapter, windows_core::IUnknown, IDXGIObject);
                impl IDXGIAdapter {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IDXGIAdapter_Vtbl {
                    pub base__: IDXGIObject_Vtbl,
                    EnumOutputs: usize,
                    GetDesc: usize,
                    pub CheckInterfaceSupport:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut i64) -> windows_core::HRESULT,
                }
                unsafe impl Send for IDXGIAdapter {}
                unsafe impl Sync for IDXGIAdapter {}
                impl windows_core::RuntimeName for IDXGIAdapter {}
                windows_core::imp::define_interface!(IDXGIDevice, IDXGIDevice_Vtbl, 0x54ec77fa_1377_44e6_8c32_88fd5f44c84c);
                impl core::ops::Deref for IDXGIDevice {
                    type Target = IDXGIObject;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(IDXGIDevice, windows_core::IUnknown, IDXGIObject);
                impl IDXGIDevice {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IDXGIDevice_Vtbl {
                    pub base__: IDXGIObject_Vtbl,
                    pub GetAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    CreateSurface: usize,
                    QueryResourceResidency: usize,
                    pub SetGPUThreadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
                    pub GetGPUThreadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                }
                unsafe impl Send for IDXGIDevice {}
                unsafe impl Sync for IDXGIDevice {}
                impl windows_core::RuntimeName for IDXGIDevice {}
                windows_core::imp::define_interface!(IDXGIObject, IDXGIObject_Vtbl, 0xaec22fb8_76f3_4639_9be0_28eb43a67a2e);
                windows_core::imp::interface_hierarchy!(IDXGIObject, windows_core::IUnknown);
                impl IDXGIObject {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IDXGIObject_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub SetPrivateData: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        u32,
                        *const core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetPrivateDataInterface: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub GetPrivateData: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *mut u32,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub GetParent: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                unsafe impl Send for IDXGIObject {}
                unsafe impl Sync for IDXGIObject {}
                impl windows_core::RuntimeName for IDXGIObject {}
                pub mod Common {
                    #[repr(transparent)]
                    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                    pub struct DXGI_FORMAT(pub i32);
                    pub const DXGI_FORMAT_B8G8R8A8_UNORM: DXGI_FORMAT = DXGI_FORMAT(87i32);
                    pub const DXGI_FORMAT_NV12: DXGI_FORMAT = DXGI_FORMAT(103i32);
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, Default, PartialEq)]
                    pub struct DXGI_RATIONAL {
                        pub Numerator: u32,
                        pub Denominator: u32,
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, Default, PartialEq)]
                    pub struct DXGI_SAMPLE_DESC {
                        pub Count: u32,
                        pub Quality: u32,
                    }
                }
            }
            pub mod Gdi {
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, PartialEq, Eq)]
                pub struct HBITMAP(pub *mut core::ffi::c_void);
                impl HBITMAP {
                    pub fn is_invalid(&self) -> bool {
                        self.0 == -1 as _ || self.0 == 0 as _
                    }
                }
                impl windows_core::Free for HBITMAP {
                    #[inline]
                    unsafe fn free(&mut self) {
                        if !self.is_invalid() {
                            windows_core::link!("gdi32.dll" "system" fn DeleteObject(ho : *mut core::ffi::c_void) -> i32);
                            unsafe {
                                DeleteObject(self.0);
                            }
                        }
                    }
                }
                impl Default for HBITMAP {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                impl windows_core::imp::CanInto<HGDIOBJ> for HBITMAP {}
                impl From<HBITMAP> for HGDIOBJ {
                    fn from(value: HBITMAP) -> Self {
                        Self(value.0)
                    }
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, PartialEq, Eq)]
                pub struct HDC(pub *mut core::ffi::c_void);
                impl HDC {
                    pub fn is_invalid(&self) -> bool {
                        self.0 == -1 as _ || self.0 == 0 as _
                    }
                }
                impl Default for HDC {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, PartialEq, Eq)]
                pub struct HGDIOBJ(pub *mut core::ffi::c_void);
                impl HGDIOBJ {
                    pub fn is_invalid(&self) -> bool {
                        self.0 == -1 as _ || self.0 == 0 as _
                    }
                }
                impl windows_core::Free for HGDIOBJ {
                    #[inline]
                    unsafe fn free(&mut self) {
                        if !self.is_invalid() {
                            windows_core::link!("gdi32.dll" "system" fn DeleteObject(ho : *mut core::ffi::c_void) -> i32);
                            unsafe {
                                DeleteObject(self.0);
                            }
                        }
                    }
                }
                impl Default for HGDIOBJ {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, PartialEq, Eq)]
                pub struct HMONITOR(pub *mut core::ffi::c_void);
                impl HMONITOR {
                    pub fn is_invalid(&self) -> bool {
                        self.0 == -1 as _ || self.0 == 0 as _
                    }
                }
                impl Default for HMONITOR {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, PartialEq, Eq)]
                pub struct HPALETTE(pub *mut core::ffi::c_void);
                impl HPALETTE {
                    pub fn is_invalid(&self) -> bool {
                        self.0 == -1 as _ || self.0 == 0 as _
                    }
                }
                impl windows_core::Free for HPALETTE {
                    #[inline]
                    unsafe fn free(&mut self) {
                        if !self.is_invalid() {
                            windows_core::link!("gdi32.dll" "system" fn DeleteObject(ho : *mut core::ffi::c_void) -> i32);
                            unsafe {
                                DeleteObject(self.0);
                            }
                        }
                    }
                }
                impl Default for HPALETTE {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                impl windows_core::imp::CanInto<HGDIOBJ> for HPALETTE {}
                impl From<HPALETTE> for HGDIOBJ {
                    fn from(value: HPALETTE) -> Self {
                        Self(value.0)
                    }
                }
            }
            pub mod Imaging {
                pub const CLSID_WICImagingFactory: windows_core::GUID =
                    windows_core::GUID::from_u128(0xcacaf262_9370_4615_a13b_9f5539da4c0a);
                pub const GUID_ContainerFormatPng: windows_core::GUID =
                    windows_core::GUID::from_u128(0x1b7cfaf4_713f_473c_bbcd_6137425faeaf);
                pub const GUID_WICPixelFormat32bppPBGRA: windows_core::GUID =
                    windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc910);
                windows_core::imp::define_interface!(IWICBitmap, IWICBitmap_Vtbl, 0x00000121_a8f2_4877_ba0a_fd2b6645fb94);
                impl core::ops::Deref for IWICBitmap {
                    type Target = IWICBitmapSource;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(IWICBitmap, windows_core::IUnknown, IWICBitmapSource);
                impl IWICBitmap {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IWICBitmap_Vtbl {
                    pub base__: IWICBitmapSource_Vtbl,
                    Lock: usize,
                    pub SetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetResolution: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IWICBitmap {}
                windows_core::imp::define_interface!(IWICBitmapDecoder, IWICBitmapDecoder_Vtbl, 0x9edde9e7_8dee_47ea_99df_e6faf2ed44bf);
                windows_core::imp::interface_hierarchy!(IWICBitmapDecoder, windows_core::IUnknown);
                impl IWICBitmapDecoder {
                    pub unsafe fn Initialize<P0>(&self, pistream: P0, cacheoptions: WICDecodeOptions) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::super::System::Com::IStream>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).Initialize)(
                                windows_core::Interface::as_raw(self),
                                pistream.param().abi(),
                                cacheoptions,
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn GetFrame(&self, index: u32) -> windows_core::Result<IWICBitmapFrameDecode> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetFrame)(windows_core::Interface::as_raw(self), index, &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IWICBitmapDecoder_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub QueryCapability:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub Initialize: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        WICDecodeOptions,
                    ) -> windows_core::HRESULT,
                    pub GetContainerFormat:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
                    GetDecoderInfo: usize,
                    pub CopyPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetMetadataQueryReader: usize,
                    pub GetPreview: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetColorContexts: usize,
                    pub GetThumbnail:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetFrameCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub GetFrame:
                        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IWICBitmapDecoder {}
                windows_core::imp::define_interface!(IWICBitmapEncoder, IWICBitmapEncoder_Vtbl, 0x00000103_a8f2_4877_ba0a_fd2b6645fb94);
                windows_core::imp::interface_hierarchy!(IWICBitmapEncoder, windows_core::IUnknown);
                impl IWICBitmapEncoder {
                    pub unsafe fn Initialize<P0>(&self, pistream: P0, cacheoption: WICBitmapEncoderCacheOption) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::super::System::Com::IStream>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).Initialize)(
                                windows_core::Interface::as_raw(self),
                                pistream.param().abi(),
                                cacheoption,
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn CreateNewFrame(
                        &self,
                        ppiframeencode: *mut Option<IWICBitmapFrameEncode>,
                        ppiencoderoptions: *mut Option<super::super::System::Com::StructuredStorage::IPropertyBag2>,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).CreateNewFrame)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute(ppiframeencode),
                                core::mem::transmute(ppiencoderoptions),
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
                        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok() }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IWICBitmapEncoder_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub Initialize: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        WICBitmapEncoderCacheOption,
                    ) -> windows_core::HRESULT,
                    pub GetContainerFormat:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
                    GetEncoderInfo: usize,
                    SetColorContexts: usize,
                    pub SetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetPreview: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CreateNewFrame: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetMetadataQueryWriter: usize,
                }
                impl windows_core::RuntimeName for IWICBitmapEncoder {}
                windows_core::imp::define_interface!(
                    IWICBitmapFrameDecode,
                    IWICBitmapFrameDecode_Vtbl,
                    0x3b16811b_6a43_4ec9_a813_3d930c13b940
                );
                impl core::ops::Deref for IWICBitmapFrameDecode {
                    type Target = IWICBitmapSource;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(IWICBitmapFrameDecode, windows_core::IUnknown, IWICBitmapSource);
                impl IWICBitmapFrameDecode {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IWICBitmapFrameDecode_Vtbl {
                    pub base__: IWICBitmapSource_Vtbl,
                    GetMetadataQueryReader: usize,
                    GetColorContexts: usize,
                    pub GetThumbnail:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IWICBitmapFrameDecode {}
                windows_core::imp::define_interface!(
                    IWICBitmapFrameEncode,
                    IWICBitmapFrameEncode_Vtbl,
                    0x00000105_a8f2_4877_ba0a_fd2b6645fb94
                );
                windows_core::imp::interface_hierarchy!(IWICBitmapFrameEncode, windows_core::IUnknown);
                impl IWICBitmapFrameEncode {
                    pub unsafe fn Initialize<P0>(&self, piencoderoptions: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::super::System::Com::StructuredStorage::IPropertyBag2>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).Initialize)(
                                windows_core::Interface::as_raw(self),
                                piencoderoptions.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn SetSize(&self, uiwidth: u32, uiheight: u32) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), uiwidth, uiheight).ok()
                        }
                    }
                    pub unsafe fn SetPixelFormat(&self, ppixelformat: *mut windows_core::GUID) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetPixelFormat)(windows_core::Interface::as_raw(self), ppixelformat as _)
                                .ok()
                        }
                    }
                    pub unsafe fn WritePixels(&self, linecount: u32, cbstride: u32, pbpixels: &[u8]) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).WritePixels)(
                                windows_core::Interface::as_raw(self),
                                linecount,
                                cbstride,
                                pbpixels.len().try_into().unwrap(),
                                core::mem::transmute(pbpixels.as_ptr()),
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn Commit(&self) -> windows_core::Result<()> {
                        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok() }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IWICBitmapFrameEncode_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
                    pub SetResolution: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
                    pub SetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
                    SetColorContexts: usize,
                    pub SetPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub WritePixels: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *const u8) -> windows_core::HRESULT,
                    pub WriteSource:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const WICRect) -> windows_core::HRESULT,
                    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetMetadataQueryWriter: usize,
                }
                impl windows_core::RuntimeName for IWICBitmapFrameEncode {}
                windows_core::imp::define_interface!(IWICBitmapScaler, IWICBitmapScaler_Vtbl, 0x00000302_a8f2_4877_ba0a_fd2b6645fb94);
                impl core::ops::Deref for IWICBitmapScaler {
                    type Target = IWICBitmapSource;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(IWICBitmapScaler, windows_core::IUnknown, IWICBitmapSource);
                impl IWICBitmapScaler {
                    pub unsafe fn Initialize<P0>(
                        &self,
                        pisource: P0,
                        uiwidth: u32,
                        uiheight: u32,
                        mode: WICBitmapInterpolationMode,
                    ) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<IWICBitmapSource>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).Initialize)(
                                windows_core::Interface::as_raw(self),
                                pisource.param().abi(),
                                uiwidth,
                                uiheight,
                                mode,
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IWICBitmapScaler_Vtbl {
                    pub base__: IWICBitmapSource_Vtbl,
                    pub Initialize: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        u32,
                        WICBitmapInterpolationMode,
                    ) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IWICBitmapScaler {}
                windows_core::imp::define_interface!(IWICBitmapSource, IWICBitmapSource_Vtbl, 0x00000120_a8f2_4877_ba0a_fd2b6645fb94);
                windows_core::imp::interface_hierarchy!(IWICBitmapSource, windows_core::IUnknown);
                impl IWICBitmapSource {
                    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).GetSize)(
                                windows_core::Interface::as_raw(self),
                                puiwidth as _,
                                puiheight as _,
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).CopyPixels)(
                                windows_core::Interface::as_raw(self),
                                prc,
                                cbstride,
                                pbbuffer.len().try_into().unwrap(),
                                core::mem::transmute(pbbuffer.as_ptr()),
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IWICBitmapSource_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
                    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
                    pub GetResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut f64) -> windows_core::HRESULT,
                    pub CopyPalette: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CopyPixels:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *const WICRect, u32, u32, *mut u8) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IWICBitmapSource {}
                windows_core::imp::define_interface!(IWICFormatConverter, IWICFormatConverter_Vtbl, 0x00000301_a8f2_4877_ba0a_fd2b6645fb94);
                impl core::ops::Deref for IWICFormatConverter {
                    type Target = IWICBitmapSource;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(IWICFormatConverter, windows_core::IUnknown, IWICBitmapSource);
                impl IWICFormatConverter {
                    pub unsafe fn Initialize<P0, P3>(
                        &self,
                        pisource: P0,
                        dstformat: *const windows_core::GUID,
                        dither: WICBitmapDitherType,
                        pipalette: P3,
                        alphathresholdpercent: f64,
                        palettetranslate: WICBitmapPaletteType,
                    ) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<IWICBitmapSource>,
                        P3: windows_core::Param<IWICPalette>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).Initialize)(
                                windows_core::Interface::as_raw(self),
                                pisource.param().abi(),
                                dstformat,
                                dither,
                                pipalette.param().abi(),
                                alphathresholdpercent,
                                palettetranslate,
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IWICFormatConverter_Vtbl {
                    pub base__: IWICBitmapSource_Vtbl,
                    pub Initialize: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        WICBitmapDitherType,
                        *mut core::ffi::c_void,
                        f64,
                        WICBitmapPaletteType,
                    ) -> windows_core::HRESULT,
                    pub CanConvert: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *const windows_core::GUID,
                        *mut windows_core::BOOL,
                    ) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IWICFormatConverter {}
                windows_core::imp::define_interface!(IWICImagingFactory, IWICImagingFactory_Vtbl, 0xec5ec8a9_c395_4314_9c77_54d7a935ff70);
                windows_core::imp::interface_hierarchy!(IWICImagingFactory, windows_core::IUnknown);
                impl IWICImagingFactory {
                    pub unsafe fn CreateDecoderFromStream<P0>(
                        &self,
                        pistream: P0,
                        pguidvendor: *const windows_core::GUID,
                        metadataoptions: WICDecodeOptions,
                    ) -> windows_core::Result<IWICBitmapDecoder>
                    where
                        P0: windows_core::Param<super::super::System::Com::IStream>,
                    {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateDecoderFromStream)(
                                windows_core::Interface::as_raw(self),
                                pistream.param().abi(),
                                pguidvendor,
                                metadataoptions,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn CreateEncoder(
                        &self,
                        guidcontainerformat: *const windows_core::GUID,
                        pguidvendor: *const windows_core::GUID,
                    ) -> windows_core::Result<IWICBitmapEncoder> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateEncoder)(
                                windows_core::Interface::as_raw(self),
                                guidcontainerformat,
                                pguidvendor,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn CreateFormatConverter(&self) -> windows_core::Result<IWICFormatConverter> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateFormatConverter)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn CreateBitmapScaler(&self) -> windows_core::Result<IWICBitmapScaler> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateBitmapScaler)(windows_core::Interface::as_raw(self), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn CreateStream(&self) -> windows_core::Result<IWICStream> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateStream)(windows_core::Interface::as_raw(self), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn CreateBitmapFromHBITMAP(
                        &self,
                        hbitmap: super::Gdi::HBITMAP,
                        hpalette: super::Gdi::HPALETTE,
                        options: WICBitmapAlphaChannelOption,
                    ) -> windows_core::Result<IWICBitmap> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).CreateBitmapFromHBITMAP)(
                                windows_core::Interface::as_raw(self),
                                hbitmap,
                                hpalette,
                                options,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IWICImagingFactory_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    CreateDecoderFromFilename: usize,
                    pub CreateDecoderFromStream: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        WICDecodeOptions,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub CreateDecoderFromFileHandle: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        usize,
                        *const windows_core::GUID,
                        WICDecodeOptions,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    CreateComponentInfo: usize,
                    pub CreateDecoder: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *const windows_core::GUID,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub CreateEncoder: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *const windows_core::GUID,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub CreatePalette:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CreateFormatConverter:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CreateBitmapScaler:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    CreateBitmapClipper: usize,
                    CreateBitmapFlipRotator: usize,
                    pub CreateStream:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    CreateColorContext: usize,
                    CreateColorTransformer: usize,
                    CreateBitmap: usize,
                    CreateBitmapFromSource: usize,
                    pub CreateBitmapFromSourceRect: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        u32,
                        u32,
                        u32,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub CreateBitmapFromMemory: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        u32,
                        *const windows_core::GUID,
                        u32,
                        u32,
                        *const u8,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub CreateBitmapFromHBITMAP: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        super::Gdi::HBITMAP,
                        super::Gdi::HPALETTE,
                        WICBitmapAlphaChannelOption,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    CreateBitmapFromHICON: usize,
                    CreateComponentEnumerator: usize,
                    CreateFastMetadataEncoderFromDecoder: usize,
                    CreateFastMetadataEncoderFromFrameDecode: usize,
                    CreateQueryWriter: usize,
                    CreateQueryWriterFromReader: usize,
                }
                impl windows_core::RuntimeName for IWICImagingFactory {}
                windows_core::imp::define_interface!(IWICPalette, IWICPalette_Vtbl, 0x00000040_a8f2_4877_ba0a_fd2b6645fb94);
                windows_core::imp::interface_hierarchy!(IWICPalette, windows_core::IUnknown);
                impl IWICPalette {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IWICPalette_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub InitializePredefined: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        WICBitmapPaletteType,
                        windows_core::BOOL,
                    ) -> windows_core::HRESULT,
                    pub InitializeCustom: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, u32) -> windows_core::HRESULT,
                    pub InitializeFromBitmap: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u32,
                        windows_core::BOOL,
                    ) -> windows_core::HRESULT,
                    pub InitializeFromPalette:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WICBitmapPaletteType) -> windows_core::HRESULT,
                    pub GetColorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub GetColors: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
                    pub IsBlackWhite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
                    pub IsGrayscale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
                    pub HasAlpha: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IWICPalette {}
                windows_core::imp::define_interface!(IWICStream, IWICStream_Vtbl, 0x135ff860_22b7_4ddf_b0f6_218f4f299a43);
                impl core::ops::Deref for IWICStream {
                    type Target = super::super::System::Com::IStream;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(
                    IWICStream,
                    windows_core::IUnknown,
                    super::super::System::Com::ISequentialStream,
                    super::super::System::Com::IStream
                );
                impl IWICStream {
                    pub unsafe fn InitializeFromFilename<P0>(&self, wzfilename: P0, dwdesiredaccess: u32) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::PCWSTR>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).InitializeFromFilename)(
                                windows_core::Interface::as_raw(self),
                                wzfilename.param().abi(),
                                dwdesiredaccess,
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn InitializeFromMemory(&self, pbbuffer: &[u8]) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).InitializeFromMemory)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute(pbbuffer.as_ptr()),
                                pbbuffer.len().try_into().unwrap(),
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IWICStream_Vtbl {
                    pub base__: super::super::System::Com::IStream_Vtbl,
                    pub InitializeFromIStream:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub InitializeFromFilename:
                        unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
                    pub InitializeFromMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
                    pub InitializeFromIStreamRegion:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, u64) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IWICStream {}
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct WICBitmapAlphaChannelOption(pub i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct WICBitmapDitherType(pub i32);
                pub const WICBitmapDitherTypeNone: WICBitmapDitherType = WICBitmapDitherType(0i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct WICBitmapEncoderCacheOption(pub i32);
                pub const WICBitmapEncoderNoCache: WICBitmapEncoderCacheOption = WICBitmapEncoderCacheOption(2i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct WICBitmapInterpolationMode(pub i32);
                pub const WICBitmapInterpolationModeFant: WICBitmapInterpolationMode = WICBitmapInterpolationMode(3i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct WICBitmapPaletteType(pub i32);
                pub const WICBitmapPaletteTypeCustom: WICBitmapPaletteType = WICBitmapPaletteType(0i32);
                pub const WICBitmapUseAlpha: WICBitmapAlphaChannelOption = WICBitmapAlphaChannelOption(0i32);
                pub const WICDecodeMetadataCacheOnDemand: WICDecodeOptions = WICDecodeOptions(0i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct WICDecodeOptions(pub i32);
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct WICRect {
                    pub X: i32,
                    pub Y: i32,
                    pub Width: i32,
                    pub Height: i32,
                }
            }
        }
        pub mod Media {
            pub mod Audio {
                pub const AUDCLNT_BUFFERFLAGS_SILENT: _AUDCLNT_BUFFERFLAGS = _AUDCLNT_BUFFERFLAGS(2i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct AUDCLNT_SHAREMODE(pub i32);
                pub const AUDCLNT_SHAREMODE_SHARED: AUDCLNT_SHAREMODE = AUDCLNT_SHAREMODE(0i32);
                pub const AUDCLNT_STREAMFLAGS_LOOPBACK: u32 = 131072u32;
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct DEVICE_STATE(pub u32);
                pub const DEVICE_STATEMASK_ALL: u32 = 15u32;
                pub const DEVICE_STATE_ACTIVE: DEVICE_STATE = DEVICE_STATE(1u32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct EDataFlow(pub i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct ERole(pub i32);
                windows_core::imp::define_interface!(IAudioCaptureClient, IAudioCaptureClient_Vtbl, 0xc8adbd64_e71e_48a0_a4de_185c395cd317);
                windows_core::imp::interface_hierarchy!(IAudioCaptureClient, windows_core::IUnknown);
                impl IAudioCaptureClient {
                    pub unsafe fn GetBuffer(
                        &self,
                        ppdata: *mut *mut u8,
                        pnumframestoread: *mut u32,
                        pdwflags: *mut u32,
                        pu64deviceposition: Option<*mut u64>,
                        pu64qpcposition: Option<*mut u64>,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).GetBuffer)(
                                windows_core::Interface::as_raw(self),
                                ppdata as _,
                                pnumframestoread as _,
                                pdwflags as _,
                                pu64deviceposition.unwrap_or(core::mem::zeroed()) as _,
                                pu64qpcposition.unwrap_or(core::mem::zeroed()) as _,
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn ReleaseBuffer(&self, numframesread: u32) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).ReleaseBuffer)(windows_core::Interface::as_raw(self), numframesread).ok()
                        }
                    }
                    pub unsafe fn GetNextPacketSize(&self) -> windows_core::Result<u32> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetNextPacketSize)(windows_core::Interface::as_raw(self), &mut result__)
                                .map(|| result__)
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IAudioCaptureClient_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub GetBuffer: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut *mut u8,
                        *mut u32,
                        *mut u32,
                        *mut u64,
                        *mut u64,
                    ) -> windows_core::HRESULT,
                    pub ReleaseBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
                    pub GetNextPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IAudioCaptureClient {}
                windows_core::imp::define_interface!(IAudioClient, IAudioClient_Vtbl, 0x1cb9ad4c_dbfa_4c32_b178_c2f568a703b2);
                windows_core::imp::interface_hierarchy!(IAudioClient, windows_core::IUnknown);
                impl IAudioClient {
                    pub unsafe fn Initialize(
                        &self,
                        sharemode: AUDCLNT_SHAREMODE,
                        streamflags: u32,
                        hnsbufferduration: i64,
                        hnsperiodicity: i64,
                        pformat: *const WAVEFORMATEX,
                        audiosessionguid: Option<*const windows_core::GUID>,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).Initialize)(
                                windows_core::Interface::as_raw(self),
                                sharemode,
                                streamflags,
                                hnsbufferduration,
                                hnsperiodicity,
                                pformat,
                                audiosessionguid.unwrap_or(core::mem::zeroed()) as _,
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn GetMixFormat(&self) -> windows_core::Result<*mut WAVEFORMATEX> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetMixFormat)(windows_core::Interface::as_raw(self), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub unsafe fn Start(&self) -> windows_core::Result<()> {
                        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)).ok() }
                    }
                    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
                        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok() }
                    }
                    pub unsafe fn GetService<T>(&self) -> windows_core::Result<T>
                    where
                        T: windows_core::Interface,
                    {
                        let mut result__ = core::ptr::null_mut();
                        unsafe {
                            (windows_core::Interface::vtable(self).GetService)(
                                windows_core::Interface::as_raw(self),
                                &T::IID,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IAudioClient_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub Initialize: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        AUDCLNT_SHAREMODE,
                        u32,
                        i64,
                        i64,
                        *const WAVEFORMATEX,
                        *const windows_core::GUID,
                    ) -> windows_core::HRESULT,
                    pub GetBufferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub GetStreamLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                    pub GetCurrentPadding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub IsFormatSupported: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        AUDCLNT_SHAREMODE,
                        *const WAVEFORMATEX,
                        *mut *mut WAVEFORMATEX,
                    ) -> windows_core::HRESULT,
                    pub GetMixFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut WAVEFORMATEX) -> windows_core::HRESULT,
                    pub GetDevicePeriod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64, *mut i64) -> windows_core::HRESULT,
                    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetEventHandle:
                        unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
                    pub GetService: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IAudioClient {}
                windows_core::imp::define_interface!(IConnector, IConnector_Vtbl, 0x9c2c4058_23f5_41de_877a_df3af236a09e);
                windows_core::imp::interface_hierarchy!(IConnector, windows_core::IUnknown);
                impl IConnector {
                    pub unsafe fn Disconnect(&self) -> windows_core::Result<()> {
                        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)).ok() }
                    }
                    pub unsafe fn GetConnectedTo(&self) -> windows_core::Result<IConnector> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetConnectedTo)(windows_core::Interface::as_raw(self), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IConnector_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    GetType: usize,
                    GetDataFlow: usize,
                    pub ConnectTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
                    pub GetConnectedTo:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetConnectorIdConnectedTo:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
                    pub GetDeviceIdConnectedTo:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IConnector {}
                windows_core::imp::define_interface!(IDeviceTopology, IDeviceTopology_Vtbl, 0x2a07407e_6497_4a18_9787_32f79bd0d98f);
                windows_core::imp::interface_hierarchy!(IDeviceTopology, windows_core::IUnknown);
                impl IDeviceTopology {
                    pub unsafe fn GetConnectorCount(&self) -> windows_core::Result<u32> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetConnectorCount)(windows_core::Interface::as_raw(self), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub unsafe fn GetConnector(&self, nindex: u32) -> windows_core::Result<IConnector> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetConnector)(
                                windows_core::Interface::as_raw(self),
                                nindex,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn GetDeviceId(&self) -> windows_core::Result<windows_core::PWSTR> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetDeviceId)(windows_core::Interface::as_raw(self), &mut result__)
                                .map(|| result__)
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IDeviceTopology_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub GetConnectorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub GetConnector:
                        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetSubunitCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    GetSubunit: usize,
                    pub GetPartById:
                        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
                    GetSignalPath: usize,
                }
                impl windows_core::RuntimeName for IDeviceTopology {}
                windows_core::imp::define_interface!(IMMDevice, IMMDevice_Vtbl, 0xd666063f_1587_4e43_81f1_b948e807363f);
                windows_core::imp::interface_hierarchy!(IMMDevice, windows_core::IUnknown);
                impl IMMDevice {
                    pub unsafe fn Activate<T>(
                        &self,
                        dwclsctx: super::super::System::Com::CLSCTX,
                        pactivationparams: Option<*const super::super::System::Com::StructuredStorage::PROPVARIANT>,
                    ) -> windows_core::Result<T>
                    where
                        T: windows_core::Interface,
                    {
                        let mut result__ = core::ptr::null_mut();
                        unsafe {
                            (windows_core::Interface::vtable(self).Activate)(
                                windows_core::Interface::as_raw(self),
                                &T::IID,
                                dwclsctx,
                                pactivationparams.unwrap_or(core::mem::zeroed()) as _,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn OpenPropertyStore(
                        &self,
                        stgmaccess: super::super::System::Com::STGM,
                    ) -> windows_core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).OpenPropertyStore)(
                                windows_core::Interface::as_raw(self),
                                stgmaccess,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn GetId(&self) -> windows_core::Result<windows_core::PWSTR> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub unsafe fn GetState(&self) -> windows_core::Result<DEVICE_STATE> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__)
                                .map(|| result__)
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IMMDevice_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub Activate: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        super::super::System::Com::CLSCTX,
                        *const super::super::System::Com::StructuredStorage::PROPVARIANT,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub OpenPropertyStore: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        super::super::System::Com::STGM,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
                    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DEVICE_STATE) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IMMDevice {}
                windows_core::imp::define_interface!(IMMDeviceCollection, IMMDeviceCollection_Vtbl, 0x0bd7a1be_7a1a_44db_8397_cc5392387b5e);
                windows_core::imp::interface_hierarchy!(IMMDeviceCollection, windows_core::IUnknown);
                impl IMMDeviceCollection {
                    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub unsafe fn Item(&self, ndevice: u32) -> windows_core::Result<IMMDevice> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), ndevice, &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IMMDeviceCollection_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IMMDeviceCollection {}
                windows_core::imp::define_interface!(IMMDeviceEnumerator, IMMDeviceEnumerator_Vtbl, 0xa95664d2_9614_4f35_a746_de8db63617e6);
                windows_core::imp::interface_hierarchy!(IMMDeviceEnumerator, windows_core::IUnknown);
                impl IMMDeviceEnumerator {
                    pub unsafe fn EnumAudioEndpoints(
                        &self,
                        dataflow: EDataFlow,
                        dwstatemask: DEVICE_STATE,
                    ) -> windows_core::Result<IMMDeviceCollection> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).EnumAudioEndpoints)(
                                windows_core::Interface::as_raw(self),
                                dataflow,
                                dwstatemask,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn GetDefaultAudioEndpoint(&self, dataflow: EDataFlow, role: ERole) -> windows_core::Result<IMMDevice> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetDefaultAudioEndpoint)(
                                windows_core::Interface::as_raw(self),
                                dataflow,
                                role,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn GetDevice<P0>(&self, pwstrid: P0) -> windows_core::Result<IMMDevice>
                    where
                        P0: windows_core::Param<windows_core::PCWSTR>,
                    {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetDevice)(
                                windows_core::Interface::as_raw(self),
                                pwstrid.param().abi(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn RegisterEndpointNotificationCallback<P0>(&self, pclient: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<IMMNotificationClient>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).RegisterEndpointNotificationCallback)(
                                windows_core::Interface::as_raw(self),
                                pclient.param().abi(),
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IMMDeviceEnumerator_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub EnumAudioEndpoints: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        EDataFlow,
                        DEVICE_STATE,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub GetDefaultAudioEndpoint: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        EDataFlow,
                        ERole,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub GetDevice: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        windows_core::PCWSTR,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub RegisterEndpointNotificationCallback:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub UnregisterEndpointNotificationCallback:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IMMDeviceEnumerator {}
                windows_core::imp::define_interface!(
                    IMMNotificationClient,
                    IMMNotificationClient_Vtbl,
                    0x7991eec9_7e89_4d85_8390_6c703cec60c0
                );
                windows_core::imp::interface_hierarchy!(IMMNotificationClient, windows_core::IUnknown);
                impl IMMNotificationClient {
                    pub unsafe fn OnDeviceStateChanged<P0>(&self, pwstrdeviceid: P0, dwnewstate: DEVICE_STATE) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::PCWSTR>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).OnDeviceStateChanged)(
                                windows_core::Interface::as_raw(self),
                                pwstrdeviceid.param().abi(),
                                dwnewstate,
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn OnDeviceAdded<P0>(&self, pwstrdeviceid: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::PCWSTR>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).OnDeviceAdded)(
                                windows_core::Interface::as_raw(self),
                                pwstrdeviceid.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn OnDeviceRemoved<P0>(&self, pwstrdeviceid: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::PCWSTR>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).OnDeviceRemoved)(
                                windows_core::Interface::as_raw(self),
                                pwstrdeviceid.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn OnDefaultDeviceChanged<P2>(
                        &self,
                        flow: EDataFlow,
                        role: ERole,
                        pwstrdefaultdeviceid: P2,
                    ) -> windows_core::Result<()>
                    where
                        P2: windows_core::Param<windows_core::PCWSTR>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).OnDefaultDeviceChanged)(
                                windows_core::Interface::as_raw(self),
                                flow,
                                role,
                                pwstrdefaultdeviceid.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn OnPropertyValueChanged<P0>(
                        &self,
                        pwstrdeviceid: P0,
                        key: super::super::Foundation::PROPERTYKEY,
                    ) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::PCWSTR>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).OnPropertyValueChanged)(
                                windows_core::Interface::as_raw(self),
                                pwstrdeviceid.param().abi(),
                                core::mem::transmute(key),
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IMMNotificationClient_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub OnDeviceStateChanged:
                        unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, DEVICE_STATE) -> windows_core::HRESULT,
                    pub OnDeviceAdded: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
                    pub OnDeviceRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
                    pub OnDefaultDeviceChanged:
                        unsafe extern "system" fn(*mut core::ffi::c_void, EDataFlow, ERole, windows_core::PCWSTR) -> windows_core::HRESULT,
                    pub OnPropertyValueChanged: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        windows_core::PCWSTR,
                        super::super::Foundation::PROPERTYKEY,
                    ) -> windows_core::HRESULT,
                }
                pub trait IMMNotificationClient_Impl: windows_core::IUnknownImpl {
                    fn OnDeviceStateChanged(
                        &self,
                        pwstrdeviceid: &windows_core::PCWSTR,
                        dwnewstate: DEVICE_STATE,
                    ) -> windows_core::Result<()>;
                    fn OnDeviceAdded(&self, pwstrdeviceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
                    fn OnDeviceRemoved(&self, pwstrdeviceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
                    fn OnDefaultDeviceChanged(
                        &self,
                        flow: EDataFlow,
                        role: ERole,
                        pwstrdefaultdeviceid: &windows_core::PCWSTR,
                    ) -> windows_core::Result<()>;
                    fn OnPropertyValueChanged(
                        &self,
                        pwstrdeviceid: &windows_core::PCWSTR,
                        key: &super::super::Foundation::PROPERTYKEY,
                    ) -> windows_core::Result<()>;
                }
                impl IMMNotificationClient_Vtbl {
                    pub const fn new<Identity: IMMNotificationClient_Impl, const OFFSET: isize>() -> Self {
                        unsafe extern "system" fn OnDeviceStateChanged<Identity: IMMNotificationClient_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            pwstrdeviceid: windows_core::PCWSTR,
                            dwnewstate: DEVICE_STATE,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                IMMNotificationClient_Impl::OnDeviceStateChanged(
                                    this,
                                    core::mem::transmute(&pwstrdeviceid),
                                    core::mem::transmute_copy(&dwnewstate),
                                )
                                .into()
                            }
                        }
                        unsafe extern "system" fn OnDeviceAdded<Identity: IMMNotificationClient_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            pwstrdeviceid: windows_core::PCWSTR,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                IMMNotificationClient_Impl::OnDeviceAdded(this, core::mem::transmute(&pwstrdeviceid)).into()
                            }
                        }
                        unsafe extern "system" fn OnDeviceRemoved<Identity: IMMNotificationClient_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            pwstrdeviceid: windows_core::PCWSTR,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                IMMNotificationClient_Impl::OnDeviceRemoved(this, core::mem::transmute(&pwstrdeviceid)).into()
                            }
                        }
                        unsafe extern "system" fn OnDefaultDeviceChanged<Identity: IMMNotificationClient_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            flow: EDataFlow,
                            role: ERole,
                            pwstrdefaultdeviceid: windows_core::PCWSTR,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                IMMNotificationClient_Impl::OnDefaultDeviceChanged(
                                    this,
                                    core::mem::transmute_copy(&flow),
                                    core::mem::transmute_copy(&role),
                                    core::mem::transmute(&pwstrdefaultdeviceid),
                                )
                                .into()
                            }
                        }
                        unsafe extern "system" fn OnPropertyValueChanged<Identity: IMMNotificationClient_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            pwstrdeviceid: windows_core::PCWSTR,
                            key: super::super::Foundation::PROPERTYKEY,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                IMMNotificationClient_Impl::OnPropertyValueChanged(
                                    this,
                                    core::mem::transmute(&pwstrdeviceid),
                                    core::mem::transmute(&key),
                                )
                                .into()
                            }
                        }
                        Self {
                            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
                            OnDeviceStateChanged: OnDeviceStateChanged::<Identity, OFFSET>,
                            OnDeviceAdded: OnDeviceAdded::<Identity, OFFSET>,
                            OnDeviceRemoved: OnDeviceRemoved::<Identity, OFFSET>,
                            OnDefaultDeviceChanged: OnDefaultDeviceChanged::<Identity, OFFSET>,
                            OnPropertyValueChanged: OnPropertyValueChanged::<Identity, OFFSET>,
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<IMMNotificationClient as windows_core::Interface>::IID
                    }
                }
                impl windows_core::RuntimeName for IMMNotificationClient {}
                windows_core::imp::define_interface!(IPart, IPart_Vtbl, 0xae2de0e4_5bca_4f2d_aa46_5d13f8fdb3a9);
                windows_core::imp::interface_hierarchy!(IPart, windows_core::IUnknown);
                impl IPart {
                    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub unsafe fn GetTopologyObject(&self) -> windows_core::Result<IDeviceTopology> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetTopologyObject)(windows_core::Interface::as_raw(self), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub unsafe fn Activate(
                        &self,
                        dwclscontext: u32,
                        refiid: *const windows_core::GUID,
                        ppvobject: Option<*mut *mut core::ffi::c_void>,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).Activate)(
                                windows_core::Interface::as_raw(self),
                                dwclscontext,
                                refiid,
                                ppvobject.unwrap_or(core::mem::zeroed()) as _,
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IPart_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
                    pub GetLocalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub GetGlobalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
                    GetPartType: usize,
                    pub GetSubType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
                    pub GetControlInterfaceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    GetControlInterface: usize,
                    EnumPartsIncoming: usize,
                    EnumPartsOutgoing: usize,
                    pub GetTopologyObject:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub Activate: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        *const windows_core::GUID,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    RegisterControlChangeCallback: usize,
                    UnregisterControlChangeCallback: usize,
                }
                impl windows_core::RuntimeName for IPart {}
                pub const MMDeviceEnumerator: windows_core::GUID = windows_core::GUID::from_u128(0xbcde0395_e52f_467c_8e3d_c4579291692e);
                #[repr(C, packed(1))]
                #[derive(Clone, Copy, Default)]
                pub struct WAVEFORMATEX {
                    pub wFormatTag: u16,
                    pub nChannels: u16,
                    pub nSamplesPerSec: u32,
                    pub nAvgBytesPerSec: u32,
                    pub nBlockAlign: u16,
                    pub wBitsPerSample: u16,
                    pub cbSize: u16,
                }
                #[repr(C, packed(1))]
                #[derive(Clone, Copy)]
                pub struct WAVEFORMATEXTENSIBLE {
                    pub Format: WAVEFORMATEX,
                    pub Samples: WAVEFORMATEXTENSIBLE_0,
                    pub dwChannelMask: u32,
                    pub SubFormat: windows_core::GUID,
                }
                impl Default for WAVEFORMATEXTENSIBLE {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C, packed(1))]
                #[derive(Clone, Copy)]
                pub union WAVEFORMATEXTENSIBLE_0 {
                    pub wValidBitsPerSample: u16,
                    pub wSamplesPerBlock: u16,
                    pub wReserved: u16,
                }
                impl Default for WAVEFORMATEXTENSIBLE_0 {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct _AUDCLNT_BUFFERFLAGS(pub i32);
                pub const eConsole: ERole = ERole(0i32);
                pub const eRender: EDataFlow = EDataFlow(0i32);
            }
            pub mod KernelStreaming {
                windows_core::imp::define_interface!(IKsControl, IKsControl_Vtbl, 0x28f54685_06fd_11d2_b27a_00a0c9223196);
                windows_core::imp::interface_hierarchy!(IKsControl, windows_core::IUnknown);
                impl IKsControl {
                    pub unsafe fn KsProperty(
                        &self,
                        property: *const KSIDENTIFIER,
                        propertylength: u32,
                        propertydata: *mut core::ffi::c_void,
                        datalength: u32,
                        bytesreturned: *mut u32,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).KsProperty)(
                                windows_core::Interface::as_raw(self),
                                property,
                                propertylength,
                                propertydata as _,
                                datalength,
                                bytesreturned as _,
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IKsControl_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub KsProperty: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const KSIDENTIFIER,
                        u32,
                        *mut core::ffi::c_void,
                        u32,
                        *mut u32,
                    ) -> windows_core::HRESULT,
                    pub KsMethod: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const KSIDENTIFIER,
                        u32,
                        *mut core::ffi::c_void,
                        u32,
                        *mut u32,
                    ) -> windows_core::HRESULT,
                    pub KsEvent: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const KSIDENTIFIER,
                        u32,
                        *mut core::ffi::c_void,
                        u32,
                        *mut u32,
                    ) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IKsControl {}
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct KSIDENTIFIER {
                    pub Anonymous: KSIDENTIFIER_0,
                }
                impl Default for KSIDENTIFIER {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub union KSIDENTIFIER_0 {
                    pub Anonymous: KSIDENTIFIER_0_0,
                    pub Alignment: i64,
                }
                impl Default for KSIDENTIFIER_0 {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct KSIDENTIFIER_0_0 {
                    pub Set: windows_core::GUID,
                    pub Id: u32,
                    pub Flags: u32,
                }
            }
            pub mod MediaFoundation {
                #[inline]
                pub unsafe fn MFCreateAttributes(
                    ppmfattributes: *mut Option<IMFAttributes>,
                    cinitialsize: u32,
                ) -> windows_core::Result<()> {
                    windows_core::link!("mfplat.dll" "system" fn MFCreateAttributes(ppmfattributes : *mut * mut core::ffi::c_void, cinitialsize : u32) -> windows_core::HRESULT);
                    unsafe { MFCreateAttributes(core::mem::transmute(ppmfattributes), cinitialsize).ok() }
                }
                #[inline]
                pub unsafe fn MFCreateDXGIDeviceManager(
                    resettoken: *mut u32,
                    ppdevicemanager: *mut Option<IMFDXGIDeviceManager>,
                ) -> windows_core::Result<()> {
                    windows_core::link!("mfplat.dll" "system" fn MFCreateDXGIDeviceManager(resettoken : *mut u32, ppdevicemanager : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
                    unsafe { MFCreateDXGIDeviceManager(resettoken as _, core::mem::transmute(ppdevicemanager)).ok() }
                }
                #[inline]
                pub unsafe fn MFCreateDXGISurfaceBuffer<P1>(
                    riid: *const windows_core::GUID,
                    punksurface: P1,
                    usubresourceindex: u32,
                    fbottomupwhenlinear: bool,
                ) -> windows_core::Result<IMFMediaBuffer>
                where
                    P1: windows_core::Param<windows_core::IUnknown>,
                {
                    windows_core::link!("mfplat.dll" "system" fn MFCreateDXGISurfaceBuffer(riid : *const windows_core::GUID, punksurface : * mut core::ffi::c_void, usubresourceindex : u32, fbottomupwhenlinear : windows_core::BOOL, ppbuffer : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        MFCreateDXGISurfaceBuffer(
                            riid,
                            punksurface.param().abi(),
                            usubresourceindex,
                            fbottomupwhenlinear.into(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                #[inline]
                pub unsafe fn MFCreateMediaType() -> windows_core::Result<IMFMediaType> {
                    windows_core::link!("mfplat.dll" "system" fn MFCreateMediaType(ppmftype : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        MFCreateMediaType(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                #[inline]
                pub unsafe fn MFCreateSample() -> windows_core::Result<IMFSample> {
                    windows_core::link!("mfplat.dll" "system" fn MFCreateSample(ppimfsample : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        MFCreateSample(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                #[inline]
                pub unsafe fn MFCreateSinkWriterFromURL<P0, P1, P2>(
                    pwszoutputurl: P0,
                    pbytestream: P1,
                    pattributes: P2,
                ) -> windows_core::Result<IMFSinkWriter>
                where
                    P0: windows_core::Param<windows_core::PCWSTR>,
                    P1: windows_core::Param<IMFByteStream>,
                    P2: windows_core::Param<IMFAttributes>,
                {
                    windows_core::link!("mfreadwrite.dll" "system" fn MFCreateSinkWriterFromURL(pwszoutputurl : windows_core::PCWSTR, pbytestream : * mut core::ffi::c_void, pattributes : * mut core::ffi::c_void, ppsinkwriter : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        MFCreateSinkWriterFromURL(
                            pwszoutputurl.param().abi(),
                            pbytestream.param().abi(),
                            pattributes.param().abi(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                #[inline]
                pub unsafe fn MFShutdown() -> windows_core::Result<()> {
                    windows_core::link!("mfplat.dll" "system" fn MFShutdown() -> windows_core::HRESULT);
                    unsafe { MFShutdown().ok() }
                }
                #[inline]
                pub unsafe fn MFStartup(version: u32, dwflags: u32) -> windows_core::Result<()> {
                    windows_core::link!("mfplat.dll" "system" fn MFStartup(version : u32, dwflags : u32) -> windows_core::HRESULT);
                    unsafe { MFStartup(version, dwflags).ok() }
                }
                windows_core::imp::define_interface!(IMF2DBuffer, IMF2DBuffer_Vtbl, 0x7dc9d5f9_9ed9_44ec_9bbf_0600bb589fbb);
                windows_core::imp::interface_hierarchy!(IMF2DBuffer, windows_core::IUnknown);
                impl IMF2DBuffer {
                    pub unsafe fn GetContiguousLength(&self) -> windows_core::Result<u32> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetContiguousLength)(
                                windows_core::Interface::as_raw(self),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IMF2DBuffer_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub Lock2D: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut i32) -> windows_core::HRESULT,
                    pub Unlock2D: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetScanline0AndPitch:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut i32) -> windows_core::HRESULT,
                    pub IsContiguousFormat:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
                    pub GetContiguousLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub ContiguousCopyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32) -> windows_core::HRESULT,
                    pub ContiguousCopyFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IMF2DBuffer {}
                windows_core::imp::define_interface!(IMFAttributes, IMFAttributes_Vtbl, 0x2cd2d921_c447_44a7_a13c_4adabfc247e3);
                windows_core::imp::interface_hierarchy!(IMFAttributes, windows_core::IUnknown);
                impl IMFAttributes {
                    pub unsafe fn GetDouble(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<f64> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetDouble)(windows_core::Interface::as_raw(self), guidkey, &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub unsafe fn GetString(
                        &self,
                        guidkey: *const windows_core::GUID,
                        pwszvalue: &mut [u16],
                        pcchlength: Option<*mut u32>,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).GetString)(
                                windows_core::Interface::as_raw(self),
                                guidkey,
                                core::mem::transmute(pwszvalue.as_ptr()),
                                pwszvalue.len().try_into().unwrap(),
                                pcchlength.unwrap_or(core::mem::zeroed()) as _,
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn SetUINT32(&self, guidkey: *const windows_core::GUID, unvalue: u32) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetUINT32)(windows_core::Interface::as_raw(self), guidkey, unvalue).ok()
                        }
                    }
                    pub unsafe fn SetUINT64(&self, guidkey: *const windows_core::GUID, unvalue: u64) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetUINT64)(windows_core::Interface::as_raw(self), guidkey, unvalue).ok()
                        }
                    }
                    pub unsafe fn SetGUID(
                        &self,
                        guidkey: *const windows_core::GUID,
                        guidvalue: *const windows_core::GUID,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetGUID)(windows_core::Interface::as_raw(self), guidkey, guidvalue).ok()
                        }
                    }
                    pub unsafe fn SetUnknown<P1>(&self, guidkey: *const windows_core::GUID, punknown: P1) -> windows_core::Result<()>
                    where
                        P1: windows_core::Param<windows_core::IUnknown>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetUnknown)(
                                windows_core::Interface::as_raw(self),
                                guidkey,
                                punknown.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__)
                                .map(|| result__)
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IMFAttributes_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub GetItem: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *mut super::super::System::Com::StructuredStorage::PROPVARIANT,
                    ) -> windows_core::HRESULT,
                    GetItemType: usize,
                    pub CompareItem: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *const super::super::System::Com::StructuredStorage::PROPVARIANT,
                        *mut windows_core::BOOL,
                    ) -> windows_core::HRESULT,
                    Compare: usize,
                    pub GetUINT32:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
                    pub GetUINT64:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u64) -> windows_core::HRESULT,
                    pub GetDouble:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut f64) -> windows_core::HRESULT,
                    pub GetGUID: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *mut windows_core::GUID,
                    ) -> windows_core::HRESULT,
                    pub GetStringLength:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
                    pub GetString: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        windows_core::PWSTR,
                        u32,
                        *mut u32,
                    ) -> windows_core::HRESULT,
                    pub GetAllocatedString: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *mut windows_core::PWSTR,
                        *mut u32,
                    ) -> windows_core::HRESULT,
                    pub GetBlobSize:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
                    pub GetBlob: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *mut u8,
                        u32,
                        *mut u32,
                    ) -> windows_core::HRESULT,
                    pub GetAllocatedBlob: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *mut *mut u8,
                        *mut u32,
                    ) -> windows_core::HRESULT,
                    pub GetUnknown: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *const windows_core::GUID,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetItem: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *const super::super::System::Com::StructuredStorage::PROPVARIANT,
                    ) -> windows_core::HRESULT,
                    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
                    pub DeleteAllItems: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetUINT32:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32) -> windows_core::HRESULT,
                    pub SetUINT64:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u64) -> windows_core::HRESULT,
                    pub SetDouble:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, f64) -> windows_core::HRESULT,
                    pub SetGUID: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *const windows_core::GUID,
                    ) -> windows_core::HRESULT,
                    pub SetString: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        windows_core::PCWSTR,
                    ) -> windows_core::HRESULT,
                    pub SetBlob: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *const u8,
                        u32,
                    ) -> windows_core::HRESULT,
                    pub SetUnknown: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub LockStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub UnlockStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub GetItemByIndex: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        *mut windows_core::GUID,
                        *mut super::super::System::Com::StructuredStorage::PROPVARIANT,
                    ) -> windows_core::HRESULT,
                    pub CopyAllItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IMFAttributes {}
                windows_core::imp::define_interface!(IMFByteStream, IMFByteStream_Vtbl, 0xad4c1b00_4bf7_422f_9175_756693d9130d);
                windows_core::imp::interface_hierarchy!(IMFByteStream, windows_core::IUnknown);
                impl IMFByteStream {
                    pub unsafe fn Read(&self, pb: &mut [u8], pcbread: *mut u32) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).Read)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute(pb.as_ptr()),
                                pb.len().try_into().unwrap(),
                                pcbread as _,
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn Write(&self, pb: &[u8]) -> windows_core::Result<u32> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Write)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute(pb.as_ptr()),
                                pb.len().try_into().unwrap(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub unsafe fn Close(&self) -> windows_core::Result<()> {
                        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IMFByteStream_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub GetLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
                    pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
                    pub GetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
                    pub SetCurrentPosition: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
                    pub IsEndOfStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
                    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
                    BeginRead: usize,
                    EndRead: usize,
                    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u32) -> windows_core::HRESULT,
                    BeginWrite: usize,
                    EndWrite: usize,
                    Seek: usize,
                    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IMFByteStream {}
                windows_core::imp::define_interface!(
                    IMFDXGIDeviceManager,
                    IMFDXGIDeviceManager_Vtbl,
                    0xeb533d5d_2db6_40f8_97a9_494692014f07
                );
                windows_core::imp::interface_hierarchy!(IMFDXGIDeviceManager, windows_core::IUnknown);
                impl IMFDXGIDeviceManager {
                    pub unsafe fn ResetDevice<P0>(&self, punkdevice: P0, resettoken: u32) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::IUnknown>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).ResetDevice)(
                                windows_core::Interface::as_raw(self),
                                punkdevice.param().abi(),
                                resettoken,
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IMFDXGIDeviceManager_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub CloseDeviceHandle:
                        unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
                    pub GetVideoService: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        super::super::Foundation::HANDLE,
                        *const windows_core::GUID,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub LockDevice: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        super::super::Foundation::HANDLE,
                        *const windows_core::GUID,
                        *mut *mut core::ffi::c_void,
                        windows_core::BOOL,
                    ) -> windows_core::HRESULT,
                    pub OpenDeviceHandle:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
                    pub ResetDevice:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
                    pub TestDevice:
                        unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
                    pub UnlockDevice: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        super::super::Foundation::HANDLE,
                        windows_core::BOOL,
                    ) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IMFDXGIDeviceManager {}
                windows_core::imp::define_interface!(IMFMediaBuffer, IMFMediaBuffer_Vtbl, 0x045fa593_8799_42b8_bc8d_8968c6453507);
                windows_core::imp::interface_hierarchy!(IMFMediaBuffer, windows_core::IUnknown);
                impl IMFMediaBuffer {
                    pub unsafe fn Lock(
                        &self,
                        ppbbuffer: *mut *mut u8,
                        pcbmaxlength: Option<*mut u32>,
                        pcbcurrentlength: Option<*mut u32>,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).Lock)(
                                windows_core::Interface::as_raw(self),
                                ppbbuffer as _,
                                pcbmaxlength.unwrap_or(core::mem::zeroed()) as _,
                                pcbcurrentlength.unwrap_or(core::mem::zeroed()) as _,
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn SetCurrentLength(&self, cbcurrentlength: u32) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetCurrentLength)(windows_core::Interface::as_raw(self), cbcurrentlength)
                                .ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IMFMediaBuffer_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32, *mut u32) -> windows_core::HRESULT,
                    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetCurrentLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub SetCurrentLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
                    pub GetMaxLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IMFMediaBuffer {}
                windows_core::imp::define_interface!(IMFMediaType, IMFMediaType_Vtbl, 0x44ae0fa8_ea31_4109_8d2e_4cae4997c555);
                impl core::ops::Deref for IMFMediaType {
                    type Target = IMFAttributes;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(IMFMediaType, windows_core::IUnknown, IMFAttributes);
                impl IMFMediaType {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IMFMediaType_Vtbl {
                    pub base__: IMFAttributes_Vtbl,
                    pub GetMajorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
                    pub IsCompressedFormat:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
                    pub IsEqual:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub GetRepresentation: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        windows_core::GUID,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub FreeRepresentation: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        windows_core::GUID,
                        *const core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IMFMediaType {}
                windows_core::imp::define_interface!(IMFSample, IMFSample_Vtbl, 0xc40a00f2_b93a_4d80_ae8c_5a1c634f58e4);
                impl core::ops::Deref for IMFSample {
                    type Target = IMFAttributes;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(IMFSample, windows_core::IUnknown, IMFAttributes);
                impl IMFSample {
                    pub unsafe fn SetSampleTime(&self, hnssampletime: i64) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetSampleTime)(windows_core::Interface::as_raw(self), hnssampletime).ok()
                        }
                    }
                    pub unsafe fn SetSampleDuration(&self, hnssampleduration: i64) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetSampleDuration)(
                                windows_core::Interface::as_raw(self),
                                hnssampleduration,
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn AddBuffer<P0>(&self, pbuffer: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<IMFMediaBuffer>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).AddBuffer)(windows_core::Interface::as_raw(self), pbuffer.param().abi())
                                .ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IMFSample_Vtbl {
                    pub base__: IMFAttributes_Vtbl,
                    pub GetSampleFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub SetSampleFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
                    pub GetSampleTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                    pub SetSampleTime: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    pub GetSampleDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                    pub SetSampleDuration: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    pub GetBufferCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub GetBufferByIndex:
                        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ConvertToContiguousBuffer:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub AddBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub RemoveBufferByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
                    pub RemoveAllBuffers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetTotalLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub CopyToBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IMFSample {}
                windows_core::imp::define_interface!(IMFSinkWriter, IMFSinkWriter_Vtbl, 0x3137f1cd_fe5e_4805_a5d8_fb477448cb3d);
                windows_core::imp::interface_hierarchy!(IMFSinkWriter, windows_core::IUnknown);
                impl IMFSinkWriter {
                    pub unsafe fn AddStream<P0>(&self, ptargetmediatype: P0) -> windows_core::Result<u32>
                    where
                        P0: windows_core::Param<IMFMediaType>,
                    {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).AddStream)(
                                windows_core::Interface::as_raw(self),
                                ptargetmediatype.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub unsafe fn SetInputMediaType<P1, P2>(
                        &self,
                        dwstreamindex: u32,
                        pinputmediatype: P1,
                        pencodingparameters: P2,
                    ) -> windows_core::Result<()>
                    where
                        P1: windows_core::Param<IMFMediaType>,
                        P2: windows_core::Param<IMFAttributes>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).SetInputMediaType)(
                                windows_core::Interface::as_raw(self),
                                dwstreamindex,
                                pinputmediatype.param().abi(),
                                pencodingparameters.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn BeginWriting(&self) -> windows_core::Result<()> {
                        unsafe { (windows_core::Interface::vtable(self).BeginWriting)(windows_core::Interface::as_raw(self)).ok() }
                    }
                    pub unsafe fn WriteSample<P1>(&self, dwstreamindex: u32, psample: P1) -> windows_core::Result<()>
                    where
                        P1: windows_core::Param<IMFSample>,
                    {
                        unsafe {
                            (windows_core::Interface::vtable(self).WriteSample)(
                                windows_core::Interface::as_raw(self),
                                dwstreamindex,
                                psample.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub unsafe fn Finalize(&self) -> windows_core::Result<()> {
                        unsafe { (windows_core::Interface::vtable(self).Finalize)(windows_core::Interface::as_raw(self)).ok() }
                    }
                    pub unsafe fn GetStatistics(
                        &self,
                        dwstreamindex: u32,
                        pstats: *mut MF_SINK_WRITER_STATISTICS,
                    ) -> windows_core::Result<()> {
                        unsafe {
                            (windows_core::Interface::vtable(self).GetStatistics)(
                                windows_core::Interface::as_raw(self),
                                dwstreamindex,
                                pstats as _,
                            )
                            .ok()
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IMFSinkWriter_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub AddStream:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    pub SetInputMediaType: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub BeginWriting: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub WriteSample:
                        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SendStreamTick: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i64) -> windows_core::HRESULT,
                    pub PlaceMarker:
                        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
                    pub NotifyEndOfSegment: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
                    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
                    pub Finalize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetServiceForStream: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        u32,
                        *const windows_core::GUID,
                        *const windows_core::GUID,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub GetStatistics:
                        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MF_SINK_WRITER_STATISTICS) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IMFSinkWriter {}
                pub const MFMediaType_Video: windows_core::GUID = windows_core::GUID::from_u128(0x73646976_0000_0010_8000_00aa00389b71);
                pub const MFSTARTUP_LITE: u32 = 1u32;
                pub const MFTranscodeContainerType_MPEG4: windows_core::GUID =
                    windows_core::GUID::from_u128(0xdc6cd05d_b9d0_40ef_bd35_fa622c1ab28a);
                pub const MFVideoFormat_ARGB32: windows_core::GUID = windows_core::GUID::from_u128(0x00000015_0000_0010_8000_00aa00389b71);
                pub const MFVideoFormat_H264: windows_core::GUID = windows_core::GUID::from_u128(0x34363248_0000_0010_8000_00aa00389b71);
                pub const MFVideoFormat_NV12: windows_core::GUID = windows_core::GUID::from_u128(0x3231564e_0000_0010_8000_00aa00389b71);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct MFVideoInterlaceMode(pub i32);
                pub const MFVideoInterlace_Progressive: MFVideoInterlaceMode = MFVideoInterlaceMode(2i32);
                pub const MF_LOW_LATENCY: windows_core::GUID = windows_core::GUID::from_u128(0x9c27891a_ed7a_40e1_88e8_b22727a024ee);
                pub const MF_MT_AVG_BITRATE: windows_core::GUID = windows_core::GUID::from_u128(0x20332624_fb0d_4d9e_bd0d_cbf6786c102e);
                pub const MF_MT_FRAME_RATE: windows_core::GUID = windows_core::GUID::from_u128(0xc459a2e8_3d2c_4e44_b132_fee5156c7bb0);
                pub const MF_MT_FRAME_SIZE: windows_core::GUID = windows_core::GUID::from_u128(0x1652c33d_d6b2_4012_b834_72030849a37d);
                pub const MF_MT_INTERLACE_MODE: windows_core::GUID = windows_core::GUID::from_u128(0xe2724bb8_e676_4806_b4b2_a8d6efb44ccd);
                pub const MF_MT_MAJOR_TYPE: windows_core::GUID = windows_core::GUID::from_u128(0x48eba18e_f8c9_4687_bf11_0a74c9f96a8f);
                pub const MF_MT_MPEG2_PROFILE: windows_core::GUID = windows_core::GUID::from_u128(0xad76a80b_2d5c_4e0b_b375_64e520137036);
                pub const MF_MT_PIXEL_ASPECT_RATIO: windows_core::GUID =
                    windows_core::GUID::from_u128(0xc6376a1e_8d0a_4027_be45_6d9a0ad39bb6);
                pub const MF_MT_SUBTYPE: windows_core::GUID = windows_core::GUID::from_u128(0xf7e34c9a_42e8_4714_b74b_cb29d72c35e5);
                pub const MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS: windows_core::GUID =
                    windows_core::GUID::from_u128(0xa634a91c_822b_41b9_a494_4de4643612b0);
                pub const MF_SINK_WRITER_D3D_MANAGER: windows_core::GUID =
                    windows_core::GUID::from_u128(0xec822da2_e1e9_4b29_a0d8_563c719f5269);
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct MF_SINK_WRITER_STATISTICS {
                    pub cb: u32,
                    pub llLastTimestampReceived: i64,
                    pub llLastTimestampEncoded: i64,
                    pub llLastTimestampProcessed: i64,
                    pub llLastStreamTickReceived: i64,
                    pub llLastSinkSampleRequest: i64,
                    pub qwNumSamplesReceived: u64,
                    pub qwNumSamplesEncoded: u64,
                    pub qwNumSamplesProcessed: u64,
                    pub qwNumStreamTicksReceived: u64,
                    pub dwByteCountQueued: u32,
                    pub qwByteCountProcessed: u64,
                    pub dwNumOutstandingSinkSampleRequests: u32,
                    pub dwAverageSampleRateReceived: u32,
                    pub dwAverageSampleRateEncoded: u32,
                    pub dwAverageSampleRateProcessed: u32,
                }
                pub const MF_TRANSCODE_CONTAINERTYPE: windows_core::GUID =
                    windows_core::GUID::from_u128(0x150ff23f_4abc_478b_ac4f_e1916fba1cca);
                pub const MF_VERSION: u32 = 131184u32;
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct eAVEncH264VProfile(pub i32);
                pub const eAVEncH264VProfile_Main: eAVEncH264VProfile = eAVEncH264VProfile(77i32);
            }
        }
        pub mod System {
            pub mod Com {
                #[inline]
                pub unsafe fn CoCreateInstance<P1, T>(
                    rclsid: *const windows_core::GUID,
                    punkouter: P1,
                    dwclscontext: CLSCTX,
                ) -> windows_core::Result<T>
                where
                    P1: windows_core::Param<windows_core::IUnknown>,
                    T: windows_core::Interface,
                {
                    windows_core::link!("ole32.dll" "system" fn CoCreateInstance(rclsid : *const windows_core::GUID, punkouter : * mut core::ffi::c_void, dwclscontext : CLSCTX, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
                    let mut result__ = core::ptr::null_mut();
                    unsafe {
                        CoCreateInstance(rclsid, punkouter.param().abi(), dwclscontext, &T::IID, &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                #[inline]
                pub unsafe fn CoInitializeEx(pvreserved: Option<*const core::ffi::c_void>, dwcoinit: COINIT) -> windows_core::HRESULT {
                    windows_core::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : u32) -> windows_core::HRESULT);
                    unsafe { CoInitializeEx(pvreserved.unwrap_or(core::mem::zeroed()) as _, dwcoinit.0 as _) }
                }
                #[inline]
                pub unsafe fn CoTaskMemFree(pv: Option<*const core::ffi::c_void>) {
                    windows_core::link!("combase.dll" "system" fn CoTaskMemFree(pv : *const core::ffi::c_void));
                    unsafe { CoTaskMemFree(pv.unwrap_or(core::mem::zeroed()) as _) }
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct ADVANCED_FEATURE_FLAGS(pub u16);
                impl ADVANCED_FEATURE_FLAGS {
                    pub const fn contains(&self, other: Self) -> bool {
                        self.0 & other.0 == other.0
                    }
                }
                impl core::ops::BitOr for ADVANCED_FEATURE_FLAGS {
                    type Output = Self;
                    fn bitor(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }
                impl core::ops::BitAnd for ADVANCED_FEATURE_FLAGS {
                    type Output = Self;
                    fn bitand(self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
                impl core::ops::BitOrAssign for ADVANCED_FEATURE_FLAGS {
                    fn bitor_assign(&mut self, other: Self) {
                        self.0.bitor_assign(other.0)
                    }
                }
                impl core::ops::BitAndAssign for ADVANCED_FEATURE_FLAGS {
                    fn bitand_assign(&mut self, other: Self) {
                        self.0.bitand_assign(other.0)
                    }
                }
                impl core::ops::Not for ADVANCED_FEATURE_FLAGS {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct BLOB {
                    pub cbSize: u32,
                    pub pBlobData: *mut u8,
                }
                impl Default for BLOB {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct CLSCTX(pub u32);
                impl CLSCTX {
                    pub const fn contains(&self, other: Self) -> bool {
                        self.0 & other.0 == other.0
                    }
                }
                impl core::ops::BitOr for CLSCTX {
                    type Output = Self;
                    fn bitor(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }
                impl core::ops::BitAnd for CLSCTX {
                    type Output = Self;
                    fn bitand(self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
                impl core::ops::BitOrAssign for CLSCTX {
                    fn bitor_assign(&mut self, other: Self) {
                        self.0.bitor_assign(other.0)
                    }
                }
                impl core::ops::BitAndAssign for CLSCTX {
                    fn bitand_assign(&mut self, other: Self) {
                        self.0.bitand_assign(other.0)
                    }
                }
                impl core::ops::Not for CLSCTX {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                pub const CLSCTX_ALL: CLSCTX = CLSCTX(23u32);
                pub const CLSCTX_INPROC_SERVER: CLSCTX = CLSCTX(1u32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct COINIT(pub i32);
                impl COINIT {
                    pub const fn contains(&self, other: Self) -> bool {
                        self.0 & other.0 == other.0
                    }
                }
                impl core::ops::BitOr for COINIT {
                    type Output = Self;
                    fn bitor(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }
                impl core::ops::BitAnd for COINIT {
                    type Output = Self;
                    fn bitand(self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
                impl core::ops::BitOrAssign for COINIT {
                    fn bitor_assign(&mut self, other: Self) {
                        self.0.bitor_assign(other.0)
                    }
                }
                impl core::ops::BitAndAssign for COINIT {
                    fn bitand_assign(&mut self, other: Self) {
                        self.0.bitand_assign(other.0)
                    }
                }
                impl core::ops::Not for COINIT {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                pub const COINIT_APARTMENTTHREADED: COINIT = COINIT(2i32);
                pub const COINIT_MULTITHREADED: COINIT = COINIT(0i32);
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub union CY {
                    pub Anonymous: CY_0,
                    pub int64: i64,
                }
                impl Default for CY {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct CY_0 {
                    pub Lo: u32,
                    pub Hi: i32,
                }
                windows_core::imp::define_interface!(IBindCtx, IBindCtx_Vtbl, 0x0000000e_0000_0000_c000_000000000046);
                windows_core::imp::interface_hierarchy!(IBindCtx, windows_core::IUnknown);
                impl IBindCtx {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBindCtx_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub RegisterObjectBound:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub RevokeObjectBound:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ReleaseBoundObjects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    SetBindOptions: usize,
                    GetBindOptions: usize,
                    GetRunningObjectTable: usize,
                    pub RegisterObjectParam: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        windows_core::PCWSTR,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub GetObjectParam: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        windows_core::PCWSTR,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    EnumObjectParam: usize,
                    pub RevokeObjectParam: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IBindCtx {}
                windows_core::imp::define_interface!(IDispatch, IDispatch_Vtbl, 0x00020400_0000_0000_c000_000000000046);
                windows_core::imp::interface_hierarchy!(IDispatch, windows_core::IUnknown);
                impl IDispatch {
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IDispatch_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub GetTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                    GetTypeInfo: usize,
                    pub GetIDsOfNames: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *const windows_core::GUID,
                        *const windows_core::PCWSTR,
                        u32,
                        u32,
                        *mut i32,
                    ) -> windows_core::HRESULT,
                    Invoke: usize,
                }
                impl windows_core::RuntimeName for IDispatch {}
                windows_core::imp::define_interface!(ISequentialStream, ISequentialStream_Vtbl, 0x0c733a30_2a1c_11ce_ade5_00aa0044773d);
                windows_core::imp::interface_hierarchy!(ISequentialStream, windows_core::IUnknown);
                impl ISequentialStream {
                    pub unsafe fn Read(&self, pv: *mut core::ffi::c_void, cb: u32, pcbread: Option<*mut u32>) -> windows_core::HRESULT {
                        unsafe {
                            (windows_core::Interface::vtable(self).Read)(
                                windows_core::Interface::as_raw(self),
                                pv as _,
                                cb,
                                pcbread.unwrap_or(core::mem::zeroed()) as _,
                            )
                        }
                    }
                    pub unsafe fn Write(
                        &self,
                        pv: *const core::ffi::c_void,
                        cb: u32,
                        pcbwritten: Option<*mut u32>,
                    ) -> windows_core::HRESULT {
                        unsafe {
                            (windows_core::Interface::vtable(self).Write)(
                                windows_core::Interface::as_raw(self),
                                pv,
                                cb,
                                pcbwritten.unwrap_or(core::mem::zeroed()) as _,
                            )
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ISequentialStream_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub Read:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
                    pub Write:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for ISequentialStream {}
                windows_core::imp::define_interface!(IStream, IStream_Vtbl, 0x0000000c_0000_0000_c000_000000000046);
                impl core::ops::Deref for IStream {
                    type Target = ISequentialStream;
                    fn deref(&self) -> &Self::Target {
                        unsafe { core::mem::transmute(self) }
                    }
                }
                windows_core::imp::interface_hierarchy!(IStream, windows_core::IUnknown, ISequentialStream);
                impl IStream {
                    pub unsafe fn SetSize(&self, libnewsize: u64) -> windows_core::Result<()> {
                        unsafe { (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), libnewsize).ok() }
                    }
                    pub unsafe fn Clone(&self) -> windows_core::Result<IStream> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IStream_Vtbl {
                    pub base__: ISequentialStream_Vtbl,
                    Seek: usize,
                    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
                    pub CopyTo: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        u64,
                        *mut u64,
                        *mut u64,
                    ) -> windows_core::HRESULT,
                    Commit: usize,
                    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    LockRegion: usize,
                    pub UnlockRegion: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32) -> windows_core::HRESULT,
                    Stat: usize,
                    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IStream {}
                #[repr(C)]
                #[derive(Clone, Copy, Debug, PartialEq)]
                pub struct SAFEARRAY {
                    pub cDims: u16,
                    pub fFeatures: ADVANCED_FEATURE_FLAGS,
                    pub cbElements: u32,
                    pub cLocks: u32,
                    pub pvData: *mut core::ffi::c_void,
                    pub rgsabound: [SAFEARRAYBOUND; 1],
                }
                impl Default for SAFEARRAY {
                    fn default() -> Self {
                        unsafe { core::mem::zeroed() }
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy, Debug, Default, PartialEq)]
                pub struct SAFEARRAYBOUND {
                    pub cElements: u32,
                    pub lLbound: i32,
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct STGM(pub u32);
                impl STGM {
                    pub const fn contains(&self, other: Self) -> bool {
                        self.0 & other.0 == other.0
                    }
                }
                impl core::ops::BitOr for STGM {
                    type Output = Self;
                    fn bitor(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }
                impl core::ops::BitAnd for STGM {
                    type Output = Self;
                    fn bitand(self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
                impl core::ops::BitOrAssign for STGM {
                    fn bitor_assign(&mut self, other: Self) {
                        self.0.bitor_assign(other.0)
                    }
                }
                impl core::ops::BitAndAssign for STGM {
                    fn bitand_assign(&mut self, other: Self) {
                        self.0.bitand_assign(other.0)
                    }
                }
                impl core::ops::Not for STGM {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                pub const STGM_READ: STGM = STGM(0u32);
                pub mod StructuredStorage {
                    #[inline]
                    pub unsafe fn PropVariantClear(pvar: *mut PROPVARIANT) -> windows_core::Result<()> {
                        windows_core::link!("ole32.dll" "system" fn PropVariantClear(pvar : *mut PROPVARIANT) -> windows_core::HRESULT);
                        unsafe { PropVariantClear(core::mem::transmute(pvar)).ok() }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct BSTRBLOB {
                        pub cbSize: u32,
                        pub pData: *mut u8,
                    }
                    impl Default for BSTRBLOB {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CABOOL {
                        pub cElems: u32,
                        pub pElems: *mut super::super::super::Foundation::VARIANT_BOOL,
                    }
                    impl Default for CABOOL {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CABSTR {
                        pub cElems: u32,
                        pub pElems: *mut windows_core::BSTR,
                    }
                    impl Default for CABSTR {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CABSTRBLOB {
                        pub cElems: u32,
                        pub pElems: *mut BSTRBLOB,
                    }
                    impl Default for CABSTRBLOB {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, Default, PartialEq)]
                    pub struct CAC {
                        pub cElems: u32,
                        pub pElems: windows_core::PSTR,
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CACLIPDATA {
                        pub cElems: u32,
                        pub pElems: *mut CLIPDATA,
                    }
                    impl Default for CACLIPDATA {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CACLSID {
                        pub cElems: u32,
                        pub pElems: *mut windows_core::GUID,
                    }
                    impl Default for CACLSID {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CACY {
                        pub cElems: u32,
                        pub pElems: *mut super::CY,
                    }
                    impl Default for CACY {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CADATE {
                        pub cElems: u32,
                        pub pElems: *mut f64,
                    }
                    impl Default for CADATE {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CADBL {
                        pub cElems: u32,
                        pub pElems: *mut f64,
                    }
                    impl Default for CADBL {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CAFILETIME {
                        pub cElems: u32,
                        pub pElems: *mut super::super::super::Foundation::FILETIME,
                    }
                    impl Default for CAFILETIME {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CAFLT {
                        pub cElems: u32,
                        pub pElems: *mut f32,
                    }
                    impl Default for CAFLT {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CAH {
                        pub cElems: u32,
                        pub pElems: *mut i64,
                    }
                    impl Default for CAH {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CAI {
                        pub cElems: u32,
                        pub pElems: *mut i16,
                    }
                    impl Default for CAI {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CAL {
                        pub cElems: u32,
                        pub pElems: *mut i32,
                    }
                    impl Default for CAL {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CALPSTR {
                        pub cElems: u32,
                        pub pElems: *mut windows_core::PSTR,
                    }
                    impl Default for CALPSTR {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CALPWSTR {
                        pub cElems: u32,
                        pub pElems: *mut windows_core::PWSTR,
                    }
                    impl Default for CALPWSTR {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CAPROPVARIANT {
                        pub cElems: u32,
                        pub pElems: *mut PROPVARIANT,
                    }
                    impl Default for CAPROPVARIANT {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CASCODE {
                        pub cElems: u32,
                        pub pElems: *mut i32,
                    }
                    impl Default for CASCODE {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CAUB {
                        pub cElems: u32,
                        pub pElems: *mut u8,
                    }
                    impl Default for CAUB {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CAUH {
                        pub cElems: u32,
                        pub pElems: *mut u64,
                    }
                    impl Default for CAUH {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CAUI {
                        pub cElems: u32,
                        pub pElems: *mut u16,
                    }
                    impl Default for CAUI {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CAUL {
                        pub cElems: u32,
                        pub pElems: *mut u32,
                    }
                    impl Default for CAUL {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Copy, Debug, PartialEq)]
                    pub struct CLIPDATA {
                        pub cbSize: u32,
                        pub ulClipFmt: i32,
                        pub pClipData: *mut u8,
                    }
                    impl Default for CLIPDATA {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    windows_core::imp::define_interface!(IPropertyBag2, IPropertyBag2_Vtbl, 0x22f55882_280b_11d0_a8a9_00a0c90c2004);
                    windows_core::imp::interface_hierarchy!(IPropertyBag2, windows_core::IUnknown);
                    impl IPropertyBag2 {
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct IPropertyBag2_Vtbl {
                        pub base__: windows_core::IUnknown_Vtbl,
                        Read: usize,
                        Write: usize,
                        pub CountProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                        GetPropertyInfo: usize,
                        LoadObject: usize,
                    }
                    impl windows_core::RuntimeName for IPropertyBag2 {}
                    windows_core::imp::define_interface!(IStorage, IStorage_Vtbl, 0x0000000b_0000_0000_c000_000000000046);
                    windows_core::imp::interface_hierarchy!(IStorage, windows_core::IUnknown);
                    impl IStorage {
                        pub unsafe fn CreateStream<P0>(
                            &self,
                            pwcsname: P0,
                            grfmode: super::STGM,
                            reserved1: u32,
                            reserved2: u32,
                        ) -> windows_core::Result<super::IStream>
                        where
                            P0: windows_core::Param<windows_core::PCWSTR>,
                        {
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(self).CreateStream)(
                                    windows_core::Interface::as_raw(self),
                                    pwcsname.param().abi(),
                                    grfmode,
                                    reserved1,
                                    reserved2,
                                    &mut result__,
                                )
                                .and_then(|| windows_core::Type::from_abi(result__))
                            }
                        }
                        pub unsafe fn Commit(&self, grfcommitflags: u32) -> windows_core::Result<()> {
                            unsafe {
                                (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), grfcommitflags).ok()
                            }
                        }
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct IStorage_Vtbl {
                        pub base__: windows_core::IUnknown_Vtbl,
                        pub CreateStream: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            windows_core::PCWSTR,
                            super::STGM,
                            u32,
                            u32,
                            *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT,
                        pub OpenStream: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            windows_core::PCWSTR,
                            *const core::ffi::c_void,
                            super::STGM,
                            u32,
                            *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT,
                        pub CreateStorage: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            windows_core::PCWSTR,
                            super::STGM,
                            u32,
                            u32,
                            *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT,
                        pub OpenStorage: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            windows_core::PCWSTR,
                            *mut core::ffi::c_void,
                            super::STGM,
                            *const *const u16,
                            u32,
                            *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT,
                        pub CopyTo: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            u32,
                            *const windows_core::GUID,
                            *const *const u16,
                            *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT,
                        MoveElementTo: usize,
                        pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
                        pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                        EnumElements: usize,
                        pub DestroyElement:
                            unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
                        pub RenameElement: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            windows_core::PCWSTR,
                            windows_core::PCWSTR,
                        ) -> windows_core::HRESULT,
                        pub SetElementTimes: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            windows_core::PCWSTR,
                            *const super::super::super::Foundation::FILETIME,
                            *const super::super::super::Foundation::FILETIME,
                            *const super::super::super::Foundation::FILETIME,
                        ) -> windows_core::HRESULT,
                        pub SetClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
                        pub SetStateBits: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
                        Stat: usize,
                    }
                    impl windows_core::RuntimeName for IStorage {}
                    #[repr(C)]
                    pub struct PROPVARIANT {
                        pub Anonymous: PROPVARIANT_0,
                    }
                    impl Default for PROPVARIANT {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    pub union PROPVARIANT_0 {
                        pub Anonymous: core::mem::ManuallyDrop<PROPVARIANT_0_0>,
                        pub decVal: super::super::super::Foundation::DECIMAL,
                    }
                    impl Clone for PROPVARIANT_0 {
                        fn clone(&self) -> Self {
                            unsafe { core::mem::transmute_copy(self) }
                        }
                    }
                    impl Default for PROPVARIANT_0 {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    pub struct PROPVARIANT_0_0 {
                        pub vt: super::super::Variant::VARENUM,
                        pub wReserved1: u16,
                        pub wReserved2: u16,
                        pub wReserved3: u16,
                        pub Anonymous: PROPVARIANT_0_0_0,
                    }
                    impl Clone for PROPVARIANT_0_0 {
                        fn clone(&self) -> Self {
                            unsafe { core::mem::transmute_copy(self) }
                        }
                    }
                    impl Default for PROPVARIANT_0_0 {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    pub union PROPVARIANT_0_0_0 {
                        pub cVal: i8,
                        pub bVal: u8,
                        pub iVal: i16,
                        pub uiVal: u16,
                        pub lVal: i32,
                        pub ulVal: u32,
                        pub intVal: i32,
                        pub uintVal: u32,
                        pub hVal: i64,
                        pub uhVal: u64,
                        pub fltVal: f32,
                        pub dblVal: f64,
                        pub boolVal: super::super::super::Foundation::VARIANT_BOOL,
                        pub __OBSOLETE__VARIANT_BOOL: super::super::super::Foundation::VARIANT_BOOL,
                        pub scode: i32,
                        pub cyVal: super::CY,
                        pub date: f64,
                        pub filetime: super::super::super::Foundation::FILETIME,
                        pub puuid: *mut windows_core::GUID,
                        pub pclipdata: *mut CLIPDATA,
                        pub bstrVal: core::mem::ManuallyDrop<windows_core::BSTR>,
                        pub bstrblobVal: BSTRBLOB,
                        pub blob: super::BLOB,
                        pub pszVal: windows_core::PSTR,
                        pub pwszVal: windows_core::PWSTR,
                        pub punkVal: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
                        pub pdispVal: core::mem::ManuallyDrop<Option<super::IDispatch>>,
                        pub pStream: core::mem::ManuallyDrop<Option<super::IStream>>,
                        pub pStorage: core::mem::ManuallyDrop<Option<IStorage>>,
                        pub pVersionedStream: *mut VERSIONEDSTREAM,
                        pub parray: *mut super::SAFEARRAY,
                        pub cac: CAC,
                        pub caub: CAUB,
                        pub cai: CAI,
                        pub caui: CAUI,
                        pub cal: CAL,
                        pub caul: CAUL,
                        pub cah: CAH,
                        pub cauh: CAUH,
                        pub caflt: CAFLT,
                        pub cadbl: CADBL,
                        pub cabool: CABOOL,
                        pub cascode: CASCODE,
                        pub cacy: CACY,
                        pub cadate: CADATE,
                        pub cafiletime: CAFILETIME,
                        pub cauuid: CACLSID,
                        pub caclipdata: CACLIPDATA,
                        pub cabstr: CABSTR,
                        pub cabstrblob: CABSTRBLOB,
                        pub calpstr: CALPSTR,
                        pub calpwstr: CALPWSTR,
                        pub capropvar: CAPROPVARIANT,
                        pub pcVal: windows_core::PSTR,
                        pub pbVal: *mut u8,
                        pub piVal: *mut i16,
                        pub puiVal: *mut u16,
                        pub plVal: *mut i32,
                        pub pulVal: *mut u32,
                        pub pintVal: *mut i32,
                        pub puintVal: *mut u32,
                        pub pfltVal: *mut f32,
                        pub pdblVal: *mut f64,
                        pub pboolVal: *mut super::super::super::Foundation::VARIANT_BOOL,
                        pub pdecVal: *mut super::super::super::Foundation::DECIMAL,
                        pub pscode: *mut i32,
                        pub pcyVal: *mut super::CY,
                        pub pdate: *mut f64,
                        pub pbstrVal: *mut windows_core::BSTR,
                        pub ppunkVal: *mut Option<windows_core::IUnknown>,
                        pub ppdispVal: *mut Option<super::IDispatch>,
                        pub pparray: *mut *mut super::SAFEARRAY,
                        pub pvarVal: *mut PROPVARIANT,
                    }
                    impl Clone for PROPVARIANT_0_0_0 {
                        fn clone(&self) -> Self {
                            unsafe { core::mem::transmute_copy(self) }
                        }
                    }
                    impl Default for PROPVARIANT_0_0_0 {
                        fn default() -> Self {
                            unsafe { core::mem::zeroed() }
                        }
                    }
                    #[repr(C)]
                    #[derive(Clone, Debug, Default, PartialEq)]
                    pub struct VERSIONEDSTREAM {
                        pub guidVersion: windows_core::GUID,
                        pub pStream: core::mem::ManuallyDrop<Option<super::IStream>>,
                    }
                }
            }
            pub mod Variant {
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct VARENUM(pub u16);
                impl VARENUM {
                    pub const fn contains(&self, other: Self) -> bool {
                        self.0 & other.0 == other.0
                    }
                }
                impl core::ops::BitOr for VARENUM {
                    type Output = Self;
                    fn bitor(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }
                impl core::ops::BitAnd for VARENUM {
                    type Output = Self;
                    fn bitand(self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
                impl core::ops::BitOrAssign for VARENUM {
                    fn bitor_assign(&mut self, other: Self) {
                        self.0.bitor_assign(other.0)
                    }
                }
                impl core::ops::BitAndAssign for VARENUM {
                    fn bitand_assign(&mut self, other: Self) {
                        self.0.bitand_assign(other.0)
                    }
                }
                impl core::ops::Not for VARENUM {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
            }
            pub mod WinRT {
                #[inline]
                pub unsafe fn RoInitialize(inittype: RO_INIT_TYPE) -> windows_core::Result<()> {
                    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoInitialize(inittype : RO_INIT_TYPE) -> windows_core::HRESULT);
                    unsafe { RoInitialize(inittype).ok() }
                }
                pub const RO_INIT_MULTITHREADED: RO_INIT_TYPE = RO_INIT_TYPE(1i32);
                pub const RO_INIT_SINGLETHREADED: RO_INIT_TYPE = RO_INIT_TYPE(0i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct RO_INIT_TYPE(pub i32);
                pub mod Direct3D11 {
                    #[inline]
                    pub unsafe fn CreateDirect3D11DeviceFromDXGIDevice<P0>(
                        dxgidevice: P0,
                    ) -> windows_core::Result<windows_core::IInspectable>
                    where
                        P0: windows_core::Param<super::super::super::Graphics::Dxgi::IDXGIDevice>,
                    {
                        windows_core::link!("d3d11.dll" "system" fn CreateDirect3D11DeviceFromDXGIDevice(dxgidevice : * mut core::ffi::c_void, graphicsdevice : *mut * mut core::ffi::c_void) -> windows_core::HRESULT);
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            CreateDirect3D11DeviceFromDXGIDevice(dxgidevice.param().abi(), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    windows_core::imp::define_interface!(
                        IDirect3DDxgiInterfaceAccess,
                        IDirect3DDxgiInterfaceAccess_Vtbl,
                        0xa9b3d012_3df2_4ee3_b8d1_8695f457d3c1
                    );
                    windows_core::imp::interface_hierarchy!(IDirect3DDxgiInterfaceAccess, windows_core::IUnknown);
                    impl IDirect3DDxgiInterfaceAccess {
                        pub unsafe fn GetInterface<T>(&self) -> windows_core::Result<T>
                        where
                            T: windows_core::Interface,
                        {
                            let mut result__ = core::ptr::null_mut();
                            unsafe {
                                (windows_core::Interface::vtable(self).GetInterface)(
                                    windows_core::Interface::as_raw(self),
                                    &T::IID,
                                    &mut result__,
                                )
                                .and_then(|| windows_core::Type::from_abi(result__))
                            }
                        }
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct IDirect3DDxgiInterfaceAccess_Vtbl {
                        pub base__: windows_core::IUnknown_Vtbl,
                        pub GetInterface: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *const windows_core::GUID,
                            *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT,
                    }
                    impl windows_core::RuntimeName for IDirect3DDxgiInterfaceAccess {}
                }
                pub mod Graphics {
                    pub mod Capture {
                        windows_core::imp::define_interface!(
                            IGraphicsCaptureItemInterop,
                            IGraphicsCaptureItemInterop_Vtbl,
                            0x3628e81b_3cac_4c60_b7f4_23ce0e0c3356
                        );
                        windows_core::imp::interface_hierarchy!(IGraphicsCaptureItemInterop, windows_core::IUnknown);
                        impl IGraphicsCaptureItemInterop {
                            pub unsafe fn CreateForWindow<T>(
                                &self,
                                window: super::super::super::super::Foundation::HWND,
                            ) -> windows_core::Result<T>
                            where
                                T: windows_core::Interface,
                            {
                                let mut result__ = core::ptr::null_mut();
                                unsafe {
                                    (windows_core::Interface::vtable(self).CreateForWindow)(
                                        windows_core::Interface::as_raw(self),
                                        window,
                                        &T::IID,
                                        &mut result__,
                                    )
                                    .and_then(|| windows_core::Type::from_abi(result__))
                                }
                            }
                            pub unsafe fn CreateForMonitor<T>(
                                &self,
                                monitor: super::super::super::super::Graphics::Gdi::HMONITOR,
                            ) -> windows_core::Result<T>
                            where
                                T: windows_core::Interface,
                            {
                                let mut result__ = core::ptr::null_mut();
                                unsafe {
                                    (windows_core::Interface::vtable(self).CreateForMonitor)(
                                        windows_core::Interface::as_raw(self),
                                        monitor,
                                        &T::IID,
                                        &mut result__,
                                    )
                                    .and_then(|| windows_core::Type::from_abi(result__))
                                }
                            }
                        }
                        #[repr(C)]
                        #[doc(hidden)]
                        pub struct IGraphicsCaptureItemInterop_Vtbl {
                            pub base__: windows_core::IUnknown_Vtbl,
                            pub CreateForWindow: unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                super::super::super::super::Foundation::HWND,
                                *const windows_core::GUID,
                                *mut *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT,
                            pub CreateForMonitor: unsafe extern "system" fn(
                                *mut core::ffi::c_void,
                                super::super::super::super::Graphics::Gdi::HMONITOR,
                                *const windows_core::GUID,
                                *mut *mut core::ffi::c_void,
                            ) -> windows_core::HRESULT,
                        }
                        impl windows_core::RuntimeName for IGraphicsCaptureItemInterop {}
                    }
                }
            }
        }
        pub mod UI {
            pub mod Shell {
                #[inline]
                pub unsafe fn SHCreateItemFromParsingName<P0, P1, T>(pszpath: P0, pbc: P1) -> windows_core::Result<T>
                where
                    P0: windows_core::Param<windows_core::PCWSTR>,
                    P1: windows_core::Param<super::super::System::Com::IBindCtx>,
                    T: windows_core::Interface,
                {
                    windows_core::link!("shell32.dll" "system" fn SHCreateItemFromParsingName(pszpath : windows_core::PCWSTR, pbc : * mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
                    let mut result__ = core::ptr::null_mut();
                    unsafe {
                        SHCreateItemFromParsingName(pszpath.param().abi(), pbc.param().abi(), &T::IID, &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                #[inline]
                pub unsafe fn SHCreateItemInKnownFolder<P2, T>(
                    kfid: *const windows_core::GUID,
                    dwkfflags: KNOWN_FOLDER_FLAG,
                    pszitem: P2,
                ) -> windows_core::Result<T>
                where
                    P2: windows_core::Param<windows_core::PCWSTR>,
                    T: windows_core::Interface,
                {
                    windows_core::link!("shell32.dll" "system" fn SHCreateItemInKnownFolder(kfid : *const windows_core::GUID, dwkfflags : u32, pszitem : windows_core::PCWSTR, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
                    let mut result__ = core::ptr::null_mut();
                    unsafe {
                        SHCreateItemInKnownFolder(kfid, dwkfflags.0 as _, pszitem.param().abi(), &T::IID, &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub const FOLDERID_AppsFolder: windows_core::GUID = windows_core::GUID::from_u128(0x1e87508d_89c2_42f0_8a7e_645a0f50ca58);
                windows_core::imp::define_interface!(
                    IShellItemImageFactory,
                    IShellItemImageFactory_Vtbl,
                    0xbcc18b79_ba16_442f_80c4_8a59c30c463b
                );
                windows_core::imp::interface_hierarchy!(IShellItemImageFactory, windows_core::IUnknown);
                impl IShellItemImageFactory {
                    pub unsafe fn GetImage(
                        &self,
                        size: super::super::Foundation::SIZE,
                        flags: SIIGBF,
                    ) -> windows_core::Result<super::super::Graphics::Gdi::HBITMAP> {
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(self).GetImage)(
                                windows_core::Interface::as_raw(self),
                                core::mem::transmute(size),
                                flags,
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IShellItemImageFactory_Vtbl {
                    pub base__: windows_core::IUnknown_Vtbl,
                    pub GetImage: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        super::super::Foundation::SIZE,
                        SIIGBF,
                        *mut super::super::Graphics::Gdi::HBITMAP,
                    ) -> windows_core::HRESULT,
                }
                impl windows_core::RuntimeName for IShellItemImageFactory {}
                pub const KF_FLAG_DEFAULT: KNOWN_FOLDER_FLAG = KNOWN_FOLDER_FLAG(0i32);
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct KNOWN_FOLDER_FLAG(pub i32);
                impl KNOWN_FOLDER_FLAG {
                    pub const fn contains(&self, other: Self) -> bool {
                        self.0 & other.0 == other.0
                    }
                }
                impl core::ops::BitOr for KNOWN_FOLDER_FLAG {
                    type Output = Self;
                    fn bitor(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }
                impl core::ops::BitAnd for KNOWN_FOLDER_FLAG {
                    type Output = Self;
                    fn bitand(self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
                impl core::ops::BitOrAssign for KNOWN_FOLDER_FLAG {
                    fn bitor_assign(&mut self, other: Self) {
                        self.0.bitor_assign(other.0)
                    }
                }
                impl core::ops::BitAndAssign for KNOWN_FOLDER_FLAG {
                    fn bitand_assign(&mut self, other: Self) {
                        self.0.bitand_assign(other.0)
                    }
                }
                impl core::ops::Not for KNOWN_FOLDER_FLAG {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct SIIGBF(pub i32);
                impl SIIGBF {
                    pub const fn contains(&self, other: Self) -> bool {
                        self.0 & other.0 == other.0
                    }
                }
                impl core::ops::BitOr for SIIGBF {
                    type Output = Self;
                    fn bitor(self, other: Self) -> Self {
                        Self(self.0 | other.0)
                    }
                }
                impl core::ops::BitAnd for SIIGBF {
                    type Output = Self;
                    fn bitand(self, other: Self) -> Self {
                        Self(self.0 & other.0)
                    }
                }
                impl core::ops::BitOrAssign for SIIGBF {
                    fn bitor_assign(&mut self, other: Self) {
                        self.0.bitor_assign(other.0)
                    }
                }
                impl core::ops::BitAndAssign for SIIGBF {
                    fn bitand_assign(&mut self, other: Self) {
                        self.0.bitand_assign(other.0)
                    }
                }
                impl core::ops::Not for SIIGBF {
                    type Output = Self;
                    fn not(self) -> Self {
                        Self(self.0.not())
                    }
                }
                pub const SIIGBF_ICONONLY: SIIGBF = SIIGBF(4i32);
                pub const SIIGBF_RESIZETOFIT: SIIGBF = SIIGBF(0i32);
                pub mod PropertiesSystem {
                    windows_core::imp::define_interface!(IPropertyStore, IPropertyStore_Vtbl, 0x886d8eeb_8cf2_4446_8d02_cdba1dbdcf99);
                    windows_core::imp::interface_hierarchy!(IPropertyStore, windows_core::IUnknown);
                    impl IPropertyStore {
                        pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__)
                                    .map(|| result__)
                            }
                        }
                        pub unsafe fn GetAt(
                            &self,
                            iprop: u32,
                            pkey: *mut super::super::super::Foundation::PROPERTYKEY,
                        ) -> windows_core::Result<()> {
                            unsafe {
                                (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), iprop, pkey as _).ok()
                            }
                        }
                        pub unsafe fn GetValue(
                            &self,
                            key: *const super::super::super::Foundation::PROPERTYKEY,
                        ) -> windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>
                        {
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), key, &mut result__)
                                    .map(|| core::mem::transmute(result__))
                            }
                        }
                        pub unsafe fn SetValue(
                            &self,
                            key: *const super::super::super::Foundation::PROPERTYKEY,
                            propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT,
                        ) -> windows_core::Result<()> {
                            unsafe {
                                (windows_core::Interface::vtable(self).SetValue)(
                                    windows_core::Interface::as_raw(self),
                                    key,
                                    core::mem::transmute(propvar),
                                )
                                .ok()
                            }
                        }
                        pub unsafe fn Commit(&self) -> windows_core::Result<()> {
                            unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self)).ok() }
                        }
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct IPropertyStore_Vtbl {
                        pub base__: windows_core::IUnknown_Vtbl,
                        pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
                        pub GetAt: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            u32,
                            *mut super::super::super::Foundation::PROPERTYKEY,
                        ) -> windows_core::HRESULT,
                        pub GetValue: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *const super::super::super::Foundation::PROPERTYKEY,
                            *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT,
                        ) -> windows_core::HRESULT,
                        pub SetValue: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *const super::super::super::Foundation::PROPERTYKEY,
                            *const super::super::super::System::Com::StructuredStorage::PROPVARIANT,
                        ) -> windows_core::HRESULT,
                        pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    }
                    impl windows_core::RuntimeName for IPropertyStore {}
                }
            }
        }
    }
}
