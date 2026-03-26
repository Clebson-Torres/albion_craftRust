use albion_crafting_overlay::api::client::AlbionApiClient;
use albion_crafting_overlay::api::dto::PriceDto;
use albion_crafting_overlay::api::mapper::map_price_dto;
use chrono::{TimeZone, Utc};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn client_fetches_price_rows() {
    let server = MockServer::start().await;
    let response = ResponseTemplate::new(200).set_body_raw(
        r#"[{
            "item_id":"T4_BAG",
            "city":"Caerleon",
            "quality":1,
            "sell_price_min":5277,
            "sell_price_min_date":"2026-03-26T05:15:00",
            "sell_price_max":5654,
            "sell_price_max_date":"2026-03-26T05:15:00",
            "buy_price_min":0,
            "buy_price_min_date":"0001-01-01T00:00:00",
            "buy_price_max":0,
            "buy_price_max_date":"0001-01-01T00:00:00"
        }]"#,
        "application/json",
    );

    Mock::given(method("GET"))
        .and(path("/api/v2/stats/prices/T4_BAG.json"))
        .respond_with(response)
        .mount(&server)
        .await;

    let client = AlbionApiClient::new(server.uri());
    let result = client
        .fetch_prices(&["T4_BAG".to_owned()], &["Caerleon".to_owned()], &[1])
        .await
        .expect("prices");

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].item_id, "T4_BAG");
}

#[test]
fn mapper_converts_price_dto_into_market_snapshot() {
    let dto = PriceDto {
        item_id: "T4_BAG".to_owned(),
        city: "Caerleon".to_owned(),
        quality: 1,
        sell_price_min: 5277,
        sell_price_min_date: "2026-03-26T05:15:00".to_owned(),
        sell_price_max: 5654,
        sell_price_max_date: "2026-03-26T05:15:00".to_owned(),
        buy_price_min: 0,
        buy_price_min_date: "0001-01-01T00:00:00".to_owned(),
        buy_price_max: 0,
        buy_price_max_date: "0001-01-01T00:00:00".to_owned(),
    };

    let snapshot = map_price_dto(&dto).expect("snapshot");

    assert_eq!(snapshot.item_id, "T4_BAG");
    assert_eq!(snapshot.city.as_str(), "Caerleon");
    assert_eq!(snapshot.sell_price_min, 5277);
    assert_eq!(
        snapshot.observed_at,
        Utc.with_ymd_and_hms(2026, 3, 26, 5, 15, 0).unwrap()
    );
}
