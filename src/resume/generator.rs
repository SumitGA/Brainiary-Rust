use super::model::ResumeRequest;
use reqwest::Client;
use std::env;

pub async fn generate_resume(data: &ResumeRequest) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let prompt = format!(
        "Generate a resume for the following:\nName: {}\nExperience: {}\nSkills: {}\nEducation: {}",
        data.name,
        data.experience.join("; "),
        data.skills.join(", "),
        data.education.join("; ")
    );

    let body = serde_json::json!({
        "model": "gpt-4",
        "messages": [{"role": "user", "content": prompt}]
    });

    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(env::var("OPENAPI_API_KEY").unwrap())
        .json(&body)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(resp["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or_default()
        .to_string())
}