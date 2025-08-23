use std::io::{self, Write};

#[derive(Debug, Clone, Copy)]
enum BoundaryLayer {
    LaminarLaminar,
    LaminarTurbulent,
    TurbulentTurbulent,
}

impl BoundaryLayer {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "L/L" => Some(BoundaryLayer::LaminarLaminar),
            "L/T" => Some(BoundaryLayer::LaminarTurbulent),
            "T/T" => Some(BoundaryLayer::TurbulentTurbulent),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            BoundaryLayer::LaminarLaminar => "L/L",
            BoundaryLayer::LaminarTurbulent => "L/T",
            BoundaryLayer::TurbulentTurbulent => "T/T",
        }
    }
}

#[derive(Debug)]
struct ProjectileInput {
    ref_diameter: f64,      // D1 - Reference diameter (mm)
    total_length: f64,       // L1 - Total length (calibers)
    nose_length: f64,        // L2 - Nose length (calibers)
    rt_r: f64,              // R1 - RT/R headshape parameter
    boattail_length: f64,    // L3 - Boattail length (calibers)
    base_diameter: f64,      // D2 - Base diameter (calibers)
    meplat_diameter: f64,    // D3 - Meplat diameter (calibers)
    band_diameter: f64,      // D4 - Rotating band diameter (calibers)
    cg_location: f64,        // X1 - Center of gravity location (calibers from nose)
    boundary_layer: BoundaryLayer,
    identification: String,
}

#[derive(Debug)]
struct DragCoefficients {
    mach: f64,
    cd0: f64,    // Total drag coefficient
    cdh: f64,    // Head drag coefficient
    cdsf: f64,   // Skin friction drag coefficient
    cdbnd: f64,  // Band drag coefficient
    cdbt: f64,   // Boattail drag coefficient
    cdb: f64,    // Base drag coefficient
    pb_pinf: f64, // Base pressure ratio
}

