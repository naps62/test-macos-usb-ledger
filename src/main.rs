use coins_ledger::LedgerError;
use coins_ledger::{
    common::{APDUAnswer, APDUCommand, APDUData},
    transports::{Ledger, LedgerAsync},
};
use futures_executor::block_on;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum INS {
    GET_PUBLIC_KEY = 0x02,
    SIGN = 0x04,
    GET_APP_CONFIGURATION = 0x06,
    SIGN_PERSONAL_MESSAGE = 0x08,
    SIGN_ETH_EIP_712 = 0x0C,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum P1 {
    NON_CONFIRM = 0x00,
    MORE = 0x80,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum P2 {
    NO_CHAINCODE = 0x00,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let ledger = ethers_signers::Ledger::new(ethers_signers::HDPath::LedgerLive(0), 1).await;
    let transport = coins_ledger::transports::Ledger::init()
        .await
        .expect("can't init ledger");
    let data = APDUData::new(&path_to_bytes("m/44'/60'/0'/0/0"));

    let command = APDUCommand {
        ins: INS::GET_PUBLIC_KEY as u8,
        p1: P1::NON_CONFIRM as u8,
        p2: P2::NO_CHAINCODE as u8,
        data,
        response_len: None,
    };

    let answer = block_on(transport.exchange(&command)).expect("failed exchange");
    let result = answer.data().expect("unexpected null response");

    // extract the address from the response
    let offset = 1 + result[0] as usize;
    let address_str = &result[offset + 1..offset + 1 + result[offset] as usize];
    let mut address = [0; 20];
    address.copy_from_slice(&hex::decode(address_str)?);

    dbg!(&address);

    Ok(())
}

fn path_to_bytes(derivation: &str) -> Vec<u8> {
    let elements = derivation.split('/').skip(1).collect::<Vec<_>>();
    let depth = elements.len();

    let mut bytes = vec![depth as u8];
    for derivation_index in elements {
        let hardened = derivation_index.contains('\'');
        let mut index = derivation_index.replace('\'', "").parse::<u32>().unwrap();
        if hardened {
            index |= 0x80000000;
        }

        bytes.extend(index.to_be_bytes());
    }

    bytes
}
