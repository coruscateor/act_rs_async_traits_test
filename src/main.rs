//mod it_works_runtime_task_actor;

//use act_rs::ActorFrontend;

//pub use it_works_runtime_task_actor::*;

mod actor_jobs;

pub use actor_jobs::*;

mod it_works_task_actor;

pub use it_works_task_actor::*;

//mod it_works_mac_runtime_task_actor;

//pub use it_works_mac_runtime_task_actor::*;

mod it_works_mac_task_actor;

pub use it_works_mac_task_actor::*;

use tokio::runtime::{Builder, Runtime};

//use tokio::sync::oneshot::{Sender, Receiver, channel};

//use act_rs::tokio::io::mpsc::{UnboundedActorIOClient, UnboundedActorIOServer, unbounded_actor_io};

//use act_rs::enter;

#[tokio::main]
async fn main()
{
    
    //let tokio_runtime = Builder::new_multi_thread().enable_all().build().expect("Tokio Runtime construction failed");

    println!("ItWorksMacTaskActor");

    let actor_io_client = ItWorksMacTaskActorState::spawn(); //enter!{tokio_runtime, ItWorksMacTaskActorState::spawn() };

    actor_io_client.input_sender_ref().send(WorkJob::DoesItWork).expect("Error: Sender Error");

    let res = actor_io_client.output_receiver_ref().recv().await.expect("Error: Receiver Error"); //.expect("Error: Output Receiver Lock Error").blocking_recv().expect("Error: Receiver Error");

    println!("{}\n", res);

    println!("ItWorksTaskActor");

    let actor_io_client = ItWorksTaskActorState::spawn(); //enter!{tokio_runtime, ItWorksTaskActorState::spawn()};

    actor_io_client.input_sender_ref().send(WorkJob::DoesItWork).expect("Error: Sender Error");

    let res = actor_io_client.output_receiver_ref().recv().await.expect("Error: Receiver Error"); //("Error: Output Receiver Lock Error").blocking_recv().expect("Error: Receiver Error");

    println!("{}\n", res);
    
}


