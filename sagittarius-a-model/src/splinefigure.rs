use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SplinefigureId {
    pub id: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SplinefigurePoint {
    pub vertex: String,
    pub x_in_tangent:String,
    pub y_in_tangent:String,
    pub z_in_tangent:String,
    pub x_out_tangent:String,
    pub y_out_tangent:String,
    pub z_out_tangent:String,
    pub x_pos:String,
    pub y_pos:String,
    pub z_pos:String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Splinefigure {
    pub id: String,
    pub name: String,
    pub points: Vec<SplinefigurePoint>
}
