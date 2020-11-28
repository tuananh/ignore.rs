pub async fn generate(keywords: Vec<String>) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let joined_keywords = keywords.join(",");
    let url = format!("https://www.toptal.com/developers/gitignore/api/{}", joined_keywords);
    let mut res = surf::get(url).await?;
    Ok(res.body_string().await?)
}
