pub mod orderbooks;
pub mod trades;

use barter_data::{
    event::{DataKind, MarketEvent},
};
use tokio::sync::mpsc;


pub async fn stream_market_event() -> mpsc::UnboundedReceiver<MarketEvent<DataKind>> {

    // Select the ExchangeId::BinanceSpot stream
    // Notes:
    //  - Use `streams.select(ExchangeId)` to interact with the individual exchange streams!
    //  - Use `streams.join()` to join all exchange streams into a single mpsc::UnboundedReceiver!

    
    let trade_streams = trades::stream_market_event_trades().await;
    let orderbook_streams = orderbooks::stream_market_event_orderbooks().await;
    let mut merge_rx = trade_streams.merge_streams(orderbook_streams).await.join().await;

    let (tx, rx) = mpsc::unbounded_channel();
    tokio::spawn(async move {
        while let Some(orderbook) = merge_rx.recv().await {
            let _ = tx.send(MarketEvent::from(orderbook));
        }
    });
    rx
}