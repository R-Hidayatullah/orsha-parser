#![allow(unused_variables)]
use crate::xac::xac_enum::XacChunkType::{
    XacMaterialDefinitionId, XacMaterialTotalId, XacMeshId, XacMetadataId, XacMorphTargetId,
    XacNodeHierarchyId, XacShaderMaterialId, XacSkinningId,
};
use crate::xac::xac_enum::XacVerticesAttributeType::{
    XacColor128Id, XacColor32Id, XacInfluenceRangeId, XacNormalId, XacPositionId, XacTangentId,
    XacUVCoordId,
};
use crate::xac::xac_struct::{
    XacActorFile, XacActorMaterial, XacActorMaterialLayer, XacActorMesh, XacActorNode,
    XacActorSubMesh, XacBoolProperties, XacChunkHeader, XacColor8, XacFloatProperties, XacHeader,
    XacInfluenceRange, XacIntProperties, XacMaterialDefinitionChunkv2Header,
    XacMaterialDefinitionChunkv2Layer, XacMaterialTotalsChunkv1, XacMatrix44, XacMeshChunkv1Header,
    XacMeshChunkv1SubMesh, XacMeshChunkv1VertexElement, XacMetaDataChunkv2Header,
    XacNodeHierarchyChunkv1NodeHeader, XacQuaternion, XacShaderMaterial, XacStringProperties,
    XacSubMeshInfluenceRange, XacVec2d, XacVec3d, XacVec4d,
};
use crate::xac::xac_util::{
    xac_read_boolean, xac_read_color8, xac_read_matrix44, xac_read_quaternion, xac_read_string,
    xac_read_vec2d, xac_read_vec3d, xac_read_vec4d,
};
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub fn xacparse(path: &str) -> XacActorFile {
    let mut xac_file = File::open(path).expect("Cannot open XAC file!");
    let mut xac_actor = XacActorFile {
        root_nodes: vec![],
        materials: vec![],
        morph_targets: vec![],
        header: XacHeader {
            magic: "".to_string(),
            major_version: 0,
            minor_version: 0,
            big_endian: false,
            multiply_order: 0,
        },
        metadata: XacMetaDataChunkv2Header {
            reposition_mask: 0,
            repositioning_node: 0,
            exporter_major_version: 0,
            exporter_minor_version: 0,
            retarget_root_offset: 0.0,
        },
        actor_name: "".to_string(),
        original_filename: "".to_string(),
        source_app: "".to_string(),
        export_date: "".to_string(),
        shader_materials: vec![],
        material_total: XacMaterialTotalsChunkv1 {
            num_total_materials: 0,
            num_standard_materials: 0,
            num_fx_materials: 0,
        },
        num_root_nodes: 0,
        num_nodes: 0,
        nodes: vec![],
        mesh_num_influence_range: Default::default(),
        sub_mesh_influence_range_indices: Default::default(),
        num_nodes_index: 0,
    };
    read_header(&mut xac_file, &mut xac_actor);
    read_chunk(&mut xac_file, &mut xac_actor);

    xac_actor
}

fn read_header<'a>(file: &'a mut File, xac: &'a mut XacActorFile) -> &'a mut XacActorFile {
    let mut magic = [0; 4];
    file.read_exact(&mut magic).unwrap();
    xac.header.magic = std::str::from_utf8(&magic).unwrap().to_string();
    if xac.header.magic != "XAC " {
        panic!("Not an XAC file: invalid header magic");
    }
    xac.header.major_version = file.read_u8().unwrap();
    xac.header.minor_version = file.read_u8().unwrap();
    if xac.header.major_version != 1 || xac.header.minor_version != 0 {
        panic!(
            "Unsupported .xac version: expected v1.0, file is {}.{}",
            xac.header.major_version, xac.header.minor_version
        );
    }
    xac.header.big_endian = xac_read_boolean(file);
    if xac.header.big_endian {
        panic!("XAC file is encoded in big endian which is not supported by this importer");
    }
    xac.header.multiply_order = file.read_u8().unwrap();
    xac
}

