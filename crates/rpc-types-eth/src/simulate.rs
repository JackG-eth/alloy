/*

eth_simulateV1 (formerly eth_multicallV1) is an in-progress RPC endpoint that seems worth supporting in reth and alloy:
ethereum/execution-apis#484

We first need some basic types to for serialization / deserialization. It looks like we will want some structures analogous to these geth structs:


// simBlock is a batch of calls to be simulated sequentially.
    type simBlock struct {
        BlockOverrides *BlockOverrides
        StateOverrides *StateOverride
        Calls          []TransactionArgs
    }

    type simCallResult struct {
        ReturnValue hexutil.Bytes  `json:"returnData"`
        Logs        []*types.Log   `json:"logs"`
        GasUsed     hexutil.Uint64 `json:"gasUsed"`
        Status      hexutil.Uint64 `json:"status"`
        Error       *callError     `json:"error,omitempty"`
    }

    type simOpts struct {
    BlockStateCalls        []simBlock
    TraceTransfers         bool
    Validation             bool
    ReturnFullTransactions bool

    for the simulate interface:
https://github.com/s1na/go-ethereum/blob/4c325ba5b365e44f1c33cfe7b3236bcb685ec2de/internal/ethapi/api.go#L1255

There seem to be many test vectors in the execution-apis PR, which should be useful
@@ -0,0 +1,22 @@
use crate::eth::revm_utils::EvmOverrides;
use reth_rpc_types::{state::StateOverride, BlockOverrides, Bundle, EthCallResponse};

use alloy_rpc_types::{TransactionInput, TransactionRequest};

use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockOverrideSim {
    pub number: Option<U256>,
    pub difficulty: Option<U256>,
    pub time: Option<U64>,
    pub random: Option<B256>,
    pub base_fee: Option<U256>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimBlock {
    pub blockOverrides: Vector<BlockOverridesSim>,
    pub stateOverride: StateOverride,
    pub calls: Vec<TransactionRequest>,
}

}

*/

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockOverrideSim {
    pub number: Option<U256>,
    pub difficulty: Option<U256>,
    pub time: Option<U64>,
    pub random: Option<B256>,
    pub base_fee: Option<U256>,
}

pub struct SimBlock {}

pub struct SimCallResults {}

pub struct SimOpts {}
