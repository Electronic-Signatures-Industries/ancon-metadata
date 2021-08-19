use crate::state::{load_from_store, save_to_store};
use cosmwasm_std::{
    debug_print, from_binary, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse,
    Querier, StdError, StdResult, Storage,
};

use crate::msg::{HandleAnswer, HandleMsg, InitMsg, QueryAnswer, QueryMsg};
use crate::state::{config, File, MetadataSchema, MetadataStorage, State};

use multicodec::MultiCodec;

use std::str::FromStr;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        tag: msg.tag,
        paused: false,
        owner: deps.api.canonical_address(&env.message.sender)?,
    };

    config(&mut deps.storage).save(&state)?;

    //    debug_print!("Contract was initialized by {}", env.message.sender);

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::AddFile {
            path,
            content_type,
            time,
            content,
            mode,
            cid,
        } => add_file(deps, env, path, content_type, time, content, mode, cid),
        HandleMsg::AddMetadata { data, path, cid } => add_metadata(deps, env, data, cid, path),
    }
}

pub fn add_metadata<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    data: String,
    cid: String,
    path: String,
) -> StdResult<HandleResponse> {
    let content_b = base64::decode(data).unwrap();
    let mut composite: String = "".to_string();
    // key: 'QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D::/'
    // key: 'QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D::/name'
    // key: 'QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D::/sources[0]'
    composite.push_str(&cid);
    composite.push_str("::");
    composite.push_str(&path);

    //Saves path & data to interal bincode2 storage
    let callback = HandleAnswer::AddMetadata { cid: cid };

    save_to_store(&mut deps.storage, &composite.into_bytes(), &content_b)?;
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&callback)?),
    })
}

pub fn add_file<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    path: String,
    content_type: String,
    time: u64,
    content: String,
    mode: String,
    cid: String,
) -> StdResult<HandleResponse> {
    let content_b = base64::decode(content).unwrap();
    // let sender_address_raw = deps.api.canonical_address(&env.message.sender)?;
    let path2 = path.clone();
    let cid2 = cid.clone();

    let file = File {
        cid,
        mode,
        content_type,
        path,
        time,
        content: content_b,
    };

    let mut composite: String = "".to_string();
    // key: 'QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D::/'
    // key: 'QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D::/name'
    // key: 'QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D::/sources[0]'
    composite.push_str(&cid2);
    composite.push_str("::");
    composite.push_str(&path2);

    //Saves path & data to interal bincode2 storage
    let callback = HandleAnswer::AddFile { cid: cid2 };

    save_to_store(&mut deps.storage, &composite.into_bytes(), &file)?;
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&callback)?),
    })
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetFile { cid, path } => get_file(deps, cid, path),
        QueryMsg::GetMetadata { cid, path } => get_metadata(deps, cid, path),
    }
}

fn get_metadata<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    cid: String,
    path: String,
) -> StdResult<Binary> {
    // if try_cid.codec() > 0 {
    //     panic!("Invalid CID");
    // }

    let mut composite: String = "".to_string();
    // key: 'QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D::/'
    // key: 'QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D::/name'
    // key: 'QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D::/sources[0]'
    composite.push_str(&cid);
    composite.push_str("::");
    composite.push_str(&path);

    let result = load_from_store(&deps.storage, &composite.into_bytes());
    let block = result.unwrap();
    let response = QueryAnswer::GetMetadata { data: block };

    //Returns the metadata loaded from store from a CID & transformed to an IpldBlock
    Ok(to_binary(&response)?)
}

