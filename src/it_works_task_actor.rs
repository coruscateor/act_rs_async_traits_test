use tokio::sync::mpsc::Receiver;

use act_rs::{tokio::{interactors::mpsc::{channel, SenderInteractor}, TaskActor}, ActorFrontend, ActorInteractor, AsyncActorState, DroppedIndicator, HasInteractor};

use async_trait::async_trait;

use crate::WorkJob;

pub struct ItWorksTaskActorState
{

    sender: SenderInteractor<WorkJob>,
    reciver: Receiver<WorkJob>

}

impl ItWorksTaskActorState
{

    pub fn new() -> Self
    {

        let (sender, reciver) = channel(5);

        Self
        {

            sender,
            reciver

        }

    }

}

impl HasInteractor<SenderInteractor<WorkJob>> for ItWorksTaskActorState
{

    fn interactor(&self) -> &SenderInteractor<WorkJob>
    {

        &self.sender
        
    }

}

#[async_trait]
impl AsyncActorState<SenderInteractor<WorkJob>> for ItWorksTaskActorState
{

    async fn run_async(&mut self, di: &DroppedIndicator) -> bool
    {

        if let Some(res) = self.reciver.recv().await
        {

            match res
            {

                WorkJob::NoJob => {},
                WorkJob::DoesItWork(sender) =>
                {

                    let _ = sender.send("Inner: It Works!".to_string());

                }
                
            }

        }

        di.not_dropped()

    }

}

pub type ItWorksTaskActor = TaskActor<ItWorksTaskActorState, SenderInteractor<WorkJob>>;


