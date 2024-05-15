use tokio::sync::mpsc::Receiver;

use act_rs::{impl_mac_runtime_task_actor, tokio::{interactors::mpsc::{channel, SenderInteractor}, RuntimeTaskActor}, ActorFrontend, ActorInteractor, AsyncActorState, DroppedIndicator, HasInteractor};

use async_trait::async_trait;

use crate::{BigWorkJob, WorkJob, ItWorksTaskActorState, ItWorksTaskActor};

use corlib::NonOption;

use std::sync::Arc;

use tokio::runtime::{Runtime, Handle};

pub struct ItWorksMacRuntimeTaskActorState
{

    sender: SenderInteractor<BigWorkJob>,
    reciver: Receiver<BigWorkJob>,
    no_inner_actor: NonOption<ItWorksTaskActor>
}

impl ItWorksMacRuntimeTaskActorState
{

    pub fn new() -> Self
    {

        let (sender, reciver) = channel(5);

        Self
        {

            sender,
            reciver,
            no_inner_actor: NonOption::invalid()

        }

    }

}

impl HasInteractor<SenderInteractor<BigWorkJob>> for ItWorksMacRuntimeTaskActorState
{

    fn interactor(&self) -> &SenderInteractor<BigWorkJob>
    {

        &self.sender
        
    }

}

#[async_trait]
impl AsyncActorState<SenderInteractor<BigWorkJob>> for ItWorksMacRuntimeTaskActorState
{

    async fn on_enter_async(&mut self, di: &DroppedIndicator) -> bool
    {

        let inner_actor_state = ItWorksTaskActorState::new();

        self.no_inner_actor.set(ItWorksTaskActor::new(inner_actor_state));

        di.not_dropped()

    }

    async fn run_async(&mut self, di: &DroppedIndicator) -> bool
    {

        if let Some(res) = self.reciver.recv().await
        {

            match res
            {
                
                BigWorkJob::NoJob => {},
                BigWorkJob::DoesItWork(sender) =>
                {

                    //Send the response

                    let _ = sender.send("It Works!".to_string());

                },
                BigWorkJob::InnerDoesItWork(sender) =>
                {

                    //Pass the sender on to the task actor.

                    let res = self.no_inner_actor.get_ref().interactor().sender().send(WorkJob::DoesItWork(sender)).await;

                    res.expect("Error: This BigWorkJob::InnerDoesItWork didn't work.");

                }

            }

        }

        di.not_dropped()

    }

}

//pub type ItWorksRuntimeMacTaskActor = RuntimeTaskActor<ItWorksMacRuntimeTaskActorState, SenderInteractor<BigWorkJob>>;

impl_mac_runtime_task_actor!(ItWorksMacRuntimeTaskActorState, SenderInteractor<BigWorkJob>, ItWorksMacRuntimeTaskActor);