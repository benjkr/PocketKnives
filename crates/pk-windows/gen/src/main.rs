//! Regenerates `crates/pk-windows/src/bindings.rs`, the workspace's lean COM/WinRT bindings
//! (replacing the full `windows` crate).
//!
//! `cargo run -p pk-windows-gen`
//!
//! Unused `pub [unsafe] fn` wrapper methods are trimmed to the names referenced by the workspace's
//! sources ([`trim`]); vtables stay whole for ABI layout, just like `runner/winui/gen`.
use std::collections::BTreeSet;
use std::path::Path;

/// Every Windows WinRT/COM type, function, and constant used by PocketKnives blades and the WinUI
/// settings app. Direct dependencies of called methods are listed explicitly (`--no-deps`) so the
/// generated crate stays tiny.
const FILTER: &[&str] = &[
    // WinRT — QuickTooth (BLE advertisements)
    "Windows.Devices.Bluetooth.BluetoothError",
    "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisement",
    "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementFilter",
    "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementReceivedEventArgs",
    "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcher",
    "Windows.Devices.Bluetooth.Advertisement.BluetoothLEAdvertisementWatcherStoppedEventArgs",
    "Windows.Devices.Bluetooth.Advertisement.BluetoothLEManufacturerData",
    "Windows.Devices.Bluetooth.Advertisement.BluetoothLEScanningMode",
    // WinRT — Foundation (shared across blades & WinUI)
    "Windows.Foundation.DateTime",
    "Windows.Foundation.EventHandler",
    "Windows.Foundation.IPropertyValue",
    "Windows.Foundation.IReference",
    "Windows.Foundation.Point",
    "Windows.Foundation.PropertyType",
    "Windows.Foundation.PropertyValue",
    "Windows.Foundation.Rect",
    "Windows.Foundation.Size",
    "Windows.Foundation.TimeSpan",
    "Windows.Foundation.TypedEventHandler",
    "Windows.Foundation.Uri",
    "Windows.Foundation.Collections.IObservableVector",
    // WinRT — Graphics & Capture (snipsnap-rec & WinUI)
    "Windows.Graphics.PointInt32",
    "Windows.Graphics.RectInt32",
    "Windows.Graphics.SizeInt32",
    "Windows.Graphics.Capture.Direct3D11CaptureFrame",
    "Windows.Graphics.Capture.Direct3D11CaptureFramePool",
    "Windows.Graphics.Capture.GraphicsCaptureItem",
    "Windows.Graphics.Capture.GraphicsCaptureSession",
    "Windows.Graphics.DirectX.DirectXPixelFormat",
    "Windows.Graphics.DirectX.Direct3D11.IDirect3DDevice",
    "Windows.Graphics.DirectX.Direct3D11.IDirect3DSurface",
    // WinRT — Media Control (NowPlaying SMTC)
    "Windows.Media.Control.CurrentSessionChangedEventArgs",
    "Windows.Media.Control.GlobalSystemMediaTransportControlsSession",
    "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionManager",
    "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionMediaProperties",
    "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackControls",
    "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackInfo",
    "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionPlaybackStatus",
    "Windows.Media.Control.GlobalSystemMediaTransportControlsSessionTimelineProperties",
    "Windows.Media.Control.MediaPropertiesChangedEventArgs",
    "Windows.Media.Control.PlaybackInfoChangedEventArgs",
    "Windows.Media.Control.SessionsChangedEventArgs",
    "Windows.Media.Control.TimelinePropertiesChangedEventArgs",
    // WinRT — Storage Streams (QuickTooth, NowPlaying, WinUI)
    "Windows.Storage.Streams.Buffer",
    "Windows.Storage.Streams.DataReader",
    "Windows.Storage.Streams.IContentTypeProvider",
    "Windows.Storage.Streams.InputStreamOptions",
    "Windows.Storage.Streams.IRandomAccessStream",
    "Windows.Storage.Streams.IRandomAccessStreamReference",
    "Windows.Storage.Streams.IRandomAccessStreamWithContentType",
    // WinRT — System & UI (WinUI bindings)
    "Windows.System.VirtualKey",
    "Windows.UI.Color",
    "Windows.UI.Core.CorePhysicalKeyStatus",
    "Windows.UI.Text.FontStretch",
    "Windows.UI.Text.FontStyle",
    "Windows.UI.Text.FontWeight",
    "Windows.UI.Text.TextDecorations",
    // Win32 — Devices & Foundation
    "Windows.Win32.Devices.FunctionDiscovery.PKEY_Device_ContainerId",
    "Windows.Win32.Devices.FunctionDiscovery.PKEY_Device_FriendlyName",
    "Windows.Win32.Devices.FunctionDiscovery.PKEY_DeviceInterface_FriendlyName",
    "Windows.Win32.Foundation.HMODULE",
    "Windows.Win32.Foundation.HWND",
    "Windows.Win32.Foundation.PROPERTYKEY",
    "Windows.Win32.Foundation.RECT",
    "Windows.Win32.Foundation.SIZE",
    // Win32 — Direct2D & DirectWrite (NowPlaying & snipsnap-rec)
    "Windows.Win32.Graphics.Direct2D.D2D1CreateFactory",
    "Windows.Win32.Graphics.Direct2D.D2D1_ANTIALIAS_MODE_PER_PRIMITIVE",
    "Windows.Win32.Graphics.Direct2D.D2D1_BITMAP_INTERPOLATION_MODE_LINEAR",
    "Windows.Win32.Graphics.Direct2D.D2D1_BITMAP_PROPERTIES",
    "Windows.Win32.Graphics.Direct2D.D2D1_BRUSH_PROPERTIES",
    "Windows.Win32.Graphics.Direct2D.D2D1_DRAW_TEXT_OPTIONS_CLIP",
    "Windows.Win32.Graphics.Direct2D.D2D1_DRAW_TEXT_OPTIONS_NONE",
    "Windows.Win32.Graphics.Direct2D.D2D1_ELLIPSE",
    "Windows.Win32.Graphics.Direct2D.D2D1_EXTEND_MODE_CLAMP",
    "Windows.Win32.Graphics.Direct2D.D2D1_FACTORY_TYPE_SINGLE_THREADED",
    "Windows.Win32.Graphics.Direct2D.D2D1_GAMMA_2_2",
    "Windows.Win32.Graphics.Direct2D.D2D1_LAYER_OPTIONS_NONE",
    "Windows.Win32.Graphics.Direct2D.D2D1_LAYER_PARAMETERS",
    "Windows.Win32.Graphics.Direct2D.D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES",
    "Windows.Win32.Graphics.Direct2D.D2D1_RENDER_TARGET_PROPERTIES",
    "Windows.Win32.Graphics.Direct2D.D2D1_RENDER_TARGET_TYPE_DEFAULT",
    "Windows.Win32.Graphics.Direct2D.D2D1_ROUNDED_RECT",
    "Windows.Win32.Graphics.Direct2D.D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE",
    "Windows.Win32.Graphics.Direct2D.ID2D1Bitmap",
    "Windows.Win32.Graphics.Direct2D.ID2D1DCRenderTarget",
    "Windows.Win32.Graphics.Direct2D.ID2D1Factory",
    "Windows.Win32.Graphics.Direct2D.ID2D1GeometrySink",
    "Windows.Win32.Graphics.Direct2D.ID2D1GradientStopCollection",
    "Windows.Win32.Graphics.Direct2D.ID2D1Layer",
    "Windows.Win32.Graphics.Direct2D.ID2D1LinearGradientBrush",
    "Windows.Win32.Graphics.Direct2D.ID2D1PathGeometry",
    "Windows.Win32.Graphics.Direct2D.ID2D1RoundedRectangleGeometry",
    "Windows.Win32.Graphics.Direct2D.ID2D1SolidColorBrush",
    "Windows.Win32.Graphics.Direct2D.ID2D1StrokeStyle",
    "Windows.Win32.Graphics.Direct2D.Common.D2D_RECT_F",
    "Windows.Win32.Graphics.Direct2D.Common.D2D_SIZE_F",
    "Windows.Win32.Graphics.Direct2D.Common.D2D_SIZE_U",
    "Windows.Win32.Graphics.Direct2D.Common.D2D1_ALPHA_MODE_PREMULTIPLIED",
    "Windows.Win32.Graphics.Direct2D.Common.D2D1_COLOR_F",
    "Windows.Win32.Graphics.Direct2D.Common.D2D1_FIGURE_BEGIN_FILLED",
    "Windows.Win32.Graphics.Direct2D.Common.D2D1_FIGURE_END_CLOSED",
    "Windows.Win32.Graphics.Direct2D.Common.D2D1_GRADIENT_STOP",
    "Windows.Win32.Graphics.Direct2D.Common.D2D1_PIXEL_FORMAT",
    "Windows.Win32.Graphics.Direct2D.Common.ID2D1SimplifiedGeometrySink",
    "Windows.Win32.Graphics.DirectWrite.DWriteCreateFactory",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_FACTORY_TYPE_SHARED",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_FONT_FEATURE",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_FONT_FEATURE_TAG_TABULAR_FIGURES",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_FONT_STRETCH_NORMAL",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_FONT_STYLE_NORMAL",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_FONT_WEIGHT_BOLD",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_FONT_WEIGHT_MEDIUM",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_FONT_WEIGHT_NORMAL",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_FONT_WEIGHT_SEMI_BOLD",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_PARAGRAPH_ALIGNMENT_CENTER",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_TEXT_ALIGNMENT_CENTER",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_TEXT_ALIGNMENT_LEADING",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_TEXT_ALIGNMENT_TRAILING",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_TEXT_METRICS",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_TEXT_RANGE",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_TRIMMING",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_TRIMMING_GRANULARITY_CHARACTER",
    "Windows.Win32.Graphics.DirectWrite.DWRITE_WORD_WRAPPING_NO_WRAP",
    "Windows.Win32.Graphics.DirectWrite.IDWriteFactory",
    "Windows.Win32.Graphics.DirectWrite.IDWriteFontCollection",
    "Windows.Win32.Graphics.DirectWrite.IDWriteInlineObject",
    "Windows.Win32.Graphics.DirectWrite.IDWriteTextFormat",
    "Windows.Win32.Graphics.DirectWrite.IDWriteTextLayout",
    "Windows.Win32.Graphics.DirectWrite.IDWriteTypography",
    // Win32 — Direct3D11 & DXGI (snipsnap-rec & NowPlaying)
    "Windows.Win32.Graphics.Direct3D.D3D_DRIVER_TYPE_HARDWARE",
    "Windows.Win32.Graphics.Direct3D.D3D_FEATURE_LEVEL_11_0",
    "Windows.Win32.Graphics.Direct3D.D3D_FEATURE_LEVEL_11_1",
    "Windows.Win32.Graphics.Direct3D11.D3D11CreateDevice",
    "Windows.Win32.Graphics.Direct3D11.D3D11_BIND_RENDER_TARGET",
    "Windows.Win32.Graphics.Direct3D11.D3D11_BIND_SHADER_RESOURCE",
    "Windows.Win32.Graphics.Direct3D11.D3D11_BOX",
    "Windows.Win32.Graphics.Direct3D11.D3D11_CREATE_DEVICE_BGRA_SUPPORT",
    "Windows.Win32.Graphics.Direct3D11.D3D11_CREATE_DEVICE_VIDEO_SUPPORT",
    "Windows.Win32.Graphics.Direct3D11.D3D11_SDK_VERSION",
    "Windows.Win32.Graphics.Direct3D11.D3D11_SUBRESOURCE_DATA",
    "Windows.Win32.Graphics.Direct3D11.D3D11_TEX2D_VPIV",
    "Windows.Win32.Graphics.Direct3D11.D3D11_TEX2D_VPOV",
    "Windows.Win32.Graphics.Direct3D11.D3D11_TEXTURE2D_DESC",
    "Windows.Win32.Graphics.Direct3D11.D3D11_USAGE_DEFAULT",
    "Windows.Win32.Graphics.Direct3D11.D3D11_VIDEO_FRAME_FORMAT_PROGRESSIVE",
    "Windows.Win32.Graphics.Direct3D11.D3D11_VIDEO_PROCESSOR_CONTENT_DESC",
    "Windows.Win32.Graphics.Direct3D11.D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC",
    "Windows.Win32.Graphics.Direct3D11.D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC",
    "Windows.Win32.Graphics.Direct3D11.D3D11_VIDEO_PROCESSOR_STREAM",
    "Windows.Win32.Graphics.Direct3D11.D3D11_VPIV_DIMENSION_TEXTURE2D",
    "Windows.Win32.Graphics.Direct3D11.D3D11_VPOV_DIMENSION_TEXTURE2D",
    "Windows.Win32.Graphics.Direct3D11.ID3D11Device",
    "Windows.Win32.Graphics.Direct3D11.ID3D11DeviceContext",
    "Windows.Win32.Graphics.Direct3D11.ID3D11Multithread",
    "Windows.Win32.Graphics.Direct3D11.ID3D11Texture2D",
    "Windows.Win32.Graphics.Direct3D11.ID3D11VideoContext",
    "Windows.Win32.Graphics.Direct3D11.ID3D11VideoDevice",
    "Windows.Win32.Graphics.Direct3D11.ID3D11VideoProcessor",
    "Windows.Win32.Graphics.Direct3D11.ID3D11VideoProcessorEnumerator",
    "Windows.Win32.Graphics.Direct3D11.ID3D11VideoProcessorOutputView",
    "Windows.Win32.Graphics.Dxgi.IDXGIDevice",
    "Windows.Win32.Graphics.Dxgi.Common.DXGI_FORMAT_B8G8R8A8_UNORM",
    "Windows.Win32.Graphics.Dxgi.Common.DXGI_FORMAT_NV12",
    "Windows.Win32.Graphics.Dxgi.Common.DXGI_RATIONAL",
    "Windows.Win32.Graphics.Dxgi.Common.DXGI_SAMPLE_DESC",
    // Win32 — GDI & WIC (NowPlaying & snipsnap-rec)
    "Windows.Win32.Graphics.Gdi.HBITMAP",
    "Windows.Win32.Graphics.Gdi.HDC",
    "Windows.Win32.Graphics.Gdi.HMONITOR",
    "Windows.Win32.Graphics.Gdi.HPALETTE",
    "Windows.Win32.Graphics.Imaging.CLSID_WICImagingFactory",
    "Windows.Win32.Graphics.Imaging.GUID_ContainerFormatPng",
    "Windows.Win32.Graphics.Imaging.GUID_WICPixelFormat32bppPBGRA",
    "Windows.Win32.Graphics.Imaging.IWICBitmap",
    "Windows.Win32.Graphics.Imaging.IWICBitmapDecoder",
    "Windows.Win32.Graphics.Imaging.IWICBitmapEncoder",
    "Windows.Win32.Graphics.Imaging.IWICBitmapFrameDecode",
    "Windows.Win32.Graphics.Imaging.IWICBitmapFrameEncode",
    "Windows.Win32.Graphics.Imaging.IWICBitmapScaler",
    "Windows.Win32.Graphics.Imaging.IWICBitmapSource",
    "Windows.Win32.Graphics.Imaging.IWICFormatConverter",
    "Windows.Win32.Graphics.Imaging.IWICImagingFactory",
    "Windows.Win32.Graphics.Imaging.IWICPalette",
    "Windows.Win32.Graphics.Imaging.IWICStream",
    "Windows.Win32.Graphics.Imaging.WICRect",
    "Windows.Win32.Graphics.Imaging.WICBitmapDitherTypeNone",
    "Windows.Win32.Graphics.Imaging.WICBitmapEncoderNoCache",
    "Windows.Win32.Graphics.Imaging.WICBitmapInterpolationModeFant",
    "Windows.Win32.Graphics.Imaging.WICBitmapPaletteTypeCustom",
    "Windows.Win32.Graphics.Imaging.WICBitmapUseAlpha",
    "Windows.Win32.Graphics.Imaging.WICDecodeMetadataCacheOnDemand",
    // Win32 — Core Audio, Kernel Streaming & Media Foundation
    "Windows.Win32.Media.Audio.AUDCLNT_BUFFERFLAGS_SILENT",
    "Windows.Win32.Media.Audio.AUDCLNT_SHAREMODE_SHARED",
    "Windows.Win32.Media.Audio.AUDCLNT_STREAMFLAGS_LOOPBACK",
    "Windows.Win32.Media.Audio.DEVICE_STATE",
    "Windows.Win32.Media.Audio.DEVICE_STATE_ACTIVE",
    "Windows.Win32.Media.Audio.DEVICE_STATEMASK_ALL",
    "Windows.Win32.Media.Audio.EDataFlow",
    "Windows.Win32.Media.Audio.ERole",
    "Windows.Win32.Media.Audio.eConsole",
    "Windows.Win32.Media.Audio.eRender",
    "Windows.Win32.Media.Audio.IAudioCaptureClient",
    "Windows.Win32.Media.Audio.IAudioClient",
    "Windows.Win32.Media.Audio.IConnector",
    "Windows.Win32.Media.Audio.IDeviceTopology",
    "Windows.Win32.Media.Audio.IMMDevice",
    "Windows.Win32.Media.Audio.IMMDeviceCollection",
    "Windows.Win32.Media.Audio.IMMDeviceEnumerator",
    "Windows.Win32.Media.Audio.IMMNotificationClient",
    "Windows.Win32.Media.Audio.IPart",
    "Windows.Win32.Media.Audio.MMDeviceEnumerator",
    "Windows.Win32.Media.Audio.WAVEFORMATEXTENSIBLE",
    "Windows.Win32.Media.KernelStreaming.IKsControl",
    "Windows.Win32.Media.KernelStreaming.KSIDENTIFIER",
    "Windows.Win32.Media.MediaFoundation.eAVEncH264VProfile_Main",
    "Windows.Win32.Media.MediaFoundation.IMF2DBuffer",
    "Windows.Win32.Media.MediaFoundation.IMFAttributes",
    "Windows.Win32.Media.MediaFoundation.IMFDXGIDeviceManager",
    "Windows.Win32.Media.MediaFoundation.IMFMediaType",
    "Windows.Win32.Media.MediaFoundation.IMFSinkWriter",
    "Windows.Win32.Media.MediaFoundation.MFCreateAttributes",
    "Windows.Win32.Media.MediaFoundation.MFCreateDXGIDeviceManager",
    "Windows.Win32.Media.MediaFoundation.MFCreateDXGISurfaceBuffer",
    "Windows.Win32.Media.MediaFoundation.MFCreateMediaType",
    "Windows.Win32.Media.MediaFoundation.MFCreateSample",
    "Windows.Win32.Media.MediaFoundation.MFCreateSinkWriterFromURL",
    "Windows.Win32.Media.MediaFoundation.MFMediaType_Video",
    "Windows.Win32.Media.MediaFoundation.MFShutdown",
    "Windows.Win32.Media.MediaFoundation.MFStartup",
    "Windows.Win32.Media.MediaFoundation.MFSTARTUP_LITE",
    "Windows.Win32.Media.MediaFoundation.MFTranscodeContainerType_MPEG4",
    "Windows.Win32.Media.MediaFoundation.MFVideoFormat_ARGB32",
    "Windows.Win32.Media.MediaFoundation.MFVideoFormat_H264",
    "Windows.Win32.Media.MediaFoundation.MFVideoFormat_NV12",
    "Windows.Win32.Media.MediaFoundation.MFVideoInterlace_Progressive",
    "Windows.Win32.Media.MediaFoundation.MF_LOW_LATENCY",
    "Windows.Win32.Media.MediaFoundation.MF_MT_AVG_BITRATE",
    "Windows.Win32.Media.MediaFoundation.MF_MT_FRAME_RATE",
    "Windows.Win32.Media.MediaFoundation.MF_MT_FRAME_SIZE",
    "Windows.Win32.Media.MediaFoundation.MF_MT_INTERLACE_MODE",
    "Windows.Win32.Media.MediaFoundation.MF_MT_MAJOR_TYPE",
    "Windows.Win32.Media.MediaFoundation.MF_MT_MPEG2_PROFILE",
    "Windows.Win32.Media.MediaFoundation.MF_MT_PIXEL_ASPECT_RATIO",
    "Windows.Win32.Media.MediaFoundation.MF_MT_SUBTYPE",
    "Windows.Win32.Media.MediaFoundation.MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS",
    "Windows.Win32.Media.MediaFoundation.MF_SINK_WRITER_D3D_MANAGER",
    "Windows.Win32.Media.MediaFoundation.MF_SINK_WRITER_STATISTICS",
    "Windows.Win32.Media.MediaFoundation.MF_TRANSCODE_CONTAINERTYPE",
    "Windows.Win32.Media.MediaFoundation.MF_VERSION",
    // Win32 — COM, WinRT interop & Shell
    "Windows.Win32.System.Com.CLSCTX_ALL",
    "Windows.Win32.System.Com.CLSCTX_INPROC_SERVER",
    "Windows.Win32.System.Com.COINIT_APARTMENTTHREADED",
    "Windows.Win32.System.Com.COINIT_MULTITHREADED",
    "Windows.Win32.System.Com.CoCreateInstance",
    "Windows.Win32.System.Com.CoInitializeEx",
    "Windows.Win32.System.Com.CoTaskMemFree",
    "Windows.Win32.System.Com.STGM_READ",
    "Windows.Win32.System.Com.StructuredStorage.IPropertyBag2",
    "Windows.Win32.System.Com.StructuredStorage.PROPVARIANT",
    "Windows.Win32.System.Com.StructuredStorage.PropVariantClear",
    "Windows.Win32.System.WinRT.RO_INIT_MULTITHREADED",
    "Windows.Win32.System.WinRT.RO_INIT_SINGLETHREADED",
    "Windows.Win32.System.WinRT.RoInitialize",
    "Windows.Win32.System.WinRT.Direct3D11.CreateDirect3D11DeviceFromDXGIDevice",
    "Windows.Win32.System.WinRT.Direct3D11.IDirect3DDxgiInterfaceAccess",
    "Windows.Win32.System.WinRT.Graphics.Capture.IGraphicsCaptureItemInterop",
    "Windows.Win32.UI.Shell.FOLDERID_AppsFolder",
    "Windows.Win32.UI.Shell.IShellItemImageFactory",
    "Windows.Win32.UI.Shell.KF_FLAG_DEFAULT",
    "Windows.Win32.UI.Shell.SHCreateItemFromParsingName",
    "Windows.Win32.UI.Shell.SHCreateItemInKnownFolder",
    "Windows.Win32.UI.Shell.SIIGBF_ICONONLY",
    "Windows.Win32.UI.Shell.SIIGBF_RESIZETOFIT",
    "Windows.Win32.UI.Shell.PropertiesSystem.IPropertyStore",
];

