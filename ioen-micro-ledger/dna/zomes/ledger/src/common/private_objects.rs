use hdk::prelude::*;

/* 
#[derive(Serialize, Deserialize, Debug)]
pub enum PrivateObjectFromBidder {
    BidCommitment(BidCommitment),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PrivateObjectFromMarket {
    MarketPlan(MarketPlan),
}
 */
pub fn common_cap_secret() -> CapSecret {
    let bytes: [u8; 64] = [0_u8; 64];
    CapSecret::from(bytes)
}