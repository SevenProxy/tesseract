use once_cell::sync::Lazy;

pub struct Certificates {
  pub title: String,
  pub description: String,
  pub icon: String,
  pub url: String,
}


pub static FETCH_CERTIFICATES: Lazy<Vec<Certificates>> = Lazy::new(|| vec![
  Certificates {
    title: String::from("Full Stack JavaScript"),
    description: String::from("Javascript e TypeScript - front-end e back-end (Full Stack) - Node, Express, noSQL, React, hooks, Redux, Design Patterns"),
    icon: String::from("/public/exp1.svg"),
    url: String::from("https://drive.google.com/file/d/1soq3TQUq8SLwbgocA8_Mlu6EstTD3Ovn/view?usp=sharing")
  },
  Certificates {
    title: String::from("Back End Python"),
    description: String::from("Python 3+ completo: PySide6, Django, Selenium, Regexp, Testes, TDD, POO, Design Patterns GoF, algoritmos e programação"),
    icon: String::from("/public/exp2.svg"),
    url: String::from("https://drive.google.com/file/d/1JuesNkFwRiET_kbRIN7oT1QtB3w6EWpI/view?usp=sharing")
  },
  Certificates {
    title: String::from("Analise e Desenvolvimento de Sistemas"),
    description: String::from("Próximo Curso"),
    icon: String::from("/public/exp4.svg"),
    url: String::from("https://inscricoes.unicesumar.edu.br/curso/analise-e-desenvolvimento-de-sistemas")
  },
]);