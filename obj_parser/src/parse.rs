use crate::{
    file,
    types::{Face, FacePoint, Material, Object, Vertex, VertexNormal, VertexTexture},
    utils,
};
use std::sync::mpsc;
use std::thread;

fn get_rgb(mtl_content: &String, key: &str) -> Option<(f32, f32, f32)> {
    let mut ret: Option<(f32, f32, f32)> = None;

    if let Some(line) = mtl_content.lines().find(|x| x.contains(key)) {
        let words: Vec<&str> = line.split_whitespace().collect();

        if words.len() >= 4 && words[0] == key {
            if let (Ok(r), Ok(g), Ok(b)) = (
                words[1].parse::<f32>(),
                words[2].parse::<f32>(),
                words[3].parse::<f32>(),
            ) {
                ret = Some((r, g, b));
            }
        }
    }
    ret
}

fn get_single_val<T: std::str::FromStr>(mtl_content: &str, key: &str) -> Option<T> {
    let mut result: Option<T> = None;

    if let Some(line) = mtl_content.lines().find(|x| x.starts_with(key)) {
        let words: Vec<&str> = line.split_whitespace().collect();

        if words.len() == 2 && words[0] == key {
            if let Ok(value) = words[1].parse::<T>() {
                result = Some(value);
            }
        }
    }

    result
}

//TAKE OBJ PATH AND MLT NAME AND JOIN THEM TOGETHER
fn get_mtl_path(obj_path: &String, obj_content: &String) -> Option<String> {
    let mlt_name = obj_content
        .lines()
        .find(|&x| x.contains("mtllib "))
        .unwrap_or("")
        .split(" ")
        .last()
        .unwrap_or("");
    if mlt_name == "" {
        return None;
    }
    let mut mlt_path: Vec<&str> = obj_path.split("/").collect();
    mlt_path.pop();
    mlt_path.insert(mlt_path.len(), mlt_name);
    Some(mlt_path.join("/"))
}

fn parse_mlt_file(mlt_content: &String) -> Option<Material> {
    let mat_name: Option<String> = get_single_val(mlt_content, "newmtl");
    let mat_ni: Option<f32> = get_single_val(mlt_content, "Ni");
    let mat_ns: Option<f32> = get_single_val(mlt_content, "Ns");
    let mat_d: Option<f32> = get_single_val(mlt_content, "d");
    let mat_illum: Option<u8> = get_single_val(mlt_content, "illum");
    let mat_mapka: Option<String> = get_single_val(mlt_content, "map_Ka");
    let mat_ka = get_rgb(mlt_content, "Ka");
    let mat_kd = get_rgb(mlt_content, "Kd");
    let mat_ke = get_rgb(mlt_content, "Ke");
    let mat_ks = get_rgb(mlt_content, "Ks");

    let mat = Material {
        newmtl: mat_name.unwrap_or("".to_string()),
        d: mat_d.unwrap_or(0.0),
        illum: mat_illum.unwrap_or(1),
        ns: mat_ns.unwrap_or(0.0),
        ni: mat_ni.unwrap_or(0.0),
        ka: mat_ka.unwrap_or((0.0, 0.0, 0.0)),
        kd: mat_kd.unwrap_or((0.0, 0.0, 0.0)),
        ks: mat_ks.unwrap_or((0.0, 0.0, 0.0)),
        ke: mat_ke.unwrap_or((0.0, 0.0, 0.0)),
        mapka: mat_mapka.unwrap_or("".to_string()),
    };

    //check if all data are good
    match mat.check() {
        Ok(..) => {}
        Err(err) => {
            println!("Error : {err}");
            std::process::exit(1);
        }
    }

    Some(mat)
}