fn read_chunk<'a>(file: &'a mut File, xac: &'a mut XacActorFile) -> &'a mut XacActorFile {
    while file.stream_position().unwrap() < file.metadata().unwrap().len() {
        let chunk = XacChunkHeader {
            type_id: file.read_i32::<LittleEndian>().unwrap(),
            length: file.read_i32::<LittleEndian>().unwrap(),
            version: file.read_i32::<LittleEndian>().unwrap(),
        };
        let position = file.stream_position().unwrap();

        if chunk.type_id == XacMeshId as i32 {
            read_mesh(file, xac);
        }

        if chunk.type_id == XacSkinningId as i32 {
            //read_skinning(file, xac);
        }
        if chunk.type_id == XacMaterialDefinitionId as i32 {
            read_material_definition(file, xac);
        }
        if chunk.type_id == XacShaderMaterialId as i32 {
            read_shader_material(file, xac);
        }

        if chunk.type_id == XacMetadataId as i32 {
            read_metadata(file, xac);
        }
        if chunk.type_id == XacNodeHierarchyId as i32 {
            read_node_hierarchy(file, xac);
        }
        if chunk.type_id == XacMorphTargetId as i32 {
            //   read_morph_target(file, xac);
        }
        if chunk.type_id == XacMaterialTotalId as i32 {
            read_material_total(file, xac);
        }

        file.seek(SeekFrom::Start(position + chunk.length as u64))
            .unwrap();
    }
    xac
}

fn read_metadata<'a>(file: &'a mut File, xac: &'a mut XacActorFile) -> &'a mut XacActorFile {
    xac.metadata.reposition_mask = file.read_u32::<LittleEndian>().unwrap();
    xac.metadata.repositioning_node = file.read_i32::<LittleEndian>().unwrap();
    xac.metadata.exporter_major_version = file.read_u8().unwrap();
    xac.metadata.exporter_minor_version = file.read_u8().unwrap();
    file.read_u8().unwrap(); //Padding
    file.read_u8().unwrap(); //Padding
    xac.metadata.retarget_root_offset = file.read_f32::<LittleEndian>().unwrap();
    xac.source_app = xac_read_string(file);
    xac.original_filename = xac_read_string(file);
    xac.export_date = xac_read_string(file);
    xac.actor_name = xac_read_string(file);
    xac
}
fn read_node_hierarchy<'a>(file: &'a mut File, xac: &'a mut XacActorFile) -> &'a mut XacActorFile {
    let num_nodes = file.read_i32::<LittleEndian>().unwrap();
    let num_root_nodes = file.read_i32::<LittleEndian>().unwrap();
    if num_nodes <= 0 {
        panic!("Invalid number of nodes");
    }

    xac.num_nodes = num_nodes;
    xac.num_root_nodes = num_root_nodes;

    for _ in 0..num_nodes {
        let mut node_info = XacNodeHierarchyChunkv1NodeHeader {
            rotation: XacQuaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            scale_rotation: XacQuaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            position: XacVec3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: XacVec3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            parent_node_id: 0,
            num_children: 0,
            include_inbounds_calc: 0,
            transform: XacMatrix44 {
                axis_1: XacVec4d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0,
                },
                axis_2: XacVec4d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0,
                },
                axis_3: XacVec4d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0,
                },
                pos: XacVec4d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0,
                },
            },
            importance_factor: 0.0,
        };

        let mut p_nodes = XacActorNode {
            parent_node_id: 0,
            node_id: 0,
            name: "".to_string(),
            position: XacVec3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },

            rotation: XacQuaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            scale: XacVec3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },

            scale_rotation: XacQuaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            transform: XacMatrix44 {
                axis_1: XacVec4d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0,
                },
                axis_2: XacVec4d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0,
                },
                axis_3: XacVec4d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0,
                },
                pos: XacVec4d {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                    w: 0.0,
                },
            },
            parent: None,
            num_children: 0,
            children: vec![],
            visual_mesh: XacActorMesh { sub_meshes: vec![] },
            collision_mesh: XacActorMesh { sub_meshes: vec![] },
        };

        node_info.rotation = xac_read_quaternion(file);
        node_info.scale_rotation = xac_read_quaternion(file);
        node_info.position = xac_read_vec3d(file);
        node_info.scale = xac_read_vec3d(file);

        file.read_i32::<LittleEndian>().unwrap(); //Padding
        file.read_i32::<LittleEndian>().unwrap(); //Padding
        file.read_i32::<LittleEndian>().unwrap(); //Padding
        file.read_i32::<LittleEndian>().unwrap(); //Padding
        file.read_i32::<LittleEndian>().unwrap(); //Padding

        node_info.parent_node_id = file.read_i32::<LittleEndian>().unwrap();
        node_info.num_children = file.read_i32::<LittleEndian>().unwrap();
        node_info.include_inbounds_calc = file.read_i32::<LittleEndian>().unwrap();
        node_info.transform = xac_read_matrix44(file);
        node_info.importance_factor = file.read_f32::<LittleEndian>().unwrap();
        let node_name = xac_read_string(file);

        p_nodes.parent_node_id = node_info.parent_node_id;
        p_nodes.name = node_name;
        p_nodes.position = node_info.position;
        p_nodes.rotation = node_info.rotation;
        p_nodes.scale = node_info.scale;
        p_nodes.scale_rotation = node_info.scale_rotation;
        p_nodes.transform = node_info.transform;
        p_nodes.num_children = node_info.num_children;
        p_nodes.node_id = xac.num_nodes_index.clone();
        xac.num_nodes_index = p_nodes.node_id + 1;
        let num_root = xac.root_nodes.len().clone() as usize;
        if node_info.parent_node_id.clone() == -1 {
            p_nodes.parent = None;
            xac.root_nodes.push(p_nodes.clone());
        } else {
            xac.root_nodes[num_root - 1].children.push(p_nodes.clone());
        }

        xac.nodes.push(p_nodes.clone());
    }

    if xac.root_nodes.len() != num_root_nodes.clone() as usize {
        panic!("Root nodes size does not match number of nodes with parent ID -1");
    }
    xac
}

