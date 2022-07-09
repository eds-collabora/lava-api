use futures::future::BoxFuture;
use futures::FutureExt;
use futures::{stream, stream::Stream, stream::StreamExt};
use serde::Deserialize;
use serde_with::DeserializeFromStr;
use std::pin::Pin;
use std::task::{Context, Poll};
use strum::{Display, EnumString};

use crate::paginator::{PaginationError, Paginator};
use crate::tag::Tag;
use crate::Lava;

#[derive(Clone, Copy, Debug, DeserializeFromStr, Display, EnumString, Eq, PartialEq)]
pub enum Health {
    Unknown,
    Maintenance,
    Good,
    Bad,
    Looping,
    Retired,
}

#[derive(Clone, Deserialize, Debug)]
struct LavaDevice {
    hostname: String,
    worker_host: String,
    device_type: String,
    description: Option<String>,
    health: Health,
    pub tags: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Device {
    pub hostname: String,
    pub worker_host: String,
    pub device_type: String,
    pub description: Option<String>,
    pub health: Health,
    pub tags: Vec<Tag>,
}

enum State<'a> {
    Paging,
    Transforming(BoxFuture<'a, Device>),
}

pub struct Devices<'a> {
    lava: &'a Lava,
    paginator: Paginator<LavaDevice>,
    state: State<'a>,
}

impl<'a> Devices<'a> {
    pub fn new(lava: &'a Lava) -> Self {
        let url = lava
            .base
            .join("devices/?ordering=hostname")
            .expect("Failed to append to base url");
        let paginator = Paginator::new(lava.client.clone(), url);
        Self {
            lava,
            paginator,
            state: State::Paging,
        }
    }
}

async fn transform_device(device: LavaDevice, lava: &Lava) -> Device {
    let t = stream::iter(device.tags.iter());
    let tags = t
        .filter_map(|i| async move { lava.tag(*i).await })
        .collect()
        .await;

    Device {
        hostname: device.hostname,
        worker_host: device.worker_host,
        device_type: device.device_type,
        description: device.description,
        health: device.health,
        tags,
    }
}

impl<'a> Stream for Devices<'a> {
    type Item = Result<Device, PaginationError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let me = self.get_mut();

        loop {
            return match &mut me.state {
                State::Paging => {
                    let p = Pin::new(&mut me.paginator);
                    match p.poll_next(cx) {
                        Poll::Ready(None) => Poll::Ready(None),
                        Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
                        Poll::Ready(Some(Ok(d))) => {
                            me.state = State::Transforming(transform_device(d, me.lava).boxed());
                            continue;
                        }
                        Poll::Pending => Poll::Pending,
                    }
                }
                State::Transforming(fut) => match fut.as_mut().poll(cx) {
                    Poll::Ready(d) => {
                        me.state = State::Paging;
                        Poll::Ready(Some(Ok(d)))
                    }
                    Poll::Pending => Poll::Pending,
                },
            };
        }
    }
}