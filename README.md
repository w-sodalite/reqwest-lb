# reqwest-lb

A crate for [reqwest](https://crates.io/crates/reqwest) support load balancer, use
the [reqwest-middleware](https://crates.io/crates/reqwest-middleware).

## Overview

This crate provide a middleware `LoadBalancerMiddleware`, it implement `reqwest-middleware::Middleware`.

- ### dependencies

    ```toml
    [dependencies]
    reqwest = "0.12"
    reqwest-middleware = "0.3"
    reqwest-lb = "0.1"
    ```

- ### example

    ```rust
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
  
    pub async fn run() {
        // use round robin policy
        let client = create([3001, 3002], LoadBalancerPolicy::RoundRobin);
        // http://127.0.0.1:3001
        let response = client.get("lb://example-server/").send().await.unwrap();
        // http://127.0.0.1:3002
        let response = client.get("lb://example-server/").send().await.unwrap();
    }
  
    ```

- ### load balancer policy

  - RoundRobin (default)
  - Random
  - First
  - Last
  - Weight

- ### discovery

  ...

## License

This project is licensed under the [Apache 2.0](./LICENSE)