impl ProjectileInput {
    fn new() -> io::Result<Self> {
        println!("ENTER THE MCDRAG INPUTS, ONE QUANTITY AT A TIME.");
        println!();

        print!("ENTER PROJECTILE REFERENCE DIAMETER (MM): ");
        io::stdout().flush()?;
        let ref_diameter = read_float()?;
        println!();

        print!("ENTER TOTAL PROJECTILE LENGTH (CALIBERS): ");
        io::stdout().flush()?;
        let total_length = read_float()?;
        println!();

        print!("ENTER NOSE LENGTH (CALIBERS): ");
        io::stdout().flush()?;
        let nose_length = read_float()?;
        println!();

        print!("ENTER RT/R (HEADSHAPE PARAMETER): ");
        io::stdout().flush()?;
        let rt_r = read_float()?;
        println!();

        print!("ENTER BOATTAIL LENGTH (CALIBERS): ");
        io::stdout().flush()?;
        let boattail_length = read_float()?;
        println!();

        print!("ENTER BASE DIAMETER (CALIBERS): ");
        io::stdout().flush()?;
        let base_diameter = read_float()?;
        println!();

        print!("ENTER MEPLAT DIAMETER (CALIBERS): ");
        io::stdout().flush()?;
        let meplat_diameter = read_float()?;
        println!();

        print!("ENTER ROTATING BAND DIAMETER (CALIBERS): ");
        io::stdout().flush()?;
        let band_diameter = read_float()?;
        println!();

        println!("[NOTE: CENTER OF GRAVITY LOCATION IS OPTIONAL; IF UNKNOWN, ENTER 0]");
        println!();
        print!("ENTER CENTER OF GRAVITY LOCATION (CALIBERS FROM NOSE): ");
        io::stdout().flush()?;
        let cg_location = read_float()?;
        println!();

        println!("FOR ALL LAMINAR BOUNDARY LAYER, CODE = L/L");
        println!("FOR LAMINAR NOSE, TURBULENT AFTERBODY, CODE = L/T");
        println!("FOR ALL TURBULENT BOUNDARY LAYER, CODE = T/T");
        println!();

        let boundary_layer = loop {
            print!("ENTER THE BOUNDARY LAYER CODE (L/L, L/T, OR T/T): ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            
            if let Some(bl) = BoundaryLayer::from_str(input) {
                break bl;
            } else {
                println!("INCORRECT BOUNDARY LAYER CODE. PLEASE TRY AGAIN.");
            }
        };
        println!();

        print!("ENTER PROJECTILE IDENTIFICATION: ");
        io::stdout().flush()?;
        let mut identification = String::new();
        io::stdin().read_line(&mut identification)?;
        let identification = identification.trim().to_string();

        Ok(ProjectileInput {
            ref_diameter,
            total_length,
            nose_length,
            rt_r,
            boattail_length,
            base_diameter,
            meplat_diameter,
            band_diameter,
            cg_location,
            boundary_layer,
            identification,
        })
    }

    fn calculate_drag_coefficients(&self) -> Vec<DragCoefficients> {
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
            
            // Skin friction coefficients
            let c7 = (1.328 / reynolds.sqrt()) * (1.0 + 0.12 * m2).powf(-0.12);
            let c8 = (0.455 / log_reynolds.powf(2.58)) * (1.0 + 0.21 * m2).powf(-0.32);
            
            // Surface area calculations
            let d5 = 1.0 + (0.333 + 0.02 / (self.nose_length * self.nose_length)) * self.rt_r;
            let s1 = 1.5708 * self.nose_length * d5 * (1.0 + 1.0 / (8.0 * self.nose_length * self.nose_length));
            let s2 = 3.1416 * (self.total_length - self.nose_length);
            let s3 = s1 + s2;
            
            // Boundary layer dependent skin friction
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
            
            // Wave drag calculations
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
            
            // Base pressure calculations
            let p2 = if mach < 1.0 {
                1.0 / (1.0 + 0.1875 * m2 + 0.0531 * m2 * m2)
            } else {
                1.0 / (1.0 + 0.2477 * m2 + 0.0345 * m2 * m2)
            };
            
            let p4 = (1.0 + 0.09 * m2 * (1.0 - (-self.total_length + self.nose_length).exp())) 
                     * (1.0 + 0.25 * m2 * (1.0 - self.base_diameter));
            
            let pb_pinf = (p2 * p4).max(0.0);
            
            let cdb = (1.4286 * (1.0 - pb_pinf) * self.base_diameter * self.base_diameter) / m2;
            
            // Band drag
            let cdbnd = if mach < 0.95 {
                mach.powf(12.5) * (self.band_diameter - 1.0)
            } else {
                (0.21 + 0.28 / m2) * (self.band_diameter - 1.0)
            };
            
            // Calculate head and boattail drag based on Mach regime
            let (cdh, cdbt) = if mach <= 1.0 {
                // Subsonic/transonic
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
                // Supersonic
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

    fn print_diagnostics(&self) {
        if self.nose_length < 1.0 {
            println!("NOSE TOO SHORT. CDH IS TOO HIGH AT TRANSONIC AND SUPERSONIC SPEEDS.");
        }
        if self.meplat_diameter > 0.5 {
            println!("NOSE TOO BLUNT. CDH IS TOO HIGH AT TRANSONIC AND SUPERSONIC SPEEDS.");
        }
        if self.boattail_length >= 1.5 {
            println!("BOATTAIL TOO LONG. CDBT AND CDB MAY BE INCORRECT.");
        }
        if self.base_diameter < 0.65 {
            println!("BOATTAIL TOO STEEP. CDBT AND CDB MAY BE INCORRECT.");
        } else if self.base_diameter > 1.35 {
            println!("CONICAL FLARE TAIL TOO STEEP. CDBT AND CDB MAY BE INCORRECT.");
        }
    }
}

fn read_float() -> io::Result<f64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    input.trim().parse().map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidInput, "Invalid number")
    })
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() -> io::Result<()> {
    loop {
        clear_screen();
        
        let input = ProjectileInput::new()?;
        
        clear_screen();
        println!("MCDRAG, DECEMBER 1974, R. L. MCCOY");
        println!();
        println!("PROJECTILE IDENTIFICATION: {}", input.identification);
        println!();
        
        // Print input parameters header
        println!("  REF.    TOTAL     NOSE    RT/R  BOATTAIL   BASE   MEPLAT   BAND     XCG   BOUND.");
        println!("  DIA.   LENGTH   LENGTH          LENGTH    DIA.    DIA.    DIA.    NOSE   LAYER");
        println!("  (MM)    (CAL)    (CAL)           (CAL)    (CAL)   (CAL)   (CAL)   (CAL)   CODE");
        println!();
        
        // Print input values
        println!("{:7.2} {:7.2} {:7.3} {:6.3} {:7.3} {:6.3} {:6.3} {:6.3} {:6.2}   {}",
                 input.ref_diameter, input.total_length, input.nose_length, input.rt_r,
                 input.boattail_length, input.base_diameter, input.meplat_diameter,
                 input.band_diameter, input.cg_location, input.boundary_layer.to_str());
        println!();
        println!();
        
        // Calculate and print results
        let results = input.calculate_drag_coefficients();
        
        println!("   M      CD0      CDH     CDSF    CDBND     CDBT     CDB    PB/PINF");
        println!();
        
        for coeff in &results {
            println!("{:6.3} {:7.3} {:7.3} {:7.3} {:7.3} {:7.3} {:7.3} {:7.3}",
                     coeff.mach, coeff.cd0, coeff.cdh, coeff.cdsf,
                     coeff.cdbnd, coeff.cdbt, coeff.cdb, coeff.pb_pinf);
        }
        
        println!();
        println!();
        
        // Print diagnostics
        input.print_diagnostics();
        
        println!();
        println!();
        
        // Ask about hardcopy (we'll skip actual printing in this implementation)
        print!("COPY THIS? (ENTER Y FOR YES, N FOR NO): ");
        io::stdout().flush()?;
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        
        if response.trim().to_uppercase() == "Y" {
            println!("[Note: Hardcopy printing not implemented in this version]");
        }
        
        println!();
        print!("RUN ANOTHER CASE? ENTER Y FOR YES, N FOR NO: ");
        io::stdout().flush()?;
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        
        if response.trim().to_uppercase() != "Y" {
            break;
        }
    }
    
    Ok(())
}