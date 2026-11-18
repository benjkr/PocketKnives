//! The WinUI `Application` subclass, made the way C#/WinRT and C++/WinRT make one: COM
//! aggregation. This outer object implements the two interfaces a subclass overrides,
//! `IApplicationOverrides` (`OnLaunched`) and `IXamlMetadataProvider`, and hands every other
//! interface to the inner `Application` that `IApplicationFactory::CreateInstance` makes for it.
//!
//! The metadata provider is what lets XAML markup inside the WinUI controls library (the styles
//! `XamlControlsResources` loads) find the library's types: it delegates to
//! `XamlControlsXamlMetaDataProvider`, as the C# app's generated XamlTypeInfo does.
//!
//! Hand-written vtables, because `#[implement]` can't forward unknown interfaces to an inner
//! object. The object lives for the whole process (never freed): `Application.Start` keeps it
//! until the process ends.
use crate::bindings::Microsoft::UI::Xaml::Markup::{IXamlMetadataProvider, IXamlMetadataProvider_Vtbl, IXamlType, XmlnsDefinition};
use crate::bindings::Microsoft::UI::Xaml::XamlTypeInfo::XamlControlsXamlMetaDataProvider;
use crate::bindings::Microsoft::UI::Xaml::{
    Application, IApplicationFactory, IApplicationOverrides, IApplicationOverrides_Vtbl,
};
use crate::bindings::Windows::UI::Xaml::Interop::TypeName;
use alloc::boxed::Box;
use core::cell::OnceCell;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicU32, Ordering};
use windows_core::{GUID, HRESULT, HSTRING, IInspectable, IInspectable_Vtbl, IUnknown, IUnknown_Vtbl, Interface};

const S_OK: HRESULT = HRESULT(0);
const E_POINTER: HRESULT = HRESULT(0x8000_4003_u32 as i32);

#[repr(C)]
struct App {
    /// The object's identity: its IUnknown, IInspectable and IApplicationOverrides.
    overrides: &'static IApplicationOverrides_Vtbl,
    provider: &'static IXamlMetadataProvider_Vtbl,
    refs: AtomicU32,
    /// The inner Application's non-delegating IInspectable.
    inner: *mut c_void,
    /// The Application, through the inner object (its references count on this object).
    application: OnceCell<Application>,
    controls: OnceCell<XamlControlsXamlMetaDataProvider>,
}

/// Creates the app inside `Application.Start`'s callback.
pub fn create() -> windows_core::Result<()> {
    let app: &'static mut App = Box::leak(Box::new(App {
        overrides: &OVERRIDES,
        provider: &PROVIDER,
        refs: AtomicU32::new(1), // ours, held forever; the inner object adds and releases more as it builds
        inner: null_mut(),
        application: OnceCell::new(),
        controls: OnceCell::new(),
    }));
    let outer = app as *mut App as *mut c_void;
    let factory = windows_core::factory::<Application, IApplicationFactory>()?;
    let (mut inner, mut instance) = (null_mut(), null_mut());
    unsafe { (factory.vtable().CreateInstance)(factory.as_raw(), outer, &mut inner, &mut instance) }.ok()?;
    app.inner = inner;
    let _ = app.application.set(unsafe { Application::from_raw(instance) });
    Ok(())
}

unsafe fn app<'a>(this: *mut c_void, offset: usize) -> &'a App {
    unsafe { &*((this as *mut *const c_void).sub(offset) as *const App) }
}

// IUnknown and IInspectable, for each of the two interface pointers (OFFSET: its slot in App).

unsafe extern "system" fn query<const OFFSET: usize>(this: *mut c_void, iid: *const GUID, out: *mut *mut c_void) -> HRESULT {
    if iid.is_null() || out.is_null() {
        return E_POINTER;
    }
    let app = unsafe { app(this, OFFSET) };
    let iid = unsafe { &*iid };
    let me = app as *const App as *mut *const c_void;
    let found = if *iid == IUnknown::IID || *iid == IInspectable::IID || *iid == IApplicationOverrides::IID {
        me
    } else if *iid == IXamlMetadataProvider::IID {
        unsafe { me.add(1) }
    } else {
        // Everything else (IApplication, IWeakReferenceSource, IMarshal, ...) is the inner object's.
        let inner = app.inner as *mut *const IUnknown_Vtbl;
        if inner.is_null() {
            unsafe { *out = null_mut() };
            return HRESULT(0x8000_4002_u32 as i32); // E_NOINTERFACE: not built yet
        }
        return unsafe { ((**inner).QueryInterface)(inner.cast(), iid, out) };
    };
    app.refs.fetch_add(1, Ordering::Relaxed);
    unsafe { *out = found.cast() };
    S_OK
}

