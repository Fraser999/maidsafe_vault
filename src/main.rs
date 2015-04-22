/*  Copyright 2015 MaidSafe.net limited
    This MaidSafe Software is licensed to you under (1) the MaidSafe.net Commercial License,
    version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
    licence you accepted on initial access to the Software (the "Licences").
    By contributing code to the MaidSafe Software, or to this project generally, you agree to be
    bound by the terms of the MaidSafe Contributor Agreement, version 1.0, found in the root
    directory of this project at LICENSE, COPYING and CONTRIBUTOR respectively and also
    available at: http://www.maidsafe.net/licenses
    Unless required by applicable law or agreed to in writing, the MaidSafe Software distributed
    under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS
    OF ANY KIND, either express or implied.
    See the Licences for the specific language governing permissions and limitations relating to
    use of the MaidSafe
    Software.                                                                 */

#![crate_name = "maidsafe_vault"]
#![crate_type = "lib"]
#![doc(html_logo_url = "http://maidsafe.net/img/Resources/branding/maidsafe_logo.fab2.png",
       html_favicon_url = "http://maidsafe.net/img/favicon.ico",
              html_root_url = "http://dirvine.github.io/dirvine/maidsafe_vault/")]

#![deny(missing_docs)]
//! MaidSafe Vault provides the interface to SAFE routing.
//!
//! The resulting executable is the Vault node for the SAFE network.
//! Refer to https://github.com/dirvine/maidsafe_vault

extern crate sodiumoxide;
extern crate rustc_serialize;
extern crate cbor;
extern crate time;
extern crate routing;
extern crate maidsafe_types;
extern crate rand;
extern crate lru_time_cache;

mod data_manager;
mod maid_manager;
mod pmid_manager;
mod version_handler;
mod chunk_store;
mod pmid_node;
mod vault;

use vault::VaultFacade;
// use routing::types::DhtId;
// use routing::routing_node::RoutingNode;

/// Placeholder doc test
pub fn always_true() -> bool { true }

/// The Vault structure to hold the logical interface to provide behavioural logic to routing.
pub struct Vault {
  my_facade : VaultFacade
}

impl Vault {
  /// Create a new interface.
  pub fn new() -> Vault {
    Vault { my_facade: VaultFacade::new() }
  }

  /// Start a RoutingNode and run the event loop to handle persona messages.
  pub fn start_vault(&self) {
    // let my_routing = RoutingNode::new(DhtId::generate_random(), &self.my_facade);
    loop {
      std::thread::sleep_ms(500u32);
      always_true();
    }
  }
}

fn main () {

  let vault = Vault::new();
  vault.start_vault();

}



#[test]
fn it_works() {
 assert_eq!(always_true(), true);
}
