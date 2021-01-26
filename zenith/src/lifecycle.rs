use tokio::sync::watch;

#[derive(Clone, Copy, Debug)]
pub enum LifecycleState {
    Started,
    Stopped,
}

pub struct AppLifecycle {
    sender: watch::Sender<LifecycleState>,
    receiver: watch::Receiver<LifecycleState>,
}

impl AppLifecycle {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (sender, receiver) = watch::channel(LifecycleState::Started);
        AppLifecycle { sender, receiver }
    }

    pub fn signal_stopped(&self) -> eyre::Result<()> {
        self.sender
            .send(LifecycleState::Stopped)
            .map_err(|e| e.into())
    }

    pub fn on_stopped(&self, f: impl std::future::Future<Output = ()> + Send + 'static) {
        let on_stopped = self.wait_stopped();

        tokio::spawn(async move {
            on_stopped.await;
            f.await;
        });
    }

    pub fn wait_stopped(&self) -> impl std::future::Future<Output = ()> {
        let mut rec = self.receiver.clone();
        async move {
            while rec.changed().await.is_ok() {
                let val = *rec.borrow();
                if matches!(val, LifecycleState::Stopped) {
                    break;
                }
            }
        }
    }
}
