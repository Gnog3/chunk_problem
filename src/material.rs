use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::{MaterialPipeline, SpecializedMaterial},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
        render_resource::{
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, Face,
            RenderPipelineDescriptor, SamplerBindingType, ShaderStages,
            SpecializedMeshPipelineError, TextureSampleType, TextureViewDimension,
        },
        renderer::RenderDevice,
    }
};

#[derive(Default)]
pub struct ChunkMaterialPlugin;

impl Plugin for ChunkMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<ChunkMaterial>::default());
    }
}

#[derive(Debug, Clone, TypeUuid, Default)]
#[uuid = "0ace2487-94b3-4dcf-a72d-cbf5728ce0c2"]
pub struct ChunkMaterial {
    pub board_texture: Option<Handle<Image>>,
    pub two_state_texture: Option<Handle<Image>>,
    pub display_texture: Option<Handle<Image>>,
    pub board_color_texture: Option<Handle<Image>>,
}

#[derive(Debug, Clone)]
pub struct GpuChunkMaterial {
    pub bind_group: BindGroup,
}

impl RenderAsset for ChunkMaterial {
    type ExtractedAsset = Self;

    type PreparedAsset = GpuChunkMaterial;

    type Param = (
        SRes<RenderDevice>,
        SRes<MaterialPipeline<ChunkMaterial>>,
        SRes<RenderAssets<Image>>,
    );

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        material: Self::ExtractedAsset,
        (render_device, pbr_pipeline, gpu_images): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let (board_texture_view, board_sampler) = if let Some(result) = pbr_pipeline
            .mesh_pipeline
            .get_image_texture(gpu_images, &material.board_texture)
        {
            result
        } else {
            return Err(PrepareAssetError::RetryNextUpdate(material));
        };

        let (two_state_texture_view, two_state_sampler) = if let Some(result) = pbr_pipeline
            .mesh_pipeline
            .get_image_texture(gpu_images, &material.two_state_texture)
        {
            result
        } else {
            return Err(PrepareAssetError::RetryNextUpdate(material));
        };

        let (display_texture_view, display_sampler) = if let Some(result) = pbr_pipeline
            .mesh_pipeline
            .get_image_texture(gpu_images, &material.display_texture)
        {
            result
        } else {
            return Err(PrepareAssetError::RetryNextUpdate(material));
        };

        let (board_color_texture_view, board_color_sampler) = if let Some(result) = pbr_pipeline
            .mesh_pipeline
            .get_image_texture(gpu_images, &material.board_color_texture)
        {
            result
        } else {
            return Err(PrepareAssetError::RetryNextUpdate(material));
        };

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: Some("pbr_chunk_material_bind_group"),
            layout: &pbr_pipeline.material_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(board_texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(board_sampler),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::TextureView(two_state_texture_view),
                },
                BindGroupEntry {
                    binding: 3,
                    resource: BindingResource::Sampler(two_state_sampler),
                },
                BindGroupEntry {
                    binding: 4,
                    resource: BindingResource::TextureView(display_texture_view),
                },
                BindGroupEntry {
                    binding: 5,
                    resource: BindingResource::Sampler(display_sampler),
                },
                BindGroupEntry {
                    binding: 6,
                    resource: BindingResource::TextureView(board_color_texture_view),
                },
                BindGroupEntry {
                    binding: 7,
                    resource: BindingResource::Sampler(board_color_sampler),
                },
            ],
        });
        Ok(GpuChunkMaterial { bind_group })
    }
}

impl SpecializedMaterial for ChunkMaterial {
    type Key = ();

    fn key(_material: &<Self as RenderAsset>::PreparedAsset) -> Self::Key {
        ()
    }

    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/chunk.wgsl"))
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/chunk.wgsl"))
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _key: Self::Key,
        _layout: &MeshVertexBufferLayout,
    ) -> Result<(), SpecializedMeshPipelineError> {
        error!("Hey, i'm here! {:?}", _layout);
        descriptor.primitive.cull_mode = Some(Face::Back);
        if let Some(label) = &mut descriptor.label {
            *label = format!("pbr_chunk_{}", *label).into();
        }
        error!("{:?}", descriptor);
        Ok(())
    }

    #[inline]
    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("pbr_chunk_material_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 4,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 5,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 6,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: false },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 7,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
        })
    }
}
