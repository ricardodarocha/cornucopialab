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

    //let all = cidade().bind(&client).all();
    //dbg!(all);
    //for cidade in all {
        // println!(
        //     "[{}] {}, {}",
        //     cidade.id,
        //     cidade.name.to_uppercase(),
        //     cidade.uf.to_uppercase()
        // )
    }

    // let author_name = author_name_by_id().bind(&client, &0).opt().await.unwrap();
    // dbg!(author_name);

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


