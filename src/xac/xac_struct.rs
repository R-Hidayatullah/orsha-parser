#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacVec2d {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacVec3d {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacVec4d {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) w: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacColor {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacColor8 {
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) z: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacQuaternion {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) w: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacMatrix44 {
    pub(crate) axis_1: XacVec4d,
    pub(crate) axis_2: XacVec4d,
    pub(crate) axis_3: XacVec4d,
    pub(crate) pos: XacVec4d,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XacActorFile {
    pub(crate) root_nodes: Vec<XacActorNode>,
    pub(crate) materials: Vec<XacActorMaterial>,
    pub(crate) morph_targets: Vec<XacActorMorphTarget>,
    pub(crate) header: XacHeader,
    pub(crate) metadata: XacMetaDataChunkv2Header,
    pub(crate) actor_name: String,
    pub(crate) original_filename: String,
    pub(crate) source_app: String,
    pub(crate) export_date: String,
    pub(crate) shader_materials: Vec<XacShaderMaterial>,
    pub(crate) material_total: XacMaterialTotalsChunkv1,
    pub(crate) num_root_nodes: i32,
    pub(crate) num_nodes: i32,
    pub(crate) nodes: Vec<XacActorNode>,
    pub(crate) mesh_num_influence_range: Vec<XacInfluenceRange>,
    pub(crate) sub_mesh_influence_range_indices: Vec<XacSubMeshInfluenceRange>,
    pub(crate) num_nodes_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacInfluenceRange {
    pub(crate) node_id: usize,
    pub(crate) num: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacSubMeshInfluenceRange {
    pub(crate) node_id: usize,
    pub(crate) indices: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacChunkHeader {
    pub(crate) type_id: i32,
    pub(crate) length: i32,
    pub(crate) version: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacHeader {
    pub(crate) magic: String,
    pub(crate) major_version: u8,
    pub(crate) minor_version: u8,
    pub(crate) big_endian: bool,
    pub(crate) multiply_order: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacMetaDataChunkv2Header {
    pub(crate) reposition_mask: u32,
    pub(crate) repositioning_node: i32,
    pub(crate) exporter_major_version: u8,
    pub(crate) exporter_minor_version: u8,
    pub(crate) retarget_root_offset: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacNodeHierarchyChunkv1NodeHeader {
    pub(crate) rotation: XacQuaternion,
    pub(crate) scale_rotation: XacQuaternion,
    pub(crate) position: XacVec3d,
    pub(crate) scale: XacVec3d,
    pub(crate) parent_node_id: i32,
    pub(crate) num_children: i32,
    pub(crate) include_inbounds_calc: i32,
    pub(crate) transform: XacMatrix44,
    pub(crate) importance_factor: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacMaterialTotalsChunkv1 {
    pub(crate) num_total_materials: i32,
    pub(crate) num_standard_materials: i32,
    pub(crate) num_fx_materials: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacMaterialDefinitionChunkv2Header {
    pub(crate) ambient_color: XacVec4d,
    pub(crate) diffuse_color: XacVec4d,
    pub(crate) specular_color: XacVec4d,
    pub(crate) emissive_color: XacVec4d,
    pub(crate) shine: f32,
    pub(crate) shine_strength: f32,
    pub(crate) opacity: f32,
    pub(crate) ior: f32,
    pub(crate) double_sided: bool,
    pub(crate) wireframe: bool,
    pub(crate) num_layers: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacMaterialDefinitionChunkv2Layer {
    pub(crate) amount: f32,
    pub(crate) v_offset: f32,
    pub(crate) u_offset: f32,
    pub(crate) u_tiling: f32,
    pub(crate) v_tiling: f32,
    pub(crate) rotation: f32, //radians
    pub(crate) material_id: i16,
    pub(crate) map_type: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacMeshChunkv1Header {
    pub(crate) node_id: i32,
    pub(crate) num_influence_ranges: i32,
    pub(crate) num_vertices: i32,
    pub(crate) num_indices: i32,
    pub(crate) num_sub_meshes: i32,
    pub(crate) num_vertex_element: i32,
    pub(crate) is_collision_mesh: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacMeshChunkv1SubMesh {
    pub(crate) num_indices: i32,
    pub(crate) num_vertices: i32,
    pub(crate) material_id: i32,
    pub(crate) num_bones: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacMeshChunkv1VertexElement {
    pub(crate) usage: u32,
    pub(crate) element_size: i32,
    pub(crate) keep_originals: bool,
    pub(crate) is_scale_factor: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacActorNode {
    pub(crate) parent_node_id: i32,
    pub(crate) node_id: usize,
    pub(crate) name: String,
    pub(crate) position: XacVec3d,
    pub(crate) rotation: XacQuaternion,
    pub(crate) scale: XacVec3d,
    pub(crate) scale_rotation: XacQuaternion,
    pub(crate) transform: XacMatrix44,
    pub(crate) parent: Option<Box<XacActorNode>>,
    pub(crate) num_children: i32,
    pub(crate) children: Vec<XacActorNode>,
    pub(crate) visual_mesh: XacActorMesh,
    pub(crate) collision_mesh: XacActorMesh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacActorMesh {
    pub(crate) sub_meshes: Vec<XacActorSubMesh>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacActorSubMesh {
    pub(crate) name: String,
    pub(crate) material_id: i32,
    pub(crate) vertex_positions: Vec<XacVec3d>,
    pub(crate) vertex_normals: Vec<XacVec3d>,
    pub(crate) vertex_tangents: Vec<XacVec4d>,
    pub(crate) vertex_bi_tangents: Vec<XacVec4d>,
    pub(crate) vertex_uvs: Vec<XacVec2d>,
    pub(crate) vertex_colors_32: Vec<XacColor8>,
    pub(crate) vertex_colors_128: Vec<XacVec3d>,
    pub(crate) vertex_influences: Vec<XacBoneInfluence>,
    pub(crate) indices: Vec<XacVec3d>,
    pub(crate) influence_range_indices: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct XacBoneInfluence {
    pub(crate) node_id: usize,
    pub(crate) weight: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacSkinningChunkv3Header {
    pub(crate) node_id: i32,
    pub(crate) num_local_bones: i32,
    pub(crate) num_influences: i32,
    pub(crate) is_for_collision_mesh: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacSkinningChunkv3Influence {
    pub(crate) weight: f32,
    pub(crate) node_id: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacSkinningChunkv3InfluenceRange {
    pub(crate) first_influence_index: i32,
    pub(crate) num_influences: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacActorMorphTarget {
    pub(crate) name: String,
    pub(crate) range_min: f32,
    pub(crate) range_max: f32,
    pub(crate) phoneme_set_bitmask: i32,
    pub(crate) deformation: Vec<XacActorNodeDeformation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacActorNodeDeformation {
    pub(crate) node: XacActorNode,
    pub(crate) vertex_indices: Vec<i32>,
    pub(crate) position_offsets: Vec<XacVec3d>,
    pub(crate) normal_offsets: Vec<XacVec3d>,
    pub(crate) tangent_offsets: Vec<XacVec3d>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacMorphTargetsChunkv1Deformation {
    pub(crate) node_id: i32,
    pub(crate) min_value: f32,
    pub(crate) max_value: f32,
    pub(crate) num_vertices: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacMorphTargetsChunkv1Transformation {
    pub(crate) node_id: i32,
    pub(crate) rotation: XacQuaternion,
    pub(crate) scale_rotation: XacQuaternion,
    pub(crate) position: XacVec3d,
    pub(crate) scale: XacVec3d,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacMorphTargetsChunkv1MorphTarget {
    pub(crate) range_min: f32,
    pub(crate) range_max: f32,
    pub(crate) lod_level: i32,
    pub(crate) num_deformations: i32,
    pub(crate) num_transformations: i32,
    pub(crate) phoneme_set_bitmask: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacActorMaterial {
    pub(crate) name: String,
    pub(crate) ambient_color: XacVec4d,
    pub(crate) diffuse_color: XacVec4d,
    pub(crate) specular_color: XacVec4d,
    pub(crate) emissive_color: XacVec4d,
    pub(crate) shine: f32,
    pub(crate) shine_strength: f32,
    pub(crate) opacity: f32,
    pub(crate) ior: f32,
    pub(crate) double_sided: bool,
    pub(crate) wireframe: bool,
    pub(crate) layers: Vec<XacActorMaterialLayer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacActorMaterialLayer {
    pub(crate) name: String,
    pub(crate) amount: f32,
    pub(crate) v_offset: f32,
    pub(crate) u_offset: f32,
    pub(crate) v_tiling: f32,
    pub(crate) u_tiling: f32,
    pub(crate) rotation: f32,
    pub(crate) map_type: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacShaderMaterial {
    pub(crate) num_int: i32,
    pub(crate) num_float: i32,
    pub(crate) num_bool: i32,
    pub(crate) num_string: i32,
    pub(crate) flag: i32,
    pub(crate) name_material: String,
    pub(crate) name_shader: String,
    pub(crate) int_property: Vec<XacIntProperties>,
    pub(crate) float_property: Vec<XacFloatProperties>,
    pub(crate) bool_property: Vec<XacBoolProperties>,
    pub(crate) string_property: Vec<XacStringProperties>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacIntProperties {
    pub(crate) name_properties: String,
    pub(crate) value: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacFloatProperties {
    pub(crate) name_properties: String,
    pub(crate) value: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacBoolProperties {
    pub(crate) name_properties: String,
    pub(crate) value: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct XacStringProperties {
    pub(crate) name_properties: String,
    pub(crate) value: String,
}
