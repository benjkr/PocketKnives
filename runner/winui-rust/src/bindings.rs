#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, clippy::all)]

pub mod Microsoft {
    pub mod UI {
        #[repr(transparent)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct Colors(windows_core::IUnknown);
        windows_core::imp::interface_hierarchy!(Colors, windows_core::IUnknown, windows_core::IInspectable);
        impl Colors {
            pub fn Gray() -> windows_core::Result<windows::UI::Color> {
                Self::IColorsStatics(|this| unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).Gray)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                })
            }
            fn IColorsStatics<R, F: FnOnce(&IColorsStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                static SHARED: windows_core::imp::FactoryCache<Colors, IColorsStatics> = windows_core::imp::FactoryCache::new();
                SHARED.call(callback)
            }
        }
        impl windows_core::RuntimeType for Colors {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IColors>();
        }
        unsafe impl windows_core::Interface for Colors {
            type Vtable = <IColors as windows_core::Interface>::Vtable;
            const IID: windows_core::GUID = <IColors as windows_core::Interface>::IID;
        }
        impl windows_core::RuntimeName for Colors {
            const NAME: &'static str = "Microsoft.UI.Colors";
        }
        unsafe impl Send for Colors {}
        unsafe impl Sync for Colors {}
        windows_core::imp::define_interface!(IColors, IColors_Vtbl, 0x8cf15863_8411_5afd_946c_328e04da2f2f);
        impl windows_core::RuntimeType for IColors {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IColors_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
        }
        windows_core::imp::define_interface!(IColorsStatics, IColorsStatics_Vtbl, 0x8620a5b0_015a_57ac_a3f3_895d0b1269ae);
        impl windows_core::RuntimeType for IColorsStatics {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
        }
        #[repr(C)]
        #[doc(hidden)]
        pub struct IColorsStatics_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub AliceBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub AntiqueWhite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Aqua: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Aquamarine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Azure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Beige: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Bisque: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Black: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub BlanchedAlmond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Blue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub BlueViolet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Brown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub BurlyWood: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub CadetBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Chartreuse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Chocolate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Coral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub CornflowerBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Cornsilk: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Crimson: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Cyan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DarkBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DarkCyan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DarkGoldenrod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DarkGray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DarkGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DarkKhaki: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DarkMagenta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DarkOliveGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DarkOrange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DarkOrchid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DarkRed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DarkSalmon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DarkSeaGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DarkSlateBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DarkSlateGray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DarkTurquoise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DarkViolet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DeepPink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DeepSkyBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DimGray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub DodgerBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Firebrick: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub FloralWhite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub ForestGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Fuchsia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Gainsboro: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub GhostWhite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Gold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Goldenrod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Gray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Green: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub GreenYellow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Honeydew: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub HotPink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub IndianRed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Indigo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Ivory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Khaki: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Lavender: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub LavenderBlush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub LawnGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub LemonChiffon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub LightBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub LightCoral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub LightCyan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub LightGoldenrodYellow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub LightGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub LightGray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub LightPink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub LightSalmon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub LightSeaGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub LightSkyBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub LightSlateGray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub LightSteelBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub LightYellow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Lime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub LimeGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Linen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Magenta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Maroon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub MediumAquamarine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub MediumBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub MediumOrchid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub MediumPurple: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub MediumSeaGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub MediumSlateBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub MediumSpringGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub MediumTurquoise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub MediumVioletRed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub MidnightBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub MintCream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub MistyRose: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Moccasin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub NavajoWhite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Navy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub OldLace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Olive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub OliveDrab: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Orange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub OrangeRed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Orchid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub PaleGoldenrod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub PaleGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub PaleTurquoise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub PaleVioletRed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub PapayaWhip: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub PeachPuff: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Peru: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Pink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Plum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub PowderBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Purple: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Red: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub RosyBrown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub RoyalBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub SaddleBrown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Salmon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub SandyBrown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub SeaGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub SeaShell: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Sienna: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Silver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub SkyBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub SlateBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub SlateGray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Snow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub SpringGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub SteelBlue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Tan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Teal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Thistle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Tomato: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Transparent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Turquoise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Violet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Wheat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub White: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub WhiteSmoke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub Yellow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
            pub YellowGreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
        }
        pub mod Composition {
            windows_core::imp::define_interface!(IAnimationObject, IAnimationObject_Vtbl, 0x8f56119d_b96d_58d0_9916_d1c5e390f890);
            impl windows_core::RuntimeType for IAnimationObject {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            windows_core::imp::interface_hierarchy!(IAnimationObject, windows_core::IUnknown, windows_core::IInspectable);
            impl windows_core::RuntimeName for IAnimationObject {
                const NAME: &'static str = "Microsoft.UI.Composition.IAnimationObject";
            }
            pub trait IAnimationObject_Impl: windows_core::IUnknownImpl {}
            impl IAnimationObject_Vtbl {
                pub const fn new<Identity: IAnimationObject_Impl, const OFFSET: isize>() -> Self {
                    Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IAnimationObject, OFFSET>(), PopulatePropertyInfo: 0 }
                }
                pub fn matches(iid: &windows_core::GUID) -> bool {
                    iid == &<IAnimationObject as windows_core::Interface>::IID
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAnimationObject_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                PopulatePropertyInfo: usize,
            }
            windows_core::imp::define_interface!(IVisualElement, IVisualElement_Vtbl, 0x2180f1f5_b5d8_4bf6_920a_12006e63efef);
            impl windows_core::RuntimeType for IVisualElement {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            windows_core::imp::interface_hierarchy!(IVisualElement, windows_core::IUnknown, windows_core::IInspectable);
            impl windows_core::RuntimeName for IVisualElement {
                const NAME: &'static str = "Microsoft.UI.Composition.IVisualElement";
            }
            pub trait IVisualElement_Impl: windows_core::IUnknownImpl {}
            impl IVisualElement_Vtbl {
                pub const fn new<Identity: IVisualElement_Impl, const OFFSET: isize>() -> Self {
                    Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IVisualElement, OFFSET>() }
                }
                pub fn matches(iid: &windows_core::GUID) -> bool {
                    iid == &<IVisualElement as windows_core::Interface>::IID
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IVisualElement_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(IVisualElement2, IVisualElement2_Vtbl, 0xbc950c8d_1db0_53aa_9dee_34271cd18ce6);
            impl windows_core::RuntimeType for IVisualElement2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            windows_core::imp::interface_hierarchy!(IVisualElement2, windows_core::IUnknown, windows_core::IInspectable);
            impl windows_core::RuntimeName for IVisualElement2 {
                const NAME: &'static str = "Microsoft.UI.Composition.IVisualElement2";
            }
            pub trait IVisualElement2_Impl: windows_core::IUnknownImpl {}
            impl IVisualElement2_Vtbl {
                pub const fn new<Identity: IVisualElement2_Impl, const OFFSET: isize>() -> Self {
                    Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IVisualElement2, OFFSET>(), GetVisualInternal: 0 }
                }
                pub fn matches(iid: &windows_core::GUID) -> bool {
                    iid == &<IVisualElement2 as windows_core::Interface>::IID
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IVisualElement2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                GetVisualInternal: usize,
            }
        }
        pub mod Text {
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct FontWeights(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(FontWeights, windows_core::IUnknown, windows_core::IInspectable);
            impl FontWeights {
                pub fn SemiBold() -> windows_core::Result<windows::UI::Text::FontWeight> {
                    Self::IFontWeightsStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).SemiBold)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    })
                }
                fn IFontWeightsStatics<R, F: FnOnce(&IFontWeightsStatics) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<FontWeights, IFontWeightsStatics> =
                        windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for FontWeights {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFontWeights>();
            }
            unsafe impl windows_core::Interface for FontWeights {
                type Vtable = <IFontWeights as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IFontWeights as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for FontWeights {
                const NAME: &'static str = "Microsoft.UI.Text.FontWeights";
            }
            unsafe impl Send for FontWeights {}
            unsafe impl Sync for FontWeights {}
            windows_core::imp::define_interface!(IFontWeights, IFontWeights_Vtbl, 0x386cd040_5404_5a8d_8bc7_2ca989f5c065);
            impl windows_core::RuntimeType for IFontWeights {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IFontWeights_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(IFontWeightsStatics, IFontWeightsStatics_Vtbl, 0xcc390df6_76b0_5807_8b9d_e949a4e623ae);
            impl windows_core::RuntimeType for IFontWeightsStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IFontWeightsStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Black: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::FontWeight) -> windows_core::HRESULT,
                pub Bold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::FontWeight) -> windows_core::HRESULT,
                pub ExtraBlack:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::FontWeight) -> windows_core::HRESULT,
                pub ExtraBold:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::FontWeight) -> windows_core::HRESULT,
                pub ExtraLight:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::FontWeight) -> windows_core::HRESULT,
                pub Light: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::FontWeight) -> windows_core::HRESULT,
                pub Medium: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::FontWeight) -> windows_core::HRESULT,
                pub Normal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::FontWeight) -> windows_core::HRESULT,
                pub SemiBold:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::FontWeight) -> windows_core::HRESULT,
                pub SemiLight:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::FontWeight) -> windows_core::HRESULT,
                pub Thin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::FontWeight) -> windows_core::HRESULT,
            }
        }
        pub mod Windowing {
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct AppWindow(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(AppWindow, windows_core::IUnknown, windows_core::IInspectable);
            impl AppWindow {
                pub fn Size(&self) -> windows_core::Result<windows::Graphics::SizeInt32> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                    }
                }
                pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                    let this = self;
                    unsafe {
                        (windows_core::Interface::vtable(this).SetTitle)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(value),
                        )
                        .ok()
                    }
                }
                pub fn Resize(&self, size: windows::Graphics::SizeInt32) -> windows_core::Result<()> {
                    let this = self;
                    unsafe { (windows_core::Interface::vtable(this).Resize)(windows_core::Interface::as_raw(this), size).ok() }
                }
                pub fn SetIcon(&self, iconpath: &windows_core::HSTRING) -> windows_core::Result<()> {
                    let this = self;
                    unsafe {
                        (windows_core::Interface::vtable(this).SetIcon)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(iconpath),
                        )
                        .ok()
                    }
                }
                fn IAppWindowStatics<R, F: FnOnce(&IAppWindowStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<AppWindow, IAppWindowStatics> = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                fn IAppWindowStatics2<R, F: FnOnce(&IAppWindowStatics2) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<AppWindow, IAppWindowStatics2> = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for AppWindow {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppWindow>();
            }
            unsafe impl windows_core::Interface for AppWindow {
                type Vtable = <IAppWindow as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IAppWindow as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for AppWindow {
                const NAME: &'static str = "Microsoft.UI.Windowing.AppWindow";
            }
            unsafe impl Send for AppWindow {}
            unsafe impl Sync for AppWindow {}
            windows_core::imp::define_interface!(IAppWindow, IAppWindow_Vtbl, 0xcfa788b3_643b_5c5e_ad4e_321d48a82acd);
            impl windows_core::RuntimeType for IAppWindow {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAppWindow_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                Id: usize,
                pub IsShownInSwitchers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetIsShownInSwitchers: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                pub IsVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                OwnerWindowId: usize,
                pub Position:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Graphics::PointInt32) -> windows_core::HRESULT,
                Presenter: usize,
                pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Graphics::SizeInt32) -> windows_core::HRESULT,
                pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                TitleBar: usize,
                pub Destroy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                pub Hide: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Graphics::PointInt32) -> windows_core::HRESULT,
                pub MoveAndResize: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Graphics::RectInt32) -> windows_core::HRESULT,
                MoveAndResizeRelativeToDisplayArea: usize,
                pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Graphics::SizeInt32) -> windows_core::HRESULT,
                pub SetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                SetIconWithIconId: usize,
                SetPresenter: usize,
                SetPresenterByKind: usize,
                pub Show: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ShowWithActivation: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                Changed: usize,
                pub RemoveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                Closing: usize,
                pub RemoveClosing: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub Destroying:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveDestroying: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IAppWindow2, IAppWindow2_Vtbl, 0x6cd41292_794c_5cac_8961_210d012c6ebc);
            impl windows_core::RuntimeType for IAppWindow2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAppWindow2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub ClientSize:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Graphics::SizeInt32) -> windows_core::HRESULT,
                pub MoveInZOrderAtBottom: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                pub MoveInZOrderAtTop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                MoveInZOrderBelow: usize,
                pub ResizeClient: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Graphics::SizeInt32) -> windows_core::HRESULT,
                pub ShowOnceWithRequestedStartupState: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IAppWindow3, IAppWindow3_Vtbl, 0x2f260cea_193d_5dd6_a904_d7649a608d2f);
            impl windows_core::RuntimeType for IAppWindow3 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAppWindow3_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                AssociateWithDispatcherQueue: usize,
                DispatcherQueue: usize,
            }
            windows_core::imp::define_interface!(IAppWindow4, IAppWindow4_Vtbl, 0x383bfb91_ea29_5414_80cd_6c76d981eb31);
            impl windows_core::RuntimeType for IAppWindow4 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAppWindow4_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub SetTaskbarIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                SetTaskbarIconWithIconId: usize,
                pub SetTitleBarIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                SetTitleBarIconWithIconId: usize,
            }
            windows_core::imp::define_interface!(IAppWindowStatics, IAppWindowStatics_Vtbl, 0x3c315c24_d540_5d72_b518_b226b83627cb);
            impl windows_core::RuntimeType for IAppWindowStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAppWindowStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                CreateWithPresenter: usize,
                CreateWithPresenterAndOwner: usize,
                GetFromWindowId: usize,
            }
            windows_core::imp::define_interface!(IAppWindowStatics2, IAppWindowStatics2_Vtbl, 0xcabc23db_4606_5d6e_89a5_06de1d8bd3e2);
            impl windows_core::RuntimeType for IAppWindowStatics2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IAppWindowStatics2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                CreateWithDispatcherQueue: usize,
            }
        }
        pub mod Xaml {
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Application(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(Application, windows_core::IUnknown, windows_core::IInspectable);
            impl Application {
                pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<ResourceDictionary>,
                {
                    let this = self;
                    unsafe {
                        (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                            .ok()
                    }
                }
                pub fn Exit(&self) -> windows_core::Result<()> {
                    let this = self;
                    unsafe { (windows_core::Interface::vtable(this).Exit)(windows_core::Interface::as_raw(this)).ok() }
                }
                pub fn new() -> windows_core::Result<Application> {
                    Self::IApplicationFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::ptr::null_mut(),
                            &mut core::ptr::null_mut(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    })
                }
                pub fn OnLaunched<P0>(&self, args: P0) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<LaunchActivatedEventArgs>,
                {
                    let this = &windows_core::Interface::cast::<IApplicationOverrides>(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this).OnLaunched)(windows_core::Interface::as_raw(this), args.param().abi()).ok()
                    }
                }
                pub fn Start<P0>(callback: P0) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<ApplicationInitializationCallback>,
                {
                    Self::IApplicationStatics(|this| unsafe {
                        (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this), callback.param().abi()).ok()
                    })
                }
                fn IApplicationFactory<R, F: FnOnce(&IApplicationFactory) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<Application, IApplicationFactory> =
                        windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                fn IApplicationStatics<R, F: FnOnce(&IApplicationStatics) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<Application, IApplicationStatics> =
                        windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for Application {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IApplication>();
            }
            unsafe impl windows_core::Interface for Application {
                type Vtable = <IApplication as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IApplication as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for Application {
                const NAME: &'static str = "Microsoft.UI.Xaml.Application";
            }
            unsafe impl Send for Application {}
            unsafe impl Sync for Application {}
            windows_core::imp::define_interface!(
                ApplicationInitializationCallback,
                ApplicationInitializationCallback_Vtbl,
                0xd8eef1c9_1234_56f1_9963_45dd9c80a661
            );
            impl windows_core::RuntimeType for ApplicationInitializationCallback {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"delegate(").push_other(windows_core::imp::ConstBuffer::for_interface::<Self>()).push_slice(b")");
            }
            impl ApplicationInitializationCallback {
                pub fn new<
                    F: Fn(windows_core::Ref<ApplicationInitializationCallbackParams>) -> windows_core::Result<()> + Send + 'static,
                >(
                    invoke: F,
                ) -> Self {
                    let com = ApplicationInitializationCallbackBox {
                        vtable: &ApplicationInitializationCallbackBox::<F>::VTABLE,
                        count: windows_core::imp::RefCount::new(1),
                        invoke,
                    };
                    unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ApplicationInitializationCallback_Vtbl {
                base__: windows_core::IUnknown_Vtbl,
                Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, p: *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            #[repr(C)]
            struct ApplicationInitializationCallbackBox<
                F: Fn(windows_core::Ref<ApplicationInitializationCallbackParams>) -> windows_core::Result<()> + Send + 'static,
            > {
                vtable: *const ApplicationInitializationCallback_Vtbl,
                invoke: F,
                count: windows_core::imp::RefCount,
            }
            impl<F: Fn(windows_core::Ref<ApplicationInitializationCallbackParams>) -> windows_core::Result<()> + Send + 'static>
                ApplicationInitializationCallbackBox<F>
            {
                const VTABLE: ApplicationInitializationCallback_Vtbl = ApplicationInitializationCallback_Vtbl {
                    base__: windows_core::IUnknown_Vtbl {
                        QueryInterface: Self::QueryInterface,
                        AddRef: Self::AddRef,
                        Release: Self::Release,
                    },
                    Invoke: Self::Invoke,
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
                        *interface = if *iid == <ApplicationInitializationCallback as windows_core::Interface>::IID
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
                unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, p: *mut core::ffi::c_void) -> windows_core::HRESULT {
                    unsafe {
                        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
                        (this.invoke)(core::mem::transmute_copy(&p)).into()
                    }
                }
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct ApplicationInitializationCallbackParams(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                ApplicationInitializationCallbackParams,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl ApplicationInitializationCallbackParams {}
            impl windows_core::RuntimeType for ApplicationInitializationCallbackParams {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IApplicationInitializationCallbackParams>();
            }
            unsafe impl windows_core::Interface for ApplicationInitializationCallbackParams {
                type Vtable = <IApplicationInitializationCallbackParams as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IApplicationInitializationCallbackParams as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for ApplicationInitializationCallbackParams {
                const NAME: &'static str = "Microsoft.UI.Xaml.ApplicationInitializationCallbackParams";
            }
            unsafe impl Send for ApplicationInitializationCallbackParams {}
            unsafe impl Sync for ApplicationInitializationCallbackParams {}
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct CornerRadius {
                pub TopLeft: f64,
                pub TopRight: f64,
                pub BottomRight: f64,
                pub BottomLeft: f64,
            }
            impl windows_core::TypeKind for CornerRadius {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for CornerRadius {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.Xaml.CornerRadius;f8;f8;f8;f8)");
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct DependencyObject(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(DependencyObject, windows_core::IUnknown, windows_core::IInspectable);
            impl DependencyObject {
                pub fn new() -> windows_core::Result<DependencyObject> {
                    Self::IDependencyObjectFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::ptr::null_mut(),
                            &mut core::ptr::null_mut(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    })
                }
                fn IDependencyObjectFactory<R, F: FnOnce(&IDependencyObjectFactory) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<DependencyObject, IDependencyObjectFactory> =
                        windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for DependencyObject {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDependencyObject>();
            }
            unsafe impl windows_core::Interface for DependencyObject {
                type Vtable = <IDependencyObject as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IDependencyObject as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for DependencyObject {
                const NAME: &'static str = "Microsoft.UI.Xaml.DependencyObject";
            }
            unsafe impl Send for DependencyObject {}
            unsafe impl Sync for DependencyObject {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct DependencyProperty(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(DependencyProperty, windows_core::IUnknown, windows_core::IInspectable);
            impl DependencyProperty {
                fn IDependencyPropertyStatics<R, F: FnOnce(&IDependencyPropertyStatics) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<DependencyProperty, IDependencyPropertyStatics> =
                        windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for DependencyProperty {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDependencyProperty>();
            }
            unsafe impl windows_core::Interface for DependencyProperty {
                type Vtable = <IDependencyProperty as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IDependencyProperty as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for DependencyProperty {
                const NAME: &'static str = "Microsoft.UI.Xaml.DependencyProperty";
            }
            unsafe impl Send for DependencyProperty {}
            unsafe impl Sync for DependencyProperty {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct FrameworkElement(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(FrameworkElement, windows_core::IUnknown, windows_core::IInspectable);
            windows_core::imp::required_hierarchy!(
                FrameworkElement,
                super::Composition::IAnimationObject,
                super::Composition::IVisualElement,
                super::Composition::IVisualElement2,
                UIElement,
                DependencyObject
            );
            impl FrameworkElement {
                pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<ResourceDictionary>,
                {
                    let this = self;
                    unsafe {
                        (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                            .ok()
                    }
                }
                pub fn Width(&self) -> windows_core::Result<f64> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                    }
                }
                pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                    let this = self;
                    unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                }
                pub fn Height(&self) -> windows_core::Result<f64> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                    let this = self;
                    unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                }
                pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                    let this = self;
                    unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                }
                pub fn HorizontalAlignment(&self) -> windows_core::Result<HorizontalAlignment> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).HorizontalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn VerticalAlignment(&self) -> windows_core::Result<VerticalAlignment> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                            .map(|| result__)
                    }
                }
                pub fn SetVerticalAlignment(&self, value: VerticalAlignment) -> windows_core::Result<()> {
                    let this = self;
                    unsafe {
                        (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                    }
                }
                pub fn SetMargin(&self, value: Thickness) -> windows_core::Result<()> {
                    let this = self;
                    unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                }
                pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                    let this = self;
                    unsafe {
                        (windows_core::Interface::vtable(this).SetName)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(value),
                        )
                        .ok()
                    }
                }
                pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                where
                    P0: windows_core::Param<RoutedEventHandler>,
                {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Loaded)(
                            windows_core::Interface::as_raw(this),
                            handler.param().abi(),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn new() -> windows_core::Result<FrameworkElement> {
                    Self::IFrameworkElementFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::ptr::null_mut(),
                            &mut core::ptr::null_mut(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    })
                }
                pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                    let this = &windows_core::Interface::cast::<IUIElement>(self)?;
                    unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                }
                pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                where
                    P0: windows_core::Param<RoutedEventHandler>,
                {
                    let this = &windows_core::Interface::cast::<IUIElement>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).LostFocus)(
                            windows_core::Interface::as_raw(this),
                            handler.param().abi(),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<RoutedEvent>,
                    P1: windows_core::Param<windows_core::IInspectable>,
                {
                    let this = &windows_core::Interface::cast::<IUIElement>(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this).AddHandler)(
                            windows_core::Interface::as_raw(this),
                            routedevent.param().abi(),
                            handler.param().abi(),
                            handledeventstoo,
                        )
                        .ok()
                    }
                }
                fn IFrameworkElementFactory<R, F: FnOnce(&IFrameworkElementFactory) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<FrameworkElement, IFrameworkElementFactory> =
                        windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                fn IFrameworkElementStatics<R, F: FnOnce(&IFrameworkElementStatics) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<FrameworkElement, IFrameworkElementStatics> =
                        windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for FrameworkElement {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFrameworkElement>();
            }
            unsafe impl windows_core::Interface for FrameworkElement {
                type Vtable = <IFrameworkElement as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IFrameworkElement as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for FrameworkElement {
                const NAME: &'static str = "Microsoft.UI.Xaml.FrameworkElement";
            }
            unsafe impl Send for FrameworkElement {}
            unsafe impl Sync for FrameworkElement {}
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct GridLength {
                pub Value: f64,
                pub GridUnitType: GridUnitType,
            }
            impl windows_core::TypeKind for GridLength {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for GridLength {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                    b"struct(Microsoft.UI.Xaml.GridLength;f8;enum(Microsoft.UI.Xaml.GridUnitType;i4))",
                );
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct GridUnitType(pub i32);
            impl GridUnitType {
                pub const Auto: Self = Self(0i32);
                pub const Pixel: Self = Self(1i32);
                pub const Star: Self = Self(2i32);
            }
            impl windows_core::TypeKind for GridUnitType {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for GridUnitType {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.GridUnitType;i4)");
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct HorizontalAlignment(pub i32);
            impl HorizontalAlignment {
                pub const Left: Self = Self(0i32);
                pub const Center: Self = Self(1i32);
                pub const Right: Self = Self(2i32);
                pub const Stretch: Self = Self(3i32);
            }
            impl windows_core::TypeKind for HorizontalAlignment {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for HorizontalAlignment {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.HorizontalAlignment;i4)");
            }
            windows_core::imp::define_interface!(IApplication, IApplication_Vtbl, 0x06a8f4e7_1146_55af_820d_ebd55643b021);
            impl windows_core::RuntimeType for IApplication {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IApplication_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Resources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                DebugSettings: usize,
                RequestedTheme: usize,
                SetRequestedTheme: usize,
                FocusVisualKind: usize,
                SetFocusVisualKind: usize,
                HighContrastAdjustment: usize,
                SetHighContrastAdjustment: usize,
                UnhandledException: usize,
                pub RemoveUnhandledException: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub Exit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IApplication2, IApplication2_Vtbl, 0x469e6d36_2e11_5b06_9e0a_c5eef0cf8f12);
            impl windows_core::RuntimeType for IApplication2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IApplication2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                ResourceManagerRequested: usize,
                pub RemoveResourceManagerRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IApplication3, IApplication3_Vtbl, 0xbe941595_61fe_5b36_a3d3_962a647d7c6f);
            impl windows_core::RuntimeType for IApplication3 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IApplication3_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                DispatcherShutdownMode: usize,
                SetDispatcherShutdownMode: usize,
            }
            windows_core::imp::define_interface!(IApplicationFactory, IApplicationFactory_Vtbl, 0x9fd96657_5294_5a65_a1db_4fea143597da);
            impl windows_core::RuntimeType for IApplicationFactory {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IApplicationFactory_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CreateInstance: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IApplicationInitializationCallbackParams,
                IApplicationInitializationCallbackParams_Vtbl,
                0x1b1906ea_5b7b_5876_81ab_7c2281ac3d20
            );
            impl windows_core::RuntimeType for IApplicationInitializationCallbackParams {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IApplicationInitializationCallbackParams_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(IApplicationOverrides, IApplicationOverrides_Vtbl, 0xa33e81ef_c665_503b_8827_d27ef1720a06);
            impl windows_core::RuntimeType for IApplicationOverrides {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IApplicationOverrides_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub OnLaunched: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IApplicationStatics, IApplicationStatics_Vtbl, 0x4e0d09f5_4358_512c_a987_503b52848e95);
            impl windows_core::RuntimeType for IApplicationStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IApplicationStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub LoadComponent: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                LoadComponentWithResourceLocation: usize,
            }
            windows_core::imp::define_interface!(IDependencyObject, IDependencyObject_Vtbl, 0xe7beaee7_160e_50f7_8789_d63463f979fa);
            impl windows_core::RuntimeType for IDependencyObject {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IDependencyObject_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub GetValue: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub SetValue: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub ClearValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ReadLocalValue: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub GetAnimationBaseValue: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                RegisterPropertyChangedCallback: usize,
                pub UnregisterPropertyChangedCallback:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub Dispatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                DispatcherQueue: usize,
            }
            windows_core::imp::define_interface!(
                IDependencyObjectFactory,
                IDependencyObjectFactory_Vtbl,
                0x936b614c_475f_5d7d_b3f7_bf1fbea28126
            );
            impl windows_core::RuntimeType for IDependencyObjectFactory {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IDependencyObjectFactory_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CreateInstance: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IDependencyProperty, IDependencyProperty_Vtbl, 0x960eab49_9672_58a0_995b_3a42e5ea6278);
            impl windows_core::RuntimeType for IDependencyProperty {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IDependencyProperty_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                GetMetadata: usize,
            }
            windows_core::imp::define_interface!(
                IDependencyPropertyStatics,
                IDependencyPropertyStatics_Vtbl,
                0x61ddc651_0383_5d6f_98ce_5c046aaaaa8f
            );
            impl windows_core::RuntimeType for IDependencyPropertyStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IDependencyPropertyStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub UnsetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                Register: usize,
                RegisterAttached: usize,
            }
            windows_core::imp::define_interface!(IFrameworkElement, IFrameworkElement_Vtbl, 0xfe08f13d_dc6a_5495_ad44_c2d8d21863b0);
            impl windows_core::RuntimeType for IFrameworkElement {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IFrameworkElement_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                Triggers: usize,
                pub Resources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub Tag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ActualWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                pub ActualHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                pub SetWidth: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                pub SetHeight: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                pub MinWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                pub SetMinWidth: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                pub MaxWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                pub SetMaxWidth: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                pub MinHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                pub SetMinHeight: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                pub MaxHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                pub SetMaxHeight: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                pub HorizontalAlignment:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut HorizontalAlignment) -> windows_core::HRESULT,
                pub SetHorizontalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, HorizontalAlignment) -> windows_core::HRESULT,
                pub VerticalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VerticalAlignment) -> windows_core::HRESULT,
                pub SetVerticalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, VerticalAlignment) -> windows_core::HRESULT,
                pub Margin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Thickness) -> windows_core::HRESULT,
                pub SetMargin: unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
                pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub BaseUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub DataContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetDataContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub AllowFocusOnInteraction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetAllowFocusOnInteraction: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                pub FocusVisualMargin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Thickness) -> windows_core::HRESULT,
                pub SetFocusVisualMargin: unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
                pub FocusVisualSecondaryThickness:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut Thickness) -> windows_core::HRESULT,
                pub SetFocusVisualSecondaryThickness: unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
                pub FocusVisualPrimaryThickness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Thickness) -> windows_core::HRESULT,
                pub SetFocusVisualPrimaryThickness: unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
                pub FocusVisualSecondaryBrush:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetFocusVisualSecondaryBrush:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub FocusVisualPrimaryBrush:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetFocusVisualPrimaryBrush:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub AllowFocusWhenDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetAllowFocusWhenDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                Style: usize,
                SetStyle: usize,
                pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                FlowDirection: usize,
                SetFlowDirection: usize,
                RequestedTheme: usize,
                SetRequestedTheme: usize,
                pub IsLoaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                ActualTheme: usize,
                pub Loaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveLoaded: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub Unloaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveUnloaded: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                DataContextChanged: usize,
                pub RemoveDataContextChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                SizeChanged: usize,
                pub RemoveSizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub LayoutUpdated:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveLayoutUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub Loading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveLoading: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub ActualThemeChanged:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveActualThemeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                EffectiveViewportChanged: usize,
                pub RemoveEffectiveViewportChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub FindName: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                SetBinding: usize,
                GetBindingExpression: usize,
            }
            windows_core::imp::define_interface!(
                IFrameworkElementFactory,
                IFrameworkElementFactory_Vtbl,
                0xbd3f2272_3efa_5f92_b759_90b1cc3e784c
            );
            impl windows_core::RuntimeType for IFrameworkElementFactory {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IFrameworkElementFactory_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CreateInstance: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IFrameworkElementOverrides,
                IFrameworkElementOverrides_Vtbl,
                0xffc6fd98_f38c_5904_9ce4_97a3427cf4ba
            );
            impl windows_core::RuntimeType for IFrameworkElementOverrides {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IFrameworkElementOverrides_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub MeasureOverride: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    windows::Foundation::Size,
                    *mut windows::Foundation::Size,
                ) -> windows_core::HRESULT,
                pub ArrangeOverride: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    windows::Foundation::Size,
                    *mut windows::Foundation::Size,
                ) -> windows_core::HRESULT,
                pub OnApplyTemplate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                pub GoToElementStateCore:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut bool) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IFrameworkElementProtected,
                IFrameworkElementProtected_Vtbl,
                0xe59a3db0_91e5_5903_9caf_d1bb9f458bf2
            );
            impl windows_core::RuntimeType for IFrameworkElementProtected {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IFrameworkElementProtected_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub InvalidateViewport: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IFrameworkElementStatics,
                IFrameworkElementStatics_Vtbl,
                0x894e2704_14e7_569a_b21e_afc7df7145a1
            );
            impl windows_core::RuntimeType for IFrameworkElementStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IFrameworkElementStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub TagProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub LanguageProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ActualWidthProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ActualHeightProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub WidthProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub HeightProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub MinWidthProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub MaxWidthProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub MinHeightProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub MaxHeightProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub HorizontalAlignmentProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub VerticalAlignmentProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub MarginProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub NameProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub DataContextProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub AllowFocusOnInteractionProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub FocusVisualMarginProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub FocusVisualSecondaryThicknessProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub FocusVisualPrimaryThicknessProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub FocusVisualSecondaryBrushProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub FocusVisualPrimaryBrushProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub AllowFocusWhenDisabledProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub StyleProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub FlowDirectionProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub RequestedThemeProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ActualThemeProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub DeferTree: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                ILaunchActivatedEventArgs,
                ILaunchActivatedEventArgs_Vtbl,
                0xd505cea9_1bcb_5b29_a8be_944e00f06f78
            );
            impl windows_core::RuntimeType for ILaunchActivatedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct ILaunchActivatedEventArgs_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub UWPLaunchActivatedEventArgs:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IResourceDictionary, IResourceDictionary_Vtbl, 0x1b690975_a710_5783_a6e1_15836f6186c2);
            impl windows_core::RuntimeType for IResourceDictionary {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IResourceDictionary_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub MergedDictionaries:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ThemeDictionaries:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IResourceDictionaryFactory,
                IResourceDictionaryFactory_Vtbl,
                0xea22a48f_ab71_56f6_a392_d82310c8aa7b
            );
            impl windows_core::RuntimeType for IResourceDictionaryFactory {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IResourceDictionaryFactory_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CreateInstance: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IRoutedEvent, IRoutedEvent_Vtbl, 0xb2b432bc_efca_575e_9d2a_703f8b9c380f);
            impl windows_core::RuntimeType for IRoutedEvent {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IRoutedEvent_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(IRoutedEventArgs, IRoutedEventArgs_Vtbl, 0x0908c407_1c7d_5de3_9c50_d971c62ec8ec);
            impl windows_core::RuntimeType for IRoutedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IRoutedEventArgs_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub OriginalSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(
                IRoutedEventArgsFactory,
                IRoutedEventArgsFactory_Vtbl,
                0x914b02c7_076b_5b89_98e7_6c373379e9af
            );
            impl windows_core::RuntimeType for IRoutedEventArgsFactory {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IRoutedEventArgsFactory_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CreateInstance: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IUIElement, IUIElement_Vtbl, 0xc3c01020_320c_5cf6_9d24_d396bbfa4d8b);
            impl windows_core::RuntimeType for IUIElement {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IUIElement_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub DesiredSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Size) -> windows_core::HRESULT,
                pub AllowDrop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetAllowDrop: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                pub Opacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                Clip: usize,
                SetClip: usize,
                RenderTransform: usize,
                SetRenderTransform: usize,
                Projection: usize,
                SetProjection: usize,
                Transform3D: usize,
                SetTransform3D: usize,
                pub RenderTransformOrigin:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
                pub SetRenderTransformOrigin:
                    unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Point) -> windows_core::HRESULT,
                pub IsHitTestVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetIsHitTestVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                Visibility: usize,
                SetVisibility: usize,
                pub RenderSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Size) -> windows_core::HRESULT,
                pub UseLayoutRounding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetUseLayoutRounding: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                Transitions: usize,
                SetTransitions: usize,
                CacheMode: usize,
                SetCacheMode: usize,
                pub IsTapEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetIsTapEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                pub IsDoubleTapEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetIsDoubleTapEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                pub CanDrag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetCanDrag: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                pub IsRightTapEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetIsRightTapEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                pub IsHoldingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetIsHoldingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                ManipulationMode: usize,
                SetManipulationMode: usize,
                PointerCaptures: usize,
                ContextFlyout: usize,
                SetContextFlyout: usize,
                CompositeMode: usize,
                SetCompositeMode: usize,
                Lights: usize,
                pub CanBeScrollAnchor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetCanBeScrollAnchor: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                pub ExitDisplayModeOnAccessKeyInvoked:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetExitDisplayModeOnAccessKeyInvoked: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                pub IsAccessKeyScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetIsAccessKeyScope: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                pub AccessKeyScopeOwner:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetAccessKeyScopeOwner:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub AccessKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetAccessKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                KeyTipPlacementMode: usize,
                SetKeyTipPlacementMode: usize,
                pub KeyTipHorizontalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                pub SetKeyTipHorizontalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                pub KeyTipVerticalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                pub SetKeyTipVerticalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                pub KeyTipTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetKeyTipTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                XYFocusKeyboardNavigation: usize,
                SetXYFocusKeyboardNavigation: usize,
                XYFocusUpNavigationStrategy: usize,
                SetXYFocusUpNavigationStrategy: usize,
                XYFocusDownNavigationStrategy: usize,
                SetXYFocusDownNavigationStrategy: usize,
                XYFocusLeftNavigationStrategy: usize,
                SetXYFocusLeftNavigationStrategy: usize,
                XYFocusRightNavigationStrategy: usize,
                SetXYFocusRightNavigationStrategy: usize,
                KeyboardAccelerators: usize,
                pub KeyboardAcceleratorPlacementTarget:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetKeyboardAcceleratorPlacementTarget:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                KeyboardAcceleratorPlacementMode: usize,
                SetKeyboardAcceleratorPlacementMode: usize,
                HighContrastAdjustment: usize,
                SetHighContrastAdjustment: usize,
                TabFocusNavigation: usize,
                SetTabFocusNavigation: usize,
                OpacityTransition: usize,
                SetOpacityTransition: usize,
                pub Translation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
                pub SetTranslation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector3) -> windows_core::HRESULT,
                TranslationTransition: usize,
                SetTranslationTransition: usize,
                pub Rotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
                pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
                RotationTransition: usize,
                SetRotationTransition: usize,
                pub Scale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
                pub SetScale: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector3) -> windows_core::HRESULT,
                ScaleTransition: usize,
                SetScaleTransition: usize,
                pub TransformMatrix:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Matrix4x4) -> windows_core::HRESULT,
                pub SetTransformMatrix:
                    unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Matrix4x4) -> windows_core::HRESULT,
                pub CenterPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
                pub SetCenterPoint: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector3) -> windows_core::HRESULT,
                pub RotationAxis:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
                pub SetRotationAxis: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector3) -> windows_core::HRESULT,
                pub ActualOffset:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
                pub ActualSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector2) -> windows_core::HRESULT,
                XamlRoot: usize,
                SetXamlRoot: usize,
                Shadow: usize,
                SetShadow: usize,
                pub RasterizationScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                pub SetRasterizationScale: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                FocusState: usize,
                pub UseSystemFocusVisuals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetUseSystemFocusVisuals: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                pub XYFocusLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetXYFocusLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub XYFocusRight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetXYFocusRight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub XYFocusUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetXYFocusUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub XYFocusDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetXYFocusDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub IsTabStop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetIsTabStop: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                pub TabIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                pub SetTabIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
                pub KeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub KeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub GotFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveGotFocus: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub LostFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveLostFocus: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                DragStarting: usize,
                pub RemoveDragStarting: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                DropCompleted: usize,
                pub RemoveDropCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                CharacterReceived: usize,
                pub RemoveCharacterReceived: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                DragEnter: usize,
                pub RemoveDragEnter: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                DragLeave: usize,
                pub RemoveDragLeave: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                DragOver: usize,
                pub RemoveDragOver: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                Drop: usize,
                pub RemoveDrop: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                PointerPressed: usize,
                pub RemovePointerPressed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                PointerMoved: usize,
                pub RemovePointerMoved: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                PointerReleased: usize,
                pub RemovePointerReleased: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                PointerEntered: usize,
                pub RemovePointerEntered: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                PointerExited: usize,
                pub RemovePointerExited: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                PointerCaptureLost: usize,
                pub RemovePointerCaptureLost: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                PointerCanceled: usize,
                pub RemovePointerCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                PointerWheelChanged: usize,
                pub RemovePointerWheelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                Tapped: usize,
                pub RemoveTapped: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                DoubleTapped: usize,
                pub RemoveDoubleTapped: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                Holding: usize,
                pub RemoveHolding: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                ContextRequested: usize,
                pub RemoveContextRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub ContextCanceled:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveContextCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                RightTapped: usize,
                pub RemoveRightTapped: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                ManipulationStarting: usize,
                pub RemoveManipulationStarting: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                ManipulationInertiaStarting: usize,
                pub RemoveManipulationInertiaStarting: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                ManipulationStarted: usize,
                pub RemoveManipulationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                ManipulationDelta: usize,
                pub RemoveManipulationDelta: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                ManipulationCompleted: usize,
                pub RemoveManipulationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                AccessKeyDisplayRequested: usize,
                pub RemoveAccessKeyDisplayRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                AccessKeyDisplayDismissed: usize,
                pub RemoveAccessKeyDisplayDismissed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                AccessKeyInvoked: usize,
                pub RemoveAccessKeyInvoked: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                ProcessKeyboardAccelerators: usize,
                pub RemoveProcessKeyboardAccelerators: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                GettingFocus: usize,
                pub RemoveGettingFocus: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                LosingFocus: usize,
                pub RemoveLosingFocus: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                NoFocusCandidateFound: usize,
                pub RemoveNoFocusCandidateFound: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub PreviewKeyDown:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemovePreviewKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub PreviewKeyUp:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemovePreviewKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                BringIntoViewRequested: usize,
                pub RemoveBringIntoViewRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub Measure: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Size) -> windows_core::HRESULT,
                pub Arrange: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Rect) -> windows_core::HRESULT,
                CapturePointer: usize,
                ReleasePointerCapture: usize,
                pub ReleasePointerCaptures: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                pub AddHandler: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    bool,
                ) -> windows_core::HRESULT,
                pub RemoveHandler: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                TransformToVisual: usize,
                pub InvalidateMeasure: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                pub InvalidateArrange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                pub UpdateLayout: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                pub CancelDirectManipulations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                StartDragAsync: usize,
                pub StartBringIntoView: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                StartBringIntoViewWithOptions: usize,
                TryInvokeKeyboardAccelerator: usize,
                Focus: usize,
                StartAnimation: usize,
                StopAnimation: usize,
            }
            windows_core::imp::define_interface!(IUIElementFactory, IUIElementFactory_Vtbl, 0x14d1d309_add0_5ccb_b946_77488cd70f87);
            impl windows_core::RuntimeType for IUIElementFactory {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IUIElementFactory_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(IUIElementOverrides, IUIElementOverrides_Vtbl, 0x9034f41e_ab7b_59e7_8168_50de6b689dde);
            impl windows_core::RuntimeType for IUIElementOverrides {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IUIElementOverrides_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                OnCreateAutomationPeer: usize,
                pub OnDisconnectVisualChildren: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                pub FindSubElementsForTouchTargeting: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    windows::Foundation::Point,
                    windows::Foundation::Rect,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
                pub GetChildrenInTabFocusOrder:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                OnKeyboardAcceleratorInvoked: usize,
                OnProcessKeyboardAccelerators: usize,
                OnBringIntoViewRequested: usize,
                PopulatePropertyInfoOverride: usize,
            }
            windows_core::imp::define_interface!(IUIElementProtected, IUIElementProtected_Vtbl, 0x8f69b9e9_1f00_5834_9bf1_a9257bed39f0);
            impl windows_core::RuntimeType for IUIElementProtected {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IUIElementProtected_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                ProtectedCursor: usize,
                SetProtectedCursor: usize,
            }
            windows_core::imp::define_interface!(IUIElementStatics, IUIElementStatics_Vtbl, 0xd2921d87_3584_5e22_8a3a_c2c78dab4f6e);
            impl windows_core::RuntimeType for IUIElementStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IUIElementStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub KeyDownEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub KeyUpEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub PointerEnteredEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub PointerPressedEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub PointerMovedEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub PointerReleasedEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub PointerExitedEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub PointerCaptureLostEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub PointerCanceledEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub PointerWheelChangedEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TappedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub DoubleTappedEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub HoldingEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub RightTappedEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ManipulationStartingEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ManipulationInertiaStartingEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ManipulationStartedEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ManipulationDeltaEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ManipulationCompletedEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub DragEnterEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub DragLeaveEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub DragOverEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub DropEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub GettingFocusEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub LosingFocusEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub NoFocusCandidateFoundEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub PreviewKeyDownEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub CharacterReceivedEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub PreviewKeyUpEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub BringIntoViewRequestedEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ContextRequestedEvent:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub AllowDropProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub OpacityProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ClipProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub RenderTransformProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ProjectionProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub Transform3DProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub RenderTransformOriginProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub IsHitTestVisibleProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub VisibilityProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub UseLayoutRoundingProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TransitionsProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub CacheModeProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub IsTapEnabledProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub IsDoubleTapEnabledProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub CanDragProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub IsRightTapEnabledProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub IsHoldingEnabledProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ManipulationModeProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub PointerCapturesProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ContextFlyoutProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub CompositeModeProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub LightsProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub CanBeScrollAnchorProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ExitDisplayModeOnAccessKeyInvokedProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub IsAccessKeyScopeProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub AccessKeyScopeOwnerProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub AccessKeyProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub KeyTipPlacementModeProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub KeyTipHorizontalOffsetProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub KeyTipVerticalOffsetProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub KeyTipTargetProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub XYFocusKeyboardNavigationProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub XYFocusUpNavigationStrategyProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub XYFocusDownNavigationStrategyProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub XYFocusLeftNavigationStrategyProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub XYFocusRightNavigationStrategyProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub KeyboardAcceleratorPlacementTargetProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub KeyboardAcceleratorPlacementModeProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub HighContrastAdjustmentProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TabFocusNavigationProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ShadowProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub FocusStateProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub UseSystemFocusVisualsProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub XYFocusLeftProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub XYFocusRightProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub XYFocusUpProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub XYFocusDownProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub IsTabStopProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub TabIndexProperty:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                TryStartDirectManipulation: usize,
                pub RegisterAsScrollPort:
                    unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IWindow, IWindow_Vtbl, 0x61f0ec79_5d52_56b5_86fb_40fa4af288b0);
            impl windows_core::RuntimeType for IWindow {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IWindow_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Bounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Rect) -> windows_core::HRESULT,
                pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub CoreWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                Compositor: usize,
                pub Dispatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                DispatcherQueue: usize,
                pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub ExtendsContentIntoTitleBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetExtendsContentIntoTitleBar: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                Activated: usize,
                pub RemoveActivated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                SizeChanged: usize,
                pub RemoveSizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                VisibilityChanged: usize,
                pub RemoveVisibilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetTitleBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IWindow2, IWindow2_Vtbl, 0x42febaa5_1c32_522a_a591_57618c6f665d);
            impl windows_core::RuntimeType for IWindow2 {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IWindow2_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub SystemBackdrop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub SetSystemBackdrop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                pub AppWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IWindowEventArgs, IWindowEventArgs_Vtbl, 0x1140827c_fe0a_5268_bc2b_f4492c2ccb49);
            impl windows_core::RuntimeType for IWindowEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IWindowEventArgs_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IWindowFactory, IWindowFactory_Vtbl, 0xf0441536_afef_5222_918f_324a9b2dec75);
            impl windows_core::RuntimeType for IWindowFactory {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IWindowFactory_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub CreateInstance: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                    *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            windows_core::imp::define_interface!(IWindowStatics, IWindowStatics_Vtbl, 0x8cc985e3_a41a_5df4_b531_d3a1788d86c5);
            impl windows_core::RuntimeType for IWindowStatics {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct IWindowStatics_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct LaunchActivatedEventArgs(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(LaunchActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
            impl LaunchActivatedEventArgs {
            }
            impl windows_core::RuntimeType for LaunchActivatedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, ILaunchActivatedEventArgs>();
            }
            unsafe impl windows_core::Interface for LaunchActivatedEventArgs {
                type Vtable = <ILaunchActivatedEventArgs as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <ILaunchActivatedEventArgs as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for LaunchActivatedEventArgs {
                const NAME: &'static str = "Microsoft.UI.Xaml.LaunchActivatedEventArgs";
            }
            unsafe impl Send for LaunchActivatedEventArgs {}
            unsafe impl Sync for LaunchActivatedEventArgs {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct ResourceDictionary(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(ResourceDictionary, windows_core::IUnknown, windows_core::IInspectable);
            windows_core::imp::required_hierarchy ! ( ResourceDictionary , windows_collections:: IIterable < windows_collections:: IKeyValuePair < windows_core::IInspectable , windows_core::IInspectable > > , windows_collections:: IMap < windows_core::IInspectable , windows_core::IInspectable > , DependencyObject );
            impl ResourceDictionary {
                pub fn First(
                    &self,
                ) -> windows_core::Result<
                    windows_collections::IIterator<
                        windows_collections::IKeyValuePair<windows_core::IInspectable, windows_core::IInspectable>,
                    >,
                > {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IIterable<
                            windows_collections::IKeyValuePair<windows_core::IInspectable, windows_core::IInspectable>,
                        >,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn Size(&self) -> windows_core::Result<u32> {
                    let this = &windows_core::Interface::cast::<
                        windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>,
                    >(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
                    }
                }
                pub fn SetSource<P0>(&self, value: P0) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<windows::Foundation::Uri>,
                {
                    let this = self;
                    unsafe {
                        (windows_core::Interface::vtable(this).SetSource)(windows_core::Interface::as_raw(this), value.param().abi()).ok()
                    }
                }
                pub fn MergedDictionaries(&self) -> windows_core::Result<windows_collections::IVector<ResourceDictionary>> {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).MergedDictionaries)(windows_core::Interface::as_raw(this), &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn new() -> windows_core::Result<ResourceDictionary> {
                    Self::IResourceDictionaryFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::ptr::null_mut(),
                            &mut core::ptr::null_mut(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    })
                }
                fn IResourceDictionaryFactory<R, F: FnOnce(&IResourceDictionaryFactory) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<ResourceDictionary, IResourceDictionaryFactory> =
                        windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for ResourceDictionary {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IResourceDictionary>();
            }
            unsafe impl windows_core::Interface for ResourceDictionary {
                type Vtable = <IResourceDictionary as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IResourceDictionary as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for ResourceDictionary {
                const NAME: &'static str = "Microsoft.UI.Xaml.ResourceDictionary";
            }
            unsafe impl Send for ResourceDictionary {}
            unsafe impl Sync for ResourceDictionary {}
            impl IntoIterator for ResourceDictionary {
                type Item = windows_collections::IKeyValuePair<windows_core::IInspectable, windows_core::IInspectable>;
                type IntoIter = windows_collections::IIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    IntoIterator::into_iter(&self)
                }
            }
            impl IntoIterator for &ResourceDictionary {
                type Item = windows_collections::IKeyValuePair<windows_core::IInspectable, windows_core::IInspectable>;
                type IntoIter = windows_collections::IIterator<Self::Item>;
                fn into_iter(self) -> Self::IntoIter {
                    self.First().unwrap()
                }
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct RoutedEvent(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(RoutedEvent, windows_core::IUnknown, windows_core::IInspectable);
            impl RoutedEvent {}
            impl windows_core::RuntimeType for RoutedEvent {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRoutedEvent>();
            }
            unsafe impl windows_core::Interface for RoutedEvent {
                type Vtable = <IRoutedEvent as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IRoutedEvent as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for RoutedEvent {
                const NAME: &'static str = "Microsoft.UI.Xaml.RoutedEvent";
            }
            unsafe impl Send for RoutedEvent {}
            unsafe impl Sync for RoutedEvent {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct RoutedEventArgs(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(RoutedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
            impl RoutedEventArgs {
                pub fn new() -> windows_core::Result<RoutedEventArgs> {
                    Self::IRoutedEventArgsFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::ptr::null_mut(),
                            &mut core::ptr::null_mut(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    })
                }
                fn IRoutedEventArgsFactory<R, F: FnOnce(&IRoutedEventArgsFactory) -> windows_core::Result<R>>(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<RoutedEventArgs, IRoutedEventArgsFactory> =
                        windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for RoutedEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRoutedEventArgs>();
            }
            unsafe impl windows_core::Interface for RoutedEventArgs {
                type Vtable = <IRoutedEventArgs as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IRoutedEventArgs as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for RoutedEventArgs {
                const NAME: &'static str = "Microsoft.UI.Xaml.RoutedEventArgs";
            }
            unsafe impl Send for RoutedEventArgs {}
            unsafe impl Sync for RoutedEventArgs {}
            windows_core::imp::define_interface!(RoutedEventHandler, RoutedEventHandler_Vtbl, 0xdae23d85_69ca_5bdf_805b_6161a3a215cc);
            impl windows_core::RuntimeType for RoutedEventHandler {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"delegate(").push_other(windows_core::imp::ConstBuffer::for_interface::<Self>()).push_slice(b")");
            }
            impl RoutedEventHandler {
                pub fn new<
                    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) -> windows_core::Result<()>
                        + Send
                        + 'static,
                >(
                    invoke: F,
                ) -> Self {
                    let com = RoutedEventHandlerBox {
                        vtable: &RoutedEventHandlerBox::<F>::VTABLE,
                        count: windows_core::imp::RefCount::new(1),
                        invoke,
                    };
                    unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
                }
            }
            #[repr(C)]
            #[doc(hidden)]
            pub struct RoutedEventHandler_Vtbl {
                base__: windows_core::IUnknown_Vtbl,
                Invoke: unsafe extern "system" fn(
                    this: *mut core::ffi::c_void,
                    sender: *mut core::ffi::c_void,
                    e: *mut core::ffi::c_void,
                ) -> windows_core::HRESULT,
            }
            #[repr(C)]
            struct RoutedEventHandlerBox<
                F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) -> windows_core::Result<()>
                    + Send
                    + 'static,
            > {
                vtable: *const RoutedEventHandler_Vtbl,
                invoke: F,
                count: windows_core::imp::RefCount,
            }
            impl<
                    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) -> windows_core::Result<()>
                        + Send
                        + 'static,
                > RoutedEventHandlerBox<F>
            {
                const VTABLE: RoutedEventHandler_Vtbl = RoutedEventHandler_Vtbl {
                    base__: windows_core::IUnknown_Vtbl {
                        QueryInterface: Self::QueryInterface,
                        AddRef: Self::AddRef,
                        Release: Self::Release,
                    },
                    Invoke: Self::Invoke,
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
                        *interface = if *iid == <RoutedEventHandler as windows_core::Interface>::IID
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
                    e: *mut core::ffi::c_void,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
                        (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
                    }
                }
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct TextWrapping(pub i32);
            impl TextWrapping {
                pub const NoWrap: Self = Self(1i32);
                pub const Wrap: Self = Self(2i32);
                pub const WrapWholeWords: Self = Self(3i32);
            }
            impl windows_core::TypeKind for TextWrapping {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for TextWrapping {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.TextWrapping;i4)");
            }
            #[repr(C)]
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub struct Thickness {
                pub Left: f64,
                pub Top: f64,
                pub Right: f64,
                pub Bottom: f64,
            }
            impl windows_core::TypeKind for Thickness {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for Thickness {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.Xaml.Thickness;f8;f8;f8;f8)");
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct UIElement(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(UIElement, windows_core::IUnknown, windows_core::IInspectable);
            windows_core::imp::required_hierarchy!(
                UIElement,
                super::Composition::IAnimationObject,
                super::Composition::IVisualElement,
                super::Composition::IVisualElement2,
                DependencyObject
            );
            impl UIElement {
                pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                    let this = self;
                    unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                }
                pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                where
                    P0: windows_core::Param<RoutedEventHandler>,
                {
                    let this = self;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).LostFocus)(
                            windows_core::Interface::as_raw(this),
                            handler.param().abi(),
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<RoutedEvent>,
                    P1: windows_core::Param<windows_core::IInspectable>,
                {
                    let this = self;
                    unsafe {
                        (windows_core::Interface::vtable(this).AddHandler)(
                            windows_core::Interface::as_raw(this),
                            routedevent.param().abi(),
                            handler.param().abi(),
                            handledeventstoo,
                        )
                        .ok()
                    }
                }
                pub fn KeyDownEvent() -> windows_core::Result<RoutedEvent> {
                    Self::IUIElementStatics(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).KeyDownEvent)(windows_core::Interface::as_raw(this), &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    })
                }
                fn IUIElementStatics<R, F: FnOnce(&IUIElementStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<UIElement, IUIElementStatics> = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for UIElement {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUIElement>();
            }
            unsafe impl windows_core::Interface for UIElement {
                type Vtable = <IUIElement as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IUIElement as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for UIElement {
                const NAME: &'static str = "Microsoft.UI.Xaml.UIElement";
            }
            unsafe impl Send for UIElement {}
            unsafe impl Sync for UIElement {}
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct VerticalAlignment(pub i32);
            impl VerticalAlignment {
                pub const Top: Self = Self(0i32);
                pub const Center: Self = Self(1i32);
                pub const Bottom: Self = Self(2i32);
                pub const Stretch: Self = Self(3i32);
            }
            impl windows_core::TypeKind for VerticalAlignment {
                type TypeKind = windows_core::CopyType;
            }
            impl windows_core::RuntimeType for VerticalAlignment {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.VerticalAlignment;i4)");
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct Window(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(Window, windows_core::IUnknown, windows_core::IInspectable);
            impl Window {
                pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<UIElement>,
                {
                    let this = self;
                    unsafe {
                        (windows_core::Interface::vtable(this).SetContent)(windows_core::Interface::as_raw(this), value.param().abi()).ok()
                    }
                }
                pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                    let this = self;
                    unsafe {
                        (windows_core::Interface::vtable(this).SetTitle)(
                            windows_core::Interface::as_raw(this),
                            core::mem::transmute_copy(value),
                        )
                        .ok()
                    }
                }
                pub fn Activate(&self) -> windows_core::Result<()> {
                    let this = self;
                    unsafe { (windows_core::Interface::vtable(this).Activate)(windows_core::Interface::as_raw(this)).ok() }
                }
                pub fn Close(&self) -> windows_core::Result<()> {
                    let this = self;
                    unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
                }
                pub fn SetSystemBackdrop<P0>(&self, value: P0) -> windows_core::Result<()>
                where
                    P0: windows_core::Param<Media::SystemBackdrop>,
                {
                    let this = &windows_core::Interface::cast::<IWindow2>(self)?;
                    unsafe {
                        (windows_core::Interface::vtable(this).SetSystemBackdrop)(
                            windows_core::Interface::as_raw(this),
                            value.param().abi(),
                        )
                        .ok()
                    }
                }
                pub fn AppWindow(&self) -> windows_core::Result<super::Windowing::AppWindow> {
                    let this = &windows_core::Interface::cast::<IWindow2>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).AppWindow)(windows_core::Interface::as_raw(this), &mut result__)
                            .and_then(|| windows_core::Type::from_abi(result__))
                    }
                }
                pub fn new() -> windows_core::Result<Window> {
                    Self::IWindowFactory(|this| unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).CreateInstance)(
                            windows_core::Interface::as_raw(this),
                            core::ptr::null_mut(),
                            &mut core::ptr::null_mut(),
                            &mut result__,
                        )
                        .and_then(|| windows_core::Type::from_abi(result__))
                    })
                }
                fn IWindowFactory<R, F: FnOnce(&IWindowFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<Window, IWindowFactory> = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                fn IWindowStatics<R, F: FnOnce(&IWindowStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<Window, IWindowStatics> = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
            }
            impl windows_core::RuntimeType for Window {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindow>();
            }
            unsafe impl windows_core::Interface for Window {
                type Vtable = <IWindow as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IWindow as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for Window {
                const NAME: &'static str = "Microsoft.UI.Xaml.Window";
            }
            unsafe impl Send for Window {}
            unsafe impl Sync for Window {}
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct WindowEventArgs(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(WindowEventArgs, windows_core::IUnknown, windows_core::IInspectable);
            impl WindowEventArgs {
                pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
                    let this = self;
                    unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
                }
            }
            impl windows_core::RuntimeType for WindowEventArgs {
                const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowEventArgs>();
            }
            unsafe impl windows_core::Interface for WindowEventArgs {
                type Vtable = <IWindowEventArgs as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IWindowEventArgs as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for WindowEventArgs {
                const NAME: &'static str = "Microsoft.UI.Xaml.WindowEventArgs";
            }
            unsafe impl Send for WindowEventArgs {}
            unsafe impl Sync for WindowEventArgs {}
            pub mod Automation {
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct AutomationProperties(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(AutomationProperties, windows_core::IUnknown, windows_core::IInspectable);
                impl AutomationProperties {
                    pub fn SetName<P0>(element: P0, value: &windows_core::HSTRING) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::DependencyObject>,
                    {
                        Self::IAutomationPropertiesStatics(|this| unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                element.param().abi(),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        })
                    }
                    fn IAutomationPropertiesStatics<R, F: FnOnce(&IAutomationPropertiesStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<AutomationProperties, IAutomationPropertiesStatics> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn IAutomationPropertiesStatics2<R, F: FnOnce(&IAutomationPropertiesStatics2) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<AutomationProperties, IAutomationPropertiesStatics2> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for AutomationProperties {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IAutomationProperties>();
                }
                unsafe impl windows_core::Interface for AutomationProperties {
                    type Vtable = <IAutomationProperties as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IAutomationProperties as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for AutomationProperties {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Automation.AutomationProperties";
                }
                unsafe impl Send for AutomationProperties {}
                unsafe impl Sync for AutomationProperties {}
                windows_core::imp::define_interface!(
                    IAutomationProperties,
                    IAutomationProperties_Vtbl,
                    0x525c6a71_dd8a_52a0_977b_db1b02f8e896
                );
                impl windows_core::RuntimeType for IAutomationProperties {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IAutomationProperties_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                }
                windows_core::imp::define_interface!(
                    IAutomationPropertiesStatics,
                    IAutomationPropertiesStatics_Vtbl,
                    0xb1e3e0f3_112f_5966_87dc_7862d4ad50e5
                );
                impl windows_core::RuntimeType for IAutomationPropertiesStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IAutomationPropertiesStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub AcceleratorKeyProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetAcceleratorKey: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetAcceleratorKey: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub AccessKeyProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetAccessKey: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetAccessKey: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub AutomationIdProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetAutomationId: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetAutomationId: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub HelpTextProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetHelpText: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetHelpText: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub IsRequiredForFormProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetIsRequiredForForm:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsRequiredForForm:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub ItemStatusProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetItemStatus: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetItemStatus: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub ItemTypeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetItemType: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetItemType: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub LabeledByProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetLabeledBy: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetLabeledBy: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub NameProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetName: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetName: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub LiveSettingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetLiveSetting: usize,
                    SetLiveSetting: usize,
                    pub AccessibilityViewProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetAccessibilityView: usize,
                    SetAccessibilityView: usize,
                    pub ControlledPeersProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetControlledPeers: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub PositionInSetProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetPositionInSet:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                    pub SetPositionInSet:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
                    pub SizeOfSetProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetSizeOfSet:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                    pub SetSizeOfSet:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
                    pub LevelProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetLevel:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                    pub SetLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
                    pub AnnotationsProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetAnnotations: usize,
                    pub LandmarkTypeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetLandmarkType: usize,
                    SetLandmarkType: usize,
                    pub LocalizedLandmarkTypeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetLocalizedLandmarkType: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetLocalizedLandmarkType: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub IsPeripheralProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetIsPeripheral:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsPeripheral:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsDataValidForFormProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetIsDataValidForForm:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsDataValidForForm:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub FullDescriptionProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetFullDescription: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetFullDescription: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub LocalizedControlTypeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetLocalizedControlType: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SetLocalizedControlType: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub DescribedByProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetDescribedBy: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub FlowsToProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetFlowsTo: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub FlowsFromProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetFlowsFrom: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub CultureProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetCulture:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                    pub SetCulture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
                    pub HeadingLevelProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetHeadingLevel: usize,
                    SetHeadingLevel: usize,
                    pub IsDialogProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetIsDialog:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsDialog:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IAutomationPropertiesStatics2,
                    IAutomationPropertiesStatics2_Vtbl,
                    0xd933a3ed_e90a_5df0_853d_cad17a0b9ec8
                );
                impl windows_core::RuntimeType for IAutomationPropertiesStatics2 {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IAutomationPropertiesStatics2_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub AutomationControlTypeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetAutomationControlType: usize,
                    SetAutomationControlType: usize,
                }
            }
            pub mod Controls {
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct Border(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(Border, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    Border,
                    super::super::Composition::IAnimationObject,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl Border {
                    pub fn new() -> windows_core::Result<Self> {
                        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                    }
                    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<Border, windows_core::imp::IGenericFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::Brush>,
                    {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetBackground)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn CornerRadius(&self) -> windows_core::Result<super::CornerRadius> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CornerRadius)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetCornerRadius(&self, value: super::CornerRadius) -> windows_core::Result<()> {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetCornerRadius)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetPadding(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetChild<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::UIElement>,
                    {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetChild)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn IBorderStatics<R, F: FnOnce(&IBorderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<Border, IBorderStatics> = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for Border {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBorder>();
                }
                unsafe impl windows_core::Interface for Border {
                    type Vtable = <IBorder as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IBorder as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for Border {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Border";
                }
                unsafe impl Send for Border {}
                unsafe impl Sync for Border {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct Button(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(Button, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    Button,
                    super::super::Composition::IAnimationObject,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    Primitives::ButtonBase,
                    ContentControl,
                    Control,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl Button {
                    pub fn Click<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<Primitives::IButtonBase>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Click)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn new() -> windows_core::Result<Button> {
                        Self::IButtonFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::ptr::null_mut(),
                                &mut core::ptr::null_mut(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<IContentControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetContent)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetPadding(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetHorizontalContentAlignment(&self, value: super::HorizontalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::Brush>,
                    {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetBackground)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn CornerRadius(&self) -> windows_core::Result<super::CornerRadius> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CornerRadius)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetCornerRadius(&self, value: super::CornerRadius) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetCornerRadius)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn IButtonFactory<R, F: FnOnce(&IButtonFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<Button, IButtonFactory> = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn IButtonStatics<R, F: FnOnce(&IButtonStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<Button, IButtonStatics> = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for Button {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IButton>();
                }
                unsafe impl windows_core::Interface for Button {
                    type Vtable = <IButton as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IButton as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for Button {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Button";
                }
                unsafe impl Send for Button {}
                unsafe impl Sync for Button {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct ColumnDefinition(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(ColumnDefinition, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(ColumnDefinition, super::DependencyObject);
                impl ColumnDefinition {
                    pub fn new() -> windows_core::Result<Self> {
                        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                    }
                    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<ColumnDefinition, windows_core::imp::IGenericFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    pub fn Width(&self) -> windows_core::Result<super::GridLength> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: super::GridLength) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    fn IColumnDefinitionStatics<R, F: FnOnce(&IColumnDefinitionStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<ColumnDefinition, IColumnDefinitionStatics> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for ColumnDefinition {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IColumnDefinition>();
                }
                unsafe impl windows_core::Interface for ColumnDefinition {
                    type Vtable = <IColumnDefinition as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IColumnDefinition as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for ColumnDefinition {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ColumnDefinition";
                }
                unsafe impl Send for ColumnDefinition {}
                unsafe impl Sync for ColumnDefinition {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct ColumnDefinitionCollection(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    ColumnDefinitionCollection,
                    windows_core::IUnknown,
                    windows_core::IInspectable,
                    windows_collections::IVector<ColumnDefinition>
                );
                windows_core::imp::required_hierarchy!(ColumnDefinitionCollection, windows_collections::IIterable<ColumnDefinition>);
                impl ColumnDefinitionCollection {
                    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<ColumnDefinition>> {
                        let this = &windows_core::Interface::cast::<windows_collections::IIterable<ColumnDefinition>>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn Size(&self) -> windows_core::Result<u32> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<ColumnDefinition>,
                    {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok()
                        }
                    }
                }
                impl windows_core::RuntimeType for ColumnDefinitionCollection {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IVector<ColumnDefinition>>();
                }
                unsafe impl windows_core::Interface for ColumnDefinitionCollection {
                    type Vtable = <windows_collections::IVector<ColumnDefinition> as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <windows_collections::IVector<ColumnDefinition> as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for ColumnDefinitionCollection {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ColumnDefinitionCollection";
                }
                unsafe impl Send for ColumnDefinitionCollection {}
                unsafe impl Sync for ColumnDefinitionCollection {}
                impl IntoIterator for ColumnDefinitionCollection {
                    type Item = ColumnDefinition;
                    type IntoIter = windows_collections::IIterator<Self::Item>;
                    fn into_iter(self) -> Self::IntoIter {
                        IntoIterator::into_iter(&self)
                    }
                }
                impl IntoIterator for &ColumnDefinitionCollection {
                    type Item = ColumnDefinition;
                    type IntoIter = windows_collections::IIterator<Self::Item>;
                    fn into_iter(self) -> Self::IntoIter {
                        self.First().unwrap()
                    }
                }
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct ComboBox(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(ComboBox, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    ComboBox,
                    super::super::Composition::IAnimationObject,
                    IItemContainerMapping,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    Primitives::Selector,
                    ItemsControl,
                    Control,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl ComboBox {
                    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| core::mem::transmute(result__))
                        }
                    }
                    pub fn SetText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetText)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn new() -> windows_core::Result<ComboBox> {
                        Self::IComboBoxFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::ptr::null_mut(),
                                &mut core::ptr::null_mut(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetPadding(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetHorizontalContentAlignment(&self, value: super::HorizontalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::Brush>,
                    {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetBackground)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn CornerRadius(&self) -> windows_core::Result<super::CornerRadius> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CornerRadius)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetCornerRadius(&self, value: super::CornerRadius) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetCornerRadius)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn Items(&self) -> windows_core::Result<ItemCollection> {
                        let this = &windows_core::Interface::cast::<IItemsControl>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Items)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn SelectedIndex(&self) -> windows_core::Result<i32> {
                        let this = &windows_core::Interface::cast::<Primitives::ISelector>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).SelectedIndex)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetSelectedIndex(&self, value: i32) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<Primitives::ISelector>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetSelectedIndex)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SelectedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
                        let this = &windows_core::Interface::cast::<Primitives::ISelector>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).SelectedItem)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn SetSelectedItem<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<Primitives::ISelector>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetSelectedItem)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SelectionChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<SelectionChangedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<Primitives::ISelector>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).SelectionChanged)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn IComboBoxFactory<R, F: FnOnce(&IComboBoxFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<ComboBox, IComboBoxFactory> = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn IComboBoxStatics<R, F: FnOnce(&IComboBoxStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<ComboBox, IComboBoxStatics> = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for ComboBox {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IComboBox>();
                }
                unsafe impl windows_core::Interface for ComboBox {
                    type Vtable = <IComboBox as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IComboBox as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for ComboBox {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ComboBox";
                }
                unsafe impl Send for ComboBox {}
                unsafe impl Sync for ComboBox {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct ContentControl(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(ContentControl, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    ContentControl,
                    super::super::Composition::IAnimationObject,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    Control,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl ContentControl {
                    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetContent)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn new() -> windows_core::Result<ContentControl> {
                        Self::IContentControlFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::ptr::null_mut(),
                                &mut core::ptr::null_mut(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetPadding(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetHorizontalContentAlignment(&self, value: super::HorizontalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::Brush>,
                    {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetBackground)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn CornerRadius(&self) -> windows_core::Result<super::CornerRadius> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CornerRadius)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetCornerRadius(&self, value: super::CornerRadius) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetCornerRadius)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn IContentControlFactory<R, F: FnOnce(&IContentControlFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<ContentControl, IContentControlFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn IContentControlStatics<R, F: FnOnce(&IContentControlStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<ContentControl, IContentControlStatics> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for ContentControl {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContentControl>();
                }
                unsafe impl windows_core::Interface for ContentControl {
                    type Vtable = <IContentControl as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IContentControl as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for ContentControl {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ContentControl";
                }
                unsafe impl Send for ContentControl {}
                unsafe impl Sync for ContentControl {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct Control(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(Control, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    Control,
                    super::super::Composition::IAnimationObject,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl Control {
                    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetPadding(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetHorizontalContentAlignment(&self, value: super::HorizontalAlignment) -> windows_core::Result<()> {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::Brush>,
                    {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetBackground)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn CornerRadius(&self) -> windows_core::Result<super::CornerRadius> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CornerRadius)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetCornerRadius(&self, value: super::CornerRadius) -> windows_core::Result<()> {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetCornerRadius)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn new() -> windows_core::Result<Control> {
                        Self::IControlFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::ptr::null_mut(),
                                &mut core::ptr::null_mut(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn IControlFactory<R, F: FnOnce(&IControlFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<Control, IControlFactory> = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn IControlStatics<R, F: FnOnce(&IControlStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<Control, IControlStatics> = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for Control {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IControl>();
                }
                unsafe impl windows_core::Interface for Control {
                    type Vtable = <IControl as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IControl as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for Control {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Control";
                }
                unsafe impl Send for Control {}
                unsafe impl Sync for Control {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct Grid(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(Grid, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    Grid,
                    super::super::Composition::IAnimationObject,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    Panel,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl Grid {
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn ColumnDefinitions(&self) -> windows_core::Result<ColumnDefinitionCollection> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).ColumnDefinitions)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn CornerRadius(&self) -> windows_core::Result<super::CornerRadius> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CornerRadius)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetCornerRadius(&self, value: super::CornerRadius) -> windows_core::Result<()> {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetCornerRadius)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetPadding(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetColumnSpacing(&self, value: f64) -> windows_core::Result<()> {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetColumnSpacing)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn new() -> windows_core::Result<Grid> {
                        Self::IGridFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::ptr::null_mut(),
                                &mut core::ptr::null_mut(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    pub fn SetColumn<P0>(element: P0, value: i32) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::FrameworkElement>,
                    {
                        Self::IGridStatics(|this| unsafe {
                            (windows_core::Interface::vtable(this).SetColumn)(
                                windows_core::Interface::as_raw(this),
                                element.param().abi(),
                                value,
                            )
                            .ok()
                        })
                    }
                    pub fn Children(&self) -> windows_core::Result<UIElementCollection> {
                        let this = &windows_core::Interface::cast::<IPanel>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Children)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::Brush>,
                    {
                        let this = &windows_core::Interface::cast::<IPanel>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetBackground)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn IGridFactory<R, F: FnOnce(&IGridFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<Grid, IGridFactory> = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn IGridStatics<R, F: FnOnce(&IGridStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<Grid, IGridStatics> = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for Grid {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGrid>();
                }
                unsafe impl windows_core::Interface for Grid {
                    type Vtable = <IGrid as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IGrid as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for Grid {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Grid";
                }
                unsafe impl Send for Grid {}
                unsafe impl Sync for Grid {}
                windows_core::imp::define_interface!(IBorder, IBorder_Vtbl, 0x1ca13b47_ff5c_5abc_a411_a177df9482a9);
                impl windows_core::RuntimeType for IBorder {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBorder_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub BorderBrush:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetBorderBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub BorderThickness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Thickness) -> windows_core::HRESULT,
                    pub SetBorderThickness: unsafe extern "system" fn(*mut core::ffi::c_void, super::Thickness) -> windows_core::HRESULT,
                    pub Background: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetBackground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    BackgroundSizing: usize,
                    SetBackgroundSizing: usize,
                    pub CornerRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CornerRadius) -> windows_core::HRESULT,
                    pub SetCornerRadius: unsafe extern "system" fn(*mut core::ffi::c_void, super::CornerRadius) -> windows_core::HRESULT,
                    pub Padding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Thickness) -> windows_core::HRESULT,
                    pub SetPadding: unsafe extern "system" fn(*mut core::ffi::c_void, super::Thickness) -> windows_core::HRESULT,
                    pub Child: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    ChildTransitions: usize,
                    SetChildTransitions: usize,
                    BackgroundTransition: usize,
                    SetBackgroundTransition: usize,
                }
                windows_core::imp::define_interface!(IBorderStatics, IBorderStatics_Vtbl, 0x9385cd66_c178_5176_b4a5_160e00e93935);
                impl windows_core::RuntimeType for IBorderStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBorderStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub BorderBrushProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub BorderThicknessProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub BackgroundProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub BackgroundSizingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CornerRadiusProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub PaddingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ChildTransitionsProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IButton, IButton_Vtbl, 0x216c183d_d07a_5aa5_b8a4_0300a2683e87);
                impl windows_core::RuntimeType for IButton {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IButton_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    Flyout: usize,
                    SetFlyout: usize,
                }
                windows_core::imp::define_interface!(IButtonFactory, IButtonFactory_Vtbl, 0xfe393422_d91c_57b1_9a9c_2c7e3f41f77c);
                impl windows_core::RuntimeType for IButtonFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IButtonFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IButtonStatics, IButtonStatics_Vtbl, 0x57823d25_b26a_5e0f_94f6_bbae70683dc5);
                impl windows_core::RuntimeType for IButtonStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IButtonStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub FlyoutProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IColumnDefinition, IColumnDefinition_Vtbl, 0x454cea14_87ec_5890_bb62_f1d82a94758e);
                impl windows_core::RuntimeType for IColumnDefinition {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IColumnDefinition_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::GridLength) -> windows_core::HRESULT,
                    pub SetWidth: unsafe extern "system" fn(*mut core::ffi::c_void, super::GridLength) -> windows_core::HRESULT,
                    pub MaxWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub SetMaxWidth: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                    pub MinWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub SetMinWidth: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                    pub ActualWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IColumnDefinitionStatics,
                    IColumnDefinitionStatics_Vtbl,
                    0xaba4b755_ca63_5ed7_ae31_45f9f91920b0
                );
                impl windows_core::RuntimeType for IColumnDefinitionStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IColumnDefinitionStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub WidthProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MaxWidthProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MinWidthProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IComboBox, IComboBox_Vtbl, 0xc77da58b_4fd7_51e0_a431_f84658a83e9e);
                impl windows_core::RuntimeType for IComboBox {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IComboBox_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsDropDownOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsDropDownOpen: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsEditable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsEditable: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsSelectionBoxHighlighted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub MaxDropDownHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub SetMaxDropDownHeight: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                    pub SelectionBoxItem:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    SelectionBoxItemTemplate: usize,
                    TemplateSettings: usize,
                    pub Header: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    HeaderTemplate: usize,
                    SetHeaderTemplate: usize,
                    pub PlaceholderText:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetPlaceholderText:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    LightDismissOverlayMode: usize,
                    SetLightDismissOverlayMode: usize,
                    pub IsTextSearchEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsTextSearchEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    SelectionChangedTrigger: usize,
                    SetSelectionChangedTrigger: usize,
                    pub PlaceholderForeground:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetPlaceholderForeground:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    TextBoxStyle: usize,
                    SetTextBoxStyle: usize,
                    pub Description:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub DropDownClosed:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                    pub RemoveDropDownClosed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    pub DropDownOpened:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                    pub RemoveDropDownOpened: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    TextSubmitted: usize,
                    pub RemoveTextSubmitted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IComboBoxFactory, IComboBoxFactory_Vtbl, 0x71c1014b_acdf_5c03_b5ed_02871caaeb6b);
                impl windows_core::RuntimeType for IComboBoxFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IComboBoxFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IComboBoxOverrides, IComboBoxOverrides_Vtbl, 0xa6afbc7a_284f_5b9a_89fe_a334b613f895);
                impl windows_core::RuntimeType for IComboBoxOverrides {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IComboBoxOverrides_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub OnDropDownClosed:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub OnDropDownOpened:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IComboBoxStatics, IComboBoxStatics_Vtbl, 0x7da7cd09_e256_5238_9fed_02b2f3afba25);
                impl windows_core::RuntimeType for IComboBoxStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IComboBoxStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsEditableProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsDropDownOpenProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MaxDropDownHeightProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub HeaderProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub HeaderTemplateProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub PlaceholderTextProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub LightDismissOverlayModeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsTextSearchEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SelectionChangedTriggerProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub PlaceholderForegroundProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TextProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TextBoxStyleProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub DescriptionProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IContentControl, IContentControl_Vtbl, 0x07e81761_11b2_52ae_8f8b_4d53d2b5900a);
                impl windows_core::RuntimeType for IContentControl {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IContentControl_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    ContentTemplate: usize,
                    SetContentTemplate: usize,
                    ContentTemplateSelector: usize,
                    SetContentTemplateSelector: usize,
                    ContentTransitions: usize,
                    SetContentTransitions: usize,
                    pub ContentTemplateRoot:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IContentControlFactory,
                    IContentControlFactory_Vtbl,
                    0x3dea958e_5acd_5f80_8938_38634f51493a
                );
                impl windows_core::RuntimeType for IContentControlFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IContentControlFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IContentControlOverrides,
                    IContentControlOverrides_Vtbl,
                    0x2504174a_017e_5a2d_9c28_d97c66ae9937
                );
                impl windows_core::RuntimeType for IContentControlOverrides {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IContentControlOverrides_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub OnContentChanged: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    OnContentTemplateChanged: usize,
                    OnContentTemplateSelectorChanged: usize,
                }
                windows_core::imp::define_interface!(
                    IContentControlStatics,
                    IContentControlStatics_Vtbl,
                    0xf25484f4_2fed_5a0a_8864_7d6d4ac43ef8
                );
                impl windows_core::RuntimeType for IContentControlStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IContentControlStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub ContentProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ContentTemplateProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ContentTemplateSelectorProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ContentTransitionsProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IControl, IControl_Vtbl, 0x857d6e8a_d45a_5c69_a99c_bf6a5c54fb38);
                impl windows_core::RuntimeType for IControl {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IControl_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsFocusEngagementEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsFocusEngagementEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsFocusEngaged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsFocusEngaged: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    RequiresPointer: usize,
                    SetRequiresPointer: usize,
                    pub FontSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub SetFontSize: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                    FontFamily: usize,
                    SetFontFamily: usize,
                    pub FontWeight:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::FontWeight) -> windows_core::HRESULT,
                    pub SetFontWeight:
                        unsafe extern "system" fn(*mut core::ffi::c_void, windows::UI::Text::FontWeight) -> windows_core::HRESULT,
                    pub FontStyle:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::FontStyle) -> windows_core::HRESULT,
                    pub SetFontStyle:
                        unsafe extern "system" fn(*mut core::ffi::c_void, windows::UI::Text::FontStyle) -> windows_core::HRESULT,
                    pub FontStretch:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::FontStretch) -> windows_core::HRESULT,
                    pub SetFontStretch:
                        unsafe extern "system" fn(*mut core::ffi::c_void, windows::UI::Text::FontStretch) -> windows_core::HRESULT,
                    pub CharacterSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                    pub SetCharacterSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
                    pub Foreground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetForeground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsTextScaleFactorEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsTextScaleFactorEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    TabNavigation: usize,
                    SetTabNavigation: usize,
                    Template: usize,
                    SetTemplate: usize,
                    pub Padding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Thickness) -> windows_core::HRESULT,
                    pub SetPadding: unsafe extern "system" fn(*mut core::ffi::c_void, super::Thickness) -> windows_core::HRESULT,
                    pub HorizontalContentAlignment:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HorizontalAlignment) -> windows_core::HRESULT,
                    pub SetHorizontalContentAlignment:
                        unsafe extern "system" fn(*mut core::ffi::c_void, super::HorizontalAlignment) -> windows_core::HRESULT,
                    pub VerticalContentAlignment:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VerticalAlignment) -> windows_core::HRESULT,
                    pub SetVerticalContentAlignment:
                        unsafe extern "system" fn(*mut core::ffi::c_void, super::VerticalAlignment) -> windows_core::HRESULT,
                    pub Background: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetBackground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    BackgroundSizing: usize,
                    SetBackgroundSizing: usize,
                    pub BorderThickness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Thickness) -> windows_core::HRESULT,
                    pub SetBorderThickness: unsafe extern "system" fn(*mut core::ffi::c_void, super::Thickness) -> windows_core::HRESULT,
                    pub BorderBrush:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetBorderBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub DefaultStyleResourceUri:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetDefaultStyleResourceUri:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    ElementSoundMode: usize,
                    SetElementSoundMode: usize,
                    pub CornerRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CornerRadius) -> windows_core::HRESULT,
                    pub SetCornerRadius: unsafe extern "system" fn(*mut core::ffi::c_void, super::CornerRadius) -> windows_core::HRESULT,
                    FocusEngaged: usize,
                    pub RemoveFocusEngaged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    FocusDisengaged: usize,
                    pub RemoveFocusDisengaged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    IsEnabledChanged: usize,
                    pub RemoveIsEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    pub RemoveFocusEngagement: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ApplyTemplate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IControlFactory, IControlFactory_Vtbl, 0x25159233_9438_5534_aeb9_00eb059cf73f);
                impl windows_core::RuntimeType for IControlFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IControlFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IControlOverrides, IControlOverrides_Vtbl, 0x5f644395_2177_5161_9f87_c6c749c73a03);
                impl windows_core::RuntimeType for IControlOverrides {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IControlOverrides_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    OnPointerEntered: usize,
                    OnPointerPressed: usize,
                    OnPointerMoved: usize,
                    OnPointerReleased: usize,
                    OnPointerExited: usize,
                    OnPointerCaptureLost: usize,
                    OnPointerCanceled: usize,
                    OnPointerWheelChanged: usize,
                    OnTapped: usize,
                    OnDoubleTapped: usize,
                    OnHolding: usize,
                    OnRightTapped: usize,
                    OnManipulationStarting: usize,
                    OnManipulationInertiaStarting: usize,
                    OnManipulationStarted: usize,
                    OnManipulationDelta: usize,
                    OnManipulationCompleted: usize,
                    pub OnKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub OnKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub OnPreviewKeyDown:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub OnPreviewKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub OnGotFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub OnLostFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    OnCharacterReceived: usize,
                    OnDragEnter: usize,
                    OnDragLeave: usize,
                    OnDragOver: usize,
                    OnDrop: usize,
                }
                windows_core::imp::define_interface!(IControlProtected, IControlProtected_Vtbl, 0x5c750339_1789_5095_a214_dd9d0564e579);
                impl windows_core::RuntimeType for IControlProtected {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IControlProtected_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub DefaultStyleKey:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetDefaultStyleKey:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetTemplateChild: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IControlStatics, IControlStatics_Vtbl, 0xc3ae388d_aa36_5e10_acac_98415f47bcc7);
                impl windows_core::RuntimeType for IControlStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IControlStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsFocusEngagementEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsFocusEngagedProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub RequiresPointerProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub FontSizeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub FontFamilyProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub FontWeightProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub FontStyleProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub FontStretchProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CharacterSpacingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ForegroundProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsTextScaleFactorEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TabNavigationProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TemplateProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub PaddingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub HorizontalContentAlignmentProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub VerticalContentAlignmentProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub BackgroundProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub BackgroundSizingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub BorderThicknessProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub BorderBrushProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub DefaultStyleKeyProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub DefaultStyleResourceUriProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ElementSoundModeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CornerRadiusProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsTemplateFocusTargetProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetIsTemplateFocusTarget:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsTemplateFocusTarget:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsTemplateKeyTipTargetProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetIsTemplateKeyTipTarget:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsTemplateKeyTipTarget:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IGrid, IGrid_Vtbl, 0xc4496219_9014_58a1_b4ad_c5044913a5bb);
                impl windows_core::RuntimeType for IGrid {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IGrid_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    RowDefinitions: usize,
                    pub ColumnDefinitions:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    BackgroundSizing: usize,
                    SetBackgroundSizing: usize,
                    pub BorderBrush:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetBorderBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub BorderThickness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Thickness) -> windows_core::HRESULT,
                    pub SetBorderThickness: unsafe extern "system" fn(*mut core::ffi::c_void, super::Thickness) -> windows_core::HRESULT,
                    pub CornerRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CornerRadius) -> windows_core::HRESULT,
                    pub SetCornerRadius: unsafe extern "system" fn(*mut core::ffi::c_void, super::CornerRadius) -> windows_core::HRESULT,
                    pub Padding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Thickness) -> windows_core::HRESULT,
                    pub SetPadding: unsafe extern "system" fn(*mut core::ffi::c_void, super::Thickness) -> windows_core::HRESULT,
                    pub RowSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub SetRowSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                    pub ColumnSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub SetColumnSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IGridFactory, IGridFactory_Vtbl, 0xb16bf561_fc6c_57c6_8ebc_0b06ce4513aa);
                impl windows_core::RuntimeType for IGridFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IGridFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IGridStatics, IGridStatics_Vtbl, 0xef9cf81d_a431_50f4_abf5_3023fe447704);
                impl windows_core::RuntimeType for IGridStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IGridStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub BackgroundSizingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub BorderBrushProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub BorderThicknessProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CornerRadiusProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub PaddingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub RowSpacingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ColumnSpacingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub RowProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetRow:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                    pub SetRow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
                    pub ColumnProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetColumn:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                    pub SetColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
                    pub RowSpanProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetRowSpan:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                    pub SetRowSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
                    pub ColumnSpanProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetColumnSpan:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                    pub SetColumnSpan:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IIconElement, IIconElement_Vtbl, 0x18f69350_279e_50ea_8d23_138e717ed939);
                impl windows_core::RuntimeType for IIconElement {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IIconElement_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Foreground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetForeground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IIconElementFactory, IIconElementFactory_Vtbl, 0x7161eeff_2cc3_59c9_b355_f752ef2030fb);
                impl windows_core::RuntimeType for IIconElementFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IIconElementFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                }
                windows_core::imp::define_interface!(IIconElementStatics, IIconElementStatics_Vtbl, 0x4ccfdc81_9d9d_5a02_b95b_ad1add921750);
                impl windows_core::RuntimeType for IIconElementStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IIconElementStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub ForegroundProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IImage, IImage_Vtbl, 0x220d3d8d_66de_53a1_a215_ba9c165565ab);
                impl windows_core::RuntimeType for IImage {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IImage_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    Stretch: usize,
                    SetStretch: usize,
                    pub NineGrid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Thickness) -> windows_core::HRESULT,
                    pub SetNineGrid: unsafe extern "system" fn(*mut core::ffi::c_void, super::Thickness) -> windows_core::HRESULT,
                    ImageFailed: usize,
                    pub RemoveImageFailed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    pub ImageOpened:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                    pub RemoveImageOpened: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    pub GetAsCastingSource:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetAlphaMask: usize,
                }
                windows_core::imp::define_interface!(IImageIcon, IImageIcon_Vtbl, 0x78a7b526_e635_59c6_93a1_d7e3c2fac6d5);
                impl windows_core::RuntimeType for IImageIcon {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IImageIcon_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IImageIconFactory, IImageIconFactory_Vtbl, 0x235e0279_a7d0_5fda_a308_9b7cb9c4c912);
                impl windows_core::RuntimeType for IImageIconFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IImageIconFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IImageIconStatics, IImageIconStatics_Vtbl, 0x6bd31828_f8f9_5d86_80d5_e7423546da7d);
                impl windows_core::RuntimeType for IImageIconStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IImageIconStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub SourceProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IImageStatics, IImageStatics_Vtbl, 0xcb5378a8_996b_547e_9c4c_7bbfd9dbe7dc);
                impl windows_core::RuntimeType for IImageStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IImageStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub SourceProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub StretchProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub NineGridProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IInfoBar, IInfoBar_Vtbl, 0xc1c3a438_dd79_5d22_9e42_5a3cdf8113a9);
                impl windows_core::RuntimeType for IInfoBar {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IInfoBar_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsOpen: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub Severity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InfoBarSeverity) -> windows_core::HRESULT,
                    pub SetSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, InfoBarSeverity) -> windows_core::HRESULT,
                    IconSource: usize,
                    SetIconSource: usize,
                    pub IsIconVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsIconVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsClosable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsClosable: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    CloseButtonStyle: usize,
                    SetCloseButtonStyle: usize,
                    CloseButtonCommand: usize,
                    SetCloseButtonCommand: usize,
                    pub CloseButtonCommandParameter:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetCloseButtonCommandParameter:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ActionButton:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetActionButton: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    ContentTemplate: usize,
                    SetContentTemplate: usize,
                    TemplateSettings: usize,
                    pub CloseButtonClick:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                    pub RemoveCloseButtonClick: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    Closing: usize,
                    pub RemoveClosing: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    Closed: usize,
                    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IInfoBarFactory, IInfoBarFactory_Vtbl, 0x60618a60_9be7_5df5_be0d_933d34ddb44c);
                impl windows_core::RuntimeType for IInfoBarFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IInfoBarFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IInfoBarStatics, IInfoBarStatics_Vtbl, 0x5fb1e80d_ea5d_57ba_9915_b6782b982aca);
                impl windows_core::RuntimeType for IInfoBarStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IInfoBarStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsOpenProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TitleProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MessageProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SeverityProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IconSourceProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsIconVisibleProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsClosableProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CloseButtonStyleProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CloseButtonCommandProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CloseButtonCommandParameterProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ActionButtonProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ContentProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ContentTemplateProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TemplateSettingsProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IInsertionPanel, IInsertionPanel_Vtbl, 0x84e13e27_2d24_59c4_a00e_16c7255901e2);
                impl windows_core::RuntimeType for IInsertionPanel {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                windows_core::imp::interface_hierarchy!(IInsertionPanel, windows_core::IUnknown, windows_core::IInspectable);
                impl IInsertionPanel {
                }
                impl windows_core::RuntimeName for IInsertionPanel {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IInsertionPanel";
                }
                pub trait IInsertionPanel_Impl: windows_core::IUnknownImpl {
                    fn GetInsertionIndexes(
                        &self,
                        position: &windows::Foundation::Point,
                        first: &mut i32,
                        second: &mut i32,
                    ) -> windows_core::Result<()>;
                }
                impl IInsertionPanel_Vtbl {
                    pub const fn new<Identity: IInsertionPanel_Impl, const OFFSET: isize>() -> Self {
                        unsafe extern "system" fn GetInsertionIndexes<Identity: IInsertionPanel_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            position: windows::Foundation::Point,
                            first: *mut i32,
                            second: *mut i32,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                IInsertionPanel_Impl::GetInsertionIndexes(
                                    this,
                                    core::mem::transmute(&position),
                                    core::mem::transmute_copy(&first),
                                    core::mem::transmute_copy(&second),
                                )
                                .into()
                            }
                        }
                        Self {
                            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInsertionPanel, OFFSET>(),
                            GetInsertionIndexes: GetInsertionIndexes::<Identity, OFFSET>,
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<IInsertionPanel as windows_core::Interface>::IID
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IInsertionPanel_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub GetInsertionIndexes: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        windows::Foundation::Point,
                        *mut i32,
                        *mut i32,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IItemContainerMapping,
                    IItemContainerMapping_Vtbl,
                    0xb3e03a32_ea83_5626_bfda_d327a1c7550c
                );
                impl windows_core::RuntimeType for IItemContainerMapping {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                windows_core::imp::interface_hierarchy!(IItemContainerMapping, windows_core::IUnknown, windows_core::IInspectable);
                impl IItemContainerMapping {
                }
                impl windows_core::RuntimeName for IItemContainerMapping {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IItemContainerMapping";
                }
                pub trait IItemContainerMapping_Impl: windows_core::IUnknownImpl {
                    fn ItemFromContainer(
                        &self,
                        container: windows_core::Ref<super::DependencyObject>,
                    ) -> windows_core::Result<windows_core::IInspectable>;
                    fn ContainerFromItem(
                        &self,
                        item: windows_core::Ref<windows_core::IInspectable>,
                    ) -> windows_core::Result<super::DependencyObject>;
                    fn IndexFromContainer(&self, container: windows_core::Ref<super::DependencyObject>) -> windows_core::Result<i32>;
                    fn ContainerFromIndex(&self, index: i32) -> windows_core::Result<super::DependencyObject>;
                }
                impl IItemContainerMapping_Vtbl {
                    pub const fn new<Identity: IItemContainerMapping_Impl, const OFFSET: isize>() -> Self {
                        unsafe extern "system" fn ItemFromContainer<Identity: IItemContainerMapping_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            container: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IItemContainerMapping_Impl::ItemFromContainer(this, core::mem::transmute_copy(&container)) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        core::mem::forget(ok__);
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn ContainerFromItem<Identity: IItemContainerMapping_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            item: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IItemContainerMapping_Impl::ContainerFromItem(this, core::mem::transmute_copy(&item)) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        core::mem::forget(ok__);
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn IndexFromContainer<Identity: IItemContainerMapping_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            container: *mut core::ffi::c_void,
                            result__: *mut i32,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IItemContainerMapping_Impl::IndexFromContainer(this, core::mem::transmute_copy(&container)) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn ContainerFromIndex<Identity: IItemContainerMapping_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            index: i32,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IItemContainerMapping_Impl::ContainerFromIndex(this, index) {
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
                            base__: windows_core::IInspectable_Vtbl::new::<Identity, IItemContainerMapping, OFFSET>(),
                            ItemFromContainer: ItemFromContainer::<Identity, OFFSET>,
                            ContainerFromItem: ContainerFromItem::<Identity, OFFSET>,
                            IndexFromContainer: IndexFromContainer::<Identity, OFFSET>,
                            ContainerFromIndex: ContainerFromIndex::<Identity, OFFSET>,
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<IItemContainerMapping as windows_core::Interface>::IID
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IItemContainerMapping_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub ItemFromContainer: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub ContainerFromItem: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub IndexFromContainer:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                    pub ContainerFromIndex:
                        unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IItemsControl, IItemsControl_Vtbl, 0xbf1ccb54_83e2_5b98_acbc_736f876c3d35);
                impl windows_core::RuntimeType for IItemsControl {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IItemsControl_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub ItemsSource:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetItemsSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub Items: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    ItemTemplate: usize,
                    SetItemTemplate: usize,
                    ItemTemplateSelector: usize,
                    SetItemTemplateSelector: usize,
                    ItemsPanel: usize,
                    SetItemsPanel: usize,
                    pub DisplayMemberPath:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetDisplayMemberPath:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ItemsPanelRoot:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    ItemContainerStyle: usize,
                    SetItemContainerStyle: usize,
                    ItemContainerStyleSelector: usize,
                    SetItemContainerStyleSelector: usize,
                    ItemContainerGenerator: usize,
                    ItemContainerTransitions: usize,
                    SetItemContainerTransitions: usize,
                    GroupStyle: usize,
                    GroupStyleSelector: usize,
                    SetGroupStyleSelector: usize,
                    pub IsGrouping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub GroupHeaderContainerFromItemContainer: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IItemsControlFactory,
                    IItemsControlFactory_Vtbl,
                    0xb2686273_2dd3_5454_9f25_247e66270336
                );
                impl windows_core::RuntimeType for IItemsControlFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IItemsControlFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IItemsControlOverrides,
                    IItemsControlOverrides_Vtbl,
                    0x285c215f_64b2_5f7d_88d2_56c6658f73ee
                );
                impl windows_core::RuntimeType for IItemsControlOverrides {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IItemsControlOverrides_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsItemItsOwnContainerOverride:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub GetContainerForItemOverride:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ClearContainerForItemOverride: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub PrepareContainerForItemOverride: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub OnItemsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    OnItemContainerStyleChanged: usize,
                    OnItemContainerStyleSelectorChanged: usize,
                    OnItemTemplateChanged: usize,
                    OnItemTemplateSelectorChanged: usize,
                    OnGroupStyleSelectorChanged: usize,
                }
                windows_core::imp::define_interface!(
                    IItemsControlStatics,
                    IItemsControlStatics_Vtbl,
                    0x4fb961e1_5f91_5585_a692_6825cde3ed1e
                );
                impl windows_core::RuntimeType for IItemsControlStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IItemsControlStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub ItemsSourceProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ItemTemplateProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ItemTemplateSelectorProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ItemsPanelProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub DisplayMemberPathProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ItemContainerStyleProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ItemContainerStyleSelectorProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ItemContainerTransitionsProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GroupStyleSelectorProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsGroupingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetItemsOwner: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub ItemsControlFromItemContainer: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(INavigationView, INavigationView_Vtbl, 0xe77a4b36_3dd1_53d9_9f97_65dccaa74a5c);
                impl windows_core::RuntimeType for INavigationView {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationView_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsPaneOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsPaneOpen: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub CompactModeThresholdWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub SetCompactModeThresholdWidth: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                    pub ExpandedModeThresholdWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub SetExpandedModeThresholdWidth: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                    pub FooterMenuItems:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub FooterMenuItemsSource:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetFooterMenuItemsSource:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub PaneFooter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetPaneFooter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub Header: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    HeaderTemplate: usize,
                    SetHeaderTemplate: usize,
                    DisplayMode: usize,
                    pub IsSettingsVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsSettingsVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsPaneToggleButtonVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsPaneToggleButtonVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub AlwaysShowHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetAlwaysShowHeader: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub CompactPaneLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub SetCompactPaneLength: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                    pub OpenPaneLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub SetOpenPaneLength: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                    PaneToggleButtonStyle: usize,
                    SetPaneToggleButtonStyle: usize,
                    pub SelectedItem:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MenuItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MenuItemsSource:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetMenuItemsSource:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SettingsItem:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    AutoSuggestBox: usize,
                    SetAutoSuggestBox: usize,
                    MenuItemTemplate: usize,
                    SetMenuItemTemplate: usize,
                    MenuItemTemplateSelector: usize,
                    SetMenuItemTemplateSelector: usize,
                    MenuItemContainerStyle: usize,
                    SetMenuItemContainerStyle: usize,
                    MenuItemContainerStyleSelector: usize,
                    SetMenuItemContainerStyleSelector: usize,
                    pub MenuItemFromContainer: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub ContainerFromMenuItem: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub SelectionChanged:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                    pub RemoveSelectionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    ItemInvoked: usize,
                    pub RemoveItemInvoked: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    DisplayModeChanged: usize,
                    pub RemoveDisplayModeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    pub IsTitleBarAutoPaddingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsTitleBarAutoPaddingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(INavigationView2, INavigationView2_Vtbl, 0x05b428cf_014c_56dd_896a_a3e7089d73b5);
                impl windows_core::RuntimeType for INavigationView2 {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationView2_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsBackButtonVisible:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut NavigationViewBackButtonVisible) -> windows_core::HRESULT,
                    pub SetIsBackButtonVisible:
                        unsafe extern "system" fn(*mut core::ffi::c_void, NavigationViewBackButtonVisible) -> windows_core::HRESULT,
                    pub IsBackEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsBackEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub PaneTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetPaneTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    BackRequested: usize,
                    pub RemoveBackRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    pub PaneClosed:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                    pub RemovePaneClosed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    PaneClosing: usize,
                    pub RemovePaneClosing: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    pub PaneOpened:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                    pub RemovePaneOpened: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    pub PaneOpening:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                    pub RemovePaneOpening: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    pub PaneDisplayMode:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut NavigationViewPaneDisplayMode) -> windows_core::HRESULT,
                    pub SetPaneDisplayMode:
                        unsafe extern "system" fn(*mut core::ffi::c_void, NavigationViewPaneDisplayMode) -> windows_core::HRESULT,
                    pub PaneHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetPaneHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub PaneCustomContent:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetPaneCustomContent:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ContentOverlay:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetContentOverlay:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsPaneVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsPaneVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    SelectionFollowsFocus: usize,
                    SetSelectionFollowsFocus: usize,
                    TemplateSettings: usize,
                    ShoulderNavigationEnabled: usize,
                    SetShoulderNavigationEnabled: usize,
                    OverflowLabelMode: usize,
                    SetOverflowLabelMode: usize,
                    Expanding: usize,
                    pub RemoveExpanding: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    Collapsed: usize,
                    pub RemoveCollapsed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub Collapse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    INavigationViewFactory,
                    INavigationViewFactory_Vtbl,
                    0xffea1ada_9232_5507_a320_ed2fadbe6127
                );
                impl windows_core::RuntimeType for INavigationViewFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(INavigationViewItem, INavigationViewItem_Vtbl, 0x3ab3d503_a37c_5836_8adb_2882062e73a1);
                impl windows_core::RuntimeType for INavigationViewItem {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewItem_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Icon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetIcon: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CompactPaneLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    INavigationViewItem2,
                    INavigationViewItem2_Vtbl,
                    0x2d5bd889_9dac_5675_b254_68226f077a61
                );
                impl windows_core::RuntimeType for INavigationViewItem2 {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewItem2_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub SelectsOnInvoked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetSelectsOnInvoked: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsExpanded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsExpanded: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub HasUnrealizedChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetHasUnrealizedChildren: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsChildSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsChildSelected: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub MenuItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MenuItemsSource:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetMenuItemsSource:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    INavigationViewItem3,
                    INavigationViewItem3_Vtbl,
                    0xc6aa3120_d888_5c32_8bb7_490ec91b1aec
                );
                impl windows_core::RuntimeType for INavigationViewItem3 {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewItem3_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    InfoBadge: usize,
                    SetInfoBadge: usize,
                }
                windows_core::imp::define_interface!(
                    INavigationViewItemBase,
                    INavigationViewItemBase_Vtbl,
                    0x33586494_af48_513f_be4d_f645e8c89005
                );
                impl windows_core::RuntimeType for INavigationViewItemBase {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewItemBase_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                }
                windows_core::imp::define_interface!(
                    INavigationViewItemBase2,
                    INavigationViewItemBase2_Vtbl,
                    0xd94ee683_d437_5523_9c5c_11d4804e471e
                );
                impl windows_core::RuntimeType for INavigationViewItemBase2 {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewItemBase2_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    INavigationViewItemBaseFactory,
                    INavigationViewItemBaseFactory_Vtbl,
                    0x31b9d7b1_7c38_5916_99c6_c71f6b012b1b
                );
                impl windows_core::RuntimeType for INavigationViewItemBaseFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewItemBaseFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                }
                windows_core::imp::define_interface!(
                    INavigationViewItemBaseStatics,
                    INavigationViewItemBaseStatics_Vtbl,
                    0x71a0c438_f0c2_5093_8906_c1c3c02de791
                );
                impl windows_core::RuntimeType for INavigationViewItemBaseStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewItemBaseStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsSelectedProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    INavigationViewItemFactory,
                    INavigationViewItemFactory_Vtbl,
                    0xde60a001_9385_5535_80e1_2b68f4bfde26
                );
                impl windows_core::RuntimeType for INavigationViewItemFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewItemFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    INavigationViewItemHeader,
                    INavigationViewItemHeader_Vtbl,
                    0x432bc062_45bc_57ef_a2d3_11851a56a882
                );
                impl windows_core::RuntimeType for INavigationViewItemHeader {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewItemHeader_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                }
                windows_core::imp::define_interface!(
                    INavigationViewItemHeaderFactory,
                    INavigationViewItemHeaderFactory_Vtbl,
                    0x6a5447cd_2918_5fe3_899b_93d6961285e6
                );
                impl windows_core::RuntimeType for INavigationViewItemHeaderFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewItemHeaderFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    INavigationViewItemStatics,
                    INavigationViewItemStatics_Vtbl,
                    0x7b6198e5_0714_531c_a056_21b3ca40ec1a
                );
                impl windows_core::RuntimeType for INavigationViewItemStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewItemStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IconProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CompactPaneLengthProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    INavigationViewItemStatics2,
                    INavigationViewItemStatics2_Vtbl,
                    0xd49d016f_abd0_51f3_acf0_118ecceea760
                );
                impl windows_core::RuntimeType for INavigationViewItemStatics2 {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewItemStatics2_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub SelectsOnInvokedProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsExpandedProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub HasUnrealizedChildrenProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsChildSelectedProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MenuItemsProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MenuItemsSourceProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    INavigationViewItemStatics3,
                    INavigationViewItemStatics3_Vtbl,
                    0x957beb66_d33a_53aa_a518_7d426bb9b177
                );
                impl windows_core::RuntimeType for INavigationViewItemStatics3 {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewItemStatics3_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub InfoBadgeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    INavigationViewSelectionChangedEventArgs,
                    INavigationViewSelectionChangedEventArgs_Vtbl,
                    0x14a064a5_c79d_5f63_ac6e_1c313fe63566
                );
                impl windows_core::RuntimeType for INavigationViewSelectionChangedEventArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewSelectionChangedEventArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub SelectedItem:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsSettingsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    INavigationViewSelectionChangedEventArgs2,
                    INavigationViewSelectionChangedEventArgs2_Vtbl,
                    0xe9b5191f_1ad1_5366_bc1a_90b96076d49e
                );
                impl windows_core::RuntimeType for INavigationViewSelectionChangedEventArgs2 {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewSelectionChangedEventArgs2_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub SelectedItemContainer:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    RecommendedNavigationTransitionInfo: usize,
                }
                windows_core::imp::define_interface!(
                    INavigationViewStatics,
                    INavigationViewStatics_Vtbl,
                    0xdcd04caf_1904_564b_b0de_babaff9962f5
                );
                impl windows_core::RuntimeType for INavigationViewStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsPaneOpenProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CompactModeThresholdWidthProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ExpandedModeThresholdWidthProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub FooterMenuItemsProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub FooterMenuItemsSourceProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub PaneFooterProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub HeaderProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub HeaderTemplateProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub DisplayModeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsSettingsVisibleProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsPaneToggleButtonVisibleProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub AlwaysShowHeaderProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CompactPaneLengthProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub OpenPaneLengthProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub PaneToggleButtonStyleProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MenuItemsProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MenuItemsSourceProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SelectedItemProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SettingsItemProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub AutoSuggestBoxProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MenuItemTemplateProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MenuItemTemplateSelectorProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MenuItemContainerStyleProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MenuItemContainerStyleSelectorProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsTitleBarAutoPaddingEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    INavigationViewStatics2,
                    INavigationViewStatics2_Vtbl,
                    0x79588150_4a1b_5e7c_9f8a_ebc814ce773d
                );
                impl windows_core::RuntimeType for INavigationViewStatics2 {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct INavigationViewStatics2_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsBackButtonVisibleProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsBackEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub PaneTitleProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub PaneDisplayModeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub PaneHeaderProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub PaneCustomContentProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ContentOverlayProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsPaneVisibleProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SelectionFollowsFocusProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TemplateSettingsProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ShoulderNavigationEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub OverflowLabelModeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IPanel, IPanel_Vtbl, 0x27a1b418_56f3_525e_b883_cefed905eed3);
                impl windows_core::RuntimeType for IPanel {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IPanel_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Children: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub Background: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetBackground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsItemsHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    ChildrenTransitions: usize,
                    SetChildrenTransitions: usize,
                    BackgroundTransition: usize,
                    SetBackgroundTransition: usize,
                }
                windows_core::imp::define_interface!(IPanelFactory, IPanelFactory_Vtbl, 0xf5e7e21c_4c97_5d20_bee6_3e4fc6ab14e9);
                impl windows_core::RuntimeType for IPanelFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IPanelFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IPanelStatics, IPanelStatics_Vtbl, 0x76a9caa7_a5d4_5061_a325_17c76f66de51);
                impl windows_core::RuntimeType for IPanelStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IPanelStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub BackgroundProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsItemsHostProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ChildrenTransitionsProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IScrollAnchorProvider,
                    IScrollAnchorProvider_Vtbl,
                    0x885df41f_f240_5b7b_a703_bfaf39c6a2cd
                );
                impl windows_core::RuntimeType for IScrollAnchorProvider {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                windows_core::imp::interface_hierarchy!(IScrollAnchorProvider, windows_core::IUnknown, windows_core::IInspectable);
                impl IScrollAnchorProvider {
                }
                impl windows_core::RuntimeName for IScrollAnchorProvider {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IScrollAnchorProvider";
                }
                pub trait IScrollAnchorProvider_Impl: windows_core::IUnknownImpl {
                    fn CurrentAnchor(&self) -> windows_core::Result<super::UIElement>;
                    fn RegisterAnchorCandidate(&self, element: windows_core::Ref<super::UIElement>) -> windows_core::Result<()>;
                    fn UnregisterAnchorCandidate(&self, element: windows_core::Ref<super::UIElement>) -> windows_core::Result<()>;
                }
                impl IScrollAnchorProvider_Vtbl {
                    pub const fn new<Identity: IScrollAnchorProvider_Impl, const OFFSET: isize>() -> Self {
                        unsafe extern "system" fn CurrentAnchor<Identity: IScrollAnchorProvider_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IScrollAnchorProvider_Impl::CurrentAnchor(this) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        core::mem::forget(ok__);
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn RegisterAnchorCandidate<Identity: IScrollAnchorProvider_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            element: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                IScrollAnchorProvider_Impl::RegisterAnchorCandidate(this, core::mem::transmute_copy(&element)).into()
                            }
                        }
                        unsafe extern "system" fn UnregisterAnchorCandidate<Identity: IScrollAnchorProvider_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            element: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                IScrollAnchorProvider_Impl::UnregisterAnchorCandidate(this, core::mem::transmute_copy(&element)).into()
                            }
                        }
                        Self {
                            base__: windows_core::IInspectable_Vtbl::new::<Identity, IScrollAnchorProvider, OFFSET>(),
                            CurrentAnchor: CurrentAnchor::<Identity, OFFSET>,
                            RegisterAnchorCandidate: RegisterAnchorCandidate::<Identity, OFFSET>,
                            UnregisterAnchorCandidate: UnregisterAnchorCandidate::<Identity, OFFSET>,
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<IScrollAnchorProvider as windows_core::Interface>::IID
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IScrollAnchorProvider_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CurrentAnchor:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub RegisterAnchorCandidate:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub UnregisterAnchorCandidate:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IScrollViewer, IScrollViewer_Vtbl, 0x1dc28c2e_996c_5394_89c3_4dc656b4ad46);
                impl windows_core::RuntimeType for IScrollViewer {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IScrollViewer_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    HorizontalScrollBarVisibility: usize,
                    SetHorizontalScrollBarVisibility: usize,
                    VerticalScrollBarVisibility: usize,
                    SetVerticalScrollBarVisibility: usize,
                    pub IsHorizontalRailEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsHorizontalRailEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsVerticalRailEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsVerticalRailEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsHorizontalScrollChainingEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsHorizontalScrollChainingEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsVerticalScrollChainingEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsVerticalScrollChainingEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsZoomChainingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsZoomChainingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsScrollInertiaEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsScrollInertiaEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsZoomInertiaEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsZoomInertiaEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    HorizontalScrollMode: usize,
                    SetHorizontalScrollMode: usize,
                    VerticalScrollMode: usize,
                    SetVerticalScrollMode: usize,
                    ZoomMode: usize,
                    SetZoomMode: usize,
                    HorizontalSnapPointsAlignment: usize,
                    SetHorizontalSnapPointsAlignment: usize,
                    VerticalSnapPointsAlignment: usize,
                    SetVerticalSnapPointsAlignment: usize,
                    HorizontalSnapPointsType: usize,
                    SetHorizontalSnapPointsType: usize,
                    VerticalSnapPointsType: usize,
                    SetVerticalSnapPointsType: usize,
                    ZoomSnapPointsType: usize,
                    SetZoomSnapPointsType: usize,
                    pub HorizontalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub ViewportWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub ScrollableWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    ComputedHorizontalScrollBarVisibility: usize,
                    pub ExtentWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub VerticalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub ViewportHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub ScrollableHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    ComputedVerticalScrollBarVisibility: usize,
                    pub ExtentHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub MinZoomFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
                    pub SetMinZoomFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
                    pub MaxZoomFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
                    pub SetMaxZoomFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
                    pub ZoomFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
                    pub ZoomSnapPoints:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TopLeftHeader:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetTopLeftHeader:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub LeftHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetLeftHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TopHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetTopHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ReduceViewportForCoreInputViewOcclusions:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetReduceViewportForCoreInputViewOcclusions:
                        unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub HorizontalAnchorRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub SetHorizontalAnchorRatio: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                    pub VerticalAnchorRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub SetVerticalAnchorRatio: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                    pub CanContentRenderOutsideBounds:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetCanContentRenderOutsideBounds: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    AnchorRequested: usize,
                    pub RemoveAnchorRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    ViewChanging: usize,
                    pub RemoveViewChanging: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    ViewChanged: usize,
                    pub RemoveViewChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    pub DirectManipulationStarted:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                    pub RemoveDirectManipulationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    pub DirectManipulationCompleted:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                    pub RemoveDirectManipulationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    pub ScrollToHorizontalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                    pub ScrollToVerticalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                    pub ZoomToFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
                    pub ChangeView: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut bool,
                    ) -> windows_core::HRESULT,
                    pub ChangeViewWithOptionalAnimation: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        bool,
                        *mut bool,
                    ) -> windows_core::HRESULT,
                    pub InvalidateScrollInfo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsDeferredScrollingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsDeferredScrollingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub BringIntoViewOnFocusChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetBringIntoViewOnFocusChange: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IScrollViewerStatics,
                    IScrollViewerStatics_Vtbl,
                    0xd971fd86_0a96_50c4_a6e1_9975faa2a142
                );
                impl windows_core::RuntimeType for IScrollViewerStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IScrollViewerStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub HorizontalSnapPointsAlignmentProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub VerticalSnapPointsAlignmentProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub HorizontalSnapPointsTypeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub VerticalSnapPointsTypeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ZoomSnapPointsTypeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub HorizontalOffsetProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ViewportWidthProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ScrollableWidthProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ComputedHorizontalScrollBarVisibilityProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ExtentWidthProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub VerticalOffsetProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ViewportHeightProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ScrollableHeightProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ComputedVerticalScrollBarVisibilityProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ExtentHeightProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MinZoomFactorProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MaxZoomFactorProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ZoomFactorProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ZoomSnapPointsProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TopLeftHeaderProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub LeftHeaderProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TopHeaderProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ReduceViewportForCoreInputViewOcclusionsProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub HorizontalAnchorRatioProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub VerticalAnchorRatioProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub HorizontalScrollBarVisibilityProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetHorizontalScrollBarVisibility: usize,
                    SetHorizontalScrollBarVisibility: usize,
                    pub VerticalScrollBarVisibilityProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetVerticalScrollBarVisibility: usize,
                    SetVerticalScrollBarVisibility: usize,
                    pub IsHorizontalRailEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetIsHorizontalRailEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsHorizontalRailEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsVerticalRailEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetIsVerticalRailEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsVerticalRailEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsHorizontalScrollChainingEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetIsHorizontalScrollChainingEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsHorizontalScrollChainingEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsVerticalScrollChainingEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetIsVerticalScrollChainingEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsVerticalScrollChainingEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsZoomChainingEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetIsZoomChainingEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsZoomChainingEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsScrollInertiaEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetIsScrollInertiaEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsScrollInertiaEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsZoomInertiaEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetIsZoomInertiaEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsZoomInertiaEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub HorizontalScrollModeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetHorizontalScrollMode: usize,
                    SetHorizontalScrollMode: usize,
                    pub VerticalScrollModeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetVerticalScrollMode: usize,
                    SetVerticalScrollMode: usize,
                    pub ZoomModeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    GetZoomMode: usize,
                    SetZoomMode: usize,
                    pub CanContentRenderOutsideBoundsProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetCanContentRenderOutsideBounds:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetCanContentRenderOutsideBounds:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub IsDeferredScrollingEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetIsDeferredScrollingEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsDeferredScrollingEnabled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub BringIntoViewOnFocusChangeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub GetBringIntoViewOnFocusChange:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetBringIntoViewOnFocusChange:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    ISelectionChangedEventArgs,
                    ISelectionChangedEventArgs_Vtbl,
                    0xb6c18076_4b76_5416_ad29_e2dc20c46246
                );
                impl windows_core::RuntimeType for ISelectionChangedEventArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ISelectionChangedEventArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub AddedItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub RemovedItems:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    ISelectionChangedEventArgsFactory,
                    ISelectionChangedEventArgsFactory_Vtbl,
                    0x0fcd041d_465f_5745_9882_0c8348e4135d
                );
                impl windows_core::RuntimeType for ISelectionChangedEventArgsFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ISelectionChangedEventArgsFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstanceWithRemovedItemsAndAddedItems: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    )
                        -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IStackPanel, IStackPanel_Vtbl, 0x493ab00b_3a6a_5e4a_9452_407cd5197406);
                impl windows_core::RuntimeType for IStackPanel {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IStackPanel_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub AreScrollSnapPointsRegular: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetAreScrollSnapPointsRegular: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    Orientation: usize,
                    SetOrientation: usize,
                    BackgroundSizing: usize,
                    SetBackgroundSizing: usize,
                    pub BorderBrush:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetBorderBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub BorderThickness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Thickness) -> windows_core::HRESULT,
                    pub SetBorderThickness: unsafe extern "system" fn(*mut core::ffi::c_void, super::Thickness) -> windows_core::HRESULT,
                    pub CornerRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CornerRadius) -> windows_core::HRESULT,
                    pub SetCornerRadius: unsafe extern "system" fn(*mut core::ffi::c_void, super::CornerRadius) -> windows_core::HRESULT,
                    pub Padding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Thickness) -> windows_core::HRESULT,
                    pub SetPadding: unsafe extern "system" fn(*mut core::ffi::c_void, super::Thickness) -> windows_core::HRESULT,
                    pub Spacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub SetSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IStackPanelFactory, IStackPanelFactory_Vtbl, 0x64c1d388_47a2_5a74_a75b_559d151ee5ac);
                impl windows_core::RuntimeType for IStackPanelFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IStackPanelFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IStackPanelStatics, IStackPanelStatics_Vtbl, 0x10bb04e3_eb01_5ea8_9f96_69508479def9);
                impl windows_core::RuntimeType for IStackPanelStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IStackPanelStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub AreScrollSnapPointsRegularProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub OrientationProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub BackgroundSizingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub BorderBrushProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub BorderThicknessProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CornerRadiusProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub PaddingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SpacingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(ITextBlock, ITextBlock_Vtbl, 0x1ac8d84f_392c_5c7e_83f5_a53e3bf0abb0);
                impl windows_core::RuntimeType for ITextBlock {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ITextBlock_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub FontSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub SetFontSize: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                    FontFamily: usize,
                    SetFontFamily: usize,
                    pub FontWeight:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::FontWeight) -> windows_core::HRESULT,
                    pub SetFontWeight:
                        unsafe extern "system" fn(*mut core::ffi::c_void, windows::UI::Text::FontWeight) -> windows_core::HRESULT,
                    pub FontStyle:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::FontStyle) -> windows_core::HRESULT,
                    pub SetFontStyle:
                        unsafe extern "system" fn(*mut core::ffi::c_void, windows::UI::Text::FontStyle) -> windows_core::HRESULT,
                    pub FontStretch:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::FontStretch) -> windows_core::HRESULT,
                    pub SetFontStretch:
                        unsafe extern "system" fn(*mut core::ffi::c_void, windows::UI::Text::FontStretch) -> windows_core::HRESULT,
                    pub CharacterSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                    pub SetCharacterSpacing: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
                    pub Foreground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetForeground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TextWrapping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::TextWrapping) -> windows_core::HRESULT,
                    pub SetTextWrapping: unsafe extern "system" fn(*mut core::ffi::c_void, super::TextWrapping) -> windows_core::HRESULT,
                    TextTrimming: usize,
                    SetTextTrimming: usize,
                    TextAlignment: usize,
                    SetTextAlignment: usize,
                    pub Text: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    Inlines: usize,
                    pub Padding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Thickness) -> windows_core::HRESULT,
                    pub SetPadding: unsafe extern "system" fn(*mut core::ffi::c_void, super::Thickness) -> windows_core::HRESULT,
                    pub LineHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub SetLineHeight: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                    LineStackingStrategy: usize,
                    SetLineStackingStrategy: usize,
                    pub IsTextSelectionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsTextSelectionEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub SelectedText:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    ContentStart: usize,
                    ContentEnd: usize,
                    SelectionStart: usize,
                    SelectionEnd: usize,
                    pub BaselineOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub SelectionHighlightColor:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetSelectionHighlightColor:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MaxLines: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                    pub SetMaxLines: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
                    TextLineBounds: usize,
                    SetTextLineBounds: usize,
                    OpticalMarginAlignment: usize,
                    SetOpticalMarginAlignment: usize,
                    pub IsColorFontEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsColorFontEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    TextReadingOrder: usize,
                    SetTextReadingOrder: usize,
                    pub IsTextScaleFactorEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsTextScaleFactorEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub TextDecorations:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Text::TextDecorations) -> windows_core::HRESULT,
                    pub SetTextDecorations:
                        unsafe extern "system" fn(*mut core::ffi::c_void, windows::UI::Text::TextDecorations) -> windows_core::HRESULT,
                    pub IsTextTrimmed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    HorizontalTextAlignment: usize,
                    SetHorizontalTextAlignment: usize,
                    TextHighlighters: usize,
                    SelectionFlyout: usize,
                    SetSelectionFlyout: usize,
                    pub SelectionChanged:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                    pub RemoveSelectionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    ContextMenuOpening: usize,
                    pub RemoveContextMenuOpening: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    IsTextTrimmedChanged: usize,
                    pub RemoveIsTextTrimmedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    pub SelectAll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    Select: usize,
                    GetAlphaMask: usize,
                    pub CopySelectionToClipboard: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(ITextBlockStatics, ITextBlockStatics_Vtbl, 0x3187104b_65c2_5e53_b889_c8272b1314cd);
                impl windows_core::RuntimeType for ITextBlockStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ITextBlockStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub FontSizeProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub FontFamilyProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub FontWeightProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub FontStyleProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub FontStretchProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CharacterSpacingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub ForegroundProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TextWrappingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TextTrimmingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TextAlignmentProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TextProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub PaddingProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub LineHeightProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub LineStackingStrategyProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsTextSelectionEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SelectedTextProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SelectionHighlightColorProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub MaxLinesProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TextLineBoundsProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub OpticalMarginAlignmentProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsColorFontEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TextReadingOrderProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsTextScaleFactorEnabledProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TextDecorationsProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsTextTrimmedProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub HorizontalTextAlignmentProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SelectionFlyoutProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IToggleSwitch, IToggleSwitch_Vtbl, 0x1b17eeb1_74bf_5a83_8161_a86f0fdcdf24);
                impl windows_core::RuntimeType for IToggleSwitch {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IToggleSwitch_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsOn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetIsOn: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub Header: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetHeader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    HeaderTemplate: usize,
                    SetHeaderTemplate: usize,
                    pub OnContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetOnContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    OnContentTemplate: usize,
                    SetOnContentTemplate: usize,
                    pub OffContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub SetOffContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    OffContentTemplate: usize,
                    SetOffContentTemplate: usize,
                    TemplateSettings: usize,
                    pub Toggled:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                    pub RemoveToggled: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IToggleSwitchOverrides,
                    IToggleSwitchOverrides_Vtbl,
                    0x772682db_e3d4_5242_b6de_36705725eeb8
                );
                impl windows_core::RuntimeType for IToggleSwitchOverrides {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IToggleSwitchOverrides_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub OnToggled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub OnOnContentChanged: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub OnOffContentChanged: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub OnHeaderChanged: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IToggleSwitchStatics,
                    IToggleSwitchStatics_Vtbl,
                    0xc9c203d6_0619_504d_9ed8_5054fe3ca51e
                );
                impl windows_core::RuntimeType for IToggleSwitchStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IToggleSwitchStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub IsOnProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub HeaderProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub HeaderTemplateProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub OnContentProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub OnContentTemplateProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub OffContentProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub OffContentTemplateProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IUIElementCollection,
                    IUIElementCollection_Vtbl,
                    0x23050cb1_db88_54ed_9083_5ecfb12512fd
                );
                impl windows_core::RuntimeType for IUIElementCollection {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IUIElementCollection_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IXamlControlsResources,
                    IXamlControlsResources_Vtbl,
                    0x918ca043_f42c_5805_861b_62d6d1d0c162
                );
                impl windows_core::RuntimeType for IXamlControlsResources {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXamlControlsResources_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub UseCompactResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetUseCompactResources: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IXamlControlsResourcesStatics,
                    IXamlControlsResourcesStatics_Vtbl,
                    0xcb49196c_1458_5e92_b701_d08d3e816bc5
                );
                impl windows_core::RuntimeType for IXamlControlsResourcesStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXamlControlsResourcesStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub EnsureRevealLights:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub UseCompactResourcesProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct IconElement(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(IconElement, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    IconElement,
                    super::super::Composition::IAnimationObject,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl IconElement {
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn IIconElementStatics<R, F: FnOnce(&IIconElementStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<IconElement, IIconElementStatics> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for IconElement {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IIconElement>();
                }
                unsafe impl windows_core::Interface for IconElement {
                    type Vtable = <IIconElement as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IIconElement as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for IconElement {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.IconElement";
                }
                unsafe impl Send for IconElement {}
                unsafe impl Sync for IconElement {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct Image(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(Image, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    Image,
                    super::super::Composition::IAnimationObject,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl Image {
                    pub fn new() -> windows_core::Result<Self> {
                        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                    }
                    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<Image, windows_core::imp::IGenericFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetSource<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::ImageSource>,
                    {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetSource)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn IImageStatics<R, F: FnOnce(&IImageStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<Image, IImageStatics> = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for Image {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IImage>();
                }
                unsafe impl windows_core::Interface for Image {
                    type Vtable = <IImage as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IImage as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for Image {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Image";
                }
                unsafe impl Send for Image {}
                unsafe impl Sync for Image {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct ImageIcon(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(ImageIcon, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    ImageIcon,
                    super::super::Composition::IAnimationObject,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    IconElement,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl ImageIcon {
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetSource<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::ImageSource>,
                    {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetSource)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn new() -> windows_core::Result<ImageIcon> {
                        Self::IImageIconFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::ptr::null_mut(),
                                &mut core::ptr::null_mut(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn IImageIconFactory<R, F: FnOnce(&IImageIconFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<ImageIcon, IImageIconFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn IImageIconStatics<R, F: FnOnce(&IImageIconStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<ImageIcon, IImageIconStatics> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for ImageIcon {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IImageIcon>();
                }
                unsafe impl windows_core::Interface for ImageIcon {
                    type Vtable = <IImageIcon as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IImageIcon as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for ImageIcon {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ImageIcon";
                }
                unsafe impl Send for ImageIcon {}
                unsafe impl Sync for ImageIcon {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct InfoBar(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(InfoBar, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    InfoBar,
                    super::super::Composition::IAnimationObject,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    Control,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl InfoBar {
                    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetPadding(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetHorizontalContentAlignment(&self, value: super::HorizontalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::Brush>,
                    {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetBackground)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn CornerRadius(&self) -> windows_core::Result<super::CornerRadius> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CornerRadius)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetCornerRadius(&self, value: super::CornerRadius) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetCornerRadius)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetIsOpen(&self, value: bool) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetIsOpen)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetTitle)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn SetMessage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetMessage)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn SetSeverity(&self, value: InfoBarSeverity) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetSeverity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetIsClosable(&self, value: bool) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetIsClosable)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetContent)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn new() -> windows_core::Result<InfoBar> {
                        Self::IInfoBarFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::ptr::null_mut(),
                                &mut core::ptr::null_mut(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn IInfoBarFactory<R, F: FnOnce(&IInfoBarFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<InfoBar, IInfoBarFactory> = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn IInfoBarStatics<R, F: FnOnce(&IInfoBarStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<InfoBar, IInfoBarStatics> = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for InfoBar {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInfoBar>();
                }
                unsafe impl windows_core::Interface for InfoBar {
                    type Vtable = <IInfoBar as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IInfoBar as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for InfoBar {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.InfoBar";
                }
                unsafe impl Send for InfoBar {}
                unsafe impl Sync for InfoBar {}
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct InfoBarSeverity(pub i32);
                impl InfoBarSeverity {
                    pub const Informational: Self = Self(0i32);
                    pub const Success: Self = Self(1i32);
                    pub const Warning: Self = Self(2i32);
                    pub const Error: Self = Self(3i32);
                }
                impl windows_core::TypeKind for InfoBarSeverity {
                    type TypeKind = windows_core::CopyType;
                }
                impl windows_core::RuntimeType for InfoBarSeverity {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Controls.InfoBarSeverity;i4)");
                }
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct ItemCollection(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    ItemCollection,
                    windows_core::IUnknown,
                    windows_core::IInspectable,
                    windows::Foundation::Collections::IObservableVector<windows_core::IInspectable>
                );
                windows_core::imp::required_hierarchy!(
                    ItemCollection,
                    windows_collections::IIterable<windows_core::IInspectable>,
                    windows_collections::IVector<windows_core::IInspectable>
                );
                impl ItemCollection {
                    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<windows_core::IInspectable>> {
                        let this = &windows_core::Interface::cast::<windows_collections::IIterable<windows_core::IInspectable>>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn Size(&self) -> windows_core::Result<u32> {
                        let this = &windows_core::Interface::cast::<windows_collections::IVector<windows_core::IInspectable>>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<windows_collections::IVector<windows_core::IInspectable>>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok()
                        }
                    }
                }
                impl windows_core::RuntimeType for ItemCollection {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<
                        Self,
                        windows::Foundation::Collections::IObservableVector<windows_core::IInspectable>,
                    >();
                }
                unsafe impl windows_core::Interface for ItemCollection {
                    type Vtable = < windows::Foundation::Collections:: IObservableVector < windows_core::IInspectable > as windows_core::Interface >::Vtable ;
                    const IID: windows_core::GUID =
                        <windows::Foundation::Collections::IObservableVector<windows_core::IInspectable> as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for ItemCollection {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ItemCollection";
                }
                unsafe impl Send for ItemCollection {}
                unsafe impl Sync for ItemCollection {}
                impl IntoIterator for ItemCollection {
                    type Item = windows_core::IInspectable;
                    type IntoIter = windows_collections::IIterator<Self::Item>;
                    fn into_iter(self) -> Self::IntoIter {
                        IntoIterator::into_iter(&self)
                    }
                }
                impl IntoIterator for &ItemCollection {
                    type Item = windows_core::IInspectable;
                    type IntoIter = windows_collections::IIterator<Self::Item>;
                    fn into_iter(self) -> Self::IntoIter {
                        self.First().unwrap()
                    }
                }
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct ItemsControl(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(ItemsControl, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    ItemsControl,
                    super::super::Composition::IAnimationObject,
                    IItemContainerMapping,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    Control,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl ItemsControl {
                    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetPadding(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetHorizontalContentAlignment(&self, value: super::HorizontalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::Brush>,
                    {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetBackground)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn CornerRadius(&self) -> windows_core::Result<super::CornerRadius> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CornerRadius)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetCornerRadius(&self, value: super::CornerRadius) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetCornerRadius)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn Items(&self) -> windows_core::Result<ItemCollection> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Items)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn new() -> windows_core::Result<ItemsControl> {
                        Self::IItemsControlFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::ptr::null_mut(),
                                &mut core::ptr::null_mut(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn IItemsControlFactory<R, F: FnOnce(&IItemsControlFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<ItemsControl, IItemsControlFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn IItemsControlStatics<R, F: FnOnce(&IItemsControlStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<ItemsControl, IItemsControlStatics> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for ItemsControl {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IItemsControl>();
                }
                unsafe impl windows_core::Interface for ItemsControl {
                    type Vtable = <IItemsControl as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IItemsControl as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for ItemsControl {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ItemsControl";
                }
                unsafe impl Send for ItemsControl {}
                unsafe impl Sync for ItemsControl {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct NavigationView(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(NavigationView, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    NavigationView,
                    super::super::Composition::IAnimationObject,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    ContentControl,
                    Control,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl NavigationView {
                    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<IContentControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetContent)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetPadding(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetHorizontalContentAlignment(&self, value: super::HorizontalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::Brush>,
                    {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetBackground)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn CornerRadius(&self) -> windows_core::Result<super::CornerRadius> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CornerRadius)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetCornerRadius(&self, value: super::CornerRadius) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetCornerRadius)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetIsPaneOpen(&self, value: bool) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetIsPaneOpen)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetPaneFooter<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::UIElement>,
                    {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetPaneFooter)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn SetIsSettingsVisible(&self, value: bool) -> windows_core::Result<()> {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetIsSettingsVisible)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetIsPaneToggleButtonVisible(&self, value: bool) -> windows_core::Result<()> {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetIsPaneToggleButtonVisible)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn SetOpenPaneLength(&self, value: f64) -> windows_core::Result<()> {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetOpenPaneLength)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SelectedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).SelectedItem)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn SetSelectedItem<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetSelectedItem)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn MenuItems(&self) -> windows_core::Result<windows_collections::IVector<windows_core::IInspectable>> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).MenuItems)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn SelectionChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<
                            windows::Foundation::TypedEventHandler<NavigationView, NavigationViewSelectionChangedEventArgs>,
                        >,
                    {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).SelectionChanged)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetIsBackButtonVisible(&self, value: NavigationViewBackButtonVisible) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetIsBackButtonVisible)(windows_core::Interface::as_raw(this), value)
                                .ok()
                        }
                    }
                    pub fn SetPaneDisplayMode(&self, value: NavigationViewPaneDisplayMode) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<INavigationView2>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetPaneDisplayMode)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn new() -> windows_core::Result<NavigationView> {
                        Self::INavigationViewFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::ptr::null_mut(),
                                &mut core::ptr::null_mut(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn INavigationViewFactory<R, F: FnOnce(&INavigationViewFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<NavigationView, INavigationViewFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn INavigationViewStatics<R, F: FnOnce(&INavigationViewStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<NavigationView, INavigationViewStatics> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn INavigationViewStatics2<R, F: FnOnce(&INavigationViewStatics2) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<NavigationView, INavigationViewStatics2> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for NavigationView {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INavigationView>();
                }
                unsafe impl windows_core::Interface for NavigationView {
                    type Vtable = <INavigationView as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <INavigationView as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for NavigationView {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationView";
                }
                unsafe impl Send for NavigationView {}
                unsafe impl Sync for NavigationView {}
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct NavigationViewBackButtonVisible(pub i32);
                impl NavigationViewBackButtonVisible {
                    pub const Collapsed: Self = Self(0i32);
                    pub const Visible: Self = Self(1i32);
                    pub const Auto: Self = Self(2i32);
                }
                impl windows_core::TypeKind for NavigationViewBackButtonVisible {
                    type TypeKind = windows_core::CopyType;
                }
                impl windows_core::RuntimeType for NavigationViewBackButtonVisible {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Controls.NavigationViewBackButtonVisible;i4)");
                }
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct NavigationViewItem(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(NavigationViewItem, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    NavigationViewItem,
                    super::super::Composition::IAnimationObject,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    NavigationViewItemBase,
                    ContentControl,
                    Control,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl NavigationViewItem {
                    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<IContentControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetContent)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetPadding(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetHorizontalContentAlignment(&self, value: super::HorizontalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::Brush>,
                    {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetBackground)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn CornerRadius(&self) -> windows_core::Result<super::CornerRadius> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CornerRadius)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetCornerRadius(&self, value: super::CornerRadius) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetCornerRadius)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetIcon<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<IconElement>,
                    {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetIcon)(windows_core::Interface::as_raw(this), value.param().abi()).ok()
                        }
                    }
                    pub fn MenuItems(&self) -> windows_core::Result<windows_collections::IVector<windows_core::IInspectable>> {
                        let this = &windows_core::Interface::cast::<INavigationViewItem2>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).MenuItems)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn new() -> windows_core::Result<NavigationViewItem> {
                        Self::INavigationViewItemFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::ptr::null_mut(),
                                &mut core::ptr::null_mut(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn INavigationViewItemFactory<R, F: FnOnce(&INavigationViewItemFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<NavigationViewItem, INavigationViewItemFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn INavigationViewItemStatics<R, F: FnOnce(&INavigationViewItemStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<NavigationViewItem, INavigationViewItemStatics> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn INavigationViewItemStatics2<R, F: FnOnce(&INavigationViewItemStatics2) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<NavigationViewItem, INavigationViewItemStatics2> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn INavigationViewItemStatics3<R, F: FnOnce(&INavigationViewItemStatics3) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<NavigationViewItem, INavigationViewItemStatics3> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for NavigationViewItem {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, INavigationViewItem>();
                }
                unsafe impl windows_core::Interface for NavigationViewItem {
                    type Vtable = <INavigationViewItem as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <INavigationViewItem as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for NavigationViewItem {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationViewItem";
                }
                unsafe impl Send for NavigationViewItem {}
                unsafe impl Sync for NavigationViewItem {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct NavigationViewItemBase(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(NavigationViewItemBase, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    NavigationViewItemBase,
                    super::super::Composition::IAnimationObject,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    ContentControl,
                    Control,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl NavigationViewItemBase {
                    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<IContentControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetContent)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetPadding(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetHorizontalContentAlignment(&self, value: super::HorizontalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::Brush>,
                    {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetBackground)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn CornerRadius(&self) -> windows_core::Result<super::CornerRadius> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CornerRadius)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetCornerRadius(&self, value: super::CornerRadius) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetCornerRadius)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn INavigationViewItemBaseStatics<R, F: FnOnce(&INavigationViewItemBaseStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<NavigationViewItemBase, INavigationViewItemBaseStatics> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for NavigationViewItemBase {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, INavigationViewItemBase>();
                }
                unsafe impl windows_core::Interface for NavigationViewItemBase {
                    type Vtable = <INavigationViewItemBase as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <INavigationViewItemBase as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for NavigationViewItemBase {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationViewItemBase";
                }
                unsafe impl Send for NavigationViewItemBase {}
                unsafe impl Sync for NavigationViewItemBase {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct NavigationViewItemHeader(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(NavigationViewItemHeader, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    NavigationViewItemHeader,
                    super::super::Composition::IAnimationObject,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    NavigationViewItemBase,
                    ContentControl,
                    Control,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl NavigationViewItemHeader {
                    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<IContentControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetContent)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetPadding(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetHorizontalContentAlignment(&self, value: super::HorizontalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::Brush>,
                    {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetBackground)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn CornerRadius(&self) -> windows_core::Result<super::CornerRadius> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CornerRadius)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetCornerRadius(&self, value: super::CornerRadius) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetCornerRadius)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn new() -> windows_core::Result<NavigationViewItemHeader> {
                        Self::INavigationViewItemHeaderFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::ptr::null_mut(),
                                &mut core::ptr::null_mut(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn INavigationViewItemHeaderFactory<R, F: FnOnce(&INavigationViewItemHeaderFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<NavigationViewItemHeader, INavigationViewItemHeaderFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for NavigationViewItemHeader {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, INavigationViewItemHeader>();
                }
                unsafe impl windows_core::Interface for NavigationViewItemHeader {
                    type Vtable = <INavigationViewItemHeader as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <INavigationViewItemHeader as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for NavigationViewItemHeader {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationViewItemHeader";
                }
                unsafe impl Send for NavigationViewItemHeader {}
                unsafe impl Sync for NavigationViewItemHeader {}
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct NavigationViewPaneDisplayMode(pub i32);
                impl NavigationViewPaneDisplayMode {
                    pub const Auto: Self = Self(0i32);
                    pub const Left: Self = Self(1i32);
                    pub const Top: Self = Self(2i32);
                    pub const LeftCompact: Self = Self(3i32);
                    pub const LeftMinimal: Self = Self(4i32);
                }
                impl windows_core::TypeKind for NavigationViewPaneDisplayMode {
                    type TypeKind = windows_core::CopyType;
                }
                impl windows_core::RuntimeType for NavigationViewPaneDisplayMode {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Controls.NavigationViewPaneDisplayMode;i4)");
                }
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct NavigationViewSelectionChangedEventArgs(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    NavigationViewSelectionChangedEventArgs,
                    windows_core::IUnknown,
                    windows_core::IInspectable
                );
                impl NavigationViewSelectionChangedEventArgs {
                    pub fn SelectedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).SelectedItem)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                }
                impl windows_core::RuntimeType for NavigationViewSelectionChangedEventArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, INavigationViewSelectionChangedEventArgs>();
                }
                unsafe impl windows_core::Interface for NavigationViewSelectionChangedEventArgs {
                    type Vtable = <INavigationViewSelectionChangedEventArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <INavigationViewSelectionChangedEventArgs as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for NavigationViewSelectionChangedEventArgs {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.NavigationViewSelectionChangedEventArgs";
                }
                unsafe impl Send for NavigationViewSelectionChangedEventArgs {}
                unsafe impl Sync for NavigationViewSelectionChangedEventArgs {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct Panel(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(Panel, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    Panel,
                    super::super::Composition::IAnimationObject,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl Panel {
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn Children(&self) -> windows_core::Result<UIElementCollection> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Children)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::Brush>,
                    {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetBackground)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn new() -> windows_core::Result<Panel> {
                        Self::IPanelFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::ptr::null_mut(),
                                &mut core::ptr::null_mut(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn IPanelFactory<R, F: FnOnce(&IPanelFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<Panel, IPanelFactory> = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn IPanelStatics<R, F: FnOnce(&IPanelStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<Panel, IPanelStatics> = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for Panel {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPanel>();
                }
                unsafe impl windows_core::Interface for Panel {
                    type Vtable = <IPanel as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IPanel as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for Panel {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Panel";
                }
                unsafe impl Send for Panel {}
                unsafe impl Sync for Panel {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct ScrollViewer(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(ScrollViewer, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    ScrollViewer,
                    super::super::Composition::IAnimationObject,
                    IScrollAnchorProvider,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    ContentControl,
                    Control,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl ScrollViewer {
                    pub fn new() -> windows_core::Result<Self> {
                        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                    }
                    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<ScrollViewer, windows_core::imp::IGenericFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<IContentControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetContent)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetPadding(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetHorizontalContentAlignment(&self, value: super::HorizontalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::Brush>,
                    {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetBackground)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn CornerRadius(&self) -> windows_core::Result<super::CornerRadius> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CornerRadius)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetCornerRadius(&self, value: super::CornerRadius) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetCornerRadius)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn IScrollViewerStatics<R, F: FnOnce(&IScrollViewerStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<ScrollViewer, IScrollViewerStatics> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for ScrollViewer {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IScrollViewer>();
                }
                unsafe impl windows_core::Interface for ScrollViewer {
                    type Vtable = <IScrollViewer as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IScrollViewer as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for ScrollViewer {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ScrollViewer";
                }
                unsafe impl Send for ScrollViewer {}
                unsafe impl Sync for ScrollViewer {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct SelectionChangedEventArgs(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(SelectionChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(SelectionChangedEventArgs, super::RoutedEventArgs);
                impl SelectionChangedEventArgs {
                    fn ISelectionChangedEventArgsFactory<R, F: FnOnce(&ISelectionChangedEventArgsFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<SelectionChangedEventArgs, ISelectionChangedEventArgsFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for SelectionChangedEventArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, ISelectionChangedEventArgs>();
                }
                unsafe impl windows_core::Interface for SelectionChangedEventArgs {
                    type Vtable = <ISelectionChangedEventArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <ISelectionChangedEventArgs as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for SelectionChangedEventArgs {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.SelectionChangedEventArgs";
                }
                unsafe impl Send for SelectionChangedEventArgs {}
                unsafe impl Sync for SelectionChangedEventArgs {}
                windows_core::imp::define_interface!(
                    SelectionChangedEventHandler,
                    SelectionChangedEventHandler_Vtbl,
                    0xa232390d_0e34_595e_8931_fa928a9909f4
                );
                impl windows_core::RuntimeType for SelectionChangedEventHandler {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"delegate(").push_other(windows_core::imp::ConstBuffer::for_interface::<Self>()).push_slice(b")");
                }
                impl SelectionChangedEventHandler {
                    pub fn new<
                        F: Fn(
                                windows_core::Ref<windows_core::IInspectable>,
                                windows_core::Ref<SelectionChangedEventArgs>,
                            ) -> windows_core::Result<()>
                            + Send
                            + 'static,
                    >(
                        invoke: F,
                    ) -> Self {
                        let com = SelectionChangedEventHandlerBox {
                            vtable: &SelectionChangedEventHandlerBox::<F>::VTABLE,
                            count: windows_core::imp::RefCount::new(1),
                            invoke,
                        };
                        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct SelectionChangedEventHandler_Vtbl {
                    base__: windows_core::IUnknown_Vtbl,
                    Invoke: unsafe extern "system" fn(
                        this: *mut core::ffi::c_void,
                        sender: *mut core::ffi::c_void,
                        e: *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                #[repr(C)]
                struct SelectionChangedEventHandlerBox<
                    F: Fn(
                            windows_core::Ref<windows_core::IInspectable>,
                            windows_core::Ref<SelectionChangedEventArgs>,
                        ) -> windows_core::Result<()>
                        + Send
                        + 'static,
                > {
                    vtable: *const SelectionChangedEventHandler_Vtbl,
                    invoke: F,
                    count: windows_core::imp::RefCount,
                }
                impl<
                        F: Fn(
                                windows_core::Ref<windows_core::IInspectable>,
                                windows_core::Ref<SelectionChangedEventArgs>,
                            ) -> windows_core::Result<()>
                            + Send
                            + 'static,
                    > SelectionChangedEventHandlerBox<F>
                {
                    const VTABLE: SelectionChangedEventHandler_Vtbl = SelectionChangedEventHandler_Vtbl {
                        base__: windows_core::IUnknown_Vtbl {
                            QueryInterface: Self::QueryInterface,
                            AddRef: Self::AddRef,
                            Release: Self::Release,
                        },
                        Invoke: Self::Invoke,
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
                            *interface = if *iid == <SelectionChangedEventHandler as windows_core::Interface>::IID
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
                        e: *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT {
                        unsafe {
                            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
                            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
                        }
                    }
                }
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct StackPanel(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(StackPanel, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    StackPanel,
                    super::super::Composition::IAnimationObject,
                    IInsertionPanel,
                    Primitives::IScrollSnapPointsInfo,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    Panel,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl StackPanel {
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn Children(&self) -> windows_core::Result<UIElementCollection> {
                        let this = &windows_core::Interface::cast::<IPanel>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Children)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::Brush>,
                    {
                        let this = &windows_core::Interface::cast::<IPanel>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetBackground)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn CornerRadius(&self) -> windows_core::Result<super::CornerRadius> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CornerRadius)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetCornerRadius(&self, value: super::CornerRadius) -> windows_core::Result<()> {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetCornerRadius)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetPadding(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetSpacing(&self, value: f64) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetSpacing)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn new() -> windows_core::Result<StackPanel> {
                        Self::IStackPanelFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::ptr::null_mut(),
                                &mut core::ptr::null_mut(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn IStackPanelFactory<R, F: FnOnce(&IStackPanelFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<StackPanel, IStackPanelFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn IStackPanelStatics<R, F: FnOnce(&IStackPanelStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<StackPanel, IStackPanelStatics> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for StackPanel {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStackPanel>();
                }
                unsafe impl windows_core::Interface for StackPanel {
                    type Vtable = <IStackPanel as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IStackPanel as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for StackPanel {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.StackPanel";
                }
                unsafe impl Send for StackPanel {}
                unsafe impl Sync for StackPanel {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct TextBlock(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(TextBlock, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    TextBlock,
                    super::super::Composition::IAnimationObject,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl TextBlock {
                    pub fn new() -> windows_core::Result<Self> {
                        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                    }
                    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<TextBlock, windows_core::imp::IGenericFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn TextWrapping(&self) -> windows_core::Result<super::TextWrapping> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).TextWrapping)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetTextWrapping(&self, value: super::TextWrapping) -> windows_core::Result<()> {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetTextWrapping)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn Text(&self) -> windows_core::Result<windows_core::HSTRING> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Text)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| core::mem::transmute(result__))
                        }
                    }
                    pub fn SetText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetText)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn SetPadding(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SelectionChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).SelectionChanged)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn ITextBlockStatics<R, F: FnOnce(&ITextBlockStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<TextBlock, ITextBlockStatics> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for TextBlock {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextBlock>();
                }
                unsafe impl windows_core::Interface for TextBlock {
                    type Vtable = <ITextBlock as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <ITextBlock as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for TextBlock {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.TextBlock";
                }
                unsafe impl Send for TextBlock {}
                unsafe impl Sync for TextBlock {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct ToggleSwitch(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(ToggleSwitch, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    ToggleSwitch,
                    super::super::Composition::IAnimationObject,
                    super::super::Composition::IVisualElement,
                    super::super::Composition::IVisualElement2,
                    Control,
                    super::FrameworkElement,
                    super::UIElement,
                    super::DependencyObject
                );
                impl ToggleSwitch {
                    pub fn new() -> windows_core::Result<Self> {
                        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                    }
                    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<ToggleSwitch, windows_core::imp::IGenericFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetPadding(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetHorizontalContentAlignment(&self, value: super::HorizontalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                                windows_core::Interface::as_raw(this),
                                value,
                            )
                            .ok()
                        }
                    }
                    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::Media::Brush>,
                    {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetBackground)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn CornerRadius(&self) -> windows_core::Result<super::CornerRadius> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CornerRadius)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetCornerRadius(&self, value: super::CornerRadius) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IControl>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetCornerRadius)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::ResourceDictionary>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn Width(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn Height(&self) -> windows_core::Result<f64> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                windows_core::Interface::as_raw(this),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok()
                        }
                    }
                    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(value),
                            )
                            .ok()
                        }
                    }
                    pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Loaded)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn IsOn(&self) -> windows_core::Result<bool> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).IsOn)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetIsOn(&self, value: bool) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetIsOn)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn SetOnContent<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetOnContent)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn SetOffContent<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetOffContent)(
                                windows_core::Interface::as_raw(this),
                                value.param().abi(),
                            )
                            .ok()
                        }
                    }
                    pub fn Toggled<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Toggled)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                    where
                        P0: windows_core::Param<super::RoutedEventHandler>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).LostFocus)(
                                windows_core::Interface::as_raw(this),
                                handler.param().abi(),
                                &mut result__,
                            )
                            .map(|| result__)
                        }
                    }
                    pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::RoutedEvent>,
                        P1: windows_core::Param<windows_core::IInspectable>,
                    {
                        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).AddHandler)(
                                windows_core::Interface::as_raw(this),
                                routedevent.param().abi(),
                                handler.param().abi(),
                                handledeventstoo,
                            )
                            .ok()
                        }
                    }
                    fn IToggleSwitchStatics<R, F: FnOnce(&IToggleSwitchStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<ToggleSwitch, IToggleSwitchStatics> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for ToggleSwitch {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IToggleSwitch>();
                }
                unsafe impl windows_core::Interface for ToggleSwitch {
                    type Vtable = <IToggleSwitch as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IToggleSwitch as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for ToggleSwitch {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ToggleSwitch";
                }
                unsafe impl Send for ToggleSwitch {}
                unsafe impl Sync for ToggleSwitch {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct UIElementCollection(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    UIElementCollection,
                    windows_core::IUnknown,
                    windows_core::IInspectable,
                    windows_collections::IVector<super::UIElement>
                );
                windows_core::imp::required_hierarchy!(UIElementCollection, windows_collections::IIterable<super::UIElement>);
                impl UIElementCollection {
                    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<super::UIElement>> {
                        let this = &windows_core::Interface::cast::<windows_collections::IIterable<super::UIElement>>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn Size(&self) -> windows_core::Result<u32> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<super::UIElement>,
                    {
                        let this = self;
                        unsafe {
                            (windows_core::Interface::vtable(this).Append)(windows_core::Interface::as_raw(this), value.param().abi()).ok()
                        }
                    }
                }
                impl windows_core::RuntimeType for UIElementCollection {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IVector<super::UIElement>>();
                }
                unsafe impl windows_core::Interface for UIElementCollection {
                    type Vtable = <windows_collections::IVector<super::UIElement> as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <windows_collections::IVector<super::UIElement> as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for UIElementCollection {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.UIElementCollection";
                }
                unsafe impl Send for UIElementCollection {}
                unsafe impl Sync for UIElementCollection {}
                impl IntoIterator for UIElementCollection {
                    type Item = super::UIElement;
                    type IntoIter = windows_collections::IIterator<Self::Item>;
                    fn into_iter(self) -> Self::IntoIter {
                        IntoIterator::into_iter(&self)
                    }
                }
                impl IntoIterator for &UIElementCollection {
                    type Item = super::UIElement;
                    type IntoIter = windows_collections::IIterator<Self::Item>;
                    fn into_iter(self) -> Self::IntoIter {
                        self.First().unwrap()
                    }
                }
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct XamlControlsResources(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(XamlControlsResources, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy ! ( XamlControlsResources , windows_collections:: IIterable < windows_collections:: IKeyValuePair < windows_core::IInspectable , windows_core::IInspectable > > , windows_collections:: IMap < windows_core::IInspectable , windows_core::IInspectable > , super:: ResourceDictionary , super:: DependencyObject );
                impl XamlControlsResources {
                    pub fn new() -> windows_core::Result<Self> {
                        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                    }
                    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<XamlControlsResources, windows_core::imp::IGenericFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    pub fn First(
                        &self,
                    ) -> windows_core::Result<
                        windows_collections::IIterator<
                            windows_collections::IKeyValuePair<windows_core::IInspectable, windows_core::IInspectable>,
                        >,
                    > {
                        let this = &windows_core::Interface::cast::<
                            windows_collections::IIterable<
                                windows_collections::IKeyValuePair<windows_core::IInspectable, windows_core::IInspectable>,
                            >,
                        >(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn Size(&self) -> windows_core::Result<u32> {
                        let this = &windows_core::Interface::cast::<
                            windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>,
                        >(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetSource<P0>(&self, value: P0) -> windows_core::Result<()>
                    where
                        P0: windows_core::Param<windows::Foundation::Uri>,
                    {
                        let this = &windows_core::Interface::cast::<super::IResourceDictionary>(self)?;
                        unsafe {
                            (windows_core::Interface::vtable(this).SetSource)(windows_core::Interface::as_raw(this), value.param().abi())
                                .ok()
                        }
                    }
                    pub fn MergedDictionaries(&self) -> windows_core::Result<windows_collections::IVector<super::ResourceDictionary>> {
                        let this = &windows_core::Interface::cast::<super::IResourceDictionary>(self)?;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).MergedDictionaries)(windows_core::Interface::as_raw(this), &mut result__)
                                .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    fn IXamlControlsResourcesStatics<R, F: FnOnce(&IXamlControlsResourcesStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<XamlControlsResources, IXamlControlsResourcesStatics> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for XamlControlsResources {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IXamlControlsResources>();
                }
                unsafe impl windows_core::Interface for XamlControlsResources {
                    type Vtable = <IXamlControlsResources as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IXamlControlsResources as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for XamlControlsResources {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.XamlControlsResources";
                }
                unsafe impl Send for XamlControlsResources {}
                unsafe impl Sync for XamlControlsResources {}
                impl IntoIterator for XamlControlsResources {
                    type Item = windows_collections::IKeyValuePair<windows_core::IInspectable, windows_core::IInspectable>;
                    type IntoIter = windows_collections::IIterator<Self::Item>;
                    fn into_iter(self) -> Self::IntoIter {
                        IntoIterator::into_iter(&self)
                    }
                }
                impl IntoIterator for &XamlControlsResources {
                    type Item = windows_collections::IKeyValuePair<windows_core::IInspectable, windows_core::IInspectable>;
                    type IntoIter = windows_collections::IIterator<Self::Item>;
                    fn into_iter(self) -> Self::IntoIter {
                        self.First().unwrap()
                    }
                }
                pub mod Primitives {
                    #[repr(transparent)]
                    #[derive(Clone, Debug, Eq, PartialEq)]
                    pub struct ButtonBase(windows_core::IUnknown);
                    windows_core::imp::interface_hierarchy!(ButtonBase, windows_core::IUnknown, windows_core::IInspectable);
                    windows_core::imp::required_hierarchy!(
                        ButtonBase,
                        super::super::super::Composition::IAnimationObject,
                        super::super::super::Composition::IVisualElement,
                        super::super::super::Composition::IVisualElement2,
                        super::ContentControl,
                        super::Control,
                        super::super::FrameworkElement,
                        super::super::UIElement,
                        super::super::DependencyObject
                    );
                    impl ButtonBase {
                        pub fn Click<P0>(&self, handler: P0) -> windows_core::Result<i64>
                        where
                            P0: windows_core::Param<super::super::RoutedEventHandler>,
                        {
                            let this = self;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).Click)(
                                    windows_core::Interface::as_raw(this),
                                    handler.param().abi(),
                                    &mut result__,
                                )
                                .map(|| result__)
                            }
                        }
                        pub fn new() -> windows_core::Result<ButtonBase> {
                            Self::IButtonBaseFactory(|this| unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).CreateInstance)(
                                    windows_core::Interface::as_raw(this),
                                    core::ptr::null_mut(),
                                    &mut core::ptr::null_mut(),
                                    &mut result__,
                                )
                                .and_then(|| windows_core::Type::from_abi(result__))
                            })
                        }
                        pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
                        where
                            P0: windows_core::Param<windows_core::IInspectable>,
                        {
                            let this = &windows_core::Interface::cast::<super::IContentControl>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetContent)(
                                    windows_core::Interface::as_raw(this),
                                    value.param().abi(),
                                )
                                .ok()
                            }
                        }
                        pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::IControl>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok()
                            }
                        }
                        pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::IControl>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok()
                            }
                        }
                        pub fn SetPadding(&self, value: super::super::Thickness) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::IControl>(self)?;
                            unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                        }
                        pub fn SetHorizontalContentAlignment(&self, value: super::super::HorizontalAlignment) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::IControl>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                                    windows_core::Interface::as_raw(this),
                                    value,
                                )
                                .ok()
                            }
                        }
                        pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                        where
                            P0: windows_core::Param<super::super::Media::Brush>,
                        {
                            let this = &windows_core::Interface::cast::<super::IControl>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetBackground)(
                                    windows_core::Interface::as_raw(this),
                                    value.param().abi(),
                                )
                                .ok()
                            }
                        }
                        pub fn CornerRadius(&self) -> windows_core::Result<super::super::CornerRadius> {
                            let this = &windows_core::Interface::cast::<super::IControl>(self)?;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).CornerRadius)(windows_core::Interface::as_raw(this), &mut result__)
                                    .map(|| result__)
                            }
                        }
                        pub fn SetCornerRadius(&self, value: super::super::CornerRadius) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::IControl>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetCornerRadius)(windows_core::Interface::as_raw(this), value).ok()
                            }
                        }
                        pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                        where
                            P0: windows_core::Param<super::super::ResourceDictionary>,
                        {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetResources)(
                                    windows_core::Interface::as_raw(this),
                                    value.param().abi(),
                                )
                                .ok()
                            }
                        }
                        pub fn Width(&self) -> windows_core::Result<f64> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                    .map(|| result__)
                            }
                        }
                        pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                        }
                        pub fn Height(&self) -> windows_core::Result<f64> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                    .map(|| result__)
                            }
                        }
                        pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                        }
                        pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok()
                            }
                        }
                        pub fn HorizontalAlignment(&self) -> windows_core::Result<super::super::HorizontalAlignment> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                    windows_core::Interface::as_raw(this),
                                    &mut result__,
                                )
                                .map(|| result__)
                            }
                        }
                        pub fn VerticalAlignment(&self) -> windows_core::Result<super::super::VerticalAlignment> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).VerticalAlignment)(
                                    windows_core::Interface::as_raw(this),
                                    &mut result__,
                                )
                                .map(|| result__)
                            }
                        }
                        pub fn SetVerticalAlignment(&self, value: super::super::VerticalAlignment) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value)
                                    .ok()
                            }
                        }
                        pub fn SetMargin(&self, value: super::super::Thickness) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                        }
                        pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetName)(
                                    windows_core::Interface::as_raw(this),
                                    core::mem::transmute_copy(value),
                                )
                                .ok()
                            }
                        }
                        pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                        where
                            P0: windows_core::Param<super::super::RoutedEventHandler>,
                        {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).Loaded)(
                                    windows_core::Interface::as_raw(this),
                                    handler.param().abi(),
                                    &mut result__,
                                )
                                .map(|| result__)
                            }
                        }
                        pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
                            unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                        }
                        pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                        where
                            P0: windows_core::Param<super::super::RoutedEventHandler>,
                        {
                            let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).LostFocus)(
                                    windows_core::Interface::as_raw(this),
                                    handler.param().abi(),
                                    &mut result__,
                                )
                                .map(|| result__)
                            }
                        }
                        pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                        where
                            P0: windows_core::Param<super::super::RoutedEvent>,
                            P1: windows_core::Param<windows_core::IInspectable>,
                        {
                            let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).AddHandler)(
                                    windows_core::Interface::as_raw(this),
                                    routedevent.param().abi(),
                                    handler.param().abi(),
                                    handledeventstoo,
                                )
                                .ok()
                            }
                        }
                        fn IButtonBaseFactory<R, F: FnOnce(&IButtonBaseFactory) -> windows_core::Result<R>>(
                            callback: F,
                        ) -> windows_core::Result<R> {
                            static SHARED: windows_core::imp::FactoryCache<ButtonBase, IButtonBaseFactory> =
                                windows_core::imp::FactoryCache::new();
                            SHARED.call(callback)
                        }
                        fn IButtonBaseStatics<R, F: FnOnce(&IButtonBaseStatics) -> windows_core::Result<R>>(
                            callback: F,
                        ) -> windows_core::Result<R> {
                            static SHARED: windows_core::imp::FactoryCache<ButtonBase, IButtonBaseStatics> =
                                windows_core::imp::FactoryCache::new();
                            SHARED.call(callback)
                        }
                    }
                    impl windows_core::RuntimeType for ButtonBase {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IButtonBase>();
                    }
                    unsafe impl windows_core::Interface for ButtonBase {
                        type Vtable = <IButtonBase as windows_core::Interface>::Vtable;
                        const IID: windows_core::GUID = <IButtonBase as windows_core::Interface>::IID;
                    }
                    impl windows_core::RuntimeName for ButtonBase {
                        const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.ButtonBase";
                    }
                    unsafe impl Send for ButtonBase {}
                    unsafe impl Sync for ButtonBase {}
                    windows_core::imp::define_interface!(IButtonBase, IButtonBase_Vtbl, 0x65714269_2473_5327_a652_0ea6bce7f403);
                    impl windows_core::RuntimeType for IButtonBase {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct IButtonBase_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        ClickMode: usize,
                        SetClickMode: usize,
                        pub IsPointerOver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                        pub IsPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                        Command: usize,
                        SetCommand: usize,
                        pub CommandParameter:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub SetCommandParameter:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub Click:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                        pub RemoveClick: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IButtonBaseFactory,
                        IButtonBaseFactory_Vtbl,
                        0x21251aa9_6fd1_5e51_ab3b_e6fcaf3395ed
                    );
                    impl windows_core::RuntimeType for IButtonBaseFactory {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct IButtonBaseFactory_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub CreateInstance: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IButtonBaseStatics,
                        IButtonBaseStatics_Vtbl,
                        0xdbe812f6_adf8_51d3_8137_a8fbf6445b3c
                    );
                    impl windows_core::RuntimeType for IButtonBaseStatics {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct IButtonBaseStatics_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub ClickModeProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub IsPointerOverProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub IsPressedProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub CommandProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub CommandParameterProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IScrollSnapPointsInfo,
                        IScrollSnapPointsInfo_Vtbl,
                        0xd3ea6e09_ecf7_51a8_bd54_fc84b9653766
                    );
                    impl windows_core::RuntimeType for IScrollSnapPointsInfo {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                    }
                    windows_core::imp::interface_hierarchy!(IScrollSnapPointsInfo, windows_core::IUnknown, windows_core::IInspectable);
                    impl IScrollSnapPointsInfo {
                    }
                    impl windows_core::RuntimeName for IScrollSnapPointsInfo {
                        const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IScrollSnapPointsInfo";
                    }
                    pub trait IScrollSnapPointsInfo_Impl: windows_core::IUnknownImpl {
                        fn AreHorizontalSnapPointsRegular(&self) -> windows_core::Result<bool>;
                        fn AreVerticalSnapPointsRegular(&self) -> windows_core::Result<bool>;
                        fn HorizontalSnapPointsChanged(
                            &self,
                            handler: windows_core::Ref<windows::Foundation::EventHandler<windows_core::IInspectable>>,
                        ) -> windows_core::Result<i64>;
                        fn RemoveHorizontalSnapPointsChanged(&self, token: i64) -> windows_core::Result<()>;
                        fn VerticalSnapPointsChanged(
                            &self,
                            handler: windows_core::Ref<windows::Foundation::EventHandler<windows_core::IInspectable>>,
                        ) -> windows_core::Result<i64>;
                        fn RemoveVerticalSnapPointsChanged(&self, token: i64) -> windows_core::Result<()>;
                    }
                    impl IScrollSnapPointsInfo_Vtbl {
                        pub const fn new<Identity: IScrollSnapPointsInfo_Impl, const OFFSET: isize>() -> Self {
                            unsafe extern "system" fn AreHorizontalSnapPointsRegular<
                                Identity: IScrollSnapPointsInfo_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                result__: *mut bool,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                    match IScrollSnapPointsInfo_Impl::AreHorizontalSnapPointsRegular(this) {
                                        Ok(ok__) => {
                                            result__.write(core::mem::transmute_copy(&ok__));
                                            windows_core::HRESULT(0)
                                        }
                                        Err(err) => err.into(),
                                    }
                                }
                            }
                            unsafe extern "system" fn AreVerticalSnapPointsRegular<
                                Identity: IScrollSnapPointsInfo_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                result__: *mut bool,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                    match IScrollSnapPointsInfo_Impl::AreVerticalSnapPointsRegular(this) {
                                        Ok(ok__) => {
                                            result__.write(core::mem::transmute_copy(&ok__));
                                            windows_core::HRESULT(0)
                                        }
                                        Err(err) => err.into(),
                                    }
                                }
                            }
                            unsafe extern "system" fn HorizontalSnapPointsChanged<
                                Identity: IScrollSnapPointsInfo_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                handler: *mut core::ffi::c_void,
                                result__: *mut i64,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                    match IScrollSnapPointsInfo_Impl::HorizontalSnapPointsChanged(this, core::mem::transmute_copy(&handler))
                                    {
                                        Ok(ok__) => {
                                            result__.write(core::mem::transmute_copy(&ok__));
                                            windows_core::HRESULT(0)
                                        }
                                        Err(err) => err.into(),
                                    }
                                }
                            }
                            unsafe extern "system" fn RemoveHorizontalSnapPointsChanged<
                                Identity: IScrollSnapPointsInfo_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                token: i64,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                    IScrollSnapPointsInfo_Impl::RemoveHorizontalSnapPointsChanged(this, token).into()
                                }
                            }
                            unsafe extern "system" fn VerticalSnapPointsChanged<
                                Identity: IScrollSnapPointsInfo_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                handler: *mut core::ffi::c_void,
                                result__: *mut i64,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                    match IScrollSnapPointsInfo_Impl::VerticalSnapPointsChanged(this, core::mem::transmute_copy(&handler)) {
                                        Ok(ok__) => {
                                            result__.write(core::mem::transmute_copy(&ok__));
                                            windows_core::HRESULT(0)
                                        }
                                        Err(err) => err.into(),
                                    }
                                }
                            }
                            unsafe extern "system" fn RemoveVerticalSnapPointsChanged<
                                Identity: IScrollSnapPointsInfo_Impl,
                                const OFFSET: isize,
                            >(
                                this: *mut core::ffi::c_void,
                                token: i64,
                            ) -> windows_core::HRESULT {
                                unsafe {
                                    let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                    IScrollSnapPointsInfo_Impl::RemoveVerticalSnapPointsChanged(this, token).into()
                                }
                            }
                            Self {
                                base__: windows_core::IInspectable_Vtbl::new::<Identity, IScrollSnapPointsInfo, OFFSET>(),
                                AreHorizontalSnapPointsRegular: AreHorizontalSnapPointsRegular::<Identity, OFFSET>,
                                AreVerticalSnapPointsRegular: AreVerticalSnapPointsRegular::<Identity, OFFSET>,
                                HorizontalSnapPointsChanged: HorizontalSnapPointsChanged::<Identity, OFFSET>,
                                RemoveHorizontalSnapPointsChanged: RemoveHorizontalSnapPointsChanged::<Identity, OFFSET>,
                                VerticalSnapPointsChanged: VerticalSnapPointsChanged::<Identity, OFFSET>,
                                RemoveVerticalSnapPointsChanged: RemoveVerticalSnapPointsChanged::<Identity, OFFSET>,
                                GetIrregularSnapPoints: 0,
                                GetRegularSnapPoints: 0,
                            }
                        }
                        pub fn matches(iid: &windows_core::GUID) -> bool {
                            iid == &<IScrollSnapPointsInfo as windows_core::Interface>::IID
                        }
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct IScrollSnapPointsInfo_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub AreHorizontalSnapPointsRegular:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                        pub AreVerticalSnapPointsRegular:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                        pub HorizontalSnapPointsChanged:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                        pub RemoveHorizontalSnapPointsChanged:
                            unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                        pub VerticalSnapPointsChanged:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                        pub RemoveVerticalSnapPointsChanged:
                            unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                        GetIrregularSnapPoints: usize,
                        GetRegularSnapPoints: usize,
                    }
                    windows_core::imp::define_interface!(ISelector, ISelector_Vtbl, 0x8f7e2159_e61d_576f_8476_f83fde3d689e);
                    impl windows_core::RuntimeType for ISelector {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct ISelector_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub SelectedIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                        pub SetSelectedIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
                        pub SelectedItem:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub SetSelectedItem:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub SelectedValue:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub SetSelectedValue:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub SelectedValuePath:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub SetSelectedValuePath:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub IsSynchronizedWithCurrentItem:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub SetIsSynchronizedWithCurrentItem:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub SelectionChanged:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                        pub RemoveSelectionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(ISelectorFactory, ISelectorFactory_Vtbl, 0x21a42024_af07_58f9_8789_848d3324d901);
                    impl windows_core::RuntimeType for ISelectorFactory {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct ISelectorFactory_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                    }
                    windows_core::imp::define_interface!(ISelectorStatics, ISelectorStatics_Vtbl, 0x569b2234_1ceb_516e_b64e_0d479452e279);
                    impl windows_core::RuntimeType for ISelectorStatics {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct ISelectorStatics_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub SelectedIndexProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub SelectedItemProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub SelectedValueProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub SelectedValuePathProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub IsSynchronizedWithCurrentItemProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub GetIsSelectionActive:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    }
                    #[repr(transparent)]
                    #[derive(Clone, Debug, Eq, PartialEq)]
                    pub struct Selector(windows_core::IUnknown);
                    windows_core::imp::interface_hierarchy!(Selector, windows_core::IUnknown, windows_core::IInspectable);
                    windows_core::imp::required_hierarchy!(
                        Selector,
                        super::super::super::Composition::IAnimationObject,
                        super::IItemContainerMapping,
                        super::super::super::Composition::IVisualElement,
                        super::super::super::Composition::IVisualElement2,
                        super::ItemsControl,
                        super::Control,
                        super::super::FrameworkElement,
                        super::super::UIElement,
                        super::super::DependencyObject
                    );
                    impl Selector {
                        pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::IControl>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok()
                            }
                        }
                        pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::IControl>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok()
                            }
                        }
                        pub fn SetPadding(&self, value: super::super::Thickness) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::IControl>(self)?;
                            unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
                        }
                        pub fn SetHorizontalContentAlignment(&self, value: super::super::HorizontalAlignment) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::IControl>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(
                                    windows_core::Interface::as_raw(this),
                                    value,
                                )
                                .ok()
                            }
                        }
                        pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
                        where
                            P0: windows_core::Param<super::super::Media::Brush>,
                        {
                            let this = &windows_core::Interface::cast::<super::IControl>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetBackground)(
                                    windows_core::Interface::as_raw(this),
                                    value.param().abi(),
                                )
                                .ok()
                            }
                        }
                        pub fn CornerRadius(&self) -> windows_core::Result<super::super::CornerRadius> {
                            let this = &windows_core::Interface::cast::<super::IControl>(self)?;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).CornerRadius)(windows_core::Interface::as_raw(this), &mut result__)
                                    .map(|| result__)
                            }
                        }
                        pub fn SetCornerRadius(&self, value: super::super::CornerRadius) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::IControl>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetCornerRadius)(windows_core::Interface::as_raw(this), value).ok()
                            }
                        }
                        pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
                        where
                            P0: windows_core::Param<super::super::ResourceDictionary>,
                        {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetResources)(
                                    windows_core::Interface::as_raw(this),
                                    value.param().abi(),
                                )
                                .ok()
                            }
                        }
                        pub fn Width(&self) -> windows_core::Result<f64> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__)
                                    .map(|| result__)
                            }
                        }
                        pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
                        }
                        pub fn Height(&self) -> windows_core::Result<f64> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__)
                                    .map(|| result__)
                            }
                        }
                        pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
                        }
                        pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok()
                            }
                        }
                        pub fn HorizontalAlignment(&self) -> windows_core::Result<super::super::HorizontalAlignment> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).HorizontalAlignment)(
                                    windows_core::Interface::as_raw(this),
                                    &mut result__,
                                )
                                .map(|| result__)
                            }
                        }
                        pub fn VerticalAlignment(&self) -> windows_core::Result<super::super::VerticalAlignment> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).VerticalAlignment)(
                                    windows_core::Interface::as_raw(this),
                                    &mut result__,
                                )
                                .map(|| result__)
                            }
                        }
                        pub fn SetVerticalAlignment(&self, value: super::super::VerticalAlignment) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value)
                                    .ok()
                            }
                        }
                        pub fn SetMargin(&self, value: super::super::Thickness) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
                        }
                        pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetName)(
                                    windows_core::Interface::as_raw(this),
                                    core::mem::transmute_copy(value),
                                )
                                .ok()
                            }
                        }
                        pub fn Loaded<P0>(&self, handler: P0) -> windows_core::Result<i64>
                        where
                            P0: windows_core::Param<super::super::RoutedEventHandler>,
                        {
                            let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).Loaded)(
                                    windows_core::Interface::as_raw(this),
                                    handler.param().abi(),
                                    &mut result__,
                                )
                                .map(|| result__)
                            }
                        }
                        pub fn Items(&self) -> windows_core::Result<super::ItemCollection> {
                            let this = &windows_core::Interface::cast::<super::IItemsControl>(self)?;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).Items)(windows_core::Interface::as_raw(this), &mut result__)
                                    .and_then(|| windows_core::Type::from_abi(result__))
                            }
                        }
                        pub fn SelectedIndex(&self) -> windows_core::Result<i32> {
                            let this = self;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).SelectedIndex)(windows_core::Interface::as_raw(this), &mut result__)
                                    .map(|| result__)
                            }
                        }
                        pub fn SetSelectedIndex(&self, value: i32) -> windows_core::Result<()> {
                            let this = self;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetSelectedIndex)(windows_core::Interface::as_raw(this), value).ok()
                            }
                        }
                        pub fn SelectedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
                            let this = self;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).SelectedItem)(windows_core::Interface::as_raw(this), &mut result__)
                                    .and_then(|| windows_core::Type::from_abi(result__))
                            }
                        }
                        pub fn SetSelectedItem<P0>(&self, value: P0) -> windows_core::Result<()>
                        where
                            P0: windows_core::Param<windows_core::IInspectable>,
                        {
                            let this = self;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetSelectedItem)(
                                    windows_core::Interface::as_raw(this),
                                    value.param().abi(),
                                )
                                .ok()
                            }
                        }
                        pub fn SelectionChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
                        where
                            P0: windows_core::Param<super::SelectionChangedEventHandler>,
                        {
                            let this = self;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).SelectionChanged)(
                                    windows_core::Interface::as_raw(this),
                                    handler.param().abi(),
                                    &mut result__,
                                )
                                .map(|| result__)
                            }
                        }
                        pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                            let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
                            unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                        }
                        pub fn LostFocus<P0>(&self, handler: P0) -> windows_core::Result<i64>
                        where
                            P0: windows_core::Param<super::super::RoutedEventHandler>,
                        {
                            let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
                            unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).LostFocus)(
                                    windows_core::Interface::as_raw(this),
                                    handler.param().abi(),
                                    &mut result__,
                                )
                                .map(|| result__)
                            }
                        }
                        pub fn AddHandler<P0, P1>(&self, routedevent: P0, handler: P1, handledeventstoo: bool) -> windows_core::Result<()>
                        where
                            P0: windows_core::Param<super::super::RoutedEvent>,
                            P1: windows_core::Param<windows_core::IInspectable>,
                        {
                            let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).AddHandler)(
                                    windows_core::Interface::as_raw(this),
                                    routedevent.param().abi(),
                                    handler.param().abi(),
                                    handledeventstoo,
                                )
                                .ok()
                            }
                        }
                        fn ISelectorStatics<R, F: FnOnce(&ISelectorStatics) -> windows_core::Result<R>>(
                            callback: F,
                        ) -> windows_core::Result<R> {
                            static SHARED: windows_core::imp::FactoryCache<Selector, ISelectorStatics> =
                                windows_core::imp::FactoryCache::new();
                            SHARED.call(callback)
                        }
                    }
                    impl windows_core::RuntimeType for Selector {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISelector>();
                    }
                    unsafe impl windows_core::Interface for Selector {
                        type Vtable = <ISelector as windows_core::Interface>::Vtable;
                        const IID: windows_core::GUID = <ISelector as windows_core::Interface>::IID;
                    }
                    impl windows_core::RuntimeName for Selector {
                        const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.Selector";
                    }
                    unsafe impl Send for Selector {}
                    unsafe impl Sync for Selector {}
                }
            }
            pub mod Input {
                windows_core::imp::define_interface!(IKeyRoutedEventArgs, IKeyRoutedEventArgs_Vtbl, 0xee357007_a2d6_5c75_9431_05fd66ec7915);
                impl windows_core::RuntimeType for IKeyRoutedEventArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IKeyRoutedEventArgs_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::System::VirtualKey) -> windows_core::HRESULT,
                    pub KeyStatus: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut windows::UI::Core::CorePhysicalKeyStatus,
                    ) -> windows_core::HRESULT,
                    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                    pub OriginalKey:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::System::VirtualKey) -> windows_core::HRESULT,
                    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(KeyEventHandler, KeyEventHandler_Vtbl, 0xdb68e7cc_9a2b_527d_9989_25284daccc03);
                impl windows_core::RuntimeType for KeyEventHandler {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"delegate(").push_other(windows_core::imp::ConstBuffer::for_interface::<Self>()).push_slice(b")");
                }
                impl KeyEventHandler {
                    pub fn new<
                        F: Fn(
                                windows_core::Ref<windows_core::IInspectable>,
                                windows_core::Ref<KeyRoutedEventArgs>,
                            ) -> windows_core::Result<()>
                            + Send
                            + 'static,
                    >(
                        invoke: F,
                    ) -> Self {
                        let com = KeyEventHandlerBox {
                            vtable: &KeyEventHandlerBox::<F>::VTABLE,
                            count: windows_core::imp::RefCount::new(1),
                            invoke,
                        };
                        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct KeyEventHandler_Vtbl {
                    base__: windows_core::IUnknown_Vtbl,
                    Invoke: unsafe extern "system" fn(
                        this: *mut core::ffi::c_void,
                        sender: *mut core::ffi::c_void,
                        e: *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                #[repr(C)]
                struct KeyEventHandlerBox<
                    F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<KeyRoutedEventArgs>) -> windows_core::Result<()>
                        + Send
                        + 'static,
                > {
                    vtable: *const KeyEventHandler_Vtbl,
                    invoke: F,
                    count: windows_core::imp::RefCount,
                }
                impl<
                        F: Fn(
                                windows_core::Ref<windows_core::IInspectable>,
                                windows_core::Ref<KeyRoutedEventArgs>,
                            ) -> windows_core::Result<()>
                            + Send
                            + 'static,
                    > KeyEventHandlerBox<F>
                {
                    const VTABLE: KeyEventHandler_Vtbl = KeyEventHandler_Vtbl {
                        base__: windows_core::IUnknown_Vtbl {
                            QueryInterface: Self::QueryInterface,
                            AddRef: Self::AddRef,
                            Release: Self::Release,
                        },
                        Invoke: Self::Invoke,
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
                            *interface = if *iid == <KeyEventHandler as windows_core::Interface>::IID
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
                        e: *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT {
                        unsafe {
                            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
                            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
                        }
                    }
                }
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct KeyRoutedEventArgs(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(KeyRoutedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(KeyRoutedEventArgs, super::RoutedEventArgs);
                impl KeyRoutedEventArgs {
                    pub fn Key(&self) -> windows_core::Result<windows::System::VirtualKey> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).Key)(windows_core::Interface::as_raw(this), &mut result__)
                                .map(|| result__)
                        }
                    }
                    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                }
                impl windows_core::RuntimeType for KeyRoutedEventArgs {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, IKeyRoutedEventArgs>();
                }
                unsafe impl windows_core::Interface for KeyRoutedEventArgs {
                    type Vtable = <IKeyRoutedEventArgs as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IKeyRoutedEventArgs as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for KeyRoutedEventArgs {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Input.KeyRoutedEventArgs";
                }
                unsafe impl Send for KeyRoutedEventArgs {}
                unsafe impl Sync for KeyRoutedEventArgs {}
            }
            pub mod Markup {
                windows_core::imp::define_interface!(
                    IXamlMetadataProvider,
                    IXamlMetadataProvider_Vtbl,
                    0xa96251f0_2214_5d53_8746_ce99a2593cd7
                );
                impl windows_core::RuntimeType for IXamlMetadataProvider {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                windows_core::imp::interface_hierarchy!(IXamlMetadataProvider, windows_core::IUnknown, windows_core::IInspectable);
                impl IXamlMetadataProvider {
                    pub fn GetXamlType(
                        &self,
                        r#type: &super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
                    ) -> windows_core::Result<IXamlType> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).GetXamlType)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(r#type),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn GetXamlTypeByFullName(&self, fullname: &windows_core::HSTRING) -> windows_core::Result<IXamlType> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).GetXamlTypeByFullName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(fullname),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn GetXmlnsDefinitions(&self) -> windows_core::Result<windows_core::Array<XmlnsDefinition>> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::MaybeUninit::zeroed();
                            (windows_core::Interface::vtable(this).GetXmlnsDefinitions)(
                                windows_core::Interface::as_raw(this),
                                windows_core::Array::<XmlnsDefinition>::set_abi_len(core::mem::transmute(&mut result__)),
                                result__.as_mut_ptr() as *mut _ as _,
                            )
                            .map(|| result__.assume_init())
                        }
                    }
                }
                impl windows_core::RuntimeName for IXamlMetadataProvider {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IXamlMetadataProvider";
                }
                pub trait IXamlMetadataProvider_Impl: windows_core::IUnknownImpl {
                    fn GetXamlType(
                        &self,
                        r#type: &super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
                    ) -> windows_core::Result<IXamlType>;
                    fn GetXamlTypeByFullName(&self, fullName: &windows_core::HSTRING) -> windows_core::Result<IXamlType>;
                    fn GetXmlnsDefinitions(&self) -> windows_core::Result<windows_core::Array<XmlnsDefinition>>;
                }
                impl IXamlMetadataProvider_Vtbl {
                    pub const fn new<Identity: IXamlMetadataProvider_Impl, const OFFSET: isize>() -> Self {
                        unsafe extern "system" fn GetXamlType<Identity: IXamlMetadataProvider_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            r#type: core::mem::MaybeUninit<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IXamlMetadataProvider_Impl::GetXamlType(this, core::mem::transmute(&r#type)) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        core::mem::forget(ok__);
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn GetXamlTypeByFullName<Identity: IXamlMetadataProvider_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            fullname: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IXamlMetadataProvider_Impl::GetXamlTypeByFullName(this, core::mem::transmute(&fullname)) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        core::mem::forget(ok__);
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn GetXmlnsDefinitions<Identity: IXamlMetadataProvider_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            result_size__: *mut u32,
                            result__: *mut *mut core::mem::MaybeUninit<XmlnsDefinition>,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IXamlMetadataProvider_Impl::GetXmlnsDefinitions(this) {
                                    Ok(ok__) => {
                                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                                        result__.write(core::mem::transmute(ok_data__));
                                        result_size__.write(ok_data_len__);
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        Self {
                            base__: windows_core::IInspectable_Vtbl::new::<Identity, IXamlMetadataProvider, OFFSET>(),
                            GetXamlType: GetXamlType::<Identity, OFFSET>,
                            GetXamlTypeByFullName: GetXamlTypeByFullName::<Identity, OFFSET>,
                            GetXmlnsDefinitions: GetXmlnsDefinitions::<Identity, OFFSET>,
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<IXamlMetadataProvider as windows_core::Interface>::IID
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXamlMetadataProvider_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub GetXamlType: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        core::mem::MaybeUninit<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub GetXamlTypeByFullName: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub GetXmlnsDefinitions: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut u32,
                        *mut *mut core::mem::MaybeUninit<XmlnsDefinition>,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IXamlType, IXamlType_Vtbl, 0xd24219df_7ec9_57f1_a27b_6af251d9c5bc);
                impl windows_core::RuntimeType for IXamlType {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                windows_core::imp::interface_hierarchy!(IXamlType, windows_core::IUnknown, windows_core::IInspectable);
                impl IXamlType {
                }
                impl windows_core::RuntimeName for IXamlType {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Markup.IXamlType";
                }
                pub trait IXamlType_Impl: windows_core::IUnknownImpl {
                    fn BaseType(&self) -> windows_core::Result<IXamlType>;
                    fn FullName(&self) -> windows_core::Result<windows_core::HSTRING>;
                    fn IsArray(&self) -> windows_core::Result<bool>;
                    fn IsCollection(&self) -> windows_core::Result<bool>;
                    fn IsConstructible(&self) -> windows_core::Result<bool>;
                    fn IsDictionary(&self) -> windows_core::Result<bool>;
                    fn IsMarkupExtension(&self) -> windows_core::Result<bool>;
                    fn IsBindable(&self) -> windows_core::Result<bool>;
                    fn ItemType(&self) -> windows_core::Result<IXamlType>;
                    fn KeyType(&self) -> windows_core::Result<IXamlType>;
                    fn BoxedType(&self) -> windows_core::Result<IXamlType>;
                    fn UnderlyingType(&self) -> windows_core::Result<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>;
                    fn ActivateInstance(&self) -> windows_core::Result<windows_core::IInspectable>;
                    fn CreateFromString(&self, value: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable>;
                    fn AddToVector(
                        &self,
                        instance: windows_core::Ref<windows_core::IInspectable>,
                        value: windows_core::Ref<windows_core::IInspectable>,
                    ) -> windows_core::Result<()>;
                    fn AddToMap(
                        &self,
                        instance: windows_core::Ref<windows_core::IInspectable>,
                        key: windows_core::Ref<windows_core::IInspectable>,
                        value: windows_core::Ref<windows_core::IInspectable>,
                    ) -> windows_core::Result<()>;
                    fn RunInitializer(&self) -> windows_core::Result<()>;
                }
                impl IXamlType_Vtbl {
                    pub const fn new<Identity: IXamlType_Impl, const OFFSET: isize>() -> Self {
                        unsafe extern "system" fn BaseType<Identity: IXamlType_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IXamlType_Impl::BaseType(this) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        core::mem::forget(ok__);
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn FullName<Identity: IXamlType_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IXamlType_Impl::FullName(this) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        core::mem::forget(ok__);
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn IsArray<Identity: IXamlType_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            result__: *mut bool,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IXamlType_Impl::IsArray(this) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn IsCollection<Identity: IXamlType_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            result__: *mut bool,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IXamlType_Impl::IsCollection(this) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn IsConstructible<Identity: IXamlType_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            result__: *mut bool,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IXamlType_Impl::IsConstructible(this) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn IsDictionary<Identity: IXamlType_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            result__: *mut bool,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IXamlType_Impl::IsDictionary(this) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn IsMarkupExtension<Identity: IXamlType_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            result__: *mut bool,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IXamlType_Impl::IsMarkupExtension(this) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn IsBindable<Identity: IXamlType_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            result__: *mut bool,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IXamlType_Impl::IsBindable(this) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn ItemType<Identity: IXamlType_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IXamlType_Impl::ItemType(this) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        core::mem::forget(ok__);
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn KeyType<Identity: IXamlType_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IXamlType_Impl::KeyType(this) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        core::mem::forget(ok__);
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn BoxedType<Identity: IXamlType_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IXamlType_Impl::BoxedType(this) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        core::mem::forget(ok__);
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn UnderlyingType<Identity: IXamlType_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            result__: *mut core::mem::MaybeUninit<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IXamlType_Impl::UnderlyingType(this) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        core::mem::forget(ok__);
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn ActivateInstance<Identity: IXamlType_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IXamlType_Impl::ActivateInstance(this) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        core::mem::forget(ok__);
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn CreateFromString<Identity: IXamlType_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            value: *mut core::ffi::c_void,
                            result__: *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                match IXamlType_Impl::CreateFromString(this, core::mem::transmute(&value)) {
                                    Ok(ok__) => {
                                        result__.write(core::mem::transmute_copy(&ok__));
                                        core::mem::forget(ok__);
                                        windows_core::HRESULT(0)
                                    }
                                    Err(err) => err.into(),
                                }
                            }
                        }
                        unsafe extern "system" fn AddToVector<Identity: IXamlType_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            instance: *mut core::ffi::c_void,
                            value: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                IXamlType_Impl::AddToVector(this, core::mem::transmute_copy(&instance), core::mem::transmute_copy(&value))
                                    .into()
                            }
                        }
                        unsafe extern "system" fn AddToMap<Identity: IXamlType_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                            instance: *mut core::ffi::c_void,
                            key: *mut core::ffi::c_void,
                            value: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                IXamlType_Impl::AddToMap(
                                    this,
                                    core::mem::transmute_copy(&instance),
                                    core::mem::transmute_copy(&key),
                                    core::mem::transmute_copy(&value),
                                )
                                .into()
                            }
                        }
                        unsafe extern "system" fn RunInitializer<Identity: IXamlType_Impl, const OFFSET: isize>(
                            this: *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT {
                            unsafe {
                                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                                IXamlType_Impl::RunInitializer(this).into()
                            }
                        }
                        Self {
                            base__: windows_core::IInspectable_Vtbl::new::<Identity, IXamlType, OFFSET>(),
                            BaseType: BaseType::<Identity, OFFSET>,
                            ContentProperty: 0,
                            FullName: FullName::<Identity, OFFSET>,
                            IsArray: IsArray::<Identity, OFFSET>,
                            IsCollection: IsCollection::<Identity, OFFSET>,
                            IsConstructible: IsConstructible::<Identity, OFFSET>,
                            IsDictionary: IsDictionary::<Identity, OFFSET>,
                            IsMarkupExtension: IsMarkupExtension::<Identity, OFFSET>,
                            IsBindable: IsBindable::<Identity, OFFSET>,
                            ItemType: ItemType::<Identity, OFFSET>,
                            KeyType: KeyType::<Identity, OFFSET>,
                            BoxedType: BoxedType::<Identity, OFFSET>,
                            UnderlyingType: UnderlyingType::<Identity, OFFSET>,
                            ActivateInstance: ActivateInstance::<Identity, OFFSET>,
                            CreateFromString: CreateFromString::<Identity, OFFSET>,
                            GetMember: 0,
                            AddToVector: AddToVector::<Identity, OFFSET>,
                            AddToMap: AddToMap::<Identity, OFFSET>,
                            RunInitializer: RunInitializer::<Identity, OFFSET>,
                        }
                    }
                    pub fn matches(iid: &windows_core::GUID) -> bool {
                        iid == &<IXamlType as windows_core::Interface>::IID
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXamlType_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub BaseType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    ContentProperty: usize,
                    pub FullName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub IsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub IsCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub IsConstructible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub IsDictionary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub IsMarkupExtension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub IsBindable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                    pub ItemType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub KeyType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub BoxedType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub UnderlyingType: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::mem::MaybeUninit<super::super::super::super::Windows::UI::Xaml::Interop::TypeName>,
                    ) -> windows_core::HRESULT,
                    pub ActivateInstance:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub CreateFromString: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    GetMember: usize,
                    pub AddToVector: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub AddToMap: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                    pub RunInitializer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                #[repr(C)]
                #[derive(Clone, Debug, Default, PartialEq)]
                pub struct XmlnsDefinition {
                    pub XmlNamespace: windows_core::HSTRING,
                    pub Namespace: windows_core::HSTRING,
                }
                impl windows_core::TypeKind for XmlnsDefinition {
                    type TypeKind = windows_core::CloneType;
                }
                impl windows_core::RuntimeType for XmlnsDefinition {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.Xaml.Markup.XmlnsDefinition;string;string)");
                }
            }
            pub mod Media {
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct Brush(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(Brush, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(Brush, super::super::Composition::IAnimationObject, super::DependencyObject);
                impl Brush {
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = self;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn new() -> windows_core::Result<Brush> {
                        Self::IBrushFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::ptr::null_mut(),
                                &mut core::ptr::null_mut(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    fn IBrushFactory<R, F: FnOnce(&IBrushFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<Brush, IBrushFactory> = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn IBrushStatics<R, F: FnOnce(&IBrushStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<Brush, IBrushStatics> = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for Brush {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBrush>();
                }
                unsafe impl windows_core::Interface for Brush {
                    type Vtable = <IBrush as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IBrush as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for Brush {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Brush";
                }
                unsafe impl Send for Brush {}
                unsafe impl Sync for Brush {}
                windows_core::imp::define_interface!(IBrush, IBrush_Vtbl, 0x2de3cb83_1329_5679_88f8_c822bc5442cb);
                impl windows_core::RuntimeType for IBrush {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBrush_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Opacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
                    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
                    Transform: usize,
                    SetTransform: usize,
                    RelativeTransform: usize,
                    SetRelativeTransform: usize,
                }
                windows_core::imp::define_interface!(IBrushFactory, IBrushFactory_Vtbl, 0xb5258717_6c49_5ba5_87fd_35df382647a5);
                impl windows_core::RuntimeType for IBrushFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBrushFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IBrushOverrides, IBrushOverrides_Vtbl, 0xb6b08394_bacf_53db_9ac7_be1c693e3513);
                impl windows_core::RuntimeType for IBrushOverrides {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBrushOverrides_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    PopulatePropertyInfoOverride: usize,
                }
                windows_core::imp::define_interface!(IBrushStatics, IBrushStatics_Vtbl, 0x5b854f50_f818_5f01_91b0_28132d3f5957);
                impl windows_core::RuntimeType for IBrushStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IBrushStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub OpacityProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub TransformProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    pub RelativeTransformProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(IImageSource, IImageSource_Vtbl, 0x6c2038f6_d6d5_55e9_9b9e_082f12dbff60);
                impl windows_core::RuntimeType for IImageSource {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IImageSource_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                }
                windows_core::imp::define_interface!(IImageSourceFactory, IImageSourceFactory_Vtbl, 0x0b1e64a3_e353_5901_b84b_ae9842aea5cd);
                impl windows_core::RuntimeType for IImageSourceFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IImageSourceFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                }
                windows_core::imp::define_interface!(IMicaBackdrop, IMicaBackdrop_Vtbl, 0xc156a404_3dac_593a_b1f3_7a33c289dc83);
                impl windows_core::RuntimeType for IMicaBackdrop {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IMicaBackdrop_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    Kind: usize,
                    SetKind: usize,
                }
                windows_core::imp::define_interface!(
                    IMicaBackdropFactory,
                    IMicaBackdropFactory_Vtbl,
                    0x774379ce_74bd_59d4_849d_d99c4184d838
                );
                impl windows_core::RuntimeType for IMicaBackdropFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IMicaBackdropFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    IMicaBackdropStatics,
                    IMicaBackdropStatics_Vtbl,
                    0xa63abdce_c796_5509_9f4d_072bc1e599f1
                );
                impl windows_core::RuntimeType for IMicaBackdropStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IMicaBackdropStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub KindProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(ISolidColorBrush, ISolidColorBrush_Vtbl, 0xb3865c31_37c8_55c1_8a72_d41c67642e2a);
                impl windows_core::RuntimeType for ISolidColorBrush {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ISolidColorBrush_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Color: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
                    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, windows::UI::Color) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    ISolidColorBrushFactory,
                    ISolidColorBrushFactory_Vtbl,
                    0x7b559384_4daa_54f4_91ef_33a23fd816ca
                );
                impl windows_core::RuntimeType for ISolidColorBrushFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ISolidColorBrushFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstanceWithColor: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        windows::UI::Color,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    ISolidColorBrushStatics,
                    ISolidColorBrushStatics_Vtbl,
                    0x6bc16da0_c4e6_59b8_995b_b31e48424c07
                );
                impl windows_core::RuntimeType for ISolidColorBrushStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ISolidColorBrushStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub ColorProperty:
                        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(ISystemBackdrop, ISystemBackdrop_Vtbl, 0x5aeed5c4_37ac_5852_b73f_1b76ebc3205f);
                impl windows_core::RuntimeType for ISystemBackdrop {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ISystemBackdrop_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    GetDefaultSystemBackdropConfiguration: usize,
                }
                windows_core::imp::define_interface!(
                    ISystemBackdropFactory,
                    ISystemBackdropFactory_Vtbl,
                    0x1e07656b_fad2_5b29_913f_b6748bc45942
                );
                impl windows_core::RuntimeType for ISystemBackdropFactory {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ISystemBackdropFactory_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub CreateInstance: unsafe extern "system" fn(
                        *mut core::ffi::c_void,
                        *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                        *mut *mut core::ffi::c_void,
                    ) -> windows_core::HRESULT,
                }
                windows_core::imp::define_interface!(
                    ISystemBackdropOverrides,
                    ISystemBackdropOverrides_Vtbl,
                    0xeb1f5399_cad7_5611_b637_09d76a07e708
                );
                impl windows_core::RuntimeType for ISystemBackdropOverrides {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ISystemBackdropOverrides_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    OnTargetConnected: usize,
                    OnTargetDisconnected: usize,
                    OnDefaultSystemBackdropConfigurationChanged: usize,
                }
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct ImageSource(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(ImageSource, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(ImageSource, super::DependencyObject);
                impl ImageSource {
                }
                impl windows_core::RuntimeType for ImageSource {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IImageSource>();
                }
                unsafe impl windows_core::Interface for ImageSource {
                    type Vtable = <IImageSource as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IImageSource as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for ImageSource {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Media.ImageSource";
                }
                unsafe impl Send for ImageSource {}
                unsafe impl Sync for ImageSource {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct MicaBackdrop(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(MicaBackdrop, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(MicaBackdrop, SystemBackdrop, super::DependencyObject);
                impl MicaBackdrop {
                    pub fn new() -> windows_core::Result<MicaBackdrop> {
                        Self::IMicaBackdropFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::ptr::null_mut(),
                                &mut core::ptr::null_mut(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    fn IMicaBackdropFactory<R, F: FnOnce(&IMicaBackdropFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<MicaBackdrop, IMicaBackdropFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn IMicaBackdropStatics<R, F: FnOnce(&IMicaBackdropStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<MicaBackdrop, IMicaBackdropStatics> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for MicaBackdrop {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMicaBackdrop>();
                }
                unsafe impl windows_core::Interface for MicaBackdrop {
                    type Vtable = <IMicaBackdrop as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <IMicaBackdrop as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for MicaBackdrop {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Media.MicaBackdrop";
                }
                unsafe impl Send for MicaBackdrop {}
                unsafe impl Sync for MicaBackdrop {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct SolidColorBrush(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(SolidColorBrush, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(
                    SolidColorBrush,
                    super::super::Composition::IAnimationObject,
                    Brush,
                    super::DependencyObject
                );
                impl SolidColorBrush {
                    pub fn new() -> windows_core::Result<Self> {
                        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                    }
                    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<SolidColorBrush, windows_core::imp::IGenericFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
                        let this = &windows_core::Interface::cast::<IBrush>(self)?;
                        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
                    }
                    pub fn CreateInstanceWithColor(color: windows::UI::Color) -> windows_core::Result<SolidColorBrush> {
                        Self::ISolidColorBrushFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstanceWithColor)(
                                windows_core::Interface::as_raw(this),
                                color,
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    fn ISolidColorBrushFactory<R, F: FnOnce(&ISolidColorBrushFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<SolidColorBrush, ISolidColorBrushFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    fn ISolidColorBrushStatics<R, F: FnOnce(&ISolidColorBrushStatics) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<SolidColorBrush, ISolidColorBrushStatics> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for SolidColorBrush {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISolidColorBrush>();
                }
                unsafe impl windows_core::Interface for SolidColorBrush {
                    type Vtable = <ISolidColorBrush as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <ISolidColorBrush as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for SolidColorBrush {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Media.SolidColorBrush";
                }
                unsafe impl Send for SolidColorBrush {}
                unsafe impl Sync for SolidColorBrush {}
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct SystemBackdrop(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(SystemBackdrop, windows_core::IUnknown, windows_core::IInspectable);
                windows_core::imp::required_hierarchy!(SystemBackdrop, super::DependencyObject);
                impl SystemBackdrop {
                    pub fn new() -> windows_core::Result<SystemBackdrop> {
                        Self::ISystemBackdropFactory(|this| unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).CreateInstance)(
                                windows_core::Interface::as_raw(this),
                                core::ptr::null_mut(),
                                &mut core::ptr::null_mut(),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        })
                    }
                    fn ISystemBackdropFactory<R, F: FnOnce(&ISystemBackdropFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<SystemBackdrop, ISystemBackdropFactory> =
                            windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for SystemBackdrop {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemBackdrop>();
                }
                unsafe impl windows_core::Interface for SystemBackdrop {
                    type Vtable = <ISystemBackdrop as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <ISystemBackdrop as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for SystemBackdrop {
                    const NAME: &'static str = "Microsoft.UI.Xaml.Media.SystemBackdrop";
                }
                unsafe impl Send for SystemBackdrop {}
                unsafe impl Sync for SystemBackdrop {}
                pub mod Imaging {
                    #[repr(transparent)]
                    #[derive(Clone, Debug, Eq, PartialEq)]
                    pub struct BitmapImage(windows_core::IUnknown);
                    windows_core::imp::interface_hierarchy!(BitmapImage, windows_core::IUnknown, windows_core::IInspectable);
                    windows_core::imp::required_hierarchy!(BitmapImage, BitmapSource, super::ImageSource, super::super::DependencyObject);
                    impl BitmapImage {
                        pub fn new() -> windows_core::Result<Self> {
                            Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                        }
                        fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(
                            callback: F,
                        ) -> windows_core::Result<R> {
                            static SHARED: windows_core::imp::FactoryCache<BitmapImage, windows_core::imp::IGenericFactory> =
                                windows_core::imp::FactoryCache::new();
                            SHARED.call(callback)
                        }
                        pub fn SetDecodePixelWidth(&self, value: i32) -> windows_core::Result<()> {
                            let this = self;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetDecodePixelWidth)(windows_core::Interface::as_raw(this), value)
                                    .ok()
                            }
                        }
                        pub fn SetDecodePixelHeight(&self, value: i32) -> windows_core::Result<()> {
                            let this = self;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetDecodePixelHeight)(windows_core::Interface::as_raw(this), value)
                                    .ok()
                            }
                        }
                        pub fn CreateInstanceWithUriSource<P0>(urisource: P0) -> windows_core::Result<BitmapImage>
                        where
                            P0: windows_core::Param<windows::Foundation::Uri>,
                        {
                            Self::IBitmapImageFactory(|this| unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).CreateInstanceWithUriSource)(
                                    windows_core::Interface::as_raw(this),
                                    urisource.param().abi(),
                                    &mut result__,
                                )
                                .and_then(|| windows_core::Type::from_abi(result__))
                            })
                        }
                        pub fn SetSource<P0>(&self, streamsource: P0) -> windows_core::Result<()>
                        where
                            P0: windows_core::Param<windows::Storage::Streams::IRandomAccessStream>,
                        {
                            let this = &windows_core::Interface::cast::<IBitmapSource>(self)?;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetSource)(
                                    windows_core::Interface::as_raw(this),
                                    streamsource.param().abi(),
                                )
                                .ok()
                            }
                        }
                        fn IBitmapImageFactory<R, F: FnOnce(&IBitmapImageFactory) -> windows_core::Result<R>>(
                            callback: F,
                        ) -> windows_core::Result<R> {
                            static SHARED: windows_core::imp::FactoryCache<BitmapImage, IBitmapImageFactory> =
                                windows_core::imp::FactoryCache::new();
                            SHARED.call(callback)
                        }
                        fn IBitmapImageStatics<R, F: FnOnce(&IBitmapImageStatics) -> windows_core::Result<R>>(
                            callback: F,
                        ) -> windows_core::Result<R> {
                            static SHARED: windows_core::imp::FactoryCache<BitmapImage, IBitmapImageStatics> =
                                windows_core::imp::FactoryCache::new();
                            SHARED.call(callback)
                        }
                    }
                    impl windows_core::RuntimeType for BitmapImage {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBitmapImage>();
                    }
                    unsafe impl windows_core::Interface for BitmapImage {
                        type Vtable = <IBitmapImage as windows_core::Interface>::Vtable;
                        const IID: windows_core::GUID = <IBitmapImage as windows_core::Interface>::IID;
                    }
                    impl windows_core::RuntimeName for BitmapImage {
                        const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.BitmapImage";
                    }
                    unsafe impl Send for BitmapImage {}
                    unsafe impl Sync for BitmapImage {}
                    #[repr(transparent)]
                    #[derive(Clone, Debug, Eq, PartialEq)]
                    pub struct BitmapSource(windows_core::IUnknown);
                    windows_core::imp::interface_hierarchy!(BitmapSource, windows_core::IUnknown, windows_core::IInspectable);
                    windows_core::imp::required_hierarchy!(BitmapSource, super::ImageSource, super::super::DependencyObject);
                    impl BitmapSource {
                        pub fn SetSource<P0>(&self, streamsource: P0) -> windows_core::Result<()>
                        where
                            P0: windows_core::Param<windows::Storage::Streams::IRandomAccessStream>,
                        {
                            let this = self;
                            unsafe {
                                (windows_core::Interface::vtable(this).SetSource)(
                                    windows_core::Interface::as_raw(this),
                                    streamsource.param().abi(),
                                )
                                .ok()
                            }
                        }
                        pub fn new() -> windows_core::Result<BitmapSource> {
                            Self::IBitmapSourceFactory(|this| unsafe {
                                let mut result__ = core::mem::zeroed();
                                (windows_core::Interface::vtable(this).CreateInstance)(
                                    windows_core::Interface::as_raw(this),
                                    core::ptr::null_mut(),
                                    &mut core::ptr::null_mut(),
                                    &mut result__,
                                )
                                .and_then(|| windows_core::Type::from_abi(result__))
                            })
                        }
                        fn IBitmapSourceFactory<R, F: FnOnce(&IBitmapSourceFactory) -> windows_core::Result<R>>(
                            callback: F,
                        ) -> windows_core::Result<R> {
                            static SHARED: windows_core::imp::FactoryCache<BitmapSource, IBitmapSourceFactory> =
                                windows_core::imp::FactoryCache::new();
                            SHARED.call(callback)
                        }
                        fn IBitmapSourceStatics<R, F: FnOnce(&IBitmapSourceStatics) -> windows_core::Result<R>>(
                            callback: F,
                        ) -> windows_core::Result<R> {
                            static SHARED: windows_core::imp::FactoryCache<BitmapSource, IBitmapSourceStatics> =
                                windows_core::imp::FactoryCache::new();
                            SHARED.call(callback)
                        }
                    }
                    impl windows_core::RuntimeType for BitmapSource {
                        const SIGNATURE: windows_core::imp::ConstBuffer =
                            windows_core::imp::ConstBuffer::for_class::<Self, IBitmapSource>();
                    }
                    unsafe impl windows_core::Interface for BitmapSource {
                        type Vtable = <IBitmapSource as windows_core::Interface>::Vtable;
                        const IID: windows_core::GUID = <IBitmapSource as windows_core::Interface>::IID;
                    }
                    impl windows_core::RuntimeName for BitmapSource {
                        const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.BitmapSource";
                    }
                    unsafe impl Send for BitmapSource {}
                    unsafe impl Sync for BitmapSource {}
                    windows_core::imp::define_interface!(IBitmapImage, IBitmapImage_Vtbl, 0x5cc29916_a411_5bc2_a3c5_a00d99a59da8);
                    impl windows_core::RuntimeType for IBitmapImage {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct IBitmapImage_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        CreateOptions: usize,
                        SetCreateOptions: usize,
                        pub UriSource:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub SetUriSource:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub DecodePixelWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                        pub SetDecodePixelWidth: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
                        pub DecodePixelHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                        pub SetDecodePixelHeight: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
                        DecodePixelType: usize,
                        SetDecodePixelType: usize,
                        pub IsAnimatedBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                        pub IsPlaying: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                        pub AutoPlay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
                        pub SetAutoPlay: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
                        DownloadProgress: usize,
                        pub RemoveDownloadProgress: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                        pub ImageOpened:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
                        pub RemoveImageOpened: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                        ImageFailed: usize,
                        pub RemoveImageFailed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
                        pub Play: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IBitmapImageFactory,
                        IBitmapImageFactory_Vtbl,
                        0xf037e0e9_f229_522e_95c9_da2211a14b05
                    );
                    impl windows_core::RuntimeType for IBitmapImageFactory {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct IBitmapImageFactory_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub CreateInstanceWithUriSource: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IBitmapImageStatics,
                        IBitmapImageStatics_Vtbl,
                        0x4bcf71a9_1897_51dc_8e3f_2c5c796d1cd9
                    );
                    impl windows_core::RuntimeType for IBitmapImageStatics {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct IBitmapImageStatics_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub CreateOptionsProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub UriSourceProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub DecodePixelWidthProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub DecodePixelHeightProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub DecodePixelTypeProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub IsAnimatedBitmapProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub IsPlayingProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub AutoPlayProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(IBitmapSource, IBitmapSource_Vtbl, 0x8424269d_9b82_534f_8fea_af5b5ef96bf2);
                    impl windows_core::RuntimeType for IBitmapSource {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct IBitmapSource_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub PixelWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                        pub PixelHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
                        pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub SetSourceAsync: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IBitmapSourceFactory,
                        IBitmapSourceFactory_Vtbl,
                        0x0392f025_1868_5876_ad67_12e94a8da5bf
                    );
                    impl windows_core::RuntimeType for IBitmapSourceFactory {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct IBitmapSourceFactory_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub CreateInstance: unsafe extern "system" fn(
                            *mut core::ffi::c_void,
                            *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                            *mut *mut core::ffi::c_void,
                        ) -> windows_core::HRESULT,
                    }
                    windows_core::imp::define_interface!(
                        IBitmapSourceStatics,
                        IBitmapSourceStatics_Vtbl,
                        0xefa3745e_4400_5f0b_bdc7_3f2911a3d719
                    );
                    impl windows_core::RuntimeType for IBitmapSourceStatics {
                        const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                    }
                    #[repr(C)]
                    #[doc(hidden)]
                    pub struct IBitmapSourceStatics_Vtbl {
                        pub base__: windows_core::IInspectable_Vtbl,
                        pub PixelWidthProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                        pub PixelHeightProperty:
                            unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
                    }
                }
            }
            pub mod XamlTypeInfo {
                windows_core::imp::define_interface!(
                    IXamlControlsXamlMetaDataProvider,
                    IXamlControlsXamlMetaDataProvider_Vtbl,
                    0x17fa3f58_3472_5aa2_a0f8_1ab8a519573d
                );
                impl windows_core::RuntimeType for IXamlControlsXamlMetaDataProvider {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXamlControlsXamlMetaDataProvider_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                }
                windows_core::imp::define_interface!(
                    IXamlControlsXamlMetaDataProviderStatics,
                    IXamlControlsXamlMetaDataProviderStatics_Vtbl,
                    0x2d7eb3fd_ecdb_5084_b7e0_12f9598381ef
                );
                impl windows_core::RuntimeType for IXamlControlsXamlMetaDataProviderStatics {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct IXamlControlsXamlMetaDataProviderStatics_Vtbl {
                    pub base__: windows_core::IInspectable_Vtbl,
                    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
                }
                #[repr(transparent)]
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct XamlControlsXamlMetaDataProvider(windows_core::IUnknown);
                windows_core::imp::interface_hierarchy!(
                    XamlControlsXamlMetaDataProvider,
                    windows_core::IUnknown,
                    windows_core::IInspectable,
                    super::Markup::IXamlMetadataProvider
                );
                impl XamlControlsXamlMetaDataProvider {
                    pub fn new() -> windows_core::Result<Self> {
                        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                    }
                    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<
                            XamlControlsXamlMetaDataProvider,
                            windows_core::imp::IGenericFactory,
                        > = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                    pub fn GetXamlType(
                        &self,
                        r#type: &super::super::super::super::Windows::UI::Xaml::Interop::TypeName,
                    ) -> windows_core::Result<super::Markup::IXamlType> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).GetXamlType)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(r#type),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn GetXamlTypeByFullName(
                        &self,
                        fullname: &windows_core::HSTRING,
                    ) -> windows_core::Result<super::Markup::IXamlType> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::zeroed();
                            (windows_core::Interface::vtable(this).GetXamlTypeByFullName)(
                                windows_core::Interface::as_raw(this),
                                core::mem::transmute_copy(fullname),
                                &mut result__,
                            )
                            .and_then(|| windows_core::Type::from_abi(result__))
                        }
                    }
                    pub fn GetXmlnsDefinitions(&self) -> windows_core::Result<windows_core::Array<super::Markup::XmlnsDefinition>> {
                        let this = self;
                        unsafe {
                            let mut result__ = core::mem::MaybeUninit::zeroed();
                            (windows_core::Interface::vtable(this).GetXmlnsDefinitions)(
                                windows_core::Interface::as_raw(this),
                                windows_core::Array::<super::Markup::XmlnsDefinition>::set_abi_len(core::mem::transmute(&mut result__)),
                                result__.as_mut_ptr() as *mut _ as _,
                            )
                            .map(|| result__.assume_init())
                        }
                    }
                    fn IXamlControlsXamlMetaDataProviderStatics<
                        R,
                        F: FnOnce(&IXamlControlsXamlMetaDataProviderStatics) -> windows_core::Result<R>,
                    >(
                        callback: F,
                    ) -> windows_core::Result<R> {
                        static SHARED: windows_core::imp::FactoryCache<
                            XamlControlsXamlMetaDataProvider,
                            IXamlControlsXamlMetaDataProviderStatics,
                        > = windows_core::imp::FactoryCache::new();
                        SHARED.call(callback)
                    }
                }
                impl windows_core::RuntimeType for XamlControlsXamlMetaDataProvider {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::for_class::<Self, super::Markup::IXamlMetadataProvider>();
                }
                unsafe impl windows_core::Interface for XamlControlsXamlMetaDataProvider {
                    type Vtable = <super::Markup::IXamlMetadataProvider as windows_core::Interface>::Vtable;
                    const IID: windows_core::GUID = <super::Markup::IXamlMetadataProvider as windows_core::Interface>::IID;
                }
                impl windows_core::RuntimeName for XamlControlsXamlMetaDataProvider {
                    const NAME: &'static str = "Microsoft.UI.Xaml.XamlTypeInfo.XamlControlsXamlMetaDataProvider";
                }
                unsafe impl Send for XamlControlsXamlMetaDataProvider {}
                unsafe impl Sync for XamlControlsXamlMetaDataProvider {}
            }
        }
    }
}
pub mod Windows {
    pub mod UI {
        pub mod Xaml {
            pub mod Interop {
                #[repr(transparent)]
                #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
                pub struct TypeKind(pub i32);
                impl TypeKind {
                    pub const Primitive: Self = Self(0i32);
                    pub const Metadata: Self = Self(1i32);
                    pub const Custom: Self = Self(2i32);
                }
                impl windows_core::TypeKind for TypeKind {
                    type TypeKind = windows_core::CopyType;
                }
                impl windows_core::RuntimeType for TypeKind {
                    const SIGNATURE: windows_core::imp::ConstBuffer =
                        windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Interop.TypeKind;i4)");
                }
                #[repr(C)]
                #[derive(Clone, Debug, Default, PartialEq)]
                pub struct TypeName {
                    pub Name: windows_core::HSTRING,
                    pub Kind: TypeKind,
                }
                impl windows_core::TypeKind for TypeName {
                    type TypeKind = windows_core::CloneType;
                }
                impl windows_core::RuntimeType for TypeName {
                    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
                        b"struct(Windows.UI.Xaml.Interop.TypeName;string;enum(Windows.UI.Xaml.Interop.TypeKind;i4))",
                    );
                }
            }
        }
    }
}