/// Types taken from the small standalone windows-rs crates (matching windows 0.62's layout).
const REFERENCES: &[&str] = &[
    "windows_collections,flat,Windows.Foundation.Collections.IIterable",
    "windows_collections,flat,Windows.Foundation.Collections.IIterator",
    "windows_collections,flat,Windows.Foundation.Collections.IKeyValuePair",
    "windows_collections,flat,Windows.Foundation.Collections.IMap",
    "windows_collections,flat,Windows.Foundation.Collections.IMapView",
    "windows_collections,flat,Windows.Foundation.Collections.IVector",
    "windows_collections,flat,Windows.Foundation.Collections.IVectorView",
    "windows_numerics,flat,Windows.Foundation.Numerics",
    "windows_future,flat,Windows.Foundation.Async*",
    "windows_future,flat,Windows.Foundation.IAsync*",
    "windows_core,flat,Windows.Win32.Foundation.WIN32_ERROR",
    "windows_core,flat,Windows.Win32.Foundation.NTSTATUS",
    "windows_core,flat,Windows.Win32.System.Rpc.RPC_STATUS",
];

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir.join("../../..");
    let out = manifest_dir.join("../src/bindings.rs");
    let mut args: Vec<String> = vec![
        "--in".into(),
        "default".into(),
        "--out".into(),
        out.display().to_string(),
        "--no-deps".into(),
        "--no-comment".into(),
        "--implement".into(),
    ];
    for reference in REFERENCES {
        args.extend(["--reference".into(), (*reference).into()]);
    }
    args.push("--filter".into());
    for item in FILTER {
        args.push((*item).into());
    }
    let warnings = windows_bindgen::bindgen(&args);
    print!("{warnings}");
    let code = std::fs::read_to_string(&out).expect("read bindings.rs");
    let used = workspace_names(&repo_root);
    let (code, kept, removed) = trim(&code, &used);
    assert!(removed > 0, "trimming removed no methods");
    std::fs::write(&out, &code).expect("write bindings.rs");
    println!("wrote {} ({:.1} KB; methods kept: {kept}, removed: {removed})", out.display(), code.len() as f64 / 1024.0);
}