fn parse_obj_file(obj_content: &String) -> Option<Object> {
    let mut vertex: Vec<Vertex> = Vec::new();
    let mut vertex_texture: Vec<VertexTexture> = Vec::new();
    let mut vertex_normal: Vec<VertexNormal> = Vec::new();

    //parse vertex
    for line in obj_content.lines() {
        let mut iter = line.split_whitespace();

        if let Some("v") = iter.next() {
            if let (Some(x), Some(y), Some(z)) = (iter.next(), iter.next(), iter.next()) {
                if let (Ok(x), Ok(y), Ok(z)) =
                    (x.parse::<f32>(), y.parse::<f32>(), z.parse::<f32>())
                {
                    let w = if let Some(w_str) = iter.next() {
                        w_str.parse::<f32>().ok()
                    } else {
                        None
                    };
                    vertex.push(Vertex { x, y, z, w })
                }
            }
        }
    }

    //parse vertex_texture
    for line in obj_content.lines() {
        let mut iter = line.split_whitespace();

        if let Some("vt") = iter.next() {
            if let (Some(u), Some(v)) = (iter.next(), iter.next()) {
                if let (Ok(u), Ok(v)) = (u.parse::<f32>(), v.parse::<f32>()) {
                    let w = if let Some(w_str) = iter.next() {
                        w_str.parse::<f32>().ok()
                    } else {
                        None
                    };
                    vertex_texture.push(VertexTexture { u, v, w })
                }
            }
        }
    }

    //parse vertex_normal
    for line in obj_content.lines() {
        let mut iter = line.split_whitespace();

        if let Some("vn") = iter.next() {
            if let (Some(x), Some(y), Some(z)) = (iter.next(), iter.next(), iter.next()) {
                if let (Ok(x), Ok(y), Ok(z)) =
                    (x.parse::<f32>(), y.parse::<f32>(), z.parse::<f32>())
                {
                    vertex_normal.push(VertexNormal { x, y, z })
                }
            }
        }
    }

    //parse obj_name
    let obj_name: Vec<&str> = obj_content
        .lines()
        .find(|&x| x.contains("o "))
        .unwrap_or("Defaut_object_name")
        .split(" ")
        .skip(1)
        .collect();
    let mut obj_name = obj_name.join("_");
    if obj_name.len() < 1 {
        obj_name = String::from("Defaut_object_name");
    };

    //parse obj_group
    let obj_input: Vec<&str> = obj_content
        .lines()
        .find(|&x| x.contains("g "))
        .unwrap_or("")
        .split(" ")
        .skip(1)
        .collect();
    let obj_input = obj_input.join("_");
    let mut obj_group: Option<String> = None;
    if obj_input.len() >= 1 {
        obj_group = Some(obj_input);
    };
    //parse faces
    let lines: Vec<&str> = obj_content
        .lines()
        .filter(|x| x.starts_with("f "))
        .collect();
    let mut faces: Vec<Face> = Vec::new();

    for line in lines {
        let mut worlds = line.split_whitespace();
        let mut face = Face { points: Vec::new() };
        worlds.next();
        while let Some(vertex) = worlds.next() {
            let mut vertex_split = vertex.split("/");
            let mut face_point: FacePoint = FacePoint {
                v_id: 0,
                vt_id: None,
                vn_id: None,
            };
            if let Some(v_id) = vertex_split.next() {
                face_point.v_id = v_id.parse::<u32>().unwrap_or_else(|_| {
                    utils::exit("Error: Invalid input on obj file (wrong f input)", 1);
                    0
                });
            };
            if let Some(vt_id) = vertex_split.next() {
                face_point.vt_id = Some(vt_id.parse::<u32>().unwrap_or_else(|_| {
                    utils::exit("Error: Invalid input on obj file (wrong f input)", 1);
                    0
                }));
            };
            if let Some(vn_id) = vertex_split.next() {
                face_point.vn_id = Some(vn_id.parse::<u32>().unwrap_or_else(|_| {
                    utils::exit("Error: Invalid input on obj file (wrong f input)", 1);
                    0
                }))
            };
            face.points.push(face_point);
        }
        if !face.points.is_empty() {
            faces.push(face);
        }
    }

    let obj = Object {
        v: vertex,
        vt: vertex_texture,
        vn: vertex_normal,
        mtllib: None,
        o: obj_name,
        g: obj_group,
        f: faces,
    };

    match obj.check() {
        Ok(..) => {}
        Err(err) => utils::exit(&err, 1),
    }

    Some(obj)
}

//pars entry
pub fn pars_obj(obj_content: &String, obj_path: &String) -> (Option<Object>, Option<Material>) {
    let mlt_path = get_mtl_path(obj_path, obj_content);
    let mut mlt_content: String = String::new();
    match mlt_path {
        Some(path) => mlt_content = file::read_file(&path),
        None => println!("Log: no texture set on file"),
    }
    if !mlt_content.is_empty() {
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            tx.send(parse_mlt_file(&mlt_content)).unwrap();
        });
        let object = parse_obj_file(&obj_content).unwrap();
        handle.join().unwrap();
        let material = rx
            .recv()
            .unwrap()
            .unwrap_or_else(|| panic!("Error: error during material parsing"));
        println!("{:#?}\n\n{:#?}", object, material);
        return (Some(object), Some(material));
    } else {
        let object = parse_obj_file(&obj_content).unwrap();
        return (Some(object), None);
    }
}
