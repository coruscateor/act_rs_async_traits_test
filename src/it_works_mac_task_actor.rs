//use tokio::sync::mpsc::Receiver;

//use act_rs::{impl_mac_task_actor, tokio::io::mpsc::{UnboundedActorIOClient, UnboundedActorIOServer, unbounded_actor_io}, ActorFrontend, AsyncActorState};

use act_rs::{impl_mac_task_actor, ActorStateAsync};

use async_trait::async_trait;

use crate::{WorkJob, ItWorksTaskActorState}; //, ItWorksTaskActor};

use corlib::{Invalid, NonOption};

use std::sync::Arc;

use tokio::runtime::{Runtime, Handle};

use paste::paste;

use tokio::task::JoinHandle;

use libsync::crossbeam::mpmc::tokio::seg_queue::{Sender, Receiver, io_channels::{IOClient, IOServer, io_channels}};

pub struct ItWorksMacTaskActorState
{

    actor_io_server: IOServer<WorkJob, String>

    //actor_io_server: UnboundedActorIOServer<WorkJob, String>

}

impl ItWorksMacTaskActorState
{

    pub fn new(actor_io_server: IOServer<WorkJob, String>) -> Self //UnboundedActorIOServer<WorkJob, String>) -> Self
    {

        Self
        {

            actor_io_server

        }

    }

    pub fn spawn() -> IOClient<WorkJob, String> //UnboundedActorIOClient<WorkJob, String>
    {

        let (actor_io_client, actor_io_server) = io_channels(); //unbounded_actor_io();

        ItWorksMacTaskActor::spawn(ItWorksMacTaskActorState::new(actor_io_server));

        actor_io_client

    }

}

#[async_trait]
impl ActorStateAsync for ItWorksMacTaskActorState //AsyncActorState
{

    async fn run_async(&mut self) -> bool
    {

        if let Some(res) = self.actor_io_server.input_receiver().recv().await
        {

            match res
            {
                
                WorkJob::NoJob => {},
                WorkJob::DoesItWork => //sender) =>
                {

                    let _ = self.actor_io_server.output_sender().send("It Works!".to_string());  //sender.send("Inner: It Works!".to_string());

                }

                /* 
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
                */

            }

            return true;

        }

        false

    }

}

impl_mac_task_actor!(ItWorksMacTaskActor);
