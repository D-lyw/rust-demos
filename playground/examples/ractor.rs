use ractor::{call_t, Actor, ActorProcessingErr, ActorRef, RpcReplyPort};

/// Ceate Usage
/// https://docs.rs/ractor/latest/ractor/index.html
/// 

enum ActMessage {
    Increment(u32),
    Decrement(u32),
    Retrieve(RpcReplyPort<u32>),
}

#[cfg(feature = "cluster")]
impl ractor::Message for ActMessage {}

#[derive(Debug)]
struct CounterState {
    count: u32,
}

struct Counter;

#[ractor::async_trait]
impl Actor for Counter {
    type Msg = ActMessage;

    type State = CounterState;

    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        println!("Counter Actor starting");
        Ok(CounterState { count: 0 })
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        println!("Handling message...");
        match message {
            ActMessage::Increment(num) => state.count += num,
            ActMessage::Decrement(num) => state.count -= num,
            ActMessage::Retrieve(reply_port) => {
                if !reply_port.is_closed() {
                    let _ = reply_port.send(state.count);
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (actor, handle) = Actor::spawn(Some(String::from("Actor1")), Counter, ()).await?;

    actor.send_message(ActMessage::Increment(5))?;
    actor.send_message(ActMessage::Increment(8))?;
    actor.send_message(ActMessage::Decrement(3))?;

    let rpc_result = call_t!(actor, ActMessage::Retrieve, 100)?;
    println!("RPC result: {}", rpc_result);

    actor.stop(None);
    handle.await?;

    Ok(())
}
