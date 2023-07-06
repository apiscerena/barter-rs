use barter_data::{
    event::{DataKind, MarketEvent},
    exchange::binance::spot::BinanceSpot,
    streams::Streams,
    subscription::trade::PublicTrades,
};
use barter_integration::model::{instrument::kind::InstrumentKind};


pub async fn stream_market_event_trades() -> Streams<MarketEvent<DataKind>> {
    // Initialise PublicTrades Streams for BinanceSpot
    // '--> each call to StreamBuilder::subscribe() creates a separate WebSocket connection
    Streams::builder_multi()
    .add(Streams::<PublicTrades>::builder()
        // Separate WebSocket connection for BTC_USDT stream since it's very high volume
        .subscribe([(
            BinanceSpot::default(),
            "btc",
            "usdt",
            InstrumentKind::Spot,
            PublicTrades,
        )])
        // Separate WebSocket connection for ETH_USDT stream since it's very high volume
        .subscribe([(
            BinanceSpot::default(),
            "eth",
            "usdt",
            InstrumentKind::Spot,
            PublicTrades,
        )])
        .subscribe([(
            BinanceSpot::default(),
            "ar",
            "usdt",
            InstrumentKind::Spot,
            PublicTrades,
        )])
    )
    .init()
    .await
    .unwrap()
}