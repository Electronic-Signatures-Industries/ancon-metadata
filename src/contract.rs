use crate::state::{load_from_store, save_to_store};

use cosmwasm_std::{
    debug_print, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdError, StdResult, Storage,
};

use crate::msg::{HandleAnswer, HandleMsg, InitMsg, QueryAnswer, QueryMsg};
use crate::state::{config, config_read, File, Metadata, MetadataSchema, MetadataStorage, State};

use libipld::{block::Block, ipld, ipld::Ipld, cbor::DagCborCodec, cid::multihash::Code, store::DefaultParams, Cid};

use std::str::FromStr;

type IpldBlock = libipld::block::Block<DefaultParams>;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        paused: false,
        owner: deps.api.canonical_address(&env.message.sender)?,
    };

    config(&mut deps.storage).save(&state)?;

    debug_print!("Contract was initialized by {}", env.message.sender);

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleAnswer> {
    match msg {
        HandleMsg::AddFile {
            path,
            content_type,
            time,
            content,
            mode,
        } => add_file(deps, env, path, content_type, time, content, mode),
        HandleMsg::AddMetadata { data } => add_metadata(deps, env, data),
    }
}

pub fn add_metadata<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    data: MetadataSchema,
) -> StdResult<HandleAnswer> {
    let links: Vec<_> = data
        .links
        .iter()
        .map(|l| Ipld::Link(Cid::from_str(l).unwrap()))
        .collect();

    let block = Block::<DefaultParams>::encode(
        DagCborCodec,
        Code::Sha2_256,
        &ipld!({
            "name":data.name,
            "description": data.description,
            "image": data.image,
            "links": links,
        }),
    )
    .unwrap();

    let cid = block.cid().to_string();
    let data = block.data().to_vec();

    //Saves the cid & data to interal bincode2 storage
    let callback = HandleAnswer::AddMetadata { cid: cid.clone() };
    let id = cid.into_bytes();
    save_to_store(&mut deps.storage, &id, &data)?;
    Ok(callback)
}

pub fn add_file<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    path: String,
    content_type: String,
    time: u64,
    content: Vec<u8>,
    mode: String,
) -> StdResult<HandleAnswer> {
    let sender_address_raw = deps.api.canonical_address(&env.message.sender)?;

    let block = Block::<DefaultParams>::encode(
        DagCborCodec,
        Code::Sha2_256,
        &ipld!({
            "owner": sender_address_raw.to_string(),
            "path": path,
            "type": content_type,
            "content": content,
            "time": time,
        }),
    )
    .unwrap();

    let cid = block.cid().to_string();
    let data = block.data().to_vec();

    let callback = HandleAnswer::AddFile { cid: cid.clone() };
    let id = cid.into_bytes();
    save_to_store(&mut deps.storage, &id, &data)?;
    Ok(callback)
}

fn get_metadata<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    cid: String,
) -> StdResult<QueryAnswer> {
    let try_cid = Cid::new_v0(
        libipld::cid::multihash::MultihashGeneric::from_bytes(&cid.into_bytes()).unwrap(),
    )
    .unwrap();

    if try_cid.codec() > 0 {
        panic!("Invalid CID");
    }

    let result = load_from_store(&deps.storage, &try_cid.to_bytes());
    let block = IpldBlock::new(try_cid, result.unwrap()).unwrap();
    let response = QueryAnswer::GetMetadata {
        data: block.data().to_vec(),
    };

    //Returns the metadata loaded from store from a CID & transformed to an IpldBlock
    Ok(response)
}

fn get_files<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    cid: String,
) -> StdResult<QueryAnswer> {
    let try_cid = Cid::new_v0(
        libipld::cid::multihash::MultihashGeneric::from_bytes(&cid.into_bytes()).unwrap(),
    )
    .unwrap();

    if try_cid.codec() > 0 {
        panic!("AAAaaaaa!!!!");
    }

    let result = load_from_store(&deps.storage, &try_cid.to_bytes());
    let block = IpldBlock::new(try_cid, result.unwrap()).unwrap();

    let response = QueryAnswer::GetFile {
        data: block.data().to_vec(),
    };

    Ok(response)
}

