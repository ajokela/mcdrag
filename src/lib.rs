use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BoundaryLayer {
    LaminarLaminar,
    LaminarTurbulent,
    TurbulentTurbulent,
}

impl BoundaryLayer {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "L/L" => Some(BoundaryLayer::LaminarLaminar),
            "L/T" => Some(BoundaryLayer::LaminarTurbulent),
            "T/T" => Some(BoundaryLayer::TurbulentTurbulent),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            BoundaryLayer::LaminarLaminar => "L/L",
            BoundaryLayer::LaminarTurbulent => "L/T",
            BoundaryLayer::TurbulentTurbulent => "T/T",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectileInput {
    pub ref_diameter: f64,
    pub total_length: f64,
    pub nose_length: f64,
    pub rt_r: f64,
    pub boattail_length: f64,
    pub base_diameter: f64,
    pub meplat_diameter: f64,
    pub band_diameter: f64,
    pub cg_location: f64,
    pub boundary_layer: BoundaryLayer,
    pub identification: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DragCoefficients {
    pub mach: f64,
    pub cd0: f64,
    pub cdh: f64,
    pub cdsf: f64,
    pub cdbnd: f64,
    pub cdbt: f64,
    pub cdb: f64,
    pub pb_pinf: f64,
}

#[derive(Serialize, Deserialize)]
pub struct CalculationResult {
    pub coefficients: Vec<DragCoefficients>,
    pub diagnostics: Vec<String>,
    pub input_summary: InputSummary,
}

#[derive(Serialize, Deserialize)]
pub struct InputSummary {
    pub ref_diameter: f64,
    pub total_length: f64,
    pub nose_length: f64,
    pub rt_r: f64,
    pub boattail_length: f64,
    pub base_diameter: f64,
    pub meplat_diameter: f64,
    pub band_diameter: f64,
    pub cg_location: f64,
    pub boundary_layer: String,
    pub identification: String,
}

impl ProjectileInput {
    pub fn calculate_drag_coefficients(&self) -> Vec<DragCoefficients> {
        let mach_numbers = vec![
            0.5, 0.6, 0.7, 0.8, 0.85, 0.9, 0.925, 0.95, 0.975, 1.0,
            1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 2.0, 2.2,
            2.5, 3.0, 3.5, 4.0, 4.5, 5.0,
        ];

        let mut results = Vec::new();

        for &mach in &mach_numbers {
            let t1 = (1.0 - self.meplat_diameter) / self.nose_length;
            let m2 = mach * mach;
            let reynolds = 23296.3 * mach * self.total_length * self.ref_diameter;
            let log_reynolds = reynolds.ln() * 0.4343;
            
            let c7 = (1.328 / reynolds.sqrt()) * (1.0 + 0.12 * m2).powf(-0.12);
            let c8 = (0.455 / log_reynolds.powf(2.58)) * (1.0 + 0.21 * m2).powf(-0.32);
            
            let d5 = 1.0 + (0.333 + 0.02 / (self.nose_length * self.nose_length)) * self.rt_r;
            let s1 = 1.5708 * self.nose_length * d5 * (1.0 + 1.0 / (8.0 * self.nose_length * self.nose_length));
            let s2 = 3.1416 * (self.total_length - self.nose_length);
            let s3 = s1 + s2;
            
            let (c9, c10) = match self.boundary_layer {
                BoundaryLayer::LaminarLaminar => {
                    let c = 1.2732 * s3 * c7;
                    (c, c)
                }
                BoundaryLayer::LaminarTurbulent => {
                    (1.2732 * s3 * c7, 1.2732 * s3 * c8)
                }
                BoundaryLayer::TurbulentTurbulent => {
                    let c = 1.2732 * s3 * c8;
                    (c, c)
                }
            };
            
            let cdsf = (c9 * s1 + c10 * s2) / s3;
            
            let c15 = (m2 - 1.0) / (2.4 * m2);
            
            let p5 = if mach <= 1.0 {
                (1.0 + 0.2 * m2).powf(3.5)
            } else {
                (1.2 * m2).powf(3.5) * (6.0 / (7.0 * m2 - 1.0)).powf(2.5)
            };
            
            let c16 = (1.122 * (p5 - 1.0) * self.meplat_diameter * self.meplat_diameter) / m2;
            
            let c18 = if mach <= 0.91 {
                0.0
            } else if mach >= 1.41 {
                0.85 * c16
            } else {
                (0.254 + 2.88 * c15) * c16
            };
            
            let p2 = if mach < 1.0 {
                1.0 / (1.0 + 0.1875 * m2 + 0.0531 * m2 * m2)
            } else {
                1.0 / (1.0 + 0.2477 * m2 + 0.0345 * m2 * m2)
            };
            
            let p4 = (1.0 + 0.09 * m2 * (1.0 - (-self.total_length + self.nose_length).exp())) 
                     * (1.0 + 0.25 * m2 * (1.0 - self.base_diameter));
            
            let pb_pinf = (p2 * p4).max(0.0);
            
            let cdb = (1.4286 * (1.0 - pb_pinf) * self.base_diameter * self.base_diameter) / m2;
            
            let cdbnd = if mach < 0.95 {
                mach.powf(12.5) * (self.band_diameter - 1.0)
            } else {
                (0.21 + 0.28 / m2) * (self.band_diameter - 1.0)
            };
            
            let (cdh, cdbt) = if mach <= 1.0 {
                let x2 = (1.0 + 0.552 * t1.powf(0.8)).powf(-0.5);
                let c17 = if mach <= x2 {
                    0.0
                } else {
                    0.368 * t1.powf(1.8) + 1.6 * t1 * c15
                };
                
                let cdbt = if self.boattail_length <= 0.0 || mach <= 0.85 {
                    0.0
                } else {
                    let t2 = (1.0 - self.base_diameter) / (2.0 * self.boattail_length);
                    let t3 = 2.0 * t2 * t2 + t2 * t2 * t2;
                    let e1 = (-2.0 * self.boattail_length).exp();
                    let b4 = 1.0 - e1 + 2.0 * t2 * (e1 * (self.boattail_length + 0.5) - 0.5);
                    2.0 * t3 * b4 * (1.0 / (0.564 + 1250.0 * c15 * c15))
                };
                
                (c17 + c18, cdbt)
            } else {
                let b2 = m2 - 1.0;
                let b = b2.sqrt();
                
                let s4 = 1.0 + 0.368 * t1.powf(1.85);
                let z = if mach >= s4 { b } else { (s4 * s4 - 1.0).sqrt() };
                
                let c11 = 0.7156 - 0.5313 * self.rt_r + 0.595 * self.rt_r * self.rt_r;
                let c12 = 0.0796 + 0.0779 * self.rt_r;
                let c13 = 1.587 + 0.049 * self.rt_r;
                let c14 = 0.1122 + 0.1658 * self.rt_r;
                
                let r4 = 1.0 / (z * z);
                let c17 = (c11 - c12 * t1 * t1) * r4 * (t1 * z).powf(c13 + c14 * t1);
                
                let cdbt = if self.boattail_length <= 0.0 {
                    0.0
                } else {
                    let t2 = (1.0 - self.base_diameter) / (2.0 * self.boattail_length);
                    
                    if mach <= 1.1 {
                        let t3 = 2.0 * t2 * t2 + t2 * t2 * t2;
                        let e1 = (-2.0 * self.boattail_length).exp();
                        let b4 = 1.0 - e1 + 2.0 * t2 * (e1 * (self.boattail_length + 0.5) - 0.5);
                        2.0 * t3 * b4 * (1.774 - 9.3 * c15)
                    } else {
                        let b3 = 0.85 / b;
                        let a12 = (5.0 * t1) / (6.0 * b) + (0.5 * t1).powf(2.0) 
                                  - (0.7435 / m2) * (t1 * mach).powf(1.6);
                        let a11 = (1.0 - (0.6 * self.rt_r) / mach) * a12;
                        let e2 = ((-1.1952 / mach) * (self.total_length - self.nose_length - self.boattail_length)).exp();
                        let x3 = ((2.4 * m2 * m2 - 4.0 * b2) * t2 * t2) / (2.0 * b2 * b2);
                        let a1 = a11 * e2 - x3 + (2.0 * t2) / b;
                        let r5 = 1.0 / b3;
                        let e3 = (-b3 * self.boattail_length).exp();
                        let a2 = 1.0 - e3 + 2.0 * t2 * (e3 * (self.boattail_length + r5) - r5);
                        4.0 * a1 * t2 * a2 * r5
                    }
                };
                
                (c17 + c18, cdbt)
            };
            
            let cd0 = cdh + cdsf + cdbnd + cdbt + cdb;
            
            results.push(DragCoefficients {
                mach,
                cd0,
                cdh,
                cdsf,
                cdbnd,
                cdbt,
                cdb,
                pb_pinf,
            });
        }
        
        results
    }

    pub fn get_diagnostics(&self) -> Vec<String> {
        let mut diagnostics = Vec::new();
        
        if self.nose_length < 1.0 {
            diagnostics.push("NOSE TOO SHORT. CDH IS TOO HIGH AT TRANSONIC AND SUPERSONIC SPEEDS.".to_string());
        }
        if self.meplat_diameter > 0.5 {
            diagnostics.push("NOSE TOO BLUNT. CDH IS TOO HIGH AT TRANSONIC AND SUPERSONIC SPEEDS.".to_string());
        }
        if self.boattail_length >= 1.5 {
            diagnostics.push("BOATTAIL TOO LONG. CDBT AND CDB MAY BE INCORRECT.".to_string());
        }
        if self.base_diameter < 0.65 {
            diagnostics.push("BOATTAIL TOO STEEP. CDBT AND CDB MAY BE INCORRECT.".to_string());
        } else if self.base_diameter > 1.35 {
            diagnostics.push("CONICAL FLARE TAIL TOO STEEP. CDBT AND CDB MAY BE INCORRECT.".to_string());
        }
        
        diagnostics
    }
}

#[wasm_bindgen]
pub struct McDragCalculator {
    current_input: Option<ProjectileInput>,
}

#[wasm_bindgen]
impl McDragCalculator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> McDragCalculator {
        McDragCalculator {
            current_input: None,
        }
    }

    #[wasm_bindgen]
    pub fn set_input(&mut self, input_json: &str) -> Result<(), JsValue> {
        match serde_json::from_str::<ProjectileInput>(input_json) {
            Ok(input) => {
                self.current_input = Some(input);
                Ok(())
            }
            Err(e) => Err(JsValue::from_str(&format!("Invalid input: {}", e)))
        }
    }

    #[wasm_bindgen]
    pub fn calculate(&self) -> Result<String, JsValue> {
        match &self.current_input {
            Some(input) => {
                let coefficients = input.calculate_drag_coefficients();
                let diagnostics = input.get_diagnostics();
                
                let result = CalculationResult {
                    coefficients,
                    diagnostics,
                    input_summary: InputSummary {
                        ref_diameter: input.ref_diameter,
                        total_length: input.total_length,
                        nose_length: input.nose_length,
                        rt_r: input.rt_r,
                        boattail_length: input.boattail_length,
                        base_diameter: input.base_diameter,
                        meplat_diameter: input.meplat_diameter,
                        band_diameter: input.band_diameter,
                        cg_location: input.cg_location,
                        boundary_layer: input.boundary_layer.to_str().to_string(),
                        identification: input.identification.clone(),
                    },
                };
                
                match serde_json::to_string(&result) {
                    Ok(json) => Ok(json),
                    Err(e) => Err(JsValue::from_str(&format!("Serialization error: {}", e)))
                }
            }
            None => Err(JsValue::from_str("No input data set"))
        }
    }

    #[wasm_bindgen]
    pub fn validate_boundary_layer(code: &str) -> bool {
        BoundaryLayer::from_str(code).is_some()
    }
}