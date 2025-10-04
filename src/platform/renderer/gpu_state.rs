//! GPUステート管理モジュール
//! WGPUのインスタンス、サーフェス、アダプタ、デバイスなどの
//! 基本的なGPU関連オブジェクトを管理します。

use anyhow::{anyhow, Result};
use log::{debug, error, info};
use std::marker::PhantomData;
use wgpu::{
    Adapter, Backends, Device, DeviceDescriptor, Instance, InstanceDescriptor, Queue,
    RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages,
};
use winit::{dpi::PhysicalSize, window::Window};

/// WGPUの状態を管理するための構造体
pub struct GpuState<'a> {
    /// WGPUインスタンス
    pub instance: Instance,
    /// ウィンドウ表面
    pub surface: Surface<'a>,
    /// グラフィックアダプタ
    pub adapter: Adapter,
    /// GPUデバイス
    pub device: Device,
    /// コマンドキュー
    pub queue: Queue,
    /// サーフェス設定
    pub config: SurfaceConfiguration,
    /// 現在のウィンドウサイズ
    pub size: PhysicalSize<u32>,
    _phantom: PhantomData<&'a Window>,
}

impl<'a> GpuState<'a> {
    /// 新しいGPUステートを初期化
    ///
    /// # 引数
    /// * `window` - レンダリング先のウィンドウ
    ///
    /// # 戻り値
    /// * 初期化されたGPUステート
    pub async fn new(window: &'a Window) -> Result<Self> {
        let size = window.inner_size();

        // WGPUのインスタンスを作成
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        // ウィンドウからサーフェスを作成
        let surface = unsafe { instance.create_surface(window) }?;

        // アダプタをリクエスト
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow!("適切なアダプタが見つかりませんでした"))?;

        debug!("選択されたアダプタ: {:?}", adapter.get_info());

        // デバイスとキューを作成
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("GPU Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;

        // サーフェスの設定
        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(caps.formats[0]);

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 1,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        info!("GPUステートが正常に初期化されました");

        Ok(Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            config,
            size,
            _phantom: PhantomData,
        })
    }

    /// ウィンドウのリサイズに対応
    ///
    /// # 引数
    /// * `new_size` - 新しいウィンドウサイズ
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            debug!(
                "サーフェスのリサイズ: {}x{}",
                new_size.width, new_size.height
            );
        } else {
            error!(
                "無効なウィンドウサイズ: {}x{}",
                new_size.width, new_size.height
            );
        }
    }

    /// フレームをレンダリングするためのサーフェステクスチャを取得
    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture> {
        self.surface
            .get_current_texture()
            .map_err(|e| anyhow!("{}", e))
    }
}
