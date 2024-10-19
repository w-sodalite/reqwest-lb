use reqwest::{Client, Url};
use reqwest_lb::LoadBalancerMiddleware;
use reqwest_lb::SimpleLoadBalancer;
use reqwest_lb::{LoadBalancerFactory, LoadBalancerPolicy};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

pub fn create<const N: usize>(
    ports: [u16; N],
    policy: LoadBalancerPolicy<Url>,
) -> ClientWithMiddleware {
    // create load balancer factory
    let mut factory = LoadBalancerFactory::default();

    // create url list
    let mut urls = Vec::with_capacity(N);
    for port in ports {
        urls.push(Url::parse(format!("http://127.0.0.1:{}", port).as_str()).unwrap());
    }

    // create load balancer
    let load_balancer = SimpleLoadBalancer::new(urls, policy);

    // register load balancer for the host: example-server
    factory.add("example-server", load_balancer);

    // create reqwest client
    let middleware = LoadBalancerMiddleware::new(factory);
    ClientBuilder::new(Client::builder().no_proxy().build().unwrap())
        .with(middleware)
        .build()
}
