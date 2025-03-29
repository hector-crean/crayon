use serde::{Deserialize, Serialize};




#[cfg_attr(feature = "postgres", derive(sqlx::FromRow))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkylarkBlock {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Optimisation")]
    pub optimisation: f32,
    #[serde(rename = "Fixings_quantity")]
    pub fixings_quantity: Option<f32>,
    #[serde(rename = "Insulation Type")]
    pub insulation_type: Option<Vec<String>>,
    #[serde(rename = "Sheet Quantity")]
    pub sheet_quantity: Option<f32>,
    #[serde(rename = "Latest release")]
    pub latest_release: Option<String>,
    #[serde(rename = "Assembly_rate")]
    pub assembly_rate: Option<Vec<String>>,
    #[serde(rename = "Length (mm)")]
    pub length: f32,
    #[serde(rename = "Structural Timber")]
    pub structural_timber: Vec<String>,
    #[serde(rename = "Insulation (m2)")]
    pub insulation_area: f32,
    #[serde(rename = "Est. Assembly Time (person-hrs)")]
    pub est_assembly_time_person_hrs: Option<f32>,
    #[serde(rename = "Github_assembly_guide")]
    pub github_assembly_guide: Option<String>,
    #[serde(rename = "Manufacturing_rate_type")]
    pub manufacturing_rate_type: Vec<String>,
    #[serde(rename = "Fixings")]
    pub fixings: Vec<String>,
    #[serde(rename = "Skylark_series")]
    pub skylark_series: String,
    #[serde(rename = "Height (mm)")]
    pub height: f32,
    #[serde(rename = "Width (mm)")]
    pub width: f32,
    #[serde(rename = "Est. mass(kg)")]
    pub est_mass: f32,
    #[serde(rename = "Global Warming Potential total (kgCO2 eq.) A1-A3 (from Structural Timber)")]
    pub gwp_structural_timber: Vec<f32>,
    #[serde(rename = "Global Warming Potential total (kgCO2 eq.) A1-A3 (from Insulation Type)")]
    pub gwp_insulation_type: Vec<f32>,
    #[serde(rename = "Embodied carbon GWP (kgCO2 eq.)")]
    pub embodied_carbon_gwp: f32,
    #[serde(rename = "Cost_per_unit (from Structural Timber)")]
    pub cost_per_unit_structural_timber: Vec<f32>,
    #[serde(rename = "Plywood_cost")]
    pub plywood_cost: f32,
    #[serde(rename = "Manufacturing Time")]
    pub manufacturing_time: f32,
    #[serde(rename = "Cost_per_unit (from Ref 2)")]
    pub cost_per_unit_ref_2: Vec<f32>,
    #[serde(rename = "Manufacturing Cost")]
    pub manufacturing_cost: f32,
    #[serde(rename = "Cost_per_unit (from Fixings)")]
    pub cost_per_unit_fixings: Vec<f32>,
    #[serde(rename = "Fixings_cost")]
    pub fixings_cost: f32,
    #[serde(rename = "Cost_per_unit (from Ref)")]
    pub cost_per_unit_ref: Vec<f32>,
    #[serde(rename = "Assembly_cost")]
    pub assembly_cost: f32,
    #[serde(rename = "Cost_per_unit (from Insulation Type)")]
    pub cost_per_unit_insulation_type: Vec<f32>,
    #[serde(rename = "Insulation_cost")]
    pub insulation_cost: f32,
    #[serde(rename = "Platform_contribution")]
    pub platform_contribution: f32,
    #[serde(rename = "Total_cost")]
    pub total_cost: f32,
    #[serde(rename = "Github_cutting_file")]
    pub github_cutting_file: String,
    #[serde(rename = "Github_model_simple")]
    pub github_model_simple: String,
    #[serde(rename = "Github_model_detailed")]
    pub github_model_detailed: String,
}
