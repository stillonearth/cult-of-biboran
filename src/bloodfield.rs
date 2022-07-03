use bevy::{
    core::FixedTimestep,
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    math::const_vec3,
    pbr::MaterialPipeline,
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            std140::{AsStd140, Std140},
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType,
            BufferInitDescriptor, BufferSize, BufferUsages, ShaderStages,
        },
        renderer::RenderDevice,
    },
};
use bevy_inspector_egui::WorldInspectorPlugin;
use rand::Rng;

#[derive(Component)]
struct ScreenTag;

pub struct BloodfieldPlugin;
impl Plugin for BloodfieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<BloodfieldMaterial>::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.05))
                    .with_system(update_bloodfield_material),
            );
    }
}

#[allow(clippy::type_complexity)]
fn update_bloodfield_material(
    time: Res<Time>,
    mut bloodfield_materials: ResMut<Assets<BloodfieldMaterial>>,
) {
    for (_id, mut bloodfield_material) in bloodfield_materials.iter_mut() {
        bloodfield_material.time = time.seconds_since_startup() as f32;
    }
}

#[derive(Component, Debug, Clone, TypeUuid, Default)]
#[uuid = "AC784C13-7197-40AB-BC3A-2ADD64F9E242"]
pub struct BloodfieldMaterial {
    pub time: f32,
    pub seed: f32,
}

#[derive(Clone)]
pub struct GpuBloodfieldMaterial {
    bind_group: BindGroup,
}

impl RenderAsset for BloodfieldMaterial {
    type ExtractedAsset = BloodfieldMaterial;
    type PreparedAsset = GpuBloodfieldMaterial;
    type Param = (SRes<RenderDevice>, SRes<MaterialPipeline<Self>>);
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let value = Vec4::new(extracted_asset.time, extracted_asset.seed, 0.0, 0.0);
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: value.as_std140().as_bytes(),
            label: Some("Bloodfield Settings Buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("Bloodfield BindGroup"),
            layout: &material_pipeline.material_layout,
        });

        Ok(GpuBloodfieldMaterial { bind_group })
    }
}

impl Material for BloodfieldMaterial {
    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/bloodfield.wgsl"))
    }
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/bloodfield.wgsl"))
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(Vec4::std140_size_static() as u64),
                },
                count: None,
            }],
            label: Some("Bloodfield BindGroup Layout"),
        })
    }
}
