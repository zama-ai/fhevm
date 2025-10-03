use crate::{
    config::CiphertextConfig,
    decryption::types::{DecryptionRequest, DecryptionType},
};
use alloy::primitives::{Address, Bytes, U256};
use fhevm_gateway_bindings::decryption::Decryption::{
    PublicDecryptionRequest, SnsCiphertextMaterial, UserDecryptionRequest,
};
use rand::Rng;

pub struct RequestBuilder {
    id_counter: U256,
    user_ct: Vec<CiphertextConfig>,
    public_ct: Vec<CiphertextConfig>,
    key_id: U256,
    copro_tx_sender_addr: Address,
}

impl RequestBuilder {
    pub fn new(
        user_ct: Vec<CiphertextConfig>,
        public_ct: Vec<CiphertextConfig>,
        key_id: U256,
        copro_tx_sender_addr: Address,
    ) -> Self {
        Self {
            // Take a high value for the id_counter to avoid polluting id that could be used in
            // the testing environment
            id_counter: (U256::MAX / U256::from(4)) * U256::from(3),
            user_ct,
            public_ct,
            key_id,
            copro_tx_sender_addr,
        }
    }

    pub fn build_requests(
        &mut self,
        decryption_type: DecryptionType,
        count: u32,
    ) -> anyhow::Result<Vec<DecryptionRequest>> {
        let mut requests = Vec::new();

        for _ in 0..count {
            let request = match decryption_type {
                DecryptionType::Public => DecryptionRequest::Public(self.build_public_request()?),
                DecryptionType::User => DecryptionRequest::User(self.build_user_request()?),
            };
            requests.push(request);
        }

        Ok(requests)
    }

    fn build_public_request(&mut self) -> anyhow::Result<PublicDecryptionRequest> {
        let decryption_id = self.generate_unique_id();
        let sns_ct_materials = self.generate_sns_materials(self.public_ct.clone());
        let extra_data = self.generate_extra_data();

        Ok(PublicDecryptionRequest {
            decryptionId: decryption_id,
            snsCtMaterials: sns_ct_materials,
            extraData: extra_data,
        })
    }

    fn build_user_request(&mut self) -> anyhow::Result<UserDecryptionRequest> {
        let decryption_id = self.generate_unique_id();
        let sns_ct_materials = self.generate_sns_materials(self.user_ct.clone());
        let user_address = Address::from(rand::rng().random::<[u8; 20]>());
        let public_key = alloy::hex::decode(COMMON_PUBLIC_KEY).unwrap().into();
        let extra_data = self.generate_extra_data();

        Ok(UserDecryptionRequest {
            decryptionId: decryption_id,
            snsCtMaterials: sns_ct_materials,
            userAddress: user_address,
            publicKey: public_key,
            extraData: extra_data,
        })
    }

    fn generate_unique_id(&mut self) -> U256 {
        let id = self.id_counter;
        self.id_counter += U256::ONE;
        id
    }

    fn generate_sns_materials(
        &self,
        ciphertexts: Vec<CiphertextConfig>,
    ) -> Vec<SnsCiphertextMaterial> {
        ciphertexts
            .iter()
            .map(|ct| SnsCiphertextMaterial {
                ctHandle: ct.handle,
                keyId: self.key_id,
                snsCiphertextDigest: ct.digest,
                coprocessorTxSenderAddresses: vec![self.copro_tx_sender_addr],
            })
            .collect()
    }

    fn generate_extra_data(&self) -> Bytes {
        vec![1].into()
    }
}

const COMMON_PUBLIC_KEY: &str = "0x0300000000000000302e35000000000300000000000000302e311300000000000000556e69666965645075626c6963456e634b6579000000000000000020030000000000003098880dd1a4a1277fb90149353c7a655c4552799d3bba834f4a7fd8eb99e89a5f1c3b37e99826250954b80bcb9a1ac2745159a300c2f86c4b1f03d048008bf9f186e288901e70362937804632537810140a6a44eb924a87771c2aea757cf72fd49670dbc8a530889ad2fa8fb59c1deb651bc7527b679175cd487caad286183610db0357b6c8a8adda92a1b008d47063de88ab17ccc536083841a394422bb90b3297f36417e127967df3ca7035c88a905a9f8842e106bfc11210e02ab309f07e91103f092067257aae444197236006edf68df47a8d9f9b42e0a998806849646a7c4a42c375102f2b1c5185e03e74387e69b20b40c60863526a0f8a93ad1823ab89ae59dc141d7c1078a1086b52b7ecf26a2c200f49b5125593b5bdb76de0a635fb298036700c162770a45b9029998775a04bd41b73884072c00688ba83177f440d6b0c79c7895c9a6c4230b891a47027cba42ed220c89a904ad20862d546621162268124092b5b4aee603b6fe74c3c7451143ba0ce847aae70a36bd76df123440d866e6029cf1e1aa9f2b5c66255c54abc023b769ba7c8990742b8961687a286a643079224c979f0da6a63a34581fab6e9d0c2f645c78dcc93c7423f2aa10e5af98f4e28aafc1186ddf79eff04a84d484577369bccc3802cd47dcbc4772947cd6c2555453a0a8fc87b83932a0c6c4dd395677797105987967909bb6a754e5af9c1775b07cbc63410ac6c51e6719d7b645e667c08e842ee5092f3e74dcd271a8bf764e9341dcdf53ddd63895da61249a6a576886f76778c4cb636acd32be88529ceea5c9fb55c91c5549ba04c6e2b52f2d08a8803c64587c11393479b0065c3114f34fc2ed81c4ae0a67574009b04094ba2429f47b0278c14c07474aa3b1404abc11f63a595d55599ca054c9c5c74ea91a8624aac2527a1c2f7a7e48744c3a94ee3908c0beb944e051062ab33c8145d85023abf1b79717946e6ca35ba56785cb5abf4d7434536c3f9b0329a3b95efc7ae0f856be77937c2335c53ab9540c44792e8288b0c3c970c3762011d39cb35a9ac7016a69e33f78096b66c00d8486db5c447e0e5f87ae1ddb55b6e2404de14510f3840e3077a6d8d0b1385aa801137f986";
