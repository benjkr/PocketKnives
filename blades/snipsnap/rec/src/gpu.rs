//! Background pre-warm and ownership of the D3D11 + WinRT Direct3D + Media Foundation DXGI
//! device manager. Dropping `GpuContext` releases all GPU / Media Foundation handles immediately.
use windows::Graphics::Capture::GraphicsCaptureItem;
use windows::Graphics::DirectX::Direct3D11::IDirect3DDevice;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE_HARDWARE, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1};
use windows::Win32::Graphics::Direct3D11::{
    D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_CREATE_DEVICE_VIDEO_SUPPORT, D3D11_SDK_VERSION, D3D11CreateDevice, ID3D11Device,
    ID3D11DeviceContext, ID3D11Multithread, ID3D11VideoContext, ID3D11VideoDevice,
};
use windows::Win32::Graphics::Dxgi::IDXGIDevice;
use windows::Win32::Media::MediaFoundation::{
    IMFDXGIDeviceManager, MF_VERSION, MFCreateDXGIDeviceManager, MFSTARTUP_LITE, MFShutdown, MFStartup,
};
use windows::Win32::System::WinRT::Direct3D11::CreateDirect3D11DeviceFromDXGIDevice;
use windows::Win32::System::WinRT::Graphics::Capture::IGraphicsCaptureItemInterop;
use windows::Win32::System::WinRT::{RO_INIT_MULTITHREADED, RoInitialize};
use windows_core::Interface;

/// Pre-warmed GPU & Media Foundation state shared across capture and encoding.
/// COM / WinRT interfaces created in MTA are safe to transfer across threads.
pub struct GpuContext {
    pub device: ID3D11Device,
    pub context: ID3D11DeviceContext,
    pub video_device: Option<ID3D11VideoDevice>,
    pub video_context: Option<ID3D11VideoContext>,
    pub winrt_device: IDirect3DDevice,
    pub mf_dxgi_manager: IMFDXGIDeviceManager,
}

unsafe impl Send for GpuContext {}

impl GpuContext {
    /// Initializes WinRT MTA, Media Foundation, D3D11 (multithread-protected), D3D11 Video,
    /// WinRT `IDirect3DDevice`, and `IMFDXGIDeviceManager`. Called on a background thread while
    /// the user selects a region or window.
    pub fn init() -> Option<Self> {
        let _ = unsafe { RoInitialize(RO_INIT_MULTITHREADED) };
        if unsafe { MFStartup(MF_VERSION, MFSTARTUP_LITE) }.is_err() {
            return None;
        }

        let levels = [D3D_FEATURE_LEVEL_11_1, D3D_FEATURE_LEVEL_11_0];
        let mut device: Option<ID3D11Device> = None;
        let mut context: Option<ID3D11DeviceContext> = None;
        let flags_video = D3D11_CREATE_DEVICE_BGRA_SUPPORT | D3D11_CREATE_DEVICE_VIDEO_SUPPORT;

        let hr = unsafe {
            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_HARDWARE,
                HMODULE::default(),
                flags_video,
                Some(&levels),
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                Some(&mut context),
            )
        };
        if hr.is_err() {
            let hr2 = unsafe {
                D3D11CreateDevice(
                    None,
                    D3D_DRIVER_TYPE_HARDWARE,
                    HMODULE::default(),
                    D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                    Some(&levels),
                    D3D11_SDK_VERSION,
                    Some(&mut device),
                    None,
                    Some(&mut context),
                )
            };
            if hr2.is_err() {
                let _ = unsafe { MFShutdown() };
                return None;
            }
        }

        let device = device?;
        let context = context?;

        if let Ok(mt) = device.cast::<ID3D11Multithread>() {
            unsafe {
                let _ = mt.SetMultithreadProtected(true);
            }
        }

        let video_device = device.cast::<ID3D11VideoDevice>().ok();
        let video_context = context.cast::<ID3D11VideoContext>().ok();

        let dxgi: IDXGIDevice = device.cast().ok()?;
        let inspectable = unsafe { CreateDirect3D11DeviceFromDXGIDevice(&dxgi) }.ok()?;
        let winrt_device: IDirect3DDevice = inspectable.cast().ok()?;

        let mut reset_token = 0u32;
        let mut mgr: Option<IMFDXGIDeviceManager> = None;
        if unsafe { MFCreateDXGIDeviceManager(&mut reset_token, &mut mgr) }.is_err() {
            let _ = unsafe { MFShutdown() };
            return None;
        }
        let mf_dxgi_manager = mgr?;
        if unsafe { mf_dxgi_manager.ResetDevice(&device, reset_token) }.is_err() {
            let _ = unsafe { MFShutdown() };
            return None;
        }

        // Pre-load the WinRT Graphics Capture activation factory while the user selects a target.
        let _ = windows_core::factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>();

        Some(Self { device, context, video_device, video_context, winrt_device, mf_dxgi_manager })
    }
}

impl Drop for GpuContext {
    fn drop(&mut self) {
        let _ = unsafe { MFShutdown() };
    }
}
