use crate::utils::file_reader;
use crate::xml::*;
use async_graphql::{Context, FieldResult, Schema, SimpleBroker, ID};
use futures::lock::Mutex;
use futures::{Stream, StreamExt};
use slab::Slab;
use std::sync::Arc;
use std::time::Duration;

pub type BooksSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;
// pub struct Application {
//     pub id: String,
//     pub name: Vec<Input>,
//     pub pkgname: String,
//     pub summaries: Vec<Input>,
//     pub descriptions: Vec<Description>,
//     pub screenshots: Vec<ScreenShot>,
//     pub categories: Vec<String>,
//     pub icons: Vec<Icon>,
//     pub launchable: Launchable,
//     pub mimetypes: Vec<String>,
//     pub urls: Vec<Url>,
//     pub keywords: Vec<Keyword>,
//     pub releases: Vec<Release>,
//     pub provides: Vec<Provide>,
//     pub languages: Vec<Language>,
//     pub project_licenses: Vec<String>,
//     pub developer_name: Vec<Input>,
//     pub project_group: Vec<String>,
// }
#[async_graphql::Object]
impl Application {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &Vec<Input> {
        &self.name
    }

    async fn pkgname(&self) -> &str {
        &self.pkgname
    }
    async fn summaries(&self) -> &Vec<Input> {
        &self.summaries
    }
    async fn descriptions(&self) -> &Vec<Description> {
        &self.descriptions
    }
    async fn screenshots(&self) -> &Vec<ScreenShot> {
        &self.screenshots
    }
    async fn categories(&self) -> &Vec<String> {
        &self.categories
    }
    async fn icons(&self) -> &Vec<Icon> {
        &self.icons
    }
    async fn launchable(&self) -> &Launchable {
        &self.launchable
    }
    async fn mimetypes(&self) -> &Vec<String> {
        &self.mimetypes
    }
    async fn urls(&self) -> &Vec<Url> {
        &self.urls
    }
    async fn keywords(&self) -> &Vec<Keyword> {
        &self.keywords
    }
    async fn releases(&self) -> &Vec<Release> {
        &self.releases
    }
    async fn provides(&self) -> &Vec<Provide> {
        &self.provides
    }
    async fn languages(&self) -> &Vec<Language> {
        &self.languages
    }
    async fn project_licenses(&self) -> &Vec<String> {
        &self.project_licenses
    }
    async fn developer_name(&self) -> &Vec<Input> {
        &self.developer_name
    }
    async fn project_group(&self) -> &Vec<String> {
        &self.project_group
    }
}

pub type Storage = Arc<Mutex<Slab<Application>>>;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn books(&self, ctx: &Context<'_>) -> Vec<Application> {
        // let books = ctx.data_unchecked::<Storage>().lock().await;
        // books.iter().map(|(_, book)| book).cloned().collect()
        let data = file_reader("/home/brilliant/Documents/projects/pkg-dist/development/discover/output/community.json");
        let store: Store = serde_json::from_str(&data).unwrap();
        store.applications
    }

    async fn book(
        &self,
        ctx: &Context<'_>,
        #[arg(desc = "Name of object")] pkgname: String,
    ) -> Application {
        let data = file_reader("/home/brilliant/Documents/projects/pkg-dist/development/discover/output/community.json");
        let store: Store = serde_json::from_str(&data).unwrap();
        let mut result: Application = Application::default();
        for app in store.applications.iter() {
            if app.pkgname == pkgname {
                result = app.clone()
            }
        }

        result
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn create_book(&self, ctx: &Context<'_>, name: String, author: String) -> ID {
        let mut books = ctx.data_unchecked::<Storage>().lock().await;
        let entry = books.vacant_entry();
        let id: ID = entry.key().into();
        // let book = Application {
        //     id: id.clone().to_string(),
        //     // name,
        //     // author,
        // };
        let book: Application = Application::default();
        entry.insert(book);
        SimpleBroker::publish(BookChanged {
            mutation_type: MutationType::Created,
            id: id.clone(),
        });
        id
    }

    async fn delete_book(&self, ctx: &Context<'_>, id: ID) -> FieldResult<bool> {
        let mut books = ctx.data_unchecked::<Storage>().lock().await;
        let id = id.parse::<usize>()?;
        if books.contains(id) {
            books.remove(id);
            SimpleBroker::publish(BookChanged {
                mutation_type: MutationType::Deleted,
                id: id.into(),
            });
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[async_graphql::Enum]
enum MutationType {
    Created,
    Deleted,
}

#[async_graphql::SimpleObject]
#[derive(Clone)]
struct BookChanged {
    mutation_type: MutationType,
    id: ID,
}

pub struct SubscriptionRoot;

#[async_graphql::Subscription]
impl SubscriptionRoot {
    async fn interval(&self, #[arg(default = 1)] n: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        tokio::time::interval(Duration::from_secs(1)).map(move |_| {
            value += n;
            value
        })
    }

    async fn books(&self, mutation_type: Option<MutationType>) -> impl Stream<Item = BookChanged> {
        SimpleBroker::<BookChanged>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            };
            async move { res }
        })
    }
}
