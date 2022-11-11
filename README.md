<div align="center"> <img src="https://raw.githubusercontent.com/cornucopia-rs/cornucopia/main/assets/logo.svg" width=200 /> </div>
<h1 align="center">Cornucopia</h1>
<div align="center">
 <strong>
   Generate type-checked  Rust from your SQL
 </strong>
</div>

<br />

<div align="center">
  <!-- Downloads -->
  <a href="https://crates.io/crates/cornucopia">
    <img src="https://img.shields.io/crates/d/cornucopia.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- Version -->
  <a href="https://crates.io/crates/cornucopia">
    <img src="https://img.shields.io/crates/v/cornucopia.svg?style=flat-square"
    alt="Crates.io version" />
  </a>

  <!-- Book -->
  <a href="https://cornucopia-rs.netlify.app/book/index.html">
  <img src="https://img.shields.io/badge/book-latest-blue?logo=mdbook&style=flat-square" alt="book">
  </a>

  <!-- Docs -->
  <a href="https://docs.rs/cornucopia/latest/cornucopia/">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/cornucopia?style=flat-square">
  </a>
  
  <!-- Dependencies -->
  <a href="https://deps.rs/repo/github/cornucopia-rs/cornucopia">
    <img src="https://deps.rs/repo/github/cornucopia-rs/cornucopia/status.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
 
  <br/>

  <!-- License -->
  <a href="https://github.com/cornucopia-rs/cornucopia#License">
    <img src="https://img.shields.io/badge/License-APACHE--2.0%2FMIT-blue?style=flat-square" alt="License">
  </a>

  <!-- Chat -->
  <a href="https://discord.gg/nYwUmQDHBZ">
    <img src="https://img.shields.io/discord/987088069280825401?label=chat&logo=discord&style=flat-square" alt="Chat">
  </a>
</div>

<div align="center">
  <h4>
    <a href="https://cornucopia-rs.netlify.app/">
      Homepage
    </a>
    <span> | </span>
    <a href="examples/basic_async/README.md">
      Example
    </a>
  </h4>
</div>

<br />

Contribua com o projeto [Cornucopia](https://github.com/cornucopia-rs/cornucopia)

Este repositório contém um exemplo básico de Cornucopia, em Português, acessando uma base Postgres Local.  
Antes de iniciar o projeto, você deve configurar a sua instância postgres para rodar localmente. Criar um banco de dados e dentro dele as tabelas que você possui.
A minha recomendação é usar uma base de dados de teste que você já possua.

## Configurando a base de dados

```sql
create database proto
```

## Criando a tabela cidade

```sql
-- DROP TABLE public.cidade;

CREATE TABLE public.cidade (
	id serial NOT NULL,
	nome varchar NULL,
	uf varchar NULL
);
```

# Gerando o projeto
Cornucopia é um CLI que irá gerar código RUST para você. Ele irá fazer isso a partir de queries SQL, por isso você precisa criar uma pasta \queries\ e dentro dela colocar 
os arquivos com o SQL que você quer usar. 

**\queries\sample.sql**
```sql
--! cidade
select * from cidade;

--! inserir_cidade
insert into cidade values (:id, :nome, :uf);
```

o arquivo sample.sql deve conter o mesmo nome do módulo que você deseja criar. 
dentro de sample.sql, cada comentário `--! cidade --!inserir_cidade` corresponde a um recurso deste módulo. 
Cornucopia irá criar toda a infraestrutura de binding para cada instrução sql. Neste exemplo eu demonstro tanto uma instrução select quanto um insert.

Acesse o livro [book](https://cornucopia-rs.netlify.app/book/index.html), or you can get a quickstart by looking at our [examples](https://cornucopia-rs.netlify.app/book/examples.html).

Para gerar os fontes rode
```shell
$ cornucopia live postgres://{usr}:{psw}@localhost:5432/{database}"
```

A partir disso, você pode consumir os fontes usados como neste projeto main.rs
As dependências cargo.toml foram configuradas com base no exemplo async

**cargo.toml**
```toml
[dependencies]
tokio = { version = "1.21.2", features = ["full"] }
deadpool-postgres = "0.10.3"
futures = "0.3.25"
postgres-types = { version = "0.2.4", features = ["derive"] }
tokio-postgres = "0.7.7"
cornucopia_async = { version = "*", features = ["with-serde_json-1"] }
```

**main.rs**
```rust
mod cornucopia;

use crate::cornucopia::queries::sample::{cidade, inserir_cidade};

#[tokio::main]
pub async fn main() {
    
    let pool = create_pool().await.unwrap();
    let mut client = pool.get().await.unwrap();
    
 {
      let mut transaction = client.transaction().await.unwrap();
   
       inserir_cidade()
        .bind(&mut transaction, &99999, &"Belzonte", &"MG")
        .await
        .unwrap();

            
        let cidade = cidade().bind(&transaction)
        //.map(|book_title| book_title.to_uppercase())
        .all()
        .await
        .unwrap();
        
        dbg!(cidade);
        transaction.commit().await.unwrap();
    }

use deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use tokio_postgres::NoTls;

async fn create_pool() -> Result<Pool, CreatePoolError> {
    let mut cfg = Config::new();
    cfg.user = Some(String::from("postgres"));
    cfg.password = Some(String::from("masterkey"));
    cfg.host = Some(String::from("127.0.0.1"));
    cfg.port = Some(5432);
    cfg.dbname = Some(String::from("proto"));
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
}

```

Recursos disponíveis neste exemplo
- [x] schema
- [x] Async
- [x] pool de conexões
- [x] postgres local
- [x] transações
- [x] select
- [x] insert
