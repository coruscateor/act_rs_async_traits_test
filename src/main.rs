mod it_works_runtime_task_actor;

use act_rs::ActorFrontend;

pub use it_works_runtime_task_actor::*;

mod actor_jobs;

pub use actor_jobs::*;

mod it_works_task_actor;

pub use it_works_task_actor::*;

mod it_works_mac_runtime_task_actor;

pub use it_works_mac_runtime_task_actor::*;

mod it_works_mac_task_actor;

pub use it_works_mac_task_actor::*;

use tokio::runtime::{Builder, Runtime};

use tokio::sync::oneshot::{Sender, Receiver, channel};

fn main()
{
    
    let tokio_runtime = Builder::new_multi_thread().enable_all().build().expect("Tokio Runtime construction failed");

    println!("{}\n", "Async Trait Oriented Actors");

    //Initialise the actor state

    let state = ItWorksRuntimeTaskActorState::new();

    //Create the actor

    let runtime_task_actor = ItWorksRuntimeTaskActor::from_runtime(&tokio_runtime, state);

    //Setup the oneshot channel for the WorkJob::DoesItWork message.

    let (sender, receiver) = channel();

    //send the job message via the actors interactor.

    let res = runtime_task_actor.interactor().sender().blocking_send(BigWorkJob::DoesItWork(sender));

    res.expect("Error: This didn't work.");

    //Wait fot the result of the job and print it to the console.

    let does_it_work = receiver.blocking_recv().expect("Error: Sender Error");

    println!("{}\n", does_it_work);

    //Big Work

    println!("{}\n", "Big Work");

    //new Oneshot channel 

    let (sender, receiver) = channel();

    let res = runtime_task_actor.interactor().sender().blocking_send(BigWorkJob::InnerDoesItWork(sender));

    res.expect("Error: This didn't work.");

    let inner_does_it_work = receiver.blocking_recv().expect("Error: Sender Error");

    println!("{}\n", inner_does_it_work);

    println!("{}\n", "Macro Generated Actors");

    //like above:

    let state = ItWorksMacRuntimeTaskActorState::new();

    let mac_runtime_task_actor = ItWorksMacRuntimeTaskActor::from_runtime(&tokio_runtime, state);

    //Initialise a single-shot channel

    let (sender, receiver) = channel();

    let res = mac_runtime_task_actor.interactor().sender().blocking_send(BigWorkJob::DoesItWork(sender));

    res.expect("Error: This didn't work.");

    let does_it_work = receiver.blocking_recv().expect("Error: Sender Error");

    println!("{}\n", does_it_work);

    //Big Work

    println!("{}\n", "Big Work");

    let (sender, receiver) = channel();

    let res = mac_runtime_task_actor.interactor().sender().blocking_send(BigWorkJob::InnerDoesItWork(sender));

    res.expect("Error: This didn't work.");

    let inner_does_it_work = receiver.blocking_recv().expect("Error: Sender Error");

    println!("{}\n", inner_does_it_work);

}