unsafe extern "system" fn add_ref<const OFFSET: usize>(this: *mut c_void) -> u32 {
    unsafe { app(this, OFFSET) }.refs.fetch_add(1, Ordering::Relaxed) + 1
}

unsafe extern "system" fn release<const OFFSET: usize>(this: *mut c_void) -> u32 {
    // Never reaches 0: `create` holds a reference for the life of the process.
    unsafe { app(this, OFFSET) }.refs.fetch_sub(1, Ordering::Release) - 1
}

unsafe extern "system" fn get_iids<const OFFSET: usize>(this: *mut c_void, count: *mut u32, iids: *mut *mut GUID) -> HRESULT {
    let inner = unsafe { app(this, OFFSET) }.inner as *mut *const IInspectable_Vtbl;
    if inner.is_null() {
        unsafe { (*count, *iids) = (0, null_mut()) };
        return S_OK;
    }
    unsafe { ((**inner).GetIids)(inner.cast(), count, iids) }
}

unsafe extern "system" fn class_name<const OFFSET: usize>(_: *mut c_void, name: *mut *mut c_void) -> HRESULT {
    unsafe { *name = core::mem::transmute::<HSTRING, *mut c_void>(HSTRING::from("PocketKnives.App")) };
    S_OK
}

unsafe extern "system" fn trust_level<const OFFSET: usize>(_: *mut c_void, level: *mut i32) -> HRESULT {
    unsafe { *level = 0 }; // BaseTrust
    S_OK
}

const fn inspectable<const OFFSET: usize>() -> IInspectable_Vtbl {
    IInspectable_Vtbl {
        base: IUnknown_Vtbl { QueryInterface: query::<OFFSET>, AddRef: add_ref::<OFFSET>, Release: release::<OFFSET> },
        GetIids: get_iids::<OFFSET>,
        GetRuntimeClassName: class_name::<OFFSET>,
        GetTrustLevel: trust_level::<OFFSET>,
    }
}

// IApplicationOverrides

static OVERRIDES: IApplicationOverrides_Vtbl = IApplicationOverrides_Vtbl { base__: inspectable::<0>(), OnLaunched: on_launched };

unsafe extern "system" fn on_launched(this: *mut c_void, _args: *mut c_void) -> HRESULT {
    let app = unsafe { app(this, 0) };
    let Some(application) = app.application.get() else { return S_OK };
    // The LaunchActivatedEventArgs aren't used: the runner passes its window on the command line.
    if let Err(e) = crate::ui::launched(application) {
        crate::report("The window could not open", &e);
        crate::fail(crate::EXIT_WINUI);
    }
    S_OK
}

// IXamlMetadataProvider: the WinUI controls library's types.

static PROVIDER: IXamlMetadataProvider_Vtbl = IXamlMetadataProvider_Vtbl {
    base__: inspectable::<1>(),
    GetXamlType: xaml_type,
    GetXamlTypeByFullName: xaml_type_by_name,
    GetXmlnsDefinitions: xmlns_definitions,
};

fn controls(app: &App) -> Option<&XamlControlsXamlMetaDataProvider> {
    if app.controls.get().is_none() {
        let _ = app.controls.set(XamlControlsXamlMetaDataProvider::new().ok()?);
    }
    app.controls.get()
}

/// Writes a found type, or null for a type the library doesn't know (S_OK either way, as the
/// generated C# provider does).
fn write_type(found: Option<windows_core::Result<IXamlType>>, out: *mut *mut c_void) -> HRESULT {
    unsafe { *out = found.and_then(|r| r.ok()).map_or(null_mut(), |t| t.into_raw()) };
    S_OK
}

unsafe extern "system" fn xaml_type(this: *mut c_void, name: MaybeUninit<TypeName>, out: *mut *mut c_void) -> HRESULT {
    // Borrowed from the caller: not dropped here.
    let name: &TypeName = unsafe { name.assume_init_ref() };
    write_type(controls(unsafe { app(this, 1) }).map(|c| c.GetXamlType(name)), out)
}

unsafe extern "system" fn xaml_type_by_name(this: *mut c_void, name: *mut c_void, out: *mut *mut c_void) -> HRESULT {
    let name: &HSTRING = unsafe { &*(&name as *const *mut c_void as *const HSTRING) };
    write_type(controls(unsafe { app(this, 1) }).map(|c| c.GetXamlTypeByFullName(name)), out)
}

unsafe extern "system" fn xmlns_definitions(_: *mut c_void, count: *mut u32, items: *mut *mut MaybeUninit<XmlnsDefinition>) -> HRESULT {
    unsafe { (*count, *items) = (0, null_mut()) };
    S_OK
}
