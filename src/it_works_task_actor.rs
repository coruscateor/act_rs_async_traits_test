//use tokio::sync::mpsc::Receiver;

//use act_rs::{impl_mac_task_actor, tokio::{io::mpsc::{unbounded_actor_io, UnboundedActorIOClient, UnboundedActorIOServer}, TaskActor}, ActorFrontend, AsyncActorState};

use act_rs::{impl_mac_task_actor, ActorStateAsync};

use async_trait::async_trait;

use crate::WorkJob;

use libsync::crossbeam::mpmc::tokio::seg_queue::{Sender, Receiver, io_channels::{IOClient, IOServer, io_channels}};

pub struct ItWorksTaskActorState
{

    actor_io_server: IOServer<WorkJob, String>

    //actor_io_server: UnboundedActorIOServer<WorkJob, String>

}

impl ItWorksTaskActorState
{

    pub fn new(actor_io_server: IOServer<WorkJob, String>) -> Self //UnboundedActorIOServer<WorkJob, String>) -> Self
    {

        Self
        {

            actor_io_server

        }

    }

    pub fn spawn() -> UnboundedActorIOClient<WorkJob, String>
    {

        let (actor_io_client, actor_io_server) = unbounded_actor_io();

        TaskActor::spawn(ItWorksTaskActorState::new(actor_io_server));

        actor_io_client

    }

}

#[async_trait]
impl AsyncActorState for ItWorksTaskActorState
{

    async fn run_async(&mut self) -> bool
    {

        if let Some(res) = self.actor_io_server.input_receiver().recv().await
        {

            match res
            {

                WorkJob::NoJob => {},
                WorkJob::DoesItWork => //(sender) =>
                {

                    let _ = self.actor_io_server.output_sender().send("It Works!".to_string()); //sender.send("Inner: It Works!".to_string());

                }
                
            }

            return true;

        }

        false

    }

}
