extern crate regex;
extern crate hyper;
extern crate select;
extern crate tokio_core;
extern crate web3;
extern crate hex_slice;
extern crate ethabi;
#[macro_use]
extern crate error_chain;
extern crate rustc_hex as hex;
extern crate futures;


use futures::future::Future;
use std::fs::File;
use hex::{ToHex, FromHex};
use hex_slice::AsHex;
use std::io::{self, Write};
use regex::Regex;
use hyper::Client;
use hyper::rt::{self, Stream};
use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};

use ethabi::{Contract, Address, Uint, RawLog};
use ethabi::token::Token;

use web3::futures::Future as w3Future;
use web3::types::{FilterBuilder, H256};

use std::boxed::Box;

error_chain! {
    foreign_links {
        Ethabi(::ethabi::Error);
        Hex(::hex::FromHexError);
        Io(::std::io::Error);
        Web3(web3::Error);
        Hyper(hyper::Error);
    }
}

fn get_party_id(document: Document) -> Option<String> {
    let href = document.find(Attr("id", "lblKlub")).next()?.parent()?.find(Name("a")).next()?.attr("href")?;
    let re = Regex::new(r"klub=(.*)$").unwrap();
    let id = String::from(&re.captures(href)?[1]);
    Some(id)
}

fn get_party(document: Document) -> Option<String> {
    let party = document.find(Attr("id", "lblKlub")).next()?.parent()?.last_child()?.text();
    Some(party)
}

fn http_test(id: String) -> Box<Future<Item=(), Error=Error>> {
    let client = Client::new();
    let urls = format!("http://www.sejm.gov.pl/sejm8.nsf/posel.xsp?id={id:0>3}&type=A", id=id);
    println!("url: {}", urls);
    let uri = urls.parse().unwrap();


    let fut = client
        .get(uri)
        .and_then(|res| {
            res.into_body().concat2()
                .map(|chunk| {
                    let v = chunk.to_vec();
                    let s = String::from_utf8_lossy(&v).to_string();
                    let document = Document::from(s.as_str());
                    let foo = get_party_id(document);
                    println!("{}", foo.unwrap());
                    ()
                })
        })
        .from_err();
    Box::new(fut)
}

#[derive(Debug)]
struct GetPartyRequest {
    from: Address,
    mp_id: Uint,
    request_id: Uint,
}

fn decode_event(event: String, topics: Vec<H256>, data: Vec<u8>) -> Result<GetPartyRequest> {
    let contractPath = "circus.abi";
    let contract = Contract::load(File::open(contractPath)?)?;

    let event = contract.event(&event)?;
    let rawLog = RawLog {topics, data};
    let params = event.parse_log(rawLog)?.params;

    let from = match params[0].value {
        Token::Address(v) => v,
        _ => panic!("not an address"),
    };
    
    let mp_id = match params[1].value {
        Token::Uint(v) => v,
        _ => panic!("not a number"),
    };
    
    let request_id = match params[2].value {
        Token::Uint(v) => v,
        _ => panic!("not a number"),
    };

    Ok(GetPartyRequest { from, mp_id, request_id })
}

fn main() {
    let ethNode = "wss://ropsten.infura.io/ws";

    //let (eloop, transport) = web3::transports::WebSocket::new(ethNode).unwrap();
    
    let mut eloop = tokio_core::reactor::Core::new().unwrap();
    let transport = web3::transports::WebSocket::with_event_loop(ethNode, &eloop.handle()).unwrap();

    let web3 = web3::Web3::new(transport);

    println!("Querying for block number");
    //let accounts = web3.eth().block_number().wait().unwrap();
    /*let blockRun = web3.eth().block_number().then(|number| {
        println!("Block number: {:?}", number);
    });
    eloop.run(blockRun).unwrap();*/


    let addr = "4a2d007fe42dec4f583cfb2cf49a068021a620ce";
    let filter = FilterBuilder::default().address(vec![addr.into()]).build();

    println!("aaa");
    let aaa = 
        web3.eth_subscribe().subscribe_logs(filter).then(|sub| {
            println!("Listening to events on {}", addr);
            sub.unwrap().map_err(Error::from).for_each(|log| {
                let v: Vec<u8> = log.data.0;
                println!("raw: {:02x}", v.plain_hex(false));
                let decoded = decode_event("GetPartyRequest".to_string(), log.topics.clone(), v.clone());
                println!("decoded: {:?}", decoded);
                http_test(decoded.unwrap().mp_id.to_string()).from_err().map(|_| {
                    println!("");
                })
            }).map_err(Error::from)
        }).map_err(Error::from);

    eloop.run(aaa).unwrap();
}
