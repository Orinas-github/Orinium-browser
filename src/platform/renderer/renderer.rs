//! レンダラーモジュール
//! WGPUを使用した描画パイプラインと実際のレンダリング処理を提供します。

use crate::platform::renderer::gpu_state::GpuState;
use anyhow::Result;
use std::{mem::size_of, sync::Arc};
use wgpu::{
    util::DeviceExt, Buffer, CommandEncoder, RenderPipeline, ShaderModuleDescriptor, ShaderSource,
};

/// 頂点データ構造体
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    /// 頂点位置
    pub position: [f32; 3],
    /// 頂点色
    pub color: [f32; 3],
}

impl Vertex {
    /// 頂点バッファレイアウトの記述子を作成
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

/// レンダラー構造体 - 実際の描画ロジックを管理
pub struct Renderer<'a> {
    /// GPUの状態
    gpu_state: Arc<GpuState<'a>>,
    /// レンダリングパイプライン
    pipeline: RenderPipeline,
    /// 描画用の頂点バッファ
    vertex_buffer: Buffer,
    /// インデックスバッファ
    index_buffer: Buffer,
    /// インデックス数
    num_indices: u32,
}

impl<'a> Renderer<'a> {
    /// 新しいレンダラーを作成
    ///
    /// # 引数
    /// * `gpu_state` - GPUの状態
    ///
    /// # 戻り値
    /// * 初期化されたレンダラー
    pub fn new(gpu_state: Arc<GpuState<'a>>) -> Result<Self> {
        // シェーダーの読み込み
        let shader = gpu_state
            .device
            .create_shader_module(ShaderModuleDescriptor {
                label: Some("Shader"),
                source: ShaderSource::Wgsl(include_str!("shaders/shader.wgsl").into()),
            });

        // パイプラインレイアウトの作成
        let pipeline_layout =
            gpu_state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        // レンダリングパイプラインの作成
        let pipeline = gpu_state
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: gpu_state.config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        // 頂点バッファのデータ定義
        let vertices = [
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [0.0, 0.0, 1.0],
            },
        ];

        // 頂点バッファの作成
        let vertex_buffer =
            gpu_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        // インデックスデータ
        let indices: &[u16] = &[0, 1, 2];
        let num_indices = indices.len() as u32;

        // インデックスバッファの作成
        let index_buffer = gpu_state
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        Ok(Self {
            gpu_state,
            pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
        })
    }

    /// フレームをレンダリング
    pub fn render(&self) -> Result<()> {
        // フレームの取得
        let output = self.gpu_state.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // コマンドエンコーダの作成
        let mut encoder =
            self.gpu_state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        self.encode_render_pass(&mut encoder, &view);

        // コマンドの送信
        self.gpu_state
            .queue
            .submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// レンダーパスをエンコード
    fn encode_render_pass<'b>(
        &'b self,
        encoder: &'a mut CommandEncoder,
        view: &'a wgpu::TextureView,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // パイプラインとバッファの設定
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        // 描画コマンド
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
