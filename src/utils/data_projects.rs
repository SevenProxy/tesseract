use once_cell::sync::Lazy;

pub struct Project {
  pub title: String,
  pub description: String,
  pub banner: String,
  pub url: String,
  pub icon_tech: Vec<String>,
}


pub static FETCH_PROJECTS: Lazy<Vec<Project>> = Lazy::new(|| vec![
  Project {
    title: String::from("asfasf"),
    description: String::from("asfasf"),
    banner: String::from("https://minimalist-portfolio-joao-batista-snowy.vercel.app/web-portifolio.png"),
    url: String::from("https://minimalist-portfolio-joao-batista-snowy.vercel.app/web-portifolio.png"),
    icon_tech: vec![
      String::from("https://minimalist-portfolio-joao-batista-snowy.vercel.app/html.svg"),
    ],
  }
]);