use albion_crafting_overlay::config::server_base_url;

#[test]
fn server_selection_maps_to_expected_base_url() {
    assert_eq!(
        server_base_url("west"),
        "https://west.albion-online-data.com"
    );
    assert_eq!(
        server_base_url("east"),
        "https://east.albion-online-data.com"
    );
    assert_eq!(
        server_base_url("europe"),
        "https://europe.albion-online-data.com"
    );
}