fn read_material_total<'a>(file: &'a mut File, xac: &'a mut XacActorFile) -> &'a mut XacActorFile {
    let mut totals = XacMaterialTotalsChunkv1 {
        num_total_materials: 0,
        num_standard_materials: 0,
        num_fx_materials: 0,
    };
    totals.num_total_materials = file.read_i32::<LittleEndian>().unwrap();
    totals.num_standard_materials = file.read_i32::<LittleEndian>().unwrap();
    totals.num_fx_materials = file.read_i32::<LittleEndian>().unwrap();
    if totals.num_standard_materials.clone() <= 0 {
        panic!("Invalid number of standard materials");
    }

    if totals.num_total_materials.clone()
        != totals.num_standard_materials.clone() + totals.num_fx_materials.clone()
    {
        panic!("Incorrect numTotalMaterials (must be sum of standard materials and fx materials");
    }
    xac.material_total = totals;
    xac
}

fn read_material_definition<'a>(
    file: &'a mut File,
    xac: &'a mut XacActorFile,
) -> &'a mut XacActorFile {
    let mut material_info = XacMaterialDefinitionChunkv2Header {
        ambient_color: XacVec4d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
        diffuse_color: XacVec4d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
        specular_color: XacVec4d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
        emissive_color: XacVec4d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
        shine: 0.0,
        shine_strength: 0.0,
        opacity: 0.0,
        ior: 0.0,
        double_sided: false,
        wireframe: false,
        num_layers: 0,
    };
    material_info.ambient_color = xac_read_vec4d(file);
    material_info.diffuse_color = xac_read_vec4d(file);
    material_info.specular_color = xac_read_vec4d(file);
    material_info.emissive_color = xac_read_vec4d(file);
    material_info.shine = file.read_f32::<LittleEndian>().unwrap();
    material_info.shine_strength = file.read_f32::<LittleEndian>().unwrap();
    material_info.opacity = file.read_f32::<LittleEndian>().unwrap();
    material_info.ior = file.read_f32::<LittleEndian>().unwrap();
    material_info.double_sided = xac_read_boolean(file);
    material_info.wireframe = xac_read_boolean(file);
    file.read_u8().unwrap(); //Padding
    material_info.num_layers = file.read_u8().unwrap();
    let material_name = xac_read_string(file);

    let mut material = XacActorMaterial {
        name: "".to_string(),

        ambient_color: XacVec4d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
        diffuse_color: XacVec4d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
        specular_color: XacVec4d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
        emissive_color: XacVec4d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
        shine: 0.0,
        shine_strength: 0.0,
        opacity: 0.0,
        ior: 0.0,
        double_sided: false,
        wireframe: false,
        layers: vec![],
    };

    material.name = material_name;
    material.ambient_color = material_info.ambient_color;
    material.diffuse_color = material_info.diffuse_color;
    material.specular_color = material_info.specular_color;
    material.emissive_color = material_info.emissive_color;
    material.shine = material_info.shine;
    material.shine_strength = material_info.shine_strength;
    material.opacity = material_info.opacity;
    material.ior = material_info.ior;
    material.double_sided = material_info.double_sided;
    material.wireframe = material_info.wireframe;

    for _i in 0..material_info.num_layers {
        let mut layer_info = XacMaterialDefinitionChunkv2Layer {
            amount: 0.0,
            v_offset: 0.0,
            u_offset: 0.0,
            u_tiling: 0.0,
            v_tiling: 0.0,
            rotation: 0.0,
            material_id: 0,
            map_type: 0,
        };

        layer_info.amount = file.read_f32::<LittleEndian>().unwrap();
        layer_info.u_offset = file.read_f32::<LittleEndian>().unwrap();
        layer_info.v_offset = file.read_f32::<LittleEndian>().unwrap();
        layer_info.u_tiling = file.read_f32::<LittleEndian>().unwrap();
        layer_info.v_tiling = file.read_f32::<LittleEndian>().unwrap();
        layer_info.rotation = file.read_f32::<LittleEndian>().unwrap();
        layer_info.material_id = file.read_i16::<LittleEndian>().unwrap();
        layer_info.map_type = file.read_u8().unwrap();
        file.read_u8().unwrap(); //Padding

        let texture = xac_read_string(file);

        let mut layer = XacActorMaterialLayer {
            name: "".to_string(),
            amount: 0.0,
            v_offset: 0.0,
            u_offset: 0.0,
            v_tiling: 0.0,
            u_tiling: 0.0,
            rotation: 0.0,
            map_type: 0,
        };

        layer.name = texture;
        layer.amount = layer_info.amount;
        layer.u_offset = layer_info.u_offset;
        layer.v_offset = layer_info.v_offset;
        layer.u_tiling = layer_info.u_tiling;
        layer.v_tiling = layer_info.v_tiling;
        layer.rotation = layer_info.rotation;
        layer.map_type = layer_info.map_type;

        material.layers.push(layer);
    }
    xac.materials.push(material);
    xac
}
fn calculate_faces(indices: &Vec<i32>) -> Vec<[i32; 4]> {
    let mut faces: Vec<[i32; 4]> = Vec::new();

    for i in (0..indices.len()).step_by(4) {
        let face_indices = &indices[i..(i + 4)];
        let face: [i32; 4] = [
            face_indices[0].clone() - 1,
            face_indices[1].clone() - 1,
            face_indices[2].clone() - 1,
            face_indices[3].clone() - 1,
        ];
        faces.push(face);
    }

    faces
}
fn read_mesh<'a>(file: &'a mut File, xac: &'a mut XacActorFile) -> &'a mut XacActorFile {
    let mut mesh_info = XacMeshChunkv1Header {
        node_id: 0,
        num_influence_ranges: 0,
        num_vertices: 0,
        num_indices: 0,
        num_sub_meshes: 0,
        num_vertex_element: 0,
        is_collision_mesh: false,
    };
    mesh_info.node_id = file.read_i32::<LittleEndian>().unwrap();
    mesh_info.num_influence_ranges = file.read_i32::<LittleEndian>().unwrap();
    mesh_info.num_vertices = file.read_i32::<LittleEndian>().unwrap();
    mesh_info.num_indices = file.read_i32::<LittleEndian>().unwrap();
    mesh_info.num_sub_meshes = file.read_i32::<LittleEndian>().unwrap();
    mesh_info.num_vertex_element = file.read_i32::<LittleEndian>().unwrap();
    mesh_info.is_collision_mesh = xac_read_boolean(file);
    file.read_u8().unwrap(); //Padding
    file.read_u8().unwrap(); //Padding
    file.read_u8().unwrap(); //Padding

    for p_node in xac.root_nodes.iter_mut() {
        if p_node.node_id == mesh_info.node_id.clone() as usize {
            let mut p_mesh = XacActorMesh { sub_meshes: vec![] };

            let mut positions: Vec<XacVec3d> = Vec::new();
            let mut normals: Vec<XacVec3d> = Vec::new();
            let mut tangents: Vec<XacVec4d> = Vec::new();
            let mut bi_tangents: Vec<XacVec4d> = Vec::new();
            let mut uv_set: Vec<Vec<XacVec2d>> = Vec::new();
            let mut influence_range_indices: Vec<i32> = Vec::new();
            let mut colors_128: Vec<Vec<XacVec3d>> = Vec::new();
            let mut colors_32: Vec<Vec<XacColor8>> = Vec::new();

            for _ in 0..mesh_info.num_vertex_element {
                let mut vertices_attribute = XacMeshChunkv1VertexElement {
                    usage: 0,
                    element_size: 0,
                    keep_originals: false,
                    is_scale_factor: false,
                };

                vertices_attribute.usage = file.read_u32::<LittleEndian>().unwrap();
                vertices_attribute.element_size = file.read_i32::<LittleEndian>().unwrap();
                vertices_attribute.keep_originals = xac_read_boolean(file);
                vertices_attribute.is_scale_factor = xac_read_boolean(file);
                file.read_u8().unwrap(); //Padding
                file.read_u8().unwrap(); //Padding

                if vertices_attribute.usage == XacPositionId as u32 {
                    for _ in 0..mesh_info.num_vertices {
                        positions.push(xac_read_vec3d(file))
                    }
                }
                if vertices_attribute.usage == XacNormalId as u32 {
                    for _ in 0..mesh_info.num_vertices {
                        normals.push(xac_read_vec3d(file))
                    }
                }
                if vertices_attribute.usage == XacTangentId as u32 {
                    if tangents.is_empty() {
                        for _ in 0..mesh_info.num_vertices {
                            tangents.push(xac_read_vec4d(file));
                        }
                    } else if bi_tangents.is_empty() {
                        for _ in 0..mesh_info.num_vertices {
                            bi_tangents.push(xac_read_vec4d(file));
                        }
                    }
                }
                if vertices_attribute.usage == XacUVCoordId as u32 {
                    let mut temp_uv_set: Vec<XacVec2d> = Vec::new();
                    for _ in 0..mesh_info.num_vertices {
                        temp_uv_set.push(xac_read_vec2d(file));
                    }
                    uv_set.push(temp_uv_set);
                }
                if vertices_attribute.usage == XacColor32Id as u32 {
                    let mut temp_colors_32: Vec<XacColor8> = Vec::new();
                    for _ in 0..mesh_info.num_vertices {
                        temp_colors_32.push(xac_read_color8(file));
                    }
                    colors_32.push(temp_colors_32);
                }
                if vertices_attribute.usage == XacInfluenceRangeId as u32 {
                    for _ in 0..mesh_info.num_vertices {
                        influence_range_indices.push(file.read_i32::<LittleEndian>().unwrap());
                    }
                }
                if vertices_attribute.usage == XacColor128Id as u32 {
                    let mut temp_colors_128: Vec<XacVec3d> = Vec::new();
                    for _ in 0..mesh_info.num_vertices {
                        temp_colors_128.push(xac_read_vec3d(file));
                    }
                    colors_128.push(temp_colors_128);
                }
            }

            if !mesh_info.is_collision_mesh.clone() {
                for position in positions.clone() {
                    println!("v {} {} {}", position.x, position.y, position.z);
                }

                for uv_set in uv_set.clone() {
                    for uv in uv_set.clone() {
                        println!("vt {} {}", uv.x, uv.y);
                    }
                }

                for normal in normals.clone() {
                    println!("v {} {} {}", normal.x, normal.y, normal.z);
                }
                let faces = calculate_faces(&influence_range_indices.clone());
                for face in faces {
                    println!("f {} {} {} {} ", face[0], face[1], face[2], face[3],);
                }
            }

            let mut vertex_offset: usize = 0;
            let mut index_offset: usize = 0;
            for _ in 0..mesh_info.num_sub_meshes {
                let mut submeshes = XacMeshChunkv1SubMesh {
                    num_indices: 0,
                    num_vertices: 0,
                    material_id: 0,
                    num_bones: 0,
                };

                let mut submesh = XacActorSubMesh {
                    name: "".to_string(),
                    material_id: 0,
                    vertex_positions: vec![],
                    vertex_normals: vec![],
                    vertex_tangents: vec![],
                    vertex_bi_tangents: vec![],
                    vertex_uvs: vec![],
                    vertex_colors_32: vec![],
                    vertex_colors_128: vec![],
                    vertex_influences: vec![],
                    indices: vec![],
                    influence_range_indices: vec![],
                };
                submeshes.num_indices = file.read_i32::<LittleEndian>().unwrap();
                submeshes.num_vertices = file.read_i32::<LittleEndian>().unwrap();
                submeshes.material_id = file.read_i32::<LittleEndian>().unwrap();
                submeshes.num_bones = file.read_i32::<LittleEndian>().unwrap();
                submesh.name = p_node.name.clone();

                if !positions.is_empty() {
                    let temp = &positions;
                    submesh.vertex_positions = temp[vertex_offset
                        ..(submeshes.num_vertices.clone() as usize
                            + vertex_offset.clone() as usize)]
                        .to_vec();
                }
                if !normals.is_empty() {
                    let temp = &normals;
                    submesh.vertex_normals = temp[vertex_offset
                        ..(submeshes.num_vertices.clone() as usize
                            + vertex_offset.clone() as usize)]
                        .to_vec();
                }
                if !tangents.is_empty() {
                    let temp = &tangents;
                    submesh.vertex_tangents = temp[vertex_offset
                        ..(submeshes.num_vertices.clone() as usize
                            + vertex_offset.clone() as usize)]
                        .to_vec();
                }
                if !bi_tangents.is_empty() {
                    let temp_bi_tangents = &bi_tangents;
                    submesh.vertex_bi_tangents = temp_bi_tangents[vertex_offset
                        ..(submeshes.num_vertices.clone() as usize
                            + vertex_offset.clone() as usize)]
                        .to_vec();
                }

                if !influence_range_indices.is_empty() {
                    xac.sub_mesh_influence_range_indices
                        .push(XacSubMeshInfluenceRange {
                            node_id: mesh_info.node_id.clone() as usize,
                            indices: influence_range_indices[vertex_offset
                                ..(submeshes.num_vertices.clone() as usize
                                    + vertex_offset.clone() as usize)]
                                .to_vec(),
                        });
                    submesh.influence_range_indices = influence_range_indices[vertex_offset
                        ..(submeshes.num_vertices.clone() as usize
                            + vertex_offset.clone() as usize)]
                        .to_vec();
                }

                for i in 0..colors_32.len() {
                    let temp = &colors_32;
                    submesh.vertex_colors_32 = temp[i][vertex_offset
                        ..(submeshes.num_vertices.clone() as usize
                            + vertex_offset.clone() as usize)]
                        .to_vec();
                }
                for i in 0..colors_128.len() {
                    let temp = &colors_128;
                    submesh.vertex_colors_128 = temp[i][vertex_offset
                        ..(submeshes.num_vertices.clone() as usize
                            + vertex_offset.clone() as usize)]
                        .to_vec();
                }
                for i in 0..uv_set.len() {
                    let temp = &uv_set;
                    submesh.vertex_uvs = temp[i][vertex_offset
                        ..(submeshes.num_vertices.clone() as usize
                            + vertex_offset.clone() as usize)]
                        .to_vec();
                }

                for _ in 0..submeshes.num_indices {
                    submesh
                        .influence_range_indices
                        .push(file.read_i32::<LittleEndian>().unwrap());
                }

                for _ in 0..(4 * &submeshes.num_bones) {
                    file.read_u8().unwrap();
                }
                vertex_offset = vertex_offset + submeshes.num_vertices.clone() as usize;
                index_offset = index_offset + submeshes.num_indices.clone() as usize;
                p_mesh.sub_meshes.push(submesh.clone());

                if !mesh_info.is_collision_mesh.clone() {
                    if !p_node.visual_mesh.sub_meshes.is_empty().clone() {
                        panic!("Node already has a visual mesh");
                    }
                    p_node.visual_mesh = p_mesh.clone();
                } else {
                    if !p_node.collision_mesh.sub_meshes.is_empty().clone() {
                        panic!("Node already has a collision mesh");
                    }

                    p_node.collision_mesh = p_mesh.clone();
                }
            }

            xac.mesh_num_influence_range.push(XacInfluenceRange {
                node_id: p_node.node_id.clone(),
                num: mesh_info.num_influence_ranges.clone(),
            });
        }
    }

    xac
}

