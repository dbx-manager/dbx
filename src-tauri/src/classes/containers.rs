use podman_api::models::ListContainer;
use podman_api::opts::{ ContainerListOpts};
use podman_api::Podman;
use std::sync::Arc;
use tokio::sync::OnceCell;
use crate::classes::socket::PodmanSocket;


pub struct Containers {
    //ListContainer represent a single contianer ("I know the name is missleading, that's how podman-api named it :> ")
    pub data: Vec<ListContainer>,
}

static SINGLETON: OnceCell<Arc<Containers>> = OnceCell::const_new();

impl Containers {
    async fn init() -> Arc<Containers> {
        let mut data = Vec::new();
        let fetched_item = self_fetch_data_async().await;
        data = fetched_item.clone();
        Arc::new(Containers { data })
    }

    pub async fn get_instance() -> Arc<Containers> {
        SINGLETON.get_or_init(Self::init).await.clone()
    }
}

// TODO: add an auto checker if podman does exsist and automaticly run podman system service --time=0 unix:///tmp/podman.sock on app startup 
//this is the "constructor" for the singleton
async fn self_fetch_data_async() -> Vec<ListContainer> {
    //keep the socket in /tmp/podman.sock so if the app was ran in the contaienr,it can use the host podman and not get lost
    let podman =PodmanSocket::get_instance().await.socket.clone();

    //stolen from the api as is :>
    return podman
        .containers()
        .list(
            &ContainerListOpts::builder()
                .all(true)
                .build(),
        )
        .await
        .unwrap();
}

