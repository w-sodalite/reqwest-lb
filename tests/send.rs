use http::StatusCode;
use reqwest::Url;
use reqwest_lb::LoadBalancerPolicy;
use std::time::Duration;
use tokio::spawn;
use tokio::sync::OnceCell;
use tokio::time::sleep;

mod client;
mod server;

const PORTS: [u16; 3] = [3000, 3001, 3002];

const STR_PORTS: [&str; 3] = ["3000", "3001", "3002"];

static INITIALIZE: OnceCell<()> = OnceCell::const_new();

async fn setup() {
    spawn(INITIALIZE.get_or_init(|| server::create(PORTS)));
    sleep(Duration::from_secs(10)).await;
}

#[tokio::test]
async fn round_robin_policy() {
    setup().await;

    let client = client::create(PORTS, LoadBalancerPolicy::RoundRobin);
    let r1 = client.get("lb://example-server/").send().await.unwrap();
    let r2 = client.get("lb://example-server/").send().await.unwrap();
    let r3 = client.get("lb://example-server/").send().await.unwrap();
    assert_eq!(r1.status(), StatusCode::OK);
    assert_eq!(r2.status(), StatusCode::OK);
    assert_eq!(r3.status(), StatusCode::OK);
    assert_eq!(r1.text().await.unwrap(), "3000");
    assert_eq!(r2.text().await.unwrap(), "3001");
    assert_eq!(r3.text().await.unwrap(), "3002");
}

#[tokio::test]
async fn random_policy() {
    setup().await;

    let client = client::create(PORTS, LoadBalancerPolicy::Random);
    let r1 = client.get("lb://example-server/").send().await.unwrap();
    let r2 = client.get("lb://example-server/").send().await.unwrap();
    let r3 = client.get("lb://example-server/").send().await.unwrap();
    assert_eq!(r1.status(), StatusCode::OK);
    assert_eq!(r2.status(), StatusCode::OK);
    assert_eq!(r3.status(), StatusCode::OK);
    assert!(STR_PORTS.contains(&r1.text().await.unwrap().as_str()));
    assert!(STR_PORTS.contains(&r2.text().await.unwrap().as_str()));
    assert!(STR_PORTS.contains(&r3.text().await.unwrap().as_str()));
}

#[tokio::test]
async fn first_policy() {
    setup().await;

    let client = client::create(PORTS, LoadBalancerPolicy::First);
    let r1 = client.get("lb://example-server/").send().await.unwrap();
    let r2 = client.get("lb://example-server/").send().await.unwrap();
    let r3 = client.get("lb://example-server/").send().await.unwrap();
    assert_eq!(r1.status(), StatusCode::OK);
    assert_eq!(r2.status(), StatusCode::OK);
    assert_eq!(r3.status(), StatusCode::OK);
    assert_eq!(r1.text().await.unwrap(), "3000");
    assert_eq!(r2.text().await.unwrap(), "3000");
    assert_eq!(r3.text().await.unwrap(), "3000");
}

#[tokio::test]
async fn last_policy() {
    setup().await;

    let client = client::create(PORTS, LoadBalancerPolicy::Last);
    let r1 = client.get("lb://example-server/").send().await.unwrap();
    let r2 = client.get("lb://example-server/").send().await.unwrap();
    let r3 = client.get("lb://example-server/").send().await.unwrap();
    assert_eq!(r1.status(), StatusCode::OK);
    assert_eq!(r2.status(), StatusCode::OK);
    assert_eq!(r3.status(), StatusCode::OK);
    assert_eq!(r1.text().await.unwrap(), "3002");
    assert_eq!(r2.text().await.unwrap(), "3002");
    assert_eq!(r3.text().await.unwrap(), "3002");
}

#[tokio::test]
async fn weight_policy() {
    setup().await;

    let client = client::create(
        PORTS,
        LoadBalancerPolicy::weight(|url: &Url| {
            let port = url.port().unwrap();
            port as usize % PORTS.len() + 1
        }),
    );
    let r1 = client.get("lb://example-server/").send().await.unwrap();
    let r2 = client.get("lb://example-server/").send().await.unwrap();
    let r3 = client.get("lb://example-server/").send().await.unwrap();
    assert_eq!(r1.status(), StatusCode::OK);
    assert_eq!(r2.status(), StatusCode::OK);
    assert_eq!(r3.status(), StatusCode::OK);
    assert!(STR_PORTS.contains(&r1.text().await.unwrap().as_str()));
    assert!(STR_PORTS.contains(&r2.text().await.unwrap().as_str()));
    assert!(STR_PORTS.contains(&r3.text().await.unwrap().as_str()));
}

#[tokio::test]
async fn dynamic_policy() {
    setup().await;

    let client = client::create(PORTS, LoadBalancerPolicy::dynamic(|_, _| 0));
    let r1 = client.get("lb://example-server/").send().await.unwrap();
    let r2 = client.get("lb://example-server/").send().await.unwrap();
    let r3 = client.get("lb://example-server/").send().await.unwrap();
    assert_eq!(r1.status(), StatusCode::OK);
    assert_eq!(r2.status(), StatusCode::OK);
    assert_eq!(r3.status(), StatusCode::OK);
    assert_eq!(r1.text().await.unwrap(), "3000");
    assert_eq!(r2.text().await.unwrap(), "3000");
    assert_eq!(r3.text().await.unwrap(), "3000");
}
