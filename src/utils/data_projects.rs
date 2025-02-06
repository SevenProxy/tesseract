use serde::Deserialize;
use gloo_net::http::Request;


#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Project {
  pub title: String,
  pub description: String,
  pub banner: String,
  pub url: String,
  pub icon_tech: Vec<String>,
}

pub async fn fetch_projects() -> Result<Vec<Project>, String> {
  match Request::get("projects.json").send().await {
    Ok(response) =>
      match response.json::<Vec<Project>>().await {
        Ok(projects) =>
          Ok(projects),
        Err(err) =>
          Err(format!("Erro ao desserializar JSON: {:?}", err)),
      },
    Err(err) =>
      Err(format!("Erro ao buscar JSON: {:?}", err)),
  }
}