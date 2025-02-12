use once_cell::sync::Lazy;

pub struct Courses {
  pub title: String,
  pub name: String,
  pub description: String,
  pub icon: String,
}


pub static FETCH_COURSES: Lazy<Vec<Courses>> = Lazy::new(|| vec![
  Courses {
    title: String::from("Unicessumar"),
    name: String::from("Analise e Desenvolvimento de Sistemas"),
    description: String::from("Faculdade em ADS na Instituição a Distância na Unicessumar."),
    icon: String::from("https://venhaparaunicesumar.com.br/favicon.ico"),
  },
  Courses {
    title: String::from("Udemy"),
    name: String::from("Curso de Python 3 do básico ao avançado - com projetos reais"),
    description: String::from("Python 3+ completo: PySide6, Django, Selenium, Regexp, Testes, TDD, POO, Design Patterns GoF, algoritmos e programação"),
    icon: String::from("https://www.udemy.com/staticx/udemy/images/v8/favicon-32x32.png")
  },
  Courses {
    title: String::from("Udemy"),
    name: String::from("Curso de JavaScript e TypeScript do básico ao avançado JS/TS"),
    description: String::from("Javascript e TypeScript - front-end e back-end (Full Stack) - Node, Express, noSQL, React, hooks, Redux, Design Patterns"),
    icon: String::from("https://www.udemy.com/staticx/udemy/images/v8/favicon-32x32.png")
  }
]);