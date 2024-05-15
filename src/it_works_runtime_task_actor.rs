use tokio::sync::mpsc::Receiver;

use act_rs::{tokio::{interactors::mpsc::{channel, SenderInteractor}, RuntimeTaskActor}, ActorFrontend, ActorInteractor, AsyncActorState, DroppedIndicator, HasInteractor};

use async_trait::async_trait;

use crate::{BigWorkJob, WorkJob, ItWorksTaskActorState, ItWorksTaskActor};

use corlib::{Invalid, NonOption};

pub struct ItWorksRuntimeTaskActorState
{

    sender: SenderInteractor<BigWorkJob>, //<Option<String>>,
    reciver: Receiver<BigWorkJob>, //<Option<String>>
    no_inner_actor: NonOption<ItWorksTaskActor>
}

impl ItWorksRuntimeTaskActorState
{

    pub fn new() -> Self
    {

        let (sender, reciver) = channel(5);

        //Setup the inner_actor

        //let inner_actor_state = ItWorksTaskActorState::new();

        //let inner_actor = ItWorksTaskActor::new(inner_actor_state);

        Self
        {

            sender,
            reciver,
            no_inner_actor: NonOption::invalid()

        }

    }

}

impl HasInteractor<SenderInteractor<BigWorkJob>> for ItWorksRuntimeTaskActorState
{

    fn interactor(&self) -> &SenderInteractor<BigWorkJob> //<Option<String>>
    {

        &self.sender //.clone()
        
    }

}

#[async_trait]
impl AsyncActorState<SenderInteractor<BigWorkJob>> for ItWorksRuntimeTaskActorState
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

                    //let res = self.inner_actor.interactor().sender().send(WorkJob::DoesItWork(sender)).await;

                    let res = self.no_inner_actor.get_ref().interactor().sender().send(WorkJob::DoesItWork(sender)).await;

                    res.expect("Error: This BigWorkJob::InnerDoesItWork didn't work.");

                }

            }

        }

        di.not_dropped()

    }

}

pub type ItWorksRuntimeTaskActor = RuntimeTaskActor<ItWorksRuntimeTaskActorState, SenderInteractor<BigWorkJob>>;

/*
impl AsyncActorState<SenderInteractor<Option<String>>> for ItWorksRuntimeTaskActorState
{
    #[must_use]
#[allow(clippy::type_complexity,clippy::type_repetition_in_bounds)]
fn run_async<'life0,'life1,'async_trait>(&'life0 mut self,di: &'life1 DroppedIndicator) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = bool> + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
        todo!()
    }
    
    #[must_use]
    #[allow(clippy::async_yields_async,clippy::diverging_sub_expression,clippy::let_unit_value,clippy::no_effect_underscore_binding,clippy::shadow_same,clippy::type_complexity,clippy::type_repetition_in_bounds,clippy::used_underscore_binding)]
    fn on_enter_async<'life0,'life1,'async_trait>(&'life0 mut self,_di: &'life1 DroppedIndicator) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = bool> + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,Self: ::core::marker::Send+'async_trait{
    Box::pin(async move {
        if let::core::option::Option::Some(__ret) =  ::core::option::Option::None:: <bool>{
            #[allow(unreachable_code)]
            return __ret;
        }let mut __self = self;
        let __ret:bool = {
            true
        };
        #[allow(unreachable_code)]
        __ret
    })
    }
    
    #[must_use]
    #[allow(clippy::async_yields_async,clippy::diverging_sub_expression,clippy::let_unit_value,clippy::no_effect_underscore_binding,clippy::shadow_same,clippy::type_complexity,clippy::type_repetition_in_bounds,clippy::used_underscore_binding)]
    fn on_exit_async<'life0,'life1,'async_trait>(&'life0 mut self,_di: &'life1 DroppedIndicator) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,Self: ::core::marker::Send+'async_trait{
    Box::pin(async move {
        let mut __self = self;
        let() = {}
        ;
    })
    }
}
*/


