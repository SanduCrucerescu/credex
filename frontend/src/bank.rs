use common::Client;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local", storage_tab_sync)]
pub struct Bank {
    pub clients: Vec<Client>,
    pub loading: bool,
}

pub fn set_client(client: Client, dispatch: Dispatch<Bank>) {
    dispatch.reduce_mut(|state| {
        state.clients.insert(0, client);
    });
}

pub fn set_loading(loading: bool, dispatch: Dispatch<Bank>) {
    dispatch.reduce_mut(|state| {
        state.loading = loading;
    })
}