/*
fn read_skinning<'a>(file: &'a mut File, xac: &'a mut XacActorFile) -> &'a mut XacActorFile {
    let mut influences_header = XacSkinningChunkv3Header {
        node_id: 0,
        num_local_bones: 0,
        num_influences: 0,
        is_for_collision_mesh: false,
    };
    influences_header.node_id = file.read_i32::<LittleEndian>().unwrap();
    influences_header.num_local_bones = file.read_i32::<LittleEndian>().unwrap();
    influences_header.num_influences = file.read_i32::<LittleEndian>().unwrap();
    influences_header.is_for_collision_mesh = xac_read_boolean(file);
    file.read_u8().unwrap(); //Padding
    file.read_u8().unwrap(); //Padding
    file.read_u8().unwrap(); //Padding

    let mut influences = XacSkinningChunkv3Influence {
        weight: 0.0,
        node_id: 0,
    };
    influences.node_id = file.read_i16::<LittleEndian>().unwrap();
    influences.weight = file.read_f32::<LittleEndian>().unwrap();

    let mut influence_ranges = XacSkinningChunkv3InfluenceRange {
        first_influence_index: 0,
        num_influences: 0,
    };

    influence_ranges.first_influence_index = file.read_i32::<LittleEndian>().unwrap();
    influence_ranges.num_influences = file.read_i32::<LittleEndian>().unwrap();

    for mut root_node in xac.root_nodes.iter_mut() {
        if root_node.node_id.clone() == influences_header.node_id.clone() as usize {
            if influences_header.is_for_collision_mesh.clone() {
                for mut collision_mesh in root_node.collision_mesh.sub_meshes.iter_mut() {
                    if !collision_mesh.vertex_influences.is_empty() {
                        panic!("Duplicate vertex influences for submesh");
                    }

                    if collision_mesh.influence_range_indices.is_empty() {}
                    for sub_mesh_influence_range_index in
                        xac.sub_mesh_influence_range_indices.clone()
                    {}

                    for i in 0..collision_mesh.influence_range_indices.len() {
                        for j in influence_ranges.first_influence_index
                            ..(influence_ranges.first_influence_index.clone()
                                + influence_ranges.num_influences.clone())
                        {
                            let mut influence_weight = XacBoneInfluence {
                                node_id: 0,
                                weight: 0.0,
                            };
                            influence_weight.node_id = influences.node_id.clone() as usize;
                            influence_weight.weight = influences.weight.clone();
                            collision_mesh.vertex_influences.push(influence_weight);
                        }
                    }
                }
            } else {
                for mut visual_mesh in root_node.visual_mesh.sub_meshes.iter_mut() {
                    if !visual_mesh.vertex_influences.is_empty() {
                        panic!("Duplicate vertex influences for submesh");
                    }

                    if visual_mesh.influence_range_indices.is_empty() {}
                    for sub_mesh_influence_range_index in
                        xac.sub_mesh_influence_range_indices.clone()
                    {}

                    /*
                                       for i in 0..visual_mesh.influence_range_indices.len() {
                                           for j in influence_ranges.first_influence_index
                                               ..(influence_ranges.first_influence_index.clone()
                                                   + influence_ranges.num_influences.clone())
                                           {
                                               let mut influence_weight = XacBoneInfluence {
                                                   node_id: 0,
                                                   weight: 0.0,
                                               };
                                               influence_weight.node_id = influences.node_id.clone() as usize;
                                               influence_weight.weight = influences.weight.clone();
                                               visual_mesh.vertex_influences.push(influence_weight);
                                           }
                                       }

                    */
                }
            };
        }
    }

    xac
}


