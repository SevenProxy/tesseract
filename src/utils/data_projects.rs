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
    title: String::from("Hospedagem Template"),
    description: String::from("Desenvolvido com Next.js, Typescript, Animate.css, Tailwindcss, este projeto visa apresentar um template de host usando as tecnologias citadas."),
    banner: String::from("/public/cheetah.png"),
    url: String::from("https://cheetahproject.netlify.app"),
    icon_tech: vec![
      String::from("/public/next.svg"),
      String::from("/public/tail.svg"),
      String::from("/public/ts.svg"),
      String::from("/public/html.svg"),
      String::from("/public/three.svg")
    ],
  },
  Project {
    title: String::from("Perfil Discord"),
    description: String::from("Desenvolvido com Vite.js, Vue.js, Typescript, Discord.py, Tailwindcss, este é um projeto sismples para template de perfil no discord."),
    banner: String::from("/public/profile-discord.png"),
    url: String::from("https://7proxy.netlify.app"),
    icon_tech: vec![
      String::from("/public/vitejs.svg"),
      String::from("/public/vue.svg"),
      String::from("/public/tail.svg"),
      String::from("/public/ts.svg"),
      String::from("/public/html.svg"),
      String::from("/public/discord.svg"),
    ],
  },
  Project {
    title: String::from("API em Phoenix"),
    description: String::from("API simples de cadastro de usuário, desenvolvida com Framework Phoenix, usando o Postgrsql como imagem no Docker."),
    banner: String::from("/public/elixir-backend.png"),
    url: String::from("https://github.com/SevenProxy/API-in-Elixir"),
    icon_tech: vec![
      String::from("/public/tail.svg"),
      String::from("/public/elixir.svg"),
      String::from("/public/phoenix.svg"),
      String::from("/public/insominia.svg"),
      String::from("/public/dock.svg"),
      String::from("/public/postgressql.svg"),
    ],
  }
]);