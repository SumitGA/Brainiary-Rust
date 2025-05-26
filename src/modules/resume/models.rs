use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResumeRequest {
    pub name: String, 
    pub experience: Vec<String>,
    pub skills: Vec<String>,
    pub education: Vec<String>,
}
