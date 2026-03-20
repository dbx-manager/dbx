use podman_api::Podman;
use std::sync::Arc;
use tokio::sync::OnceCell;

pub struct PodmanSocket {
    //ListContainer represent a single contianer ("I know the name is missleading, that's how podman-api named it :> ")
    pub socket:Podman,
}

static SOCKET: OnceCell<Arc<PodmanSocket>> = OnceCell::const_new();

impl PodmanSocket {
    async fn init() -> Arc<PodmanSocket> {
        let data = self_fetch_data_async().await;
        Arc::new(PodmanSocket { socket: data })
    }

    pub async fn get_instance() -> Arc<PodmanSocket> {
        SOCKET.get_or_init(Self::init).await.clone()
    }
}

// TODO: add an auto checker if podman does exsist and automaticly run podman system service --time=0 unix:///tmp/podman.sock on app startup 
//this is the "constructor" for the singleton
async fn self_fetch_data_async() -> Podman{
    //keep the socket in /tmp/podman.sock so if the app was ran in the contaienr,it can use the host podman and not get lost
    Podman::unix("/tmp/podman.sock")
}

