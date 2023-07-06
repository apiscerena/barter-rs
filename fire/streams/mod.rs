pub mod orderbooks;
pub mod trades;

use barter_data::{
    event::{DataKind, MarketEvent},
    exchange::ExchangeId,
};
use tokio::sync::mpsc;


pub async fn stream_market_event() -> mpsc::UnboundedReceiver<MarketEvent<DataKind>> {

    // Select the ExchangeId::BinanceSpot stream
    // Notes:
    //  - Use `streams.select(ExchangeId)` to interact with the individual exchange streams!
    //  - Use `streams.join()` to join all exchange streams into a single mpsc::UnboundedReceiver!

    
    let mut trade_streams = trades::stream_market_event_trades().await;
    let mut orderbook_streams = orderbooks::stream_market_event_orderbooks().await;


    //let mut trade_rx = trade_streams.select(ExchangeId::BinanceSpot).unwrap();
    //let mut orderbook_rx = orderbook_streams.select(ExchangeId::BinanceSpot).unwrap();
    let mut merge_rx = orderbook_streams.select(ExchangeId::BinanceSpot).unwrap();

    let (tx, rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        // while let Some(trade) = trade_rx.recv().await {
        //     let _ = tx.send(MarketEvent::from(trade));
        // }
        while let Some(orderbook) = merge_rx.recv().await {
            let _ = tx.send(MarketEvent::from(orderbook));
        }
    });
    rx
}