use crate::lb::BoxLoadBalancer;
use crate::LoadBalancerTrait;
use std::collections::HashMap;
use std::convert::Infallible;

pub struct LoadBalancerFactory<I, E = Infallible> {
    registry: HashMap<String, BoxLoadBalancer<I, E>>,
}

impl<I, E> Default for LoadBalancerFactory<I, E> {
    fn default() -> Self {
        Self {
            registry: HashMap::default(),
        }
    }
}

impl<I, E> LoadBalancerFactory<I, E> {
    pub fn add<L>(&mut self, host: &str, load_balancer: L)
    where
        L: LoadBalancerTrait<Element = I, Error = E> + Send + Sync + 'static,
        L::Future: Send + 'static,
    {
        self.registry
            .insert(host.to_string(), load_balancer.boxed());
    }

    pub fn remove(&mut self, host: &str) {
        self.registry.remove(host);
    }

    pub fn get(&self, host: &str) -> Option<&BoxLoadBalancer<I, E>> {
        self.registry.get(host)
    }
}
