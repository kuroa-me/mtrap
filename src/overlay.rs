use geo_types::{coord, LineString};
use polyline;
use std::fmt;

pub enum PinName {
    PinS,
    PinL,
}

impl fmt::Display for PinName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PinName::PinS => write!(f, "pin-s"),
            PinName::PinL => write!(f, "pin-l"),
        }
    }
}

struct Marker {
    pub name: PinName,
    pub label: Option<char>,
    pub color: Option<String>,
    pub lon: f64,
    pub lat: f64,
}

impl Marker {
    fn new(lon: f64, lat: f64) -> Self {
        Marker {
            name: PinName::PinS,
            label: None,
            color: None,
            lon,
            lat,
        }
    }
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut marker = String::new();
        marker.push_str(&format!("{}", self.name));
        if let Some(label) = self.label {
            marker.push_str(&format!("-{}", label));
        };
        if let Some(color) = &self.color {
            marker.push_str(&format!("+{}", color));
        };
        marker.push_str(&format!("({},{})", self.lon, self.lat));
        write!(f, "{}", marker)
    }
}

struct Path {
    pub stroke_width: Option<i32>,
    pub stroke_color: Option<String>,
    pub stroke_opacity: Option<f32>,
    pub fill_color: Option<f32>,
    pub fill_opacity: Option<f32>,
    polyline: String,
}

impl Path {
    fn new() -> Self {
        Path {
            stroke_width: None,
            stroke_color: None,
            stroke_opacity: None,
            fill_color: None,
            fill_opacity: None,
            polyline: String::new(),
        }
    }

    fn from_marker(markers: Vec<Marker>) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut path = Path::new();
        let coords: LineString = markers
            .into_iter()
            .map(|marker| coord! {x: marker.lon, y: marker.lat})
            .collect();
        path.polyline = polyline::encode_coordinates(coords, 5)?;
        Ok(path)
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut path = String::new();
        if let Some(stroke_width) = self.stroke_width {
            path.push_str(&format!("-{},", stroke_width));
        };
        if let Some(stroke_color) = &self.stroke_color {
            path.push_str(&format!("+{},", stroke_color));
        };
        if let Some(stroke_opacity) = self.stroke_opacity {
            path.push_str(&format!("-{},", stroke_opacity));
        };
        if let Some(fill_color) = self.fill_color {
            path.push_str(&format!("+{}", fill_color));
        };
        if let Some(fill_opacity) = self.fill_opacity {
            path.push_str(&format!("-{}", fill_opacity));
        };
        write!(f, "{}({})", path, self.polyline)
    }
}