/// Every identifier appearing in workspace Rust sources outside `crates/pk-windows`.
fn workspace_names(root: &Path) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for dir in ["blades", "runner", "crates/pk"] {
        collect_rs(&root.join(dir), &mut names);
    }
    names
}

fn collect_rs(dir: &Path, names: &mut BTreeSet<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if !path.file_name().is_some_and(|n| n == "target" || n == "node_modules") {
                collect_rs(&path, names);
            }
        } else if path.extension().is_some_and(|e| e == "rs") {
            let code = std::fs::read_to_string(&path).expect("read source");
            names.extend(code.split(|c: char| !(c.is_alphanumeric() || c == '_')).filter(|w| !w.is_empty()).map(String::from));
        }
    }
}

/// Removes each `pub [unsafe] fn` wrapper method whose name isn't in `used` and isn't called inside
/// `bindings.rs` itself. Vtables, trait impls, structs, and top-level Win32 `pub unsafe fn`
/// functions are kept intact.
fn trim(code: &str, used: &BTreeSet<String>) -> (String, usize, usize) {
    let lines: Vec<&str> = code.lines().collect();
    let mut keep: BTreeSet<String> = used.clone();
    // Keep methods referenced internally by generated trait impls (e.g. `self.First()`).
    for line in &lines {
        let mut rest = *line;
        while let Some(at) = rest.find("self.") {
            rest = &rest[at + 5..];
            let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
            if rest[name.len()..].starts_with('(') {
                keep.insert(name);
            }
        }
    }
    let mut out = String::with_capacity(code.len());
    let (mut kept, mut removed) = (0, 0);
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();
        // Drop unused `pub trait Foo_Impl` and `impl Foo_Vtbl { pub const fn new<Identity: Foo_Impl, ...> }`
        // blocks generated by `--implement` (only 3 interfaces are implemented in the workspace).
        let unused_impl_block = if let Some(rest) = trimmed.strip_prefix("pub trait ") {
            let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
            name.ends_with("_Impl") && !keep.contains(&name)
        } else if let Some(rest) = trimmed.strip_prefix("impl").and_then(|s| s.split("_Vtbl").next()) {
            let is_vtbl_impl = trimmed.contains("_Vtbl") && !trimmed.contains(" for ");
            let impl_name =
                rest.rsplit(|c: char| !(c.is_alphanumeric() || c == '_')).next().map(|s| format!("{s}_Impl")).unwrap_or_default();
            is_vtbl_impl && !impl_name.is_empty() && !keep.contains(&impl_name)
        } else {
            false
        };
        if unused_impl_block {
            let close = format!("{}}}", &line[..indent]);
            let end = if line.trim_end().ends_with('}') {
                i
            } else {
                (i + 1..lines.len()).find(|&j| lines[j].trim_end() == close).expect("end of impl/trait block")
            };
            removed += 1;
            i = end + 1;
            continue;
        }
        let fn_rest = trimmed.strip_prefix("pub fn ").or_else(|| trimmed.strip_prefix("pub unsafe fn "));
        if let Some(rest) = fn_rest {
            let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
            let end = if line.trim_end().ends_with('}') {
                i
            } else {
                let close = format!("{}}}", &line[..indent]);
                (i + 1..lines.len()).find(|&j| lines[j].trim_end() == close).expect("end of method block")
            };
            let is_impl_method = i > 0 && indent >= 8 && !lines[i..=end].iter().any(|l| l.contains("windows_core::link!"));
            if is_impl_method && !keep.contains(&name) {
                removed += 1;
                i = end + 1;
                continue;
            }
            kept += 1;
        }
        out.push_str(line);
        out.push('\n');
        i += 1;
    }
    (out, kept, removed)
}