fn get_file<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    cid: String,
    path: String,
) -> StdResult<Binary> {
    // if try_cid.codec() > 0 {
    //     panic!("Invalid CID");
    // }

    let mut composite: String = "".to_string();
    // key: 'QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D::/'
    // key: 'QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D::/name'
    // key: 'QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D::/sources[0]'
    composite.push_str(&cid);
    composite.push_str("::");
    composite.push_str(&path);

    let result = load_from_store(&deps.storage, &composite.into_bytes());
    let block = result.unwrap();

    let response = QueryAnswer::GetFile { data: block };

    Ok(to_binary(&response)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{coins, CosmosMsg};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg {
            tag: "test".to_string(),
        };
        let env = mock_env("creator", &coins(1000, "xdv"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    // #[test]
    // fn add_metadata() {
    //     let mut deps = mock_dependencies(20, &coins(2, "token"));

    //     let amount = coins(40, "ETH");
    //     let collateral = coins(1, "BTC");
    //     let expires = 100_000;
    //     let msg = InitMsg {
    //         tag: "test".to_string(),
    //     };
    //     let env = mock_env("creator", &collateral);

    //     // we can just call .unwrap() to assert this was a success
    //     let _ = init(&mut deps, env, msg).unwrap();

    //     let data = MetadataSchema {
    //         name: "XDV metadata sample: NFT".to_string(),
    //         description: "testing sample".to_string(),
    //         image:
    //             "https://explore.ipld.io/#/explore/QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D"
    //                 .to_string(),
    //         sources: vec!["QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D".to_string()],
    //         parent: "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D".to_string(),
    //         refs: vec![
    //             "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D".to_string(),
    //             "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D".to_string(),
    //         ],
    //     };
    //     let cid = "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D".to_string();
    //     // add metadata

    //     // add metadata - success message
    //     let payload = HandleMsg::AddMetadata {
    //         data: data,
    //         path: "/".to_string(),
    //     };
    //     let resp: HandleResponse =
    //         handle(&mut deps, mock_env("creator", &collateral), payload).unwrap();

    //     let b = resp.data.unwrap_or_default();
    //     let object = from_binary(&b).unwrap();
    //     match object {
    //         HandleAnswer::AddFile { cid } => {}
    //         HandleAnswer::AddMetadata { cid } => {
    //             assert_eq!(
    //                 cid,
    //                 "bafyreicnuvbp2lhmanra7r5o564fo4n5hhynqmwqv5l3ymz27gqbmlf2xa"
    //             );
    //             assert_ne!(
    //                 cid,
    //                 "hhynqmwqv5l3ymz27gqbmlf2xabafyreicnuvbp2lhmanra7r5o564fo4n5"
    //             );
    //             assert_ne!(cid, "57576");
    //             assert_ne!(cid, "");
    //             assert_ne!(cid, "0");
    //         }
    //     }
    // }
    // fn add_file() {
    //     let mut deps = mock_dependencies(20, &coins(2, "token"));

    //     let amount = coins(40, "ETH");
    //     let collateral = coins(1, "BTC");
    //     let expires = 100_000;
    //     let msg = InitMsg {};
    //     let env = mock_env("creator", &collateral);

    //     // we can just call .unwrap() to assert this was a success
    //     let _ = init(&mut deps, env, msg).unwrap();

    //     let data = MetadataSchema {
    //     };
    //     let cid = "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D".to_string();
    //     // add metadata

    //     // add metadata - success message
    //     let payload = HandleMsg::AddFile {
    //         data: data,
    //         path: "/".to_string(),
    //     };
    //     let resp: HandleResponse =
    //         handle(&mut deps, mock_env("creator", &collateral), payload).unwrap();

    //     let b = resp.data.unwrap_or_default();
    //     let object = from_binary(&b).unwrap();
    //     match object {
    //         HandleAnswer::AddFile { cid } => {
    //             assert_eq!(
    //             cid,
    //             "bafyreicnuvbp2lhmanra7r5o564fo4n5hhynqmwqv5l3ymz27gqbmlf2xa"
    //         );
    //     }}
    //         HandleAnswer::AddMetadata { cid } => {
    //     }
    // }
    // #[test]
    // fn get_metadata() {
    //     let mut deps = mock_dependencies(20, &coins(2, "token"));

    //     let amount = coins(40, "ETH");
    //     let collateral = coins(1, "BTC");
    //     let expires = 100_000;
    //     let msg = InitMsg {
    //         tag: "test".to_string(),
    //     };
    //     let env = mock_env("creator", &collateral);

    //     // we can just call .unwrap() to assert this was a success
    //     let _ = init(&mut deps, env, msg).unwrap();
    //     //Adding Metadata for the test
    //     let data_payload = MetadataSchema {
    //         name: "XDV metadata sample: NFT".to_string(),
    //         description: "testing sample".to_string(),
    //         image:
    //             "https://explore.ipld.io/#/explore/QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D"
    //                 .to_string(),
    //         sources: vec!["QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D".to_string()],
    //         parent: "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D".to_string(),
    //         refs: vec![
    //             "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D".to_string(),
    //             "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D".to_string(),
    //         ],
    //     };
    //     let cid = "bafyreicnuvbp2lhmanra7r5o564fo4n5hhynqmwqv5l3ymz27gqbmlf2xa".to_string();
    //     // add metadata

    //     // add metadata - success message
    //     let payload_m = HandleMsg::AddMetadata {
    //         data: data_payload,
    //         path: "/".to_string(),
    //     };

    //     let resp: HandleResponse =
    //         handle(&mut deps, mock_env("creator", &collateral), payload_m).unwrap();

    //     // get metadata
    //     // get metadata - success message
    //     let payload_q = QueryMsg::GetMetadata {
    //         cid: cid,
    //         path: "/".to_string(),
    //     };
    //     let resp: Binary = query(&mut deps, payload_q).unwrap();

    //     let object = from_binary(&resp).unwrap();
    //     match object {
    //         QueryAnswer::GetFile { data } => {}
    //         QueryAnswer::GetMetadata { data } => {
    //             assert_eq!(data.len(), 325,);
    //         }
    //     }
    // }

    // fn get_file() {
    //     let mut deps = mock_dependencies(20, &coins(2, "token"));

    //     let amount = coins(40, "ETH");
    //     let collateral = coins(1, "BTC");
    //     let expires = 100_000;
    //     let msg = InitMsg {};
    //     let env = mock_env("creator", &collateral);

    //     // we can just call .unwrap() to assert this was a success
    //     let _ = init(&mut deps, env, msg).unwrap();

    //     let data = File {
    //     };
    //     let cid = "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D".to_string();
    //     // add metadata

    //     // add metadata - success message
    //     let payload = HandleMsg::GetFile {
    //         data: data,
    //         path: "/".to_string(),
    //     };
    //     let resp: HandleResponse =
    //         handle(&mut deps, mock_env("creator", &collateral), payload).unwrap();

    //     let b = resp.data.unwrap_or_default();
    //     let object = from_binary(&b).unwrap();
    //     match object {
    //         HandleAnswer::GetFile { cid } => {}
    //         HandleAnswer::GetMetadata { cid } => {
    //             assert_eq!(
    //             );
    //         }
    //     }

    // let cid = "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D".to_string();

    // match object {
    //     HandleAnswer::AddFile { cid } => {}
    //     HandleAnswer::AddMetadata { cid } => {
    //         assert_eq!(
    //             cid,
    //             "hhynqmwqv5l3ymz27gqbmlf2xabafyreicnuvbp2lhmanra7r5o564fo4n5"
    //         );
    //     }
    // }

    //   // expired cannot execute
    //   let info = mock_env("owner", &amount);
    //   let mut env = mock_env();
    //   env.block.height = 200_000;
    //   let err = handle_execute(deps.as_mut(), env, info).unwrap_err();
    //   match err {s
    //     ContractError::OptionExpired { expired } => assert_eq!(expired, expires),
    //     e => panic!("unexpected error: {}", e),
    //   }

    //   // bad counter_offer cannot execute
    //   let msg_offer = coins(39, "ETH");
    //   let info = mock_env("owner", &msg_offer);
    //   let err = handle_execute(deps.as_mut(), mock_env(), info).unwrap_err();
    //   match err {
    //     ContractError::CounterOfferMismatch {
    //       offer,
    //       counter_offer,
    //     } => {
    //       assert_eq!(msg_offer, offer);
    //       assert_eq!(amount, counter_offer);
    //     }
    //     e => panic!("unexpected error: {}", e),
    //   }

    //   // proper execution
    //   let info = mock_env("owner", &amount);
    //   let res = handle_execute(deps.as_mut(), mock_env(), info).unwrap();
    //   assert_eq!(res.messages.len(), 2);
    //   assert_eq!(
    //     res.messages[0],
    //     CosmosMsg::Bank(BankMsg::Send {
    //       from_address: MOCK_CONTRACT_ADDR.into(),
    //       to_address: "creator".into(),
    //       amount,
    //     })
    //   );
    //   assert_eq!(
    //     res.messages[1],
    //     CosmosMsg::Bank(BankMsg::Send {
    //       from_address: MOCK_CONTRACT_ADDR.into(),
    //       to_address: "owner".into(),
    //       amount: collateral,
    //     })
    //   );

    //   // check deleted
    //   let _ = query_config(deps.as_ref()).unwrap_err();
    // }
    //}
}
