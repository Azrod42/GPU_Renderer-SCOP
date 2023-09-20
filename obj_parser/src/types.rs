use crate::utils::check_rgb;

#[derive(Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: Option<f32>,
}

#[derive(Debug)]
pub struct VertexTexture {
    pub u: f32,
    pub v: f32,
    pub w: Option<f32>,
}

#[derive(Debug)]
pub struct VertexNormal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct FacePoint {
    pub v_id: u32,
    pub vt_id: Option<u32>,
    pub vn_id: Option<u32>,
}

#[derive(Debug)]
pub struct Face {
    pub points: Vec<FacePoint>,
}

#[derive(Debug)]
pub struct Material {
    pub newmtl: String,
    pub mapka: String,
    pub ka: (f32, f32, f32),
    pub kd: (f32, f32, f32),
    pub ks: (f32, f32, f32),
    pub ke: (f32, f32, f32),
    pub ni: f32,
    pub illum: u8,
    pub ns: f32,
    pub d: f32,
}
impl Material {
    pub fn check(&self) -> Result<bool, String> {
        //check newmtl (name)
        if self.newmtl.is_empty() {
            return Err(String::from("invalid / missing name on mtl file"));
        }

        //check ns
        if self.ns > 100.0 || self.ns < 0.0 {
            return Err(String::from("invalid ns(specular) on mtl file "));
        }

        //check ni
        if self.ni > 1.0 || self.ni < 0.0 {
            return Err(String::from("invalid ni(optical density) on mtl file "));
        }

        //check d
        if self.d > 1.0 || self.d < 0.0 {
            return Err(String::from("invalid d(transparency) on mtl file "));
        }

        //check illum
        match self.illum {
            1 | 2 => {}
            _ => {
                return Err(String::from("invalid illum(light settings) on mtl file "));
            }
        }

        //check ka
        match check_rgb(&self.ka) {
            Err(err) => return Err(err),
            Ok(..) => {}
        }

        //check kd
        match check_rgb(&self.kd) {
            Err(err) => return Err(err),
            Ok(..) => {}
        }

        //check ks
        match check_rgb(&self.ks) {
            Err(err) => return Err(err),
            Ok(..) => {}
        }

        //check ke
        match check_rgb(&self.ke) {
            Err(err) => return Err(err),
            Ok(..) => {}
        }
        Ok(true)
    }
}

#[derive(Debug)]
pub struct Object {
    //vertex points in 3D space
    pub v: Vec<Vertex>,

    //texture coordinates
    pub vt: Vec<VertexTexture>,

    //Defines normals
    pub vn: Vec<VertexNormal>,

    //face vertex
    pub f: Vec<Face>,

    //Point to an external .mtl file
    pub mtllib: Option<Material>,

    //Object name
    pub o: String,

    //group name
    pub g: Option<String>,
    //Defines a material to use, this material will continue to be used until another usemtl line (corresponding material is saved in .mtl file)
    // usemtl: String,
}

impl Object {
    pub fn check(&self) -> Result<bool, String> {
        if self.v.len() < 1 {
            return Err(String::from("no vector on object file"));
        }
        Ok(true)
    }
}
