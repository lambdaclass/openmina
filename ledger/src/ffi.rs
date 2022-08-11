use std::{str::FromStr, cell::RefCell, sync::Mutex};

use mina_hasher::Fp;
use ocaml_interop::{ocaml_export, OCaml, OCamlBytes, OCamlRef};
use once_cell::sync::Lazy;

use crate::{account::Account, tree_version::V2, tree::Database};

static DATABASE: Lazy<Mutex<Database<V2>>> = Lazy::new(|| {
    Mutex::new(Database::create(30))
});

ocaml_export! {
    fn rust_add_account(
        rt,
        account: OCamlRef<OCamlBytes>,
    ) {
        println!("RUST BEGIN");
        let account_ref = rt.get(account);
        let account = account_ref.as_bytes();

        let account: Account = serde_binprot::from_slice(account).unwrap();

        println!("account={:?}", account);
        println!("account_hash={:?}", account.hash().to_string());

        println!("RUST END 1");
        OCaml::unit()
    }

    fn rust_add_account_with_hash(
        rt,
        account: OCamlRef<OCamlBytes>,
        hash: OCamlRef<String>,
    ) {
        println!("RUST BEGIN");
        let account_ref = rt.get(account);
        let account = account_ref.as_bytes();
        let hash: String = hash.to_rust(rt);
        let hash = Fp::from_str(&hash).unwrap();

        let account: Account = serde_binprot::from_slice(account).unwrap();
        let account_hash = account.hash();

        assert_eq!(hash, account_hash);

        println!("provided={:?}", hash.to_string());
        println!("computed={:?}", account_hash.to_string());

        // println!("account={:?}", account);
        // println!("account_hash={:?}", account.hash().to_string());

        let mut db = DATABASE.lock().unwrap();
        db.create_account((), account).unwrap();

        println!("RUST END");
        OCaml::unit()
    }

    fn rust_root_hash(rt, _unused: OCamlRef<String>) {
        let db = DATABASE.lock().unwrap();
        let hash = db.root_hash();

        println!("rust_root_hash={:?}", hash.to_string());

        OCaml::unit()
    }
}
