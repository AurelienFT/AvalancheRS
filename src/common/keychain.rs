pub trait StandardKeyPair {
    fn get_public_key(&self) -> &[u8];
    fn get_private_key(&self) -> &[u8];
    fn generate_key(&self, entropy: Option<&[u8]>);
    fn import_key(&self, priv_key: &[u8]);
    fn sign(&self, message: &[u8]) -> &[u8];
    fn recover(&self, message: &[u8], signature: &[u8]) -> Result<&[u8], ()>;
    fn verify(&self, message: &[u8], signature: &[u8], pubk: &[u8]) -> bool;
    fn get_public_key_string(&self) -> String;
    fn get_private_key_string(&self) -> String;
    fn get_address(&self) -> &[u8];
    fn get_address_string(&self) -> String;
}

pub trait StandardKeyChain<KPClass>
where
    KPClass: StandardKeyPair
{
    fn make_key(&self) -> KPClass;
    fn import_key(&self, priv_key: &[u8]) -> KPClass;
    fn get_addresses(&self) -> Vec<&[u8]>;
    fn get_addresses_strings(&self) -> Vec<String>;
    fn add_key(&mut self, key: KPClass);
    fn remove_key(&mut self, key: KPClass);
    fn has_key(&self, address: &[u8]) -> bool;
    fn get_key(&self, address: &[u8]) -> Option<&KPClass>;
}