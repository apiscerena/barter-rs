use barter_data::{
    event::{DataKind, MarketEvent},
    exchange::binance::spot::BinanceSpot,
    streams::Streams,
    subscription::book::OrderBooksL1,
};
use barter_integration::model::{instrument::kind::InstrumentKind};

pub async fn stream_market_event_orderbooks() -> Streams<MarketEvent<DataKind>> {
    // Initialise PublicTrades Streams for BinanceSpot
    // '--> each call to StreamBuilder::subscribe() creates a separate WebSocket connection
    Streams::builder_multi()
    .add(Streams::<OrderBooksL1>::builder()
            .subscribe([
                (BinanceSpot::default(), "btc", "usdt", InstrumentKind::Spot, OrderBooksL1),
         ])
     )
    .init()
    .await
    .unwrap()
}