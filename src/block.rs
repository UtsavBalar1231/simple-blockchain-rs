use super::transaction::Transaction;

#[derive(Debug)]
pub struct Block {
    pub index: u64,
    pub verified_transactions: Vec<Transaction>,
    pub pre_hash: Option<String>,
    pub nonce: Option<u64>,
    pub signature: Option<String>,
}

impl Block {
    pub fn new(index: u64) -> Self {
        Self {
            index,
            verified_transactions: vec![],
            pre_hash: Some(String::new()),
            nonce: None,
            signature: None,
        }
    }

    pub fn has_valid_transactions(&self) -> bool {
        for tran in &self.verified_transactions {
            if !tran.is_valid_transaction() {
                return false;
            }
        }

        return true;
    }
    pub fn sign_block(&mut self) {
        self.signature = Some(Self::calculate_hash(
            &self.pre_hash,
            &self.verified_transactions,
            &self.nonce,
            &self.signature,
        ));
    }

    pub fn calculate_hash(
        pre_hash: &Option<String>,
        transactions: &Vec<Transaction>,
        nonce: &Option<u64>,
        signature: &Option<String>,
    ) -> String {
        let mut bytes = vec![];
        bytes.extend(
            transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>(),
        );

        if let Some(pre_hash) = pre_hash {
            bytes.extend(pre_hash.as_bytes());
        }

        if let Some(nonce) = nonce {
            bytes.extend(&nonce.to_le_bytes());
        }

        if let Some(signature) = signature {
            bytes.extend(signature.as_bytes());
        }

        crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
    }
}
