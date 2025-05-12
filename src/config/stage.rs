use anyhow::{Ok, Result};
use std::fmt;
#[derive(Debug, Clone, Default, PartialEq)]
pub enum Stage {
    Local,
    #[default]//Set Development เป็น default
    Development,
    Production,
}
// แปลง enum เป็น string เพื่อ return stage format string
impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stage = match self {
            Stage::Local => "Local",
            Stage::Development => "Development",
            Stage::Production => "Production",
        };
        write!(f, "{}",stage)
    }
}

// แปลง string เป็น enum
impl Stage {
 pub fn try_from(stage: &str) -> Result<Self>{
    match stage {
        "Local" => Ok(Stage::Local),
        "Development" => Ok(Stage::Development),
        "Production" => Ok(Stage::Production),
        _ => Err(anyhow::anyhow!("Invalid stage")),// ถ้าหาไม่เจอจะ return 
    }
 } 
}