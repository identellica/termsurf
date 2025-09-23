//! macOS IOSurface texture import implementation

use super::common::{format, texture};
use super::{TextureImportError, TextureImportResult, TextureImporter};
use crate::{sys::cef_color_type_t, AcceleratedPaintInfo};
use core_foundation::base::{CFType, TCFType};
use objc2_io_surface::{IOSurface, IOSurfaceRef};
use std::cell::RefCell;
use std::ptr::null_mut;
use wgpu::{Extent3d, TextureDescriptor, TextureDimension, TextureUsages};

use std::os::raw::c_void;
use wgpu::hal::api;

pub struct IOSurfaceImporter {
    pub handle: *mut c_void,
    pub format: cef_color_type_t,
    pub width: u32,
    pub height: u32,
}

impl TextureImporter for IOSurfaceImporter {
    fn new(info: &AcceleratedPaintInfo) -> Self {
        Self {
            handle: info.shared_texture_handle,
            format: *info.format.as_ref(),
            width: info.extra.coded_size.width as u32,
            height: info.extra.coded_size.height as u32,
        }
    }

    fn import_to_wgpu(&self, device: &wgpu::Device) -> TextureImportResult {
        // Try hardware acceleration first
        if self.supports_hardware_acceleration(device) {
            match self.import_via_metal(device) {
                Ok(texture) => {
                    tracing::trace!("Successfully imported IOSurface texture via Metal");
                    return Ok(texture);
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to import IOSurface via Metal: {}, falling back to CPU texture",
                        e
                    );
                }
            }
        }

        // Fallback to CPU texture
        texture::create_fallback(
            device,
            self.width,
            self.height,
            self.format,
            "CEF IOSurface Texture (fallback)",
        )
    }

    fn supports_hardware_acceleration(&self, device: &wgpu::Device) -> bool {
        // Check if handle is valid
        if self.handle.is_null() {
            return false;
        }

        // Check if wgpu is using Metal backend
        self.is_metal_backend(device)
    }
}

impl IOSurfaceImporter {
    fn get_metal_desc(&self) -> metal::TextureDescriptor {
        use metal::{MTLPixelFormat, MTLTextureType, MTLTextureUsage};

        if self.width == 0 || self.height == 0 {
            return Err(TextureImportError::InvalidHandle(
                "Invalid IOSurface texture dimensions".to_string(),
            ));
        }

        let metal_desc = metal::TextureDescriptor::new();
        metal_desc.set_width(self.width as _);
        metal_desc.set_height(self.height as _);
        // metal_desc.set_array_length(texture_desc.array_layer_count() as _);
        metal_desc.set_mipmap_level_count(1);
        metal_desc.set_sample_count(1);
        metal_desc.set_depth(1);
        metal_desc.set_texture_type(MTLTextureType::D2);
        metal_desc.set_usage(MTLTextureUsage::ShaderRead);
        metal_desc.set_pixel_format(match texture_desc.format {
            wgpu::TextureFormat::Rgba8Unorm => MTLPixelFormat::RGBA8Unorm,
            wgpu::TextureFormat::Bgra8Unorm => MTLPixelFormat::BGRA8Unorm,
            _ => unimplemented!(),
        });
        metal_desc.set_storage_mode(metal::MTLStorageMode::Managed);

        metal_desc
    }

    fn import_via_metal(&self, device: &wgpu::Device) -> TextureImportResult {
        // Get wgpu's Metal device
        use wgpu::{
            hal::TextureDescriptor, wgc::api::Metal, Extent3d, TextureDimension, TextureUses,
        };

        let texture = unsafe {
            // Convert handle to IOSurface
            let iosurface = unsafe {
                let cf_type = CFType::wrap_under_get_rule(self.handle as IOSurfaceRef);
                IOSurface::from(cf_type)
            };

            let texture_desc = self.get_metal_desc();

            device.as_hal::<api::Metal, _, _>(|device| {
                let texture = device
                        .as_hal::<wgpu::wgc::api::Metal, _, _>(|hdevice| {
                            hdevice.map(|hdevice|  {
                                use objc::*;
                                objc::msg_send![std::mem::transmute::<_,&metal::NSObject>(hdevice.raw_device().lock().as_ref()),
                                    newTextureWithDescriptor:std::mem::transmute::<_,&metal::NSObject>(metal_desc.as_ref())
                                                                        iosurface:io_surface
                                                                            plane:0]
                            })
                        }).unwrap();

                let hal_tex = <wgpu::wgc::api::Metal as wgpu::hal::Api>::Device::texture_from_raw(
                    texture,
                    texture_desc.format,
                    MTLTextureType::D2,
                    texture_desc.array_layer_count(),
                    texture_desc.mip_level_count,
                    wgpu::hal::CopyExtent {
                        width: texture_desc.size.width,
                        height: texture_desc.size.height,
                        depth: texture_desc.array_layer_count(),
                    },
                );

                device.create_texture_from_hal::<wgpu::wgc::api::Metal>(hal_tex, &texture_desc)
            })
        }?;

        Ok(texture)
    }

    fn is_metal_backend(&self, device: &wgpu::Device) -> bool {
        use wgpu::hal::api;
        let mut is_metal = false;
        unsafe {
            device.as_hal::<api::Metal, _, _>(|device| {
                is_metal = device.is_some();
            });
        }
        is_metal
    }
}
