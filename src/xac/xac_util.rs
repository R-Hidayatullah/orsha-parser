#![allow(dead_code)]
use crate::xac::xac_struct::{
    XacColor, XacColor8, XacMatrix44, XacQuaternion, XacVec2d, XacVec3d, XacVec4d,
};
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;

pub(crate) fn xac_read_string(file: &mut File) -> String {
    let mut text = String::new();
    let length = file.read_i32::<LittleEndian>().unwrap();
    for _ in 0..length {
        let character = file.read_u8().unwrap();
        text.push(character as char);
    }
    text
}

pub(crate) fn xac_read_boolean(file: &mut File) -> bool {
    let number = file.read_u8().unwrap();
    let boolean = if number != 0 { true } else { false };
    boolean
}
pub(crate) fn xac_read_color8(file: &mut File) -> XacColor8 {
    let color8 = XacColor8 {
        x: file.read_u8().unwrap(),
        y: file.read_u8().unwrap(),
        z: file.read_u8().unwrap(),
    };
    color8
}

pub(crate) fn xac_read_color(file: &mut File) -> XacColor {
    let color = XacColor {
        x: file.read_f32::<LittleEndian>().unwrap(),
        y: file.read_f32::<LittleEndian>().unwrap(),
        z: file.read_f32::<LittleEndian>().unwrap(),
    };
    color
}

pub(crate) fn xac_read_vec2d(file: &mut File) -> XacVec2d {
    let vec2d = XacVec2d {
        x: file.read_f32::<LittleEndian>().unwrap(),
        y: file.read_f32::<LittleEndian>().unwrap(),
    };
    vec2d
}
pub(crate) fn xac_read_vec3d(file: &mut File) -> XacVec3d {
    let vec3d = XacVec3d {
        x: file.read_f32::<LittleEndian>().unwrap(),
        y: file.read_f32::<LittleEndian>().unwrap(),
        z: file.read_f32::<LittleEndian>().unwrap(),
    };
    vec3d
}
pub(crate) fn xac_read_vec4d(file: &mut File) -> XacVec4d {
    let vec4d = XacVec4d {
        x: file.read_f32::<LittleEndian>().unwrap(),
        y: file.read_f32::<LittleEndian>().unwrap(),
        z: file.read_f32::<LittleEndian>().unwrap(),
        w: file.read_f32::<LittleEndian>().unwrap(),
    };
    vec4d
}

pub(crate) fn xac_read_quaternion(file: &mut File) -> XacQuaternion {
    let quaternion = XacQuaternion {
        x: file.read_f32::<LittleEndian>().unwrap(),
        y: file.read_f32::<LittleEndian>().unwrap(),
        z: file.read_f32::<LittleEndian>().unwrap(),
        w: file.read_f32::<LittleEndian>().unwrap(),
    };
    quaternion
}

pub(crate) fn xac_read_matrix44(file: &mut File) -> XacMatrix44 {
    let matrix44 = XacMatrix44 {
        axis_1: xac_read_vec4d(file),
        axis_2: xac_read_vec4d(file),
        axis_3: xac_read_vec4d(file),
        pos: xac_read_vec4d(file),
    };
    matrix44
}
