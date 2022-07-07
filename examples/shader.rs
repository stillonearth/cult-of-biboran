use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::MaterialPipeline,
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            std140::{AsStd140, Std140},
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer,
            BufferBindingType, BufferInitDescriptor, BufferSize, BufferUsages, ShaderStages,
        },
        renderer::RenderDevice,
    },
};

use bytes::{BufMut, BytesMut};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<FireMaterial>::default())
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FireMaterial>>,
) {
    // cube
    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(FireMaterial {
            base_color: Color::GREEN,
            flame_height: 10.0,
            distorsion_level: 1.0,
            bottom_threshold: 1.0,
            time: 0.0,
        }),
        ..default()
    });

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// This is the struct that will be passed to your shader
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct FireMaterial {
    pub base_color: Color,
    pub flame_height: f32,
    pub distorsion_level: f32,
    pub bottom_threshold: f32,
    pub time: f32,
}

#[derive(Clone)]
pub struct GpuFireMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

// The implementation of [`Material`] needs this impl to work properly.
impl RenderAsset for FireMaterial {
    type ExtractedAsset = FireMaterial;
    type PreparedAsset = GpuFireMaterial;
    type Param = (SRes<RenderDevice>, SRes<MaterialPipeline<Self>>);
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let base_color = Vec4::from_slice(&extracted_asset.base_color.as_linear_rgba_f32());
        let flame_height = extracted_asset.flame_height;
        let distorsion_level = extracted_asset.distorsion_level;
        let bottom_threshold = extracted_asset.bottom_threshold;
        let time = extracted_asset.time;

        let mut buf = BytesMut::new();
        buf.put(base_color.as_std140().as_bytes());
        buf.put_f32(flame_height);
        buf.put_f32(distorsion_level);
        buf.put_f32(bottom_threshold);
        buf.put_f32(time);

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: &buf[..],
            label: None,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
            layout: &material_pipeline.material_layout,
        });

        Ok(GpuFireMaterial {
            _buffer: buffer,
            bind_group,
        })
    }
}

impl Material for FireMaterial {
    // When creating a custom material, you need to define either a vertex shader, a fragment shader or both.
    // If you don't define one of them it will use the default mesh shader which can be found at
    // <https://github.com/bevyengine/bevy/blob/latest/crates/bevy_pbr/src/render/mesh.wgsl>

    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/fire.vert"))
    }

    // fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
    //     Some(asset_server.load("shaders/fire.frag"))
    // }

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
                    min_binding_size: BufferSize::new(
                        Vec4::std140_size_static() as u64 + (4 * 4) as u64,
                    ),
                },
                count: None,
            }],
            label: None,
        })
    }
}
