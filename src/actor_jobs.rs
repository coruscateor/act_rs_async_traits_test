use tokio::sync::oneshot::Sender;

#[derive(Default)]
pub enum WorkJob
{

    #[default]
    NoJob,
    DoesItWork

}

/*
#[derive(Default)]
pub enum BigWorkJob
{

    #[default]
    NoJob,
    DoesItWork(Sender<String>),
    InnerDoesItWork(Sender<String>)

}
*/

//Watch for casing:

//#[Default]


