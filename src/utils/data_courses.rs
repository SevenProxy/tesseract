use once_cell::sync::Lazy;

pub struct Courses {
  pub title: String,
  pub name: String,
  pub description: String,

}


pub static FETCH_COURSES: Lazy<Vec<Courses>> = Lazy::new(|| vec![
  Courses {
    title: String::from("Unicessumar"),
    name: String::from("Analise e Desenvolvimento de Sistemas"),
    description: String::from("Faculdade em ADS na Instituição a Distância na Unicessumar."),
  }
]);