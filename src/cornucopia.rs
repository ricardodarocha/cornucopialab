// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types {}
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod queries {
    pub mod sample {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct InserirCidadeParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub id: i32,
            pub nome: T1,
            pub uf: T2,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Cidade {
            pub id: i32,
            pub nome: String,
            pub uf: String,
        }
        pub struct CidadeBorrowed<'a> {
            pub id: i32,
            pub nome: &'a str,
            pub uf: &'a str,
        }
        impl<'a> From<CidadeBorrowed<'a>> for Cidade {
            fn from(CidadeBorrowed { id, nome, uf }: CidadeBorrowed<'a>) -> Self {
                Self {
                    id,
                    nome: nome.into(),
                    uf: uf.into(),
                }
            }
        }
        pub struct CidadeQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> CidadeBorrowed,
            mapper: fn(CidadeBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> CidadeQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(CidadeBorrowed) -> R) -> CidadeQuery<'a, C, R, N> {
                CidadeQuery {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                let row = self.client.query_one(stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn cidade() -> CidadeStmt {
            CidadeStmt(cornucopia_async::private::Stmt::new("select * from cidade"))
        }
        pub struct CidadeStmt(cornucopia_async::private::Stmt);
        impl CidadeStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> CidadeQuery<'a, C, Cidade, 0> {
                CidadeQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| CidadeBorrowed {
                        id: row.get(0),
                        nome: row.get(1),
                        uf: row.get(2),
                    },
                    mapper: |it| <Cidade>::from(it),
                }
            }
        }
        pub fn inserir_cidade() -> InserirCidadeStmt {
            InserirCidadeStmt(cornucopia_async::private::Stmt::new(
                "insert into cidade values ($1, $2, $3)",
            ))
        }
        pub struct InserirCidadeStmt(cornucopia_async::private::Stmt);
        impl InserirCidadeStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                id: &'a i32,
                nome: &'a T1,
                uf: &'a T2,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[id, nome, uf]).await
            }
        }
        impl<
                'a,
                C: GenericClient + Send + Sync,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                InserirCidadeParams<T1, T2>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                >,
                C,
            > for InserirCidadeStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InserirCidadeParams<T1, T2>,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            > {
                Box::pin(self.bind(client, &params.id, &params.nome, &params.uf))
            }
        }
    }
}