fn read_morph_target<'a>(file: &'a mut File, xac: &'a mut XacActorFile) -> &'a mut XacActorFile {
    let num_morph_targets = file.read_i32::<LittleEndian>().unwrap();
    let morph_target_lod_idx = file.read_i32::<LittleEndian>().unwrap();
    let mut morph_target = XacMorphTargetsChunkv1MorphTarget {
        range_min: 0.0,
        range_max: 0.0,
        lod_level: 0,
        num_deformations: 0,
        num_transformations: 0,
        phoneme_set_bitmask: 0,
    };
    morph_target.range_min = file.read_f32::<LittleEndian>().unwrap();
    morph_target.range_max = file.read_f32::<LittleEndian>().unwrap();
    morph_target.lod_level = file.read_i32::<LittleEndian>().unwrap();
    morph_target.num_deformations = file.read_i32::<LittleEndian>().unwrap();
    morph_target.num_transformations = file.read_i32::<LittleEndian>().unwrap();
    morph_target.phoneme_set_bitmask = file.read_i32::<LittleEndian>().unwrap();
    xac
}

 */

fn read_shader_material<'a>(file: &'a mut File, xac: &'a mut XacActorFile) -> &'a mut XacActorFile {
    let mut shader_material = XacShaderMaterial {
        num_int: 0,
        num_float: 0,
        num_bool: 0,
        num_string: 0,
        flag: 0,
        name_material: "".to_string(),
        name_shader: "".to_string(),
        int_property: vec![],
        float_property: vec![],
        bool_property: vec![],
        string_property: vec![],
    };

    shader_material.num_int = file.read_i32::<LittleEndian>().unwrap();
    shader_material.num_float = file.read_i32::<LittleEndian>().unwrap();
    file.read_i32::<LittleEndian>().unwrap(); //Padding
    shader_material.num_bool = file.read_i32::<LittleEndian>().unwrap();
    shader_material.flag = file.read_i32::<LittleEndian>().unwrap();
    shader_material.num_string = file.read_i32::<LittleEndian>().unwrap();
    shader_material.name_material = xac_read_string(file);
    shader_material.name_shader = xac_read_string(file);

    for i in 0..shader_material.num_int {
        let mut property = XacIntProperties {
            name_properties: "".to_string(),
            value: 0,
        };
        property.name_properties = xac_read_string(file);
        property.value = file.read_i32::<LittleEndian>().unwrap();
        shader_material.int_property.push(property);
    }

    for i in 0..shader_material.num_float {
        let mut property = XacFloatProperties {
            name_properties: "".to_string(),
            value: 0.0,
        };
        property.name_properties = xac_read_string(file);
        property.value = file.read_f32::<LittleEndian>().unwrap();
        shader_material.float_property.push(property);
    }

    for i in 0..shader_material.num_bool {
        let mut property = XacBoolProperties {
            name_properties: "".to_string(),
            value: 0,
        };

        property.name_properties = xac_read_string(file);
        property.value = file.read_u8().unwrap();
        shader_material.bool_property.push(property);
    }
    let skip = file.read_i32::<LittleEndian>().unwrap();
    for _ in 0..skip {
        file.read_u8().unwrap();
    }
    for i in 0..shader_material.num_string {
        let mut property = XacStringProperties {
            name_properties: "".to_string(),
            value: "".to_string(),
        };
        property.name_properties = xac_read_string(file);
        property.value = xac_read_string(file);
        shader_material.string_property.push(property);
    }

    xac.shader_materials.push(shader_material);
    xac
}
